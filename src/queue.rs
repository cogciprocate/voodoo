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
    // pub fn new(device: Device, queue_family_index: u32, queue_index: u32) -> VdResult<Queue> {
    //     let handle = device.get_device_queue(queue_family_index, queue_index)
    //         .ok_or(Error::from(format!("Unable to get device queue with: family_index: {}, index: {}",
    //             queue_family_index, queue_index)))?;

    //     Ok(Queue {
    //         handle,
    //         device,
    //         family_idx: queue_family_index,
    //         idx: queue_index,
    //     })
    // }

    pub(crate) unsafe fn from_parts(handle: QueueHandle, device: Device, queue_family_index: u32,
            queue_index: u32) -> Queue {
        Queue {
            handle,
            device,
            family_idx: queue_family_index,
            idx: queue_index,
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn family_index(&self) -> u32 {
        self.family_idx
    }

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

    #[inline]
    pub fn bind_sparse<Q, F>(&self, bind_info: &[BindSparseInfo], fence: F) -> VdResult<()>
            where Q: Handle<Target=QueueHandle>, F: Handle<Target=FenceHandle> {
        unsafe { self.device.queue_bind_sparse(self.handle, bind_info, fence) }
    }

    #[inline]
    pub fn present_khr(&self, present_info: &PresentInfoKhr) -> VdResult<()> {
        unsafe { self.device.queue_present_khr(self.handle, present_info) }
    }
}

unsafe impl<'a> Handle for &'a Queue {
    type Target = QueueHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.handle
    }
}