use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::marker::PhantomData;
use vks;
use ::{util, VooResult, Device, ShaderModule};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RenderPassHandle(pub(crate) vks::VkRenderPass);


#[derive(Debug)]
struct Inner {
    handle: vks::VkRenderPass,
    device: Device,
}

/// A render pass.
#[derive(Debug, Clone)]
pub struct RenderPass {
    inner: Arc<Inner>,
}

impl RenderPass {
    /// Returns a new `RenderPassBuilder`.
    pub fn builder<'b>() -> RenderPassBuilder<'b> {
        RenderPassBuilder::new()
    }

    pub fn handle(&self) -> vks::VkRenderPass {
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
            self.device.proc_addr_loader().core.vkDestroyRenderPass(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `RenderPass`.
//
// typedef struct VkRenderPassCreateInfo {
//     VkStructureType                   sType;
//     const void*                       pNext;
//     VkRenderPassCreateFlags           flags;
//     uint32_t                          attachmentCount;
//     const VkAttachmentDescription*    pAttachments;
//     uint32_t                          subpassCount;
//     const VkSubpassDescription*       pSubpasses;
//     uint32_t                          dependencyCount;
//     const VkSubpassDependency*        pDependencies;
// } VkRenderPassCreateInfo;
//
#[derive(Debug, Clone)]
pub struct RenderPassBuilder<'b> {
    create_info: ::RenderPassCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> RenderPassBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> RenderPassBuilder<'b> {
        RenderPassBuilder {
            create_info: ::RenderPassCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// pAttachments points to an array of attachmentCount number of
    /// VkAttachmentDescription structures describing properties of the
    /// attachments, or NULL if attachmentCount is zero.
    pub fn attachments<'s, 'ad>(&'s mut self,
            attachments: &'ad [::AttachmentDescription])
            -> &'s mut RenderPassBuilder<'b>
            where 'ad: 'b {
        // self.create_info.attachmentCount = attachments.len() as u32;
        self.create_info.set_attachments(attachments);
        self
    }

    /// pSubpasses points to an array of subpassCount number of
    /// VkSubpassDescription structures describing properties of the
    /// subpasses.
    pub fn subpasses<'s, 'ad>(&'s mut self, subpasses: &'ad [::SubpassDescription])
            -> &'s mut RenderPassBuilder<'b>
            where 'ad: 'b {
        // self.create_info.subpassCount = subpasses.len() as u32;
        self.create_info.set_subpasses(subpasses);
        self
    }

    /// pDependencies points to an array of dependencyCount number of
    /// VkSubpassDependency structures describing dependencies between pairs
    /// of subpasses, or NULL if dependencyCount is zero.
    pub fn dependencies<'s, 'ad>(&'s mut self,
            dependencies: &'ad [::SubpassDependency])
            -> &'s mut RenderPassBuilder<'b>
            where 'ad: 'b {
        // self.create_info.dependencyCount = dependencies.len() as u32;
        self.create_info.set_dependencies(dependencies);
        self
    }

    /// Builds and returns a new `RenderPass`
    pub fn build(&self, device: Device) -> VooResult<RenderPass> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateRenderPass(device.handle(),
                self.create_info.as_raw(), ptr::null(), &mut handle));
        }

        Ok(RenderPass {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}