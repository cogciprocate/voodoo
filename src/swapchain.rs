use std::sync::Arc;
use std::mem;
use std::ptr;
use std::cmp;
use std::fmt;
use smallvec::SmallVec;
use vks;
use ::{queue, VooResult, Instance, Surface, Device};


pub struct SwapchainSupportDetails {
    pub capabilities: vks::khr_surface::VkSurfaceCapabilitiesKHR,
    pub formats: Vec<vks::khr_surface::VkSurfaceFormatKHR>,
    pub present_modes: Vec<vks::khr_surface::VkPresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn new(instance: &Instance, surface: &Surface, physical_device: vks::VkPhysicalDevice)
            -> SwapchainSupportDetails
    {
        unsafe {
            // Capabilities:
            let mut capabilities: vks::khr_surface::VkSurfaceCapabilitiesKHR = mem::uninitialized();
            instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                physical_device, surface.handle(), &mut capabilities);

            // Formats:
            let mut format_count = 0u32;
            instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device,
                surface.handle(), &mut format_count, ptr::null_mut());
            let mut formats: Vec<vks::khr_surface::VkSurfaceFormatKHR> = Vec::with_capacity(format_count as usize);
            formats.set_len(format_count as usize);
            if format_count != 0 {
                instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device,
                    surface.handle(), &mut format_count, formats.as_mut_ptr());
            }

            // Present Modes:
            let mut present_mode_count = 0u32;
            instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device,
                surface.handle(), &mut present_mode_count, ptr::null_mut());
            let mut present_modes: Vec<vks::khr_surface::VkPresentModeKHR> = Vec::with_capacity(present_mode_count as usize);
            present_modes.set_len(present_mode_count as usize);
            if present_mode_count != 0 {
                instance.proc_addr_loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device,
                    surface.handle(), &mut present_mode_count, present_modes.as_mut_ptr());
            }

            SwapchainSupportDetails {
                capabilities,
                formats,
                present_modes,
            }
        }
    }
}

fn choose_swap_surface_format(available_formats: &[vks::khr_surface::VkSurfaceFormatKHR]) -> vks::khr_surface::VkSurfaceFormatKHR {
    if available_formats.len() == 1 && available_formats[0].format == vks::VK_FORMAT_UNDEFINED {
        return vks::khr_surface::VkSurfaceFormatKHR {
            format: vks::VK_FORMAT_B8G8R8A8_UNORM,
            colorSpace: vks::khr_surface::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        };
    }

    for available_format in available_formats {
        if available_format.format == vks::VK_FORMAT_B8G8R8A8_UNORM &&
                available_format.colorSpace == vks::khr_surface::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
        {
            return vks::khr_surface::VkSurfaceFormatKHR {
                format: vks::VK_FORMAT_B8G8R8A8_UNORM,
                colorSpace: vks::khr_surface::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            };
        }
    }

    vks::khr_surface::VkSurfaceFormatKHR {
        format: available_formats[0].format,
        colorSpace: available_formats[0].colorSpace,
    }
}

fn choose_swap_present_mode(available_present_modes: &[vks::khr_surface::VkPresentModeKHR]) -> vks::khr_surface::VkPresentModeKHR {
    let mut best_mode = vks::khr_surface::VK_PRESENT_MODE_FIFO_KHR;
    for &available_present_mode in available_present_modes {
        // if available_present_mode == vks::khr_surface::VK_PRESENT_MODE_MAILBOX_KHR {
        if available_present_mode == vks::khr_surface::VK_PRESENT_MODE_FIFO_KHR {
            return available_present_mode;
        } else if available_present_mode == vks::khr_surface::VK_PRESENT_MODE_IMMEDIATE_KHR {
            best_mode = available_present_mode;
        }
    }
    best_mode
}

fn choose_swap_extent(capabilities: &vks::khr_surface::VkSurfaceCapabilitiesKHR,
        window_size: Option<vks::VkExtent2D>) -> vks::VkExtent2D
{
    if capabilities.currentExtent.width != u32::max_value() {
        // return vks::VkExtent2D { width: capabilities.currentExtent.width,
        //     height: capabilities.currentExtent.height };
        // println!("Swap Extent (current): {:?}", &capabilities.currentExtent);
        return capabilities.currentExtent.clone();
    } else {
        let mut actual_extent = window_size
            .unwrap_or(vks::VkExtent2D { width: 1024, height: 768 });
        actual_extent.width = cmp::max(capabilities.minImageExtent.width,
            cmp::min(capabilities.maxImageExtent.width, actual_extent.width));
        actual_extent.height = cmp::max(capabilities.minImageExtent.height,
            cmp::min(capabilities.maxImageExtent.height, actual_extent.height));
        // println!("Swap Extent (actual): {:?}", &actual_extent);
        return actual_extent
    }
}


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
    pub fn new(surface: Surface, device: Device, queue_flags: vks::VkQueueFlags,
            window_size: Option<vks::VkExtent2D>, old_swapchain: Option<Swapchain>) -> VooResult<Swapchain>
    {
        let swapchain_details: SwapchainSupportDetails = SwapchainSupportDetails::new(device.instance(),
            &surface, device.physical_device());
        let surface_format = choose_swap_surface_format(&swapchain_details.formats);
        let present_mode = choose_swap_present_mode(&swapchain_details.present_modes);
        let extent = choose_swap_extent(&swapchain_details.capabilities, window_size);

        // TODO: REVISIT THIS: https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Swap_chain
        let mut image_count = swapchain_details.capabilities.minImageCount + 1;
        if swapchain_details.capabilities.maxImageCount > 0 &&
                image_count > swapchain_details.capabilities.maxImageCount
        {
            image_count = swapchain_details.capabilities.maxImageCount;
        }

        let indices = queue::queue_families(device.instance(), &surface, device.physical_device(), queue_flags);
        let queue_family_indices = [indices.flag_idxs[0] as u32, indices.presentation_support_idxs[0] as u32];

        let (image_sharing_mode, queue_family_index_count, p_queue_family_indices);
        if queue_family_indices[0] != queue_family_indices[1] {
            image_sharing_mode = vks::VK_SHARING_MODE_CONCURRENT;
            queue_family_index_count = 2;
            p_queue_family_indices = queue_family_indices.as_ptr();
        } else {
            image_sharing_mode = vks::VK_SHARING_MODE_EXCLUSIVE;
            queue_family_index_count = 0; // Optional
            p_queue_family_indices = ptr::null(); // Optional
        }

        // let image_extent = vks::VkExtent2D { width: extent.width, height: extent.height };

        let create_info = vks::khr_swapchain::VkSwapchainCreateInfoKHR {
            sType: vks::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: 0,
            surface: surface.handle(),
            minImageCount: image_count,
            imageFormat: surface_format.format,
            imageColorSpace: surface_format.colorSpace,
            imageExtent: extent.clone(),
            imageArrayLayers: 1,
            imageUsage: vks::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            imageSharingMode: image_sharing_mode,
            queueFamilyIndexCount: queue_family_index_count,
            pQueueFamilyIndices: p_queue_family_indices,
            preTransform: swapchain_details.capabilities.currentTransform,
            compositeAlpha: vks::khr_surface::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            presentMode: present_mode,
            clipped: vks::VK_TRUE,
            oldSwapchain: old_swapchain.map(|sc| sc.handle()).unwrap_or(0),
        };

        let mut handle = 0;
        let res = unsafe { device.proc_addr_loader().vkCreateSwapchainKHR(device.handle(), &create_info, ptr::null(), &mut handle) };
        if res != vks::VK_SUCCESS {
            panic!("failed to create swap chain!");
        }

        let mut image_count = 0;
        let mut images = SmallVec::new();
        unsafe {
            ::check(device.proc_addr_loader().vkGetSwapchainImagesKHR(device.handle(), handle, &mut image_count, ptr::null_mut()));
            images.set_len(image_count as usize);
            ::check(device.proc_addr_loader().vkGetSwapchainImagesKHR(device.handle(), handle, &mut image_count, images.as_mut_ptr()));
        }

        Ok(Swapchain {
            inner: Arc::new(Inner {
                handle,
                device,
                surface,
                images,
                image_format: surface_format.format,
                extent,
            })
        })
    }

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

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Swapchain")
    }
}