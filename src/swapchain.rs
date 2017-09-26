use std::sync::Arc;
use std::mem;
use std::ptr;
use std::cmp;
use std::fmt;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{queue, VooResult, Instance, Surface, Device, PhysicalDevice};


pub struct SwapchainSupportDetails {
    pub capabilities: vks::khr_surface::VkSurfaceCapabilitiesKHR,
    pub formats: SmallVec<[vks::khr_surface::VkSurfaceFormatKHR; 64]>,
    pub present_modes: SmallVec<[vks::khr_surface::VkPresentModeKHR; 16]>,
}

impl SwapchainSupportDetails {
    pub fn new(instance: &Instance, surface: &Surface, physical_device: &PhysicalDevice)
            -> SwapchainSupportDetails {
        let capabilities = physical_device.capabilities(surface);
        let formats = physical_device.formats(surface);
        let present_modes = physical_device.present_modes(surface);

        SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        }
    }
}


#[derive(Debug)]
struct Inner {
    handle: vks::khr_swapchain::VkSwapchainKHR,
    device: Device,
    surface: Surface,
    images: SmallVec<[vks::VkImage; 8]>,
    image_format: vks::VkFormat,
    extent: vks::VkExtent2D,
}

#[derive(Debug, Clone)]
pub struct Swapchain {
    inner: Arc<Inner>,
}

impl Swapchain {
    pub fn builder<'sc>() -> SwapchainBuilder<'sc> {
        SwapchainBuilder::new()
    }

    pub fn images(&self) -> &[vks::VkImage] {
        &self.inner.images
    }

    pub fn image_format(&self) -> vks::VkFormat {
        self.inner.image_format
    }

    pub fn extent(&self) -> &vks::VkExtent2D {
        &self.inner.extent
    }

    pub fn handle(&self) -> vks::VkShaderModule {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().vkDestroySwapchainKHR(self.device.handle(), self.handle, ptr::null());
        }
    }
}

unsafe impl Sync for Swapchain {}


/// A Swapchain builder.
//
// typedef struct VkSwapchainCreateInfoKHR {
//     VkStructureType                  sType;
//     const void*                      pNext;
//     VkSwapchainCreateFlagsKHR        flags;
//     VkSurfaceKHR                     surface;
//     uint32_t                         minImageCount;
//     VkFormat                         imageFormat;
//     VkColorSpaceKHR                  imageColorSpace;
//     VkExtent2D                       imageExtent;
//     uint32_t                         imageArrayLayers;
//     VkImageUsageFlags                imageUsage;
//     VkSharingMode                    imageSharingMode;
//     uint32_t                         queueFamilyIndexCount;
//     const uint32_t*                  pQueueFamilyIndices;
//     VkSurfaceTransformFlagBitsKHR    preTransform;
//     VkCompositeAlphaFlagBitsKHR      compositeAlpha;
//     VkPresentModeKHR                 presentMode;
//     VkBool32                         clipped;
//     VkSwapchainKHR                   oldSwapchain;
// } VkSwapchainCreateInfoKHR;
//
#[derive(Debug, Clone)]
pub struct SwapchainBuilder<'sc> {
    create_info: vks::VkSwapchainCreateInfoKHR,
    // Must keep alive to maintain destruction order:
    surface: Option<Surface>,
    _p: PhantomData<&'sc ()>,
}

impl<'sc> SwapchainBuilder<'sc> {
    /// Returns a new swapchain builder.
    pub fn new() -> SwapchainBuilder<'sc> {
        SwapchainBuilder {
            create_info: vks::VkSwapchainCreateInfoKHR::default(),
            surface: None,
            _p: PhantomData,
        }
    }

    /// Specifies the parameters of swapchain creation.
    pub fn flags<'s>(&'s mut self, flags: vks::VkSwapchainCreateFlagsKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.flags = flags;
        self
    }

    /// Specifies the surface that the swapchain will present images to.
    pub fn surface<'s>(&'s mut self, surface: Surface)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.surface = surface.handle();
        self.surface = Some(surface);
        self
    }

    /// Specifies the minimum number of presentable images that the
    /// application needs. The platform will either create the swapchain with
    /// at least that many images, or will fail to create the swapchain.
    pub fn min_image_count<'s>(&'s mut self, min_image_count: u32)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.minImageCount = min_image_count;
        self
    }

    /// Specifies the format that is valid for swapchains on the specified
    /// surface.
    pub fn image_format<'s>(&'s mut self, image_format: vks::VkFormat)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageFormat = image_format;
        self
    }

    /// Specifies the color space that is valid for swapchains on the
    /// specified surface.
    pub fn image_color_space<'s>(&'s mut self, image_color_space: vks::VkColorSpaceKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageColorSpace = image_color_space;
        self
    }

    /// Specifies the size (in pixels) of the swapchain. Behavior is
    /// platform-dependent when the image extent does not match the surface’s
    /// current extent as returned by `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`.
    pub fn image_extent<'s>(&'s mut self, image_extent: vks::VkExtent2D)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageExtent = image_extent;
        self
    }

    /// Specifies the number of views in a multiview/stereo surface. For
    /// non-stereoscopic-3D applications, this value is 1.
    pub fn image_array_layers<'s>(&'s mut self, image_array_layers: u32)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageArrayLayers = image_array_layers;
        self
    }

    /// Specifies the bitmask of `ImageUsageFlagBits`, indicating how the
    /// application will use the swapchain’s presentable images
    pub fn image_usage<'s>(&'s mut self, image_usage: vks::VkImageUsageFlags)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageUsage = image_usage;
        self
    }

    /// Specifies the sharing mode used for the images of the swapchain.
    pub fn image_sharing_mode<'s>(&'s mut self, image_sharing_mode: vks::VkSharingMode)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageSharingMode = image_sharing_mode;
        self
    }

    /// Specifies the queue family indices having access to the images of the
    /// swapchain in case imageSharingMode is VK_SHARING_MODE_CONCURRENT.
    pub fn queue_family_indices<'s, 'qfi>(&'s mut self, queue_family_indices: &'qfi [u32])
            -> &'s mut SwapchainBuilder<'sc>
            where 'qfi: 'sc {
        self.create_info.queueFamilyIndexCount = queue_family_indices.len() as u32;
        self.create_info.pQueueFamilyIndices = queue_family_indices.as_ptr();
        self
    }

    /// Specifies the bitmask of VkSurfaceTransformFlagBitsKHR, describing the
    /// transform, relative to the presentation engine’s natural orientation,
    /// applied to the image content prior to presentation. If it does not
    /// match the currentTransform value returned by
    /// vkGetPhysicalDeviceSurfaceCapabilitiesKHR, the presentation engine
    /// will transform the image content as part of the presentation operation.
    pub fn pre_transform<'s>(&'s mut self, pre_transform: vks::VkSurfaceTransformFlagBitsKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.preTransform = pre_transform;
        self
    }

    /// Specifies the bitmask of VkCompositeAlphaFlagBitsKHR indicating the
    /// alpha compositing mode to use when this surface is composited together
    /// with other surfaces on certain window systems.
    pub fn composite_alpha<'s>(&'s mut self, composite_alpha: vks::VkCompositeAlphaFlagBitsKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.compositeAlpha = composite_alpha;
        self
    }

    /// Specifies the presentation mode the swapchain will use. A swapchain’s
    /// present mode determines how incoming present requests will be
    /// processed and queued internally.
    pub fn present_mode<'s>(&'s mut self, present_mode: vks::VkPresentModeKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.presentMode = present_mode;
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
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.clipped = if clipped { vks::VK_TRUE } else { vks::VK_FALSE };
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
    pub fn old_swapchain<'s>(&'s mut self, old_swapchain: vks::VkSwapchainKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.oldSwapchain = old_swapchain;
        self
    }

    /// Builds and returns a new `Swapchain`.
    pub fn build(&mut self, device: Device) -> VooResult<Swapchain> {
        let image_format = self.create_info.imageFormat.clone();
        let extent = self.create_info.imageExtent.clone();

        let mut handle = 0;
        let res = unsafe { device.proc_addr_loader().vkCreateSwapchainKHR(device.handle(),
            &self.create_info, ptr::null(), &mut handle) };
        if res != vks::VK_SUCCESS {
            panic!("failed to create swap chain!");
        }

        let mut image_count = 0;
        let mut images = SmallVec::new();
        unsafe {
            ::check(device.proc_addr_loader().vkGetSwapchainImagesKHR(device.handle(), handle,
                &mut image_count, ptr::null_mut()));
            images.set_len(image_count as usize);
            ::check(device.proc_addr_loader().vkGetSwapchainImagesKHR(device.handle(), handle,
                &mut image_count, images.as_mut_ptr()));
        }

        Ok(Swapchain {
            inner: Arc::new(Inner {
                handle,
                device,
                surface: self.surface.take()
                    .expect("unable to create swapchain: no surface specified"),
                images,
                image_format: image_format,
                extent,
            })
        })
    }
}