use std::sync::Arc;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use libc::c_char;
use vks;
use ::{VooResult, Instance, Surface, SwapchainSupportDetails};
use queue::{self, Queue};
use instance;


static REQUIRED_EXTENSIONS: [&[u8]; 1] = [
    b"VK_KHR_swapchain\0",
];

// bool checkDeviceExtensionSupport(VkPhysicalDevice device) {
//     uint32_t extensionCount;
//     vkEnumerateDeviceExtensionProperties(device, nullptr, &extensionCount, nullptr);

//     std::vector<VkExtensionProperties> availableExtensions(extensionCount);
//     vkEnumerateDeviceExtensionProperties(device, nullptr, &extensionCount, availableExtensions.data());

//     std::set<std::string> requiredExtensions(deviceExtensions.begin(), deviceExtensions.end());

//     for (const auto& extension : availableExtensions) {
//         requiredExtensions.erase(extension.extensionName);
//     }

//     return requiredExtensions.empty();
// }

fn check_device_extension_support(instance: &Instance, device: vks::VkPhysicalDevice) -> bool {
    let mut avail_ext_count = 0u32;
    let mut avail_exts: Vec<vks::VkExtensionProperties>;
    unsafe {
        ::check(instance.vk().core.vkEnumerateDeviceExtensionProperties(device, ptr::null(),
            &mut avail_ext_count, ptr::null_mut()));
        avail_exts = Vec::with_capacity(avail_ext_count as usize);
        avail_exts.set_len(avail_ext_count as usize);
        ::check(instance.vk().core.vkEnumerateDeviceExtensionProperties(device, ptr::null(),
            &mut avail_ext_count, avail_exts.as_mut_ptr()));

        // Print available:
        for ext in &avail_exts {
                let name = (&ext.extensionName) as *const c_char;
                println!("Available device extension: '{}' (version: {})",
                    CStr::from_ptr(name).to_str().unwrap(), ext.specVersion);
        };

        for reqd_ext_name in &REQUIRED_EXTENSIONS[..] {
            let mut ext_avail = false;
            for avail_ext in &avail_exts {
                if CStr::from_ptr(reqd_ext_name.as_ptr() as *const c_char) ==
                    CStr::from_ptr(avail_ext.extensionName.as_ptr())
                {
                    println!("Required device extension available: '{}'",
                        CStr::from_ptr(reqd_ext_name.as_ptr() as *const c_char).to_str().unwrap());
                    ext_avail = true;
                    break;
                }
            }
            if !ext_avail { return false; }
        }
    }
    true
}

unsafe fn device_is_suitable(instance: &Instance, surface: &Surface, device: vks::VkPhysicalDevice,
        queue_flags: vks::VkQueueFlags) -> bool
{
    let mut device_properties: vks::VkPhysicalDeviceProperties = mem::uninitialized();
    let mut device_features: vks::VkPhysicalDeviceFeatures = mem::uninitialized();
    instance.vk().core.vkGetPhysicalDeviceProperties(device, &mut device_properties);
    instance.vk().core.vkGetPhysicalDeviceFeatures(device, &mut device_features);

    let extensions_supported = check_device_extension_support(instance, device);

    let mut swap_chain_adequate = false;
    if extensions_supported {
        let swap_chain_details = SwapchainSupportDetails::new(instance, surface, device);
        swap_chain_adequate = !swap_chain_details.formats.is_empty() &&
            !swap_chain_details.present_modes.is_empty()
    }

    queue::queue_families(instance, surface, device, queue_flags).is_complete() &&
        extensions_supported &&
        swap_chain_adequate &&
        device_features.samplerAnisotropy != 0
}

pub fn choose_physical_device(instance: &Instance, surface: &Surface, queue_flags: vks::VkQueueFlags)
        -> VooResult<vks::VkPhysicalDevice>
{
    let mut preferred_device = ptr::null_mut();

    for &device in instance.physical_devices() {
        if unsafe { device_is_suitable(instance, surface, device, queue_flags) } {
            preferred_device = device;
            break;
        }
    }

    if preferred_device.is_null() {
        panic!("Failed to find a suitable device.");
    } else {
        println!("Preferred device: {:?}", preferred_device);
    }

    Ok(preferred_device)
}

fn device_features_none() -> vks::VkPhysicalDeviceFeatures {
    vks::VkPhysicalDeviceFeatures {
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
        // samplerAnisotropy: vks::VK_FALSE,
        samplerAnisotropy: vks::VK_TRUE,
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


#[derive(Debug)]
struct Inner {
    handle: vks::VkDevice,
    physical_device: vks::VkPhysicalDevice,
    // features: vks::VkPhysicalDeviceFeatures,
    // queues: SmallVec<[u32; 32]>,
    queue_family_idx: u32,
    // vk: vks::VkDevicePointers,
    instance: Instance,
    loader: vks::DeviceProcAddrLoader,
}

#[derive(Debug, Clone)]
pub struct Device {
    inner: Arc<Inner>,
}

impl Device {
    pub fn new(instance: Instance, surface: &Surface, physical_device: vks::VkPhysicalDevice,
            queue_familiy_flags: vks::VkQueueFlags) -> VooResult<Device>
    {
        let queue_family_idx = queue::queue_families(&instance, surface,
            physical_device, queue_familiy_flags).family_idxs()[0] as u32;

        let queue_create_info = vks::VkDeviceQueueCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueFamilyIndex: queue_family_idx,
            queueCount: 1,
            pQueuePriorities: &1.0,
        };

        let features = device_features_none();

        // createInfo.enabledExtensionCount = static_cast<uint32_t>(deviceExtensions.size());
        // createInfo.ppEnabledExtensionNames = deviceExtensions.data();


        let enabled_layer_names = instance.loader().enabled_layer_names(false);
        let mut enabled_layer_name_ptrs = Vec::with_capacity(enabled_layer_names.len());
        for ln in enabled_layer_names {
            enabled_layer_name_ptrs.push(ln.as_ptr());
        }

        let enabled_extension_names: Vec<_> = (&REQUIRED_EXTENSIONS[..]).iter().map(|ext_name|
            ext_name.as_ptr() as *const c_char).collect();
        let mut enabled_extension_name_ptrs = Vec::with_capacity(enabled_extension_names.len());
        for en in enabled_extension_names {
            enabled_extension_name_ptrs.push(en);
        }

        let create_info = vks::VkDeviceCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueCreateInfoCount: 1,
            pQueueCreateInfos: &queue_create_info,
            enabledLayerCount: enabled_layer_name_ptrs.len() as u32,
            ppEnabledLayerNames: enabled_layer_name_ptrs.as_ptr(),
            enabledExtensionCount: enabled_extension_name_ptrs.len() as u32,
            ppEnabledExtensionNames: enabled_extension_name_ptrs.as_ptr(),
            pEnabledFeatures: &features,
        };

        // Device:
        let mut handle = ptr::null_mut();
        unsafe {
            ::check(instance.vk().core.vkCreateDevice(physical_device, &create_info, ptr::null(), &mut handle));
        }

        let mut loader = vks::DeviceProcAddrLoader::from_get_device_proc_addr(instance.vk().core.pfn_vkGetDeviceProcAddr);

        unsafe {
            loader.load_core(handle);
            // create_info.enabled_extensions.load_device(&mut loader, handle);
            // instance.loader().get_enabled_extensions().load_device(&mut loader, handle);
            // loader.load_khr_sampler_mirror_clamp_to_edge(handle);
            // loader.load_khr_draw_parameters(handle);
            loader.load_khr_swapchain(handle);
            // loader.load_khr_maintenance1(handle);
            // loader.load_amd_rasterization_order(handle);
            // loader.load_amd_draw_indirect_count(handle);
            // loader.load_amd_shader_ballot(handle);
            // loader.load_amd_shader_trinary_minmax(handle);
            // loader.load_amd_shader_explicit_vertex_parameter(handle);
            // loader.load_amd_gcn_shader(handle);
            // loader.load_amd_draw_indirect_count(handle);
            // loader.load_amd_negative_viewport_height(handle);
            // loader.load_amd_shader_info(handle);
            // loader.load_amd_wave_limits(handle);
            // loader.load_amd_texture_gather_bias_lod(handle);
            // loader.load_amd_programmable_sample_locations(handle);
            // loader.load_amd_mixed_attachment_samples(handle);
            // loader.load_ext_shader_subgroup_vote(handle);
            // loader.load_amd_gpa_interface(handle);
            // loader.load_ext_shader_subgroup_ballot(handle);
        }


        Ok(Device {
            inner: Arc::new(Inner {
                handle,
                physical_device,
                // features,
                queue_family_idx,
                instance,
                loader,
            }),
        })
    }

    #[inline]
    pub fn queue(&self, queue_idx: u32) -> vks::VkQueue {
        let mut queue_handle = ptr::null_mut();
        unsafe {
            self.vk().core.vkGetDeviceQueue(self.inner.handle, self.inner.queue_family_idx, queue_idx,
                &mut queue_handle);
        }
        queue_handle
    }

    #[inline]
    pub fn vk(&self) -> &vks::DeviceProcAddrLoader {
        // &self.inner.vk
        &self.inner.loader
    }

    #[inline]
    pub fn handle(&self) -> vks::VkDevice {
        self.inner.handle
    }

    #[inline]
    pub fn physical_device(&self) -> vks::VkPhysicalDevice {
        self.inner.physical_device
    }

    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.inner.instance
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        println!("Destroying device...");
        unsafe {
            self.instance.vk().core.vkDestroyDevice(self.handle, ptr::null());
        }
    }
}