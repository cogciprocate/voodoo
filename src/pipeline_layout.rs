use std::sync::Arc;
use std::marker::PhantomData;
use vks;
use ::{VdResult, Device,  DescriptorSetLayoutHandle, Handle,
    PipelineLayoutCreateInfo, PushConstantRange};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PipelineLayoutHandle(pub(crate) vks::VkPipelineLayout);

impl PipelineLayoutHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkPipelineLayout {
        self.0
    }
}

unsafe impl Handle for PipelineLayoutHandle {
    type Target = PipelineLayoutHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: PipelineLayoutHandle,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct PipelineLayout {
    inner: Arc<Inner>,
}

impl PipelineLayout {
    /// Returns a new `PipelineLayoutBuilder`.
    pub fn builder<'b>() -> PipelineLayoutBuilder<'b> {
        PipelineLayoutBuilder::new()
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> PipelineLayoutHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'h> Handle for &'h PipelineLayout {
    type Target = PipelineLayoutHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline_layout(self.handle, None);
        }
    }
}


/// A builder for `PipelineLayout`.
#[derive(Debug, Clone)]
pub struct PipelineLayoutBuilder<'b> {
    create_info: PipelineLayoutCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineLayoutBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> PipelineLayoutBuilder<'b> {
        PipelineLayoutBuilder {
            create_info: PipelineLayoutCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies a list of VkDescriptorSetLayout objects.
    pub fn set_layouts<'s, 'p>(&'s mut self,
            set_layouts: &'p [DescriptorSetLayoutHandle])
            -> &'s mut PipelineLayoutBuilder<'b>
            where 'p: 'b {
        // self.create_info.setLayoutCount = set_layouts.len() as u32;
        // self.create_info.pSetLayouts = set_layouts.as_ptr() as *const vks::VkDescriptorSetLayout;
        self.create_info.set_set_layouts(set_layouts);
        self
    }

    /// Specifies a list of VkPushConstantRange structures defining a set of
    /// push constant ranges for use in a single pipeline layout. In addition
    /// to descriptor set layouts, a pipeline layout also describes how many
    /// push constants can be accessed by each stage of the pipeline.
    /// Specifies a list of VkDescriptorSetLayout objects.
    pub fn push_constant_ranges<'s, 'p>(&'s mut self,
            push_constant_ranges: &'p [PushConstantRange])
            -> &'s mut PipelineLayoutBuilder<'b>
            where 'p: 'b {
        // self.create_info.pushConstantRangeCount = push_constant_ranges.len() as u32;
        // self.create_info.pPushConstantRanges = push_constant_ranges.as_ptr();
        self.create_info.set_push_constant_ranges(push_constant_ranges);
        self
    }

    /// Creates and returns a new `PipelineLayout`
    pub fn build(&self, device: Device) -> VdResult<PipelineLayout> {
        let handle = unsafe { device.create_pipeline_layout(&self.create_info, None)? };

        Ok(PipelineLayout {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}