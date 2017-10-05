use std::sync::Arc;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::marker::PhantomData;
use libc::c_char;
use smallvec::SmallVec;
use vks;
use ::{VooResult, Instance, Surface, PhysicalDevice, SwapchainSupportDetails,
    DeviceQueueCreateInfo, CharStrs, PhysicalDeviceFeatures, PRINT};
use queue::{self, Queue};
use instance;


#[derive(Debug)]
struct Inner {
    handle: vks::VkDevice,
    physical_device: PhysicalDevice,
    // features: vks::VkPhysicalDeviceFeatures,
    // queues: SmallVec<[u32; 32]>,
    queue_family_indices: SmallVec<[u32; 16]>,
    // vk: vks::VkDevicePointers,
    instance: Instance,
    loader: vks::DeviceProcAddrLoader,
}

#[derive(Debug, Clone)]
pub struct Device {
    inner: Arc<Inner>,
}

impl Device {
    /// Returns a new `DeviceBuilder`.
    pub fn builder<'db>() -> DeviceBuilder<'db> {
        DeviceBuilder::new()
    }

    #[inline]
    pub fn queue(&self, queue_idx: u32) -> vks::VkQueue {
        let mut queue_handle = ptr::null_mut();
        assert!(self.inner.queue_family_indices.len() == 1,
            "Update this shitty queue family code.");
        unsafe {
            self.proc_addr_loader().core.vkGetDeviceQueue(self.inner.handle,
                self.inner.queue_family_indices[0], queue_idx,
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

    /// Returns the memory type index on this device matching the provided
    /// type filter and properties.
    //
    // [HELPER]
    pub fn memory_type_index(&self, type_filter: u32, properties: ::MemoryPropertyFlags)
            -> u32 {
        let mem_props = self.physical_device().memory_properties();

        for i in 0..mem_props.memory_type_count() {
            if (type_filter & (1 << i)) != 0 &&
                (mem_props.memory_types()[i as usize].property_flags() & properties) == properties
            {
                return i;
            }
        }
        panic!("failed to find suitable memory type index with: type_filter: '{}', properties: '{:?}'",
            type_filter, properties);
    }

    /// Updates descriptor sets.
    pub fn update_descriptor_sets(&self, descriptor_writes: Option<&[::WriteDescriptorSet]>,
            descriptor_copies: Option<&[::CopyDescriptorSet]>) {
        let (descriptor_writes_len, descriptor_writes_ptr) = match descriptor_writes {
            Some(ref dws) => (dws.len() as u32, dws.as_ptr()),
            None => (0, ptr::null()),
        };

        let (descriptor_copies_len, descriptor_copies_ptr) = match descriptor_copies {
            Some(ref dcs) => (dcs.len() as u32, dcs.as_ptr()),
            None => (0, ptr::null()),
        };

        unsafe {
            self.proc_addr_loader().vkUpdateDescriptorSets(self.handle(),
                descriptor_writes_len, descriptor_writes_ptr as *const vks::VkWriteDescriptorSet,
                descriptor_copies_len, descriptor_copies_ptr as *const vks::VkCopyDescriptorSet);
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        if PRINT { println!("Destroying device..."); }
        unsafe {
            self.instance.proc_addr_loader().core.vkDestroyDevice(self.handle, ptr::null());
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}


// typedef struct VkDeviceCreateInfo {
//     VkStructureType                    sType;
//     const void*                        pNext;
//     VkDeviceCreateFlags                flags;
//     uint32_t                           queueCreateInfoCount;
//     const VkDeviceQueueCreateInfo*     pQueueCreateInfos;
//     uint32_t                           enabledLayerCount;
//     const char* const*                 ppEnabledLayerNames;
//     uint32_t                           enabledExtensionCount;
//     const char* const*                 ppEnabledExtensionNames;
//     const VkPhysicalDeviceFeatures*    pEnabledFeatures;
// } VkDeviceCreateInfo;
//
#[derive(Debug, Clone)]
pub struct DeviceBuilder<'db> {
    create_info: ::DeviceCreateInfo<'db>,
    enabled_layer_names: Option<CharStrs<'db>>,
    enabled_extension_names: Option<CharStrs<'db>>,
    _p: PhantomData<&'db ()>,
}

impl<'db> DeviceBuilder<'db> {
    /// Returns a new instance builder.
    pub fn new() -> DeviceBuilder<'db> {
        DeviceBuilder {
            create_info: ::DeviceCreateInfo::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            _p: PhantomData,
        }
    }

    /// Specifies the list of VkDeviceQueueCreateInfo structures describing
    /// the queues that are requested to be created along with the logical
    /// device.
    pub fn queue_create_infos<'s, 'ci>(&'s mut self,
            queue_create_infos: &'ci [DeviceQueueCreateInfo])
            -> &'s mut DeviceBuilder<'db>
            where 'ci: 'db {
        // self.create_info.queueCreateInfoCount = queue_create_infos.len() as u32;
        debug_assert_eq!(mem::align_of::<DeviceQueueCreateInfo>(),
            mem::align_of::<vks::VkDeviceQueueCreateInfo>());
        debug_assert_eq!(mem::size_of::<DeviceQueueCreateInfo>(),
            mem::size_of::<vks::VkDeviceQueueCreateInfo>());
        // self.create_info.queue_create_infos = queue_create_infos.as_ptr() as *const _;
        self.create_info.set_queue_create_infos(queue_create_infos);
        self
    }

    /// Specifies the layer names to enable.
    ///
    /// Ignored.
    #[deprecated(note = "No longer used")]
    pub fn enabled_layer_names<'s, 'cs, Cs>(&'s mut self, enabled_layer_names: Cs)
            -> &'s mut DeviceBuilder<'db>
            where 'cs: 'db, Cs: 'cs + Into<CharStrs<'cs>> {
        // self.enabled_layer_names = Some(enabled_layer_names.into());
        // if let Some(ref elns) = self.enabled_layer_names {
        //     self.create_info.enabledLayerCount = elns.len() as u32;
        //     self.create_info.enabled_layer_names = elns.as_ptr() as *const _;
        // }
        self.create_info.set_enabled_layer_names(enabled_layer_names);
        self
    }

    /// Specifies the list of names of extensions to enable for the created
    /// device.
    pub fn enabled_extension_names<'s, 'cs, Cs>(&'s mut self, enabled_extension_names: Cs)
            -> &'s mut DeviceBuilder<'db>
            where 'cs: 'db, Cs: 'cs + Into<CharStrs<'cs>> {
        // self.enabled_extension_names = Some(enabled_extension_names.into());
        // if let Some(ref eens) = self.enabled_extension_names {
        //     // self.create_info.enabledExtensionCount = eens.len() as u32;
        //     self.create_info.enabled_extension_names = eens.as_ptr() as *const _;
        // }
        self.create_info.set_enabled_extension_names(enabled_extension_names);
        self
    }

    /// Specifies the structure that contains boolean indicators of all the
    /// features to be enabled.
    pub fn enabled_features<'s, 'f>(&'s mut self, enabled_features: &'f PhysicalDeviceFeatures)
            -> &'s mut DeviceBuilder<'db>
            where 'f: 'db {
        self.create_info.set_enabled_features(enabled_features);
        self
    }

    /// Builds and returns a new `Device`.
    pub fn build(&self, physical_device: PhysicalDevice) -> VooResult<Device> {
                // Device:
        let mut handle = ptr::null_mut();
        unsafe {
            ::check(physical_device.instance().proc_addr_loader().core.vkCreateDevice(physical_device.handle(),
                self.create_info.raw(), ptr::null(), &mut handle));
        }

        let mut loader = vks::DeviceProcAddrLoader::from_get_device_proc_addr(
            physical_device.instance().proc_addr_loader().core.pfn_vkGetDeviceProcAddr);

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

        let instance = physical_device.instance().clone();
        let mut queue_family_indices = SmallVec::<[u32; 16]>::new();
        // for i in 0..(self.create_info.queueCreateInfoCount as isize) {
        //     unsafe {
        //         let queue_create_info_ptr = self.create_info.pQueueCreateInfos.offset(i);
        //         queue_family_indices.push((*queue_create_info_ptr).queueFamilyIndex);
        //     }
        // }

        for queue_create_info in self.create_info.queue_create_infos() {
            queue_family_indices.push(queue_create_info.queue_family_index())
        }
        assert!(queue_family_indices.len() == 1, "Update this shitty queue family code.");

        Ok(Device {
            inner: Arc::new(Inner {
                handle,
                physical_device,
                // features,
                queue_family_indices: queue_family_indices,
                instance,
                loader,
            }),
        })
    }
}