
use std::sync::Arc;
use smallvec::SmallVec;
use vks;
use ::{VdResult, Device, Handle, CommandPool, CommandBufferUsageFlags, CommandBufferBeginInfo,
    DeviceSize, PipelineStageFlags, DependencyFlags, MemoryBarrier, BufferMemoryBarrier,
    ImageMemoryBarrier, BufferImageCopy, ImageLayout, BufferCopy, CommandBufferResetFlags,
    PipelineBindPoint, Viewport, Rect2d, StencilFaceFlags, DebugMarkerMarkerInfoExt,
    DescriptorSetHandle, QueryResultFlags, ShaderStageFlags, RenderPassBeginInfo, SubpassContents,
    ImageCopy, IndexType, ImageBlit, Filter, ClearColorValue, ImageSubresourceRange,
    ClearDepthStencilValue, ClearAttachment, ImageResolve, QueryControlFlags, ClearRect,
    BufferHandle, EventHandle,Buffer, Image, Event, QueryPool, PipelineLayout, DescriptorSet,
    PipelineHandle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CommandBufferHandle(pub(crate) vks::VkCommandBuffer);

impl CommandBufferHandle {
    pub fn to_raw(&self) -> vks::VkCommandBuffer {
        self.0
    }
}

unsafe impl Handle for CommandBufferHandle {
    type Target = CommandBufferHandle;

    #[inline]
    fn handle(&self) -> Self::Target {
        *self
    }
}

#[derive(Debug)]
struct Inner {
    handle: CommandBufferHandle,
    command_pool: CommandPool,
}

#[derive(Debug, Clone)]
pub struct CommandBuffer {
    inner: Arc<Inner>,
}

impl CommandBuffer {
    // FIXME: MAKE pub(crate)
    pub(crate) fn from_parts(command_pool: CommandPool, handle: CommandBufferHandle)
            -> VdResult<CommandBuffer> {
        Ok(CommandBuffer {
            inner: Arc::new(Inner {
                command_pool,
                handle,
            })
        })
    }

    /// Returns this object's handle.
    #[inline]
    pub fn handle(&self) -> CommandBufferHandle {
        self.inner.handle
    }

    /// Returns a reference to this object's associated device.
    #[inline]
    pub fn device(&self) -> &Device {
        self.inner.command_pool.device()
    }

    /// Starts recording this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkBeginCommandBuffer.html
    //
    #[inline]
    pub fn begin(&self, flags: CommandBufferUsageFlags) -> VdResult<()> {
        let begin_info = CommandBufferBeginInfo::builder()
            .flags(flags)
            .build();

        unsafe {
            self.inner.command_pool.device().begin_command_buffer(self.inner.handle, &begin_info)
        }
    }

    /// Finishes recording this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkEndCommandBuffer.html
    //
    #[inline]
    pub fn end(&self) -> VdResult<()> {
        unsafe {
            self.inner.command_pool.device().end_command_buffer(self.inner.handle)
        }
    }

    /// Resets this command buffer to the initial state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkResetCommandBuffer.html
    //
    #[inline]
    pub fn reset(&self, flags: CommandBufferResetFlags) -> VdResult<()> {
        unsafe { self.device().cmd_reset_command_buffer(self.handle(), flags) }
    }

    /// Binds a pipeline object to this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBindPipeline.html
    //
    #[inline]
    pub fn bind_pipeline<P>(&self, pipeline_bind_point: PipelineBindPoint,
            pipeline: &P) where P: Handle<Target=PipelineHandle> {
        unsafe { self.device().cmd_bind_pipeline(self.handle(), pipeline_bind_point,
            pipeline.handle()); }
    }

    /// Sets the viewport on this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetViewport.html
    //
    #[inline]
    pub fn set_viewport(&self, first_viewport: u32, viewports: &[Viewport]) {
        unsafe { self.device().cmd_set_viewport(self.handle(), first_viewport, viewports); }
    }

    /// Sets the dynamic scissor rectangles on this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetScissor.html
    //
    #[inline]
    pub fn set_scissor(&self, first_scissor: u32, scissors: &[Rect2d]) {
        unsafe { self.device().cmd_set_scissor(self.handle(), first_scissor, scissors); }
    }

    /// Sets the dynamic line width state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetLineWidth.html
    //
    #[inline]
    pub fn set_line_width(&self, line_width: f32) {
        unsafe { self.device().cmd_set_line_width(self.handle(), line_width); }
    }

    /// Sets the depth bias dynamic state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetDepthBias.html
    //
    #[inline]
    pub fn set_depth_bias(&self, depth_bias_constant_factor: f32, depth_bias_clamp: f32,
            depth_bias_slope_factor: f32) {
        unsafe { self.device().cmd_set_depth_bias(self.handle(),
            depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor); }
    }

    /// Sets the values of blend constants.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetBlendConstants.html
    //
    #[inline]
    pub fn set_blend_constants(&self, blend_constants: [f32; 4]) {
        unsafe { self.device().cmd_set_blend_constants(self.handle(), blend_constants); }
    }

    /// Sets the depth bounds test values for this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetDepthBounds.html
    //
    pub fn set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
        unsafe { self.device().cmd_set_depth_bounds(self.handle(), min_depth_bounds, max_depth_bounds); }
    }

    /// Sets the stencil compare mask dynamic state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetStencilCompareMask.html
    //
    #[inline]
    pub fn set_stencil_compare_mask(&self, face_mask: StencilFaceFlags, compare_mask: u32) {
        unsafe { self.device().cmd_set_stencil_compare_mask(self.handle(), face_mask, compare_mask); }
    }

    /// Sets the stencil write mask dynamic state
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetStencilWriteMask.html
    //
    #[inline]
    pub fn set_stencil_write_mask(&self, face_mask: StencilFaceFlags, write_mask: u32) {
        unsafe { self.device().cmd_set_stencil_write_mask(self.handle(), face_mask, write_mask); }
    }

    /// Sets the stencil reference dynamic state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetStencilReference.html
    //
    #[inline]
    pub fn set_stencil_reference(&self, face_mask: StencilFaceFlags, reference: u32) {
        unsafe { self.device().cmd_set_stencil_reference(self.handle(), face_mask, reference); }
    }

    /// Binds descriptor sets to this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBindDescriptorSets.html
    //
    #[inline]
    pub fn bind_descriptor_sets(&self, pipeline_bind_point: PipelineBindPoint,
        layout: &PipelineLayout, first_set: u32, descriptor_sets: &[&DescriptorSet],
            dynamic_offsets: &[u32]) {
        let ds_handles: SmallVec<[DescriptorSetHandle; 16]> = descriptor_sets.iter()
            .map(|ds| ds.handle()).collect();
        unsafe {
            self.device().cmd_bind_descriptor_sets(self.handle(), pipeline_bind_point,
                layout.handle(), first_set, &ds_handles, dynamic_offsets);
        }
    }

    /// Binds an index buffer to this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBindIndexBuffer.html
    //
    #[inline]
    pub fn bind_index_buffer(&self, buffer: &Buffer, offset: u64, index_type: IndexType) {
        unsafe { self.device().cmd_bind_index_buffer(self.handle(), buffer.handle(),
            offset, index_type); }
    }

    /// Binds vertex buffers to this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBindVertexBuffers.html
    //
    #[inline]
    pub fn bind_vertex_buffers(&self, first_binding: u32, buffers: &[&Buffer], offsets: &[u64]) {
        let buffer_handles: SmallVec<[BufferHandle; 16]> = buffers.iter()
            .map(|b| b.handle()).collect();
        unsafe { self.device().cmd_bind_vertex_buffers(self.handle(),
            first_binding, &buffer_handles, offsets); }
    }

    /// Draws primitives.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdDraw.html
    //
    #[inline]
    pub fn draw(&self, vertex_count: u32, instance_count: u32, first_vertex: u32,
            first_instance: u32) {
        unsafe { self.device().cmd_draw(self.handle(), vertex_count, instance_count,
            first_vertex, first_instance); }
    }

    /// Issues an indexed draw into this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdDrawIndexed.html
    //
    #[inline]
    pub fn draw_indexed(&self, index_count: u32, instance_count: u32, first_index: u32,
            vertex_offset: i32, first_instance: u32) {
        unsafe { self.device().cmd_draw_indexed(self.handle(), index_count,
            instance_count, first_index, vertex_offset, first_instance); }
    }

    /// Issues an indirect draw into this command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdDrawIndirect.html
    //
    #[inline]
    pub unsafe fn draw_indirect(&self, buffer: &Buffer, offset: u64, draw_count: u32,
            stride: u32) {
        self.device().cmd_draw_indirect(self.handle(),
            buffer.handle(), offset, draw_count, stride);
    }

    /// Performs an indexed indirect draw.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdDrawIndexedIndirect.html
    //
    #[inline]
    pub unsafe fn draw_indexed_indirect(&self, buffer: &Buffer, offset: u64, draw_count: u32,
            stride: u32) {
        self.device().cmd_draw_indexed_indirect(self.handle(),
            buffer.handle(), offset, draw_count, stride);
    }

    /// Dispatches compute work items.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdDispatch.html
    //
    pub fn dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            self.device().cmd_dispatch(self.handle(), group_count_x, group_count_y, group_count_z);
        }
    }

    /// Dispatches compute work items using indirect parameters.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdDispatchIndirect.html
    //
    #[inline]
    pub unsafe fn dispatch_indirect(&self, buffer: &Buffer, offset: u64) {
            self.device().cmd_dispatch_indirect(self.handle(), buffer.handle(), offset);
    }

    /// Copies data between buffer regions.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdCopyBuffer.html
    //
    #[inline]
    pub unsafe fn copy_buffer(&self, src_buffer: &Buffer, dst_buffer: &Buffer,
            regions: &[BufferCopy]) {
        self.device().cmd_copy_buffer(self.handle(), src_buffer.handle(),
            dst_buffer.handle(), regions);
    }

    /// Copies data between images.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdCopyImage.html
    //
    #[inline]
    pub unsafe fn copy_image(&self, src_image: &Image, src_image_layout: ImageLayout,
            dst_image: &Image, dst_image_layout: ImageLayout, regions: &[ImageCopy]) {
        self.device().cmd_copy_image(self.handle(),
            src_image.handle(), src_image_layout, dst_image.handle(), dst_image_layout, regions);
    }

    /// Copies regions of an image, potentially performing format conversion.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBlitImage.html
    //
    #[inline]
    pub unsafe fn blit_image(&self, src_image: &Image, src_image_layout: ImageLayout,
            dst_image: &Image, dst_image_layout: ImageLayout, regions: &[ImageBlit],
            filter: Filter) {
        self.device().cmd_blit_image(self.handle(), src_image.handle(), src_image_layout,
            dst_image.handle(), dst_image_layout, regions, filter);
    }

    /// Copies data from a buffer into an image.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdCopyBufferToImage.html
    //
    #[inline]
    pub unsafe fn copy_buffer_to_image(&self, src_buffer: &Buffer, dst_image: &Image,
            dst_image_layout: ImageLayout, regions: &[BufferImageCopy]) {
        self.device().cmd_copy_buffer_to_image(self.handle(), src_buffer.handle(),
            dst_image.handle(), dst_image_layout, regions, );
    }

    /// Copies image data into a buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdCopyImageToBuffer.html
    //
    #[inline]
    pub unsafe fn copy_image_to_buffer(&self, src_image: &Image, src_image_layout: ImageLayout,
            dst_buffer: &Buffer, regions: &[BufferImageCopy]) {
        self.device().cmd_copy_image_to_buffer(self.handle(),
            src_image.handle(), src_image_layout, dst_buffer.handle(), regions);
    }

    /// Updates a buffer's contents from host memory.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdUpdateBuffer.html
    //
    #[inline]
    pub unsafe fn update_buffer(&self, dst_buffer: &Buffer, dst_offset: u64, data: &[u8]) {
        self.device().cmd_update_buffer(self.handle(),
            dst_buffer.handle(), dst_offset, data);
    }

    /// Fills a region of a buffer with a fixed value.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdFillBuffer.html
    //
    #[inline]
    pub unsafe fn fill_buffer(&self, dst_buffer: &Buffer, dst_offset: u64,
            size: Option<DeviceSize>, data: u32) {
        self.device().cmd_fill_buffer(self.handle(),
            dst_buffer.handle(), dst_offset, size, data);
    }

    /// Clears regions of a color image.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdClearColorImage.html
    //
    #[inline]
    pub unsafe fn clear_color_image(&self, image: &Image, image_layout: ImageLayout,
            color: &ClearColorValue, ranges: &[ImageSubresourceRange]) {
        self.device().cmd_clear_color_image(self.handle(),
            image.handle(), image_layout, color, ranges);
    }

    /// Fills regions of a combined depth/stencil image.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdClearDepthStencilImage.html
    //
    #[inline]
    pub unsafe fn clear_depth_stencil_image(&self, image: &Image, image_layout: ImageLayout,
            depth_stencil: &ClearDepthStencilValue, ranges: &[ImageSubresourceRange]) {
        self.device().cmd_clear_depth_stencil_image(self.handle(),
            image.handle(), image_layout, depth_stencil, ranges);
    }

    /// Clears regions within currently bound framebuffer attachments.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdClearAttachments.html
    //
    #[inline]
    pub fn clear_attachments(&self, attachments: &[ClearAttachment], rects: &[ClearRect]) {
        unsafe { self.device().cmd_clear_attachments(self.handle(), attachments, rects); }
    }

    /// Resolves regions of an image.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdResolveImage.html
    //
    #[inline]
    pub unsafe fn resolve_image(&self, src_image: &Image, src_image_layout: ImageLayout, dst_image: &Image,
            dst_image_layout: ImageLayout, regions: &[ImageResolve]) {
        self.device().cmd_resolve_image(self.handle(), src_image.handle(), src_image_layout,
            dst_image.handle(), dst_image_layout, regions);
    }

    /// Sets an event object to signaled state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdSetEvent.html
    //
    #[inline]
    pub fn set_event(&self, event: &Event, stage_mask: PipelineStageFlags) {
        unsafe { self.device().cmd_set_event(self.handle(),
            event.handle(), stage_mask); }
    }

    /// Resets an event object to non-signaled state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdResetEvent.html
    //
    #[inline]
    pub fn reset_event(&self, event: &Event, stage_mask: PipelineStageFlags) {
        unsafe { self.device().cmd_reset_event(self.handle(), event.handle(), stage_mask); }
    }

    /// Waits for one or more events and insert a set of memory.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdWaitEvents.html
    //
    #[inline]
    pub fn wait_events(&self, events: &[&Event],
            src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags,
            memory_barriers: &[MemoryBarrier],
            buffer_memory_barriers: &[BufferMemoryBarrier],
            image_memory_barriers: &[ImageMemoryBarrier]) {
        let event_handles: SmallVec<[EventHandle; 16]> = events.iter()
            .map(|e| e.handle()).collect();
        unsafe { self.device().cmd_wait_events(self.handle(), &event_handles, src_stage_mask,
            dst_stage_mask, memory_barriers, buffer_memory_barriers, image_memory_barriers); }
    }

    /// Inserts a memory dependency.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdPipelineBarrier.html
    //
    #[inline]
    pub fn pipeline_barrier(&self, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags,
            dependency_flags: DependencyFlags, memory_barriers: &[MemoryBarrier],
            buffer_memory_barriers: &[BufferMemoryBarrier],
            image_memory_barriers: &[ImageMemoryBarrier]) {
        unsafe {
            self.device().cmd_pipeline_barrier(self.handle(), src_stage_mask,
            dst_stage_mask, dependency_flags, memory_barriers, buffer_memory_barriers,
            image_memory_barriers);
        }
    }

    /// Begins a query.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBeginQuery.html
    //
    #[inline]
    pub fn begin_query(&self, query_pool: &QueryPool, query: u32, flags: QueryControlFlags) {
        unsafe { self.device().cmd_begin_query(self.handle(), query_pool.handle(), query, flags); }
    }

    /// Ends a query.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdEndQuery.html
    //
    #[inline]
    pub fn end_query(&self, query_pool: &QueryPool, query: u32) {
        unsafe { self.device().cmd_end_query(self.handle(), query_pool.handle(), query); }
    }

    /// Resets queries in a query pool.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdResetQueryPool.html
    //
    #[inline]
    pub fn reset_query_pool(&self, query_pool: &QueryPool, first_query: u32, query_count: u32) {
        unsafe { self.device().cmd_reset_query_pool(self.handle(),
            query_pool.handle(), first_query, query_count); }
    }

    /// Writes a device timestamp into a query object.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdWriteTimestamp.html
    //
    #[inline]
    pub fn write_timestamp(&self, pipeline_stage: PipelineStageFlags, query_pool: &QueryPool, query: u32) {
        unsafe { self.device().cmd_write_timestamp(self.handle(),
            pipeline_stage, query_pool.handle(), query); }
    }

    /// Copies the results of queries in a query pool to a buffer object.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdCopyQueryPoolResults.html
    //
    #[inline]
    pub unsafe fn copy_query_pool_results(&self, query_pool: &QueryPool, first_query: u32, query_count: u32,
            dst_buffer: &Buffer, dst_offset: u64, stride: u64, flags: QueryResultFlags) {
        self.device().cmd_copy_query_pool_results(self.handle(), query_pool.handle(),
            first_query, query_count, dst_buffer.handle(), dst_offset, stride, flags);
    }

    /// Updates the values of push constants.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdPushConstants.html
    //
    #[inline]
    pub fn push_constants(&self, layout: &PipelineLayout, stage_flags: ShaderStageFlags, offset: u32,
            values: &[u8]) {
        unsafe { self.device().cmd_push_constants(self.handle(), layout.handle(),
            stage_flags, offset, values); }
    }

    /// Begins a new render pass.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdBeginRenderPass.html
    //
    #[inline]
    pub fn begin_render_pass(&self, render_pass_begin: &RenderPassBeginInfo, contents: SubpassContents) {
        unsafe { self.device().cmd_begin_render_pass(self.handle(),
            render_pass_begin, contents); }
    }

    /// Transitions to the next subpass of a render pass.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdNextSubpass.html
    //
    #[inline]
    pub fn next_subpass(&self, contents: SubpassContents) {
        unsafe { self.device().cmd_next_subpass(self.handle(), contents); }
    }

    /// Ends the current render pass.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdEndRenderPass.html
    //
    #[inline]
    pub fn end_render_pass(&self) {
        unsafe { self.device().cmd_end_render_pass(self.handle()); }
    }

    /// Executes a secondary command buffer from a primary command buffer.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkCmdExecuteCommands.html
    //
    #[inline]
    pub fn execute_commands(&self, command_buffers: &[&CommandBuffer]) {
        let command_buffer_handles: SmallVec<[CommandBufferHandle; 16]> = command_buffers.iter()
            .map(|cb| cb.handle()).collect();
        unsafe { self.device().cmd_execute_commands(self.handle(), &command_buffer_handles); }
    }

    /// Begins a debug marker.
    #[inline]
    pub fn debug_marker_begin_ext(&self, marker_info: &DebugMarkerMarkerInfoExt) {
        unsafe { self.device().cmd_debug_marker_begin_ext(self.handle(), marker_info); }
    }

    /// Ends a debug marker.
    #[inline]
    pub fn debug_marker_end_ext(&self) {
        unsafe { self.device().cmd_debug_marker_end_ext(self.handle()); }
    }

    /// Inserts a debug marker.
    #[inline]
    pub fn debug_marker_insert_ext(&self, marker_info: &DebugMarkerMarkerInfoExt) {
        unsafe { self.device().cmd_debug_marker_insert_ext(self.handle(), marker_info); }
    }

}

unsafe impl<'h> Handle for &'h CommandBuffer {
    type Target = CommandBufferHandle;

    #[inline]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.command_pool.device().free_command_buffers(self.command_pool.handle(),
                &[self.handle]);
        }
    }
}