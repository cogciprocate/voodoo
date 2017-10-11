
use std::sync::Arc;
use vks;
use ::{VooResult, CallResult, Handle, Device, FenceCreateFlags, FenceCreateInfo};

pub enum FenceStatus {
    Signaled,
    Unsignaled,
    Error(CallResult),
}

impl From<CallResult> for FenceStatus {
    fn from(res: CallResult) -> FenceStatus {
        match res {
            CallResult::Success => FenceStatus::Signaled,
            CallResult::NotReady => FenceStatus::Unsignaled,
            _ => FenceStatus::Error(res),
        }
    }
}

impl From<i32> for FenceStatus {
    fn from(res: i32) -> FenceStatus {
        CallResult::from(res).into()
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct FenceHandle(pub(crate) vks::VkFence);

impl FenceHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkFence {
        self.0
    }
}

unsafe impl Handle for FenceHandle {
    type Target = FenceHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: FenceHandle,
    device: Device,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(self.handle, None);
        }
    }
}


#[derive(Debug, Clone)]
pub struct Fence {
    inner: Arc<Inner>,
}

impl Fence {
    pub fn new(device: Device, flags: FenceCreateFlags) -> VooResult<Fence> {
        let create_info = FenceCreateInfo::builder()
            .flags(flags)
            .build();

        let handle = unsafe { device.create_fence(&create_info, None)? };

        Ok(Fence {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> FenceHandle {
        self.inner.handle
    }

    pub fn status(&self) -> FenceStatus {
        unsafe { self.inner.device.get_fence_status(self.handle()).into() }
    }
}

unsafe impl<'h> Handle for &'h Fence {
    type Target = FenceHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}