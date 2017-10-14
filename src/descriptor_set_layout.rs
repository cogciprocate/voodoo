
use std::sync::Arc;
use std::marker::PhantomData;
use vks;
use ::{VdResult, Device, Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DescriptorSetLayoutHandle(pub(crate) vks::VkDescriptorSetLayout);

impl DescriptorSetLayoutHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDescriptorSetLayout {
        self.0
    }
}

unsafe impl Handle for DescriptorSetLayoutHandle {
    type Target = DescriptorSetLayoutHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: DescriptorSetLayoutHandle,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct DescriptorSetLayout {
    inner: Arc<Inner>,
}

impl DescriptorSetLayout {
    /// Returns a new `DescriptorSetLayoutBuilder`.
    pub fn builder<'b>() -> DescriptorSetLayoutBuilder<'b> {
        DescriptorSetLayoutBuilder::new()
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> DescriptorSetLayoutHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'h> Handle for &'h DescriptorSetLayout {
    type Target = DescriptorSetLayoutHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_set_layout(self.handle, None);
        }
    }
}


/// A builder for `DescriptorSetLayout`.
#[derive(Debug, Clone)]
pub struct DescriptorSetLayoutBuilder<'b> {
    create_info: ::DescriptorSetLayoutCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetLayoutBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> DescriptorSetLayoutBuilder<'b> {
        DescriptorSetLayoutBuilder {
            create_info: ::DescriptorSetLayoutCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies options for descriptor set layout creation.
    pub fn flags<'s>(&'s mut self, flags: ::DescriptorSetLayoutCreateFlags)
            -> &'s mut DescriptorSetLayoutBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// Specifies a list of binding configuration structures.
    pub fn bindings<'s, 'p>(&'s mut self,
            bindings: &'p [::DescriptorSetLayoutBinding])
            -> &'s mut DescriptorSetLayoutBuilder<'b>
            where 'p: 'b {
        self.create_info.set_bindings(bindings);
        self
    }

    /// Creates and returns a new `DescriptorSetLayout`
    pub fn build(&self, device: Device) -> VdResult<DescriptorSetLayout> {
        let handle = unsafe { device.create_descriptor_set_layout(&self.create_info, None)? };

        Ok(DescriptorSetLayout {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

}
