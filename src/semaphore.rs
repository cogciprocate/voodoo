use std::sync::Arc;
use vks;
use ::{VdResult, Device, Handle, SemaphoreCreateFlags, SemaphoreCreateInfo};



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SemaphoreHandle(pub(crate) vks::VkSemaphore);

impl SemaphoreHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkSemaphore {
        self.0
    }
}

unsafe impl Handle for SemaphoreHandle {
    type Target = SemaphoreHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: SemaphoreHandle,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Semaphore {
    inner: Arc<Inner>,
}

impl Semaphore {
    /// Creates and returns a new `Semaphore`.
    pub fn new(device: Device, flags: SemaphoreCreateFlags) -> VdResult<Semaphore> {
        let create_info = SemaphoreCreateInfo::builder()
            .flags(flags)
            .build();

        let handle = unsafe { device.create_semaphore(&create_info, None)? };

        Ok(Semaphore {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> SemaphoreHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'h> Handle for &'h Semaphore {
    type Target = SemaphoreHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_semaphore(self.handle, None);
        }
    }
}

