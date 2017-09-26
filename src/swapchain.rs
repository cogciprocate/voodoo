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

    // // pub fn new(surface: Surface, device: Device, queue_flags: vks::VkQueueFlags,
    // //         window_size: Option<vks::VkExtent2D>, old_swapchain: Option<Swapchain>) -> VooResult<Swapchain>
    // pub fn new(device: Device, surface: Surface, create_info: vks::VkSwapchainCreateInfoKHR)
    //         -> VooResult<Swapchain> {
    //     // let swapchain_details: SwapchainSupportDetails = SwapchainSupportDetails::new(device.instance(),
    //     //     &surface, device.physical_device());
    //     // let surface_format = choose_swap_surface_format(&swapchain_details.formats);
    //     // let present_mode = choose_swap_present_mode(&swapchain_details.present_modes);
    //     // let extent = choose_swap_extent(&swapchain_details.capabilities, window_size);

    //     // // TODO: REVISIT THIS: https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Swap_chain
    //     // let mut image_count = swapchain_details.capabilities.minImageCount + 1;
    //     // if swapchain_details.capabilities.maxImageCount > 0 &&
    //     //         image_count > swapchain_details.capabilities.maxImageCount
    //     // {
    //     //     image_count = swapchain_details.capabilities.maxImageCount;
    //     // }

    //     // let indices = queue::queue_families(device.instance(), &surface,
    //     //     device.physical_device(), queue_flags);
    //     // let queue_family_indices = [indices.flag_idxs[0] as u32, indices.presentation_support_idxs[0] as u32];

    //     // let (image_sharing_mode, queue_family_index_count, p_queue_family_indices);
    //     // if queue_family_indices[0] != queue_family_indices[1] {
    //     //     image_sharing_mode = vks::VK_SHARING_MODE_CONCURRENT;
    //     //     queue_family_index_count = 2;
    //     //     p_queue_family_indices = queue_family_indices.as_ptr();
    //     // } else {
    //     //     image_sharing_mode = vks::VK_SHARING_MODE_EXCLUSIVE;
    //     //     queue_family_index_count = 0; // Optional
    //     //     p_queue_family_indices = ptr::null(); // Optional
    //     // }

    //     // // let image_extent = vks::VkExtent2D { width: extent.width, height: extent.height };

    //     // let create_info = vks::khr_swapchain::VkSwapchainCreateInfoKHR {
    //     //     sType: vks::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
    //     //     pNext: ptr::null(),
    //     //     flags: 0,
    //     //     surface: surface.handle(),
    //     //     minImageCount: image_count,
    //     //     imageFormat: surface_format.format,
    //     //     imageColorSpace: surface_format.colorSpace,
    //     //     imageExtent: extent.clone(),
    //     //     imageArrayLayers: 1,
    //     //     imageUsage: vks::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
    //     //     imageSharingMode: image_sharing_mode,
    //     //     queueFamilyIndexCount: queue_family_index_count,
    //     //     pQueueFamilyIndices: p_queue_family_indices,
    //     //     preTransform: swapchain_details.capabilities.currentTransform,
    //     //     compositeAlpha: vks::khr_surface::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
    //     //     presentMode: present_mode,
    //     //     clipped: vks::VK_TRUE,
    //     //     oldSwapchain: old_swapchain.map(|sc| sc.handle()).unwrap_or(0),
    //     // };
    //     let image_format = create_info.imageFormat.clone();
    //     let extent = create_info.imageExtent.clone();

    //     let mut handle = 0;
    //     let res = unsafe { device.proc_addr_loader().vkCreateSwapchainKHR(device.handle(), &create_info, ptr::null(), &mut handle) };
    //     if res != vks::VK_SUCCESS {
    //         panic!("failed to create swap chain!");
    //     }

    //     let mut image_count = 0;
    //     let mut images = SmallVec::new();
    //     unsafe {
    //         ::check(device.proc_addr_loader().vkGetSwapchainImagesKHR(device.handle(), handle, &mut image_count, ptr::null_mut()));
    //         images.set_len(image_count as usize);
    //         ::check(device.proc_addr_loader().vkGetSwapchainImagesKHR(device.handle(), handle, &mut image_count, images.as_mut_ptr()));
    //     }

    //     Ok(Swapchain {
    //         inner: Arc::new(Inner {
    //             handle,
    //             device,
    //             // surface,
    //             images,
    //             image_format: image_format,
    //             extent,
    //         })
    //     })
    // }

    pub fn images(&self) -> &[vks::VkImage] {
        &self.inner.images
    }

    pub fn image_format(&self) -> vks::VkFormat {
        self.inner.image_format
    }

    pub fn extent(&self) -> &vks::VkExtent2D {
        // vks::VkExtent2D { width: self.inner.extent.width, height: self.inner.extent.height }
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

    /// Specifies the create flags.
    pub fn flags<'s>(&'s mut self, flags: vks::VkSwapchainCreateFlagsKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.flags = flags;
        self
    }

    pub fn surface<'s>(&'s mut self, surface: Surface)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.surface = surface.handle();
        self.surface = Some(surface);
        self
    }

    pub fn min_image_count<'s>(&'s mut self, min_image_count: u32)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.minImageCount = min_image_count;
        self
    }

    pub fn image_format<'s>(&'s mut self, image_format: vks::VkFormat)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageFormat = image_format;
        self
    }

    pub fn image_color_space<'s>(&'s mut self, image_color_space: vks::VkColorSpaceKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageColorSpace = image_color_space;
        self
    }

    pub fn image_extent<'s>(&'s mut self, image_extent: vks::VkExtent2D)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageExtent = image_extent;
        self
    }

    pub fn image_array_layers<'s>(&'s mut self, image_array_layers: u32)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageArrayLayers = image_array_layers;
        self
    }

    pub fn image_usage<'s>(&'s mut self, image_usage: vks::VkImageUsageFlags)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageUsage = image_usage;
        self
    }

    pub fn image_sharing_mode<'s>(&'s mut self, image_sharing_mode: vks::VkSharingMode)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.imageSharingMode = image_sharing_mode;
        self
    }

    pub fn queue_family_indices<'s, 'qfi>(&'s mut self, queue_family_indices: Option<&'qfi [u32]>)
            -> &'s mut SwapchainBuilder<'sc>
            where 'qfi: 'sc {
        if let Some(qfis) = queue_family_indices {
            self.create_info.queueFamilyIndexCount = qfis.len() as u32;
            self.create_info.pQueueFamilyIndices = qfis.as_ptr();
        } else {
            self.create_info.queueFamilyIndexCount = 0;
            self.create_info.pQueueFamilyIndices = ptr::null();
        }
        self
    }

    pub fn pre_transform<'s>(&'s mut self, pre_transform: vks::VkSurfaceTransformFlagBitsKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.preTransform = pre_transform;
        self
    }

    pub fn composite_alpha<'s>(&'s mut self, composite_alpha: vks::VkCompositeAlphaFlagBitsKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.compositeAlpha = composite_alpha;
        self
    }

    pub fn present_mode<'s>(&'s mut self, present_mode: vks::VkPresentModeKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.presentMode = present_mode;
        self
    }

    pub fn clipped<'s>(&'s mut self, clipped: bool)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.clipped = if clipped { vks::VK_TRUE } else { vks::VK_FALSE };
        self
    }

    pub fn old_swapchain<'s>(&'s mut self, old_swapchain: vks::VkSwapchainKHR)
            -> &'s mut SwapchainBuilder<'sc> {
        self.create_info.oldSwapchain = old_swapchain;
        self
    }

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