use std::ptr;
use smallvec::SmallVec;
use vks;
use ::{VooResult, Instance, Device, Surface};

pub struct QueueFamilyIndices {
    // family_idx: i32,
    physical_device: vks::VkPhysicalDevice,
    flags: vks::VkQueueFlags,
    pub flag_idxs: SmallVec<[i32; 64]>,
    pub presentation_support_idxs: SmallVec<[i32; 64]>,
}

impl QueueFamilyIndices {
    pub fn new(physical_device: vks::VkPhysicalDevice, flags: vks::VkQueueFlags) -> QueueFamilyIndices {
        QueueFamilyIndices {
            flag_idxs: SmallVec::new(),
            presentation_support_idxs: SmallVec::new(),
            physical_device,
            flags
        }
    }

    pub fn is_complete(&self) -> bool {
        // self.family_idx >= 0
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

pub fn queue_families(instance: &Instance, surface: &Surface, device: vks::VkPhysicalDevice,
        queue_flags: vks::VkQueueFlags) -> QueueFamilyIndices
{
    let mut indices = QueueFamilyIndices::new(device, queue_flags);
    let mut queue_family_count = 0u32;
    let mut queue_families: Vec<vks::VkQueueFamilyProperties>;

    unsafe {
        instance.vk().core.vkGetPhysicalDeviceQueueFamilyProperties(device, &mut queue_family_count, ptr::null_mut());
        queue_families = Vec::with_capacity(queue_family_count as usize);
        queue_families.set_len(queue_family_count as usize);
        instance.vk().core.vkGetPhysicalDeviceQueueFamilyProperties(device, &mut queue_family_count, queue_families.as_mut_ptr());
    }

    let mut i = 0i32;
    for queue_family in &queue_families {
        if queue_family.queueCount > 0 && queue_family.queueFlags & queue_flags != 0 {
            indices.flag_idxs.push(i);
        }

        let mut presentation_support: vks::VkBool32 = vks::VK_FALSE;
        unsafe {
            ::check(instance.vk().khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR(device, i as u32, surface.handle(),
                &mut presentation_support));
        }
        if queue_family.queueCount > 0 && presentation_support != 0 {
            indices.presentation_support_idxs.push(i);
        }

        if indices.is_complete() {
            break;
        }
        i += 1;
    }
    indices
}


pub struct Queue {
    handle: vks::VkQueue,
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
    pub unsafe fn new(device: Device, queue_family_index: u32, queue_index: u32) -> VooResult<Queue> {
        let mut handle = ptr::null_mut();
        device.vk().core.vkGetDeviceQueue(device.handle(), queue_family_index, queue_index, &mut handle);

        Ok(Queue {
            handle,
            device,
            family_idx: queue_family_index,
            idx: queue_index,
        })
    }
}