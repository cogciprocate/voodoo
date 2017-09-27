
use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device};

#[derive(Debug)]
struct Inner {
    handle: vks::VkDescriptorPool,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct DescriptorPool {
    inner: Arc<Inner>,
}

impl DescriptorPool {
    pub fn new(device: Device) -> VooResult<DescriptorPool> {
        let pool_sizes = [
            vks::VkDescriptorPoolSize {
                type_: vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptorCount: 1,
            },
            vks::VkDescriptorPoolSize {
                type_: vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                descriptorCount: 1,
            },
        ];

        let create_info = vks::VkDescriptorPoolCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: ptr::null(),
            // optional flag similar to command pools that determines if
            // individual descriptor sets can be freed or not:
            // `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT`:
            flags: 0,
            maxSets: 1,
            poolSizeCount: pool_sizes.len() as u32,
            pPoolSizes: pool_sizes.as_ptr(),
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().vkCreateDescriptorPool(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        Ok(DescriptorPool {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkDescriptorPool {
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
            self.device.proc_addr_loader().vkDestroyDescriptorPool(self.device.handle(), self.handle, ptr::null());
        }
    }
}