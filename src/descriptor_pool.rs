
use std::sync::Arc;
use std::ptr;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{util, VooResult, Device, DescriptorSetLayout};

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

    pub fn handle(&self) -> vks::VkDescriptorPool {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }

    /// Updates descriptor sets.
    pub fn allocate_descriptor_sets(&self, descriptor_sets: &[&DescriptorSetLayout])
            -> SmallVec<[::DescriptorSet; 8]> {
        // let descriptor_set_handles: SmallVec<[_; 8]> =
        //     descriptor_sets.iter().map(|dsl| dsl).collect();
        let len = descriptor_sets.len();

        let alloc_info = ::DescriptorSetAllocateInfo::builder()
            // sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            // pNext: ptr::null(),
            .descriptor_pool(self)
            // .descriptor_set_count(descriptor_set_handles.len() as u32,
            .set_layouts(descriptor_sets)
            .build();

        let mut descriptor_sets = SmallVec::<[::DescriptorSet; 8]>::new();
        descriptor_sets.reserve_exact(len);
        unsafe {
            descriptor_sets.set_len(len);
            ::check(self.inner.device.proc_addr_loader().vkAllocateDescriptorSets(
                self.inner.device.handle(), alloc_info.raw(),
                descriptor_sets.as_mut_ptr() as *mut vks::VkDescriptorSet));
        }
        descriptor_sets
    }

    pub fn update_descriptor_sets(&self, descriptor_writes: Option<&[::WriteDescriptorSet]>,
            descriptor_copies: Option<&[::CopyDescriptorSet]>) {
        self.inner.device.update_descriptor_sets(descriptor_writes, descriptor_copies)
    }

}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().vkDestroyDescriptorPool(self.device.handle(),
                self.handle, ptr::null());
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
    create_info: ::DescriptorPoolCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorPoolBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> DescriptorPoolBuilder<'b> {
        DescriptorPoolBuilder {
            create_info: ::DescriptorPoolCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// flags is a bitmask of VkDescriptorPoolCreateFlagBits specifying
    /// certain supported operations on the pool.
    pub fn flags<'s>(&'s mut self, flags: ::DescriptorPoolCreateFlags)
            -> &'s mut DescriptorPoolBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// maxSets is the maximum number of descriptor sets that can be allocated
    /// from the pool.
    pub fn max_sets<'s>(&'s mut self, max_sets: u32)
            -> &'s mut DescriptorPoolBuilder<'b> {
        self.create_info.set_max_sets(max_sets);
        self
    }

    /// pPoolSizes is a pointer to an array of VkDescriptorPoolSize
    /// structures, each containing a descriptor type and number of
    /// descriptors of that type to be allocated in the pool.
    pub fn pool_sizes<'s, 'p>(&'s mut self,
            pool_sizes: &'p [::DescriptorPoolSize])
            -> &'s mut DescriptorPoolBuilder<'b>
            where 'p: 'b {
        // self.create_info.poolSizeCount = pool_sizes.len() as u32;
        self.create_info.set_pool_sizes(pool_sizes);
        self
    }

    /// Creates and returns a new `DescriptorPool`
    pub fn build(&self, device: Device) -> VooResult<DescriptorPool> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateDescriptorPool(device.handle(),
                self.create_info.raw(), ptr::null(), &mut handle));
        }

        Ok(DescriptorPool {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}