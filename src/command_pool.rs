
use std::sync::Arc;
use std::ptr;
use std::marker::PhantomData;
use vks;
use ::{util, VooResult, Device, Surface};


#[derive(Debug)]
struct Inner {
    handle: vks::VkCommandPool,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct CommandPool {
    inner: Arc<Inner>,
}

impl CommandPool {
    /// Returns a new `CommandPoolBuilder`.
    pub fn builder<'b>() -> CommandPoolBuilder<'b> {
        CommandPoolBuilder::new()
    }

    // pub fn new(device: Device, surface: &Surface, queue_family_flags: vks::VkQueueFlags)
    //     -> VooResult<CommandPool>
    // {
    //     let queue_family_idx = ::queue_families(device.instance(), surface,
    //         device.physical_device(), queue_family_flags).family_idxs()[0];

    //     let create_info = vks::VkCommandPoolCreateInfo {
    //         sType: vks::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    //         pNext: ptr::null(),
    //         // vks::VK_COMMAND_POOL_CREATE_TRANSIENT_BIT
    //         // vks::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT
    //         flags: 0,
    //         queueFamilyIndex: queue_family_idx as u32,
    //     };

    //     let mut handle = 0;
    //     unsafe {
    //         ::check(device.proc_addr_loader().core.vkCreateCommandPool(device.handle(), &create_info,
    //             ptr::null(), &mut handle));
    //     }

    //     Ok(CommandPool {
    //         inner: Arc::new(Inner {
    //             handle,
    //             device,
    //         })
    //     })
    // }

    pub fn handle(&self) -> vks::VkCommandPool {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyCommandPool(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `CommandPool`.
//
// typedef struct VkCommandPoolCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkCommandPoolCreateFlags    flags;
//     uint32_t                    queueFamilyIndex;
// } VkCommandPoolCreateInfo;
//
#[derive(Debug, Clone)]
pub struct CommandPoolBuilder<'b> {
    create_info: vks::VkCommandPoolCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> CommandPoolBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> CommandPoolBuilder<'b> {
        CommandPoolBuilder {
            create_info: vks::VkCommandPoolCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies the usage behavior for the pool and command buffers
    /// allocated from it.
    pub fn flags<'s>(&'s mut self, flags: vks::VkCommandPoolCreateFlags)
            -> &'s mut CommandPoolBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// Specifies a queue family.
    ///
    /// All command buffers allocated from this command pool must be submitted
    /// on queues from the same queue family.
    pub fn queue_family_index<'s>(&'s mut self, queue_family_index: u32)
            -> &'s mut CommandPoolBuilder<'b> {
        self.create_info.queueFamilyIndex = queue_family_index;
        self
    }

    /// Creates and returns a new `CommandPool`
    pub fn build(&self, device: Device) -> VooResult<CommandPool> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateCommandPool(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(CommandPool {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}





