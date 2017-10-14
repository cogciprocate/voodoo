use vks;
use ::{VdResult, Device, Handle, SubmitInfo, FenceHandle, BindSparseInfo, PresentInfoKhr};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct QueueHandle(pub(crate) vks::VkQueue);

impl QueueHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkQueue {
        self.0
    }
}

unsafe impl Handle for QueueHandle {
    type Target = QueueHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}

#[derive(Clone, Debug)]
pub struct Queue {
    handle: QueueHandle,
    device: Device,
    family_idx: u32,
    idx: u32,
}

impl Queue {
    /// Assembles and returns a new `Queue` from parts.
    pub(crate) unsafe fn from_parts(handle: QueueHandle, device: Device, queue_family_index: u32,
            queue_index: u32) -> Queue {
        Queue {
            handle,
            device,
            family_idx: queue_family_index,
            idx: queue_index,
        }
    }

    /// Returns a reference to this object's associated device.
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns this queue's family index.
    pub fn family_index(&self) -> u32 {
        self.family_idx
    }

    /// Returns this queue's index within its family.
    pub fn index(&self) -> u32 {
        self.idx
    }

    /// Submits a sequence of semaphores or command buffers to this queue.
    #[inline]
    pub fn submit(&self, submit_info: &[SubmitInfo], fence: Option<FenceHandle>) -> VdResult<()> {
        unsafe { self.device.queue_submit(self.handle, submit_info, fence) }
    }

    /// Waits for this queue to become idle.
    pub fn wait_idle(&self) {
        self.device.queue_wait_idle(self.handle)
    }

    /// Binds device memory to a sparse resource object.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkQueueBindSparse.html
    //
    #[inline]
    pub fn bind_sparse<Q, F>(&self, bind_info: &[BindSparseInfo], fence: F) -> VdResult<()>
            where Q: Handle<Target=QueueHandle>, F: Handle<Target=FenceHandle> {
        unsafe { self.device.queue_bind_sparse(self.handle, bind_info, fence) }
    }

    /// Queues an image for presentation.
    ///
    /// https://manned.org/vkQueuePresentKHR.3
    //
    #[inline]
    pub fn present_khr(&self, present_info: &PresentInfoKhr) -> VdResult<()> {
        unsafe { self.device.queue_present_khr(self.handle, present_info) }
    }
}

unsafe impl<'a> Handle for &'a Queue {
    type Target = QueueHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.handle
    }
}