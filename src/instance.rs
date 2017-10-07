use std::sync::Arc;
use std::ffi::{self, CStr};
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use smallvec::SmallVec;
use libc::{c_char, c_void};
use vks;
use ::{VooResult, Loader, ApplicationInfo, PhysicalDeviceHandle, PhysicalDevice, CharStrs,
    PRINT, FormatProperties, Format, Handle};


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
    pub fn raw(&self) -> vks::VkInstance {
        self.0
    }
}

impl Handle for InstanceHandle {
    type Target = InstanceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


fn enumerate_physical_devices(instance: InstanceHandle, loader: &vks::InstanceProcAddrLoader)
        -> SmallVec<[vks::VkPhysicalDevice; 16]> {
    let mut device_count = 0;
    let mut devices_raw = SmallVec::new();
    unsafe {
        ::check(loader.core.vkEnumeratePhysicalDevices(instance.0, &mut device_count, ptr::null_mut()));
        if device_count == 0 { panic!("No physical devices found."); }
        assert!(device_count as usize <= devices_raw.inline_size());
        devices_raw.set_len(device_count as usize);
        ::check(loader.core.vkEnumeratePhysicalDevices(instance.0, &mut device_count, devices_raw.as_mut_ptr()));
    }
    if PRINT { println!("Available devices: {:?}", devices_raw); }
    devices_raw
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

    #[inline]
    pub fn proc_addr_loader(&self) -> &vks::InstanceProcAddrLoader {
        self.inner.loader.loader()
    }

    #[inline]
    pub fn handle(&self) -> InstanceHandle {
        self.inner.handle
    }

    #[inline]
    pub fn loader(&self) -> &Loader {
        &self.inner.loader
    }

    #[inline]
    pub fn physical_devices(&self) -> SmallVec<[PhysicalDevice; 16]> {
        enumerate_physical_devices(self.inner.handle, self.inner.loader.loader())
            .iter().map(|&pdr| PhysicalDevice::new(self.clone(), PhysicalDeviceHandle(pdr)))
            .collect()
    }

    pub fn physical_device_format_properties(&self, physical_device: &PhysicalDevice,
            format: Format) -> FormatProperties {
        unsafe {
            let mut props: FormatProperties = mem::uninitialized();
            self.proc_addr_loader().vkGetPhysicalDeviceFormatProperties(
                physical_device.handle().0, format.into(),
                &mut props as *mut _ as *mut vks::VkFormatProperties);
            props
        }
    }

                // *PFN_vkCreateInstance)(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance);

                // *PFN_vkDestroyInstance)(VkInstance instance, const VkAllocationCallbacks* pAllocator);

                // *PFN_vkEnumeratePhysicalDevices)(VkInstance instance, uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices);

                // *PFN_vkGetPhysicalDeviceFeatures)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures);

                // *PFN_vkGetPhysicalDeviceFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties);

                // *PFN_vkGetPhysicalDeviceImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties);

                // *PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties);

                // *PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties);

                // *PFN_vkGetPhysicalDeviceMemoryProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties);

                // *PFN_vkCreateDevice)(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice);

                // *PFN_vkDestroyDevice)(VkDevice device, const VkAllocationCallbacks* pAllocator);

                // *PFN_vkEnumerateDeviceExtensionProperties)(VkPhysicalDevice physicalDevice, const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties);

                // *PFN_vkEnumerateDeviceLayerProperties)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties);

                // *PFN_vkGetPhysicalDeviceSparseImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties);

                // *PFN_vkDestroySurfaceKHR)(VkInstance instance, VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator);

                // *PFN_vkGetPhysicalDeviceSurfaceSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported);

                // *PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities);

                // *PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats);

                // *PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes);

                // *PFN_vkGetPhysicalDeviceDisplayPropertiesKHR)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPropertiesKHR* pProperties);

                // *PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR)(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPlanePropertiesKHR* pProperties);

                // *PFN_vkGetDisplayPlaneSupportedDisplaysKHR)(VkPhysicalDevice physicalDevice, uint32_t planeIndex, uint32_t* pDisplayCount, VkDisplayKHR* pDisplays);

                // *PFN_vkGetDisplayModePropertiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, uint32_t* pPropertyCount, VkDisplayModePropertiesKHR* pProperties);

                // *PFN_vkCreateDisplayModeKHR)(VkPhysicalDevice physicalDevice, VkDisplayKHR display, const VkDisplayModeCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDisplayModeKHR* pMode);

                // *PFN_vkGetDisplayPlaneCapabilitiesKHR)(VkPhysicalDevice physicalDevice, VkDisplayModeKHR mode, uint32_t planeIndex, VkDisplayPlaneCapabilitiesKHR* pCapabilities);

                // *PFN_vkCreateDisplayPlaneSurfaceKHR)(VkInstance instance, const VkDisplaySurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkCreateXlibSurfaceKHR)(VkInstance instance, const VkXlibSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkCreateXlibSurfaceKHR)(VkInstance instance, const VkXlibSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, Display* dpy, VisualID visualID);

                // *PFN_vkCreateXcbSurfaceKHR)(VkInstance instance, const VkXcbSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, xcb_connection_t* connection, xcb_visualid_t visual_id);

                // *PFN_vkCreateWaylandSurfaceKHR)(VkInstance instance, const VkWaylandSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, struct wl_display* display);

                // *PFN_vkCreateMirSurfaceKHR)(VkInstance instance, const VkMirSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkGetPhysicalDeviceMirPresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, MirConnection* connection);

                // *PFN_vkCreateAndroidSurfaceKHR)(VkInstance instance, const VkAndroidSurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkCreateWin32SurfaceKHR)(VkInstance instance, const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR)(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex);

                // *PFN_vkGetPhysicalDeviceFeatures2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2KHR* pFeatures);

                // *PFN_vkGetPhysicalDeviceProperties2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2KHR* pProperties);

                // *PFN_vkGetPhysicalDeviceFormatProperties2KHR)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2KHR* pFormatProperties);

                // *PFN_vkGetPhysicalDeviceImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2KHR* pImageFormatInfo, VkImageFormatProperties2KHR* pImageFormatProperties);

                // *PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR)(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2KHR* pQueueFamilyProperties);

                // *PFN_vkGetPhysicalDeviceMemoryProperties2KHR)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2KHR* pMemoryProperties);

                // *PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2KHR* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2KHR* pProperties);

                // *PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalBufferInfoKHR* pExternalBufferInfo, VkExternalBufferPropertiesKHR* pExternalBufferProperties);

                // *PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalSemaphoreInfoKHR* pExternalSemaphoreInfo, VkExternalSemaphorePropertiesKHR* pExternalSemaphoreProperties);

                // *PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalFenceInfoKHR* pExternalFenceInfo, VkExternalFencePropertiesKHR* pExternalFenceProperties);

                // *PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities);

                // *PFN_vkGetPhysicalDeviceSurfaceFormats2KHR)(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats);

                // *PFN_vkCreateDebugReportCallbackEXT)(VkInstance instance, const VkDebugReportCallbackCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugReportCallbackEXT* pCallback);

                // *PFN_vkDestroyDebugReportCallbackEXT)(VkInstance instance, VkDebugReportCallbackEXT callback, const VkAllocationCallbacks* pAllocator);

                // *PFN_vkDebugReportMessageEXT)(VkInstance instance, VkDebugReportFlagsEXT flags, VkDebugReportObjectTypeEXT objectType, uint64_t object, size_t location, int32_t messageCode, const char* pLayerPrefix, const char* pMessage);

                // *PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkExternalMemoryHandleTypeFlagsNV externalHandleType, VkExternalImageFormatPropertiesNV* pExternalImageFormatProperties);

                // *PFN_vkGetPhysicalDevicePresentRectanglesKHX)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects);

                // *PFN_vkCreateViSurfaceNN)(VkInstance instance, const VkViSurfaceCreateInfoNN* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkEnumeratePhysicalDeviceGroupsKHX)(VkInstance instance, uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupPropertiesKHX* pPhysicalDeviceGroupProperties);

                // *PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX)(VkPhysicalDevice physicalDevice, VkDeviceGeneratedCommandsFeaturesNVX* pFeatures, VkDeviceGeneratedCommandsLimitsNVX* pLimits);

                // *PFN_vkReleaseDisplayEXT)(VkPhysicalDevice physicalDevice, VkDisplayKHR display);

                // *PFN_vkAcquireXlibDisplayEXT)(VkPhysicalDevice physicalDevice, Display* dpy, VkDisplayKHR display);

                // *PFN_vkGetRandROutputDisplayEXT)(VkPhysicalDevice physicalDevice, Display* dpy, RROutput rrOutput, VkDisplayKHR* pDisplay);

                // *PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT)(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilities2EXT* pSurfaceCapabilities);

                // *PFN_vkCreateIOSSurfaceMVK)(VkInstance instance, const VkIOSSurfaceCreateInfoMVK* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkCreateMacOSSurfaceMVK)(VkInstance instance, const VkMacOSSurfaceCreateInfoMVK* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface);

                // *PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT)(VkPhysicalDevice physicalDevice, VkSampleCountFlagBits samples, VkMultisamplePropertiesEXT* pMultisampleProperties);
}

impl<'h> Handle for &'h Instance {
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
                self.loader.loader().ext_debug_report.vkDestroyDebugReportCallbackEXT(self.handle.0,
                    callback, ptr::null());
            }

            if PRINT { println!("Destroying instance..."); }
            self.loader.loader().core.vkDestroyInstance(self.handle.0, ptr::null());
        }
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}



/// A builder used to create an `Instance`.
//
// typedef struct VkInstanceCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkInstanceCreateFlags       flags;
//     const VkApplicationInfo*    pApplicationInfo;
//     uint32_t                    enabledLayerCount;
//     const char* const*          ppEnabledLayerNames;
//     uint32_t                    enabledExtensionCount;
//     const char* const*          ppEnabledExtensionNames;
// } VkInstanceCreateInfo;
//
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
            loader.loader_mut().load_core(handle);
            loader.loader_mut().load_khr_surface(handle);
            loader.loader_mut().load_khr_win32_surface(handle);
            loader.loader_mut().load_khr_get_physical_device_properties2(handle);
            loader.loader_mut().load_khr_external_memory_capabilities(handle);
        }

        // TODO: Ensure that the debug extension is enabled by consulting the
        // enabled extension list instead.
        if enable_debug_callback { unsafe { loader.loader_mut().load_ext_debug_report(handle); } }

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
            if unsafe { loader.loader().ext_debug_report.vkCreateDebugReportCallbackEXT(handle,
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
