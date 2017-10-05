
use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device};



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SemaphoreHandle(pub(crate) vks::VkSemaphore);

#[derive(Debug)]
struct Inner {
    handle: vks::VkSemaphore,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Semaphore {
    inner: Arc<Inner>,
}

impl Semaphore {
    pub fn new(device: Device) -> VooResult<Semaphore> {
        let create_info = vks::VkSemaphoreCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateSemaphore(device.handle().0, &create_info,
                ptr::null(), &mut handle));
        }

        Ok(Semaphore {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkSemaphore {
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
            self.device.proc_addr_loader().core.vkDestroySemaphore(self.device.handle().0, self.handle, ptr::null());
        }
    }
}

