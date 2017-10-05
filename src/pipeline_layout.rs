use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::marker::PhantomData;
use vks;
use smallvec::SmallVec;
use ::{util, VooResult, Device, ShaderModule, DescriptorSetLayout, Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PipelineLayoutHandle(pub(crate) vks::VkPipelineLayout);

impl Handle for PipelineLayoutHandle {
    type Target = PipelineLayoutHandle;

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

    pub fn handle(&self) -> PipelineLayoutHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl<'h> Handle for &'h PipelineLayout {
    type Target = PipelineLayoutHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyPipelineLayout(self.device.handle().0,
                self.handle.0, ptr::null());
        }
    }
}


/// A builder for `PipelineLayout`.
//
// typedef struct VkPipelineLayoutCreateInfo {
//     VkStructureType                 sType;
//     const void*                     pNext;
//     VkPipelineLayoutCreateFlags     flags;
//     uint32_t                        setLayoutCount;
//     const VkDescriptorSetLayout*    pSetLayouts;
//     uint32_t                        pushConstantRangeCount;
//     const VkPushConstantRange*      pPushConstantRanges;
// } VkPipelineLayoutCreateInfo;
//
#[derive(Debug, Clone)]
pub struct PipelineLayoutBuilder<'b> {
    create_info: vks::VkPipelineLayoutCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineLayoutBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> PipelineLayoutBuilder<'b> {
        PipelineLayoutBuilder {
            create_info: vks::VkPipelineLayoutCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies a list of VkDescriptorSetLayout objects.
    pub fn set_layouts<'s, 'p>(&'s mut self,
            set_layouts: &'p [vks::VkDescriptorSetLayout])
            -> &'s mut PipelineLayoutBuilder<'b>
            where 'p: 'b {
        self.create_info.setLayoutCount = set_layouts.len() as u32;
        self.create_info.pSetLayouts = set_layouts.as_ptr();
        self
    }

    /// Specifies a list of VkPushConstantRange structures defining a set of
    /// push constant ranges for use in a single pipeline layout. In addition
    /// to descriptor set layouts, a pipeline layout also describes how many
    /// push constants can be accessed by each stage of the pipeline.
    /// Specifies a list of VkDescriptorSetLayout objects.
    pub fn push_constant_ranges<'s, 'p>(&'s mut self,
            push_constant_ranges: &'p [vks::VkPushConstantRange])
            -> &'s mut PipelineLayoutBuilder<'b>
            where 'p: 'b {
        self.create_info.pushConstantRangeCount = push_constant_ranges.len() as u32;
        self.create_info.pPushConstantRanges = push_constant_ranges.as_ptr();
        self
    }

    /// Creates and returns a new `PipelineLayout`
    pub fn build(&self, device: Device) -> VooResult<PipelineLayout> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreatePipelineLayout(device.handle().0,
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(PipelineLayout {
            inner: Arc::new(Inner {
                handle: PipelineLayoutHandle(handle),
                device,
            })
        })
    }
}