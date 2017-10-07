*PFN_vkGetDeviceQueue)(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue);


*PFN_vkGetDeviceQueue)(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue);


*PFN_vkQueueSubmit)(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence);


*PFN_vkQueueWaitIdle)(VkQueue queue);


*PFN_vkDeviceWaitIdle)(VkDevice device);


*PFN_vkAllocateMemory)(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const VkAllocationCallbacks* pAllocator, VkDeviceMemory* pMemory);


*PFN_vkFreeMemory)(VkDevice device, VkDeviceMemory memory, const VkAllocationCallbacks* pAllocator);


*PFN_vkMapMemory)(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData);


*PFN_vkUnmapMemory)(VkDevice device, VkDeviceMemory memory);


*PFN_vkFlushMappedMemoryRanges)(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);


*PFN_vkInvalidateMappedMemoryRanges)(VkDevice device, uint32_t memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);


*PFN_vkGetDeviceMemoryCommitment)(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes);


*PFN_vkBindBufferMemory)(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset);


*PFN_vkBindImageMemory)(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset);


*PFN_vkGetBufferMemoryRequirements)(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements);


*PFN_vkGetImageMemoryRequirements)(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements);


*PFN_vkGetImageSparseMemoryRequirements)(VkDevice device, VkImage image, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements);


*PFN_vkQueueBindSparse)(VkQueue queue, uint32_t bindInfoCount, const VkBindSparseInfo* pBindInfo, VkFence fence);


*PFN_vkCreateFence)(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);


*PFN_vkDestroyFence)(VkDevice device, VkFence fence, const VkAllocationCallbacks* pAllocator);


*PFN_vkResetFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences);


*PFN_vkGetFenceStatus)(VkDevice device, VkFence fence);


*PFN_vkWaitForFences)(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout);


*PFN_vkCreateSemaphore)(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSemaphore* pSemaphore);


*PFN_vkDestroySemaphore)(VkDevice device, VkSemaphore semaphore, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateEvent)(VkDevice device, const VkEventCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkEvent* pEvent);


*PFN_vkDestroyEvent)(VkDevice device, VkEvent event, const VkAllocationCallbacks* pAllocator);


*PFN_vkGetEventStatus)(VkDevice device, VkEvent event);


*PFN_vkSetEvent)(VkDevice device, VkEvent event);


*PFN_vkResetEvent)(VkDevice device, VkEvent event);


*PFN_vkCreateQueryPool)(VkDevice device, const VkQueryPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkQueryPool* pQueryPool);


*PFN_vkDestroyQueryPool)(VkDevice device, VkQueryPool queryPool, const VkAllocationCallbacks* pAllocator);


*PFN_vkGetQueryPoolResults)(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, size_t dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags);


*PFN_vkCreateBuffer)(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBuffer* pBuffer);


*PFN_vkDestroyBuffer)(VkDevice device, VkBuffer buffer, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateBufferView)(VkDevice device, const VkBufferViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBufferView* pView);


*PFN_vkDestroyBufferView)(VkDevice device, VkBufferView bufferView, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateImage)(VkDevice device, const VkImageCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImage* pImage);


*PFN_vkDestroyImage)(VkDevice device, VkImage image, const VkAllocationCallbacks* pAllocator);


*PFN_vkGetImageSubresourceLayout)(VkDevice device, VkImage image, const VkImageSubresource* pSubresource, VkSubresourceLayout* pLayout);


*PFN_vkCreateImageView)(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView);


*PFN_vkDestroyImageView)(VkDevice device, VkImageView imageView, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateShaderModule)(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule);


*PFN_vkDestroyShaderModule)(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreatePipelineCache)(VkDevice device, const VkPipelineCacheCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineCache* pPipelineCache);


*PFN_vkDestroyPipelineCache)(VkDevice device, VkPipelineCache pipelineCache, const VkAllocationCallbacks* pAllocator);


*PFN_vkGetPipelineCacheData)(VkDevice device, VkPipelineCache pipelineCache, size_t* pDataSize, void* pData);


*PFN_vkMergePipelineCaches)(VkDevice device, VkPipelineCache dstCache, uint32_t srcCacheCount, const VkPipelineCache* pSrcCaches);


*PFN_vkCreateGraphicsPipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);


*PFN_vkCreateComputePipelines)(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, const VkComputePipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);


*PFN_vkDestroyPipeline)(VkDevice device, VkPipeline pipeline, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreatePipelineLayout)(VkDevice device, const VkPipelineLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineLayout* pPipelineLayout);


*PFN_vkDestroyPipelineLayout)(VkDevice device, VkPipelineLayout pipelineLayout, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateSampler)(VkDevice device, const VkSamplerCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSampler* pSampler);


*PFN_vkDestroySampler)(VkDevice device, VkSampler sampler, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateDescriptorSetLayout)(VkDevice device, const VkDescriptorSetLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorSetLayout* pSetLayout);


*PFN_vkDestroyDescriptorSetLayout)(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateDescriptorPool)(VkDevice device, const VkDescriptorPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorPool* pDescriptorPool);


*PFN_vkDestroyDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, const VkAllocationCallbacks* pAllocator);


*PFN_vkResetDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags);


*PFN_vkAllocateDescriptorSets)(VkDevice device, const VkDescriptorSetAllocateInfo* pAllocateInfo, VkDescriptorSet* pDescriptorSets);


*PFN_vkFreeDescriptorSets)(VkDevice device, VkDescriptorPool descriptorPool, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets);


*PFN_vkUpdateDescriptorSets)(VkDevice device, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites, uint32_t descriptorCopyCount, const VkCopyDescriptorSet* pDescriptorCopies);


*PFN_vkCreateFramebuffer)(VkDevice device, const VkFramebufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFramebuffer* pFramebuffer);


*PFN_vkDestroyFramebuffer)(VkDevice device, VkFramebuffer framebuffer, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateRenderPass)(VkDevice device, const VkRenderPassCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkRenderPass* pRenderPass);


*PFN_vkDestroyRenderPass)(VkDevice device, VkRenderPass renderPass, const VkAllocationCallbacks* pAllocator);


*PFN_vkGetRenderAreaGranularity)(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity);


*PFN_vkCreateCommandPool)(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool);


*PFN_vkDestroyCommandPool)(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator);


*PFN_vkResetCommandPool)(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags);


*PFN_vkAllocateCommandBuffers)(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers);


*PFN_vkFreeCommandBuffers)(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);


*PFN_vkBeginCommandBuffer)(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo);


*PFN_vkEndCommandBuffer)(VkCommandBuffer commandBuffer);


*PFN_vkResetCommandBuffer)(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags);


*PFN_vkCmdBindPipeline)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);


*PFN_vkCmdSetViewport)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewport* pViewports);


*PFN_vkCmdSetScissor)(VkCommandBuffer commandBuffer, uint32_t firstScissor, uint32_t scissorCount, const VkRect2D* pScissors);


*PFN_vkCmdSetLineWidth)(VkCommandBuffer commandBuffer, float lineWidth);


*PFN_vkCmdSetDepthBias)(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor);


*PFN_vkCmdSetBlendConstants)(VkCommandBuffer commandBuffer, const float blendConstants[4]);


*PFN_vkCmdSetDepthBounds)(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds);


*PFN_vkCmdSetStencilCompareMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t compareMask);


*PFN_vkCmdSetStencilWriteMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t writeMask);


*PFN_vkCmdSetStencilReference)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, uint32_t reference);


*PFN_vkCmdBindDescriptorSets)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t firstSet, uint32_t descriptorSetCount, const VkDescriptorSet* pDescriptorSets, uint32_t dynamicOffsetCount, const uint32_t* pDynamicOffsets);


*PFN_vkCmdBindIndexBuffer)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType);


*PFN_vkCmdBindVertexBuffers)(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets);


*PFN_vkCmdDraw)(VkCommandBuffer commandBuffer, uint32_t vertexCount, uint32_t instanceCount, uint32_t firstVertex, uint32_t firstInstance);


*PFN_vkCmdDrawIndexed)(VkCommandBuffer commandBuffer, uint32_t indexCount, uint32_t instanceCount, uint32_t firstIndex, int32_t vertexOffset, uint32_t firstInstance);


*PFN_vkCmdDrawIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride);


*PFN_vkCmdDrawIndexedIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride);


*PFN_vkCmdDispatch)(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);


*PFN_vkCmdDispatchIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset);


*PFN_vkCmdCopyBuffer)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferCopy* pRegions);


*PFN_vkCmdCopyImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageCopy* pRegions);


*PFN_vkCmdBlitImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageBlit* pRegions, VkFilter filter);


*PFN_vkCmdCopyBufferToImage)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkBufferImageCopy* pRegions);


*PFN_vkCmdCopyImageToBuffer)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, uint32_t regionCount, const VkBufferImageCopy* pRegions);


*PFN_vkCmdUpdateBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, const void* pData);


*PFN_vkCmdFillBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, uint32_t data);


*PFN_vkCmdClearColorImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearColorValue* pColor, uint32_t rangeCount, const VkImageSubresourceRange* pRanges);


*PFN_vkCmdClearDepthStencilImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearDepthStencilValue* pDepthStencil, uint32_t rangeCount, const VkImageSubresourceRange* pRanges);


*PFN_vkCmdClearAttachments)(VkCommandBuffer commandBuffer, uint32_t attachmentCount, const VkClearAttachment* pAttachments, uint32_t rectCount, const VkClearRect* pRects);


*PFN_vkCmdResolveImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, uint32_t regionCount, const VkImageResolve* pRegions);


*PFN_vkCmdSetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);


*PFN_vkCmdResetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);


*PFN_vkCmdWaitEvents)(VkCommandBuffer commandBuffer, uint32_t eventCount, const VkEvent* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);


*PFN_vkCmdPipelineBarrier)(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);


*PFN_vkCmdBeginQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags);


*PFN_vkCmdEndQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query);


*PFN_vkCmdResetQueryPool)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount);


*PFN_vkCmdWriteTimestamp)(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, uint32_t query);


*PFN_vkCmdCopyQueryPoolResults)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags);


*PFN_vkCmdPushConstants)(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, uint32_t offset, uint32_t size, const void* pValues);


*PFN_vkCmdBeginRenderPass)(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents);


*PFN_vkCmdNextSubpass)(VkCommandBuffer commandBuffer, VkSubpassContents contents);


*PFN_vkCmdEndRenderPass)(VkCommandBuffer commandBuffer);


*PFN_vkCmdExecuteCommands)(VkCommandBuffer commandBuffer, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers);


*PFN_vkCreateSwapchainKHR)(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain);


*PFN_vkDestroySwapchainKHR)(VkDevice device, VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator);


*PFN_vkGetSwapchainImagesKHR)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages);


*PFN_vkAcquireNextImageKHR)(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex);


*PFN_vkQueuePresentKHR)(VkQueue queue, const VkPresentInfoKHR* pPresentInfo);


*PFN_vkCreateSharedSwapchainsKHR)(VkDevice device, uint32_t swapchainCount, const VkSwapchainCreateInfoKHR* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchains);


*PFN_vkTrimCommandPoolKHR)(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlagsKHR flags);


*PFN_vkGetMemoryWin32HandleKHR)(VkDevice device, const VkMemoryGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);


*PFN_vkGetMemoryWin32HandlePropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, HANDLE handle, VkMemoryWin32HandlePropertiesKHR* pMemoryWin32HandleProperties);


*PFN_vkGetMemoryFdKHR)(VkDevice device, const VkMemoryGetFdInfoKHR* pGetFdInfo, int* pFd);


*PFN_vkGetMemoryFdPropertiesKHR)(VkDevice device, VkExternalMemoryHandleTypeFlagBitsKHR handleType, int fd, VkMemoryFdPropertiesKHR* pMemoryFdProperties);


*PFN_vkImportSemaphoreWin32HandleKHR)(VkDevice device, const VkImportSemaphoreWin32HandleInfoKHR* pImportSemaphoreWin32HandleInfo);


*PFN_vkGetSemaphoreWin32HandleKHR)(VkDevice device, const VkSemaphoreGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);


*PFN_vkImportSemaphoreFdKHR)(VkDevice device, const VkImportSemaphoreFdInfoKHR* pImportSemaphoreFdInfo);


*PFN_vkGetSemaphoreFdKHR)(VkDevice device, const VkSemaphoreGetFdInfoKHR* pGetFdInfo, int* pFd);


*PFN_vkCmdPushDescriptorSetKHR)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites);


*PFN_vkCreateDescriptorUpdateTemplateKHR)(VkDevice device, const VkDescriptorUpdateTemplateCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorUpdateTemplateKHR* pDescriptorUpdateTemplate);


*PFN_vkDestroyDescriptorUpdateTemplateKHR)(VkDevice device, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const VkAllocationCallbacks* pAllocator);


*PFN_vkUpdateDescriptorSetWithTemplateKHR)(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, const void* pData);


*PFN_vkCmdPushDescriptorSetWithTemplateKHR)(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplateKHR descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, const void* pData);


*PFN_vkGetSwapchainStatusKHR)(VkDevice device, VkSwapchainKHR swapchain);


*PFN_vkImportFenceWin32HandleKHR)(VkDevice device, const VkImportFenceWin32HandleInfoKHR* pImportFenceWin32HandleInfo);


*PFN_vkGetFenceWin32HandleKHR)(VkDevice device, const VkFenceGetWin32HandleInfoKHR* pGetWin32HandleInfo, HANDLE* pHandle);


*PFN_vkImportFenceFdKHR)(VkDevice device, const VkImportFenceFdInfoKHR* pImportFenceFdInfo);


*PFN_vkGetFenceFdKHR)(VkDevice device, const VkFenceGetFdInfoKHR* pGetFdInfo, int* pFd);


*PFN_vkGetImageMemoryRequirements2KHR)(VkDevice device, const VkImageMemoryRequirementsInfo2KHR* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements);


*PFN_vkGetBufferMemoryRequirements2KHR)(VkDevice device, const VkBufferMemoryRequirementsInfo2KHR* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements);


*PFN_vkGetImageSparseMemoryRequirements2KHR)(VkDevice device, const VkImageSparseMemoryRequirementsInfo2KHR* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2KHR* pSparseMemoryRequirements);


*PFN_vkCreateSamplerYcbcrConversionKHR)(VkDevice device, const VkSamplerYcbcrConversionCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSamplerYcbcrConversionKHR* pYcbcrConversion);


*PFN_vkDestroySamplerYcbcrConversionKHR)(VkDevice device, VkSamplerYcbcrConversionKHR ycbcrConversion, const VkAllocationCallbacks* pAllocator);


*PFN_vkBindBufferMemory2KHR)(VkDevice device, uint32_t bindInfoCount, const VkBindBufferMemoryInfoKHR* pBindInfos);


*PFN_vkBindImageMemory2KHR)(VkDevice device, uint32_t bindInfoCount, const VkBindImageMemoryInfoKHR* pBindInfos);


*PFN_vkDebugMarkerSetObjectTagEXT)(VkDevice device, const VkDebugMarkerObjectTagInfoEXT* pTagInfo);


*PFN_vkDebugMarkerSetObjectNameEXT)(VkDevice device, const VkDebugMarkerObjectNameInfoEXT* pNameInfo);


*PFN_vkCmdDebugMarkerBeginEXT)(VkCommandBuffer commandBuffer, const VkDebugMarkerMarkerInfoEXT* pMarkerInfo);


*PFN_vkCmdDebugMarkerEndEXT)(VkCommandBuffer commandBuffer);


*PFN_vkCmdDebugMarkerInsertEXT)(VkCommandBuffer commandBuffer, const VkDebugMarkerMarkerInfoEXT* pMarkerInfo);


*PFN_vkCmdDrawIndirectCountAMD)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride);


*PFN_vkCmdDrawIndexedIndirectCountAMD)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride);


*PFN_vkGetMemoryWin32HandleNV)(VkDevice device, VkDeviceMemory memory, VkExternalMemoryHandleTypeFlagsNV handleType, HANDLE* pHandle);


*PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX)(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlagsKHX* pPeerMemoryFeatures);


*PFN_vkCmdSetDeviceMaskKHX)(VkCommandBuffer commandBuffer, uint32_t deviceMask);


*PFN_vkCmdDispatchBaseKHX)(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ);


*PFN_vkGetDeviceGroupPresentCapabilitiesKHX)(VkDevice device, VkDeviceGroupPresentCapabilitiesKHX* pDeviceGroupPresentCapabilities);


*PFN_vkGetDeviceGroupSurfacePresentModesKHX)(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHX* pModes);


*PFN_vkAcquireNextImage2KHX)(VkDevice device, const VkAcquireNextImageInfoKHX* pAcquireInfo, uint32_t* pImageIndex);


*PFN_vkCmdProcessCommandsNVX)(VkCommandBuffer commandBuffer, const VkCmdProcessCommandsInfoNVX* pProcessCommandsInfo);


*PFN_vkCmdReserveSpaceForCommandsNVX)(VkCommandBuffer commandBuffer, const VkCmdReserveSpaceForCommandsInfoNVX* pReserveSpaceInfo);


*PFN_vkCreateIndirectCommandsLayoutNVX)(VkDevice device, const VkIndirectCommandsLayoutCreateInfoNVX* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkIndirectCommandsLayoutNVX* pIndirectCommandsLayout);


*PFN_vkDestroyIndirectCommandsLayoutNVX)(VkDevice device, VkIndirectCommandsLayoutNVX indirectCommandsLayout, const VkAllocationCallbacks* pAllocator);


*PFN_vkCreateObjectTableNVX)(VkDevice device, const VkObjectTableCreateInfoNVX* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkObjectTableNVX* pObjectTable);


*PFN_vkDestroyObjectTableNVX)(VkDevice device, VkObjectTableNVX objectTable, const VkAllocationCallbacks* pAllocator);


*PFN_vkRegisterObjectsNVX)(VkDevice device, VkObjectTableNVX objectTable, uint32_t objectCount, const VkObjectTableEntryNVX* const*    ppObjectTableEntries, const uint32_t* pObjectIndices);


*PFN_vkUnregisterObjectsNVX)(VkDevice device, VkObjectTableNVX objectTable, uint32_t objectCount, const VkObjectEntryTypeNVX* pObjectEntryTypes, const uint32_t* pObjectIndices);


*PFN_vkCmdSetViewportWScalingNV)(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, const VkViewportWScalingNV* pViewportWScalings);


*PFN_vkDisplayPowerControlEXT)(VkDevice device, VkDisplayKHR display, const VkDisplayPowerInfoEXT* pDisplayPowerInfo);


*PFN_vkRegisterDeviceEventEXT)(VkDevice device, const VkDeviceEventInfoEXT* pDeviceEventInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);


*PFN_vkRegisterDisplayEventEXT)(VkDevice device, VkDisplayKHR display, const VkDisplayEventInfoEXT* pDisplayEventInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);


*PFN_vkGetSwapchainCounterEXT)(VkDevice device, VkSwapchainKHR swapchain, VkSurfaceCounterFlagBitsEXT counter, uint64_t* pCounterValue);


*PFN_vkGetRefreshCycleDurationGOOGLE)(VkDevice device, VkSwapchainKHR swapchain, VkRefreshCycleDurationGOOGLE* pDisplayTimingProperties);


*PFN_vkGetPastPresentationTimingGOOGLE)(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pPresentationTimingCount, VkPastPresentationTimingGOOGLE* pPresentationTimings);


*PFN_vkCmdSetDiscardRectangleEXT)(VkCommandBuffer commandBuffer, uint32_t firstDiscardRectangle, uint32_t discardRectangleCount, const VkRect2D* pDiscardRectangles);


*PFN_vkSetHdrMetadataEXT)(VkDevice device, uint32_t swapchainCount, const VkSwapchainKHR* pSwapchains, const VkHdrMetadataEXT* pMetadata);


*PFN_vkCmdSetSampleLocationsEXT)(VkCommandBuffer commandBuffer, const VkSampleLocationsInfoEXT* pSampleLocationsInfo);


*PFN_vkCreateValidationCacheEXT)(VkDevice device, const VkValidationCacheCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkValidationCacheEXT* pValidationCache);


*PFN_vkDestroyValidationCacheEXT)(VkDevice device, VkValidationCacheEXT validationCache, const VkAllocationCallbacks* pAllocator);


*PFN_vkMergeValidationCachesEXT)(VkDevice device, VkValidationCacheEXT dstCache, uint32_t srcCacheCount, const VkValidationCacheEXT* pSrcCaches);


*PFN_vkGetValidationCacheDataEXT)(VkDevice device, VkValidationCacheEXT validationCache, size_t* pDataSize, void* pData);