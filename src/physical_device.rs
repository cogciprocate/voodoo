use std::sync::Arc;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use libc::c_char;
use smallvec::SmallVec;
use vks;
use ::{VooResult, Instance, Surface, SwapchainSupportDetails, PRINT};
use queue::{self, Queue};
use instance;

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    handle: vks::VkPhysicalDevice,
    instance: Instance
}

impl PhysicalDevice {
    pub fn new(instance: Instance, handle: vks::VkPhysicalDevice) -> PhysicalDevice {
        PhysicalDevice {
            handle,
            instance,
        }
    }

    pub fn handle(&self) -> vks::VkPhysicalDevice {
        self.handle
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn extensions(&self) -> SmallVec<[vks::VkExtensionProperties; 64]> {
        let mut ext_count = 0u32;
        let mut exts = SmallVec::new();
        unsafe {
            ::check(self.instance.proc_addr_loader().core.vkEnumerateDeviceExtensionProperties(self.handle, ptr::null(),
                &mut ext_count, ptr::null_mut()));
            assert!(ext_count as usize <= exts.inline_size());
            exts.set_len(ext_count as usize);
            ::check(self.instance.proc_addr_loader().core.vkEnumerateDeviceExtensionProperties(self.handle, ptr::null(),
                &mut ext_count, exts.as_mut_ptr()));
        }
        exts
    }

    pub fn properties(&self) -> vks::VkPhysicalDeviceProperties {
        unsafe {
            let mut device_properties: vks::VkPhysicalDeviceProperties = mem::uninitialized();
            self.instance.proc_addr_loader().vkGetPhysicalDeviceProperties(self.handle,
                &mut device_properties);
            device_properties
        }
    }

    pub fn features(&self) -> vks::VkPhysicalDeviceFeatures {
        unsafe {
            let mut device_features: vks::VkPhysicalDeviceFeatures = mem::uninitialized();
            self.instance.proc_addr_loader().core.vkGetPhysicalDeviceFeatures(self.handle,
                &mut device_features);
            device_features
        }
    }

    pub fn capabilities(&self, surface: &Surface) -> vks::VkSurfaceCapabilitiesKHR {
        unsafe {
            let mut capabilities = mem::uninitialized();
            self.instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                self.handle(), surface.handle(), &mut capabilities);
            capabilities
        }
    }

    pub fn formats(&self, surface: &Surface) -> SmallVec<[vks::VkSurfaceFormatKHR; 64]> {
        let mut format_count = 0u32;
        let mut formats = SmallVec::new();
        unsafe {
            self.instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(self.handle(),
                surface.handle(), &mut format_count, ptr::null_mut());
            assert!(format_count as usize <= formats.inline_size());
            formats.set_len(format_count as usize);
            if format_count != 0 {
                self.instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(self.handle(),
                    surface.handle(), &mut format_count, formats.as_mut_ptr());
            }
        }
        if PRINT { println!("Physical device format count: {:?}", formats.len()); }
        formats
    }

    pub fn present_modes(&self, surface: &Surface) -> SmallVec<[vks::VkPresentModeKHR; 16]> {
        let mut present_mode_count = 0u32;
        let mut present_modes = SmallVec::new();
        unsafe {
            self.instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(self.handle(),
                surface.handle(), &mut present_mode_count, ptr::null_mut());
            assert!(present_mode_count as usize <= present_modes.inline_size());
            present_modes.set_len(present_mode_count as usize);
            if present_mode_count != 0 {
                self.instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(self.handle(),
                    surface.handle(), &mut present_mode_count, present_modes.as_mut_ptr());
            }
        }
        if PRINT { println!("Physical device present mode count: {:?}", present_modes.len()); }
        present_modes
    }

    pub fn queue_families(&self) -> SmallVec<[vks::VkQueueFamilyProperties; 16]> {
        let mut queue_family_count = 0u32;
        let mut queue_families = SmallVec::new();
        unsafe {
            self.instance.proc_addr_loader().core.vkGetPhysicalDeviceQueueFamilyProperties(
                self.handle, &mut queue_family_count, ptr::null_mut());
            assert!(queue_family_count as usize <= queue_families.inline_size());
            queue_families.set_len(queue_family_count as usize);
            self.instance.proc_addr_loader().core.vkGetPhysicalDeviceQueueFamilyProperties(
                self.handle, &mut queue_family_count, queue_families.as_mut_ptr());
        }
        if PRINT {  println!("Physical device queue family count: {:?}", queue_families.len()); }
        queue_families
    }

    pub fn surface_support_khr(&self, queue_family_index: u32, surface: &Surface) -> bool {
        let mut supported: vks::VkBool32 = vks::VK_FALSE;
        unsafe {
            ::check(self.instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR(
                self.handle, queue_family_index, surface.handle(), &mut supported));
        }
        supported == vks::VK_TRUE
    }

    /// Verifies that the extensions listed are supported by this physical device.
    pub fn verify_extensions_support(&self, extension_names: &[&CStr]) -> bool {
        let avail_exts = self.extensions();
        unsafe {
            // Print available:
            for ext in &avail_exts {
                    let name = (&ext.extensionName) as *const c_char;
                    if PRINT { println!("Available device extension: '{}' (version: {})",
                        CStr::from_ptr(name).to_str().unwrap(), ext.specVersion); }
            };

            for reqd_ext_name in extension_names {
                let mut ext_avail = false;
                for avail_ext in &avail_exts {
                    if CStr::from_ptr(reqd_ext_name.as_ptr() as *const c_char) ==
                        CStr::from_ptr(avail_ext.extensionName.as_ptr())
                    {
                        if PRINT { println!("Required device extension available: '{}'",
                            CStr::from_ptr(reqd_ext_name.as_ptr() as *const c_char).to_str().unwrap()); }
                        ext_avail = true;
                        break;
                    }
                }
                if !ext_avail { return false; }
            }
        }
        true
    }

    /// Returns the memory properties for this device.
    pub fn memory_properties(&self) -> vks::VkPhysicalDeviceMemoryProperties {
        let mut mem_props: vks::VkPhysicalDeviceMemoryProperties;
        unsafe {
            mem_props = mem::uninitialized();
            self.instance().proc_addr_loader().core.vkGetPhysicalDeviceMemoryProperties(
                self.handle, &mut mem_props);
        }
        mem_props
    }
}



pub struct PhysicalDeviceFeatures {
    raw: vks::VkPhysicalDeviceFeatures,
}

impl PhysicalDeviceFeatures {
    pub fn new() -> PhysicalDeviceFeatures {
        PhysicalDeviceFeatures {
            raw: vks::VkPhysicalDeviceFeatures {
                robustBufferAccess: vks::VK_FALSE,
                fullDrawIndexUint32: vks::VK_FALSE,
                imageCubeArray: vks::VK_FALSE,
                independentBlend: vks::VK_FALSE,
                geometryShader: vks::VK_FALSE,
                tessellationShader: vks::VK_FALSE,
                sampleRateShading: vks::VK_FALSE,
                dualSrcBlend: vks::VK_FALSE,
                logicOp: vks::VK_FALSE,
                multiDrawIndirect: vks::VK_FALSE,
                drawIndirectFirstInstance: vks::VK_FALSE,
                depthClamp: vks::VK_FALSE,
                depthBiasClamp: vks::VK_FALSE,
                fillModeNonSolid: vks::VK_FALSE,
                depthBounds: vks::VK_FALSE,
                wideLines: vks::VK_FALSE,
                largePoints: vks::VK_FALSE,
                alphaToOne: vks::VK_FALSE,
                multiViewport: vks::VK_FALSE,
                samplerAnisotropy: vks::VK_FALSE,
                textureCompressionETC2: vks::VK_FALSE,
                textureCompressionASTC_LDR: vks::VK_FALSE,
                textureCompressionBC: vks::VK_FALSE,
                occlusionQueryPrecise: vks::VK_FALSE,
                pipelineStatisticsQuery: vks::VK_FALSE,
                vertexPipelineStoresAndAtomics: vks::VK_FALSE,
                fragmentStoresAndAtomics: vks::VK_FALSE,
                shaderTessellationAndGeometryPointSize: vks::VK_FALSE,
                shaderImageGatherExtended: vks::VK_FALSE,
                shaderStorageImageExtendedFormats: vks::VK_FALSE,
                shaderStorageImageMultisample: vks::VK_FALSE,
                shaderStorageImageReadWithoutFormat: vks::VK_FALSE,
                shaderStorageImageWriteWithoutFormat: vks::VK_FALSE,
                shaderUniformBufferArrayDynamicIndexing: vks::VK_FALSE,
                shaderSampledImageArrayDynamicIndexing: vks::VK_FALSE,
                shaderStorageBufferArrayDynamicIndexing: vks::VK_FALSE,
                shaderStorageImageArrayDynamicIndexing: vks::VK_FALSE,
                shaderClipDistance: vks::VK_FALSE,
                shaderCullDistance: vks::VK_FALSE,
                shaderFloat64: vks::VK_FALSE,
                shaderInt64: vks::VK_FALSE,
                shaderInt16: vks::VK_FALSE,
                shaderResourceResidency: vks::VK_FALSE,
                shaderResourceMinLod: vks::VK_FALSE,
                sparseBinding: vks::VK_FALSE,
                sparseResidencyBuffer: vks::VK_FALSE,
                sparseResidencyImage2D: vks::VK_FALSE,
                sparseResidencyImage3D: vks::VK_FALSE,
                sparseResidency2Samples: vks::VK_FALSE,
                sparseResidency4Samples: vks::VK_FALSE,
                sparseResidency8Samples: vks::VK_FALSE,
                sparseResidency16Samples: vks::VK_FALSE,
                sparseResidencyAliased: vks::VK_FALSE,
                variableMultisampleRate: vks::VK_FALSE,
                inheritedQueries: vks::VK_FALSE,
            }
        }
    }

    pub fn sampler_anisotropy(mut self, b: bool) -> PhysicalDeviceFeatures {
        self.raw.samplerAnisotropy = if b { vks::VK_TRUE } else {vks::VK_FALSE };
        self
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceFeatures {
        &self.raw
    }
}
