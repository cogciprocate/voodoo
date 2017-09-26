
use std::sync::Arc;
use std::ptr;
use std::mem;
use vks;
use ::{util, VooResult, Device, DeviceMemory, PRINT};

#[derive(Debug)]
struct Inner {
    handle: vks::VkBuffer,
    device_memory: DeviceMemory,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    inner: Arc<Inner>,
}

impl Buffer {
    pub fn new(device: Device, bytes: u64, usage: vks::VkBufferUsageFlags,
            sharing_mode: vks::VkSharingMode, memory_properties: vks::VkMemoryPropertyFlags)
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

        // Memory Requirements:
        let mut mem_requirements: vks::VkMemoryRequirements;
        unsafe {
            mem_requirements = mem::uninitialized();
            device.proc_addr_loader().core.vkGetBufferMemoryRequirements(device.handle(), handle,
                &mut mem_requirements);
        }

        // It should be noted that in a real world application, you're not
        // supposed to actually call vkAllocateMemory for every individual
        // buffer. The maximum number of simultaneous memory allocations is
        // limited by the maxMemoryAllocationCount physical device limit,
        // which may be as low as 4096 even on high end hardware like an
        // NVIDIA GTX 1080. The right way to allocate memory for a large
        // number of objects at the same time is to create a custom allocator
        // that splits up a single allocation among many different objects by
        // using the offset parameters that we've seen in many functions.
        //
        // You can either implement such an allocator yourself, or use the
        // VulkanMemoryAllocator library provided by the GPUOpen initiative. However,
        // for this tutorial it's okay to use a separate allocation for every
        // resource, because we won't come close to hitting any of these limits for
        // now.
        //
        // The previous chapter already mentioned that you should allocate
        // multiple resources like buffers from a single memory allocation,
        // but in fact you should go a step further. Driver developers
        // recommend that you also store multiple buffers, like the vertex and
        // index buffer, into a single VkBuffer and use offsets in commands
        // like vkCmdBindVertexBuffers. The advantage is that your data is
        // more cache friendly in that case, because it's closer together. It
        // is even possible to reuse the same chunk of memory for multiple
        // resources if they are not used during the same render operations,
        // provided that their data is refreshed, of course. This is known as
        // aliasing and some Vulkan functions have explicit flags to specify
        // that you want to do this.

        // * Use a memory heap that is host coherent, indicated with
        //   VK_MEMORY_PROPERTY_HOST_COHERENT_BIT (or)
        // * Call vkFlushMappedMemoryRanges to after writing to the mapped
        //   memory, and call vkInvalidateMappedMemoryRanges before reading from
        //   the mapped memory
        let memory_type_index = ::find_memory_type(&device, mem_requirements.memoryTypeBits,
            memory_properties);

        let alloc_info = vks::VkMemoryAllocateInfo {
            sType: vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: ptr::null(),
            allocationSize: mem_requirements.size,
            memoryTypeIndex: memory_type_index,
        };

        if PRINT { println!("Buffer: {:?}", mem_requirements); }

        let device_memory = DeviceMemory::new(device.clone(), mem_requirements.size,
            memory_type_index)?;

        unsafe {
            ::check(device.proc_addr_loader().core.vkBindBufferMemory(device.handle(), handle,
                device_memory.handle(), 0));
        }

        Ok(Buffer {
            inner: Arc::new(Inner {
                handle,
                device,
                device_memory,
            })
        })
    }

    pub fn handle(&self) -> vks::VkBuffer {
        self.inner.handle
    }

    pub fn device_memory(&self) -> &DeviceMemory {
        &self.inner.device_memory
    }

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