
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
    pub fn builder<'b>() -> ImageViewBuilder<'b> {
        ImageViewBuilder::new()
    }

    pub fn handle(&self) -> vks::VkImageView {
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
            self.device.proc_addr_loader().core.vkDestroyImageView(self.device.handle(),
                self.handle, ptr::null());
        }
    }
}

/// A builder for an `ImageView`.
#[derive(Debug, Clone)]
pub struct ImageViewBuilder<'b> {
    create_info: ::ImageViewCreateInfo<'b>,
}

impl<'b> ImageViewBuilder<'b> {
    /// Returns a new `ImageViewBuilder`.
    pub fn new() -> ImageViewBuilder<'b> {
        ImageViewBuilder { create_info: ::ImageViewCreateInfo::default() }
    }

    /// Specifies the image on which the view will be created.
    pub fn image<'s>(&'s mut self, image: vks::VkImage) -> &'s mut ImageViewBuilder<'b> {
        self.create_info.set_image_handle(image);
        self
    }

    /// Specifies the type of the image view.
    pub fn view_type<'s>(&'s mut self, view_type: ::ImageViewType) -> &'s mut ImageViewBuilder<'b> {
        self.create_info.set_view_type(view_type);
        self
    }

    /// Specifies the format and type used to interpret data elements in the
    /// image.
    pub fn format<'s>(&'s mut self, format: ::Format) -> &'s mut ImageViewBuilder<'b> {
        self.create_info.set_format(format);
        self
    }

    /// Specifies a remapping of color components (or of depth or stencil
    /// components after they have been converted into color components).
    pub fn components<'s>(&'s mut self, components: ::ComponentMapping)
            -> &'s mut ImageViewBuilder<'b> {
        self.create_info.set_components(components);
        self
    }

    /// Specifies the range selecting the set of mipmap levels and array
    /// layers to be accessible to the view.
    pub fn subresource_range<'s>(&'s mut self, subresource_range: ::ImageSubresourceRange)
            -> &'s mut ImageViewBuilder<'b> {
        self.create_info.set_subresource_range(subresource_range);
        self
    }

    pub fn build(&self, device: Device, swapchain: Option<Swapchain>) -> VooResult<ImageView> {
        let mut handle = 0;

        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateImageView(device.handle(),
                self.create_info.raw(), ptr::null(), &mut handle));
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