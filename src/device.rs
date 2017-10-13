use std::sync::Arc;
use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::ffi::CStr;
use libc::{c_void};
use smallvec::SmallVec;
use vks;
use ::{error, VdResult, Instance, PhysicalDevice, DeviceQueueCreateInfo, CharStrs,
    PhysicalDeviceFeatures, PRINT, Handle, SubmitInfo, QueueHandle, MemoryAllocateInfo,
    DeviceMemoryHandle, MemoryMapFlags, SwapchainKhrHandle, SwapchainCreateInfoKhr,
    ShaderModuleCreateInfo, ShaderModuleHandle, SemaphoreCreateInfo, SemaphoreHandle,
    SamplerCreateInfo, SamplerHandle, RenderPassCreateInfo, RenderPassHandle, BufferCreateInfo,
    BufferHandle, BufferViewCreateInfo, BufferViewHandle, ImageViewCreateInfo, ImageViewHandle,
    ImageCreateInfo, ImageHandle, FramebufferCreateInfo, FramebufferHandle,
    DescriptorSetLayoutCreateInfo, DescriptorSetLayoutHandle, DescriptorPoolCreateInfo,
    DescriptorPoolHandle, CommandPoolCreateInfo, CommandPoolHandle, CommandBufferAllocateInfo,
    CommandBufferHandle, PipelineLayoutCreateInfo, PipelineLayoutHandle, FenceCreateInfo,
    FenceHandle, EventCreateInfo, EventHandle, PipelineCacheCreateInfo, PipelineCacheHandle,
    MemoryRequirements, DeviceSize, CommandBufferBeginInfo, GraphicsPipelineCreateInfo,
    PipelineHandle, ComputePipelineCreateInfo, PipelineStageFlags, DependencyFlags, MemoryBarrier,
    BufferMemoryBarrier, ImageMemoryBarrier, WriteDescriptorSet, CopyDescriptorSet,
    BufferImageCopy, ImageLayout, BufferCopy, CommandBufferResetFlags, PipelineBindPoint, Viewport,
    Rect2d, StencilFaceFlags, DebugMarkerMarkerInfoExt, DescriptorSetHandle, QueryPoolHandle,
    QueryResultFlags, ShaderStageFlags, RenderPassBeginInfo, SubpassContents, ImageCopy, IndexType,
    ImageBlit, Filter, ClearColorValue, ImageSubresourceRange, ClearDepthStencilValue,
    ClearAttachment, ImageResolve, QueryControlFlags, ClearRect, PresentInfoKhr, MappedMemoryRange,
    SparseImageMemoryRequirements, BindSparseInfo, CallResult, QueryPoolCreateInfo,
    ImageSubresource, SubresourceLayout, DescriptorSetAllocateInfo, DescriptorPoolResetFlags,
    Extent2d, CommandPoolResetFlags, CommandPoolTrimFlagsKhr, MemoryGetWin32HandleInfoKhr,
    ExternalMemoryHandleTypeFlagsKhr, HANDLE, MemoryGetFdInfoKhr,
    ImportSemaphoreWin32HandleInfoKhr, SemaphoreGetWin32HandleInfoKhr, ImportSemaphoreFdInfoKhr,
    SemaphoreGetFdInfoKhr, PipelineLayout, BufferMemoryRequirementsInfo2Khr,
    ImportFenceWin32HandleInfoKhr, FenceGetWin32HandleInfoKhr, ImportFenceFdInfoKhr,
    FenceGetFdInfoKhr, ImageMemoryRequirementsInfo2Khr, ImageSparseMemoryRequirementsInfo2Khr,
    DebugMarkerObjectTagInfoExt, DebugMarkerObjectNameInfoExt, DisplayPowerInfoExt,
    DisplayKhrHandle, DeviceEventInfoExt, DisplayEventInfoExt, HdrMetadataExt,
    SurfaceCounterFlagsExt,};

// #[cfg(feature = "experimental")]
// use ::{};

#[cfg(feature = "unimplemented")]
use ::{SamplerYcbcrConversionCreateInfoKhr, IndirectCommandsLayoutNvxCreateInfo,
    ObjectTableNvxCreateInfo, ValidationCacheExtCreateInfo, DescriptorUpdateTemplateCreateInfoKhr,
    DescriptorUpdateTemplateKhrHandle, SamplerYcbcrConversionKhrHandle, IndirectCommandsLayoutNvxHandle,
    ValidationCacheExtHandle, ObjectTableNvxHandle, SampleLocationsInfoExt, ValidationCacheExt,};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DeviceHandle(pub(crate) vks::VkDevice);

impl DeviceHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDevice {
        self.0
    }
}

unsafe impl Handle for DeviceHandle {
    type Target = DeviceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: DeviceHandle,
    physical_device: PhysicalDevice,
    // features: vks::VkPhysicalDeviceFeatures,
    // queues: SmallVec<[u32; 32]>,
    queue_family_indices: SmallVec<[u32; 16]>,
    instance: Instance,
    loader: vks::DeviceProcAddrLoader,
}

#[derive(Debug, Clone)]
pub struct Device {
    inner: Arc<Inner>,
}

impl Device {
    /// Returns a new `DeviceBuilder`.
    pub fn builder<'db>() -> DeviceBuilder<'db> {
        DeviceBuilder::new()
    }

    #[inline]
    pub fn queue(&self, queue_idx: u32) -> QueueHandle {
        assert!(self.inner.queue_family_indices.len() == 1,
            "Update this shitty queue family code.");
        self.get_device_queue(self.inner.queue_family_indices[0], queue_idx)
    }

    #[inline]
    pub fn proc_addr_loader(&self) -> &vks::DeviceProcAddrLoader {
        &self.inner.loader
    }

    #[inline]
    pub fn handle(&self) -> DeviceHandle {
        self.inner.handle
    }

    #[inline]
    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.inner.physical_device
    }

    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.inner.instance
    }

    #[inline]
    pub fn wait_idle(&self) {
        self.device_wait_idle()
    }

    /// Returns the memory type index on this device matching the provided
    /// type filter and properties.
    //
    // [HELPER]
    pub fn memory_type_index(&self, type_filter: u32, properties: ::MemoryPropertyFlags)
            -> VdResult<u32> {
        let mem_props = self.physical_device().memory_properties();

        for i in 0..mem_props.memory_type_count() {
            if (type_filter & (1 << i)) != 0 &&
                (mem_props.memory_types()[i as usize].property_flags() & properties) == properties
            {
                return Ok(i);
            }
        }
        panic!("failed to find suitable memory type index with: type_filter: '{}', properties: '{:?}'",
            type_filter, properties);
    }

    // *PFN_vkGetDeviceQueue)(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue);
    pub fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> QueueHandle {
        let mut handle = ptr::null_mut();
        unsafe {
            self.proc_addr_loader().core.vkGetDeviceQueue(self.inner.handle.0,
                queue_family_index, queue_index, &mut handle);
        }
        if !handle.is_null() {
            QueueHandle(handle)
        } else {
            panic!("unable to get device queue with family index: {} and index: {}",
                queue_family_index, queue_index);
        }
    }


    // *PFN_vkQueueSubmit)(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence);
    //
    // queue is the queue that the command buffers will be submitted to.
    //
    // submitCount is the number of elements in the pSubmits array.
    //
    // pSubmits is a pointer to an array of VkSubmitInfo structures, each
    // specifying a command buffer submission batch.
    //
    // fence is an optional handle to a fence to be signaled. If fence is not
    // VK_NULL_HANDLE, it defines a fence signal operation.
    pub unsafe fn queue_submit(&self, queue: QueueHandle, submit_info: &[SubmitInfo],
            fence: Option<FenceHandle>) -> VdResult<()> {
        let fence_handle_raw = fence.map(|f| f.to_raw()).unwrap_or(0);
        let result = self.proc_addr_loader().core.vkQueueSubmit(queue.to_raw(),
            submit_info.len() as u32, submit_info.as_ptr() as *const vks::VkSubmitInfo,
            fence_handle_raw);
        error::check(result, "vkQueueSubmit", ())
    }

    // *PFN_vkQueueWaitIdle)(VkQueue queue);
    pub fn queue_wait_idle<Q>(&self, queue: Q)
            where Q: Handle<Target=QueueHandle> {
        unsafe {
            self.proc_addr_loader().core.vkQueueWaitIdle(queue.handle().to_raw());
        }
    }

    // *PFN_vkDeviceWaitIdle)(VkDevice device);
    pub fn device_wait_idle(&self) {
        unsafe {
            self.proc_addr_loader().core.vkDeviceWaitIdle(self.handle().to_raw());
        }
    }

    // *PFN_vkAllocateMemory)(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const VkAllocationCallbacks* pAllocator, VkDeviceMemory* pMemory);
    pub unsafe fn allocate_memory(&self, allocate_info: &MemoryAllocateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<DeviceMemoryHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkAllocateMemory(self.handle().0,
            allocate_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkAllocateMemory", DeviceMemoryHandle(handle))
    }

    // *PFN_vkFreeMemory)(VkDevice device, VkDeviceMemory memory, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn free_memory(&self, memory: DeviceMemoryHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkFreeMemory(self.handle().0,
            memory.handle().to_raw(), allocator);
    }

    // *PFN_vkMapMemory)(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData);
    pub unsafe fn map_memory<T>(&self, memory: DeviceMemoryHandle, offset_bytes: u64, size_bytes: u64,
            flags: MemoryMapFlags) -> VdResult<*mut T> {
        let mut data = ptr::null_mut();
        let result = self.proc_addr_loader().core.vkMapMemory(self.handle().to_raw(),
            memory.to_raw(), offset_bytes, size_bytes, flags.bits(), &mut data);
        error::check(result, "vkMapMemory", data as *mut T)
    }

    // *PFN_vkUnmapMemory)(VkDevice device, VkDeviceMemory memory);
    pub unsafe fn unmap_memory(&self, memory: DeviceMemoryHandle) {
        self.proc_addr_loader().core.vkUnmapMemory(self.handle().0, memory.to_raw());
    }

    // *PFN_vkFlushMappedMemoryRanges)(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);
    pub unsafe fn flush_mapped_memory_ranges(&self, memory_ranges: &[MappedMemoryRange])
            -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkFlushMappedMemoryRanges(self.handle().to_raw(),
            memory_ranges.len() as u32, memory_ranges.as_ptr() as *const vks::VkMappedMemoryRange);
        error::check(result, "vkFlushMappedMemoryRanges", ())
    }


    // *PFN_vkInvalidateMappedMemoryRanges)(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);
    pub unsafe fn invalidate_mapped_memory_ranges(&self, memory_ranges: &[MappedMemoryRange])
            -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkInvalidateMappedMemoryRanges(self.handle().to_raw(),
            memory_ranges.len() as u32, memory_ranges.as_ptr() as *const vks::VkMappedMemoryRange);
        error::check(result, "vkInvalidateMappedMemoryRanges", ())
    }

    // *PFN_vkGetDeviceMemoryCommitment)(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes);
    pub unsafe fn get_device_memory_commitment<Dm>(&self, memory: Dm)
            -> DeviceSize
            where Dm: Handle<Target=DeviceMemoryHandle> {
        let mut committed_memory_in_bytes = 0;
        self.proc_addr_loader().core.vkGetDeviceMemoryCommitment(self.handle().to_raw(),
            memory.handle().to_raw(), &mut committed_memory_in_bytes);
        committed_memory_in_bytes
    }

    // *PFN_vkBindBufferMemory)(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset);
    pub unsafe fn bind_buffer_memory(&self, buffer: BufferHandle, memory: DeviceMemoryHandle,
            memory_offset: DeviceSize) -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkBindBufferMemory(
            self.handle().to_raw(), buffer.to_raw(), memory.to_raw(), memory_offset);
        error::check(result, "vkBindBufferMemory", ())
    }

    // *PFN_vkBindImageMemory)(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset);
    pub unsafe fn bind_image_memory(&self, image: ImageHandle, memory: DeviceMemoryHandle,
            memory_offset: DeviceSize) -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkBindImageMemory(
            self.handle().to_raw(), image.to_raw(), memory.to_raw(), memory_offset);
        error::check(result, "vkBindImageMemory", ())
    }

    // *PFN_vkGetBufferMemoryRequirements)(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements);
    pub unsafe fn get_buffer_memory_requirements(&self, buffer: BufferHandle) -> MemoryRequirements {
        let mut memory_requirements: vks::VkMemoryRequirements;
        memory_requirements = mem::uninitialized();
        self.proc_addr_loader().core.vkGetBufferMemoryRequirements(self.handle().to_raw(),
            buffer.to_raw(), &mut memory_requirements);
        MemoryRequirements::from_raw(memory_requirements)
    }

    // *PFN_vkGetImageMemoryRequirements)(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements);
    pub unsafe fn get_image_memory_requirements<I>(&self, image: I) -> MemoryRequirements
            where I: Handle<Target=ImageHandle> {
        let mut memory_requirements: vks::VkMemoryRequirements;
        memory_requirements = mem::uninitialized();
        self.proc_addr_loader().core.vkGetImageMemoryRequirements(self.handle().to_raw(),
            image.handle().to_raw(), &mut memory_requirements);
        MemoryRequirements::from_raw(memory_requirements)
    }

    // *PFN_vkGetImageSparseMemoryRequirements)(VkDevice device, VkImage image, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements);
    pub unsafe fn get_image_sparse_memory_requirements<I>(&self, image: I)
            -> SmallVec<[SparseImageMemoryRequirements; 32]>
            where I: Handle<Target=ImageHandle> {
        let mut sparse_memory_requirement_count = 0u32;
        let mut sparse_memory_requirements: SmallVec<[SparseImageMemoryRequirements; 32]> = SmallVec::new();
        self.proc_addr_loader().core.vkGetImageSparseMemoryRequirements(self.handle().to_raw(),
            image.handle().to_raw(), &mut sparse_memory_requirement_count, ptr::null_mut());
        sparse_memory_requirements.reserve_exact(sparse_memory_requirement_count as usize);
        sparse_memory_requirements.set_len(sparse_memory_requirement_count as usize);
        self.proc_addr_loader().core.vkGetImageSparseMemoryRequirements(self.handle().to_raw(),
            image.handle().to_raw(), &mut sparse_memory_requirement_count,
            sparse_memory_requirements.as_mut_ptr() as *mut vks::VkSparseImageMemoryRequirements);
        sparse_memory_requirements
    }

    // *PFN_vkQueueBindSparse)(VkQueue queue, uint32_t bindInfoCount, const VkBindSparseInfo* pBindInfo, VkFence fence);
    pub unsafe fn queue_bind_sparse<Q, F>(&self, queue: Q, bind_info: &[BindSparseInfo], fence: F)
            -> VdResult<()>
            where Q: Handle<Target=QueueHandle>, F: Handle<Target=FenceHandle> {
        let result = self.proc_addr_loader().core.vkQueueBindSparse(queue.handle().to_raw(),
            bind_info.len() as u32, bind_info.as_ptr() as *const _ as *const vks::VkBindSparseInfo,
            fence.handle().to_raw());
        error::check(result, "vkQueueBindSparse", ())
    }

    // *PFN_vkCreateFence)(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);
    pub unsafe fn create_fence(&self, create_info: &FenceCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<FenceHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateFence(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateFence", FenceHandle(handle))
    }

    // *PFN_vkDestroyFence)(VkDevice device, VkFence fence, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_fence(&self, fence: FenceHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyFence(self.handle().to_raw(),
            fence.to_raw(), allocator);
    }

    // *PFN_vkResetFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences);
    pub unsafe fn reset_fences(&self, fences: &[FenceHandle]) -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkResetFences(self.handle().to_raw(),
            fences.len() as u32, fences.as_ptr() as *const vks::VkFence);
        error::check(result, "vkResetFences", ())
    }


    // *PFN_vkGetFenceStatus)(VkDevice device, VkFence fence);
    pub unsafe fn get_fence_status<F>(&self, fence: F) -> VdResult<CallResult>
            where F: Handle<Target=FenceHandle> {
        let result = self.proc_addr_loader().core.vkGetFenceStatus(self.handle().to_raw(), fence.handle().to_raw());
        error::check(result, "vkGetFenceStatus", CallResult::from(result))
    }

    // *PFN_vkWaitForFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout);
    pub unsafe fn wait_for_fences(&self, fences: &[FenceHandle], wait_all: bool, timeout: u64)
            -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkWaitForFences(self.handle().to_raw(),
            fences.len() as u32, fences.as_ptr() as *const vks::VkFence,
            wait_all as vks::VkBool32, timeout);
        error::check(result, "vkWaitForFences", ())
    }

    // *PFN_vkCreateSemaphore)(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSemaphore* pSemaphore);
    pub unsafe fn create_semaphore(&self, create_info: &SemaphoreCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SemaphoreHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateSemaphore(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateSemaphore", SemaphoreHandle(handle))
    }

    // *PFN_vkDestroySemaphore)(VkDevice device, VkSemaphore semaphore, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_semaphore(&self, shader_module: SemaphoreHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroySemaphore(self.handle().to_raw(),
            shader_module.to_raw(), allocator);
    }

    // *PFN_vkCreateEvent)(VkDevice device, const VkEventCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkEvent* pEvent);
    pub unsafe fn create_event(&self, create_info: &EventCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<EventHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateEvent(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateEvent", EventHandle(handle))
    }

    // *PFN_vkDestroyEvent)(VkDevice device, VkEvent event, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_event(&self, event: EventHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyEvent(self.handle().to_raw(),
            event.to_raw(), allocator);
    }

    // *PFN_vkGetEventStatus)(VkDevice device, VkEvent event);
    pub unsafe fn get_event_status<E>(&self, event: E) -> VdResult<CallResult>
            where E: Handle<Target=EventHandle> {
        let result = self.proc_addr_loader().core.vkGetEventStatus(self.handle().to_raw(),
            event.handle().to_raw());
        error::check(result, "vkGetEventStatus", CallResult::from(result))
    }

    // *PFN_vkSetEvent)(VkDevice device, VkEvent event);
    pub unsafe fn set_event<E>(&self, event: E) -> VdResult<()>
            where E: Handle<Target=EventHandle> {
        let result = self.proc_addr_loader().core.vkSetEvent(self.handle().to_raw(),
            event.handle().to_raw());
        error::check(result, "vkSetEvent", ())
    }

    // *PFN_vkResetEvent)(VkDevice device, VkEvent event);
    pub unsafe fn reset_event<E>(&self, event: E) -> VdResult<()>
            where E: Handle<Target=EventHandle> {
        let result = self.proc_addr_loader().core.vkResetEvent(self.handle().to_raw(),
            event.handle().to_raw());
        error::check(result, "vkResetEvent", ())
    }

    // *PFN_vkCreateQueryPool)(VkDevice device, const VkQueryPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkQueryPool* pQueryPool);
    pub unsafe fn create_query_pool(&self, create_info: &QueryPoolCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<QueryPoolHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateQueryPool(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateQueryPool", QueryPoolHandle(handle))
    }

    // *PFN_vkDestroyQueryPool)(VkDevice device, VkQueryPool queryPool, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_query_pool(&self, query_pool: QueryPoolHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyQueryPool(self.handle().to_raw(),
            query_pool.to_raw(), allocator);
    }

    // *PFN_vkGetQueryPoolResults)(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, size_t dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags);
    pub unsafe fn get_query_pool_results<Q>(&self, query_pool: Q, first_query: u32, query_count: u32,
            data_size: usize, data: *mut c_void, stride: DeviceSize, flags: QueryResultFlags)
            -> VdResult<()>
            where Q: Handle<Target=QueryPoolHandle> {
        let result = self.proc_addr_loader().core.vkGetQueryPoolResults(self.handle().to_raw(),
            query_pool.handle().to_raw(), first_query, query_count, data_size, data, stride,
            flags.bits());
        error::check(result, "vkGetQueryPoolResults", ())
    }

    // *PFN_vkCreateBuffer)(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBuffer* pBuffer);
    pub unsafe fn create_buffer(&self, create_info: &BufferCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<BufferHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateBuffer(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateBuffer", BufferHandle(handle))
    }

    // *PFN_vkDestroyBuffer)(VkDevice device, VkBuffer buffer, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_buffer(&self, buffer: BufferHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyBuffer(self.handle().to_raw(),
            buffer.to_raw(), allocator);
    }

    // *PFN_vkCreateBufferView)(VkDevice device, const VkBufferViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBufferView* pView);
    pub unsafe fn create_buffer_view(&self, create_info: &BufferViewCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<BufferViewHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateBufferView(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateBufferView", BufferViewHandle(handle))
    }

    // *PFN_vkDestroyBufferView)(VkDevice device, VkBufferView bufferView, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_buffer_view(&self, buffer_view: BufferViewHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyBufferView(self.handle().to_raw(),
            buffer_view.to_raw(), allocator);
    }

    // *PFN_vkCreateImage)(VkDevice device, const VkImageCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImage* pImage);
    pub unsafe fn create_image(&self, create_info: &ImageCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<ImageHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateImage(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateImage", ImageHandle(handle))
    }

    // *PFN_vkDestroyImage)(VkDevice device, VkImage image, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_image(&self, image: ImageHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyImage(self.handle().to_raw(),
            image.to_raw(), allocator);
    }

    // *PFN_vkGetImageSubresourceLayout)(VkDevice device, VkImage image, const VkImageSubresource* pSubresource, VkSubresourceLayout* pLayout);
    pub unsafe fn get_image_subresource_layout<I>(&self, image: I, subresource: &ImageSubresource)
            -> SubresourceLayout
            where I: Handle<Target=ImageHandle> {
        let mut layout = mem::uninitialized();
        self.proc_addr_loader().core.vkGetImageSubresourceLayout(self.handle().to_raw(),
            image.handle().to_raw(), subresource.as_raw(),
            &mut layout as *mut _ as *mut vks::VkSubresourceLayout);
        layout
    }

    // *PFN_vkCreateImageView)(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView);
    pub unsafe fn create_image_view(&self, create_info: &ImageViewCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<ImageViewHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateImageView(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateImageView", ImageViewHandle(handle))
    }

    // *PFN_vkDestroyImageView)(VkDevice device, VkImageView imageView, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_image_view(&self, image_view: ImageViewHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyImageView(self.handle().to_raw(),
            image_view.to_raw(), allocator);
    }

    // *PFN_vkCreateShaderModule)(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule);
    pub unsafe fn create_shader_module(&self, create_info: &ShaderModuleCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<ShaderModuleHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateShaderModule(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateShaderModule", ShaderModuleHandle(handle))
    }

    // *PFN_vkDestroyShaderModule)(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_shader_module(&self, shader_module: ShaderModuleHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyShaderModule(self.handle().to_raw(),
            shader_module.to_raw(), allocator);
    }

    // *PFN_vkCreatePipelineCache)(VkDevice device, const VkPipelineCacheCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineCache* pPipelineCache);
    pub unsafe fn create_pipeline_cache(&self, create_info: &PipelineCacheCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<PipelineCacheHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreatePipelineCache(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreatePipelineCache", PipelineCacheHandle(handle))
    }

    // *PFN_vkDestroyPipelineCache)(VkDevice device, VkPipelineCache pipelineCache, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_pipeline_cache(&self, pipeline_cache: PipelineCacheHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyPipelineCache(self.handle().to_raw(),
            pipeline_cache.to_raw(), allocator);
    }

    // *PFN_vkGetPipelineCacheData)(VkDevice device, VkPipelineCache pipelineCache, size_t* pDataSize, void* pData);
    pub unsafe fn get_pipeline_cache_data<Pc>(&self, pipeline_cache: Pc, data_size: *mut usize,
            data: *mut c_void) -> VdResult<()>
            where Pc: Handle<Target=PipelineCacheHandle> {
        let result = self.proc_addr_loader().core.vkGetPipelineCacheData(self.handle().to_raw(),
            pipeline_cache.handle().to_raw(), data_size, data);
        error::check(result, "vkGetPipelineCacheData", ())
    }

    // *PFN_vkMergePipelineCaches)(VkDevice device, VkPipelineCache dstCache, uint32_t srcCacheCount, const VkPipelineCache* pSrcCaches);
    pub unsafe fn merge_pipeline_caches<Pc>(&self, dst_cache: Pc, src_caches: &[PipelineCacheHandle])
            -> VdResult<()>
            where Pc: Handle<Target=PipelineCacheHandle> {
        let result = self.proc_addr_loader(). core.vkMergePipelineCaches(self.handle().to_raw(),
            dst_cache.handle().to_raw(), src_caches.len() as u32,
            src_caches.as_ptr() as *const vks::VkPipelineCache);
        error::check(result, "vkMergePipelineCaches", ())
    }

    // *PFN_vkCreateGraphicsPipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);
    pub unsafe fn create_graphics_pipelines(&self, pipeline_cache: Option<PipelineCacheHandle>,
            create_infos: &[GraphicsPipelineCreateInfo],
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SmallVec<[PipelineHandle; 4]>> {
        let allocator = allocator.unwrap_or(ptr::null());
        let pipeline_cache = pipeline_cache.map(|pc| pc.to_raw()).unwrap_or(0);
        let mut pipelines = SmallVec::<[PipelineHandle; 4]>::new();
        pipelines.reserve_exact(create_infos.len());
        pipelines.set_len(create_infos.len());
        let result = self.proc_addr_loader().core.vkCreateGraphicsPipelines(self.handle().to_raw(),
            pipeline_cache, create_infos.len() as u32,
            create_infos.as_ptr() as *const vks::VkGraphicsPipelineCreateInfo,
            allocator,
            pipelines.as_mut_ptr() as *mut vks::VkPipeline);
        error::check(result, "vkCreateGraphicsPipelines", pipelines)
    }

    // *PFN_vkCreateComputePipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkComputePipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);
    pub unsafe fn create_compute_pipelines(&self, pipeline_cache: Option<PipelineCacheHandle>,
            create_infos: &[ComputePipelineCreateInfo],
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SmallVec<[PipelineHandle; 4]>> {
        let allocator = allocator.unwrap_or(ptr::null());
        let pipeline_cache = pipeline_cache.map(|pc| pc.to_raw()).unwrap_or(0);
        let mut pipelines = SmallVec::<[PipelineHandle; 4]>::new();
        pipelines.reserve_exact(create_infos.len());
        pipelines.set_len(create_infos.len());
        let result = self.proc_addr_loader().core.vkCreateComputePipelines(self.handle().to_raw(),
            pipeline_cache, create_infos.len() as u32,
            create_infos.as_ptr() as *const vks::VkComputePipelineCreateInfo,
            allocator,
            pipelines.as_mut_ptr() as *mut vks::VkPipeline);
        error::check(result, "vkCreateComputePipelines", pipelines)
    }

    // *PFN_vkDestroyPipeline)(VkDevice device, VkPipeline pipeline, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_pipeline(&self, pipeline: PipelineHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyPipeline(self.handle().to_raw(),
            pipeline.to_raw(), allocator);
    }

    // *PFN_vkCreatePipelineLayout)(VkDevice device, const VkPipelineLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineLayout* pPipelineLayout);
    pub unsafe fn create_pipeline_layout(&self, create_info: &PipelineLayoutCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<PipelineLayoutHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreatePipelineLayout(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreatePipelineLayout", PipelineLayoutHandle(handle))
    }

    // *PFN_vkDestroyPipelineLayout)(VkDevice device, VkPipelineLayout pipelineLayout, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_pipeline_layout(&self, pipeline_layout: PipelineLayoutHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyPipelineLayout(self.handle().to_raw(),
            pipeline_layout.to_raw(), allocator);
    }

    // *PFN_vkCreateSampler)(VkDevice device, const VkSamplerCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSampler* pSampler);
    pub unsafe fn create_sampler(&self, create_info: &SamplerCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SamplerHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateSampler(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateSampler", SamplerHandle(handle))
    }

    // *PFN_vkDestroySampler)(VkDevice device, VkSampler sampler, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_sampler(&self, sampler: SamplerHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroySampler(self.handle().to_raw(),
            sampler.to_raw(), allocator);
    }

    // *PFN_vkCreateDescriptorSetLayout)(VkDevice device, const VkDescriptorSetLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorSetLayout* pSetLayout);
    pub unsafe fn create_descriptor_set_layout(&self, create_info: &DescriptorSetLayoutCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<DescriptorSetLayoutHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateDescriptorSetLayout(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateDescriptorSetLayout", DescriptorSetLayoutHandle(handle))
    }

    // *PFN_vkDestroyDescriptorSetLayout)(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_descriptor_set_layout(&self, descriptor_set_layout: DescriptorSetLayoutHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyDescriptorSetLayout(self.handle().to_raw(),
            descriptor_set_layout.to_raw(), allocator);
    }

    // *PFN_vkCreateDescriptorPool)(VkDevice device, const VkDescriptorPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorPool* pDescriptorPool);
    pub unsafe fn create_descriptor_pool(&self, create_info: &DescriptorPoolCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<DescriptorPoolHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateDescriptorPool(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateDescriptorPool", DescriptorPoolHandle(handle))
    }

    // *PFN_vkDestroyDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_descriptor_pool(&self, descriptor_pool: DescriptorPoolHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyDescriptorPool(self.handle().to_raw(),
            descriptor_pool.to_raw(), allocator);
    }

    // *PFN_vkResetDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags);
    pub unsafe fn reset_descriptor_pool<Dp>(&self, descriptor_pool: Dp,
            flags: DescriptorPoolResetFlags) -> VdResult<()>
            where Dp: Handle<Target=DescriptorPoolHandle> {
        let result = self.proc_addr_loader().core.vkResetDescriptorPool(self.handle().to_raw(),
            descriptor_pool.handle().to_raw(), flags.bits());
        error::check(result, "vkResetDescriptorPool", ())
    }

    // *PFN_vkAllocateDescriptorSets)(VkDevice device, const VkDescriptorSetAllocateInfo* pAllocateInfo, VkDescriptorSet* pDescriptorSets);
    pub unsafe fn allocate_descriptor_sets(&self, allocate_info: &DescriptorSetAllocateInfo)
            -> VdResult<SmallVec<[DescriptorSetHandle; 8]>> {
        let mut descriptor_sets = SmallVec::<[DescriptorSetHandle; 8]>::new();
        let count = allocate_info.set_layouts().len();
        descriptor_sets.reserve_exact(count);
        descriptor_sets.set_len(count);
        let result = self.proc_addr_loader().core.vkAllocateDescriptorSets(
            self.handle().to_raw(), allocate_info.as_raw(),
            descriptor_sets.as_mut_ptr() as *mut vks::VkDescriptorSet);
        error::check(result, "vkAllocateDescriptorSets", descriptor_sets)
    }

    // *PFN_vkFreeDescriptorSets)(VkDevice device, VkDescriptorPool descriptorPool, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets);
    pub unsafe fn free_descriptor_sets<Dp>(&self, descriptor_pool: Dp,
            descriptor_sets: &[DescriptorSetHandle]) -> VdResult<()>
            where Dp: Handle<Target=DescriptorPoolHandle> {
        let result = self.proc_addr_loader().core.vkFreeDescriptorSets(self.handle().to_raw(),
            descriptor_pool.handle().to_raw(), descriptor_sets.len() as u32,
            descriptor_sets.as_ptr() as *const vks::VkDescriptorSet);
        error::check(result, "vkFreeDescriptorSets", ())
    }

    // *PFN_vkUpdateDescriptorSets)(VkDevice device, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites, uint32_t descriptorCopyCount, const VkCopyDescriptorSet* pDescriptorCopies);
    /// Updates descriptor sets.
    pub fn update_descriptor_sets(&self, descriptor_writes: &[WriteDescriptorSet],
            descriptor_copies: &[CopyDescriptorSet]) {
        unsafe {
            self.proc_addr_loader().core.vkUpdateDescriptorSets(self.handle().0,
                descriptor_writes.len() as u32,
                descriptor_writes.as_ptr() as *const vks::VkWriteDescriptorSet,
                descriptor_copies.len() as u32,
                descriptor_copies.as_ptr() as *const vks::VkCopyDescriptorSet);
        }
    }

    // *PFN_vkCreateFramebuffer)(VkDevice device, const VkFramebufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFramebuffer* pFramebuffer);
    pub unsafe fn create_framebuffer(&self, create_info: &FramebufferCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<FramebufferHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateFramebuffer(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateFramebuffer", FramebufferHandle(handle))
    }

    // *PFN_vkDestroyFramebuffer)(VkDevice device, VkFramebuffer framebuffer, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_framebuffer(&self, framebuffer: FramebufferHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyFramebuffer(self.handle().to_raw(),
            framebuffer.to_raw(), allocator);
    }

    // *PFN_vkCreateRenderPass)(VkDevice device, const VkRenderPassCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkRenderPass* pRenderPass);
    pub unsafe fn create_render_pass(&self, create_info: &RenderPassCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<RenderPassHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateRenderPass(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateRenderPass", RenderPassHandle(handle))
    }

    // *PFN_vkDestroyRenderPass)(VkDevice device, VkRenderPass renderPass, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_render_pass(&self, render_pass: RenderPassHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyRenderPass(self.handle().to_raw(),
            render_pass.to_raw(), allocator);
    }

    // *PFN_vkGetRenderAreaGranularity)(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity);
    pub unsafe fn get_render_area_granularity<Rp>(&self, render_pass: Rp)
            -> Extent2d
            where Rp: Handle<Target=RenderPassHandle> {
        let mut granularity = mem::uninitialized();
        self.proc_addr_loader().core.vkGetRenderAreaGranularity(self.handle().to_raw(),
            render_pass.handle().to_raw(), &mut granularity as *mut _ as *mut vks::VkExtent2D);
        granularity
    }

    // *PFN_vkCreateCommandPool)(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool);
    pub unsafe fn create_command_pool(&self, create_info: &CommandPoolCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<CommandPoolHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateCommandPool(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateCommandPool", CommandPoolHandle(handle))
    }

    // *PFN_vkDestroyCommandPool)(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_command_pool(&self, command_pool: CommandPoolHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyCommandPool(self.handle().to_raw(),
            command_pool.to_raw(), allocator);
    }

    // *PFN_vkResetCommandPool)(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags);
    pub unsafe fn reset_command_pool<Cp>(&self, command_pool: Cp, flags: CommandPoolResetFlags)
            -> VdResult<()>
            where Cp: Handle<Target=CommandPoolHandle> {
        let result = self.proc_addr_loader().core.vkResetCommandPool(self.handle().to_raw(),
            command_pool.handle().to_raw(), flags.bits());
        error::check(result, "vkResetCommandPool", ())
    }

    // *PFN_vkAllocateCommandBuffers)(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers);
    pub unsafe fn allocate_command_buffers(&self, allocate_info: &CommandBufferAllocateInfo)
            -> VdResult<SmallVec<[CommandBufferHandle; 16]>> {
        let mut command_buffers: SmallVec<[CommandBufferHandle; 16]> = SmallVec::new();
        command_buffers.reserve_exact(allocate_info.command_buffer_count() as usize);
        command_buffers.set_len(allocate_info.command_buffer_count() as usize);
        let result = self.proc_addr_loader().core.vkAllocateCommandBuffers(
            self.handle().to_raw(), allocate_info.as_raw(),
            command_buffers.as_mut_ptr() as *mut vks::VkCommandBuffer);
        error::check(result, "vkAllocateCommandBuffers", command_buffers)
    }

    // *PFN_vkFreeCommandBuffers)(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);
    pub unsafe fn free_command_buffers<Cp>(&self, command_pool: Cp, command_buffers: &[CommandBufferHandle])
            where Cp: Handle<Target=CommandPoolHandle> {
        self.proc_addr_loader().core.vkFreeCommandBuffers(self.handle().to_raw(),
            command_pool.handle().to_raw(), command_buffers.len() as u32,
            command_buffers.as_ptr() as *const vks::VkCommandBuffer);
    }

    // *PFN_vkBeginCommandBuffer)(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo);
    pub unsafe fn begin_command_buffer(&self, command_buffer: CommandBufferHandle,
            begin_info: &CommandBufferBeginInfo) -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkBeginCommandBuffer(command_buffer.to_raw(), begin_info.as_raw());
        error::check(result, "vkBeginCommandBuffer", ())
    }

    // *PFN_vkEndCommandBuffer)(VkCommandBuffer commandBuffer);
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBufferHandle) -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkEndCommandBuffer(command_buffer.to_raw());
        error::check(result, "vkEndCommandBuffer", ())
    }


    // *PFN_vkResetCommandBuffer)(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags);
    pub unsafe fn cmd_reset_command_buffer(&self, command_buffer: CommandBufferHandle,
            flags: CommandBufferResetFlags) -> VdResult<()> {
        let result = self.proc_addr_loader().core.vkResetCommandBuffer(command_buffer.to_raw(), flags.bits());
        error::check(result, "vkResetCommandBuffer", ())
    }


    // *PFN_vkCmdBindPipeline)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);
    pub unsafe fn cmd_bind_pipeline(&self, command_buffer: CommandBufferHandle,
            pipeline_bind_point: PipelineBindPoint, pipeline: PipelineHandle) {
        self.proc_addr_loader().core.vkCmdBindPipeline(command_buffer.to_raw(),
            pipeline_bind_point.into(), pipeline.handle().to_raw());
    }

    // *PFN_vkCmdSetViewport)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewport* pViewports);
    pub unsafe fn cmd_set_viewport(&self, command_buffer: CommandBufferHandle,
            first_viewport: u32, viewports: &[Viewport]) {
        self.proc_addr_loader().core.vkCmdSetViewport(command_buffer.to_raw(),
            first_viewport, viewports.len() as u32, viewports.as_ptr() as *const vks::VkViewport);
    }

    // *PFN_vkCmdSetScissor)(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, const VkRect2D* pScissors);
    pub unsafe fn cmd_set_scissor(&self, command_buffer: CommandBufferHandle, first_scissor: u32,
            scissors: &[Rect2d]) {
        self.proc_addr_loader().core.vkCmdSetScissor(command_buffer.to_raw(),
            first_scissor, scissors.len() as u32, scissors.as_ptr() as *const vks::VkRect2D);
    }

    // *PFN_vkCmdSetLineWidth)(VkCommandBuffer commandBuffer, float lineWidth);
    pub unsafe fn cmd_set_line_width(&self, command_buffer: CommandBufferHandle, line_width: f32) {
        self.proc_addr_loader().core.vkCmdSetLineWidth(command_buffer.to_raw(), line_width);
    }

    // *PFN_vkCmdSetDepthBias)(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor);
    pub unsafe fn cmd_set_depth_bias(&self, command_buffer: CommandBufferHandle,
            depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32) {
        self.proc_addr_loader().core.vkCmdSetDepthBias(command_buffer.to_raw(),
            depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor);
    }

    // *PFN_vkCmdSetBlendConstants)(VkCommandBuffer commandBuffer, const float blendConstants[4]);
    pub unsafe fn cmd_set_blend_constants(&self, command_buffer: CommandBufferHandle,
            blend_constants: [f32; 4]) {
        self.proc_addr_loader().core.vkCmdSetBlendConstants(command_buffer.to_raw(),
            blend_constants.as_ptr());
    }

    // *PFN_vkCmdSetDepthBounds)(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds);
    pub unsafe fn cmd_set_depth_bounds(&self, command_buffer: CommandBufferHandle,
            min_depth_bounds: f32, max_depth_bounds: f32) {
        self.proc_addr_loader().core.vkCmdSetDepthBounds(command_buffer.to_raw(),
            min_depth_bounds, max_depth_bounds);
    }

    // *PFN_vkCmdSetStencilCompareMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t compareMask);
    pub unsafe fn cmd_set_stencil_compare_mask(&self, command_buffer: CommandBufferHandle,
            face_mask: StencilFaceFlags, compare_mask: u32) {
        self.proc_addr_loader().core.vkCmdSetStencilCompareMask(command_buffer.to_raw(),
            face_mask.bits(), compare_mask);
    }

    // *PFN_vkCmdSetStencilWriteMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t writeMask);
    pub unsafe fn cmd_set_stencil_write_mask(&self, command_buffer: CommandBufferHandle,
            face_mask: StencilFaceFlags, write_mask: u32) {
        self.proc_addr_loader().core.vkCmdSetStencilWriteMask(command_buffer.to_raw(),
            face_mask.bits(), write_mask);
    }

    // *PFN_vkCmdSetStencilReference)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t reference);
    pub unsafe fn cmd_set_stencil_reference(&self, command_buffer: CommandBufferHandle,
            face_mask: StencilFaceFlags, reference: u32) {
        self.proc_addr_loader().core.vkCmdSetStencilReference(command_buffer.to_raw(),
            face_mask.bits(), reference);
    }

    // *PFN_vkCmdBindDescriptorSets)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets, uint32_t dynamicOffsetCount, const uint32_t* pDynamicOffsets);
    pub unsafe fn cmd_bind_descriptor_sets(&self, command_buffer: CommandBufferHandle,
            pipeline_bind_point: PipelineBindPoint, layout: PipelineLayoutHandle,
            first_set: u32, descriptor_sets: &[DescriptorSetHandle],
            dynamic_offsets: &[u32]) {
        self.proc_addr_loader().core.vkCmdBindDescriptorSets(command_buffer.to_raw(), pipeline_bind_point.into(),
            layout.handle().to_raw(), first_set, descriptor_sets.len() as u32,
            descriptor_sets.as_ptr() as *const vks::VkDescriptorSet,
            dynamic_offsets.len() as u32, dynamic_offsets.as_ptr());
    }

    // *PFN_vkCmdBindIndexBuffer)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType);
    pub unsafe fn cmd_bind_index_buffer(&self, command_buffer: CommandBufferHandle, buffer: BufferHandle,
            offset: u64, index_type: IndexType) {
            self.proc_addr_loader().core.vkCmdBindIndexBuffer(command_buffer.to_raw(),
                buffer.handle().to_raw(), offset, index_type.into());
    }

    // *PFN_vkCmdBindVertexBuffers)(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets);
    pub unsafe fn cmd_bind_vertex_buffers(&self, command_buffer: CommandBufferHandle, first_binding: u32,
            buffers: &[BufferHandle], offsets: &[u64]) {
        self.proc_addr_loader().core.vkCmdBindVertexBuffers(command_buffer.to_raw(),
            first_binding, buffers.len() as u32, buffers.as_ptr() as *const vks::VkBuffer,
            offsets.as_ptr());
    }

    // *PFN_vkCmdDraw)(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance);
    pub unsafe fn cmd_draw(&self, command_buffer: CommandBufferHandle, vertex_count: u32, instance_count: u32,
            first_vertex: u32, first_instance: u32) {
        self.proc_addr_loader().core.vkCmdDraw(command_buffer.to_raw(), vertex_count, instance_count,
            first_vertex, first_instance);
    }

    // *PFN_vkCmdDrawIndexed)(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance);
    pub unsafe fn cmd_draw_indexed(&self, command_buffer: CommandBufferHandle, index_count: u32,
            instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
        self.proc_addr_loader().core.vkCmdDrawIndexed(command_buffer.to_raw(), index_count,
            instance_count, first_index, vertex_offset, first_instance);
    }

    // *PFN_vkCmdDrawIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride);
    pub unsafe fn cmd_draw_indirect(&self, command_buffer: CommandBufferHandle, buffer: BufferHandle,
            offset: u64, draw_count: u32, stride: u32) {
        self.proc_addr_loader().core.vkCmdDrawIndirect(command_buffer.to_raw(),
            buffer.handle().to_raw(), offset, draw_count, stride);
    }

    // *PFN_vkCmdDrawIndexedIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride);
    pub unsafe fn cmd_draw_indexed_indirect(&self, command_buffer: CommandBufferHandle, buffer: BufferHandle,
            offset: u64, draw_count: u32, stride: u32) {
        self.proc_addr_loader().core.vkCmdDrawIndexedIndirect(command_buffer.to_raw(),
            buffer.handle().to_raw(), offset, draw_count, stride);
    }

    // *PFN_vkCmdDispatch)(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);
    pub unsafe fn cmd_dispatch(&self, command_buffer: CommandBufferHandle, group_count_x: u32,
            group_count_y: u32, group_count_z: u32) {
        self.proc_addr_loader().core.vkCmdDispatch(command_buffer.to_raw(), group_count_x,
            group_count_y, group_count_z);
    }

    // *PFN_vkCmdDispatchIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset);
    pub unsafe fn cmd_dispatch_indirect(&self, command_buffer: CommandBufferHandle, buffer: BufferHandle,
            offset: u64) {
        self.proc_addr_loader().core.vkCmdDispatchIndirect(command_buffer.to_raw(),
            buffer.handle().to_raw(), offset);
    }

    // *PFN_vkCmdCopyBuffer)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferCopy* pRegions);
    pub unsafe fn cmd_copy_buffer(&self, command_buffer: CommandBufferHandle, src_buffer: BufferHandle,
            dst_buffer: BufferHandle, regions: &[BufferCopy]) {
        self.proc_addr_loader().core.vkCmdCopyBuffer(
            command_buffer.to_raw(),
            src_buffer.to_raw(),
            dst_buffer.to_raw(),
            regions.len() as u32,
            regions.as_ptr() as *const vks::VkBufferCopy,
        );
    }

    // *PFN_vkCmdCopyImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageCopy* pRegions);
    pub unsafe fn cmd_copy_image(&self, command_buffer: CommandBufferHandle, src_image: ImageHandle,
            src_image_layout: ImageLayout, dst_image: ImageHandle, dst_image_layout: ImageLayout,
            regions: &[ImageCopy]) {
        self.proc_addr_loader().core.vkCmdCopyImage(command_buffer.to_raw(),
        src_image.to_raw(), src_image_layout.into(), dst_image.to_raw(), dst_image_layout.into(),
        regions.len() as u32, regions.as_ptr() as *const vks::VkImageCopy);
    }

    // *PFN_vkCmdBlitImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageBlit* pRegions, VkFilter filter);
    pub unsafe fn cmd_blit_image(&self, command_buffer: CommandBufferHandle, src_image: ImageHandle,
            src_image_layout: ImageLayout, dst_image: ImageHandle, dst_image_layout: ImageLayout,
            regions: &[ImageBlit], filter: Filter) {
        self.proc_addr_loader().core.vkCmdBlitImage(command_buffer.to_raw(),
            src_image.to_raw(), src_image_layout.into(), dst_image.to_raw(),
            dst_image_layout.into(), regions.len() as u32,
            regions.as_ptr() as *const vks::VkImageBlit, filter.into());
    }

    // *PFN_vkCmdCopyBufferToImage)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkBufferImageCopy* pRegions);
    pub unsafe fn cmd_copy_buffer_to_image(&self, command_buffer: CommandBufferHandle,
            src_buffer: BufferHandle, dst_image: ImageHandle, dst_image_layout: ImageLayout,
            regions: &[BufferImageCopy]) {
        self.proc_addr_loader().core.vkCmdCopyBufferToImage(
            command_buffer.to_raw(),
            src_buffer.to_raw(),
            dst_image.to_raw(),
            dst_image_layout as u32,
            regions.len() as u32,
            regions.as_ptr() as *const vks::VkBufferImageCopy,
        );
    }

    // *PFN_vkCmdCopyImageToBuffer)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferImageCopy* pRegions);
    pub unsafe fn cmd_copy_image_to_buffer(&self, command_buffer: CommandBufferHandle,
            src_image: ImageHandle, src_image_layout: ImageLayout, dst_buffer: BufferHandle,
            regions: &[BufferImageCopy]) {
        self.proc_addr_loader().core.vkCmdCopyImageToBuffer(command_buffer.to_raw(),
            src_image.to_raw(), src_image_layout.into(), dst_buffer.to_raw(), regions.len() as u32,
            regions.as_ptr() as *const vks::VkBufferImageCopy);
    }

    // *PFN_vkCmdUpdateBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, const void* pData);
    pub unsafe fn cmd_update_buffer(&self, command_buffer: CommandBufferHandle, dst_buffer: BufferHandle,
            dst_offset: u64, data: &[u8]) {
        self.proc_addr_loader().core.vkCmdUpdateBuffer(command_buffer.to_raw(),
            dst_buffer.to_raw(), dst_offset, data.len() as u64, data.as_ptr() as *const _);
    }

    // *PFN_vkCmdFillBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, uint32_t data);
    pub unsafe fn cmd_fill_buffer(&self,command_buffer: CommandBufferHandle,  dst_buffer: BufferHandle,
            dst_offset: u64, size: Option<DeviceSize>, data: u32) {
        self.proc_addr_loader().core.vkCmdFillBuffer(command_buffer.to_raw(),
            dst_buffer.to_raw(), dst_offset, size.unwrap_or(0), data);
    }

    // *PFN_vkCmdClearColorImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearColorValue* pColor, uint32_t rangeCount, const VkImageSubresourceRange* pRanges);
    pub unsafe fn cmd_clear_color_image(&self, command_buffer: CommandBufferHandle, image: ImageHandle,
            image_layout: ImageLayout, color: &ClearColorValue, ranges: &[ImageSubresourceRange]) {
        self.proc_addr_loader().core.vkCmdClearColorImage(command_buffer.to_raw(),
            image.to_raw(), image_layout.into(), color, ranges.len() as u32,
            ranges.as_ptr() as *const vks::VkImageSubresourceRange);
    }

    // *PFN_vkCmdClearDepthStencilImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearDepthStencilValue* pDepthStencil, uint32_t rangeCount, const VkImageSubresourceRange* pRanges);
    pub unsafe fn cmd_clear_depth_stencil_image(&self, command_buffer: CommandBufferHandle,
            image: ImageHandle, image_layout: ImageLayout, depth_stencil: &ClearDepthStencilValue,
            ranges: &[ImageSubresourceRange]) {
        self.proc_addr_loader().core.vkCmdClearDepthStencilImage(command_buffer.to_raw(),
            image.to_raw(), image_layout.into(), depth_stencil.as_raw(), ranges.len() as u32,
            ranges.as_ptr() as *const vks::VkImageSubresourceRange);
    }

    // *PFN_vkCmdClearAttachments)(VkCommandBuffer commandBuffer, uint32_t attachmentCount, const VkClearAttachment* pAttachments, uint32_t rectCount, const VkClearRect* pRects);
    pub unsafe fn cmd_clear_attachments(&self, command_buffer: CommandBufferHandle,
            attachments: &[ClearAttachment], rects: &[ClearRect]) {
        self.proc_addr_loader().core.vkCmdClearAttachments(command_buffer.to_raw(),
            attachments.len() as u32, attachments.as_ptr() as *const vks::VkClearAttachment,
            rects.len() as u32, rects.as_ptr() as *const vks::VkClearRect);
    }

    // *PFN_vkCmdResolveImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageResolve* pRegions);
    pub unsafe fn cmd_resolve_image(&self, command_buffer: CommandBufferHandle,
            src_image: ImageHandle, src_image_layout: ImageLayout, dst_image: ImageHandle,
            dst_image_layout: ImageLayout, regions: &[ImageResolve]) {
        self.proc_addr_loader().core.vkCmdResolveImage(command_buffer.to_raw(),
            src_image.to_raw(), src_image_layout.into(), dst_image.to_raw(),
            dst_image_layout.into(), regions.len() as u32,
            regions.as_ptr() as *const vks::VkImageResolve);
    }

    // *PFN_vkCmdSetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);
    pub unsafe fn cmd_set_event(&self, command_buffer: CommandBufferHandle, event: EventHandle,
            stage_mask: PipelineStageFlags) {
        self.proc_addr_loader().core.vkCmdSetEvent(command_buffer.to_raw(),
            event.to_raw(), stage_mask.bits());
    }

    // *PFN_vkCmdResetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);
    pub unsafe fn cmd_reset_event(&self, command_buffer: CommandBufferHandle, event: EventHandle,
            stage_mask: PipelineStageFlags) {
        self.proc_addr_loader().core.vkCmdResetEvent(command_buffer.to_raw(),
            event.to_raw(), stage_mask.bits());
    }

    // *PFN_vkCmdWaitEvents)(VkCommandBuffer commandBuffer, uint32_t eventCount, const VkEvent* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);
    pub unsafe fn cmd_wait_events(&self, command_buffer: CommandBufferHandle,
            events: &[EventHandle],
            src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags,
            memory_barriers: &[MemoryBarrier],
            buffer_memory_barriers: &[BufferMemoryBarrier],
            image_memory_barriers: &[ImageMemoryBarrier]) {
        self.proc_addr_loader().core.vkCmdWaitEvents(command_buffer.to_raw(),
            events.len() as u32, events.as_ptr() as *const vks::VkEvent,
            src_stage_mask.bits(), dst_stage_mask.bits(),
            memory_barriers.len() as u32, memory_barriers.as_ptr() as *const vks::VkMemoryBarrier,
            buffer_memory_barriers.len() as u32,
            buffer_memory_barriers.as_ptr() as *const vks::VkBufferMemoryBarrier,
            image_memory_barriers.len() as u32,
            image_memory_barriers.as_ptr() as *const vks::VkImageMemoryBarrier,
        );
    }

    // *PFN_vkCmdPipelineBarrier)(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);
    pub unsafe fn cmd_pipeline_barrier(&self, command_buffer: CommandBufferHandle,
            src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags,
            dependency_flags: DependencyFlags, memory_barriers: &[MemoryBarrier],
            buffer_memory_barriers: &[BufferMemoryBarrier],
            image_memory_barriers: &[ImageMemoryBarrier]) {
        self.proc_addr_loader().core.vkCmdPipelineBarrier(command_buffer.to_raw(),
            src_stage_mask.bits(), dst_stage_mask.bits(), dependency_flags.bits(),
            memory_barriers.len() as u32, memory_barriers.as_ptr() as *const vks::VkMemoryBarrier,
            buffer_memory_barriers.len() as u32,
            buffer_memory_barriers.as_ptr() as *const vks::VkBufferMemoryBarrier,
            image_memory_barriers.len() as u32,
            image_memory_barriers.as_ptr() as *const vks::VkImageMemoryBarrier,
        );
    }

    // *PFN_vkCmdBeginQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags);
    pub unsafe fn cmd_begin_query(&self, command_buffer: CommandBufferHandle,
            query_pool: QueryPoolHandle, query: u32, flags: QueryControlFlags) {
        self.proc_addr_loader().core.vkCmdBeginQuery(command_buffer.to_raw(),
            query_pool.to_raw(), query, flags.bits());
    }

    // *PFN_vkCmdEndQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query);
    pub unsafe fn cmd_end_query(&self, command_buffer: CommandBufferHandle,
            query_pool: QueryPoolHandle, query: u32) {
        self.proc_addr_loader().core.vkCmdEndQuery(command_buffer.to_raw(),
            query_pool.to_raw(), query);
    }

    // *PFN_vkCmdResetQueryPool)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount);
    pub unsafe fn cmd_reset_query_pool(&self, command_buffer: CommandBufferHandle,
            query_pool: QueryPoolHandle, first_query: u32, query_count: u32) {
        self.proc_addr_loader().core.vkCmdResetQueryPool(command_buffer.to_raw(),
            query_pool.to_raw(), first_query, query_count);
    }

    // *PFN_vkCmdWriteTimestamp)(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, uint32_t query);
    pub unsafe fn cmd_write_timestamp(&self, command_buffer: CommandBufferHandle,
        pipeline_stage: PipelineStageFlags, query_pool: QueryPoolHandle, query: u32) {
        self.proc_addr_loader().core.vkCmdWriteTimestamp(command_buffer.to_raw(),
            pipeline_stage.bits(), query_pool.to_raw(), query);
    }

    // *PFN_vkCmdCopyQueryPoolResults)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags);
    pub unsafe fn cmd_copy_query_pool_results(&self, command_buffer: CommandBufferHandle,
            query_pool: QueryPoolHandle, first_query: u32, query_count: u32,
            dst_buffer: BufferHandle, dst_offset: u64, stride: u64, flags: QueryResultFlags) {
        self.proc_addr_loader().core.vkCmdCopyQueryPoolResults(command_buffer.to_raw(),
            query_pool.to_raw(), first_query, query_count, dst_buffer.to_raw(), dst_offset, stride,
            flags.bits());
    }

    // *PFN_vkCmdPushConstants)(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, uint32_t offset, uint32_t size, const void* pValues);
    pub unsafe fn cmd_push_constants(&self, command_buffer: CommandBufferHandle,
            layout: PipelineLayoutHandle, stage_flags: ShaderStageFlags, offset: u32,
            values: &[u8]) {
        self.proc_addr_loader().core.vkCmdPushConstants(command_buffer.to_raw(),
            layout.to_raw(),
            stage_flags.bits(), offset, values.len() as u32, values.as_ptr() as *const c_void);
    }

    // *PFN_vkCmdBeginRenderPass)(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents);
    pub unsafe fn cmd_begin_render_pass(&self, command_buffer: CommandBufferHandle,
            render_pass_begin: &RenderPassBeginInfo, contents: SubpassContents) {
        self.proc_addr_loader().core.vkCmdBeginRenderPass(command_buffer.to_raw(),
            render_pass_begin.as_raw(), contents.into());
    }

    // *PFN_vkCmdNextSubpass)(VkCommandBuffer commandBuffer, VkSubpassContents contents);
    pub unsafe fn cmd_next_subpass(&self, command_buffer: CommandBufferHandle,
            contents: SubpassContents) {
        self.proc_addr_loader().core.vkCmdNextSubpass(command_buffer.to_raw(), contents.into());
    }

    // *PFN_vkCmdEndRenderPass)(VkCommandBuffer commandBuffer);
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBufferHandle, ) {
        self.proc_addr_loader().core.vkCmdEndRenderPass(command_buffer.to_raw());
    }

    // *PFN_vkCmdExecuteCommands)(VkCommandBuffer commandBuffer, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);
    pub unsafe fn cmd_execute_commands(&self, command_buffer: CommandBufferHandle,
            command_buffers: &[CommandBufferHandle]) {
        self.proc_addr_loader().core.vkCmdExecuteCommands(command_buffer.to_raw(),
            command_buffers.len() as u32, command_buffers.as_ptr() as *const vks::VkCommandBuffer);
    }

    // *PFN_vkCreateSwapchainKHR)(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain);
    pub unsafe fn create_swapchain_khr(&self, create_info: &SwapchainCreateInfoKhr,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<SwapchainKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().khr_swapchain.vkCreateSwapchainKHR(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateSwapchainKHR", SwapchainKhrHandle(handle))
    }

    // *PFN_vkDestroySwapchainKHR)(VkDevice device, VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator);
    pub unsafe fn destroy_swapchain_khr(&mut self, swapchain: SwapchainKhrHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let _allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().khr_swapchain.vkDestroySwapchainKHR(self.handle().to_raw(),
            swapchain.to_raw(), ptr::null());
    }

    // *PFN_vkGetSwapchainImagesKHR)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages);
    pub unsafe fn get_swapchain_images_khr(&self, swapchain: SwapchainKhrHandle)
            -> VdResult<SmallVec<[ImageHandle; 4]>> {
        let mut image_count = 0;
        let mut image_handles = SmallVec::<[ImageHandle; 4]>::new();
        let result = self.proc_addr_loader().khr_swapchain.vkGetSwapchainImagesKHR(self.handle().to_raw(),
            swapchain.to_raw(), &mut image_count, ptr::null_mut());
        error::check(result, "vkGetSwapchainImagesKHR", ())?;
        image_handles.reserve_exact(image_count as usize);
        image_handles.set_len(image_count as usize);
        loop {
            let result = self.proc_addr_loader().khr_swapchain.vkGetSwapchainImagesKHR(self.handle().to_raw(),
                swapchain.to_raw(), &mut image_count, image_handles.as_mut_ptr() as *mut vks::VkImage);
            if result != CallResult::Incomplete as i32 {
                return error::check(result, "vkGetSwapchainImagesKHR", image_handles);
            }
        }
    }

    // *PFN_vkAcquireNextImageKHR)(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex);
    pub unsafe fn acquire_next_image_khr(&self, swapchain: SwapchainKhrHandle, _timeout: u64,
            semaphore: Option<SemaphoreHandle>, fence: Option<FenceHandle>, _image_index: u32)
            -> VdResult<u32> {
        let mut image_index = 0;
        let result = self.proc_addr_loader().khr_swapchain.vkAcquireNextImageKHR(
                self.handle().to_raw(), swapchain.to_raw(), u64::max_value(),
                semaphore.map(|s| s.to_raw()).unwrap_or(0),
                fence.map(|f| f.to_raw()).unwrap_or(0), &mut image_index);
        error::check(result, "vkAcquireNextImageKHR", image_index)
    }

    // *PFN_vkQueuePresentKHR)(VkQueue queue, const VkPresentInfoKHR* pPresentInfo);
    pub unsafe fn queue_present_khr(&self, queue: QueueHandle, present_info: &PresentInfoKhr)
            -> VdResult<()> {
        let result = self.proc_addr_loader().khr_swapchain.vkQueuePresentKHR(queue.to_raw(),
            present_info.as_raw());
        error::check(result, "vkQueuePresentKHR", ())
    }

    // *PFN_vkCreateSharedSwapchainsKHR)(VkDevice device, uint32_t swapchainCount, const VkSwapchainCreateInfoKHR* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchains);
    pub unsafe fn create_shared_swapchains_khr(&self, create_infos: &[SwapchainCreateInfoKhr],
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SmallVec<[SwapchainKhrHandle; 4]>> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut swapchains = SmallVec::<[SwapchainKhrHandle; 4]>::new();
        swapchains.reserve_exact(create_infos.len());
        swapchains.set_len(create_infos.len());
        let result = self.proc_addr_loader().khr_display_swapchain.vkCreateSharedSwapchainsKHR(self.handle().to_raw(),
            create_infos.len() as u32, create_infos as *const _ as *const vks::VkSwapchainCreateInfoKHR,
            allocator, swapchains.as_mut_ptr() as *mut vks::VkSwapchainKHR);
        error::check(result, "vkCreateSharedSwapchainsKHR", swapchains)
    }

    // *PFN_vkTrimCommandPoolKHR)(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlagsKHR flags);
    pub unsafe fn trim_command_pool_khr<P>(&self, _command_pool: P, _flags: CommandPoolTrimFlagsKhr)
             -> VdResult<()>
            where P: Handle<Target=CommandPoolHandle> {
        // self.proc_addr_loader().
        //     vkTrimCommandPoolKHR)(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlagsKHR flags);
        unimplemented!();
    }

    // *PFN_vkGetMemoryWin32HandleKHR)(VkDevice device, const VkMemoryGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);
    pub unsafe fn get_memory_win32_handle_khr(&self,
            _get_win32_handle_info: &MemoryGetWin32HandleInfoKhr)
             -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetMemoryWin32HandleKHR)(VkDevice device, const VkMemoryGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);
        unimplemented!();
    }

    // *PFN_vkGetMemoryWin32HandlePropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, HANDLE handle, VkMemoryWin32HandlePropertiesKHR* pMemoryWin32HandleProperties);
    pub unsafe fn get_memory_win32_handle_properties_khr(&self,
            _handle_type: ExternalMemoryHandleTypeFlagsKhr, _handle: HANDLE) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetMemoryWin32HandlePropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, HANDLE handle, VkMemoryWin32HandlePropertiesKHR* pMemoryWin32HandleProperties);
        unimplemented!();
    }

    // *PFN_vkGetMemoryFdKHR)(VkDevice device, const VkMemoryGetFdInfoKHR* pGetFdInfo, int* pFd);
    pub unsafe fn get_memory_fd_khr(&self, _get_fd_info: &MemoryGetFdInfoKhr, _fd: &mut i32)
            -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetMemoryFdKHR)(VkDevice device, const VkMemoryGetFdInfoKHR* pGetFdInfo, int* pFd);
        unimplemented!();
    }

    // *PFN_vkGetMemoryFdPropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, int fd, VkMemoryFdPropertiesKHR* pMemoryFdProperties);
    pub unsafe fn get_memory_fd_properties_khr(&self, _handle_type: ExternalMemoryHandleTypeFlagsKhr,
            _fd: i32) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetMemoryFdPropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, int fd, VkMemoryFdPropertiesKHR* pMemoryFdProperties);
        unimplemented!();
    }

    // *PFN_vkImportSemaphoreWin32HandleKHR)(VkDevice device, const VkImportSemaphoreWin32HandleInfoKHR* pImportSemaphoreWin32HandleInfo);
    pub unsafe fn import_semaphore_win32_handle_khr(&self,
            _import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKhr) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkImportSemaphoreWin32HandleKHR)(VkDevice device, const VkImportSemaphoreWin32HandleInfoKHR* pImportSemaphoreWin32HandleInfo);
        unimplemented!();
    }

    // *PFN_vkGetSemaphoreWin32HandleKHR)(VkDevice device, const VkSemaphoreGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);
    pub unsafe fn get_semaphore_win32_handle_khr(&self,
            _get_win32_handle_info: &SemaphoreGetWin32HandleInfoKhr) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetSemaphoreWin32HandleKHR)(VkDevice device, const VkSemaphoreGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);
        unimplemented!();
    }

    // *PFN_vkImportSemaphoreFdKHR)(VkDevice device, const VkImportSemaphoreFdInfoKHR* pImportSemaphoreFdInfo);
    pub unsafe fn import_semaphore_fd_khr(&self,
            _import_semaphore_fd_info: &ImportSemaphoreFdInfoKhr) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkImportSemaphoreFdKHR)(VkDevice device, const VkImportSemaphoreFdInfoKHR* pImportSemaphoreFdInfo);
        unimplemented!();
    }

    // *PFN_vkGetSemaphoreFdKHR)(VkDevice device, const VkSemaphoreGetFdInfoKHR* pGetFdInfo, int* pFd);
    pub unsafe fn get_semaphore_fd_khr(&self, _get_fd_info: &SemaphoreGetFdInfoKhr)
            -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetSemaphoreFdKHR)(VkDevice device, const VkSemaphoreGetFdInfoKHR* pGetFdInfo, int* pFd);
        unimplemented!();
    }

    // *PFN_vkCmdPushDescriptorSetKHR)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites);
    pub unsafe fn cmd_push_descriptor_set_khr<Cb>(&self, _command_buffer: Cb,
            _pipeline_bind_point: PipelineBindPoint, _layout: PipelineLayout, _set: u32,
            _descriptor_writes: &[WriteDescriptorSet]) -> VdResult<()>
            where Cb: Handle<Target=CommandBufferHandle> {
        // self.proc_addr_loader().
        //     vkCmdPushDescriptorSetKHR)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites);
        unimplemented!();
    }

    // *PFN_vkCreateDescriptorUpdateTemplateKHR)(VkDevice device, const VkDescriptorUpdateTemplateCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorUpdateTemplateKHR* pDescriptorUpdateTemplate);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn create_descriptor_update_template_khr(&self,
            create_info: &DescriptorUpdateTemplateKhrCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<DescriptorUpdateTemplateKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateDescriptorUpdateTemplateKhr(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateDescriptorUpdateTemplateKhr",
            DescriptorUpdateTemplateKhrHandle(handle))
    }

    // *PFN_vkDestroyDescriptorUpdateTemplateKHR)(VkDevice device, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const VkAllocationCallbacks* pAllocator);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn destroy_descriptor_update_template_khr(&self,
            descriptor_update_template_khr: DescriptorUpdateTemplateKhrHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyDescriptorUpdateTemplateKhr(self.handle().to_raw(),
            descriptor_update_template_khr.to_raw(), allocator);
    }

    // *PFN_vkUpdateDescriptorSetWithTemplateKHR)(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const void* pData);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn update_descriptor_set_with_template_khr<Ds>(&self, descriptor_set: Ds,
            descriptor_update_template: DescriptorUpdateTemplateKhrHandle, data: *const c_void)
            where Ds: Handle<Target=DescriptorSetHandle> {
        // self.proc_addr_loader().
        //     vkUpdateDescriptorSetWithTemplateKHR)(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const void* pData);
        unimplemented!();
    }

    // *PFN_vkCmdPushDescriptorSetWithTemplateKHR)(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, const void* pData);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn cmd_push_descriptor_set_with_template_khr<Cb, Pl>(&self, command_buffer: Cb,
            descriptor_update_template: DescriptorUpdateTemplateKhr, layout: Pl, set: u32,
            data: *const c_void) -> VdResult<()>
            where Cb: Handle<Target=CommandBufferHandle>, Pl: Handle<Target=PipelineLayoutHandle> {
        // self.proc_addr_loader().
        //     vkCmdPushDescriptorSetWithTemplateKHR)(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, const void* pData);
        unimplemented!();
    }

    // *PFN_vkGetSwapchainStatusKHR)(VkDevice device, VkSwapchainKHR swapchain);
    pub unsafe fn get_swapchain_status_khr<Sk>(&self, _swapchain: Sk) -> VdResult<()>
            where Sk: Handle<Target=SwapchainKhrHandle> {
        // self.proc_addr_loader().
        //     vkGetSwapchainStatusKHR)(VkDevice device, VkSwapchainKHR swapchain);
        unimplemented!();
    }

    // *PFN_vkImportFenceWin32HandleKHR)(VkDevice device, const VkImportFenceWin32HandleInfoKHR* pImportFenceWin32HandleInfo);
    pub unsafe fn import_fence_win32_handle_khr(&self,
            _import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKhr) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkImportFenceWin32HandleKHR)(VkDevice device, const VkImportFenceWin32HandleInfoKHR* pImportFenceWin32HandleInfo);
        unimplemented!();
    }

    // *PFN_vkGetFenceWin32HandleKHR)(VkDevice device, const VkFenceGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);
    pub unsafe fn get_fence_win32_handle_khr(&self,
            _get_win32_handle_info: &FenceGetWin32HandleInfoKhr) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetFenceWin32HandleKHR)(VkDevice device, const VkFenceGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);
        unimplemented!();
    }

    // *PFN_vkImportFenceFdKHR)(VkDevice device, const VkImportFenceFdInfoKHR* pImportFenceFdInfo);
    pub unsafe fn import_fence_fd_khr(&self, _import_fence_fd_info: &ImportFenceFdInfoKhr)
            -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkImportFenceFdKHR)(VkDevice device, const VkImportFenceFdInfoKHR* pImportFenceFdInfo);
        unimplemented!();
    }

    // *PFN_vkGetFenceFdKHR)(VkDevice device, const VkFenceGetFdInfoKHR* pGetFdInfo, int* pFd);
    pub unsafe fn get_fence_fd_khr(&self, _get_fd_info: &FenceGetFdInfoKhr) -> VdResult<()> {
        // self.proc_addr_loader().
        //     vkGetFenceFdKHR)(VkDevice device, const VkFenceGetFdInfoKHR* pGetFdInfo, int* pFd);
        unimplemented!();
    }

    // *PFN_vkGetImageMemoryRequirements2KHR)(VkDevice device, const VkImageMemoryRequirementsInfo2KHR* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements);
    pub unsafe fn get_image_memory_requirements_2_khr(&self,
            _info: &ImageMemoryRequirementsInfo2Khr) -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkGetBufferMemoryRequirements2KHR)(VkDevice device, const VkBufferMemoryRequirementsInfo2KHR* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements);
    pub fn get_buffer_memory_requirements_2_khr(&self, _info: &BufferMemoryRequirementsInfo2Khr)
            -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkGetImageSparseMemoryRequirements2KHR)(VkDevice device, const VkImageSparseMemoryRequirementsInfo2KHR* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2KHR* pSparseMemoryRequirements);
    pub unsafe fn get_image_sparse_memory_requirements_2_khr(&self,
            _info: &ImageSparseMemoryRequirementsInfo2Khr) -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkCreateSamplerYcbcrConversionKHR)(VkDevice device, const VkSamplerYcbcrConversionCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSamplerYcbcrConversionKHR* pYcbcrConversion);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn create_sampler_ycbcr_conversion_khr(&self,
            create_info: &SamplerYcbcrConversionKhrCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<SamplerYcbcrConversionKhrHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateSamplerYcbcrConversionKhr(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateSamplerYcbcrConversionKhr",
            SamplerYcbcrConversionKhrHandle(handle))
    }

    // *PFN_vkDestroySamplerYcbcrConversionKHR)(VkDevice device, VkSamplerYcbcrConversionKHR ycbcrConversion, const VkAllocationCallbacks* pAllocator);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(&self,
            sampler_ycbcr_conversion_khr: SamplerYcbcrConversionKhrHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroySamplerYcbcrConversionKhr(self.handle().to_raw(),
            sampler_ycbcr_conversion_khr.to_raw(), allocator);
    }

    // *PFN_vkBindBufferMemory2KHR)(VkDevice device, uint32_t bindInfoCount, const VkBindBufferMemoryInfoKHR* pBindInfos);
    pub unsafe fn bind_buffer_memory_2_khr(&self) {
        unimplemented!();
    }

    // *PFN_vkBindImageMemory2KHR)(VkDevice device, uint32_t bindInfoCount, const VkBindImageMemoryInfoKHR* pBindInfos);
    pub unsafe fn bind_image_memory_2_khr(&self) {
        unimplemented!();
    }

    // *PFN_vkDebugMarkerSetObjectTagEXT)(VkDevice device, const VkDebugMarkerObjectTagInfoEXT* pTagInfo);
    pub unsafe fn debug_marker_set_object_tag_ext(&self, _tag_info: &DebugMarkerObjectTagInfoExt)
            -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkDebugMarkerSetObjectNameEXT)(VkDevice device, const VkDebugMarkerObjectNameInfoEXT* pNameInfo);
    pub unsafe fn debug_marker_set_object_name_ext(&self, _name_info: &DebugMarkerObjectNameInfoExt)
            -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkCmdDebugMarkerBeginEXT)(VkCommandBuffer commandBuffer, const VkDebugMarkerMarkerInfoEXT* pMarkerInfo);
    pub unsafe fn cmd_debug_marker_begin_ext(&self, command_buffer: CommandBufferHandle,
            marker_info: &DebugMarkerMarkerInfoExt) {
        self.proc_addr_loader().ext_debug_marker.vkCmdDebugMarkerBeginEXT(command_buffer.to_raw(),
            marker_info.as_raw());
    }

    // *PFN_vkCmdDebugMarkerEndEXT)(VkCommandBuffer commandBuffer);
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: CommandBufferHandle) {
        self.proc_addr_loader().ext_debug_marker.vkCmdDebugMarkerEndEXT(command_buffer.to_raw());
    }

    // *PFN_vkCmdDebugMarkerInsertEXT)(VkCommandBuffer commandBuffer, const VkDebugMarkerMarkerInfoEXT* pMarkerInfo);
    pub unsafe fn cmd_debug_marker_insert_ext(&self, command_buffer: CommandBufferHandle,
            marker_info: &DebugMarkerMarkerInfoExt) {
        self.proc_addr_loader().ext_debug_marker.vkCmdDebugMarkerInsertEXT(command_buffer.to_raw(),
            marker_info.as_raw());
    }

    // *PFN_vkCmdDrawIndirectCountAMD)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride);
    pub unsafe fn cmd_draw_indirect_count_amd(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdDrawIndexedIndirectCountAMD)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride);
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(&self) {
        unimplemented!();
    }

    // *PFN_vkGetMemoryWin32HandleNV)(VkDevice device, VkDeviceMemory memory, VkExternalMemoryHandleTypeFlagsNV handleType, HANDLE* pHandle);
    pub unsafe fn get_memory_win32_handle_nv(&self) {
        unimplemented!();
    }

    // *PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX)(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlagsKHX* pPeerMemoryFeatures);
    pub unsafe fn get_device_group_peer_memory_features_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdSetDeviceMaskKHX)(VkCommandBuffer commandBuffer, uint32_t deviceMask);
    pub unsafe fn cmd_set_device_mask_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdDispatchBaseKHX)(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);
    pub unsafe fn cmd_dispatch_base_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkGetDeviceGroupPresentCapabilitiesKHX)(VkDevice device, VkDeviceGroupPresentCapabilitiesKHX* pDeviceGroupPresentCapabilities);
    pub unsafe fn get_device_group_present_capabilities_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkGetDeviceGroupSurfacePresentModesKHX)(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHX* pModes);
    pub unsafe fn get_device_group_surface_present_modes_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkAcquireNextImage2KHX)(VkDevice device, const VkAcquireNextImageInfoKHX* pAcquireInfo, uint32_t* pImageIndex);
    pub unsafe fn acquire_next_image2_khx(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdProcessCommandsNVX)(VkCommandBuffer commandBuffer, const VkCmdProcessCommandsInfoNVX* pProcessCommandsInfo);
    pub unsafe fn cmd_process_commands_nvx(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdReserveSpaceForCommandsNVX)(VkCommandBuffer commandBuffer, const VkCmdReserveSpaceForCommandsInfoNVX* pReserveSpaceInfo);
    pub unsafe fn cmd_reserve_space_for_commands_nvx(&self) {
        unimplemented!();
    }

    // *PFN_vkCreateIndirectCommandsLayoutNVX)(VkDevice device, const VkIndirectCommandsLayoutCreateInfoNVX* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkIndirectCommandsLayoutNVX* pIndirectCommandsLayout);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn create_indirect_commands_layout_nvx(&self,
            create_info: &IndirectCommandsLayoutNvxCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<IndirectCommandsLayoutNvxHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateIndirectCommandsLayoutNvx(
            self.handle().to_raw(), create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateIndirectCommandsLayoutNvx",
            IndirectCommandsLayoutNvxHandle(handle))
    }

    // *PFN_vkDestroyIndirectCommandsLayoutNVX)(VkDevice device, VkIndirectCommandsLayoutNVX indirectCommandsLayout, const VkAllocationCallbacks* pAllocator);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn destroy_indirect_commands_layout_nvx(&self,
            indirect_commands_layout_nvx: IndirectCommandsLayoutNvxHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyIndirectCommandsLayoutNvx(self.handle().to_raw(),
            indirect_commands_layout_nvx.to_raw(), allocator);
    }

    // *PFN_vkCreateObjectTableNVX)(VkDevice device, const VkObjectTableCreateInfoNVX* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkObjectTableNVX* pObjectTable);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn create_object_table_nvx(&self, create_info: &ObjectTableNvxCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<ObjectTableNvxHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateObjectTableNvx(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        error::check(result, "vkCreateObjectTableNvx", ObjectTableNvxHandle(handle))
    }

    // *PFN_vkDestroyObjectTableNVX)(VkDevice device, VkObjectTableNVX objectTable, const VkAllocationCallbacks* pAllocator);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn destroy_object_table_nvx(&self, object_table_nvx: ObjectTableNvxHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyObjectTableNvx(self.handle().to_raw(),
            object_table_nvx.to_raw(), allocator);
    }

    // *PFN_vkRegisterObjectsNVX)(VkDevice device, VkObjectTableNVX objectTable, uint32_t objectCount, const VkObjectTableEntryNVX* const*    ppObjectTableEntries, const uint32_t* pObjectIndices);
    pub unsafe fn register_objects_nvx(&self) {
        unimplemented!();
    }

    // *PFN_vkUnregisterObjectsNVX)(VkDevice device, VkObjectTableNVX objectTable, uint32_t objectCount, const VkObjectEntryTypeNVX* pObjectEntryTypes, const uint32_t* pObjectIndices);
    pub unsafe fn unregister_objects_nvx(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdSetViewportWScalingNV)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewportWScalingNV* pViewportWScalings);
    pub unsafe fn cmd_set_viewport_w_scaling_nv(&self) {
        unimplemented!();
    }

    // *PFN_vkDisplayPowerControlEXT)(VkDevice device, VkDisplayKHR display, const VkDisplayPowerInfoEXT* pDisplayPowerInfo);
    pub unsafe fn display_power_control_ext<Dk>(&self, _display: Dk,
            _display_power_info: &DisplayPowerInfoExt)
            where Dk: Handle<Target=DisplayKhrHandle> {
        unimplemented!();
    }

    // *PFN_vkRegisterDeviceEventEXT)(VkDevice device, const VkDeviceEventInfoEXT* pDeviceEventInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);
    pub unsafe fn register_device_event_ext(&self, _device_event_info: &DeviceEventInfoExt,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<()> {
        let _allocator = allocator.unwrap_or(ptr::null());
        unimplemented!();
    }

    // *PFN_vkRegisterDisplayEventEXT)(VkDevice device, VkDisplayKHR display, const VkDisplayEventInfoEXT* pDisplayEventInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);
    pub unsafe fn register_display_event_ext<Dk>(&self, _display: Dk,
            _display_event_info: &DisplayEventInfoExt,
            allocator: Option<*const vks::VkAllocationCallbacks>) -> VdResult<()>
            where Dk: Handle<Target=DisplayKhrHandle> {
        let _allocator = allocator.unwrap_or(ptr::null());
        unimplemented!();
    }

    // *PFN_vkGetSwapchainCounterEXT)(VkDevice device, VkSwapchainKHR swapchain, VkSurfaceCounterFlagBitsEXT counter, uint64_t* pCounterValue);
    pub unsafe fn get_swapchain_counter_ext<Sk>(&self, _swapchain: Sk,
            _counter: SurfaceCounterFlagsExt) -> VdResult<u64>
            where Sk: Handle<Target=SwapchainKhrHandle> {
        unimplemented!();
    }

    // *PFN_vkGetRefreshCycleDurationGOOGLE)(VkDevice device, VkSwapchainKHR swapchain, VkRefreshCycleDurationGOOGLE* pDisplayTimingProperties);
    pub unsafe fn get_refresh_cycle_duration_google(&self) {
        unimplemented!();
    }

    // *PFN_vkGetPastPresentationTimingGOOGLE)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pPresentationTimingCount, VkPastPresentationTimingGOOGLE* pPresentationTimings);
    pub unsafe fn get_past_presentation_timing_google(&self) {
        unimplemented!();
    }

    // *PFN_vkCmdSetDiscardRectangleEXT)(VkCommandBuffer commandBuffer, uint32_t firstDiscardRectangle, uint32_t discardRectangleCount, const VkRect2D* pDiscardRectangles);
    pub unsafe fn cmd_set_discard_rectangle_ext<Cb>(&self, _command_buffer: Cb,
            _first_discard_rectangle: u32, _discard_rectangle_count: u32, _discard_rectangles: &Rect2d)
            -> VdResult<()>
            where Cb: Handle<Target=CommandBufferHandle> {
        unimplemented!();
    }

    // *PFN_vkSetHdrMetadataEXT)(VkDevice device, uint32_t swapchainCount, const VkSwapchainKHR* pSwapchains, const VkHdrMetadataEXT* pMetadata);
    pub unsafe fn set_hdr_metadata_ext(&self, _swapchains: &[SwapchainKhrHandle],
            _metadata: &HdrMetadataExt) -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkCmdSetSampleLocationsEXT)(VkCommandBuffer commandBuffer, const VkSampleLocationsInfoEXT* pSampleLocationsInfo);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn cmd_set_sample_locations_ext<Cb>(&self, command_buffer: Cb,
            sample_locations_info: &SampleLocationsInfoExt) -> VdResult<()>
            where Cb: Handle<Target=CommandBufferHandle> {
        unimplemented!();
    }

    // *PFN_vkCreateValidationCacheEXT)(VkDevice device, const VkValidationCacheCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkValidationCacheEXT* pValidationCache);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn create_validation_cache_ext(&self,
            create_info: &ValidationCacheExtCreateInfo,
            allocator: Option<*const vks::VkAllocationCallbacks>)
            -> VdResult<ValidationCacheExtHandle> {
        let allocator = allocator.unwrap_or(ptr::null());
        let mut handle = 0;
        let result = self.proc_addr_loader().core.vkCreateValidationCacheExt(self.handle().to_raw(),
            create_info.as_raw(), allocator, &mut handle);
        // Ok(ValidationCacheExtHandle(handle))
        error::check(result, "vkCreateValidationCacheExt", ValidationCacheExtHandle(handle))
    }

    // *PFN_vkDestroyValidationCacheEXT)(VkDevice device, VkValidationCacheEXT validationCache, const VkAllocationCallbacks* pAllocator);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn destroy_validation_cache_ext(&self,
            validation_cache_ext: ValidationCacheExtHandle,
            allocator: Option<*const vks::VkAllocationCallbacks>) {
        let allocator = allocator.unwrap_or(ptr::null());
        self.proc_addr_loader().core.vkDestroyValidationCacheExt(self.handle().to_raw(),
            validation_cache_ext.to_raw(), allocator);
    }

    // *PFN_vkMergeValidationCachesEXT)(VkDevice device, VkValidationCacheEXT dstCache, uint32_t srcCacheCount, const VkValidationCacheEXT* pSrcCaches);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn merge_validation_caches_ext(&self, dst_cache: ValidationCacheExt,
            src_caches: &[ValidationCacheExt]) -> VdResult<()> {
        unimplemented!();
    }

    // *PFN_vkGetValidationCacheDataEXT)(VkDevice device, VkValidationCacheEXT validationCache, size_t* pDataSize, void* pData);
    #[cfg(feature = "unimplemented")]
    pub unsafe fn get_validation_cache_data_ext(&self, validation_cache: ValidationCacheExt,
            data_size: *mut usize, data: *mut c_void) -> VdResult<()> {
        unimplemented!();
    }
}

unsafe impl<'h> Handle for &'h Device {
    type Target = DeviceHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        if PRINT { println!("Destroying device..."); }
        unsafe {
            self.instance.destroy_device(self.handle, None);
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}


/// A builder for `Device`.
#[derive(Debug, Clone)]
pub struct DeviceBuilder<'db> {
    create_info: ::DeviceCreateInfo<'db>,
    enabled_layer_names: Option<CharStrs<'db>>,
    enabled_extension_names: Option<CharStrs<'db>>,
    _p: PhantomData<&'db ()>,
}

impl<'db> DeviceBuilder<'db> {
    /// Returns a new instance builder.
    pub fn new() -> DeviceBuilder<'db> {
        DeviceBuilder {
            create_info: ::DeviceCreateInfo::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            _p: PhantomData,
        }
    }

    /// Specifies the list of VkDeviceQueueCreateInfo structures describing
    /// the queues that are requested to be created along with the logical
    /// device.
    pub fn queue_create_infos<'s, 'ci>(&'s mut self,
            queue_create_infos: &'ci [DeviceQueueCreateInfo])
            -> &'s mut DeviceBuilder<'db>
            where 'ci: 'db {
        // self.create_info.queueCreateInfoCount = queue_create_infos.len() as u32;
        debug_assert_eq!(mem::align_of::<DeviceQueueCreateInfo>(),
            mem::align_of::<vks::VkDeviceQueueCreateInfo>());
        debug_assert_eq!(mem::size_of::<DeviceQueueCreateInfo>(),
            mem::size_of::<vks::VkDeviceQueueCreateInfo>());
        // self.create_info.queue_create_infos = queue_create_infos.as_ptr() as *const _;
        self.create_info.set_queue_create_infos(queue_create_infos);
        self
    }

    /// Specifies the layer names to enable.
    ///
    /// Ignored.
    #[deprecated(note = "No longer used")]
    pub fn enabled_layer_names<'s, 'cs, Cs>(&'s mut self, enabled_layer_names: Cs)
            -> &'s mut DeviceBuilder<'db>
            where 'cs: 'db, Cs: 'cs + Into<CharStrs<'cs>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        if let Some(ref elns) = self.enabled_layer_names {
            // self.create_info.set_enabled_layer_count(elns.len() as u32);
            self.create_info.set_enabled_layer_names(elns.as_ptr_slice());
        }
        // self.create_info.set_enabled_layer_names(enabled_layer_names);
        self
    }

    /// Specifies the list of names of extensions to enable for the created
    /// device.
    pub fn enabled_extension_names<'s, 'cs, Cs>(&'s mut self, enabled_extension_names: Cs)
            -> &'s mut DeviceBuilder<'db>
            where 'cs: 'db, Cs: 'cs + Into<CharStrs<'cs>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        if let Some(ref eens) = self.enabled_extension_names {
            // self.create_info.enabledExtensionCount = eens.len() as u32;
            // self.create_info.enabled_extension_names = eens.as_ptr() as *const _;
            self.create_info.set_enabled_extension_names(eens.as_ptr_slice());
        }
        // self.create_info.set_enabled_extension_names(enabled_extension_names);
        self
    }

    /// Specifies the structure that contains boolean indicators of all the
    /// features to be enabled.
    pub fn enabled_features<'s, 'f>(&'s mut self, enabled_features: &'f PhysicalDeviceFeatures)
            -> &'s mut DeviceBuilder<'db>
            where 'f: 'db {
        self.create_info.set_enabled_features(enabled_features);
        self
    }

    /// Builds and returns a new `Device`.
    pub fn build(&self, physical_device: PhysicalDevice) -> VdResult<Device> {
        let handle = unsafe {
            physical_device.instance().create_device(physical_device.handle(), &self.create_info, None)?
        };

        let mut loader = vks::DeviceProcAddrLoader::from_get_device_proc_addr(
            physical_device.instance().proc_addr_loader().core.pfn_vkGetDeviceProcAddr);

        unsafe {
            loader.load_core(handle.to_raw());
        }

        unsafe {
            if let Some(extension_name_char_strs) = self.enabled_extension_names.as_ref() {
                let extension_names = extension_name_char_strs.as_ptr_slice();
                for &extension_name in extension_names {
                    match CStr::from_ptr(extension_name).to_str().expect("invalid extension name") {
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_16bit_storage" => loader.load_khr_16bit_storage(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_android_surface" => loader.load_khr_android_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_bind_memory2" => loader.load_khr_bind_memory2(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_dedicated_allocation" => loader.load_khr_dedicated_allocation(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_descriptor_update_template" => loader.load_khr_descriptor_update_template(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_display" => loader.load_khr_display(handle.to_raw()),
                        "VK_KHR_display_swapchain" => loader.load_khr_display_swapchain(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_fence" => loader.load_khr_external_fence(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_fence_capabilities" => loader.load_khr_external_fence_capabilities(handle.to_raw()),
                        "VK_KHR_external_fence_fd" => loader.load_khr_external_fence_fd(handle.to_raw()),
                        "VK_KHR_external_fence_win32" => loader.load_khr_external_fence_win32(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_memory" => loader.load_khr_external_memory(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_memory_capabilities" => loader.load_khr_external_memory_capabilities(handle.to_raw()),
                        "VK_KHR_external_memory_fd" => loader.load_khr_external_memory_fd(handle.to_raw()),
                        "VK_KHR_external_memory_win32" => loader.load_khr_external_memory_win32(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_semaphore" => loader.load_khr_external_semaphore(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_external_semaphore_capabilities" => loader.load_khr_external_semaphore_capabilities(handle.to_raw()),
                        "VK_KHR_external_semaphore_fd" => loader.load_khr_external_semaphore_fd(handle.to_raw()),
                        "VK_KHR_external_semaphore_win32" => loader.load_khr_external_semaphore_win32(handle.to_raw()),
                        "VK_KHR_get_memory_requirements2" => loader.load_khr_get_memory_requirements2(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_get_physical_device_properties2" => loader.load_khr_get_physical_device_properties2(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_get_surface_capabilities2" => loader.load_khr_get_surface_capabilities2(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_image_format_list" => loader.load_khr_image_format_list(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_incremental_present" => loader.load_khr_incremental_present(handle.to_raw()),
                        "VK_KHR_maintenance1" => loader.load_khr_maintenance1(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_maintenance2" => loader.load_khr_maintenance2(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_mir_surface" => loader.load_khr_mir_surface(handle.to_raw()),
                        "VK_KHR_push_descriptor" => loader.load_khr_push_descriptor(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_relaxed_block_layout" => loader.load_khr_relaxed_block_layout(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_sampler_mirror_clamp_to_edge" => loader.load_khr_sampler_mirror_clamp_to_edge(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_sampler_ycbcr_conversion" => loader.load_khr_sampler_ycbcr_conversion(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_shader_draw_parameters" => loader.load_khr_shader_draw_parameters(handle.to_raw()),
                        "VK_KHR_shared_presentable_image" => loader.load_khr_shared_presentable_image(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_storage_buffer_storage_class" => loader.load_khr_storage_buffer_storage_class(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_surface" => loader.load_khr_surface(handle.to_raw()),
                        "VK_KHR_swapchain" => loader.load_khr_swapchain(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_variable_pointers" => loader.load_khr_variable_pointers(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_wayland_surface" => loader.load_khr_wayland_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_win32_keyed_mutex" => loader.load_khr_win32_keyed_mutex(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_win32_surface" => loader.load_khr_win32_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_xcb_surface" => loader.load_khr_xcb_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHR_xlib_surface" => loader.load_khr_xlib_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_acquire_xlib_display" => loader.load_ext_acquire_xlib_display(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_blend_operation_advanced" => loader.load_ext_blend_operation_advanced(handle.to_raw()),
                        "VK_EXT_debug_marker" => loader.load_ext_debug_marker(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_debug_report" => loader.load_ext_debug_report(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_depth_range_unrestricted" => loader.load_ext_depth_range_unrestricted(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_direct_mode_display" => loader.load_ext_direct_mode_display(handle.to_raw()),
                        "VK_EXT_discard_rectangles" => loader.load_ext_discard_rectangles(handle.to_raw()),
                        "VK_EXT_display_control" => loader.load_ext_display_control(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_display_surface_counter" => loader.load_ext_display_surface_counter(handle.to_raw()),
                        "VK_EXT_hdr_metadata" => loader.load_ext_hdr_metadata(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_post_depth_coverage" => loader.load_ext_post_depth_coverage(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_sample_locations" => loader.load_ext_sample_locations(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_sampler_filter_minmax" => loader.load_ext_sampler_filter_minmax(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_stencil_export" => loader.load_ext_shader_stencil_export(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_subgroup_ballot" => loader.load_ext_shader_subgroup_ballot(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_subgroup_vote" => loader.load_ext_shader_subgroup_vote(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_shader_viewport_index_layer" => loader.load_ext_shader_viewport_index_layer(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_swapchain_colorspace" => loader.load_ext_swapchain_colorspace(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_validation_cache" => loader.load_ext_validation_cache(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_EXT_validation_flags" => loader.load_ext_validation_flags(handle.to_raw()),
                        "VK_AMD_draw_indirect_count" => loader.load_amd_draw_indirect_count(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_gcn_shader" => loader.load_amd_gcn_shader(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_gpu_shader_half_float" => loader.load_amd_gpu_shader_half_float(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_gpu_shader_int16" => loader.load_amd_gpu_shader_int16(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_mixed_attachment_samples" => loader.load_amd_mixed_attachment_samples(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_negative_viewport_height" => loader.load_amd_negative_viewport_height(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_rasterization_order" => loader.load_amd_rasterization_order(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_ballot" => loader.load_amd_shader_ballot(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_explicit_vertex_parameter" => loader.load_amd_shader_explicit_vertex_parameter(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_fragment_mask" => loader.load_amd_shader_fragment_mask(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_image_load_store_lod" => loader.load_amd_shader_image_load_store_lod(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_shader_trinary_minmax" => loader.load_amd_shader_trinary_minmax(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_AMD_texture_gather_bias_lod" => loader.load_amd_texture_gather_bias_lod(handle.to_raw()),
                        "VK_GOOGLE_display_timing" => loader.load_google_display_timing(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_IMG_filter_cubic" => loader.load_img_filter_cubic(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_IMG_format_pvrtc" => loader.load_img_format_pvrtc(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHX_device_group" => loader.load_khx_device_group(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHX_device_group_creation" => loader.load_khx_device_group_creation(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_KHX_multiview" => loader.load_khx_multiview(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_MVK_ios_surface" => loader.load_mvk_ios_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_MVK_macos_surface" => loader.load_mvk_macos_surface(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NN_vi_surface" => loader.load_nn_vi_surface(handle.to_raw()),
                        "VK_NV_clip_space_w_scaling" => loader.load_nv_clip_space_w_scaling(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_dedicated_allocation" => loader.load_nv_dedicated_allocation(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_external_memory" => loader.load_nv_external_memory(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_external_memory_capabilities" => loader.load_nv_external_memory_capabilities(handle.to_raw()),
                        "VK_NV_external_memory_win32" => loader.load_nv_external_memory_win32(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_fill_rectangle" => loader.load_nv_fill_rectangle(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_fragment_coverage_to_color" => loader.load_nv_fragment_coverage_to_color(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_framebuffer_mixed_samples" => loader.load_nv_framebuffer_mixed_samples(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_geometry_shader_passthrough" => loader.load_nv_geometry_shader_passthrough(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_glsl_shader" => loader.load_nv_glsl_shader(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_sample_mask_override_coverage" => loader.load_nv_sample_mask_override_coverage(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_viewport_array2" => loader.load_nv_viewport_array2(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_viewport_swizzle" => loader.load_nv_viewport_swizzle(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NV_win32_keyed_mutex" => loader.load_nv_win32_keyed_mutex(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NVX_device_generated_commands" => loader.load_nvx_device_generated_commands(handle.to_raw()),
                        #[cfg(feature = "unimplemented")]
                        "VK_NVX_multiview_per_view_attributes" => loader.load_nvx_multiview_per_view_attributes(handle.to_raw()),
                        &_ => (),
                    }
                }
            }
        }

        let instance = physical_device.instance().clone();
        let mut queue_family_indices = SmallVec::<[u32; 16]>::new();
        // for i in 0..(self.create_info.queueCreateInfoCount as isize) {
        //     unsafe {
        //         let queue_create_info_ptr = self.create_info.pQueueCreateInfos.offset(i);
        //         queue_family_indices.push((*queue_create_info_ptr).queueFamilyIndex);
        //     }
        // }

        for queue_create_info in self.create_info.queue_create_infos() {
            queue_family_indices.push(queue_create_info.queue_family_index())
        }
        assert!(queue_family_indices.len() == 1, "Update this shitty queue family code.");

        Ok(Device {
            inner: Arc::new(Inner {
                // handle: DeviceHandle(handle),
                handle,
                physical_device,
                // features,
                queue_family_indices: queue_family_indices,
                instance,
                loader,
            }),
        })
    }
}
