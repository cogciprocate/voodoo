use std::sync::Arc;
use vks;
use ::{VdResult, CallResult, Handle, Device, FenceCreateFlags, FenceCreateInfo};

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

    /// Returns this object's handle.
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
    /// Creates and returns a new fence.
    pub fn new(device: Device, flags: FenceCreateFlags) -> VdResult<Fence> {
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

    /// Returns this object's handle.
    pub fn handle(&self) -> FenceHandle {
        self.inner.handle
    }

    /// Returns a reference to this object's associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }

    /// Returns the status of this fence.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkGetFenceStatus.html
    //
    pub fn status(&self) -> VdResult<FenceStatus> {
        unsafe { Ok(FenceStatus::from(self.inner.device.get_fence_status(self.handle())?)) }
    }
}

unsafe impl<'h> Handle for &'h Fence {
    type Target = FenceHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}
