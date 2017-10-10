use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use smallvec::SmallVec;
use libc::{c_char, c_void};
use vks;
use ::{PRINT, VooResult, Loader, ApplicationInfo, PhysicalDeviceHandle, PhysicalDevice, CharStrs,
    FormatProperties, Format, Handle, SurfaceKhrHandle, SurfaceFormatKhr, PhysicalDeviceFeatures,
    PhysicalDeviceProperties, QueueFamilyProperties, PhysicalDeviceMemoryProperties,
    ExtensionProperties, SurfaceCapabilitiesKhr, Win32SurfaceCreateInfoKhr, DeviceCreateInfo,
    DeviceHandle, ImageType, ImageTiling, ImageCreateFlags, ImageFormatProperties};


unsafe extern "system" fn __debug_callback(_flags: vks::VkDebugReportFlagsEXT,
        _obj_type: vks::VkDebugReportObjectTypeEXT, _obj: u64, _location: usize, _code: i32,
        _layer_prefix: *const c_char, msg: *const c_char, _user_data: *mut c_void) -> u32 {
    println!("{}", CStr::from_ptr(msg).to_str().unwrap());
    vks::VK_FALSE
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct InstanceHandle(pub(crate) vks::VkInstance);

impl InstanceHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkInstance {
        self.0
    }
}

unsafe impl Handle for InstanceHandle {
    type Target = InstanceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: InstanceHandle,
    loader: Loader,
    debug_callback: Option<vks::VkDebugReportCallbackEXT>,
    // physical_devices: SmallVec<[PhysicalDevice; 16]>,
}

#[derive(Debug, Clone)]
pub struct Instance {
    inner: Arc<Inner>,
}

impl Instance {
    #[inline]
    pub fn builder<'ib>() -> InstanceBuilder<'ib> {
        InstanceBuilder::new()
    }

    #[inline(always)]
    pub fn handle(&self) -> InstanceHandle {
        self.inner.handle
    }

    #[inline(always)]
    pub fn proc_addr_loader(&self) -> &vks::InstanceProcAddrLoader {
        self.inner.loader.instance_proc_addr_loader()
    }

    #[inline]
    pub fn loader(&self) -> &Loader {
        &self.inner.loader
    }

    #[inline]
    pub fn physical_devices(&self) -> SmallVec<[PhysicalDevice; 16]> {
        self.loader().enumerate_physical_devices(self.inner.handle)
            .iter().map(|&pdr| PhysicalDevice::from_parts(self.clone(), PhysicalDeviceHandle(pdr)))
            .collect()
    }

    // *PFN_vkGetPhysicalDeviceFeatures)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures);
    pub fn get_physical_device_features<Pd>(&self, physical_device: Pd)
            -> VooResult<PhysicalDeviceFeatures>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut device_features: vks::VkPhysicalDeviceFeatures = mem::uninitialized();
            self.proc_addr_loader().core.vkGetPhysicalDeviceFeatures(physical_device.handle().to_raw(),
                &mut device_features);
            Ok(PhysicalDeviceFeatures::from_raw(device_features))
        }
    }

    // *PFN_vkGetPhysicalDeviceFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties);
    pub fn get_physical_device_format_properties<Pd>(&self, physical_device: Pd, format: Format)
            -> VooResult<FormatProperties>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut props: FormatProperties = mem::uninitialized();
            self.proc_addr_loader().vkGetPhysicalDeviceFormatProperties(physical_device.handle().to_raw(),
                format.into(), &mut props as *mut _ as *mut vks::VkFormatProperties);
            Ok(props)
        }
    }

    // *PFN_vkGetPhysicalDeviceImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties);
    pub fn get_physical_device_image_format_properties<Pd>(&self, physical_device: Pd, format: Format,
            type_: ImageType, tiling: ImageTiling, usage: ImageCreateFlags, flags: ImageCreateFlags)
            -> VooResult<ImageFormatProperties>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut device_image_format_properties = mem::uninitialized();
            ::check(self.proc_addr_loader().vkGetPhysicalDeviceImageFormatProperties(
                physical_device.handle().to_raw(), format.into(), type_.into(),
                tiling.into(), usage.bits(), flags.bits(), &mut device_image_format_properties));
            Ok(ImageFormatProperties::from_raw(device_image_format_properties))
        }
    }

    // *PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties);
    pub fn get_physical_device_properties<Pd>(&self, physical_device: Pd)
            -> VooResult<PhysicalDeviceProperties>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut device_properties: vks::VkPhysicalDeviceProperties = mem::uninitialized();
            self.proc_addr_loader().vkGetPhysicalDeviceProperties(physical_device.handle().to_raw(),
                &mut device_properties);
            Ok(PhysicalDeviceProperties::from_raw(device_properties))
        }
    }

    // *PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties);
    pub fn get_physical_device_queue_family_properties<Pd>(&self, physical_device: Pd)
            -> VooResult<SmallVec<[QueueFamilyProperties; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut queue_family_count = 0u32;
        let mut queue_families = SmallVec::<[::QueueFamilyProperties; 16]>::new();
        unsafe {
            self.proc_addr_loader().core.vkGetPhysicalDeviceQueueFamilyProperties(
                physical_device.handle().to_raw(), &mut queue_family_count, ptr::null_mut());
            assert!(queue_family_count as usize <= queue_families.inline_size());
            queue_families.set_len(queue_family_count as usize);
            self.proc_addr_loader().core.vkGetPhysicalDeviceQueueFamilyProperties(
                physical_device.handle().to_raw(), &mut queue_family_count,
                queue_families.as_mut_ptr() as *mut vks::VkQueueFamilyProperties);
        }
        if PRINT {  println!("Physical device queue family count: {:?}", queue_families.len()); }
        Ok(queue_families)
    }


    // *PFN_vkGetPhysicalDeviceMemoryProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties);
    /// Returns the memory properties for this device.
    pub fn get_physical_device_memory_properties<Pd>(&self, physical_device: Pd)
            -> VooResult<PhysicalDeviceMemoryProperties>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut mem_props: vks::VkPhysicalDeviceMemoryProperties;
        unsafe {
            mem_props = mem::uninitialized();
            self.proc_addr_loader().core.vkGetPhysicalDeviceMemoryProperties(
                physical_device.handle().to_raw(), &mut mem_props);
            Ok(PhysicalDeviceMemoryProperties::from_raw(mem_props))
        }
    }

    // *PFN_vkCreateDevice)(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice);
    pub unsafe fn create_device(&self, physical_device: PhysicalDeviceHandle,
            create_info: &DeviceCreateInfo, allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VooResult<DeviceHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = ptr::null_mut();
        ::check(self.proc_addr_loader().core.vkCreateDevice(physical_device.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle));
        Ok(DeviceHandle(handle))
    }

    // *PFN_vkDestroyDevice)(VkDevice device, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_device(&self, device: DeviceHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyDevice(device.to_raw(), allocator);
    }

    // *PFN_vkEnumerateDeviceExtensionProperties)(VkPhysicalDevice physicalDevice, const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties);
    pub fn enumerate_device_extension_properties<Pd>(&self, physical_device: Pd)
            -> VooResult<SmallVec<[ExtensionProperties; 64]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut ext_count = 0u32;
        let mut exts = SmallVec::<[::ExtensionProperties; 64]>::new();
        unsafe {
            ::check(self.proc_addr_loader().core.vkEnumerateDeviceExtensionProperties(
                physical_device.handle().to_raw(), ptr::null(), &mut ext_count, ptr::null_mut()));
            assert!(ext_count as usize <= exts.inline_size());
            exts.set_len(ext_count as usize);
            ::check(self.proc_addr_loader().core.vkEnumerateDeviceExtensionProperties(
                physical_device.handle().to_raw(), ptr::null(), &mut ext_count,
                exts.as_mut_ptr() as *mut vks::VkExtensionProperties));
        }
        Ok(exts)
    }

    // *PFN_vkEnumerateDeviceLayerProperties)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties);
    pub fn enumerate_device_layer_properties(&self) {
        unimplemented!();
    }

    // *PFN_vkGetPhysicalDeviceSparseImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties);
    pub fn get_physical_device_sparse_image_format_properties<Pd>(&self, physical_device: Pd, format: Format,
            type_: ImageType, tiling: ImageTiling, usage: ImageCreateFlags, flags: ImageCreateFlags)
            -> VooResult<ImageFormatProperties>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut device_image_format_properties = mem::uninitialized();
            ::check(self.proc_addr_loader().vkGetPhysicalDeviceImageFormatProperties(
                physical_device.handle().to_raw(), format.into(), type_.into(),
                tiling.into(), usage.bits(), flags.bits(), &mut device_image_format_properties));
            Ok(ImageFormatProperties::from_raw(device_image_format_properties))
        }
    }

    // *PFN_vkDestroySurfaceKHR)(VkInstance instance, VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_surface_khr(&self, surface: SurfaceKhrHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().khr_surface.vkDestroySurfaceKHR(self.handle().to_raw(),
            surface.to_raw(), allocator);
    }

    // *PFN_vkGetPhysicalDeviceSurfaceSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported);
    pub unsafe fn get_physical_device_surface_support_khr<Pd, Sk>(&self, physical_device: Pd,
            queue_family_index: u32, surface: Sk) -> VooResult<bool>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut supported: vks::VkBool32 = vks::VK_FALSE;
        ::check(self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR(
            physical_device.handle().to_raw(), queue_family_index, surface.handle().to_raw(), &mut supported));
        Ok(supported == vks::VK_TRUE)
    }

    // *PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);
    pub unsafe fn get_physical_device_surface_capabilities_khr<Pd, Sk>(&self, physical_device: Pd,
            surface: Sk) -> VooResult<SurfaceCapabilitiesKhr>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut capabilities = mem::uninitialized();
        self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut capabilities);
        Ok(SurfaceCapabilitiesKhr::from_raw(capabilities))
    }

    // *PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats);
    pub unsafe fn get_physical_device_surface_formats_khr<Pd, Sk>(&self, physical_device: Pd,
            surface: Sk) -> VooResult<SmallVec<[SurfaceFormatKhr; 64]>>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut format_count = 0u32;
        let mut formats: SmallVec<[::SurfaceFormatKhr; 64]> = SmallVec::new();
        self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut format_count, ptr::null_mut());
        assert!(format_count as usize <= formats.inline_size());
        formats.set_len(format_count as usize);
        if format_count != 0 {
            self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(
                physical_device.handle().to_raw(), surface.handle().to_raw(), &mut format_count,
                formats.as_mut_ptr() as *mut vks::VkSurfaceFormatKHR);
        }
        if PRINT { println!("Physical device format count: {:?}", formats.len()); }
        Ok(formats)
    }

    // *PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes);
    pub unsafe fn get_physical_device_surface_present_modes_khr<Pd, Sk>(&self, physical_device: Pd,
            surface: Sk) -> VooResult<SmallVec<[::PresentModeKhr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut present_mode_count = 0u32;
        let mut present_modes: SmallVec<[::PresentModeKhr; 16]> = SmallVec::new();
        self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut present_mode_count, ptr::null_mut());
        assert!(present_mode_count as usize <= present_modes.inline_size());
        present_modes.set_len(present_mode_count as usize);
        if present_mode_count != 0 {
            self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(
                physical_device.handle().to_raw(), surface.handle().to_raw(), &mut present_mode_count, present_modes.as_mut_ptr() as *mut _);
        }
        if PRINT { println!("Physical device present mode count: {:?}", present_modes.len()); }
        Ok(present_modes)
    }


    // *PFN_vkGetPhysicalDeviceDisplayPropertiesKHR)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPropertiesKHR* pProperties);
    pub unsafe fn get_physical_device_display_properties_khr(&self) {
        unimplemented!();
    }


        // *PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPlanePropertiesKHR* pProperties);
    pub unsafe fn get_physical_device_display_plane_properties_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetDisplayPlaneSupportedDisplaysKHR)(VkPhysicalDevice physicalDevice, uint32_t planeIndex, uint32_t* pDisplayCount, VkDisplayKHR* pDisplays);
    pub unsafe fn get_display_plane_supported_displays_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetDisplayModePropertiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, uint32_t* pPropertyCount, VkDisplayModePropertiesKHR* pProperties);
    pub unsafe fn get_display_mode_properties_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateDisplayModeKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, const VkDisplayModeCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDisplayModeKHR* pMode);
    pub unsafe fn create_display_mode_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetDisplayPlaneCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayModeKHR mode, uint32_t planeIndex, VkDisplayPlaneCapabilitiesKHR* pCapabilities);
    pub unsafe fn get_display_plane_capabilities_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateDisplayPlaneSurfaceKHR)(VkInstance instance, const VkDisplaySurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_display_plane_surface_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateXlibSurfaceKHR)(VkInstance instance, const VkXlibSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_xlib_surface_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, Display* dpy, VisualID visualID);
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateXcbSurfaceKHR)(VkInstance instance, const VkXcbSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_xcb_surface_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, xcb_connection_t* connection, xcb_visualid_t visual_id);
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateWaylandSurfaceKHR)(VkInstance instance, const VkWaylandSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_wayland_surface_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, struct wl_display* display);
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateMirSurfaceKHR)(VkInstance instance, const VkMirSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_mir_surface_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceMirPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, MirConnection* connection);
    pub unsafe fn get_physical_device_mir_presentation_support_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateAndroidSurfaceKHR)(VkInstance instance, const VkAndroidSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_android_surface_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateWin32SurfaceKHR)(VkInstance instance, const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_win32_surface_khr(&self, create_info: &Win32SurfaceCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VooResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        ::check(self.proc_addr_loader().khr_win32_surface.vkCreateWin32SurfaceKHR(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut handle));
        Ok(SurfaceKhrHandle(handle))
    }

    // *PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex);
    pub unsafe fn get_physical_device_win32_presentation_support_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceFeatures2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2KHR* pFeatures);
    pub unsafe fn get_physical_device_features_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceProperties2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2KHR* pProperties);
    pub unsafe fn get_physical_device_properties_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceFormatProperties2KHR)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2KHR* pFormatProperties);
    pub unsafe fn get_physical_device_format_properties_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2KHR* pImageFormatInfo, VkImageFormatProperties2KHR* pImageFormatProperties);
    pub unsafe fn get_physical_device_image_format_properties_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2KHR* pQueueFamilyProperties);
    pub unsafe fn get_physical_device_queue_family_properties_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceMemoryProperties2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2KHR* pMemoryProperties);
    pub unsafe fn get_physical_device_memory_properties_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2KHR* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2KHR* pProperties);
    pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalBufferInfoKHR* pExternalBufferInfo, VkExternalBufferPropertiesKHR* pExternalBufferProperties);
    pub unsafe fn get_physical_device_external_buffer_properties_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalSemaphoreInfoKHR* pExternalSemaphoreInfo, VkExternalSemaphorePropertiesKHR* pExternalSemaphoreProperties);
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalFenceInfoKHR* pExternalFenceInfo, VkExternalFencePropertiesKHR* pExternalFenceProperties);
    pub unsafe fn get_physical_device_external_fence_properties_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities);
    pub unsafe fn get_physical_device_surface_capabilities_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceSurfaceFormats2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats);
    pub unsafe fn get_physical_device_surface_formats_2_khr(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateDebugReportCallbackEXT)(VkInstance instance, const VkDebugReportCallbackCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugReportCallbackEXT* pCallback);
    pub unsafe fn create_debug_report_callback_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkDestroyDebugReportCallbackEXT)(VkInstance instance, VkDebugReportCallbackEXT callback, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_debug_report_callback_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkDebugReportMessageEXT)(VkInstance instance, VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT objectType, uint64_t object, size_t location, int32_t messageCode, const char* pLayerPrefix, const char* pMessage);
    pub unsafe fn debug_report_message_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkExternalMemoryHandleTypeFlagsNV externalHandleType, VkExternalImageFormatPropertiesNV* pExternalImageFormatProperties);
    pub unsafe fn get_physical_device_external_image_format_properties_nv(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDevicePresentRectanglesKHX)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects);
    pub unsafe fn get_physical_device_present_rectangles_khx(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateViSurfaceNN)(VkInstance instance, const VkViSurfaceCreateInfoNN* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_vi_surface_nn(&self) {
        unimplemented!();
    }


    // *PFN_vkEnumeratePhysicalDeviceGroupsKHX)(VkInstance instance, uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupPropertiesKHX* pPhysicalDeviceGroupProperties);
    pub unsafe fn enumerate_physical_device_groups_khx(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)(VkPhysicalDevice physicalDevice, VkDeviceGeneratedCommandsFeaturesNVX* pFeatures, VkDeviceGeneratedCommandsLimitsNVX* pLimits);
    pub unsafe fn get_physical_device_generated_commands_properties_nvx(&self) {
        unimplemented!();
    }


    // *PFN_vkReleaseDisplayEXT)(VkPhysicalDevice physicalDevice, VkDisplayKHR display);
    pub unsafe fn release_display_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkAcquireXlibDisplayEXT)(VkPhysicalDevice physicalDevice, Display* dpy, VkDisplayKHR display);
    pub unsafe fn acquire_xlib_display_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkGetRandROutputDisplayEXT)(VkPhysicalDevice physicalDevice, Display* dpy, RROutput rrOutput, VkDisplayKHR* pDisplay);
    pub unsafe fn get_rand_r_output_display_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilities2EXT* pSurfaceCapabilities);
    pub unsafe fn get_physical_device_surface_capabilities2_ext(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateIOSSurfaceMVK)(VkInstance instance, const VkIOSSurfaceCreateInfoMVK* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_i_o_s_surface_mvk(&self) {
        unimplemented!();
    }


    // *PFN_vkCreateMacOSSurfaceMVK)(VkInstance instance, const VkMacOSSurfaceCreateInfoMVK* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_mac_o_s_surface_mvk(&self) {
        unimplemented!();
    }


    // *PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT)(VkPhysicalDevice physicalDevice, VkSampleCountFlagBits samples, VkMultisamplePropertiesEXT* pMultisampleProperties);
    pub unsafe fn get_physical_device_multisample_properties_ext(&self) {
        unimplemented!();
    }

}

unsafe impl<'h> Handle for &'h Instance {
    type Target = InstanceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            if PRINT { println!("Destroying debug callback..."); }
            if let Some(callback) = self.debug_callback {
                self.loader.instance_proc_addr_loader().ext_debug_report.vkDestroyDebugReportCallbackEXT(self.handle.0,
                    callback, ptr::null());
            }

            if PRINT { println!("Destroying instance..."); }
            self.loader.instance_proc_addr_loader().core.vkDestroyInstance(self.handle.0, ptr::null());
        }
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}



/// A builder used to create an `Instance`.
#[derive(Debug, Clone)]
pub struct InstanceBuilder<'ib> {
    create_info: vks::VkInstanceCreateInfo,
    enabled_layer_names: Option<CharStrs<'ib>>,
    enabled_extension_names: Option<CharStrs<'ib>>,
    _p: PhantomData<&'ib ()>,
}

impl<'ib> InstanceBuilder<'ib> {
    /// Returns a new instance builder.
    pub fn new() -> InstanceBuilder<'ib> {
        InstanceBuilder {
            create_info: vks::VkInstanceCreateInfo::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            _p: PhantomData,
        }
    }

    /// Sets the application info.
    pub fn application_info<'ai, 's>(&'s mut self, application_info: &'ai ApplicationInfo)
            -> &'s mut InstanceBuilder<'ib>
            where 'ai: 'ib {
        self.create_info.pApplicationInfo = application_info.as_raw();
        self
    }

    /// Sets the enabled layer names.
    // pub fn enabled_layer_names<'eln, 's>(&'s mut self, enabled_layer_names: &'eln [&'eln CStr])
    pub fn enabled_layer_names<'s, 'cs, Cs>(&'s mut self, enabled_layer_names: Cs)
            -> &'s mut InstanceBuilder<'ib>
            where 'cs: 'ib, Cs: 'cs + Into<CharStrs<'cs>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        if let Some(ref elns) = self.enabled_layer_names {
            self.create_info.ppEnabledLayerNames = elns.as_ptr();
            self.create_info.enabledLayerCount = elns.len() as u32;
        }
        self
    }

    /// Sets the enabled extension names.
    ///
    /// May not be used with `::enabled_extensions`.
    // pub fn enabled_extension_names<'een, 's>(&'s mut self, enabled_extension_names: &'een [&'een CStr])
    //         -> &'s mut InstanceBuilder<'ib>
    //         where 'een: 'ib {
    pub fn enabled_extension_names<'s, 'cs, Cs>(&'s mut self, enabled_extension_names: Cs)
            -> &'s mut InstanceBuilder<'ib>
            where 'cs: 'ib, Cs: 'cs + Into<CharStrs<'cs>> {
        if !self.create_info.ppEnabledExtensionNames.is_null() {
            panic!("Enabled extension names have already been set.");
        }
        self.enabled_extension_names = Some(enabled_extension_names.into());
        if let Some(ref eens) = self.enabled_extension_names {
            self.create_info.ppEnabledExtensionNames = eens.as_ptr();
            self.create_info.enabledExtensionCount = eens.len() as u32;
        }
        self
    }

    /// Sets the enabled extension names by providing a list of extensions.
    ///
    /// May not be used with `::enabled_extension_lists`.
    pub fn enabled_extensions<'een, 's>(&'s mut self, enabled_extensions: &'een [vks::VkExtensionProperties])
            -> &'s mut InstanceBuilder<'ib>
            where 'een: 'ib {
        if !self.create_info.ppEnabledExtensionNames.is_null() {
            panic!("Enabled extension names have already been set.");
        }
        let enabled_extension_name_ptrs: Vec<_> = enabled_extensions.iter().map(|eext| {
            if PRINT { println!("Enabling instance extension: '{}' (version: {})",
                unsafe { CStr::from_ptr(&eext.extensionName as *const c_char).to_str().unwrap() },
                    eext.specVersion); }
            eext.extensionName.as_ptr()
        }).collect();

        self.enabled_extension_names = Some(CharStrs::OwnedPtr { ptrs: enabled_extension_name_ptrs });
        if let Some(ref eens) = self.enabled_extension_names {
            self.create_info.ppEnabledExtensionNames = eens.as_ptr();
            self.create_info.enabledExtensionCount = eens.len() as u32;
        }
        self
    }

    /// Builds and returns a new `Instance`.
    pub fn build(&self, mut loader: Loader, enable_debug_callback: bool) -> VooResult<Instance> {
        let mut handle = ptr::null_mut();

        unsafe {
            ::check(loader.core_global().vkCreateInstance(&self.create_info, ptr::null(), &mut handle));
            // [FIXME: do this properly] Load extension function pointers:
            loader.instance_proc_addr_loader_mut().load_core(handle);
            loader.instance_proc_addr_loader_mut().load_khr_surface(handle);
            loader.instance_proc_addr_loader_mut().load_khr_win32_surface(handle);
            loader.instance_proc_addr_loader_mut().load_khr_get_physical_device_properties2(handle);
            loader.instance_proc_addr_loader_mut().load_khr_external_memory_capabilities(handle);
        }

        // TODO: Ensure that the debug extension is enabled by consulting the
        // enabled extension list instead.
        if enable_debug_callback { unsafe { loader.instance_proc_addr_loader_mut().load_ext_debug_report(handle); } }

        // TODO: Ensure that the debug extension is enabled by consulting the
        // enabled extension list instead.
        let debug_callback = if enable_debug_callback {
            let create_info = vks::VkDebugReportCallbackCreateInfoEXT {
                sType:  vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
                pNext: ptr::null(),
                flags: vks::VK_DEBUG_REPORT_ERROR_BIT_EXT | vks::VK_DEBUG_REPORT_WARNING_BIT_EXT,
                pfnCallback: Some(__debug_callback),
                pUserData: ptr::null_mut(),
            };

            let mut callback: vks::VkDebugReportCallbackEXT = 0;
            if unsafe { loader.instance_proc_addr_loader().ext_debug_report.vkCreateDebugReportCallbackEXT(handle,
                    &create_info, ptr::null(), &mut callback) } != vks::VK_SUCCESS
            {
                panic!("failed to set up debug callback");
            } else {
                if PRINT { println!("Debug report callback initialized."); }
            }
            Some(callback)
        } else {
            None
        };

        // // Device:
        // let physical_devices = unsafe { enumerate_physical_devices(handle, loader.loader()) };

        Ok(Instance {
            inner: Arc::new(Inner {
                handle: InstanceHandle(handle),
                loader,
                debug_callback,
                // physical_devices,
            }),
        })
    }
}
