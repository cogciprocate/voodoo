use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::marker::PhantomData;
use vks;
use ::{util, VooResult, Device, ShaderModule};


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
    pub fn builder<'rpb>() -> RenderPassBuilder<'rpb> {
        RenderPassBuilder::new()
    }

    pub fn handle(&self) -> vks::VkRenderPass {
        self.inner.handle
    }

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
pub struct RenderPassBuilder<'rpb> {
    create_info: vks::VkRenderPassCreateInfo,
    _p: PhantomData<&'rpb ()>,
}

impl<'rpb> RenderPassBuilder<'rpb> {
    /// Returns a new render pass builder.
    pub fn new() -> RenderPassBuilder<'rpb> {
        RenderPassBuilder {
            create_info: vks::VkRenderPassCreateInfo::default(),
            _p: PhantomData,
        }
    }

    // /// sType is the type of this structure.
    // /// pNext is NULL or a pointer to an extension-specific structure.
    // /// flags is reserved for future use.
    // pub fn eeeeeeeeeee<'s, 'ci>(&'s mut self, eeeeeeeeeee: &'ci [DeviceQueueCreateInfo])
    //         -> &'s mut RenderPassBuilder<'rpb>
    //         where 'ci: 'rpb {
    //     self.create_info.EeeeeEeeeee = eeeeeeeeeee;
    //     self
    // }


    /// pAttachments points to an array of attachmentCount number of
    /// VkAttachmentDescription structures describing properties of the
    /// attachments, or NULL if attachmentCount is zero.
    pub fn attachments<'s, 'ad>(&'s mut self,
            attachments: &'ad [vks::VkAttachmentDescription])
            -> &'s mut RenderPassBuilder<'rpb>
            where 'ad: 'rpb {
        self.create_info.attachmentCount = attachments.len() as u32;
        self.create_info.pAttachments = attachments.as_ptr();
        self
    }

    /// pSubpasses points to an array of subpassCount number of
    /// VkSubpassDescription structures describing properties of the
    /// subpasses.
    pub fn subpasses<'s, 'ad>(&'s mut self, subpasses: &'ad [vks::VkSubpassDescription])
            -> &'s mut RenderPassBuilder<'rpb>
            where 'ad: 'rpb {
        self.create_info.subpassCount = subpasses.len() as u32;
        self.create_info.pSubpasses = subpasses.as_ptr();
        self
    }

    /// pDependencies points to an array of dependencyCount number of
    /// VkSubpassDependency structures describing dependencies between pairs
    /// of subpasses, or NULL if dependencyCount is zero.
    pub fn dependencies<'s, 'ad>(&'s mut self,
            dependencies: &'ad [vks::VkSubpassDependency])
            -> &'s mut RenderPassBuilder<'rpb>
            where 'ad: 'rpb {
        self.create_info.dependencyCount = dependencies.len() as u32;
        self.create_info.pDependencies = dependencies.as_ptr();
        self
    }

    /// Builds and returns a new `RenderPass`
    pub fn build(&self, device: Device) -> VooResult<RenderPass> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateRenderPass(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(RenderPass {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}