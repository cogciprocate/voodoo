
use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device};

#[derive(Debug)]
struct Inner {
    handle: vks::VkDeviceMemory,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct DeviceMemory {
    inner: Arc<Inner>,
}

impl DeviceMemory {
    /// Returns a new `DeviceMemoryBuilder`.
    pub fn builder() -> DeviceMemoryBuilder {
        DeviceMemoryBuilder::new()
    }

    pub fn new(device: Device, allocation_size: u64, memory_type_index: u32) -> VooResult<DeviceMemory> {
        DeviceMemoryBuilder::new()
            .allocation_size(allocation_size)
            .memory_type_index(memory_type_index)
            .build(device)
    }

    pub fn handle(&self) -> vks::VkDeviceMemory {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkFreeMemory(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `DeviceMemory`.
//
// typedef struct VkMemoryAllocateInfo {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkDeviceSize       allocationSize;
//     uint32_t           memoryTypeIndex;
// } VkMemoryAllocateInfo;
//
#[derive(Debug, Clone)]
pub struct DeviceMemoryBuilder {
    allocate_info: vks::VkMemoryAllocateInfo,
}

impl DeviceMemoryBuilder {
    /// Returns a new render pass builder.
    pub fn new() -> DeviceMemoryBuilder {
        DeviceMemoryBuilder {
            allocate_info: vks::VkMemoryAllocateInfo::default(),
        }
    }

    /// Specifies the size of the allocation in bytes
    pub fn allocation_size<'s>(&'s mut self, allocation_size: vks::VkDeviceSize)
            -> &'s mut DeviceMemoryBuilder {
        self.allocate_info.allocationSize = allocation_size;
        self
    }

    /// Specifies the memory type index, which selects the properties of the
    /// memory to be allocated, as well as the heap the memory will come from.
    pub fn memory_type_index<'s>(&'s mut self, memory_type_index: u32)
            -> &'s mut DeviceMemoryBuilder {
        self.allocate_info.memoryTypeIndex = memory_type_index;
        self
    }

    /// Creates and returns a new `DeviceMemory`
    pub fn build(&self, device: Device) -> VooResult<DeviceMemory> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkAllocateMemory(device.handle(),
                &self.allocate_info, ptr::null(), &mut handle));
        }

        Ok(DeviceMemory {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}