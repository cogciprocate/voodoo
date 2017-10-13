use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use smallvec::SmallVec;
use libc::{c_char, c_void};
use vks;
use ::{error, PRINT, CallResult, VdResult, Loader, ApplicationInfo, PhysicalDeviceHandle,
    PhysicalDevice, CharStrs, FormatProperties, Format, Handle, SurfaceKhrHandle, SurfaceFormatKhr,
    PhysicalDeviceFeatures, PhysicalDeviceProperties, QueueFamilyProperties,
    PhysicalDeviceMemoryProperties, ExtensionProperties, SurfaceCapabilitiesKhr,
    Win32SurfaceCreateInfoKhr, DeviceCreateInfo, DeviceHandle, ImageType, ImageTiling,
    ImageUsageFlags, ImageCreateFlags, ImageFormatProperties, SampleCountFlags,
    SparseImageFormatProperties, PresentModeKhr, DisplayPropertiesKhr, DisplayPlanePropertiesKhr,
    DisplayKhr, DisplayModePropertiesKhr, DisplayKhrHandle, DisplayModeCreateInfoKhr,
    DisplayPlaneCapabilitiesKhr, DisplayModeKhrHandle, DisplaySurfaceCreateInfoKhr,
    XlibSurfaceCreateInfoKhr, Display, XcbSurfaceCreateInfoKhr, xcb_connection_t, xcb_visualid_t,
    WaylandSurfaceCreateInfoKhr, wl_display, VisualID, MirSurfaceCreateInfoKhr, MirConnection,
    AndroidSurfaceCreateInfoKhr, PhysicalDeviceFeatures2Khr, PhysicalDeviceProperties2Khr,
    PhysicalDeviceExternalBufferInfoKhr, ExternalBufferPropertiesKhr,
    PhysicalDeviceExternalSemaphoreInfoKhr, ExternalSemaphorePropertiesKhr,
    PhysicalDeviceExternalFenceInfoKhr, ExternalFencePropertiesKhr,
    DebugReportCallbackCreateInfoExt, DebugReportCallbackExtHandle, DebugReportFlagsExt,
    DebugReportObjectTypeExt, LayerProperties, FormatProperties2Khr, ImageFormatProperties2Khr,
    QueueFamilyProperties2Khr, PhysicalDeviceMemoryProperties2Khr, SparseImageFormatProperties2Khr,
    PhysicalDeviceImageFormatInfo2Khr, PhysicalDeviceSparseImageFormatInfo2Khr,
    SurfaceCapabilities2Khr, SurfaceFormat2Khr, PhysicalDeviceSurfaceInfo2Khr,
    ViSurfaceCreateInfoNn, IosSurfaceCreateInfoMvk, MacOsSurfaceCreateInfoMvk,
    ExternalImageFormatPropertiesNv, ExternalMemoryHandleTypeFlagsNv, SurfaceCapabilities2Ext,
    RROutput};

#[cfg(feature = "experimental")]
use ::{Rect2d, };

#[cfg(feature = "unimplemented")]
use ::{MultisamplePropertiesExt, };


unsafe extern "system" fn __debug_callback(_flags: vks::VkDebugReportFlagsEXT,
        _obj_type: vks::VkDebugReportObjectTypeEXT, _obj: u64, _location: usize, _code: i32,
        _layer_prefix: *const c_char, msg: *const c_char, _user_data: *mut c_void) -> u32 {
    println!("DEBUG_REPORT: {}", CStr::from_ptr(msg).to_str().unwrap());
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
    pub fn physical_devices(&self) -> VdResult<SmallVec<[PhysicalDevice; 16]>> {
        Ok(self.loader().enumerate_physical_devices(self.inner.handle)?
            .iter().map(|&pdr| {
                PhysicalDevice::from_parts(self.clone(), PhysicalDeviceHandle(pdr))
            }).collect())
    }

    // *PFN_vkGetPhysicalDeviceFeatures)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures);
    pub fn get_physical_device_features<Pd>(&self, physical_device: Pd)
            -> PhysicalDeviceFeatures
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut features: vks::VkPhysicalDeviceFeatures = mem::uninitialized();
            self.proc_addr_loader().core.vkGetPhysicalDeviceFeatures(physical_device.handle().to_raw(),
                &mut features);
            PhysicalDeviceFeatures::from_raw(features)
        }
    }

    // *PFN_vkGetPhysicalDeviceFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties);
    pub fn get_physical_device_format_properties<Pd>(&self, physical_device: Pd, format: Format)
            -> FormatProperties
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut props: FormatProperties = mem::uninitialized();
            self.proc_addr_loader().core.vkGetPhysicalDeviceFormatProperties(physical_device.handle().to_raw(),
                format.into(), &mut props as *mut _ as *mut vks::VkFormatProperties);
            props
        }
    }

    // *PFN_vkGetPhysicalDeviceImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties);
    pub fn get_physical_device_image_format_properties<Pd>(&self, physical_device: Pd, format: Format,
            type_: ImageType, tiling: ImageTiling, usage: ImageUsageFlags, flags: ImageCreateFlags)
            -> VdResult<ImageFormatProperties>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut image_format_properties = mem::uninitialized();
            let result = self.proc_addr_loader().core.vkGetPhysicalDeviceImageFormatProperties(
                physical_device.handle().to_raw(), format.into(), type_.into(),
                tiling.into(), usage.bits(), flags.bits(), &mut image_format_properties);
            error::check(result, "vkGetPhysicalDeviceImageFormatProperties", ImageFormatProperties::from_raw(image_format_properties))
        }
    }

    // *PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties);
    pub fn get_physical_device_properties<Pd>(&self, physical_device: Pd)
            -> PhysicalDeviceProperties
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        unsafe {
            let mut device_properties: vks::VkPhysicalDeviceProperties = mem::uninitialized();
            self.proc_addr_loader().core.vkGetPhysicalDeviceProperties(physical_device.handle().to_raw(),
                &mut device_properties);
            PhysicalDeviceProperties::from_raw(device_properties)
        }
    }

    // *PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties);
    pub fn get_physical_device_queue_family_properties<Pd>(&self, physical_device: Pd)
            -> VdResult<SmallVec<[QueueFamilyProperties; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut queue_family_count = 0u32;
        let mut queue_families = SmallVec::<[::QueueFamilyProperties; 16]>::new();
        unsafe {
            self.proc_addr_loader().core.vkGetPhysicalDeviceQueueFamilyProperties(
                physical_device.handle().to_raw(), &mut queue_family_count, ptr::null_mut());
            queue_families.reserve_exact(queue_family_count as usize);
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
            -> PhysicalDeviceMemoryProperties
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut mem_props: vks::VkPhysicalDeviceMemoryProperties;
        unsafe {
            mem_props = mem::uninitialized();
            self.proc_addr_loader().core.vkGetPhysicalDeviceMemoryProperties(
                physical_device.handle().to_raw(), &mut mem_props);
            PhysicalDeviceMemoryProperties::from_raw(mem_props)
        }
    }

    // *PFN_vkCreateDevice)(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice);
    pub unsafe fn create_device(&self, physical_device: PhysicalDeviceHandle,
            create_info: &DeviceCreateInfo, allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<DeviceHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = ptr::null_mut();
        let result = self.proc_addr_loader().core.vkCreateDevice(physical_device.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateDevice", DeviceHandle(handle))
    }

    // *PFN_vkDestroyDevice)(VkDevice device, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_device(&self, device: DeviceHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyDevice(device.to_raw(), allocator);
    }

    // *PFN_vkEnumerateDeviceExtensionProperties)(VkPhysicalDevice physicalDevice, const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties);
    pub fn enumerate_device_extension_properties<Pd>(&self, physical_device: Pd,
            layer_name: Option<&CStr>)
            -> VdResult<SmallVec<[ExtensionProperties; 64]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let layer_name = layer_name.map(|ln| ln.as_ptr()).unwrap_or(ptr::null());
        let mut property_count = 0u32;
        let mut properties = SmallVec::<[ExtensionProperties; 64]>::new();
        unsafe {
            let result = self.proc_addr_loader().core.vkEnumerateDeviceExtensionProperties(
                physical_device.handle().to_raw(), layer_name, &mut property_count, ptr::null_mut());
            error::check(result, "vkEnumerateDeviceExtensionProperties", ())?;
            properties.reserve_exact(property_count as usize);
            properties.set_len(property_count as usize);
            loop {
                let result = self.proc_addr_loader().core.vkEnumerateDeviceExtensionProperties(
                    physical_device.handle().to_raw(), layer_name, &mut property_count,
                    properties.as_mut_ptr() as *mut vks::VkExtensionProperties);
                if result != CallResult::Incomplete as i32 {
                    return error::check(result, "vkEnumerateDeviceExtensionProperties", properties);
                }
            }
        }
    }

    // *PFN_vkEnumerateDeviceLayerProperties)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties);
    pub fn enumerate_device_layer_properties<Pd>(&self, physical_device: Pd)
            -> VdResult<SmallVec<[LayerProperties; 64]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut property_count = 0u32;
        let mut properties = SmallVec::<[LayerProperties; 64]>::new();
        unsafe {
            let result = self.proc_addr_loader().core.vkEnumerateDeviceLayerProperties(
                physical_device.handle().to_raw(), &mut property_count, ptr::null_mut());
            error::check(result, "vkEnumerateDeviceLayerProperties", ())?;
            properties.reserve_exact(property_count as usize);
            properties.set_len(property_count as usize);
            loop {
                let result = self.proc_addr_loader().core.vkEnumerateDeviceLayerProperties(
                    physical_device.handle().to_raw(), &mut property_count,
                    properties.as_mut_ptr() as *mut vks::VkLayerProperties);
                if result != CallResult::Incomplete as i32 {
                    return error::check(result, "vkEnumerateDeviceLayerProperties", properties);
                }
            }
        }
    }

    // *PFN_vkGetPhysicalDeviceSparseImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties);
    pub fn get_physical_device_sparse_image_format_properties<Pd>(&self, physical_device: Pd,
            format: Format, type_: ImageType, samples: SampleCountFlags, usage: ImageCreateFlags,
            tiling: ImageTiling) -> SmallVec<[SparseImageFormatProperties; 8]>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut property_count = 0u32;
        let mut properties = SmallVec::<[SparseImageFormatProperties; 8]>::new();
        unsafe {
            self.proc_addr_loader().core.vkGetPhysicalDeviceSparseImageFormatProperties(
                physical_device.handle().to_raw(), format.into(), type_.into(),
                samples.bits(), tiling.into(), usage.bits(), &mut property_count, ptr::null_mut());
            properties.reserve_exact(property_count as usize);
            properties.set_len(property_count as usize);
            self.proc_addr_loader().core.vkGetPhysicalDeviceSparseImageFormatProperties(
                physical_device.handle().to_raw(), format.into(), type_.into(),
                samples.bits(), tiling.into(), usage.bits(), &mut property_count,
                properties.as_mut_ptr() as *mut vks::VkSparseImageFormatProperties);
        }
        properties
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
            queue_family_index: u32, surface: Sk) -> VdResult<bool>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut supported: vks::VkBool32 = vks::VK_FALSE;
        let result = self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR(
            physical_device.handle().to_raw(), queue_family_index, surface.handle().to_raw(), &mut supported);
        error::check(result, "vkGetPhysicalDeviceSurfaceSupportKHR", supported == vks::VK_TRUE)
    }

    // *PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);
    pub unsafe fn get_physical_device_surface_capabilities_khr<Pd, Sk>(&self, physical_device: Pd,
            surface: Sk) -> VdResult<SurfaceCapabilitiesKhr>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut capabilities = mem::uninitialized();
        let result = self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut capabilities);
        error::check(result, "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
            SurfaceCapabilitiesKhr::from_raw(capabilities))
    }

    // *PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats);
    pub unsafe fn get_physical_device_surface_formats_khr<Pd, Sk>(&self, physical_device: Pd,
            surface: Sk) -> VdResult<SmallVec<[SurfaceFormatKhr; 64]>>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut format_count = 0u32;
        let mut formats: SmallVec<[::SurfaceFormatKhr; 64]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut format_count, ptr::null_mut());
        error::check(result, "vkGetPhysicalDeviceSurfaceFormatsKHR", ())?;
        formats.reserve_exact(format_count as usize);
        formats.set_len(format_count as usize);
        if format_count != 0 {
            let result = self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(
                physical_device.handle().to_raw(), surface.handle().to_raw(), &mut format_count,
                formats.as_mut_ptr() as *mut vks::VkSurfaceFormatKHR);
            if PRINT { println!("Physical device format count: {:?}", formats.len()); }
            error::check(result, "vkGetPhysicalDeviceSurfaceFormatsKHR", formats)
        } else {
            Ok(formats)
        }
    }

    // *PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes);
    pub unsafe fn get_physical_device_surface_present_modes_khr<Pd, Sk>(&self, physical_device: Pd,
            surface: Sk) -> VdResult<SmallVec<[PresentModeKhr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle>, Sk: Handle<Target=SurfaceKhrHandle> {
        let mut present_mode_count = 0u32;
        let mut present_modes: SmallVec<[PresentModeKhr; 16]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut present_mode_count, ptr::null_mut());
        error::check(result, "vkGetPhysicalDeviceSurfacePresentModesKHR", ())?;
        present_modes.reserve_exact(present_mode_count as usize);
        present_modes.set_len(present_mode_count as usize);
        if present_mode_count != 0 {
            loop {
                let result = self.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(
                    physical_device.handle().to_raw(), surface.handle().to_raw(),
                    &mut present_mode_count, present_modes.as_mut_ptr() as *mut _);
                if result != CallResult::Incomplete as i32 {
                    if PRINT { println!("Physical device present mode count: {:?}", present_modes.len()); }
                    return error::check(result, "vkGetPhysicalDeviceSurfacePresentModesKHR", present_modes);
                }
            }
        }
        Ok(present_modes)
    }

    // *PFN_vkGetPhysicalDeviceDisplayPropertiesKHR)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPropertiesKHR* pProperties);
    pub unsafe fn get_physical_device_display_properties_khr<Pd>(&self, physical_device: Pd)
            -> VdResult<SmallVec<[DisplayPropertiesKhr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut property_count = 0u32;
        let mut properties: SmallVec<[DisplayPropertiesKhr; 16]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_display.vkGetPhysicalDeviceDisplayPropertiesKHR(
            physical_device.handle().to_raw(), &mut property_count, ptr::null_mut());
        error::check(result, "vkGetPhysicalDeviceDisplayPropertiesKHR", ())?;
        properties.reserve_exact(property_count as usize);
        properties.set_len(property_count as usize);
        loop {
            let result = self.proc_addr_loader().khr_display.vkGetPhysicalDeviceDisplayPropertiesKHR(
                physical_device.handle().to_raw(), &mut property_count,
                properties.as_mut_ptr() as *mut vks::VkDisplayPropertiesKHR);
            if result != CallResult::Incomplete as i32 {
                return error::check(result, "vkGetPhysicalDeviceDisplayPropertiesKHR", properties);
            }
        }
    }

    // *PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPlanePropertiesKHR* pProperties);
    pub unsafe fn get_physical_device_display_plane_properties_khr<Pd>(&self, physical_device: Pd)
            -> VdResult<SmallVec<[DisplayPlanePropertiesKhr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut property_count = 0u32;
        let mut properties: SmallVec<[DisplayPlanePropertiesKhr; 16]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_display.vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
            physical_device.handle().to_raw(), &mut property_count, ptr::null_mut());
        error::check(result, "vkGetPhysicalDeviceDisplayPlanePropertiesKHR", ())?;
        properties.reserve_exact(property_count as usize);
        properties.set_len(property_count as usize);
        loop {
            let result = self.proc_addr_loader().khr_display.vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
                physical_device.handle().to_raw(), &mut property_count,
                properties.as_mut_ptr() as *mut vks::VkDisplayPlanePropertiesKHR);
            if result != CallResult::Incomplete as i32 {
                return error::check(result, "vkGetPhysicalDeviceDisplayPlanePropertiesKHR", properties);
            }
        }
    }

    // *PFN_vkGetDisplayPlaneSupportedDisplaysKHR)(VkPhysicalDevice physicalDevice, uint32_t planeIndex, uint32_t* pDisplayCount, VkDisplayKHR* pDisplays);
    pub unsafe fn get_display_plane_supported_displays_khr<Pd>(&self, physical_device: Pd, plane_index: u32)
            -> VdResult<SmallVec<[DisplayKhr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut display_count = 0u32;
        let mut displays: SmallVec<[DisplayKhr; 16]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_display.vkGetDisplayPlaneSupportedDisplaysKHR(
            physical_device.handle().to_raw(), plane_index, &mut display_count, ptr::null_mut());
        error::check(result, "vkGetDisplayPlaneSupportedDisplaysKHR", ())?;
        displays.reserve_exact(display_count as usize);
        displays.set_len(display_count as usize);
        loop {
            let result = self.proc_addr_loader().khr_display.vkGetDisplayPlaneSupportedDisplaysKHR(
                physical_device.handle().to_raw(), plane_index, &mut display_count,
                displays.as_mut_ptr() as *mut vks::VkDisplayKHR);
            if result != CallResult::Incomplete as i32 {
                return error::check(result, "vkGetDisplayPlaneSupportedDisplaysKHR", displays);
            }
        }
    }

    // *PFN_vkGetDisplayModePropertiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, uint32_t* pPropertyCount, VkDisplayModePropertiesKHR* pProperties);
    pub unsafe fn get_display_mode_properties_khr<Pd, D>(&self, physical_device: Pd, display: D)
            -> VdResult<SmallVec<[DisplayModePropertiesKhr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle>, D: Handle<Target=DisplayKhrHandle> {
        let mut property_count = 0u32;
        let mut properties: SmallVec<[DisplayModePropertiesKhr; 16]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_display.vkGetDisplayModePropertiesKHR(
            physical_device.handle().to_raw(), display.handle().to_raw(), &mut property_count, ptr::null_mut());
        error::check(result, "vkGetDisplayModePropertiesKHR", ())?;
        properties.reserve_exact(property_count as usize);
        properties.set_len(property_count as usize);
        loop {
            let result = self.proc_addr_loader().khr_display.vkGetDisplayModePropertiesKHR(
                physical_device.handle().to_raw(), display.handle().to_raw(), &mut property_count,
                properties.as_mut_ptr() as *mut vks::VkDisplayModePropertiesKHR);
            if result != CallResult::Incomplete as i32 {
                return error::check(result, "vkGetDisplayModePropertiesKHR", properties);
            }
        }
    }

    // *PFN_vkCreateDisplayModeKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, const VkDisplayModeCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDisplayModeKHR* pMode);
    pub unsafe fn create_display_mode_khr<Pd, D>(&self, physical_device: Pd, display: D,
            create_info: &DisplayModeCreateInfoKhr, allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<DisplayModeKhrHandle>
            where Pd: Handle<Target=PhysicalDeviceHandle>, D: Handle<Target=DisplayKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut mode = 0;
        let result = self.proc_addr_loader().khr_display.vkCreateDisplayModeKHR(physical_device.handle().to_raw(),
            display.handle().to_raw(), create_info.as_raw(), allocator, &mut mode);
        error::check(result, "vkCreateDisplayModeKHR", DisplayModeKhrHandle(mode))
    }

    // *PFN_vkGetDisplayPlaneCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayModeKHR mode, uint32_t planeIndex, VkDisplayPlaneCapabilitiesKHR* pCapabilities);
    pub unsafe fn get_display_plane_capabilities_khr<Pd, M>(&self, physical_device: Pd, mode: M,
            plane_index: u32)
            -> VdResult<DisplayPlaneCapabilitiesKhr>
            where Pd: Handle<Target=PhysicalDeviceHandle>, M: Handle<Target=DisplayModeKhrHandle> {
        let mut capabilities = mem::uninitialized();
        let result = self.proc_addr_loader().khr_display.vkGetDisplayPlaneCapabilitiesKHR(physical_device.handle().to_raw(),
            mode.handle().to_raw(), plane_index, &mut capabilities);
        error::check(result, "vkGetDisplayPlaneCapabilitiesKHR",
            DisplayPlaneCapabilitiesKhr::from_raw(capabilities))
    }

    // *PFN_vkCreateDisplayPlaneSurfaceKHR)(VkInstance instance, const VkDisplaySurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_display_plane_surface_khr(&self, create_info: DisplaySurfaceCreateInfoKhr,
             allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_display.vkCreateDisplayPlaneSurfaceKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateDisplayPlaneSurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkCreateXlibSurfaceKHR)(VkInstance instance, const VkXlibSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_xlib_surface_khr(&self, create_info: &XlibSurfaceCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_xlib_surface.vkCreateXlibSurfaceKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateXlibSurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, Display* dpy, VisualID visualID);
    pub unsafe fn get_physical_device_xlib_presentation_support_khr<Pd>(&self, physical_device: Pd,
            queue_family_index: u32, dpy: *mut Display, visual_id: VisualID) -> bool
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let result = self.proc_addr_loader().khr_xlib_surface.vkGetPhysicalDeviceXlibPresentationSupportKHR(
            physical_device.handle().to_raw(), queue_family_index, dpy, visual_id);
        result != 0
    }

    // *PFN_vkCreateXcbSurfaceKHR)(VkInstance instance, const VkXcbSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_xcb_surface_khr(&self, create_info: &XcbSurfaceCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_xcb_surface.vkCreateXcbSurfaceKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateXcbSurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, xcb_connection_t* connection, xcb_visualid_t visual_id);
    pub unsafe fn get_physical_device_xcb_presentation_support_khr<Pd>(&self, physical_device: Pd,
        queue_family_index: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t)
             -> bool
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let result = self.proc_addr_loader().khr_xcb_surface.vkGetPhysicalDeviceXcbPresentationSupportKHR(
            physical_device.handle().to_raw(), queue_family_index, connection, visual_id);
        result != 0

    }

    // *PFN_vkCreateWaylandSurfaceKHR)(VkInstance instance, const VkWaylandSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_wayland_surface_khr(&self, create_info: &WaylandSurfaceCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_wayland_surface.vkCreateWaylandSurfaceKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateWaylandSurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, struct wl_display* display);
    pub unsafe fn get_physical_device_wayland_presentation_support_khr<Pd>(&self,
            physical_device: Pd, queue_family_index: u32, display: *mut wl_display) -> bool
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let result = self.proc_addr_loader().khr_wayland_surface.vkGetPhysicalDeviceWaylandPresentationSupportKHR(
            physical_device.handle().to_raw(), queue_family_index, display);
        result != 0
    }

    // *PFN_vkCreateMirSurfaceKHR)(VkInstance instance, const VkMirSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_mir_surface_khr(&self,
            create_info: &MirSurfaceCreateInfoKhr, allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_mir_surface.vkCreateMirSurfaceKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateMirSurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkGetPhysicalDeviceMirPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, MirConnection* connection);
    pub unsafe fn get_physical_device_mir_presentation_support_khr<Pd>(&self,
            physical_device: Pd, queue_family_index: u32, connection: *mut MirConnection) -> bool
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let result = self.proc_addr_loader().khr_mir_surface.vkGetPhysicalDeviceMirPresentationSupportKHR(
            physical_device.handle().to_raw(), queue_family_index, connection);
        result != 0
    }

    // *PFN_vkCreateAndroidSurfaceKHR)(VkInstance instance, const VkAndroidSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_android_surface_khr(&self, create_info: &AndroidSurfaceCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_android_surface.vkCreateAndroidSurfaceKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateAndroidSurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkCreateWin32SurfaceKHR)(VkInstance instance, const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_win32_surface_khr(&self, create_info: &Win32SurfaceCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().khr_win32_surface.vkCreateWin32SurfaceKHR(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateWin32SurfaceKHR", SurfaceKhrHandle(surface))
    }

    // *PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex);
    pub unsafe fn get_physical_device_win32_presentation_support_khr<Pd>(&self, physical_device: Pd,
            queue_family_index: u32) -> bool
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let result = self.proc_addr_loader().khr_win32_surface.vkGetPhysicalDeviceWin32PresentationSupportKHR(
            physical_device.handle().to_raw(), queue_family_index);
        result != 0
    }


    // *PFN_vkGetPhysicalDeviceFeatures2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2KHR* pFeatures);
    pub unsafe fn get_physical_device_features_2_khr<Pd>(&self, physical_device: Pd)
            -> PhysicalDeviceFeatures2Khr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut features = mem::uninitialized();
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceFeatures2KHR(
            physical_device.handle().to_raw(), &mut features);
        PhysicalDeviceFeatures2Khr::from_raw(features)
    }


    // *PFN_vkGetPhysicalDeviceProperties2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2KHR* pProperties);
    pub unsafe fn get_physical_device_properties_2_khr<Pd>(&self, physical_device: Pd)
            -> PhysicalDeviceProperties2Khr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut properties = mem::uninitialized();
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceProperties2KHR(
            physical_device.handle().to_raw(), &mut properties);
        PhysicalDeviceProperties2Khr::from_raw(properties)
    }

    // *PFN_vkGetPhysicalDeviceFormatProperties2KHR)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2KHR* pFormatProperties);
    pub unsafe fn get_physical_device_format_properties_2_khr<Pd>(&self, physical_device: Pd, format: Format)
            -> FormatProperties2Khr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut props: FormatProperties2Khr = mem::uninitialized();
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceFormatProperties2KHR(
            physical_device.handle().to_raw(),
            format.into(), &mut props as *mut _ as *mut vks::VkFormatProperties2KHR);
        props
    }

    // *PFN_vkGetPhysicalDeviceImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2KHR* pImageFormatInfo, VkImageFormatProperties2KHR* pImageFormatProperties);
    pub unsafe fn get_physical_device_image_format_properties_2_khr<Pd>(&self, physical_device: Pd,
            image_format_info: &PhysicalDeviceImageFormatInfo2Khr)
            -> VdResult<ImageFormatProperties2Khr>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut image_format_properties = mem::uninitialized();
        let result = self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceImageFormatProperties2KHR(
            physical_device.handle().to_raw(), image_format_info.as_raw(),
            &mut image_format_properties);
        error::check(result, "vkGetPhysicalDeviceImageFormatProperties2KHR",
            ImageFormatProperties2Khr::from_raw(image_format_properties))
    }

    // *PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2KHR* pQueueFamilyProperties);
    pub unsafe fn get_physical_device_queue_family_properties_2_khr<Pd>(&self, physical_device: Pd)
            -> VdResult<SmallVec<[QueueFamilyProperties2Khr; 16]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut queue_family_count = 0u32;
        let mut queue_families = SmallVec::<[QueueFamilyProperties2Khr; 16]>::new();
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceQueueFamilyProperties2KHR(
            physical_device.handle().to_raw(), &mut queue_family_count, ptr::null_mut());
        queue_families.reserve_exact(queue_family_count as usize);
        queue_families.set_len(queue_family_count as usize);
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceQueueFamilyProperties2KHR(
            physical_device.handle().to_raw(), &mut queue_family_count,
            queue_families.as_mut_ptr() as *mut vks::VkQueueFamilyProperties2KHR);
        Ok(queue_families)
    }

    // *PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2KHR* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2KHR* pProperties);
    pub unsafe fn get_physical_device_memory_properties_2_khr<Pd>(&self, physical_device: Pd)
            -> PhysicalDeviceMemoryProperties2Khr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut mem_props: vks::VkPhysicalDeviceMemoryProperties2KHR;
        mem_props = mem::uninitialized();
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceMemoryProperties2KHR(
            physical_device.handle().to_raw(), &mut mem_props);
        PhysicalDeviceMemoryProperties2Khr::from_raw(mem_props)
    }

    // typedef void (VKAPI_PTR *PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2KHR* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2KHR* pProperties);
    pub unsafe fn get_physical_device_sparse_image_format_properties_2_khr<Pd>(&self, physical_device: Pd,
            format_info: &PhysicalDeviceSparseImageFormatInfo2Khr)
            -> SmallVec<[SparseImageFormatProperties2Khr; 8]>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut property_count = 0u32;
        let mut properties = SmallVec::<[SparseImageFormatProperties2Khr; 8]>::new();
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
            physical_device.handle().to_raw(), format_info.as_raw(), &mut property_count,
            ptr::null_mut());
        properties.reserve_exact(property_count as usize);
        properties.set_len(property_count as usize);
        self.proc_addr_loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
            physical_device.handle().to_raw(), format_info.as_raw(), &mut property_count,
            properties.as_mut_ptr() as *mut vks::VkSparseImageFormatProperties2KHR);
        properties
    }

    // *PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalBufferInfoKHR* pExternalBufferInfo, VkExternalBufferPropertiesKHR* pExternalBufferProperties);
    pub unsafe fn get_physical_device_external_buffer_properties_khr<Pd>(&self,
            physical_device: Pd, external_buffer_info: &PhysicalDeviceExternalBufferInfoKhr)
            -> ExternalBufferPropertiesKhr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut external_buffer_properties = mem::uninitialized();
        self.proc_addr_loader().khr_external_memory_capabilities.vkGetPhysicalDeviceExternalBufferPropertiesKHR(
            physical_device.handle().to_raw(), external_buffer_info.as_raw(),
            &mut external_buffer_properties);
        ExternalBufferPropertiesKhr::from_raw(external_buffer_properties)
    }

    // *PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalSemaphoreInfoKHR* pExternalSemaphoreInfo, VkExternalSemaphorePropertiesKHR* pExternalSemaphoreProperties);
    pub unsafe fn get_physical_device_external_semaphore_properties_khr<Pd>(&self,
            physical_device: Pd, external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfoKhr)
            -> ExternalSemaphorePropertiesKhr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut external_semaphore_properties = mem::uninitialized();
        self.proc_addr_loader().khr_external_semaphore_capabilities.vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
            physical_device.handle().to_raw(), external_semaphore_info.as_raw(),
            &mut external_semaphore_properties);
        ExternalSemaphorePropertiesKhr::from_raw(external_semaphore_properties)
    }

    // *PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalFenceInfoKHR* pExternalFenceInfo, VkExternalFencePropertiesKHR* pExternalFenceProperties);
    pub unsafe fn get_physical_device_external_fence_properties_khr<Pd>(&self,
            physical_device: Pd, external_fence_info: &PhysicalDeviceExternalFenceInfoKhr)
            -> ExternalFencePropertiesKhr
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut external_fence_properties = mem::uninitialized();
        self.proc_addr_loader().khr_external_fence_capabilities.vkGetPhysicalDeviceExternalFencePropertiesKHR(
            physical_device.handle().to_raw(), external_fence_info.as_raw(),
            &mut external_fence_properties);
        ExternalFencePropertiesKhr::from_raw(external_fence_properties)
    }

    // *PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities);
    pub unsafe fn get_physical_device_surface_capabilities_2_khr<Pd>(&self, physical_device: Pd,
            surface_info: &PhysicalDeviceSurfaceInfo2Khr) -> VdResult<SurfaceCapabilities2Khr>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut capabilities = mem::uninitialized();
        let result = self.proc_addr_loader().khr_get_surface_capabilities2.vkGetPhysicalDeviceSurfaceCapabilities2KHR(
            physical_device.handle().to_raw(), surface_info.as_raw(), &mut capabilities);
        error::check(result, "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
            SurfaceCapabilities2Khr::from_raw(capabilities))
    }

    // *PFN_vkGetPhysicalDeviceSurfaceFormats2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats);
    pub unsafe fn get_physical_device_surface_formats_2_khr<Pd>(&self, physical_device: Pd,
            surface_info: &PhysicalDeviceSurfaceInfo2Khr) -> VdResult<SmallVec<[SurfaceFormat2Khr; 64]>>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut format_count = 0u32;
        let mut formats: SmallVec<[::SurfaceFormat2Khr; 64]> = SmallVec::new();
        let result = self.proc_addr_loader().khr_get_surface_capabilities2.vkGetPhysicalDeviceSurfaceFormats2KHR(
            physical_device.handle().to_raw(), surface_info.as_raw(), &mut format_count, ptr::null_mut());
        error::check(result, "vkGetPhysicalDeviceSurfaceFormats2KHR", ())?;
        formats.reserve_exact(format_count as usize);
        formats.set_len(format_count as usize);
        if format_count != 0 {
            loop {
                let result = self.proc_addr_loader().khr_get_surface_capabilities2.vkGetPhysicalDeviceSurfaceFormats2KHR(
                    physical_device.handle().to_raw(), surface_info.as_raw(), &mut format_count,
                    formats.as_mut_ptr() as *mut vks::VkSurfaceFormat2KHR);
                if result != CallResult::Incomplete as i32 {
                    return error::check(result, "vkGetPhysicalDeviceSurfaceFormats2KHR", formats);
                }
            }
        }
        Ok(formats)
    }

    // *PFN_vkCreateDebugReportCallbackEXT)(VkInstance instance, const VkDebugReportCallbackCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugReportCallbackEXT* pCallback);
    pub unsafe fn create_debug_report_callback_ext(&self,
            create_info: &DebugReportCallbackCreateInfoExt,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            ->  VdResult<DebugReportCallbackExtHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut callback = 0;
        let result = self.proc_addr_loader().ext_debug_report.vkCreateDebugReportCallbackEXT(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut callback);
        error::check(result, "vkCreateDebugReportCallbackEXT", DebugReportCallbackExtHandle(callback))
    }

    // *PFN_vkDestroyDebugReportCallbackEXT)(VkInstance instance, VkDebugReportCallbackEXT callback, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_debug_report_callback_ext(&self,
            callback: DebugReportCallbackExtHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().ext_debug_report.vkDestroyDebugReportCallbackEXT(
            self.handle().to_raw(), callback.to_raw(), allocator);
    }

    // *PFN_vkDebugReportMessageEXT)(VkInstance instance, VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT objectType, uint64_t object, size_t location, int32_t messageCode, const char* pLayerPrefix, const char* pMessage);
    pub unsafe fn debug_report_message_ext(&self, flags: DebugReportFlagsExt,
            object_type: DebugReportObjectTypeExt, object: u64, location: usize, message_code: i32,
            layer_prefix: &CStr, message: &CStr) {
        self.proc_addr_loader().ext_debug_report.vkDebugReportMessageEXT(self.handle().to_raw(), flags.bits(),
            object_type.into(), object, location, message_code, layer_prefix.as_ptr(),
            message.as_ptr());
    }

    // *PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkExternalMemoryHandleTypeFlagsNV externalHandleType, VkExternalImageFormatPropertiesNV* pExternalImageFormatProperties);
    pub unsafe fn get_physical_device_external_image_format_properties_nv<Pd>(&self,
            physical_device: Pd, format: Format, type_: ImageType, tiling: ImageTiling,
            usage: ImageUsageFlags, flags: ImageCreateFlags,
            external_handle_type: ExternalMemoryHandleTypeFlagsNv)
            -> VdResult<ExternalImageFormatPropertiesNv>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut external_image_format_properties = mem::uninitialized();
        let result = self.proc_addr_loader().nv_external_memory_capabilities.vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
            physical_device.handle().to_raw(), format.into(), type_.into(),
            tiling.into(), usage.bits(), flags.bits(), external_handle_type.bits(),
            &mut external_image_format_properties);
        error::check(result, "vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
            ExternalImageFormatPropertiesNv::from_raw(external_image_format_properties))
    }

    // *PFN_vkGetPhysicalDevicePresentRectanglesKHX)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects);
    #[cfg(feature = "experimental")]
    pub unsafe fn get_physical_device_present_rectangles_khx<Pd, S>(&self,
            physical_device: Pd, surface: S)
            -> VdResult<SmallVec<[Rect2d; 8]>>
            where Pd: Handle<Target=PhysicalDeviceHandle>, S: Handle<Target=SurfaceKhrHandle> {
        let mut rect_count = 0u32;
        let mut rects: SmallVec<[Rect2d; 8]> = SmallVec::new();
        let result = self.proc_addr_loader().vkGetPhysicalDevicePresentRectanglesKHX(
            physical_device.handle().to_raw(), surface.handle().to_raw(), &mut rect_count,
            ptr::null_mut());
        error::check(result, "vkGetPhysicalDevicePresentRectanglesKHX", ())?;
        rects.reserve_exact(rect_count as usize);
        rects.set_len(rect_count as usize);
        loop {
            let result = self.proc_addr_loader().vkGetPhysicalDevicePresentRectanglesKHX(
                physical_device.handle().to_raw(), surface.handle().to_raw(), &mut rect_count,
                rects.as_mut_ptr() as *mut vks::VkRect2D);
            if result != CallResult::Incomplete as i32 {
                return error::check(result, "vkGetPhysicalDevicePresentRectanglesKHX", rects);
            }
        }
        Ok(rects)
    }

    // *PFN_vkCreateViSurfaceNN)(VkInstance instance, const VkViSurfaceCreateInfoNN* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_vi_surface_nn(&self, create_info: &ViSurfaceCreateInfoNn,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().nn_vi_surface.vkCreateViSurfaceNN(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateViSurfaceNN", SurfaceKhrHandle(surface))
    }

    // *PFN_vkEnumeratePhysicalDeviceGroupsKHX)(VkInstance instance, uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupPropertiesKHX* pPhysicalDeviceGroupProperties);
    #[cfg(feature = "experimental")]
    pub unsafe fn enumerate_physical_device_groups_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)(VkPhysicalDevice physicalDevice, VkDeviceGeneratedCommandsFeaturesNVX* pFeatures, VkDeviceGeneratedCommandsLimitsNVX* pLimits);
    #[cfg(feature = "experimental")]
    pub unsafe fn get_physical_device_generated_commands_properties_nvx<Pd>(&self) {
        unimplemented!();
    }

    // *PFN_vkReleaseDisplayEXT)(VkPhysicalDevice physicalDevice, VkDisplayKHR display);
    pub unsafe fn release_display_ext<Pd, D>(&self, physical_device: Pd, display: D)
            -> VdResult<()>
            where Pd: Handle<Target=PhysicalDeviceHandle>, D: Handle<Target=DisplayKhrHandle> {
        let result = self.proc_addr_loader().ext_direct_mode_display.vkReleaseDisplayEXT(
            physical_device.handle().to_raw(), display.handle().to_raw());
        error::check(result, "vkReleaseDisplayEXT", ())
    }

    // *PFN_vkAcquireXlibDisplayEXT)(VkPhysicalDevice physicalDevice, Display* dpy, VkDisplayKHR display);
    pub unsafe fn acquire_xlib_display_ext<Pd, D>(&self, physical_device: Pd, dpy: *mut Display, display: D)
            -> VdResult<()>
            where Pd: Handle<Target=PhysicalDeviceHandle>, D: Handle<Target=DisplayKhrHandle> {
        let result = self.proc_addr_loader().ext_acquire_xlib_display.vkAcquireXlibDisplayEXT(
            physical_device.handle().to_raw(), dpy, display.handle().to_raw());
        error::check(result, "vkAcquireXlibDisplayEXT", ())
    }

    // *PFN_vkGetRandROutputDisplayEXT)(VkPhysicalDevice physicalDevice, Display* dpy, RROutput rrOutput, VkDisplayKHR* pDisplay);
    pub unsafe fn get_rand_r_output_display_ext<Pd, D>(&self, physical_device: Pd,
            dpy: *mut Display, rr_output: RROutput) -> VdResult<DisplayKhrHandle>
            where Pd: Handle<Target=PhysicalDeviceHandle>, D: Handle<Target=DisplayKhrHandle> {
        let mut display = 0;
        let result = self.proc_addr_loader().ext_acquire_xlib_display.vkGetRandROutputDisplayEXT(physical_device.handle().to_raw(),
            dpy, rr_output, &mut display);
        error::check(result, "vkGetRandROutputDisplayEXT", DisplayKhrHandle(display))
    }

    // *PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilities2EXT* pSurfaceCapabilities);
    pub unsafe fn get_physical_device_surface_capabilities_2_ext<Pd, S>(&self, physical_device: Pd,
            surface: S) -> VdResult<SurfaceCapabilities2Ext>
            where Pd: Handle<Target=PhysicalDeviceHandle>, S: Handle<Target=SurfaceKhrHandle> {
        let mut surface_capabilities = mem::uninitialized();
        let result = self.proc_addr_loader().ext_display_surface_counter.vkGetPhysicalDeviceSurfaceCapabilities2EXT(
            physical_device.handle().to_raw(), surface.handle().to_raw(),
            &mut surface_capabilities as *mut _ as *mut vks::VkSurfaceCapabilities2EXT);
        error::check(result, "vkGetPhysicalDeviceSurfaceCapabilities2EXT", surface_capabilities)
    }

    // *PFN_vkCreateIOSSurfaceMVK)(VkInstance instance, const VkIOSSurfaceCreateInfoMVK* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_ios_surface_mvk(&self, create_info: &IosSurfaceCreateInfoMvk,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().mvk_ios_surface.vkCreateIOSSurfaceMVK(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateIOSSurfaceMVK", SurfaceKhrHandle(surface))
    }

    // *PFN_vkCreateMacOSSurfaceMVK)(VkInstance instance, const VkMacOSSurfaceCreateInfoMVK* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);
    pub unsafe fn create_mac_os_surface_mvk(&self, create_info: &MacOsSurfaceCreateInfoMvk,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SurfaceKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut surface = 0;
        let result = self.proc_addr_loader().mvk_macos_surface.vkCreateMacOSSurfaceMVK(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut surface);
        error::check(result, "vkCreateMacOSSurfaceMVK", SurfaceKhrHandle(surface))
    }

    // *PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT)(VkPhysicalDevice physicalDevice, VkSampleCountFlagBits samples, VkMultisamplePropertiesEXT* pMultisampleProperties);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn get_physical_device_multisample_properties_ext<Pd>(&self, physical_device: Pd,
            samples: SampleCountFlags)
            -> VdResult<MultisamplePropertiesExt>
            where Pd: Handle<Target=PhysicalDeviceHandle> {
        let mut multisample_properties = mem::uninitialized();
        let result = self.proc_addr_loader().vkGetPhysicalDeviceMultisamplePropertiesEXT(
            physical_device.handle().to_raw(), samples.bits(),
            multisample_properties as *mut _ as *mut vks::VkMultisamplePropertiesEXT);
        error::check(result, "vkGetPhysicalDeviceMultisamplePropertiesEXT", multisample_properties)
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
    print_debug_report_enable: bool,
    _p: PhantomData<&'ib ()>,
}

impl<'ib> InstanceBuilder<'ib> {
    /// Returns a new instance builder.
    pub fn new() -> InstanceBuilder<'ib> {
        InstanceBuilder {
            create_info: vks::VkInstanceCreateInfo::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            print_debug_report_enable: false,
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

    /// Creates a debug report callback which prints debug messages to stdout.
    ///
    /// If the `VK_EXT_debug_report` extension is not listed among the enabled
    /// extensions, a warning message will be printed once instead.
    ///
    /// If the appropriate validation layers are not enabled, no messages will
    /// be printed even if debug report printing is enabled.
    ///
    pub fn print_debug_report<'s>(&'s mut self, enable: bool) -> &'s mut InstanceBuilder<'ib> {
        self.print_debug_report_enable = enable;
        self
    }

    /// Builds and returns a new `Instance`.
    pub fn build(&self, mut loader: Loader) -> VdResult<Instance> {
        let mut handle = ptr::null_mut();
        let mut enable_debug_callback = false;

        unsafe {
            loader.core_global().vkCreateInstance(&self.create_info, ptr::null(), &mut handle);
            loader.instance_proc_addr_loader_mut().load_core(handle);
        }

        unsafe {
            if let Some(extension_name_char_strs) = self.enabled_extension_names.as_ref() {
                let extension_names = extension_name_char_strs.as_ptr_slice();
                for &extension_name in extension_names {
                    match CStr::from_ptr(extension_name).to_str().expect("invalid extension name") {
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_16bit_storage" => loader.instance_proc_addr_loader_mut().load_khr_16bit_storage(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_android_surface" => loader.instance_proc_addr_loader_mut().load_khr_android_surface(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_bind_memory2" => loader.instance_proc_addr_loader_mut().load_khr_bind_memory2(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_dedicated_allocation" => loader.instance_proc_addr_loader_mut().load_khr_dedicated_allocation(handle),
                        "VK_KHR_descriptor_update_template" => loader.instance_proc_addr_loader_mut().load_khr_descriptor_update_template(handle),
                        "VK_KHR_display" => loader.instance_proc_addr_loader_mut().load_khr_display(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_display_swapchain" => loader.instance_proc_addr_loader_mut().load_khr_display_swapchain(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_fence" => loader.instance_proc_addr_loader_mut().load_khr_external_fence(handle),
                        "VK_KHR_external_fence_capabilities" => loader.instance_proc_addr_loader_mut().load_khr_external_fence_capabilities(handle),
                        "VK_KHR_external_fence_fd" => loader.instance_proc_addr_loader_mut().load_khr_external_fence_fd(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_fence_win32" => loader.instance_proc_addr_loader_mut().load_khr_external_fence_win32(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_memory" => loader.instance_proc_addr_loader_mut().load_khr_external_memory(handle),
                        "VK_KHR_external_memory_capabilities" => loader.instance_proc_addr_loader_mut().load_khr_external_memory_capabilities(handle),
                        "VK_KHR_external_memory_fd" => loader.instance_proc_addr_loader_mut().load_khr_external_memory_fd(handle),
                        "VK_KHR_external_memory_win32" => loader.instance_proc_addr_loader_mut().load_khr_external_memory_win32(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_semaphore" => loader.instance_proc_addr_loader_mut().load_khr_external_semaphore(handle),
                        "VK_KHR_external_semaphore_capabilities" => loader.instance_proc_addr_loader_mut().load_khr_external_semaphore_capabilities(handle),
                        "VK_KHR_external_semaphore_fd" => loader.instance_proc_addr_loader_mut().load_khr_external_semaphore_fd(handle),
                        "VK_KHR_external_semaphore_win32" => loader.instance_proc_addr_loader_mut().load_khr_external_semaphore_win32(handle),
                        "VK_KHR_get_memory_requirements2" => loader.instance_proc_addr_loader_mut().load_khr_get_memory_requirements2(handle),
                        "VK_KHR_get_physical_device_properties2" => loader.instance_proc_addr_loader_mut().load_khr_get_physical_device_properties2(handle),
                        "VK_KHR_get_surface_capabilities2" => loader.instance_proc_addr_loader_mut().load_khr_get_surface_capabilities2(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_image_format_list" => loader.instance_proc_addr_loader_mut().load_khr_image_format_list(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_incremental_present" => loader.instance_proc_addr_loader_mut().load_khr_incremental_present(handle),
                        "VK_KHR_maintenance1" => loader.instance_proc_addr_loader_mut().load_khr_maintenance1(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_maintenance2" => loader.instance_proc_addr_loader_mut().load_khr_maintenance2(handle),
                        "VK_KHR_mir_surface" => loader.instance_proc_addr_loader_mut().load_khr_mir_surface(handle),
                        "VK_KHR_push_descriptor" => loader.instance_proc_addr_loader_mut().load_khr_push_descriptor(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_relaxed_block_layout" => loader.instance_proc_addr_loader_mut().load_khr_relaxed_block_layout(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_sampler_mirror_clamp_to_edge" => loader.instance_proc_addr_loader_mut().load_khr_sampler_mirror_clamp_to_edge(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_sampler_ycbcr_conversion" => loader.instance_proc_addr_loader_mut().load_khr_sampler_ycbcr_conversion(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_shader_draw_parameters" => loader.instance_proc_addr_loader_mut().load_khr_shader_draw_parameters(handle),
                        "VK_KHR_shared_presentable_image" => loader.instance_proc_addr_loader_mut().load_khr_shared_presentable_image(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_storage_buffer_storage_class" => loader.instance_proc_addr_loader_mut().load_khr_storage_buffer_storage_class(handle),
                        "VK_KHR_surface" => loader.instance_proc_addr_loader_mut().load_khr_surface(handle),
                        "VK_KHR_swapchain" => loader.instance_proc_addr_loader_mut().load_khr_swapchain(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_variable_pointers" => loader.instance_proc_addr_loader_mut().load_khr_variable_pointers(handle),
                        "VK_KHR_wayland_surface" => loader.instance_proc_addr_loader_mut().load_khr_wayland_surface(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_win32_keyed_mutex" => loader.instance_proc_addr_loader_mut().load_khr_win32_keyed_mutex(handle),
                        "VK_KHR_win32_surface" => loader.instance_proc_addr_loader_mut().load_khr_win32_surface(handle),
                        "VK_KHR_xcb_surface" => loader.instance_proc_addr_loader_mut().load_khr_xcb_surface(handle),
                        "VK_KHR_xlib_surface" => loader.instance_proc_addr_loader_mut().load_khr_xlib_surface(handle),
                        "VK_EXT_acquire_xlib_display" => loader.instance_proc_addr_loader_mut().load_ext_acquire_xlib_display(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_blend_operation_advanced" => loader.instance_proc_addr_loader_mut().load_ext_blend_operation_advanced(handle),
                        "VK_EXT_debug_marker" => loader.instance_proc_addr_loader_mut().load_ext_debug_marker(handle),
                        "VK_EXT_debug_report" => {
                            loader.instance_proc_addr_loader_mut().load_ext_debug_report(handle);
                            enable_debug_callback = true;
                        },
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_depth_range_unrestricted" => loader.instance_proc_addr_loader_mut().load_ext_depth_range_unrestricted(handle),
                        "VK_EXT_direct_mode_display" => loader.instance_proc_addr_loader_mut().load_ext_direct_mode_display(handle),
                        "VK_EXT_discard_rectangles" => loader.instance_proc_addr_loader_mut().load_ext_discard_rectangles(handle),
                        "VK_EXT_display_control" => loader.instance_proc_addr_loader_mut().load_ext_display_control(handle),
                        "VK_EXT_display_surface_counter" => loader.instance_proc_addr_loader_mut().load_ext_display_surface_counter(handle),
                        "VK_EXT_hdr_metadata" => loader.instance_proc_addr_loader_mut().load_ext_hdr_metadata(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_post_depth_coverage" => loader.instance_proc_addr_loader_mut().load_ext_post_depth_coverage(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_sample_locations" => loader.instance_proc_addr_loader_mut().load_ext_sample_locations(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_sampler_filter_minmax" => loader.instance_proc_addr_loader_mut().load_ext_sampler_filter_minmax(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_stencil_export" => loader.instance_proc_addr_loader_mut().load_ext_shader_stencil_export(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_subgroup_ballot" => loader.instance_proc_addr_loader_mut().load_ext_shader_subgroup_ballot(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_subgroup_vote" => loader.instance_proc_addr_loader_mut().load_ext_shader_subgroup_vote(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_viewport_index_layer" => loader.instance_proc_addr_loader_mut().load_ext_shader_viewport_index_layer(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_swapchain_colorspace" => loader.instance_proc_addr_loader_mut().load_ext_swapchain_colorspace(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_validation_cache" => loader.instance_proc_addr_loader_mut().load_ext_validation_cache(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_validation_flags" => loader.instance_proc_addr_loader_mut().load_ext_validation_flags(handle),
                        "VK_AMD_draw_indirect_count" => loader.instance_proc_addr_loader_mut().load_amd_draw_indirect_count(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_gcn_shader" => loader.instance_proc_addr_loader_mut().load_amd_gcn_shader(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_gpu_shader_half_float" => loader.instance_proc_addr_loader_mut().load_amd_gpu_shader_half_float(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_gpu_shader_int16" => loader.instance_proc_addr_loader_mut().load_amd_gpu_shader_int16(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_mixed_attachment_samples" => loader.instance_proc_addr_loader_mut().load_amd_mixed_attachment_samples(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_negative_viewport_height" => loader.instance_proc_addr_loader_mut().load_amd_negative_viewport_height(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_rasterization_order" => loader.instance_proc_addr_loader_mut().load_amd_rasterization_order(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_ballot" => loader.instance_proc_addr_loader_mut().load_amd_shader_ballot(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_explicit_vertex_parameter" => loader.instance_proc_addr_loader_mut().load_amd_shader_explicit_vertex_parameter(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_fragment_mask" => loader.instance_proc_addr_loader_mut().load_amd_shader_fragment_mask(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_image_load_store_lod" => loader.instance_proc_addr_loader_mut().load_amd_shader_image_load_store_lod(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_trinary_minmax" => loader.instance_proc_addr_loader_mut().load_amd_shader_trinary_minmax(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_texture_gather_bias_lod" => loader.instance_proc_addr_loader_mut().load_amd_texture_gather_bias_lod(handle),

                        "VK_GOOGLE_display_timing" => loader.instance_proc_addr_loader_mut().load_google_display_timing(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_IMG_filter_cubic" => loader.instance_proc_addr_loader_mut().load_img_filter_cubic(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_IMG_format_pvrtc" => loader.instance_proc_addr_loader_mut().load_img_format_pvrtc(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHX_device_group" => loader.instance_proc_addr_loader_mut().load_khx_device_group(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHX_device_group_creation" => loader.instance_proc_addr_loader_mut().load_khx_device_group_creation(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHX_multiview" => loader.instance_proc_addr_loader_mut().load_khx_multiview(handle),
                        "VK_MVK_ios_surface" => loader.instance_proc_addr_loader_mut().load_mvk_ios_surface(handle),
                        "VK_MVK_macos_surface" => loader.instance_proc_addr_loader_mut().load_mvk_macos_surface(handle),
                        "VK_NN_vi_surface" => loader.instance_proc_addr_loader_mut().load_nn_vi_surface(handle),
                        "VK_NV_clip_space_w_scaling" => loader.instance_proc_addr_loader_mut().load_nv_clip_space_w_scaling(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_dedicated_allocation" => loader.instance_proc_addr_loader_mut().load_nv_dedicated_allocation(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_external_memory" => loader.instance_proc_addr_loader_mut().load_nv_external_memory(handle),
                        "VK_NV_external_memory_capabilities" => loader.instance_proc_addr_loader_mut().load_nv_external_memory_capabilities(handle),
                        "VK_NV_external_memory_win32" => loader.instance_proc_addr_loader_mut().load_nv_external_memory_win32(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_fill_rectangle" => loader.instance_proc_addr_loader_mut().load_nv_fill_rectangle(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_fragment_coverage_to_color" => loader.instance_proc_addr_loader_mut().load_nv_fragment_coverage_to_color(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_framebuffer_mixed_samples" => loader.instance_proc_addr_loader_mut().load_nv_framebuffer_mixed_samples(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_geometry_shader_passthrough" => loader.instance_proc_addr_loader_mut().load_nv_geometry_shader_passthrough(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_glsl_shader" => loader.instance_proc_addr_loader_mut().load_nv_glsl_shader(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_sample_mask_override_coverage" => loader.instance_proc_addr_loader_mut().load_nv_sample_mask_override_coverage(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_viewport_array2" => loader.instance_proc_addr_loader_mut().load_nv_viewport_array2(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_viewport_swizzle" => loader.instance_proc_addr_loader_mut().load_nv_viewport_swizzle(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_win32_keyed_mutex" => loader.instance_proc_addr_loader_mut().load_nv_win32_keyed_mutex(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NVX_device_generated_commands" => loader.instance_proc_addr_loader_mut().load_nvx_device_generated_commands(handle),
                        #[cfg(feature = "unimplemented")]
                        "VK_NVX_multiview_per_view_attributes" => loader.instance_proc_addr_loader_mut().load_nvx_multiview_per_view_attributes(handle),
                        &_ => (),
                    }
                }
            }
        }

        let debug_callback = if self.print_debug_report_enable {
            if enable_debug_callback {
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
                    println!("DEBUG_REPORT: Debug report printing enabled.");
                }
                Some(callback)
            } else {
                println!("DEBUG_REPORT: WARNING: Debug report printing requested but the \
                    'VK_EXT_debug_report' extension is not loaded. \
                    Debug report printing is NOT enabled.");
                None
            }
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
