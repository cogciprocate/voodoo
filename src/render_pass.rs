use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
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
            ::check(device.vk().core.vkCreateRenderPass(device.handle(), &create_info, ptr::null(), &mut handle));
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
            self.device.vk().core.vkDestroyRenderPass(self.device.handle(), self.handle, ptr::null());
        }
    }
}