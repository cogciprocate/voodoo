
use std::sync::Arc;
use std::ptr;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{util, VooResult, Device, Handle, CommandPoolCreateInfo, CommandPoolCreateFlags,
    CommandBufferAllocateInfo, CommandPool, CommandBufferUsageFlags, CommandBufferBeginInfo};



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CommandBufferHandle(pub vks::VkCommandBuffer);

impl Handle for CommandBufferHandle {
    type Target = CommandBufferHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Debug)]
pub struct CommandBuffer {
    handle: CommandBufferHandle,
    command_pool: CommandPool,
}

impl CommandBuffer {
    // FIXME: MAKE pub(crate)
    pub fn from_parts(command_pool: CommandPool, handle: CommandBufferHandle)
            -> VooResult<CommandBuffer> {
        Ok(CommandBuffer {
            command_pool,
            handle,
        })
    }

    pub fn begin(&self, flags: CommandBufferUsageFlags) {
        let begin_info = CommandBufferBeginInfo::builder()
            .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            self.command_pool.device().proc_addr_loader()
                .vkBeginCommandBuffer(self.handle.0, begin_info.as_raw());
        }
    }

    pub fn handle(&self) -> CommandBufferHandle {
        self.handle
    }
}

impl<'h> Handle for &'h CommandBuffer {
    type Target = CommandBufferHandle;

    fn handle(&self) -> Self::Target {
        self.handle
    }
}
