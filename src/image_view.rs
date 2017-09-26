
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
    /// Returns a new `ImageViewBuilder`.
    pub fn builder() -> ImageViewBuilder {
        ImageViewBuilder::new()
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
        ImageView::builder()
            .image(image)
            .view_type(vks::VK_IMAGE_VIEW_TYPE_2D)
            .format(swapchain.image_format())
            .components(vks::VkComponentMapping::default())
            .subresource_range(vks::VkImageSubresourceRange {
                aspectMask: vks::VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            })
            .build(swapchain.device().clone(), Some(swapchain.clone()))

    }).collect::<Result<Vec<_>, _>>()
}

/// A builder for an `ImageView`.
//
// typedef struct VkImageViewCreateInfo {
//     VkStructureType            sType;
//     const void*                pNext;
//     VkImageViewCreateFlags     flags;
//     VkImage                    image;
//     VkImageViewType            viewType;
//     VkFormat                   format;
//     VkComponentMapping         components;
//     VkImageSubresourceRange    subresourceRange;
// } VkImageViewCreateInfo;
//
#[derive(Debug, Clone)]
pub struct ImageViewBuilder {
    create_info: vks::VkImageViewCreateInfo,
}

impl ImageViewBuilder {
    /// Returns a new `ImageViewBuilder`.
    pub fn new() -> ImageViewBuilder {
        ImageViewBuilder { create_info: vks::VkImageViewCreateInfo::default() }
    }

    /// Specifies the image on which the view will be created.
    pub fn image<'s>(&'s mut self, image: vks::VkImage) -> &'s mut ImageViewBuilder {
        self.create_info.image = image;
        self
    }

    /// Specifies the type of the image view.
    pub fn view_type<'s>(&'s mut self, view_type: vks::VkImageViewType) -> &'s mut ImageViewBuilder {
        self.create_info.viewType = view_type;
        self
    }

    /// Specifies the format and type used to interpret data elements in the
    /// image.
    pub fn format<'s>(&'s mut self, format: vks::VkFormat) -> &'s mut ImageViewBuilder {
        self.create_info.format = format;
        self
    }

    /// Specifies a remapping of color components (or of depth or stencil
    /// components after they have been converted into color components).
    pub fn components<'s>(&'s mut self, components: vks::VkComponentMapping)
            -> &'s mut ImageViewBuilder {
        self.create_info.components = components;
        self
    }

    /// Specifies the range selecting the set of mipmap levels and array
    /// layers to be accessible to the view.
    pub fn subresource_range<'s>(&'s mut self, subresource_range: vks::VkImageSubresourceRange)
            -> &'s mut ImageViewBuilder {
        self.create_info.subresourceRange = subresource_range;
        self
    }

    pub fn build(&self, device: Device, swapchain: Option<Swapchain>) -> VooResult<ImageView> {
        let mut handle = 0;

        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateImageView(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(ImageView {
            inner: Arc::new(Inner {
                handle,
                device,
                swapchain,
            })
        })
    }
}