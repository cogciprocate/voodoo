use std::sync::Arc;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{VdResult, SurfaceKhr, Device, PhysicalDevice, ImageHandle, Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SwapchainKhrHandle(pub(crate) vks::VkSwapchainKHR);

impl SwapchainKhrHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkSwapchainKHR {
        self.0
    }
}

unsafe impl Handle for SwapchainKhrHandle {
    type Target = SwapchainKhrHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


pub struct SwapchainSupportDetails {
    pub capabilities: ::SurfaceCapabilitiesKhr,
    pub formats: SmallVec<[::SurfaceFormatKhr; 64]>,
    pub present_modes: SmallVec<[::PresentModeKhr; 16]>,
}

impl SwapchainSupportDetails {
    pub fn new(surface: &SurfaceKhr, physical_device: &PhysicalDevice)
            -> VdResult<SwapchainSupportDetails> {
        let capabilities = physical_device.surface_capabilities_khr(surface)?;
        let formats = physical_device.surface_formats_khr(surface)?;
        let present_modes = physical_device.surface_present_modes_khr(surface)?;

        Ok(SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }
}


#[derive(Debug)]
struct Inner {
    handle: SwapchainKhrHandle,
    device: Device,
    surface: SurfaceKhr,
    // TODO: Revisit whether we should simply store a handle.
    images: SmallVec<[ImageHandle; 4]>,
    image_format: ::Format,
    extent: ::Extent2d,
}

#[derive(Debug, Clone)]
pub struct SwapchainKhr {
    inner: Arc<Inner>,
}

impl SwapchainKhr {
    pub fn builder<'b>() -> SwapchainKhrBuilder<'b> {
        SwapchainKhrBuilder::new()
    }

    pub fn images(&self) -> &[ImageHandle] {
        &self.inner.images
    }

    pub fn image_format(&self) -> ::Format {
        self.inner.image_format
    }

    pub fn extent(&self) -> &::Extent2d {
        &self.inner.extent
    }

    pub fn handle(&self) -> SwapchainKhrHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'s> Handle for &'s SwapchainKhr {
    type Target = SwapchainKhrHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}


impl Drop for Inner {
    fn drop(&mut self) {
        unsafe { self.device.destroy_swapchain_khr(self.handle, None); }
    }
}

unsafe impl Sync for SwapchainKhr {}


/// A Swapchain builder.
#[derive(Debug, Clone)]
pub struct SwapchainKhrBuilder<'b> {
    create_info: ::SwapchainCreateInfoKhr<'b>,
    // Must keep alive to maintain destruction order:
    surface: Option<&'b SurfaceKhr>,
    _p: PhantomData<&'b ()>,
}

impl<'b> SwapchainKhrBuilder<'b> {
    /// Returns a new swapchain builder.
    pub fn new() -> SwapchainKhrBuilder<'b> {
        SwapchainKhrBuilder {
            create_info: ::SwapchainCreateInfoKhr::default(),
            surface: None,
            _p: PhantomData,
        }
    }

    /// Specifies the parameters of swapchain creation.
    pub fn flags<'s>(&'s mut self, flags: ::SwapchainCreateFlagsKhr)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// Specifies the surface that the swapchain will present images to.
    pub fn surface<'s, 'p>(&'s mut self, surface: &'p SurfaceKhr)
            -> &'s mut SwapchainKhrBuilder<'b>
            where 'p: 'b {
        self.create_info.set_surface(surface.handle());
        self.surface = Some(surface);
        self
    }

    /// Specifies the minimum number of presentable images that the
    /// application needs. The platform will either create the swapchain with
    /// at least that many images, or will fail to create the swapchain.
    pub fn min_image_count<'s>(&'s mut self, min_image_count: u32)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_min_image_count(min_image_count);
        self
    }

    /// Specifies the format that is valid for swapchains on the specified
    /// surface.
    pub fn image_format<'s>(&'s mut self, image_format: ::Format)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_image_format(image_format);
        self
    }

    /// Specifies the color space that is valid for swapchains on the
    /// specified surface.
    pub fn image_color_space<'s>(&'s mut self, image_color_space: ::ColorSpaceKhr)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_image_color_space(image_color_space);
        self
    }

    /// Specifies the size (in pixels) of the swapchain. Behavior is
    /// platform-dependent when the image extent does not match the surface’s
    /// current extent as returned by `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`.
    pub fn image_extent<'s>(&'s mut self, image_extent: ::Extent2d)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_image_extent(image_extent);
        self
    }

    /// Specifies the number of views in a multiview/stereo surface. For
    /// non-stereoscopic-3D applications, this value is 1.
    pub fn image_array_layers<'s>(&'s mut self, image_array_layers: u32)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_image_array_layers(image_array_layers);
        self
    }

    /// Specifies the bitmask of `ImageUsageFlagBits`, indicating how the
    /// application will use the swapchain’s presentable images
    pub fn image_usage<'s>(&'s mut self, image_usage: ::ImageUsageFlags)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_image_usage(image_usage);
        self
    }

    /// Specifies the sharing mode used for the images of the swapchain.
    pub fn image_sharing_mode<'s>(&'s mut self, image_sharing_mode: ::SharingMode)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_image_sharing_mode(image_sharing_mode);
        self
    }

    /// Specifies the queue family indices having access to the images of the
    /// swapchain in case imageSharingMode is VK_SHARING_MODE_CONCURRENT.
    pub fn queue_family_indices<'s, 'qfi>(&'s mut self, queue_family_indices: &'qfi [u32])
            -> &'s mut SwapchainKhrBuilder<'b>
            where 'qfi: 'b {
        self.create_info.set_queue_family_indices(queue_family_indices);
        self
    }

    /// Specifies the bitmask of VkSurfaceTransformFlagBitsKHR, describing the
    /// transform, relative to the presentation engine’s natural orientation,
    /// applied to the image content prior to presentation. If it does not
    /// match the currentTransform value returned by
    /// vkGetPhysicalDeviceSurfaceCapabilitiesKHR, the presentation engine
    /// will transform the image content as part of the presentation operation.
    pub fn pre_transform<'s>(&'s mut self, pre_transform: ::SurfaceTransformFlagsKhr)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_pre_transform(pre_transform);
        self
    }

    /// Specifies the bitmask of VkCompositeAlphaFlagBitsKHR indicating the
    /// alpha compositing mode to use when this surface is composited together
    /// with other surfaces on certain window systems.
    pub fn composite_alpha<'s>(&'s mut self, composite_alpha: ::CompositeAlphaFlagsKhr)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_composite_alpha(composite_alpha);
        self
    }

    /// Specifies the presentation mode the swapchain will use. A swapchain’s
    /// present mode determines how incoming present requests will be
    /// processed and queued internally.
    pub fn present_mode<'s>(&'s mut self, present_mode: ::PresentModeKhr)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_present_mode(present_mode);
        self
    }

    /// Specifies the whether the Vulkan implementation is allowed to discard
    /// rendering operations that affect regions of the surface which are not
    /// visible.
    ///
    /// * If set to `true`, the presentable images associated with the
    ///   swapchain may not own all of their pixels. Pixels in the presentable
    ///   images that correspond to regions of the target surface obscured by
    ///   another window on the desktop or subject to some other clipping
    ///   mechanism will have undefined content when read back. Pixel shaders
    ///   may not execute for these pixels, and thus any side affects they
    ///   would have had will not occur.
    ///
    /// * If set to `false`, presentable images associated with the swapchain
    ///   will own all the pixels they contain. Setting this value to VK_TRUE
    ///   does not guarantee any clipping will occur, but allows more optimal
    ///   presentation methods to be used on some platforms.
    ///
    ///
    /// Note: Applications should set this value to VK_TRUE if they do not
    /// expect to read back the content of presentable images before
    /// presenting them or after reacquiring them and if their pixel shaders
    /// do not have any side effects that require them to run for all pixels
    /// in the presentable image.
    ///
    pub fn clipped<'s>(&'s mut self, clipped: bool)
            -> &'s mut SwapchainKhrBuilder<'b> {
        self.create_info.set_clipped(clipped);
        self
    }

    /// If not VK_NULL_HANDLE, specifies the swapchain that will be replaced
    /// by the new swapchain being created. The new swapchain will be a
    /// descendant of oldSwapchain. Further, any descendants of the new
    /// swapchain will also be descendants of oldSwapchain. Upon calling
    /// vkCreateSwapchainKHR with a oldSwapchain that is not VK_NULL_HANDLE,
    /// any images not acquired by the application may be freed by the
    /// implementation, which may occur even if creation of the new swapchain
    /// fails. The application must destroy the old swapchain to free all
    /// memory associated with the old swapchain. The application must wait
    /// for the completion of any outstanding rendering to images it currently
    /// has acquired at the time the swapchain is destroyed. The application
    /// can continue to present any images it acquired and has not yet
    /// presented using the old swapchain, as long as it has not entered a
    /// state that causes it to return VK_ERROR_OUT_OF_DATE_KHR. However, the
    /// application cannot acquire any more images from the old swapchain
    /// regardless of whether or not creation of the new swapchain succeeds.
    /// The application can continue to use a shared presentable image
    /// obtained from oldSwapchain until a presentable image is acquired from
    /// the new swapchain, as long as it has not entered a state that causes
    /// it to return VK_ERROR_OUT_OF_DATE_KHR.
    pub fn old_swapchain<'s, H>(&'s mut self, old_swapchain: H)
            -> &'s mut SwapchainKhrBuilder<'b>
            where H: Handle<Target=SwapchainKhrHandle> {
        self.create_info.set_old_swapchain(old_swapchain);
        self
    }

    /// Builds and returns a new `SwapchainKhr`.
    pub fn build(&mut self, device: Device) -> VdResult<SwapchainKhr> {
        let image_format = self.create_info.image_format().clone();
        let extent = self.create_info.image_extent().clone();

        let handle = unsafe { device.create_swapchain_khr(&self.create_info, None)? };

        let image_handles = unsafe { device.get_swapchain_images_khr(handle)? };

        Ok(SwapchainKhr {
            inner: Arc::new(Inner {
                handle,
                device,
                surface: self.surface.cloned()
                    .expect("unable to create swapchain: no surface specified"),
                images: image_handles,
                image_format: image_format,
                extent,
            })
        })
    }
}