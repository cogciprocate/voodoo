
use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device, Surface};


#[derive(Debug)]
struct Inner {
    handle: vks::VkCommandPool,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct CommandPool {
    inner: Arc<Inner>,
}

impl CommandPool {
    pub fn new(device: Device, surface: &Surface, queue_family_flags: vks::VkQueueFlags)
        -> VooResult<CommandPool>
    {
        let queue_family_idx = ::queue_families(device.instance(), surface,
            device.physical_device(), queue_family_flags).family_idxs()[0];

        let create_info = vks::VkCommandPoolCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null(),
            // vks::VK_COMMAND_POOL_CREATE_TRANSIENT_BIT
            // vks::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT
            flags: 0,
            queueFamilyIndex: queue_family_idx as u32,
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateCommandPool(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        Ok(CommandPool {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkCommandPool {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyCommandPool(self.device.handle(), self.handle, ptr::null());
        }
    }
}