use std::sync::Arc;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::marker::PhantomData;
use libc::c_char;
use smallvec::SmallVec;
use vks;
use ::{VooResult, Instance, PhysicalDevice, SwapchainSupportDetails,
    DeviceQueueCreateInfo, CharStrs, PhysicalDeviceFeatures, PRINT, Handle};
use queue::{self, Queue};
use instance;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DeviceHandle(pub(crate) vks::VkDevice);

impl DeviceHandle {
    #[inline(always)]
    pub fn raw(&self) -> vks::VkDevice {
        self.0
    }
}

impl Handle for DeviceHandle {
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
    // vk: vks::VkDevicePointers,
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
    pub fn queue(&self, queue_idx: u32) -> vks::VkQueue {
        let mut queue_handle = ptr::null_mut();
        assert!(self.inner.queue_family_indices.len() == 1,
            "Update this shitty queue family code.");
        unsafe {
            self.proc_addr_loader().core.vkGetDeviceQueue(self.inner.handle.0,
                self.inner.queue_family_indices[0], queue_idx,
                &mut queue_handle);
        }
        queue_handle
    }

    #[inline]
    pub fn proc_addr_loader(&self) -> &vks::DeviceProcAddrLoader {
        // &self.inner.vk
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

    /// Returns the memory type index on this device matching the provided
    /// type filter and properties.
    //
    // [HELPER]
    pub fn memory_type_index(&self, type_filter: u32, properties: ::MemoryPropertyFlags)
            -> u32 {
        let mem_props = self.physical_device().memory_properties();

        for i in 0..mem_props.memory_type_count() {
            if (type_filter & (1 << i)) != 0 &&
                (mem_props.memory_types()[i as usize].property_flags() & properties) == properties
            {
                return i;
            }
        }
        panic!("failed to find suitable memory type index with: type_filter: '{}', properties: '{:?}'",
            type_filter, properties);
    }

    /// Updates descriptor sets.
    pub fn update_descriptor_sets(&self, descriptor_writes: Option<&[::WriteDescriptorSet]>,
            descriptor_copies: Option<&[::CopyDescriptorSet]>) {
        let (descriptor_writes_len, descriptor_writes_ptr) = match descriptor_writes {
            Some(ref dws) => (dws.len() as u32, dws.as_ptr()),
            None => (0, ptr::null()),
        };

        let (descriptor_copies_len, descriptor_copies_ptr) = match descriptor_copies {
            Some(ref dcs) => (dcs.len() as u32, dcs.as_ptr()),
            None => (0, ptr::null()),
        };

        unsafe {
            self.proc_addr_loader().vkUpdateDescriptorSets(self.handle().0,
                descriptor_writes_len, descriptor_writes_ptr as *const vks::VkWriteDescriptorSet,
                descriptor_copies_len, descriptor_copies_ptr as *const vks::VkCopyDescriptorSet);
        }
    }

                // *PFN_vkGetDeviceQueue)(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue);


                // *PFN_vkGetDeviceQueue)(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue);


                // *PFN_vkQueueSubmit)(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence);


                // *PFN_vkQueueWaitIdle)(VkQueue queue);


                // *PFN_vkDeviceWaitIdle)(VkDevice device);


                // *PFN_vkAllocateMemory)(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const VkAllocationCallbacks* pAllocator, VkDeviceMemory* pMemory);


                // *PFN_vkFreeMemory)(VkDevice device, VkDeviceMemory memory, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkMapMemory)(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData);


                // *PFN_vkUnmapMemory)(VkDevice device, VkDeviceMemory memory);


                // *PFN_vkFlushMappedMemoryRanges)(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);


                // *PFN_vkInvalidateMappedMemoryRanges)(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);


                // *PFN_vkGetDeviceMemoryCommitment)(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes);


                // *PFN_vkBindBufferMemory)(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset);


                // *PFN_vkBindImageMemory)(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset);


                // *PFN_vkGetBufferMemoryRequirements)(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements);


                // *PFN_vkGetImageMemoryRequirements)(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements);


                // *PFN_vkGetImageSparseMemoryRequirements)(VkDevice device, VkImage image, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements);


                // *PFN_vkQueueBindSparse)(VkQueue queue, uint32_t bindInfoCount, const VkBindSparseInfo* pBindInfo, VkFence fence);


                // *PFN_vkCreateFence)(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);


                // *PFN_vkDestroyFence)(VkDevice device, VkFence fence, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkResetFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences);


                // *PFN_vkGetFenceStatus)(VkDevice device, VkFence fence);


                // *PFN_vkWaitForFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout);


                // *PFN_vkCreateSemaphore)(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSemaphore* pSemaphore);


                // *PFN_vkDestroySemaphore)(VkDevice device, VkSemaphore semaphore, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateEvent)(VkDevice device, const VkEventCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkEvent* pEvent);


                // *PFN_vkDestroyEvent)(VkDevice device, VkEvent event, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkGetEventStatus)(VkDevice device, VkEvent event);


                // *PFN_vkSetEvent)(VkDevice device, VkEvent event);


                // *PFN_vkResetEvent)(VkDevice device, VkEvent event);


                // *PFN_vkCreateQueryPool)(VkDevice device, const VkQueryPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkQueryPool* pQueryPool);


                // *PFN_vkDestroyQueryPool)(VkDevice device, VkQueryPool queryPool, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkGetQueryPoolResults)(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, size_t dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags);


                // *PFN_vkCreateBuffer)(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBuffer* pBuffer);


                // *PFN_vkDestroyBuffer)(VkDevice device, VkBuffer buffer, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateBufferView)(VkDevice device, const VkBufferViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBufferView* pView);


                // *PFN_vkDestroyBufferView)(VkDevice device, VkBufferView bufferView, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateImage)(VkDevice device, const VkImageCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImage* pImage);


                // *PFN_vkDestroyImage)(VkDevice device, VkImage image, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkGetImageSubresourceLayout)(VkDevice device, VkImage image, const VkImageSubresource* pSubresource, VkSubresourceLayout* pLayout);


                // *PFN_vkCreateImageView)(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView);


                // *PFN_vkDestroyImageView)(VkDevice device, VkImageView imageView, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateShaderModule)(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule);


                // *PFN_vkDestroyShaderModule)(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreatePipelineCache)(VkDevice device, const VkPipelineCacheCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineCache* pPipelineCache);


                // *PFN_vkDestroyPipelineCache)(VkDevice device, VkPipelineCache pipelineCache, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkGetPipelineCacheData)(VkDevice device, VkPipelineCache pipelineCache, size_t* pDataSize, void* pData);


                // *PFN_vkMergePipelineCaches)(VkDevice device, VkPipelineCache dstCache, uint32_t srcCacheCount, const VkPipelineCache* pSrcCaches);


                // *PFN_vkCreateGraphicsPipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);


                // *PFN_vkCreateComputePipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkComputePipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);


                // *PFN_vkDestroyPipeline)(VkDevice device, VkPipeline pipeline, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreatePipelineLayout)(VkDevice device, const VkPipelineLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineLayout* pPipelineLayout);


                // *PFN_vkDestroyPipelineLayout)(VkDevice device, VkPipelineLayout pipelineLayout, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateSampler)(VkDevice device, const VkSamplerCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSampler* pSampler);


                // *PFN_vkDestroySampler)(VkDevice device, VkSampler sampler, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateDescriptorSetLayout)(VkDevice device, const VkDescriptorSetLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorSetLayout* pSetLayout);


                // *PFN_vkDestroyDescriptorSetLayout)(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateDescriptorPool)(VkDevice device, const VkDescriptorPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorPool* pDescriptorPool);


                // *PFN_vkDestroyDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkResetDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags);


                // *PFN_vkAllocateDescriptorSets)(VkDevice device, const VkDescriptorSetAllocateInfo* pAllocateInfo, VkDescriptorSet* pDescriptorSets);


                // *PFN_vkFreeDescriptorSets)(VkDevice device, VkDescriptorPool descriptorPool, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets);


                // *PFN_vkUpdateDescriptorSets)(VkDevice device, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites, uint32_t descriptorCopyCount, const VkCopyDescriptorSet* pDescriptorCopies);


                // *PFN_vkCreateFramebuffer)(VkDevice device, const VkFramebufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFramebuffer* pFramebuffer);


                // *PFN_vkDestroyFramebuffer)(VkDevice device, VkFramebuffer framebuffer, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateRenderPass)(VkDevice device, const VkRenderPassCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkRenderPass* pRenderPass);


                // *PFN_vkDestroyRenderPass)(VkDevice device, VkRenderPass renderPass, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkGetRenderAreaGranularity)(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity);


                // *PFN_vkCreateCommandPool)(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool);


                // *PFN_vkDestroyCommandPool)(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkResetCommandPool)(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags);


                // *PFN_vkAllocateCommandBuffers)(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers);


                // *PFN_vkFreeCommandBuffers)(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);


                // *PFN_vkBeginCommandBuffer)(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo);


                // *PFN_vkEndCommandBuffer)(VkCommandBuffer commandBuffer);


                // *PFN_vkResetCommandBuffer)(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags);


                // *PFN_vkCmdBindPipeline)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);


                // *PFN_vkCmdSetViewport)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewport* pViewports);


                // *PFN_vkCmdSetScissor)(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, const VkRect2D* pScissors);


                // *PFN_vkCmdSetLineWidth)(VkCommandBuffer commandBuffer, float lineWidth);


                // *PFN_vkCmdSetDepthBias)(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor);


                // *PFN_vkCmdSetBlendConstants)(VkCommandBuffer commandBuffer, const float blendConstants[4]);


                // *PFN_vkCmdSetDepthBounds)(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds);


                // *PFN_vkCmdSetStencilCompareMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t compareMask);


                // *PFN_vkCmdSetStencilWriteMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t writeMask);


                // *PFN_vkCmdSetStencilReference)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t reference);


                // *PFN_vkCmdBindDescriptorSets)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets, uint32_t dynamicOffsetCount, const uint32_t* pDynamicOffsets);


                // *PFN_vkCmdBindIndexBuffer)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType);


                // *PFN_vkCmdBindVertexBuffers)(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets);


                // *PFN_vkCmdDraw)(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance);


                // *PFN_vkCmdDrawIndexed)(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance);


                // *PFN_vkCmdDrawIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride);


                // *PFN_vkCmdDrawIndexedIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride);


                // *PFN_vkCmdDispatch)(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);


                // *PFN_vkCmdDispatchIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset);


                // *PFN_vkCmdCopyBuffer)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferCopy* pRegions);


                // *PFN_vkCmdCopyImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageCopy* pRegions);


                // *PFN_vkCmdBlitImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageBlit* pRegions, VkFilter filter);


                // *PFN_vkCmdCopyBufferToImage)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkBufferImageCopy* pRegions);


                // *PFN_vkCmdCopyImageToBuffer)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferImageCopy* pRegions);


                // *PFN_vkCmdUpdateBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, const void* pData);


                // *PFN_vkCmdFillBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, uint32_t data);


                // *PFN_vkCmdClearColorImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearColorValue* pColor, uint32_t rangeCount, const VkImageSubresourceRange* pRanges);


                // *PFN_vkCmdClearDepthStencilImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearDepthStencilValue* pDepthStencil, uint32_t rangeCount, const VkImageSubresourceRange* pRanges);


                // *PFN_vkCmdClearAttachments)(VkCommandBuffer commandBuffer, uint32_t attachmentCount, const VkClearAttachment* pAttachments, uint32_t rectCount, const VkClearRect* pRects);


                // *PFN_vkCmdResolveImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageResolve* pRegions);


                // *PFN_vkCmdSetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);


                // *PFN_vkCmdResetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);


                // *PFN_vkCmdWaitEvents)(VkCommandBuffer commandBuffer, uint32_t eventCount, const VkEvent* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);


                // *PFN_vkCmdPipelineBarrier)(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);


                // *PFN_vkCmdBeginQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags);


                // *PFN_vkCmdEndQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query);


                // *PFN_vkCmdResetQueryPool)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount);


                // *PFN_vkCmdWriteTimestamp)(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, uint32_t query);


                // *PFN_vkCmdCopyQueryPoolResults)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags);


                // *PFN_vkCmdPushConstants)(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, uint32_t offset, uint32_t size, const void* pValues);


                // *PFN_vkCmdBeginRenderPass)(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents);


                // *PFN_vkCmdNextSubpass)(VkCommandBuffer commandBuffer, VkSubpassContents contents);


                // *PFN_vkCmdEndRenderPass)(VkCommandBuffer commandBuffer);


                // *PFN_vkCmdExecuteCommands)(VkCommandBuffer commandBuffer, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);


                // *PFN_vkCreateSwapchainKHR)(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain);


                // *PFN_vkDestroySwapchainKHR)(VkDevice device, VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkGetSwapchainImagesKHR)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages);


                // *PFN_vkAcquireNextImageKHR)(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex);


                // *PFN_vkQueuePresentKHR)(VkQueue queue, const VkPresentInfoKHR* pPresentInfo);


                // *PFN_vkCreateSharedSwapchainsKHR)(VkDevice device, uint32_t swapchainCount, const VkSwapchainCreateInfoKHR* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchains);


                // *PFN_vkTrimCommandPoolKHR)(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlagsKHR flags);


                // *PFN_vkGetMemoryWin32HandleKHR)(VkDevice device, const VkMemoryGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);


                // *PFN_vkGetMemoryWin32HandlePropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, HANDLE handle, VkMemoryWin32HandlePropertiesKHR* pMemoryWin32HandleProperties);


                // *PFN_vkGetMemoryFdKHR)(VkDevice device, const VkMemoryGetFdInfoKHR* pGetFdInfo, int* pFd);


                // *PFN_vkGetMemoryFdPropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, int fd, VkMemoryFdPropertiesKHR* pMemoryFdProperties);


                // *PFN_vkImportSemaphoreWin32HandleKHR)(VkDevice device, const VkImportSemaphoreWin32HandleInfoKHR* pImportSemaphoreWin32HandleInfo);


                // *PFN_vkGetSemaphoreWin32HandleKHR)(VkDevice device, const VkSemaphoreGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);


                // *PFN_vkImportSemaphoreFdKHR)(VkDevice device, const VkImportSemaphoreFdInfoKHR* pImportSemaphoreFdInfo);


                // *PFN_vkGetSemaphoreFdKHR)(VkDevice device, const VkSemaphoreGetFdInfoKHR* pGetFdInfo, int* pFd);


                // *PFN_vkCmdPushDescriptorSetKHR)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites);


                // *PFN_vkCreateDescriptorUpdateTemplateKHR)(VkDevice device, const VkDescriptorUpdateTemplateCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorUpdateTemplateKHR* pDescriptorUpdateTemplate);


                // *PFN_vkDestroyDescriptorUpdateTemplateKHR)(VkDevice device, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkUpdateDescriptorSetWithTemplateKHR)(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const void* pData);


                // *PFN_vkCmdPushDescriptorSetWithTemplateKHR)(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, const void* pData);


                // *PFN_vkGetSwapchainStatusKHR)(VkDevice device, VkSwapchainKHR swapchain);


                // *PFN_vkImportFenceWin32HandleKHR)(VkDevice device, const VkImportFenceWin32HandleInfoKHR* pImportFenceWin32HandleInfo);


                // *PFN_vkGetFenceWin32HandleKHR)(VkDevice device, const VkFenceGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);


                // *PFN_vkImportFenceFdKHR)(VkDevice device, const VkImportFenceFdInfoKHR* pImportFenceFdInfo);


                // *PFN_vkGetFenceFdKHR)(VkDevice device, const VkFenceGetFdInfoKHR* pGetFdInfo, int* pFd);


                // *PFN_vkGetImageMemoryRequirements2KHR)(VkDevice device, const VkImageMemoryRequirementsInfo2KHR* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements);


                // *PFN_vkGetBufferMemoryRequirements2KHR)(VkDevice device, const VkBufferMemoryRequirementsInfo2KHR* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements);


                // *PFN_vkGetImageSparseMemoryRequirements2KHR)(VkDevice device, const VkImageSparseMemoryRequirementsInfo2KHR* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2KHR* pSparseMemoryRequirements);


                // *PFN_vkCreateSamplerYcbcrConversionKHR)(VkDevice device, const VkSamplerYcbcrConversionCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSamplerYcbcrConversionKHR* pYcbcrConversion);


                // *PFN_vkDestroySamplerYcbcrConversionKHR)(VkDevice device, VkSamplerYcbcrConversionKHR ycbcrConversion, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkBindBufferMemory2KHR)(VkDevice device, uint32_t bindInfoCount, const VkBindBufferMemoryInfoKHR* pBindInfos);


                // *PFN_vkBindImageMemory2KHR)(VkDevice device, uint32_t bindInfoCount, const VkBindImageMemoryInfoKHR* pBindInfos);


                // *PFN_vkDebugMarkerSetObjectTagEXT)(VkDevice device, const VkDebugMarkerObjectTagInfoEXT* pTagInfo);


                // *PFN_vkDebugMarkerSetObjectNameEXT)(VkDevice device, const VkDebugMarkerObjectNameInfoEXT* pNameInfo);


                // *PFN_vkCmdDebugMarkerBeginEXT)(VkCommandBuffer commandBuffer, const VkDebugMarkerMarkerInfoEXT* pMarkerInfo);


                // *PFN_vkCmdDebugMarkerEndEXT)(VkCommandBuffer commandBuffer);


                // *PFN_vkCmdDebugMarkerInsertEXT)(VkCommandBuffer commandBuffer, const VkDebugMarkerMarkerInfoEXT* pMarkerInfo);


                // *PFN_vkCmdDrawIndirectCountAMD)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride);


                // *PFN_vkCmdDrawIndexedIndirectCountAMD)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride);


                // *PFN_vkGetMemoryWin32HandleNV)(VkDevice device, VkDeviceMemory memory, VkExternalMemoryHandleTypeFlagsNV handleType, HANDLE* pHandle);


                // *PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX)(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlagsKHX* pPeerMemoryFeatures);


                // *PFN_vkCmdSetDeviceMaskKHX)(VkCommandBuffer commandBuffer, uint32_t deviceMask);


                // *PFN_vkCmdDispatchBaseKHX)(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);


                // *PFN_vkGetDeviceGroupPresentCapabilitiesKHX)(VkDevice device, VkDeviceGroupPresentCapabilitiesKHX* pDeviceGroupPresentCapabilities);


                // *PFN_vkGetDeviceGroupSurfacePresentModesKHX)(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHX* pModes);


                // *PFN_vkAcquireNextImage2KHX)(VkDevice device, const VkAcquireNextImageInfoKHX* pAcquireInfo, uint32_t* pImageIndex);


                // *PFN_vkCmdProcessCommandsNVX)(VkCommandBuffer commandBuffer, const VkCmdProcessCommandsInfoNVX* pProcessCommandsInfo);


                // *PFN_vkCmdReserveSpaceForCommandsNVX)(VkCommandBuffer commandBuffer, const VkCmdReserveSpaceForCommandsInfoNVX* pReserveSpaceInfo);


                // *PFN_vkCreateIndirectCommandsLayoutNVX)(VkDevice device, const VkIndirectCommandsLayoutCreateInfoNVX* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkIndirectCommandsLayoutNVX* pIndirectCommandsLayout);


                // *PFN_vkDestroyIndirectCommandsLayoutNVX)(VkDevice device, VkIndirectCommandsLayoutNVX indirectCommandsLayout, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkCreateObjectTableNVX)(VkDevice device, const VkObjectTableCreateInfoNVX* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkObjectTableNVX* pObjectTable);


                // *PFN_vkDestroyObjectTableNVX)(VkDevice device, VkObjectTableNVX objectTable, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkRegisterObjectsNVX)(VkDevice device, VkObjectTableNVX objectTable, uint32_t objectCount, const VkObjectTableEntryNVX* const*    ppObjectTableEntries, const uint32_t* pObjectIndices);


                // *PFN_vkUnregisterObjectsNVX)(VkDevice device, VkObjectTableNVX objectTable, uint32_t objectCount, const VkObjectEntryTypeNVX* pObjectEntryTypes, const uint32_t* pObjectIndices);


                // *PFN_vkCmdSetViewportWScalingNV)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewportWScalingNV* pViewportWScalings);


                // *PFN_vkDisplayPowerControlEXT)(VkDevice device, VkDisplayKHR display, const VkDisplayPowerInfoEXT* pDisplayPowerInfo);


                // *PFN_vkRegisterDeviceEventEXT)(VkDevice device, const VkDeviceEventInfoEXT* pDeviceEventInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);


                // *PFN_vkRegisterDisplayEventEXT)(VkDevice device, VkDisplayKHR display, const VkDisplayEventInfoEXT* pDisplayEventInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);


                // *PFN_vkGetSwapchainCounterEXT)(VkDevice device, VkSwapchainKHR swapchain, VkSurfaceCounterFlagBitsEXT counter, uint64_t* pCounterValue);


                // *PFN_vkGetRefreshCycleDurationGOOGLE)(VkDevice device, VkSwapchainKHR swapchain, VkRefreshCycleDurationGOOGLE* pDisplayTimingProperties);


                // *PFN_vkGetPastPresentationTimingGOOGLE)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pPresentationTimingCount, VkPastPresentationTimingGOOGLE* pPresentationTimings);


                // *PFN_vkCmdSetDiscardRectangleEXT)(VkCommandBuffer commandBuffer, uint32_t firstDiscardRectangle, uint32_t discardRectangleCount, const VkRect2D* pDiscardRectangles);


                // *PFN_vkSetHdrMetadataEXT)(VkDevice device, uint32_t swapchainCount, const VkSwapchainKHR* pSwapchains, const VkHdrMetadataEXT* pMetadata);


                // *PFN_vkCmdSetSampleLocationsEXT)(VkCommandBuffer commandBuffer, const VkSampleLocationsInfoEXT* pSampleLocationsInfo);


                // *PFN_vkCreateValidationCacheEXT)(VkDevice device, const VkValidationCacheCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkValidationCacheEXT* pValidationCache);


                // *PFN_vkDestroyValidationCacheEXT)(VkDevice device, VkValidationCacheEXT validationCache, const VkAllocationCallbacks* pAllocator);


                // *PFN_vkMergeValidationCachesEXT)(VkDevice device, VkValidationCacheEXT dstCache, uint32_t srcCacheCount, const VkValidationCacheEXT* pSrcCaches);


                // *PFN_vkGetValidationCacheDataEXT)(VkDevice device, VkValidationCacheEXT validationCache, size_t* pDataSize, void* pData);


}

impl<'h> Handle for &'h Device {
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
            self.instance.proc_addr_loader().core.vkDestroyDevice(self.handle.0, ptr::null());
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}


// typedef struct VkDeviceCreateInfo {
//     VkStructureType                    sType;
//     const void*                        pNext;
//     VkDeviceCreateFlags                flags;
//     uint32_t                           queueCreateInfoCount;
//     const VkDeviceQueueCreateInfo*     pQueueCreateInfos;
//     uint32_t                           enabledLayerCount;
//     const char* const*                 ppEnabledLayerNames;
//     uint32_t                           enabledExtensionCount;
//     const char* const*                 ppEnabledExtensionNames;
//     const VkPhysicalDeviceFeatures*    pEnabledFeatures;
// } VkDeviceCreateInfo;
//
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
    pub fn build(&self, physical_device: PhysicalDevice) -> VooResult<Device> {
                // Device:
        let mut handle = ptr::null_mut();
        unsafe {
            ::check(physical_device.instance().proc_addr_loader().core.vkCreateDevice(physical_device.handle().0,
                self.create_info.as_raw(), ptr::null(), &mut handle));
        }

        let mut loader = vks::DeviceProcAddrLoader::from_get_device_proc_addr(
            physical_device.instance().proc_addr_loader().core.pfn_vkGetDeviceProcAddr);

        unsafe {
            loader.load_core(handle);
            // create_info.enabled_extensions.load_device(&mut loader, handle);
            // instance.loader().get_enabled_extensions().load_device(&mut loader, handle);
            // loader.load_khr_sampler_mirror_clamp_to_edge(handle);
            // loader.load_khr_draw_parameters(handle);
            loader.load_khr_swapchain(handle);
            // loader.load_khr_maintenance1(handle);
            // loader.load_amd_rasterization_order(handle);
            // loader.load_amd_draw_indirect_count(handle);
            // loader.load_amd_shader_ballot(handle);
            // loader.load_amd_shader_trinary_minmax(handle);
            // loader.load_amd_shader_explicit_vertex_parameter(handle);
            // loader.load_amd_gcn_shader(handle);
            // loader.load_amd_draw_indirect_count(handle);
            // loader.load_amd_negative_viewport_height(handle);
            // loader.load_amd_shader_info(handle);
            // loader.load_amd_wave_limits(handle);
            // loader.load_amd_texture_gather_bias_lod(handle);
            // loader.load_amd_programmable_sample_locations(handle);
            // loader.load_amd_mixed_attachment_samples(handle);
            // loader.load_ext_shader_subgroup_vote(handle);
            // loader.load_amd_gpa_interface(handle);
            // loader.load_ext_shader_subgroup_ballot(handle);
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
                handle: DeviceHandle(handle),
                physical_device,
                // features,
                queue_family_indices: queue_family_indices,
                instance,
                loader,
            }),
        })
    }
}