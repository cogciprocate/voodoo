
use std::sync::Arc;
use std::ptr;
use vks;
use ::{VooResult, Swapchain, Device};


#[derive(Debug)]
pub struct Inner {
    handle: vks::VkImageView,
    device: Device,
    swapchain: Option<Swapchain>,
}

#[derive(Debug, Clone)]
pub struct ImageView {
    inner: Arc<Inner>,
}

impl ImageView {
    pub fn new(device: Device, swapchain: Option<Swapchain>, image: vks::VkImage,
            format: vks::VkFormat, aspect_flags: vks::VkImageAspectFlags)
            -> VooResult<ImageView>
    {
        let create_info = vks::VkImageViewCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            image: image,
            viewType: vks::VK_IMAGE_VIEW_TYPE_2D,
            format: format,
            components: vks::VkComponentMapping {
                r: vks::VK_COMPONENT_SWIZZLE_IDENTITY,
                g: vks::VK_COMPONENT_SWIZZLE_IDENTITY,
                b: vks::VK_COMPONENT_SWIZZLE_IDENTITY,
                a: vks::VK_COMPONENT_SWIZZLE_IDENTITY,
            },
            subresourceRange: vks::VkImageSubresourceRange {
                // aspectMask: vks::VK_IMAGE_ASPECT_COLOR_BIT,
                aspectMask: aspect_flags,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };

        let mut handle = 0;

        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateImageView(device.handle(),
                &create_info, ptr::null(), &mut handle));
        }

        Ok(ImageView {
            inner: Arc::new(Inner {
                handle,
                device,
                swapchain,
            })
        })
    }

    pub fn handle(&self) -> vks::VkImageView {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyImageView(self.device.handle(),
                self.handle, ptr::null());
        }
    }
}


pub fn create_image_views(swapchain: &Swapchain) -> VooResult<Vec<ImageView>> {
    swapchain.images().iter().map(|&image| {
        ImageView::new(swapchain.device().clone(), Some(swapchain.clone()), image,
            swapchain.image_format(), vks::VK_IMAGE_ASPECT_COLOR_BIT)
    }).collect::<Result<Vec<_>, _>>()
}
