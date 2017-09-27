use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device, Framebuffer, CommandPool, RenderPass, GraphicsPipeline, Buffer,
    PipelineLayout};



// #[derive(Debug, Clone)]
// pub struct CommandBuffer {
//     handle: vks::VkCommandBuffer,
//     device: Device,
// }

// impl CommandBuffer {
//     pub fn new() -> VooResult<CommandBuffer> {

//         let mut handle = 0;
//         unsafe {
//             ::check(device.proc_addr_loader().CreateCommandBuffer(device.handle(), &create_info,
//                 ptr::null(), &mut handle));
//         }

//         Ok(CommandBuffer {
//             handle,
//             device,
//         })
//     }

//     pub fn handle(&self) -> vks::VkCommandBuffer {
//         self.inner.handle
//     }
        /// Returns a reference to the associated device.
//     pub fn device(&self) -> &Device {
//         &self.inner.device
//     }
// }


pub fn create_command_buffers(device: &Device, command_pool: &CommandPool,
        render_pass: &RenderPass, graphics_pipeline: &GraphicsPipeline,
        swapchain_framebuffers: &[Framebuffer], swapchain_extent: &vks::VkExtent2D,
        vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: u32,
        index_count: u32, pipeline_layout: &PipelineLayout,
        descriptor_set: vks::VkDescriptorSet)
        -> VooResult<Vec<vks::VkCommandBuffer>>
{
    let mut command_buffers = Vec::with_capacity(swapchain_framebuffers.len());
    unsafe { command_buffers.set_len(swapchain_framebuffers.len()); }

    let alloc_info = vks::VkCommandBufferAllocateInfo {
        sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: ptr::null(),
        commandPool: command_pool.handle(),
        // * COMMAND_BUFFER_LEVEL_PRIMARY: Can be submitted to a queue for
        //   execution, but cannot be called from other command buffers.
        // * COMMAND_BUFFER_LEVEL_SECONDARY: Cannot be submitted directly, but
        //   can be called from primary command buffers.
        level: vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: command_buffers.len() as u32,
    };

    unsafe {
        ::check(device.proc_addr_loader().vkAllocateCommandBuffers(device.handle(), &alloc_info,
            command_buffers.as_mut_ptr()));
    }

    for (&command_buffer, swapchain_framebuffer) in command_buffers.iter()
            .zip(swapchain_framebuffers.iter())
    {
        let begin_info = vks::VkCommandBufferBeginInfo {
            sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            // * COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: The command buffer
            //   will be rerecorded right after executing it once.
            // * COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: This is a
            //   secondary command buffer that will be entirely within a
            //   single render pass.
            // * COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: The command buffer
            //   can be resubmitted while it is also already pending
            //   execution.
            flags: vks::VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
            pInheritanceInfo: ptr::null(),
        };

        unsafe {
            ::check(device.proc_addr_loader().core.vkBeginCommandBuffer(command_buffer, &begin_info));
        }

        // let clear_color = vks::VkClearValue {
        //     color: vks::VkClearColorValue { float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32] }
        // };

        let clear_values = [
            vks::VkClearValue { color: vks::VkClearColorValue { float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32] } },
            vks::VkClearValue { depthStencil: vks::VkClearDepthStencilValue { depth: 1.0, stencil: 0, } },
        ];

        let render_pass_info = vks::VkRenderPassBeginInfo {
            sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: ptr::null(),
            renderPass: render_pass.handle(),
            framebuffer:swapchain_framebuffer.handle(),
            renderArea: vks::VkRect2D {
                offset: vks::VkOffset2D { x: 0, y: 0, },
                extent: swapchain_extent.clone(),
            },
            clearValueCount: clear_values.len() as u32,
            pClearValues: clear_values.as_ptr(),
        };

        unsafe {
            device.proc_addr_loader().core.vkCmdBeginRenderPass(command_buffer, &render_pass_info,
                vks::VK_SUBPASS_CONTENTS_INLINE);
            device.proc_addr_loader().core.vkCmdBindPipeline(command_buffer, vks::VK_PIPELINE_BIND_POINT_GRAPHICS,
                graphics_pipeline.handle());

            let vertex_buffers = [vertex_buffer.handle()];
            let offsets = [0];
            device.proc_addr_loader().core.vkCmdBindVertexBuffers(command_buffer, 0, 1, vertex_buffers.as_ptr(),
                offsets.as_ptr());
            // device.proc_addr_loader().core.vkCmdBindIndexBuffer(command_buffer, index_buffer.handle(), 0,
            //     vks::VK_INDEX_TYPE_UINT16);
            device.proc_addr_loader().core.vkCmdBindIndexBuffer(command_buffer, index_buffer.handle(), 0,
                vks::VK_INDEX_TYPE_UINT32);

            device.proc_addr_loader().core.vkCmdBindDescriptorSets(command_buffer,
                vks::VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline_layout.handle(), 0, 1,
                &descriptor_set, 0, ptr::null());

            // // * vertexCount: Even though we don't have a vertex buffer, we
            // //   technically still have 3 vertices to draw.
            // // * instanceCount: Used for instanced rendering, use 1 if you're
            // //   not doing that.
            // // * firstVertex: Used as an offset into the vertex buffer,
            // //   defines the lowest value of gl_VertexIndex.
            // // * firstInstance: Used as an offset for instanced rendering,
            // //   defines the lowest value of gl_InstanceIndex.
            // device.proc_addr_loader().core.vkCmdDraw(command_buffer, vertex_count, 1, 0, 0);
            device.proc_addr_loader().core.vkCmdDrawIndexed(command_buffer, index_count, 1, 0, 0, 0);

            device.proc_addr_loader().core.vkCmdEndRenderPass(command_buffer);
            device.proc_addr_loader().core.vkEndCommandBuffer(command_buffer);
        }
    }
    Ok(command_buffers)
}






// #[derive(Debug)]
// struct Inner {
//     handle: vks::VkCommandBuffer,
//     device: Device,
// }

// #[derive(Debug, Clone)]
// pub struct CommandBuffer {
//     inner: Arc<Inner>,
// }

// impl CommandBuffer {
//     pub fn new() -> VooResult<CommandBuffer> {

//         let mut handle = 0;
//         unsafe {
//             ::check(device.proc_addr_loader().CreateCommandBuffer(device.handle(), &create_info,
//                 ptr::null(), &mut handle));
//         }

//         Ok(CommandBuffer {
//             inner: Arc::new(Inner {
//                 handle,
//                 device,
//             })
//         })
//     }

//     pub fn handle(&self) -> vks::VkCommandBuffer {
//         self.inner.handle
//     }
// /// Returns a reference to the associated device.
//     pub fn device(&self) -> &Device {
//         &self.inner.device
//     }
// }

// impl Drop for Inner {
//     fn drop(&mut self) {
//         unsafe {
//             self.device.proc_addr_loader().DestroyCommandBuffer(self.device.handle(), self.handle, ptr::null());
//         }
//     }
// }