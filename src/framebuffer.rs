
use std::sync::Arc;
use std::ptr;
use smallvec::SmallVec;
use vks;
use ::{util, VooResult, Device, RenderPass, ImageView};


#[derive(Debug)]
struct Inner {
    handle: vks::VkFramebuffer,
    device: Device,
    render_pass: RenderPass,
    attachments: SmallVec<[ImageView; 8]>,
}

#[derive(Debug, Clone)]
pub struct Framebuffer {
    inner: Arc<Inner>,
}

impl Framebuffer {
    pub fn new(device: Device, render_pass: RenderPass, swapchain_image_view: ImageView,
            depth_image_view: ImageView, swapchain_extent: vks::VkExtent2D) -> VooResult<Framebuffer>
    {
        let attachment_handles = [swapchain_image_view.handle(),
            depth_image_view.handle()];

        let create_info = vks::VkFramebufferCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            renderPass: render_pass.handle(),
            attachmentCount: attachment_handles.len() as u32,
            pAttachments: attachment_handles.as_ptr(),
            width: swapchain_extent.width,
            height: swapchain_extent.height,
            layers: 1,
        };

        let mut attachments = SmallVec::new();
        attachments.push(swapchain_image_view);
        attachments.push(depth_image_view);

        let mut handle = 0;
        unsafe {
            ::check(device.vk().core.vkCreateFramebuffer(device.handle(), &create_info, ptr::null(),
                &mut handle));
        }

        Ok(Framebuffer {
            inner: Arc::new(Inner {
                handle,
                device,
                render_pass,
                attachments,
            })
        })
    }

    pub fn handle(&self) -> vks::VkFramebuffer {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.vk().core.vkDestroyFramebuffer(self.device.handle(), self.handle, ptr::null());
        }
    }
}


pub fn create_framebuffers(device: &Device, render_pass: &RenderPass,
        swapchain_image_views: &[ImageView], depth_image_view: &ImageView,
        swapchain_extent: vks::VkExtent2D) -> VooResult<Vec<Framebuffer>>
{
    swapchain_image_views.iter().map(|image_view| {
        Framebuffer::new(device.clone(), render_pass.clone(), image_view.clone(),
            depth_image_view.clone(), swapchain_extent.clone())
    }).collect::<Result<Vec<_>, _>>()
}