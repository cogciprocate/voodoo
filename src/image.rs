

use std::sync::Arc;
use std::ptr;
use std::mem;
use vks;
use ::{util, VooResult, Device, DeviceMemory};

#[derive(Debug)]
struct Inner {
    handle: vks::VkImage,
    device_memory: DeviceMemory,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Image {
    inner: Arc<Inner>,
}

impl Image {
    pub fn new(device: Device, extent: vks::VkExtent3D, format: vks::VkFormat,
            tiling: vks::VkImageTiling, usage: vks::VkImageUsageFlags,
            memory_properties: vks::VkMemoryPropertyFlags) -> VooResult<Image>
    {
        let create_info = vks::VkImageCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            imageType: vks::VK_IMAGE_TYPE_2D,
            // format: vks::VK_FORMAT_R8G8B8A8_UNORM,
            format,
            extent: extent,
            mipLevels: 1,
            arrayLayers: 1,
            samples: vks::VK_SAMPLE_COUNT_1_BIT,
            // tiling: vks::VK_IMAGE_TILING_OPTIMAL,
            tiling,
            // usage: vks::VK_IMAGE_USAGE_TRANSFER_DST_BIT | vks::VK_IMAGE_USAGE_SAMPLED_BIT,
            usage,
            sharingMode: vks::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null(),
            initialLayout: vks::VK_IMAGE_LAYOUT_UNDEFINED,
        };

        let mut handle = 0;
        unsafe {
            ::check(device.vk().vkCreateImage(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        // Memory Requirements:
        let mut mem_requirements: vks::VkMemoryRequirements;
        unsafe {
            mem_requirements = mem::uninitialized();
            device.vk().core.vkGetImageMemoryRequirements(device.handle(), handle,
                &mut mem_requirements);
        }

        let memory_type_index = ::find_memory_type(&device, mem_requirements.memoryTypeBits,
            memory_properties);

        let alloc_info = vks::VkMemoryAllocateInfo {
            sType: vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: ptr::null(),
            allocationSize: mem_requirements.size,
            memoryTypeIndex: memory_type_index,
        };

        println!("Image: {:?}", mem_requirements);

        let device_memory = DeviceMemory::new(device.clone(), mem_requirements.size,
            memory_type_index)?;

        unsafe {
            ::check(device.vk().vkBindImageMemory(device.handle(), handle,
                device_memory.handle(), 0));
        }

        Ok(Image {
            inner: Arc::new(Inner {
                handle,
                device_memory,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkImage {
        self.inner.handle
    }

    pub fn device_memory(&self) -> &DeviceMemory {
        &self.inner.device_memory
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.vk().vkDestroyImage(self.device.handle(), self.handle, ptr::null());
        }
    }
}