use std::sync::Arc;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use libc::c_char;
use vks;
use ::{VooResult, Instance, Surface, PhysicalDevice, SwapchainSupportDetails};
use queue::{self, Queue};
use instance;


#[derive(Debug)]
struct Inner {
    handle: vks::VkDevice,
    physical_device: PhysicalDevice,
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
    // pub fn new(instance: Instance, surface: &Surface, physical_device: vks::VkPhysicalDevice,
    //         queue_familiy_flags: vks::VkQueueFlags) -> VooResult<Device>
    pub fn new(instance: Instance, physical_device: PhysicalDevice,
            create_info: vks::VkDeviceCreateInfo, queue_family_idx: u32) -> VooResult<Device> {

        // let create_info = vks::VkDeviceCreateInfo {
        //     sType: vks::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        //     pNext: ptr::null(),
        //     flags: 0,
        //     queueCreateInfoCount: 1,
        //     pQueueCreateInfos: &queue_create_info,
        //     enabledLayerCount: enabled_layer_name_ptrs.len() as u32,
        //     ppEnabledLayerNames: enabled_layer_name_ptrs.as_ptr(),
        //     enabledExtensionCount: enabled_extension_name_ptrs.len() as u32,
        //     ppEnabledExtensionNames: enabled_extension_name_ptrs.as_ptr(),
        //     pEnabledFeatures: &features,
        // };

        // Device:
        let mut handle = ptr::null_mut();
        unsafe {
            ::check(instance.proc_addr_loader().core.vkCreateDevice(physical_device.handle(),
                &create_info, ptr::null(), &mut handle));
        }

        let mut loader = vks::DeviceProcAddrLoader::from_get_device_proc_addr(
            instance.proc_addr_loader().core.pfn_vkGetDeviceProcAddr);

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
            self.proc_addr_loader().core.vkGetDeviceQueue(self.inner.handle, self.inner.queue_family_idx, queue_idx,
                &mut queue_handle);
        }
        queue_handle
    }

    #[inline]
    pub fn proc_addr_loader(&self) -> &vks::DeviceProcAddrLoader {
        // &self.inner.vk
        &self.inner.loader
    }

    #[inline]
    pub fn handle(&self) -> vks::VkDevice {
        self.inner.handle
    }

    #[inline]
    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.inner.physical_device
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
            self.instance.proc_addr_loader().core.vkDestroyDevice(self.handle, ptr::null());
        }
    }
}