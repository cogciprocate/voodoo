// use std::ptr;
use smallvec::SmallVec;
use vks;
use ::{VdResult, PhysicalDevice, Device, SurfaceKhr, QueueFlags, Handle, SubmitInfo, FenceHandle};



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


pub struct QueueFamilyIndices {
    _physical_device: PhysicalDevice,
    _flags: QueueFlags,
    pub flag_idxs: SmallVec<[i32; 64]>,
    pub presentation_support_idxs: SmallVec<[i32; 64]>,
}

impl QueueFamilyIndices {
    pub fn new(_physical_device: PhysicalDevice, _flags: QueueFlags) -> QueueFamilyIndices {
        QueueFamilyIndices {
            flag_idxs: SmallVec::new(),
            presentation_support_idxs: SmallVec::new(),
            _physical_device,
            _flags
        }
    }

    pub fn is_complete(&self) -> bool {
        self.flag_idxs.len() > 0
    }

    pub fn family_idxs(&self) -> &[i32] {
        &self.flag_idxs

        // let mut i = 0i32;
        // for queue_family in &queue_families {
        //     if (queue_family.queueCount > 0) && (queue_family.queueFlags & queue_flags) != 0 {
        //         indices.family_idx = i;
        //     }
        //     if indices.is_complete() {
        //         break;
        //     }
        //     i += 1;
        // }
        // indices
    }
}

pub fn queue_families(surface: &SurfaceKhr, physical_device: &PhysicalDevice,
        queue_flags: QueueFlags) -> VdResult<QueueFamilyIndices> {
    let mut indices = QueueFamilyIndices::new(physical_device.clone(), queue_flags);
    let queue_families = physical_device.queue_family_properties()?;

    let mut i = 0i32;
    for queue_family in &queue_families {
        if queue_family.queue_count() > 0 && queue_family.queue_flags().contains(queue_flags) {
            indices.flag_idxs.push(i);
        }

        let presentation_support = physical_device.surface_support_khr(i as u32, surface)?;
        if queue_family.queue_count() > 0 && presentation_support {
            indices.presentation_support_idxs.push(i);
        }

        if indices.is_complete() {
            break;
        }
        i += 1;
    }
    Ok(indices)
}


pub struct Queue {
    handle: QueueHandle,
    device: Device,
    family_idx: u32,
    idx: u32,
}

impl Queue {
    // Queue families:
    // QUEUE_COMPUTE_BIT
    // QUEUE_FAMILY_IGNORED
    // QUEUE_GRAPHICS_BIT
    // QUEUE_SPARSE_BINDING_BIT
    // QUEUE_TRANSFER_BIT
    pub fn new(device: Device, queue_family_index: u32, queue_index: u32) -> VdResult<Queue> {
        let handle = device.get_device_queue(queue_family_index, queue_index);

        Ok(Queue {
            handle,
            device,
            family_idx: queue_family_index,
            idx: queue_index,
        })
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
    pub fn submit(&self, submit_info: &[SubmitInfo], fence: Option<FenceHandle>) -> VdResult<()> {
        unsafe { self.device.queue_submit(self.handle, submit_info, fence) }
    }

    /// Waits for this queue to become idle.
    pub fn wait_idle(&self) {
        self.device.queue_wait_idle(self.handle)
    }
}

unsafe impl Handle for Queue {
    type Target = QueueHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.handle
    }
}