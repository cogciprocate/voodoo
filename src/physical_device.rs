
use std::ffi::CStr;
use smallvec::SmallVec;
use vks;
use ::{PRINT, VdResult, Instance, Handle, SurfaceFormatKhr, PhysicalDeviceFeatures,
    PhysicalDeviceProperties, QueueFamilyProperties, PhysicalDeviceMemoryProperties,
    ExtensionProperties, SurfaceCapabilitiesKhr, PresentModeKhr, FormatProperties, Format,
    SurfaceKhr, CharStrs};




#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PhysicalDeviceHandle(pub(crate) vks::VkPhysicalDevice);

impl PhysicalDeviceHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkPhysicalDevice {
        self.0
    }
}

unsafe impl Handle for PhysicalDeviceHandle {
    type Target = PhysicalDeviceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    handle: PhysicalDeviceHandle,
    instance: Instance
}

impl PhysicalDevice {
    pub fn from_parts(instance: Instance, handle: PhysicalDeviceHandle) -> PhysicalDevice {
        PhysicalDevice {
            handle,
            instance,
        }
    }

    #[inline]
    pub fn handle(&self) -> PhysicalDeviceHandle {
        self.handle
    }

    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    #[inline]
    pub fn features(&self) -> PhysicalDeviceFeatures {
        self.instance().get_physical_device_features(self)
    }

    #[inline]
    pub fn format_properties(&self, format: Format) -> FormatProperties {
        self.instance().get_physical_device_format_properties(self, format)
    }

    #[inline]
    pub fn properties(&self) -> PhysicalDeviceProperties {
        self.instance().get_physical_device_properties(self)
    }

    #[inline]
    pub fn queue_family_properties(&self) -> VdResult<SmallVec<[QueueFamilyProperties; 16]>> {
        self.instance().get_physical_device_queue_family_properties(self)
    }

    /// Returns the memory properties for this device.
    #[inline]
    pub fn memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        self.instance().get_physical_device_memory_properties(self)
    }

    #[inline]
    pub fn extension_properties(&self) -> VdResult<SmallVec<[ExtensionProperties; 64]>> {
        self.instance().enumerate_device_extension_properties(self, None)

    }

    #[inline]
    pub fn surface_support_khr(&self, queue_family_index: u32, surface: &SurfaceKhr)
            -> VdResult<bool> {
        unsafe { self.instance().get_physical_device_surface_support_khr(self, queue_family_index, surface) }
    }

    #[inline]
    pub fn surface_capabilities_khr(&self, surface: &SurfaceKhr) -> VdResult<SurfaceCapabilitiesKhr> {
        unsafe { self.instance().get_physical_device_surface_capabilities_khr(self, surface) }
    }

    #[inline]
    pub fn surface_formats_khr(&self, surface: &SurfaceKhr) -> VdResult<SmallVec<[SurfaceFormatKhr; 64]>> {
        unsafe { self.instance().get_physical_device_surface_formats_khr(self, surface) }
    }

    #[inline]
    pub fn surface_present_modes_khr(&self, surface: &SurfaceKhr) -> VdResult<SmallVec<[PresentModeKhr; 16]>> {
        unsafe { self.instance().get_physical_device_surface_present_modes_khr(self, surface) }
    }


    /// Verifies that the extensions listed are supported by this physical device.
    #[inline]
    pub fn verify_extension_availability<'a, 'cs, Cs>(&'a self, extension_names: Cs) -> VdResult<bool>
            where 'cs: 'a, Cs: 'cs + Into<CharStrs<'cs>> {
        let avail_exts = self.extension_properties()?;
        unsafe {
            // Print available:
            for ext in &avail_exts {
                    if PRINT { println!("Available device extension: '{}' (version: {})",
                        ext.extension_name().to_str().unwrap(), ext.spec_version()); }
            };

            for &reqd_ext_name in extension_names.into().as_ptr_slice() {
                let mut ext_avail = false;
                for avail_ext in &avail_exts {
                    if CStr::from_ptr(reqd_ext_name) == avail_ext.extension_name() {
                        if PRINT { println!("Required device extension available: '{}'",
                            CStr::from_ptr(reqd_ext_name).to_str().unwrap()); }
                        ext_avail = true;
                        break;
                    }
                }
                if !ext_avail { return Ok(false); }
            }
        }
        Ok(true)
    }

}

unsafe impl<'p> Handle for &'p PhysicalDevice {
    type Target = PhysicalDeviceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.handle
    }
}

