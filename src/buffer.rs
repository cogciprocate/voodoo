
use std::sync::Arc;
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use vks;
use ::{util, VooResult, Device, DeviceMemory, PRINT};

#[derive(Debug)]
struct Inner {
    handle: vks::VkBuffer,
    // device_memory: DeviceMemory,
    memory_requirements: vks::VkMemoryRequirements,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    inner: Arc<Inner>,
}

impl Buffer {
    /// Returns a new `BufferBuilder`.
    pub fn builder<'b>() -> BufferBuilder<'b> {
        BufferBuilder::new()
    }

    pub fn new(device: Device, bytes: u64, usage: vks::VkBufferUsageFlags,
            sharing_mode: vks::VkSharingMode, /*memory_properties: vks::VkMemoryPropertyFlags*/)
            -> VooResult<Buffer>
    {
        let create_info = vks::VkBufferCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            size: bytes,
            usage: usage,
            sharingMode: sharing_mode,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null(),
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateBuffer(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        // // Memory Requirements:
        // let mut mem_requirements: vks::VkMemoryRequirements;
        // unsafe {
        //     mem_requirements = mem::uninitialized();
        //     device.proc_addr_loader().core.vkGetBufferMemoryRequirements(device.handle(), handle,
        //         &mut mem_requirements);
        // }


        // let memory_type_index = device.memory_type_index(mem_requirements.memoryTypeBits,
        //     memory_properties);

        // let alloc_info = vks::VkMemoryAllocateInfo {
        //     sType: vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        //     pNext: ptr::null(),
        //     allocationSize: mem_requirements.size,
        //     memoryTypeIndex: memory_type_index,
        // };

        // if PRINT { println!("Buffer: {:?}", mem_requirements); }

        // let device_memory = DeviceMemory::new(device.clone(), mem_requirements.size,
        //     memory_type_index)?;

        // unsafe {
        //     ::check(device.proc_addr_loader().core.vkBindBufferMemory(device.handle(), handle,
        //         device_memory.handle(), 0));
        // }

       // Memory Requirements:
        let mut memory_requirements: vks::VkMemoryRequirements;
        unsafe {
            memory_requirements = mem::uninitialized();
            device.proc_addr_loader().core.vkGetBufferMemoryRequirements(device.handle(),
                handle, &mut memory_requirements);
        }

        Ok(Buffer {
            inner: Arc::new(Inner {
                handle,
                device,
                // device_memory,
                memory_requirements,
            })
        })
    }

    pub fn handle(&self) -> vks::VkBuffer {
        self.inner.handle
    }

    // pub fn device_memory(&self) -> &DeviceMemory {
    //     &self.inner.device_memory
    // }

    pub fn memory_requirements(&self) -> &vks::VkMemoryRequirements {
        &self.inner.memory_requirements
    }

    /// Binds this buffer to device memory. `offset` is the start offset of the
    /// region of memory which is to be bound. The number of bytes returned in
    /// the VkMemoryRequirements::size member in memory, starting from
    /// memoryOffset bytes, will be bound to the specified buffer.
    pub fn bind_memory(&self, device_memory: &DeviceMemory, offset: vks::VkDeviceSize)
            -> VooResult<()> {
        unsafe {
            ::check(self.inner.device.proc_addr_loader().vkBindBufferMemory(
                self.inner.device.handle(), self.inner.handle, device_memory.handle(), offset));
        }
        Ok(())
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyBuffer(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `Buffer`.
//
// typedef struct VkBufferCreateInfo {
//     VkStructureType        sType;
//     const void*            pNext;
//     VkBufferCreateFlags    flags;
//     VkDeviceSize           size;
//     VkBufferUsageFlags     usage;
//     VkSharingMode          sharingMode;
//     uint32_t               queueFamilyIndexCount;
//     const uint32_t*        pQueueFamilyIndices;
// } VkBufferCreateInfo;
//
#[derive(Debug, Clone)]
pub struct BufferBuilder<'b> {
    create_info: vks::VkBufferCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> BufferBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> BufferBuilder<'b> {
        BufferBuilder {
            create_info: vks::VkBufferCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// flags is a bitmask of VkBufferCreateFlagBits specifying additional
    /// parameters of the buffer.
    pub fn flags<'s>(&'s mut self, flags: vks::VkBufferCreateFlags)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// size is the size in bytes of the buffer to be created.
    pub fn size<'s>(&'s mut self, size: vks::VkDeviceSize)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.size = size;
        self
    }

    /// usage is a bitmask of VkBufferUsageFlagBits specifying allowed usages
    /// of the buffer.
    pub fn usage<'s>(&'s mut self, usage: vks::VkBufferUsageFlagBits)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.usage = usage;
        self
    }

    /// sharingMode is a VkSharingMode value specifying the sharing mode of
    /// the buffer when it will be accessed by multiple queue families.
    pub fn sharing_mode<'s>(&'s mut self, sharing_mode: vks::VkSharingMode)
            -> &'s mut BufferBuilder<'b> {
        self.create_info.sharingMode = sharing_mode;
        self
    }

    /// queueFamilyIndexCount is the number of entries in the
    /// pQueueFamilyIndices array.
    /// pQueueFamilyIndices is a list of queue families that will access this
    /// buffer (ignored if sharingMode is not VK_SHARING_MODE_CONCURRENT).
    pub fn queue_family_indices<'s, 'p>(&'s mut self, queue_family_indices: &'p [u32])
            -> &'s mut BufferBuilder<'b>
            where 'p: 'b {
        self.create_info.queueFamilyIndexCount = queue_family_indices.len() as u32;
        self.create_info.pQueueFamilyIndices = queue_family_indices.as_ptr();
        self
    }

    /// Creates and returns a new `Buffer`
    pub fn build(&self, device: Device) -> VooResult<Buffer> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateBuffer(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        // Memory Requirements:
        let mut memory_requirements: vks::VkMemoryRequirements;
        unsafe {
            memory_requirements = mem::uninitialized();
            device.proc_addr_loader().core.vkGetBufferMemoryRequirements(device.handle(),
                handle, &mut memory_requirements);
        }

        Ok(Buffer {
            inner: Arc::new(Inner {
                handle,
                device,
                memory_requirements,
            })
        })
    }
}
