
use std::sync::Arc;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{VdResult, Device, DescriptorSetLayoutHandle, Handle,
    WriteDescriptorSet, CopyDescriptorSet, DescriptorSet,
    DescriptorSetAllocateInfo, DescriptorSetHandle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DescriptorPoolHandle(pub(crate) vks::VkDescriptorPool);

impl DescriptorPoolHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDescriptorPool {
        self.0
    }
}

unsafe impl Handle for DescriptorPoolHandle {
    type Target = DescriptorPoolHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: DescriptorPoolHandle,
    device: Device,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_pool(self.handle, None);
        }
    }
}

/// A descriptor pool.
///
///
/// ### Destruction
/// 
/// Dropping this `DescriptorPool` will cause `Device::destroy_descriptor_pool` to be called, 
/// automatically releasing any resources associated with it.
///
#[derive(Debug, Clone)]
pub struct DescriptorPool {
    inner: Arc<Inner>,
}

impl DescriptorPool {
    /// Returns a new `DescriptorPoolBuilder`.
    pub fn builder<'b>() -> DescriptorPoolBuilder<'b> {
        DescriptorPoolBuilder::new()
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> DescriptorPoolHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }

    /// Updates descriptor sets.
    pub fn allocate_descriptor_sets<Ds>(&self, layouts: &[Ds])
            -> VdResult<SmallVec<[DescriptorSet; 8]>>
            where Ds: Handle<Target=DescriptorSetLayoutHandle> {
        let layouts: SmallVec<[DescriptorSetLayoutHandle; 8]> = layouts.iter().map(|ds|
            ds.handle()).collect();
        // let len = layouts.len();

        let alloc_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.handle())
            .set_layouts(&layouts)
            .build();

        let descriptor_set_handles: SmallVec<[DescriptorSetHandle; 8]> = unsafe {
            self.inner.device.allocate_descriptor_sets(&alloc_info)?
        };

        Ok(descriptor_set_handles.iter().map(|&dsh| DescriptorSet(dsh)).collect())
    }

    /// Updates the contents of a descriptor set object.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkUpdateDescriptorSets.html
    //
    pub fn update_descriptor_sets(&self, descriptor_writes: &[WriteDescriptorSet],
            descriptor_copies: &[CopyDescriptorSet]) {
        self.inner.device.update_descriptor_sets(descriptor_writes, descriptor_copies)
    }

}

unsafe impl<'d> Handle for &'d DescriptorPool {
    type Target = DescriptorPoolHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}


/// A builder for `DescriptorPool`.
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
    pub fn build(&self, device: Device) -> VdResult<DescriptorPool> {
        let handle = unsafe { device.create_descriptor_pool(&self.create_info, None)? };

        Ok(DescriptorPool {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}