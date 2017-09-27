
use std::sync::Arc;
use std::ptr;
use std::marker::PhantomData;
use vks;
use ::{util, VooResult, Device};

#[derive(Debug)]
struct Inner {
    handle: vks::VkDescriptorPool,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct DescriptorPool {
    inner: Arc<Inner>,
}

impl DescriptorPool {
    /// Returns a new `DescriptorPoolBuilder`.
    pub fn builder<'b>() -> DescriptorPoolBuilder<'b> {
        DescriptorPoolBuilder::new()
    }

    // pub fn new(device: Device) -> VooResult<DescriptorPool> {
    //     let pool_sizes = [
    //         vks::VkDescriptorPoolSize {
    //             type_: vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
    //             descriptorCount: 1,
    //         },
    //         vks::VkDescriptorPoolSize {
    //             type_: vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    //             descriptorCount: 1,
    //         },
    //     ];

    //     let create_info = vks::VkDescriptorPoolCreateInfo {
    //         sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
    //         pNext: ptr::null(),
    //         // optional flag similar to command pools that determines if
    //         // individual descriptor sets can be freed or not:
    //         // `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT`:
    //         flags: 0,
    //         maxSets: 1,
    //         poolSizeCount: pool_sizes.len() as u32,
    //         pPoolSizes: pool_sizes.as_ptr(),
    //     };

    //     let mut handle = 0;
    //     unsafe {
    //         ::check(device.proc_addr_loader().vkCreateDescriptorPool(device.handle(), &create_info,
    //             ptr::null(), &mut handle));
    //     }

    //     Ok(DescriptorPool {
    //         inner: Arc::new(Inner {
    //             handle,
    //             device,
    //         })
    //     })
    // }

    pub fn handle(&self) -> vks::VkDescriptorPool {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().vkDestroyDescriptorPool(self.device.handle(), self.handle, ptr::null());
        }
    }
}






/// A builder for `DescriptorPool`.
//
// typedef struct VkDescriptorPoolCreateInfo {
//     VkStructureType                sType;
//     const void*                    pNext;
//     VkDescriptorPoolCreateFlags    flags;
//     uint32_t                       maxSets;
//     uint32_t                       poolSizeCount;
//     const VkDescriptorPoolSize*    pPoolSizes;
// } VkDescriptorPoolCreateInfo;
//
#[derive(Debug, Clone)]
pub struct DescriptorPoolBuilder<'b> {
    create_info: vks::VkDescriptorPoolCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorPoolBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> DescriptorPoolBuilder<'b> {
        DescriptorPoolBuilder {
            create_info: vks::VkDescriptorPoolCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// flags is a bitmask of VkDescriptorPoolCreateFlagBits specifying
    /// certain supported operations on the pool.
    pub fn flags<'s>(&'s mut self, flags: vks::VkDescriptorPoolCreateFlags)
            -> &'s mut DescriptorPoolBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// maxSets is the maximum number of descriptor sets that can be allocated
    /// from the pool.
    pub fn max_sets<'s>(&'s mut self, max_sets: u32)
            -> &'s mut DescriptorPoolBuilder<'b> {
        self.create_info.maxSets = max_sets;
        self
    }

    /// pPoolSizes is a pointer to an array of VkDescriptorPoolSize
    /// structures, each containing a descriptor type and number of
    /// descriptors of that type to be allocated in the pool.
    pub fn pool_sizes<'s, 'p>(&'s mut self,
            pool_sizes: &'p [vks::VkDescriptorPoolSize])
            -> &'s mut DescriptorPoolBuilder<'b>
            where 'p: 'b {
        self.create_info.poolSizeCount = pool_sizes.len() as u32;
        self.create_info.pPoolSizes = pool_sizes.as_ptr();
        self
    }

    /// Creates and returns a new `DescriptorPool`
    pub fn build(&self, device: Device) -> VooResult<DescriptorPool> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateDescriptorPool(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(DescriptorPool {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}