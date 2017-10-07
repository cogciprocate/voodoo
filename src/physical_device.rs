use std::sync::Arc;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use libc::c_char;
use smallvec::SmallVec;
use vks;
use ::{VooResult, Instance, SurfaceKhrHandle, SurfaceKhr, SwapchainSupportDetails, PRINT, Handle, SurfaceFormatKhr,
    PhysicalDeviceFeatures, PhysicalDeviceProperties, QueueFamilyProperties,
    PhysicalDeviceMemoryProperties, ExtensionProperties, SurfaceCapabilitiesKhr, PresentModeKhr,
    FormatProperties, Format};
use queue::{self, Queue};
use instance;



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PhysicalDeviceHandle(pub(crate) vks::VkPhysicalDevice);

impl PhysicalDeviceHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkPhysicalDevice {
        self.0
    }
}

impl Handle for PhysicalDeviceHandle {
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
    pub fn new(instance: Instance, handle: PhysicalDeviceHandle) -> PhysicalDevice {
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
    pub fn features(&self) -> VooResult<PhysicalDeviceFeatures> {
        self.instance().get_physical_device_features(self)
    }

    #[inline]
    pub fn format_properties(&self, format: Format) -> VooResult<FormatProperties> {
        self.instance().get_physical_device_format_properties(self, format)
    }

    #[inline]
    pub fn properties(&self) -> VooResult<PhysicalDeviceProperties> {
        self.instance().get_physical_device_properties(self)
    }

    #[inline]
    pub fn queue_family_properties(&self) -> VooResult<SmallVec<[QueueFamilyProperties; 16]>> {
        self.instance().get_physical_device_queue_family_properties(self)
    }

    /// Returns the memory properties for this device.
    #[inline]
    pub fn memory_properties(&self) -> VooResult<PhysicalDeviceMemoryProperties> {
        self.instance().get_physical_device_memory_properties(self)
    }

    #[inline]
    pub fn extension_properties(&self) -> VooResult<SmallVec<[ExtensionProperties; 64]>> {
        self.instance().enumerate_device_extension_properties(self)

    }

    #[inline]
    pub fn surface_support_khr<Sk>(&self, queue_family_index: u32, surface: Sk) -> VooResult<bool>
            where Sk: Handle<Target=SurfaceKhrHandle> {
        self.instance().get_physical_device_surface_support_khr(self, queue_family_index, surface)
    }

    #[inline]
    pub fn surface_capabilities_khr<Sk>(&self, surface: Sk) -> VooResult<SurfaceCapabilitiesKhr>
            where Sk: Handle<Target=SurfaceKhrHandle> {
        self.instance().get_physical_device_surface_capabilities_khr(self, surface)
    }

    #[inline]
    pub fn surface_formats_khr<Sk>(&self, surface: Sk) -> VooResult<SmallVec<[SurfaceFormatKhr; 64]>>
            where Sk: Handle<Target=SurfaceKhrHandle> {
        self.instance().get_physical_device_surface_formats_khr(self, surface)
    }

    #[inline]
    pub fn surface_present_modes_khr<Sk>(&self, surface: Sk) -> VooResult<SmallVec<[PresentModeKhr; 16]>>
            where Sk: Handle<Target=SurfaceKhrHandle> {
        self.instance().get_physical_device_surface_present_modes_khr(self, surface)
    }


    /// Verifies that the extensions listed are supported by this physical device.
    #[inline]
    pub fn verify_extensions_support(&self, extension_names: &[&CStr]) -> VooResult<bool> {
        let avail_exts = self.extension_properties()?;
        unsafe {
            // Print available:
            for ext in &avail_exts {
                    if PRINT { println!("Available device extension: '{}' (version: {})",
                        ext.extension_name().to_str().unwrap(), ext.spec_version()); }
            };

            for reqd_ext_name in extension_names {
                let mut ext_avail = false;
                for avail_ext in &avail_exts {
                    if reqd_ext_name == &avail_ext.extension_name() {
                        if PRINT { println!("Required device extension available: '{}'",
                            CStr::from_ptr(reqd_ext_name.as_ptr() as *const c_char).to_str().unwrap()); }
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

impl<'p> Handle for &'p PhysicalDevice {
    type Target = PhysicalDeviceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.handle
    }
}

