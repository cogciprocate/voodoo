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

#[derive(Debug, Clone)]
pub struct RenderPass {
    inner: Arc<Inner>,
}

impl RenderPass {
    pub fn builder<'rpb>() -> RenderPassBuilder<'rpb> {
        RenderPassBuilder::new()
    }

    pub fn new(device: Device, swapchain_image_format: vks::VkFormat,
            depth_image_format: vks::VkFormat) -> VooResult<RenderPass>
    {
        let color_attachment = vks::VkAttachmentDescription {
            flags: 0,
            format: swapchain_image_format,
            samples: vks::VK_SAMPLE_COUNT_1_BIT,
            loadOp: vks::VK_ATTACHMENT_LOAD_OP_CLEAR,
            storeOp: vks::VK_ATTACHMENT_STORE_OP_STORE,
            stencilLoadOp: vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: vks::VK_IMAGE_LAYOUT_UNDEFINED,
            finalLayout: vks::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        };

        let depth_attachment = vks::VkAttachmentDescription {
            flags: 0,
            format: depth_image_format,
            samples: vks::VK_SAMPLE_COUNT_1_BIT,
            loadOp: vks::VK_ATTACHMENT_LOAD_OP_CLEAR,
            storeOp: vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            stencilLoadOp: vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: vks::VK_IMAGE_LAYOUT_UNDEFINED,
            finalLayout: vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let color_attachment_ref = vks::VkAttachmentReference {
            attachment: 0,
            layout: vks::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment_ref = vks::VkAttachmentReference {
            attachment: 1,
            layout: vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let subpass = vks::VkSubpassDescription {
            flags: 0,
            pipelineBindPoint: vks::VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: 0,
            pInputAttachments: ptr::null(),
            colorAttachmentCount: 1,
            pColorAttachments: &color_attachment_ref,
            pResolveAttachments: ptr::null(),
            pDepthStencilAttachment: &depth_attachment_ref,
            preserveAttachmentCount: 0,
            pPreserveAttachments: ptr::null(),
        };

        let dependency = vks::VkSubpassDependency {
            dependencyFlags: 0,
            srcSubpass: vks::VK_SUBPASS_EXTERNAL,
            dstSubpass: 0,
            srcStageMask: vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            srcAccessMask: 0,
            dstStageMask: vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            dstAccessMask: vks::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT | vks::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
        };

        let attachments = [color_attachment, depth_attachment];

        let create_info = vks::VkRenderPassCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            subpassCount: 1,
            pSubpasses: &subpass,
            dependencyCount: 1,
            pDependencies: &dependency,
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateRenderPass(device.handle(), &create_info, ptr::null(), &mut handle));
        }

        Ok(RenderPass {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
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



// typedef struct VkRenderPassCreateInfo {
//     VkStructureType                   sType;
//     const void*                       pNext;
//     VkRenderPassCreateFlags           flags;
//     uint32_t                          attachmentCount;
//     const VkAttachmentDescription*    pAttachments;
    // uint32_t                          subpassCount;
    // const VkSubpassDescription*       pSubpasses;
//     uint32_t                          dependencyCount;
//     const VkSubpassDependency*        pDependencies;
// } VkRenderPassCreateInfo;


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
        // if let Some(atts) = attachments {
        //     self.create_info.attachmentCount = atts.len() as u32;
        //     self.create_info.pAttachments = atts.as_ptr();
        // } else {
        //     self.create_info.attachmentCount = 0;
        //     self.create_info.pAttachments = ptr::null();
        // }
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
        // if let Some(deps) = dependencies {
        //     self.create_info.dependencyCount = deps.len() as u32;
        //     self.create_info.pDependencies = deps.as_ptr();
        // } else {
        //     self.create_info.dependencyCount = 0;
        //     self.create_info.pDependencies = ptr::null();
        // }
        self.create_info.dependencyCount = dependencies.len() as u32;
        self.create_info.pDependencies = dependencies.as_ptr();
        self
    }

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