
use vks;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PipelineCacheHeaderVersion {
    PipelineCacheHeaderVersionOne = vks::VK_PIPELINE_CACHE_HEADER_VERSION_ONE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum Result {
    Success = vks::VK_SUCCESS as isize,
    NotReady = vks::VK_NOT_READY as isize,
    Timeout = vks::VK_TIMEOUT as isize,
    EventSet = vks::VK_EVENT_SET as isize,
    EventReset = vks::VK_EVENT_RESET as isize,
    Incomplete = vks::VK_INCOMPLETE as isize,
    ErrorOutOfHostMemory = vks::VK_ERROR_OUT_OF_HOST_MEMORY as isize,
    ErrorOutOfDeviceMemory = vks::VK_ERROR_OUT_OF_DEVICE_MEMORY as isize,
    ErrorInitializationFailed = vks::VK_ERROR_INITIALIZATION_FAILED as isize,
    ErrorDeviceLost = vks::VK_ERROR_DEVICE_LOST as isize,
    ErrorMemoryMapFailed = vks::VK_ERROR_MEMORY_MAP_FAILED as isize,
    ErrorLayerNotPresent = vks::VK_ERROR_LAYER_NOT_PRESENT as isize,
    ErrorExtensionNotPresent = vks::VK_ERROR_EXTENSION_NOT_PRESENT as isize,
    ErrorFeatureNotPresent = vks::VK_ERROR_FEATURE_NOT_PRESENT as isize,
    ErrorIncompatibleDriver = vks::VK_ERROR_INCOMPATIBLE_DRIVER as isize,
    ErrorTooManyObjects = vks::VK_ERROR_TOO_MANY_OBJECTS as isize,
    ErrorFormatNotSupported = vks::VK_ERROR_FORMAT_NOT_SUPPORTED as isize,
    ErrorFragmentedPool = vks::VK_ERROR_FRAGMENTED_POOL as isize,
    ErrorSurfaceLostKhr = vks::VK_ERROR_SURFACE_LOST_KHR as isize,
    ErrorNativeWindowInUseKhr = vks::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR as isize,
    SuboptimalKhr = vks::VK_SUBOPTIMAL_KHR as isize,
    ErrorOutOfDateKhr = vks::VK_ERROR_OUT_OF_DATE_KHR as isize,
    ErrorIncompatibleDisplayKhr = vks::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR as isize,
    ErrorValidationFailedExt = vks::VK_ERROR_VALIDATION_FAILED_EXT as isize,
    ErrorInvalidShaderNv = vks::VK_ERROR_INVALID_SHADER_NV as isize,
    ErrorOutOfPoolMemoryKhr = vks::VK_ERROR_OUT_OF_POOL_MEMORY_KHR as isize,
    /// [`VK_KHR_external_semaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_external_semaphore)
    ErrorInvalidExternalHandleKhr = vks::VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum StructureType {
    ApplicationInfo = vks::VK_STRUCTURE_TYPE_APPLICATION_INFO as isize,
    InstanceCreateInfo = vks::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO as isize,
    DeviceQueueCreateInfo = vks::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO as isize,
    DeviceCreateInfo = vks::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO as isize,
    SubmitInfo = vks::VK_STRUCTURE_TYPE_SUBMIT_INFO as isize,
    MemoryAllocateInfo = vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO as isize,
    MappedMemoryRange = vks::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE as isize,
    BindSparseInfo = vks::VK_STRUCTURE_TYPE_BIND_SPARSE_INFO as isize,
    FenceCreateInfo = vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO as isize,
    SemaphoreCreateInfo = vks::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO as isize,
    EventCreateInfo = vks::VK_STRUCTURE_TYPE_EVENT_CREATE_INFO as isize,
    QueryPoolCreateInfo = vks::VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO as isize,
    BufferCreateInfo = vks::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO as isize,
    BufferViewCreateInfo = vks::VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO as isize,
    ImageCreateInfo = vks::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO as isize,
    ImageViewCreateInfo = vks::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO as isize,
    ShaderModuleCreateInfo = vks::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO as isize,
    PipelineCacheCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO as isize,
    PipelineShaderStageCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO as isize,
    PipelineVertexInputStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO as isize,
    PipelineInputAssemblyStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO as isize,
    PipelineTessellationStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO as isize,
    PipelineViewportStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO as isize,
    PipelineRasterizationStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO as isize,
    PipelineMultisampleStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO as isize,
    PipelineDepthStencilStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO as isize,
    PipelineColorBlendStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO as isize,
    PipelineDynamicStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO as isize,
    GraphicsPipelineCreateInfo = vks::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO as isize,
    ComputePipelineCreateInfo = vks::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO as isize,
    PipelineLayoutCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO as isize,
    SamplerCreateInfo = vks::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO as isize,
    DescriptorSetLayoutCreateInfo = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO as isize,
    DescriptorPoolCreateInfo = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO as isize,
    DescriptorSetAllocateInfo = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO as isize,
    WriteDescriptorSet = vks::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET as isize,
    CopyDescriptorSet = vks::VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET as isize,
    FramebufferCreateInfo = vks::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO as isize,
    RenderPassCreateInfo = vks::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO as isize,
    CommandPoolCreateInfo = vks::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO as isize,
    CommandBufferAllocateInfo = vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO as isize,
    CommandBufferInheritanceInfo = vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO as isize,
    CommandBufferBeginInfo = vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO as isize,
    RenderPassBeginInfo = vks::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO as isize,
    BufferMemoryBarrier = vks::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER as isize,
    ImageMemoryBarrier = vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER as isize,
    MemoryBarrier = vks::VK_STRUCTURE_TYPE_MEMORY_BARRIER as isize,
    LoaderInstanceCreateInfo = vks::VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO as isize,
    LoaderDeviceCreateInfo = vks::VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO as isize,
    SwapchainCreateInfoKhr = vks::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR as isize,
    PresentInfoKhr = vks::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR as isize,
    DisplayModeCreateInfoKhr = vks::VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR as isize,
    DisplaySurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR as isize,
    DisplayPresentInfoKhr = vks::VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR as isize,
    XlibSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR as isize,
    XcbSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR as isize,
    WaylandSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR as isize,
    MirSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR as isize,
    AndroidSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR as isize,
    Win32SurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR as isize,
    DebugReportCreateInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT as isize,
    // VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT as isize,
    PipelineRasterizationStateRasterizationOrderAmd = vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD as isize,
    DebugMarkerObjectNameInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT as isize,
    DebugMarkerObjectTagInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT as isize,
    DebugMarkerMarkerInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT as isize,
    DedicatedAllocationImageCreateInfoNv = vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV as isize,
    DedicatedAllocationBufferCreateInfoNv = vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV as isize,
    DedicatedAllocationMemoryAllocateInfoNv = vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV as isize,
    TextureLodGatherFormatPropertiesAmd = vks::VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD as isize,
    #[cfg(feature = "experimental")]
    RenderPassMultiviewCreateInfoKhx = vks::VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    PhysicalDeviceMultiviewFeaturesKhx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX as isize,
    #[cfg(feature = "experimental")]
    PhysicalDeviceMultiviewPropertiesKhx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX as isize,
    ExternalMemoryImageCreateInfoNv = vks::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV as isize,
    ExportMemoryAllocateInfoNv = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV as isize,
    ImportMemoryWin32HandleInfoNv = vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV as isize,
    ExportMemoryWin32HandleInfoNv = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV as isize,
    Win32KeyedMutexAcquireReleaseInfoNv = vks::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV as isize,
    PhysicalDeviceFeatures2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR as isize,
    PhysicalDeviceProperties2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR as isize,
    FormatProperties2Khr = vks::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR as isize,
    ImageFormatProperties2Khr = vks::VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR as isize,
    PhysicalDeviceImageFormatInfo2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR as isize,
    QueueFamilyProperties2Khr = vks::VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR as isize,
    PhysicalDeviceMemoryProperties2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR as isize,
    SparseImageFormatProperties2Khr = vks::VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR as isize,
    PhysicalDeviceSparseImageFormatInfo2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR as isize,
    #[cfg(feature = "experimental")]
    MemoryAllocateFlagsInfoKhx = vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    BindBufferMemoryInfoKhx = vks::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    BindImageMemoryInfoKhx = vks::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupRenderPassBeginInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupCommandBufferBeginInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupSubmitInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupBindSparseInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupPresentCapabilitiesKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX as isize,
    #[cfg(feature = "experimental")]
    ImageSwapchainCreateInfoKhx = vks::VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    BindImageMemorySwapchainInfoKhx = vks::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    AcquireNextImageInfoKhx = vks::VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupPresentInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupSwapchainCreateInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX as isize,
    ValidationFlagsExt = vks::VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT as isize,
    ViSurfaceCreateInfoNn = vks::VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN as isize,
    #[cfg(feature = "experimental")]
    PhysicalDeviceGroupPropertiesKhx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX as isize,
    #[cfg(feature = "experimental")]
    DeviceGroupDeviceCreateInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX as isize,
    PhysicalDeviceExternalImageFormatInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR as isize,
    ExternalImageFormatPropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR as isize,
    PhysicalDeviceExternalBufferInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR as isize,
    ExternalBufferPropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR as isize,
    PhysicalDeviceIDPropertiesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR as isize,
    ExternalMemoryBufferCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR as isize,
    ExternalMemoryImageCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR as isize,
    ExportMemoryAllocateInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR as isize,
    ImportMemoryWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR as isize,
    ExportMemoryWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR as isize,
    MemoryWin32HandlePropertiesKhr = vks::VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR as isize,
    MemoryGetWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR as isize,
    ImportMemoryFdInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR as isize,
    MemoryFdPropertiesKhr = vks::VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR as isize,
    MemoryGetFdInfoKhr = vks::VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR as isize,
    Win32KeyedMutexAcquireReleaseInfoKhr = vks::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR as isize,
    PhysicalDeviceExternalSemaphoreInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR as isize,
    ExternalSemaphorePropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR as isize,
    ExportSemaphoreCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR as isize,
    ImportSemaphoreWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR as isize,
    ExportSemaphoreWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR as isize,
    D3d12FenceSubmitInfoKhr = vks::VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR as isize,
    SemaphoreGetWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR as isize,
    ImportSemaphoreFdInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR as isize,
    SemaphoreGetFdInfoKhr = vks::VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR as isize,
    PhysicalDevicePushDescriptorPropertiesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR as isize,
    PhysicalDevice16bitStorageFeaturesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR as isize,
    PresentRegionsKhr = vks::VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR as isize,
    DescriptorUpdateTemplateCreateInfoKhr = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR as isize,
    #[cfg(feature = "experimental")]
    ObjectTableCreateInfoNvx = vks::VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX as isize,
    #[cfg(feature = "experimental")]
    IndirectCommandsLayoutCreateInfoNvx = vks::VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX as isize,
    #[cfg(feature = "experimental")]
    CmdProcessCommandsInfoNvx = vks::VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX as isize,
    #[cfg(feature = "experimental")]
    CmdReserveSpaceForCommandsInfoNvx = vks::VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX as isize,
    #[cfg(feature = "experimental")]
    DeviceGeneratedCommandsLimitsNvx = vks::VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX as isize,
    #[cfg(feature = "experimental")]
    DeviceGeneratedCommandsFeaturesNvx = vks::VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX as isize,
    PipelineViewportWScalingStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV as isize,
    SurfaceCapabilities2Ext = vks::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT as isize,
    // VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT = vks::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT as isize,
    DisplayPowerInfoExt = vks::VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT as isize,
    DeviceEventInfoExt = vks::VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT as isize,
    DisplayEventInfoExt = vks::VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT as isize,
    SwapchainCounterCreateInfoExt = vks::VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT as isize,
    PresentTimesInfoGoogle = vks::VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE as isize,
    #[cfg(feature = "experimental")]
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX as isize,
    PipelineViewportSwizzleStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV as isize,
    PhysicalDeviceDiscardRectanglePropertiesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT as isize,
    PipelineDiscardRectangleStateCreateInfoExt = vks::VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT as isize,
    HdrMetadataExt = vks::VK_STRUCTURE_TYPE_HDR_METADATA_EXT as isize,
    SharedPresentSurfaceCapabilitiesKhr = vks::VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR as isize,
    PhysicalDeviceExternalFenceInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR as isize,
    ExternalFencePropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR as isize,
    ExportFenceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR as isize,
    ImportFenceWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR as isize,
    ExportFenceWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR as isize,
    FenceGetWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR as isize,
    ImportFenceFdInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR as isize,
    FenceGetFdInfoKhr = vks::VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR as isize,
    PhysicalDeviceSurfaceInfo2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR as isize,
    SurfaceCapabilities2Khr = vks::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR as isize,
    SurfaceFormat2Khr = vks::VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR as isize,
    PhysicalDeviceVariablePointerFeaturesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR as isize,
    IosSurfaceCreateInfoMvk = vks::VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK as isize,
    MacosSurfaceCreateInfoMvk = vks::VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK as isize,
    MemoryDedicatedRequirementsKhr = vks::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR as isize,
    MemoryDedicatedAllocateInfoKhr = vks::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR as isize,
    PhysicalDeviceSamplerFilterMinmaxPropertiesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT as isize,
    SamplerReductionModeCreateInfoExt = vks::VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT as isize,
    BufferMemoryRequirementsInfo2Khr = vks::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR as isize,
    ImageMemoryRequirementsInfo2Khr = vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR as isize,
    ImageSparseMemoryRequirementsInfo2Khr = vks::VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR as isize,
    MemoryRequirements2Khr = vks::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR as isize,
    SparseImageMemoryRequirements2Khr = vks::VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR as isize,
    PhysicalDeviceBlendOperationAdvancedFeaturesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT as isize,
    PhysicalDeviceBlendOperationAdvancedPropertiesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT as isize,
    PipelineColorBlendAdvancedStateCreateInfoExt = vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT as isize,
    PipelineCoverageToColorStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV as isize,
    PipelineCoverageModulationStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SystemAllocationScope {
    Command = vks::VK_SYSTEM_ALLOCATION_SCOPE_COMMAND as isize,
    Object = vks::VK_SYSTEM_ALLOCATION_SCOPE_OBJECT as isize,
    Cache = vks::VK_SYSTEM_ALLOCATION_SCOPE_CACHE as isize,
    Device = vks::VK_SYSTEM_ALLOCATION_SCOPE_DEVICE as isize,
    Instance = vks::VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum InternalAllocationType {
    Executable = vks::VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum Format {
    Undefined = vks::VK_FORMAT_UNDEFINED as isize,
    R4G4UnormPack8 = vks::VK_FORMAT_R4G4_UNORM_PACK8 as isize,
    R4G4B4B4UnormPack16 = vks::VK_FORMAT_R4G4B4A4_UNORM_PACK16 as isize,
    B4G4R4A4UnormPack16 = vks::VK_FORMAT_B4G4R4A4_UNORM_PACK16 as isize,
    R5G6B5UnormPack16 = vks::VK_FORMAT_R5G6B5_UNORM_PACK16 as isize,
    B5G6R5UnormPack16 = vks::VK_FORMAT_B5G6R5_UNORM_PACK16 as isize,
    R5G5B5A1UnormPack16 = vks::VK_FORMAT_R5G5B5A1_UNORM_PACK16 as isize,
    B5G5R5A1UnormPack16 = vks::VK_FORMAT_B5G5R5A1_UNORM_PACK16 as isize,
    A1R5G5B5UnormPack16 = vks::VK_FORMAT_A1R5G5B5_UNORM_PACK16 as isize,
    R8Unorm = vks::VK_FORMAT_R8_UNORM as isize,
    R8Snorm = vks::VK_FORMAT_R8_SNORM as isize,
    R8Uscaled = vks::VK_FORMAT_R8_USCALED as isize,
    R8Sscaled = vks::VK_FORMAT_R8_SSCALED as isize,
    R8Uint = vks::VK_FORMAT_R8_UINT as isize,
    R8Sint = vks::VK_FORMAT_R8_SINT as isize,
    R8Srgb = vks::VK_FORMAT_R8_SRGB as isize,
    R8G8Unorm = vks::VK_FORMAT_R8G8_UNORM as isize,
    R8G8Snorm = vks::VK_FORMAT_R8G8_SNORM as isize,
    R8G8Uscaled = vks::VK_FORMAT_R8G8_USCALED as isize,
    R8G8Sscaled = vks::VK_FORMAT_R8G8_SSCALED as isize,
    R8G8Uint = vks::VK_FORMAT_R8G8_UINT as isize,
    R8G8Sint = vks::VK_FORMAT_R8G8_SINT as isize,
    R8G8Srgb = vks::VK_FORMAT_R8G8_SRGB as isize,
    R8G8B8Unorm = vks::VK_FORMAT_R8G8B8_UNORM as isize,
    R8G8B8Snorm = vks::VK_FORMAT_R8G8B8_SNORM as isize,
    R8G8B8Uscaled = vks::VK_FORMAT_R8G8B8_USCALED as isize,
    R8G8B8Sscaled = vks::VK_FORMAT_R8G8B8_SSCALED as isize,
    R8G8B8Uint = vks::VK_FORMAT_R8G8B8_UINT as isize,
    R8G8B8Sint = vks::VK_FORMAT_R8G8B8_SINT as isize,
    R8G8B8Srgb = vks::VK_FORMAT_R8G8B8_SRGB as isize,
    B8G8R8Unorm = vks::VK_FORMAT_B8G8R8_UNORM as isize,
    B8G8R8Snorm = vks::VK_FORMAT_B8G8R8_SNORM as isize,
    B8G8R8Uscaled = vks::VK_FORMAT_B8G8R8_USCALED as isize,
    B8G8R8Sscaled = vks::VK_FORMAT_B8G8R8_SSCALED as isize,
    B8G8R8Uint = vks::VK_FORMAT_B8G8R8_UINT as isize,
    B8G8R8Sint = vks::VK_FORMAT_B8G8R8_SINT as isize,
    B8G8R8Srgb = vks::VK_FORMAT_B8G8R8_SRGB as isize,
    R8G8B8A8Unorm = vks::VK_FORMAT_R8G8B8A8_UNORM as isize,
    R8G8B8A8Snorm = vks::VK_FORMAT_R8G8B8A8_SNORM as isize,
    R8G8B8A8Uscaled = vks::VK_FORMAT_R8G8B8A8_USCALED as isize,
    R8G8B8A8Sscaled = vks::VK_FORMAT_R8G8B8A8_SSCALED as isize,
    R8G8B8A8Uint = vks::VK_FORMAT_R8G8B8A8_UINT as isize,
    R8G8B8A8Sint = vks::VK_FORMAT_R8G8B8A8_SINT as isize,
    R8G8B8A8Srgb = vks::VK_FORMAT_R8G8B8A8_SRGB as isize,
    B8G8R8A8Unorm = vks::VK_FORMAT_B8G8R8A8_UNORM as isize,
    B8G8R8A8Snorm = vks::VK_FORMAT_B8G8R8A8_SNORM as isize,
    B8G8R8A8Uscaled = vks::VK_FORMAT_B8G8R8A8_USCALED as isize,
    B8G8R8A8Sscaled = vks::VK_FORMAT_B8G8R8A8_SSCALED as isize,
    B8G8R8A8Uint = vks::VK_FORMAT_B8G8R8A8_UINT as isize,
    B8G8R8A8Sint = vks::VK_FORMAT_B8G8R8A8_SINT as isize,
    B8G8R8A8Srgb = vks::VK_FORMAT_B8G8R8A8_SRGB as isize,
    A8B8G8R8UnormPack32 = vks::VK_FORMAT_A8B8G8R8_UNORM_PACK32 as isize,
    A8B8G8R8SnormPack32 = vks::VK_FORMAT_A8B8G8R8_SNORM_PACK32 as isize,
    A8B8G8R8UscaledPack32 = vks::VK_FORMAT_A8B8G8R8_USCALED_PACK32 as isize,
    A8B8G8R8SscaledPack32 = vks::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 as isize,
    A8B8G8R8UintPack32 = vks::VK_FORMAT_A8B8G8R8_UINT_PACK32 as isize,
    A8B8G8R8SintPack32 = vks::VK_FORMAT_A8B8G8R8_SINT_PACK32 as isize,
    A8B8G8R8SrgbPack32 = vks::VK_FORMAT_A8B8G8R8_SRGB_PACK32 as isize,
    A2R10G10B10UnormPack32 = vks::VK_FORMAT_A2R10G10B10_UNORM_PACK32 as isize,
    A2R10G10B10SnormPack32 = vks::VK_FORMAT_A2R10G10B10_SNORM_PACK32 as isize,
    A2R10G10B10UscaledPack32 = vks::VK_FORMAT_A2R10G10B10_USCALED_PACK32 as isize,
    A2R10G10B10SscaledPack32 = vks::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 as isize,
    A2R10G10B10UintPack32 = vks::VK_FORMAT_A2R10G10B10_UINT_PACK32 as isize,
    A2R10G10B10SintPack32 = vks::VK_FORMAT_A2R10G10B10_SINT_PACK32 as isize,
    A2B10G10R10UnormPack32 = vks::VK_FORMAT_A2B10G10R10_UNORM_PACK32 as isize,
    A2B10G10R10SnormPack32 = vks::VK_FORMAT_A2B10G10R10_SNORM_PACK32 as isize,
    A2B10G10R10UscaledPack32 = vks::VK_FORMAT_A2B10G10R10_USCALED_PACK32 as isize,
    A2B10G10R10SscaledPack32 = vks::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 as isize,
    A2B10G10R10UintPack32 = vks::VK_FORMAT_A2B10G10R10_UINT_PACK32 as isize,
    A2B10G10R10SintPack32 = vks::VK_FORMAT_A2B10G10R10_SINT_PACK32 as isize,
    R16Unorm = vks::VK_FORMAT_R16_UNORM as isize,
    R16Snorm = vks::VK_FORMAT_R16_SNORM as isize,
    R16Uscaled = vks::VK_FORMAT_R16_USCALED as isize,
    R16Sscaled = vks::VK_FORMAT_R16_SSCALED as isize,
    R16Uint = vks::VK_FORMAT_R16_UINT as isize,
    R16Sint = vks::VK_FORMAT_R16_SINT as isize,
    R16Sfloat = vks::VK_FORMAT_R16_SFLOAT as isize,
    R16G16Unorm = vks::VK_FORMAT_R16G16_UNORM as isize,
    R16G16Snorm = vks::VK_FORMAT_R16G16_SNORM as isize,
    R16G16Uscaled = vks::VK_FORMAT_R16G16_USCALED as isize,
    R16G16Sscaled = vks::VK_FORMAT_R16G16_SSCALED as isize,
    R16G16Uint = vks::VK_FORMAT_R16G16_UINT as isize,
    R16G16Sint = vks::VK_FORMAT_R16G16_SINT as isize,
    R16G16Sfloat = vks::VK_FORMAT_R16G16_SFLOAT as isize,
    R16G16B16Unorm = vks::VK_FORMAT_R16G16B16_UNORM as isize,
    R16G16B16Snorm = vks::VK_FORMAT_R16G16B16_SNORM as isize,
    R16G16B16Uscaled = vks::VK_FORMAT_R16G16B16_USCALED as isize,
    R16G16B16Sscaled = vks::VK_FORMAT_R16G16B16_SSCALED as isize,
    R16G16B16Uint = vks::VK_FORMAT_R16G16B16_UINT as isize,
    R16G16B16Sint = vks::VK_FORMAT_R16G16B16_SINT as isize,
    R16G16B16Sfloat = vks::VK_FORMAT_R16G16B16_SFLOAT as isize,
    R16G16B16A16Unorm = vks::VK_FORMAT_R16G16B16A16_UNORM as isize,
    R16G16B16A16Snorm = vks::VK_FORMAT_R16G16B16A16_SNORM as isize,
    R16G16B16A16Uscaled = vks::VK_FORMAT_R16G16B16A16_USCALED as isize,
    R16G16B16A16Sscaled = vks::VK_FORMAT_R16G16B16A16_SSCALED as isize,
    R16G16B16A16Uint = vks::VK_FORMAT_R16G16B16A16_UINT as isize,
    R16G16B16A16Sint = vks::VK_FORMAT_R16G16B16A16_SINT as isize,
    R16G16B16A16Sfloat = vks::VK_FORMAT_R16G16B16A16_SFLOAT as isize,
    R32Uint = vks::VK_FORMAT_R32_UINT as isize,
    R32Sint = vks::VK_FORMAT_R32_SINT as isize,
    R32Sfloat = vks::VK_FORMAT_R32_SFLOAT as isize,
    R32G32Uint = vks::VK_FORMAT_R32G32_UINT as isize,
    R32G32Sint = vks::VK_FORMAT_R32G32_SINT as isize,
    R32G32Sfloat = vks::VK_FORMAT_R32G32_SFLOAT as isize,
    R32G32B32Uint = vks::VK_FORMAT_R32G32B32_UINT as isize,
    R32G32B32Sint = vks::VK_FORMAT_R32G32B32_SINT as isize,
    R32G32B32Sfloat = vks::VK_FORMAT_R32G32B32_SFLOAT as isize,
    R32G32B32A32Uint = vks::VK_FORMAT_R32G32B32A32_UINT as isize,
    R32G32B32A32Sint = vks::VK_FORMAT_R32G32B32A32_SINT as isize,
    R32G32B32A32Sfloat = vks::VK_FORMAT_R32G32B32A32_SFLOAT as isize,
    R64Uint = vks::VK_FORMAT_R64_UINT as isize,
    R64Sint = vks::VK_FORMAT_R64_SINT as isize,
    R64Sfloat = vks::VK_FORMAT_R64_SFLOAT as isize,
    R64G64Uint = vks::VK_FORMAT_R64G64_UINT as isize,
    R64G64Sint = vks::VK_FORMAT_R64G64_SINT as isize,
    R64G64Sfloat = vks::VK_FORMAT_R64G64_SFLOAT as isize,
    R64G64B64Uint = vks::VK_FORMAT_R64G64B64_UINT as isize,
    R64G64B64Sint = vks::VK_FORMAT_R64G64B64_SINT as isize,
    R64G64B64Sfloat = vks::VK_FORMAT_R64G64B64_SFLOAT as isize,
    R64G64B64A64Uint = vks::VK_FORMAT_R64G64B64A64_UINT as isize,
    R64G64B64A64Sint = vks::VK_FORMAT_R64G64B64A64_SINT as isize,
    R64G64B64A64Sfloat = vks::VK_FORMAT_R64G64B64A64_SFLOAT as isize,
    B10G11R11UfloatPack32 = vks::VK_FORMAT_B10G11R11_UFLOAT_PACK32 as isize,
    E5B9G9R9UfloatPack32 = vks::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 as isize,
    D16Unorm = vks::VK_FORMAT_D16_UNORM as isize,
    X8D24UnormPack32 = vks::VK_FORMAT_X8_D24_UNORM_PACK32 as isize,
    D32Sfloat = vks::VK_FORMAT_D32_SFLOAT as isize,
    S8Uint = vks::VK_FORMAT_S8_UINT as isize,
    D16UnormS8Uint = vks::VK_FORMAT_D16_UNORM_S8_UINT as isize,
    D24UnormS8Uint = vks::VK_FORMAT_D24_UNORM_S8_UINT as isize,
    D32SfloatS8Uint = vks::VK_FORMAT_D32_SFLOAT_S8_UINT as isize,
    Bc1RgbUnormBlock = vks::VK_FORMAT_BC1_RGB_UNORM_BLOCK as isize,
    Bc1RgbSrgbBlock = vks::VK_FORMAT_BC1_RGB_SRGB_BLOCK as isize,
    Bc1RgbaUnormBlock = vks::VK_FORMAT_BC1_RGBA_UNORM_BLOCK as isize,
    Bc1RgbaSrgbBlock = vks::VK_FORMAT_BC1_RGBA_SRGB_BLOCK as isize,
    Bc2UnormBlock = vks::VK_FORMAT_BC2_UNORM_BLOCK as isize,
    Bc2SrgbBlock = vks::VK_FORMAT_BC2_SRGB_BLOCK as isize,
    Bc3UnormBlock = vks::VK_FORMAT_BC3_UNORM_BLOCK as isize,
    Bc3SrgbBlock = vks::VK_FORMAT_BC3_SRGB_BLOCK as isize,
    Bc4UnormBlock = vks::VK_FORMAT_BC4_UNORM_BLOCK as isize,
    Bc4SnormBlock = vks::VK_FORMAT_BC4_SNORM_BLOCK as isize,
    Bc5UnormBlock = vks::VK_FORMAT_BC5_UNORM_BLOCK as isize,
    Bc5SnormBlock = vks::VK_FORMAT_BC5_SNORM_BLOCK as isize,
    Bc6hUfloatBlock = vks::VK_FORMAT_BC6H_UFLOAT_BLOCK as isize,
    Bc6hSfloatBlock = vks::VK_FORMAT_BC6H_SFLOAT_BLOCK as isize,
    Bc7UnormBlock = vks::VK_FORMAT_BC7_UNORM_BLOCK as isize,
    Bc7SrgbBlock = vks::VK_FORMAT_BC7_SRGB_BLOCK as isize,
    Etc2R8G8B8UnormBlock = vks::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK as isize,
    Etc2R8G8B8SrgbBlock = vks::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK as isize,
    Etc2R8G8B8A1UnormBlock = vks::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK as isize,
    Etc2R8G8B8A1SrgbBlock = vks::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK as isize,
    Etc2R8G8B8A8UnormBlock = vks::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK as isize,
    Etc2R8G8B8A8SrgbBlock = vks::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK as isize,
    EacR11UnormBlock = vks::VK_FORMAT_EAC_R11_UNORM_BLOCK as isize,
    EacR11SnormBlock = vks::VK_FORMAT_EAC_R11_SNORM_BLOCK as isize,
    EacR11G11UnormBlock = vks::VK_FORMAT_EAC_R11G11_UNORM_BLOCK as isize,
    EacR11G11SnormBlock = vks::VK_FORMAT_EAC_R11G11_SNORM_BLOCK as isize,
    Astc4x4UnormBlock = vks::VK_FORMAT_ASTC_4x4_UNORM_BLOCK as isize,
    Astc4x4SrgbBlock = vks::VK_FORMAT_ASTC_4x4_SRGB_BLOCK as isize,
    Astc5x4UnormBlock = vks::VK_FORMAT_ASTC_5x4_UNORM_BLOCK as isize,
    Astc5x4SrgbBlock = vks::VK_FORMAT_ASTC_5x4_SRGB_BLOCK as isize,
    Astc5x5UnormBlock = vks::VK_FORMAT_ASTC_5x5_UNORM_BLOCK as isize,
    Astc5x5SrgbBlock = vks::VK_FORMAT_ASTC_5x5_SRGB_BLOCK as isize,
    Astc6x5UnormBlock = vks::VK_FORMAT_ASTC_6x5_UNORM_BLOCK as isize,
    Astc6x5SrgbBlock = vks::VK_FORMAT_ASTC_6x5_SRGB_BLOCK as isize,
    Astc6x6UnormBlock = vks::VK_FORMAT_ASTC_6x6_UNORM_BLOCK as isize,
    Astc6x6SrgbBlock = vks::VK_FORMAT_ASTC_6x6_SRGB_BLOCK as isize,
    Astc8x5UnormBlock = vks::VK_FORMAT_ASTC_8x5_UNORM_BLOCK as isize,
    Astc8x5SrgbBlock = vks::VK_FORMAT_ASTC_8x5_SRGB_BLOCK as isize,
    Astc8x6UnormBlock = vks::VK_FORMAT_ASTC_8x6_UNORM_BLOCK as isize,
    Astc8x6SrgbBlock = vks::VK_FORMAT_ASTC_8x6_SRGB_BLOCK as isize,
    Astc8x8UnormBlock = vks::VK_FORMAT_ASTC_8x8_UNORM_BLOCK as isize,
    Astc8x8SrgbBlock = vks::VK_FORMAT_ASTC_8x8_SRGB_BLOCK as isize,
    Astc10x5UnormBlock = vks::VK_FORMAT_ASTC_10x5_UNORM_BLOCK as isize,
    Astc10x5SrgbBlock = vks::VK_FORMAT_ASTC_10x5_SRGB_BLOCK as isize,
    Astc10x6UnormBlock = vks::VK_FORMAT_ASTC_10x6_UNORM_BLOCK as isize,
    Astc10x6SrgbBlock = vks::VK_FORMAT_ASTC_10x6_SRGB_BLOCK as isize,
    Astc10x8UnormBlock = vks::VK_FORMAT_ASTC_10x8_UNORM_BLOCK as isize,
    Astc10x8SrgbBlock = vks::VK_FORMAT_ASTC_10x8_SRGB_BLOCK as isize,
    Astc10x10UnormBlock = vks::VK_FORMAT_ASTC_10x10_UNORM_BLOCK as isize,
    Astc10x10SrgbBlock = vks::VK_FORMAT_ASTC_10x10_SRGB_BLOCK as isize,
    Astc12x10UnormBlock = vks::VK_FORMAT_ASTC_12x10_UNORM_BLOCK as isize,
    Astc12x10SrgbBlock = vks::VK_FORMAT_ASTC_12x10_SRGB_BLOCK as isize,
    Astc12x12UnormBlock = vks::VK_FORMAT_ASTC_12x12_UNORM_BLOCK as isize,
    Astc12x12SrgbBlock = vks::VK_FORMAT_ASTC_12x12_SRGB_BLOCK as isize,
    Pvrtc12bppUnormBlockImg = vks::VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG as isize,
    Pvrtc14bppUnormBlockImg = vks::VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG as isize,
    Pvrtc22bppUnormBlockImg = vks::VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG as isize,
    Pvrtc24bppUnormBlockImg = vks::VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG as isize,
    Pvrtc12bppSrgbBlockImg = vks::VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG as isize,
    Pvrtc14bppSrgbBlockImg = vks::VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG as isize,
    Pvrtc22bppSrgbBlockImg = vks::VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG as isize,
    Pvrtc24bppSrgbBlockImg = vks::VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageType {
    Type1d = vks::VK_IMAGE_TYPE_1D as isize,
    Type2d = vks::VK_IMAGE_TYPE_2D as isize,
    Type3d = vks::VK_IMAGE_TYPE_3D as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageTiling {
    Optimal = vks::VK_IMAGE_TILING_OPTIMAL as isize,
    Linear = vks::VK_IMAGE_TILING_LINEAR as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PhysicalDeviceType {
    Other = vks::VK_PHYSICAL_DEVICE_TYPE_OTHER as isize,
    IntegratedGpu = vks::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU as isize,
    DiscreteGpu = vks::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU as isize,
    VirtualGpu = vks::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU as isize,
    Cpu = vks::VK_PHYSICAL_DEVICE_TYPE_CPU as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum QueryType {
    Occlusion = vks::VK_QUERY_TYPE_OCCLUSION as isize,
    PipelineStatistics = vks::VK_QUERY_TYPE_PIPELINE_STATISTICS as isize,
    Timestamp = vks::VK_QUERY_TYPE_TIMESTAMP as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SharingMode {
    Exclusive = vks::VK_SHARING_MODE_EXCLUSIVE as isize,
    Concurrent = vks::VK_SHARING_MODE_CONCURRENT as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageLayout {
    Undefined = vks::VK_IMAGE_LAYOUT_UNDEFINED as isize,
    General = vks::VK_IMAGE_LAYOUT_GENERAL as isize,
    ColorAttachmentOptimal = vks::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL as isize,
    DepthStencilAttachmentOptimal = vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL as isize,
    DepthStencilReadOnlyOptimal = vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL as isize,
    ShaderReadOnlyOptimal = vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL as isize,
    TransferSrcOptimal = vks::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL as isize,
    TransferDstOptimal = vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as isize,
    Preinitialized = vks::VK_IMAGE_LAYOUT_PREINITIALIZED as isize,
    PresentSrcKhr = vks::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR as isize,
    SharedPresentKhr = vks::VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageViewType {
    Type1d = vks::VK_IMAGE_VIEW_TYPE_1D as isize,
    Type2d = vks::VK_IMAGE_VIEW_TYPE_2D as isize,
    Type3d = vks::VK_IMAGE_VIEW_TYPE_3D as isize,
    Cube = vks::VK_IMAGE_VIEW_TYPE_CUBE as isize,
    Type1dArray = vks::VK_IMAGE_VIEW_TYPE_1D_ARRAY as isize,
    Type2dArray = vks::VK_IMAGE_VIEW_TYPE_2D_ARRAY as isize,
    CubeArray = vks::VK_IMAGE_VIEW_TYPE_CUBE_ARRAY as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ComponentSwizzle {
    Identity = vks::VK_COMPONENT_SWIZZLE_IDENTITY as isize,
    Zero = vks::VK_COMPONENT_SWIZZLE_ZERO as isize,
    One = vks::VK_COMPONENT_SWIZZLE_ONE as isize,
    R = vks::VK_COMPONENT_SWIZZLE_R as isize,
    G = vks::VK_COMPONENT_SWIZZLE_G as isize,
    B = vks::VK_COMPONENT_SWIZZLE_B as isize,
    A = vks::VK_COMPONENT_SWIZZLE_A as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum VertexInputRate {
    Vertex = vks::VK_VERTEX_INPUT_RATE_VERTEX as isize,
    Instance = vks::VK_VERTEX_INPUT_RATE_INSTANCE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PrimitiveTopology {
    PointList = vks::VK_PRIMITIVE_TOPOLOGY_POINT_LIST as isize,
    LineList = vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST as isize,
    LineStrip = vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP as isize,
    TriangleList = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST as isize,
    TriangleStrip = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP as isize,
    TriangleFan = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN as isize,
    LineListWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY as isize,
    LineStripWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY as isize,
    TriangleListWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY as isize,
    TriangleStripWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY as isize,
    PatchList = vks::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PolygonMode {
    Fill = vks::VK_POLYGON_MODE_FILL as isize,
    Line = vks::VK_POLYGON_MODE_LINE as isize,
    Point = vks::VK_POLYGON_MODE_POINT as isize,
    FillRectangleNv = vks::VK_POLYGON_MODE_FILL_RECTANGLE_NV as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum FrontFace {
    CounterClockwise = vks::VK_FRONT_FACE_COUNTER_CLOCKWISE as isize,
    Clockwise = vks::VK_FRONT_FACE_CLOCKWISE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum CompareOp {
    Never = vks::VK_COMPARE_OP_NEVER as isize,
    Less = vks::VK_COMPARE_OP_LESS as isize,
    Equal = vks::VK_COMPARE_OP_EQUAL as isize,
    LessOrEqual = vks::VK_COMPARE_OP_LESS_OR_EQUAL as isize,
    Greater = vks::VK_COMPARE_OP_GREATER as isize,
    NotEqual = vks::VK_COMPARE_OP_NOT_EQUAL as isize,
    GreaterOrEqual = vks::VK_COMPARE_OP_GREATER_OR_EQUAL as isize,
    Always = vks::VK_COMPARE_OP_ALWAYS as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum StencilOp {
    Keep = vks::VK_STENCIL_OP_KEEP as isize,
    Zero = vks::VK_STENCIL_OP_ZERO as isize,
    Replace = vks::VK_STENCIL_OP_REPLACE as isize,
    IncrementAndClamp = vks::VK_STENCIL_OP_INCREMENT_AND_CLAMP as isize,
    DecrementAndClamp = vks::VK_STENCIL_OP_DECREMENT_AND_CLAMP as isize,
    Invert = vks::VK_STENCIL_OP_INVERT as isize,
    IncrementAndWrap = vks::VK_STENCIL_OP_INCREMENT_AND_WRAP as isize,
    DecrementAndWrap = vks::VK_STENCIL_OP_DECREMENT_AND_WRAP as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum LogicOp {
    Clear = vks::VK_LOGIC_OP_CLEAR as isize,
    And = vks::VK_LOGIC_OP_AND as isize,
    AndReverse = vks::VK_LOGIC_OP_AND_REVERSE as isize,
    Copy = vks::VK_LOGIC_OP_COPY as isize,
    AndInverted = vks::VK_LOGIC_OP_AND_INVERTED as isize,
    NoOp = vks::VK_LOGIC_OP_NO_OP as isize,
    Xor = vks::VK_LOGIC_OP_XOR as isize,
    Or = vks::VK_LOGIC_OP_OR as isize,
    Nor = vks::VK_LOGIC_OP_NOR as isize,
    Equivalent = vks::VK_LOGIC_OP_EQUIVALENT as isize,
    Invert = vks::VK_LOGIC_OP_INVERT as isize,
    OrReverse = vks::VK_LOGIC_OP_OR_REVERSE as isize,
    CopyInverted = vks::VK_LOGIC_OP_COPY_INVERTED as isize,
    OrInverted = vks::VK_LOGIC_OP_OR_INVERTED as isize,
    Nand = vks::VK_LOGIC_OP_NAND as isize,
    Set = vks::VK_LOGIC_OP_SET as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum BlendFactor {
    Zero = vks::VK_BLEND_FACTOR_ZERO as isize,
    One = vks::VK_BLEND_FACTOR_ONE as isize,
    SrcColor = vks::VK_BLEND_FACTOR_SRC_COLOR as isize,
    OneMinusSrcColor = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR as isize,
    DstColor = vks::VK_BLEND_FACTOR_DST_COLOR as isize,
    OneMinusDstColor = vks::VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR as isize,
    SrcAlpha = vks::VK_BLEND_FACTOR_SRC_ALPHA as isize,
    OneMinusSrcAlpha = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA as isize,
    DstAlpha = vks::VK_BLEND_FACTOR_DST_ALPHA as isize,
    OneMinusDstAlpha = vks::VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA as isize,
    ConstantColor = vks::VK_BLEND_FACTOR_CONSTANT_COLOR as isize,
    OneMinusConstantColor = vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR as isize,
    ConstantAlpha = vks::VK_BLEND_FACTOR_CONSTANT_ALPHA as isize,
    OneMinusConstantAlpha = vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA as isize,
    SrcAlphaSaturate = vks::VK_BLEND_FACTOR_SRC_ALPHA_SATURATE as isize,
    Src1Color = vks::VK_BLEND_FACTOR_SRC1_COLOR as isize,
    OneMinusSrc1Color = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR as isize,
    Src1Alpha = vks::VK_BLEND_FACTOR_SRC1_ALPHA as isize,
    OneMinusSrc1Alpha = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum BlendOp {
    Add = vks::VK_BLEND_OP_ADD as isize,
    Subtract = vks::VK_BLEND_OP_SUBTRACT as isize,
    ReverseSubtract = vks::VK_BLEND_OP_REVERSE_SUBTRACT as isize,
    Min = vks::VK_BLEND_OP_MIN as isize,
    Max = vks::VK_BLEND_OP_MAX as isize,
    ZeroExt = vks::VK_BLEND_OP_ZERO_EXT as isize,
    SrcExt = vks::VK_BLEND_OP_SRC_EXT as isize,
    DstExt = vks::VK_BLEND_OP_DST_EXT as isize,
    SrcOverExt = vks::VK_BLEND_OP_SRC_OVER_EXT as isize,
    DstOverExt = vks::VK_BLEND_OP_DST_OVER_EXT as isize,
    SrcInExt = vks::VK_BLEND_OP_SRC_IN_EXT as isize,
    DstInExt = vks::VK_BLEND_OP_DST_IN_EXT as isize,
    SrcOutExt = vks::VK_BLEND_OP_SRC_OUT_EXT as isize,
    DstOutExt = vks::VK_BLEND_OP_DST_OUT_EXT as isize,
    SrcAtopExt = vks::VK_BLEND_OP_SRC_ATOP_EXT as isize,
    DstAtopExt = vks::VK_BLEND_OP_DST_ATOP_EXT as isize,
    XorExt = vks::VK_BLEND_OP_XOR_EXT as isize,
    MultiplyExt = vks::VK_BLEND_OP_MULTIPLY_EXT as isize,
    ScreenExt = vks::VK_BLEND_OP_SCREEN_EXT as isize,
    OverlayExt = vks::VK_BLEND_OP_OVERLAY_EXT as isize,
    DarkenExt = vks::VK_BLEND_OP_DARKEN_EXT as isize,
    LightenExt = vks::VK_BLEND_OP_LIGHTEN_EXT as isize,
    ColordodgeExt = vks::VK_BLEND_OP_COLORDODGE_EXT as isize,
    ColorburnExt = vks::VK_BLEND_OP_COLORBURN_EXT as isize,
    HardlightExt = vks::VK_BLEND_OP_HARDLIGHT_EXT as isize,
    SoftlightExt = vks::VK_BLEND_OP_SOFTLIGHT_EXT as isize,
    DifferenceExt = vks::VK_BLEND_OP_DIFFERENCE_EXT as isize,
    ExclusionExt = vks::VK_BLEND_OP_EXCLUSION_EXT as isize,
    InvertExt = vks::VK_BLEND_OP_INVERT_EXT as isize,
    InvertRgbExt = vks::VK_BLEND_OP_INVERT_RGB_EXT as isize,
    LineardodgeExt = vks::VK_BLEND_OP_LINEARDODGE_EXT as isize,
    LinearburnExt = vks::VK_BLEND_OP_LINEARBURN_EXT as isize,
    VividlightExt = vks::VK_BLEND_OP_VIVIDLIGHT_EXT as isize,
    LinearlightExt = vks::VK_BLEND_OP_LINEARLIGHT_EXT as isize,
    PinlightExt = vks::VK_BLEND_OP_PINLIGHT_EXT as isize,
    HardmixExt = vks::VK_BLEND_OP_HARDMIX_EXT as isize,
    HslHueExt = vks::VK_BLEND_OP_HSL_HUE_EXT as isize,
    HslSaturationExt = vks::VK_BLEND_OP_HSL_SATURATION_EXT as isize,
    HslColorExt = vks::VK_BLEND_OP_HSL_COLOR_EXT as isize,
    HslLuminosityExt = vks::VK_BLEND_OP_HSL_LUMINOSITY_EXT as isize,
    PlusExt = vks::VK_BLEND_OP_PLUS_EXT as isize,
    PlusClampedExt = vks::VK_BLEND_OP_PLUS_CLAMPED_EXT as isize,
    PlusClampedAlphaExt = vks::VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT as isize,
    PlusDarkerExt = vks::VK_BLEND_OP_PLUS_DARKER_EXT as isize,
    MinusExt = vks::VK_BLEND_OP_MINUS_EXT as isize,
    MinusClampedExt = vks::VK_BLEND_OP_MINUS_CLAMPED_EXT as isize,
    ContrastExt = vks::VK_BLEND_OP_CONTRAST_EXT as isize,
    InvertOvgExt = vks::VK_BLEND_OP_INVERT_OVG_EXT as isize,
    RedExt = vks::VK_BLEND_OP_RED_EXT as isize,
    GreenExt = vks::VK_BLEND_OP_GREEN_EXT as isize,
    BlueExt = vks::VK_BLEND_OP_BLUE_EXT as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum DynamicState {
    Viewport = vks::VK_DYNAMIC_STATE_VIEWPORT as isize,
    Scissor = vks::VK_DYNAMIC_STATE_SCISSOR as isize,
    LineWidth = vks::VK_DYNAMIC_STATE_LINE_WIDTH as isize,
    DepthBias = vks::VK_DYNAMIC_STATE_DEPTH_BIAS as isize,
    BlendConstants = vks::VK_DYNAMIC_STATE_BLEND_CONSTANTS as isize,
    DepthBounds = vks::VK_DYNAMIC_STATE_DEPTH_BOUNDS as isize,
    StencilCompareMask = vks::VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK as isize,
    StencilWriteMask = vks::VK_DYNAMIC_STATE_STENCIL_WRITE_MASK as isize,
    StencilReference = vks::VK_DYNAMIC_STATE_STENCIL_REFERENCE as isize,
    ViewportWScalingNv = vks::VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV as isize,
    DiscardRectangleExt = vks::VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum Filter {
    Nearest = vks::VK_FILTER_NEAREST as isize,
    Linear = vks::VK_FILTER_LINEAR as isize,
    CubicImg = vks::VK_FILTER_CUBIC_IMG as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SamplerMipmapMode {
    Nearest = vks::VK_SAMPLER_MIPMAP_MODE_NEAREST as isize,
    Linear = vks::VK_SAMPLER_MIPMAP_MODE_LINEAR as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SamplerAddressMode {
    Repeat = vks::VK_SAMPLER_ADDRESS_MODE_REPEAT as isize,
    MirroredRepeat = vks::VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as isize,
    ClampToEdge = vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE as isize,
    ClampToBorder = vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as isize,
    MirrorClampToEdge = vks::VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum BorderColor {
    FloatTransparentBlack = vks::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK as isize,
    IntTransparentBlack = vks::VK_BORDER_COLOR_INT_TRANSPARENT_BLACK as isize,
    FloatOpaqueBlack = vks::VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK as isize,
    IntOpaqueBlack = vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK as isize,
    FloatOpaqueWhite = vks::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE as isize,
    IntOpaqueWhite = vks::VK_BORDER_COLOR_INT_OPAQUE_WHITE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum DescriptorType {
    Sampler = vks::VK_DESCRIPTOR_TYPE_SAMPLER as isize,
    CombinedImageSampler = vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER as isize,
    SampledImage = vks::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE as isize,
    StorageImage = vks::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE as isize,
    UniformTexelBuffer = vks::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER as isize,
    StorageTexelBuffer = vks::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER as isize,
    UniformBuffer = vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER as isize,
    StorageBuffer = vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER as isize,
    UniformBufferDynamic = vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC as isize,
    StorageBufferDynamic = vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC as isize,
    InputAttachment = vks::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum AttachmentLoadOp {
    Load = vks::VK_ATTACHMENT_LOAD_OP_LOAD as isize,
    Clear = vks::VK_ATTACHMENT_LOAD_OP_CLEAR as isize,
    DontCare = vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum AttachmentStoreOp {
    Store = vks::VK_ATTACHMENT_STORE_OP_STORE as isize,
    DontCare = vks::VK_ATTACHMENT_STORE_OP_DONT_CARE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PipelineBindPoint {
    Graphics = vks::VK_PIPELINE_BIND_POINT_GRAPHICS as isize,
    Compute = vks::VK_PIPELINE_BIND_POINT_COMPUTE as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum CommandBufferLevel {
    Primary = vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY as isize,
    Secondary = vks::VK_COMMAND_BUFFER_LEVEL_SECONDARY as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum IndexType {
    Uint16 = vks::VK_INDEX_TYPE_UINT16 as isize,
    Uint32 = vks::VK_INDEX_TYPE_UINT32 as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SubpassContents {
    Inline = vks::VK_SUBPASS_CONTENTS_INLINE as isize,
    SecondaryCommandBuffers = vks::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS as isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ObjectType {
    Unknown = vks::VK_OBJECT_TYPE_UNKNOWN as isize,
    Instance = vks::VK_OBJECT_TYPE_INSTANCE as isize,
    PhysicalDevice = vks::VK_OBJECT_TYPE_PHYSICAL_DEVICE as isize,
    Device = vks::VK_OBJECT_TYPE_DEVICE as isize,
    Queue = vks::VK_OBJECT_TYPE_QUEUE as isize,
    Semaphore = vks::VK_OBJECT_TYPE_SEMAPHORE as isize,
    CommandBuffer = vks::VK_OBJECT_TYPE_COMMAND_BUFFER as isize,
    Fence = vks::VK_OBJECT_TYPE_FENCE as isize,
    DeviceMemory = vks::VK_OBJECT_TYPE_DEVICE_MEMORY as isize,
    Buffer = vks::VK_OBJECT_TYPE_BUFFER as isize,
    Image = vks::VK_OBJECT_TYPE_IMAGE as isize,
    Event = vks::VK_OBJECT_TYPE_EVENT as isize,
    QueryPool = vks::VK_OBJECT_TYPE_QUERY_POOL as isize,
    BufferView = vks::VK_OBJECT_TYPE_BUFFER_VIEW as isize,
    ImageView = vks::VK_OBJECT_TYPE_IMAGE_VIEW as isize,
    ShaderModule = vks::VK_OBJECT_TYPE_SHADER_MODULE as isize,
    PipelineCache = vks::VK_OBJECT_TYPE_PIPELINE_CACHE as isize,
    PipelineLayout = vks::VK_OBJECT_TYPE_PIPELINE_LAYOUT as isize,
    RenderPass = vks::VK_OBJECT_TYPE_RENDER_PASS as isize,
    Pipeline = vks::VK_OBJECT_TYPE_PIPELINE as isize,
    DescriptorSetLayout = vks::VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT as isize,
    Sampler = vks::VK_OBJECT_TYPE_SAMPLER as isize,
    DescriptorPool = vks::VK_OBJECT_TYPE_DESCRIPTOR_POOL as isize,
    DescriptorSet = vks::VK_OBJECT_TYPE_DESCRIPTOR_SET as isize,
    Framebuffer = vks::VK_OBJECT_TYPE_FRAMEBUFFER as isize,
    CommandPool = vks::VK_OBJECT_TYPE_COMMAND_POOL as isize,
    SurfaceKhr = vks::VK_OBJECT_TYPE_SURFACE_KHR as isize,
    SwapchainKhr = vks::VK_OBJECT_TYPE_SWAPCHAIN_KHR as isize,
    DisplayKhr = vks::VK_OBJECT_TYPE_DISPLAY_KHR as isize,
    DisplayModeKhr = vks::VK_OBJECT_TYPE_DISPLAY_MODE_KHR as isize,
    DebugReportCallbackExt = vks::VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT as isize,
    DescriptorUpdateTemplateKhr = 1000085000 as isize,
    #[cfg(feature = "experimental")]
    ObjectTableNvx = vks::VK_OBJECT_TYPE_OBJECT_TABLE_NVX as isize,
    #[cfg(feature = "experimental")]
    IndirectCommandsLayoutNvx = vks::VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX as isize,
}
