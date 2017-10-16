use vks;
use num_traits::FromPrimitive;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PipelineCacheHeaderVersion {
    PipelineCacheHeaderVersionOne = vks::VK_PIPELINE_CACHE_HEADER_VERSION_ONE as i32,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum CallResult {
    Success = vks::VK_SUCCESS as i32,
    NotReady = vks::VK_NOT_READY as i32,
    Timeout = vks::VK_TIMEOUT as i32,
    EventSet = vks::VK_EVENT_SET as i32,
    EventReset = vks::VK_EVENT_RESET as i32,
    Incomplete = vks::VK_INCOMPLETE as i32,
    ErrorOutOfHostMemory = vks::VK_ERROR_OUT_OF_HOST_MEMORY as i32,
    ErrorOutOfDeviceMemory = vks::VK_ERROR_OUT_OF_DEVICE_MEMORY as i32,
    ErrorInitializationFailed = vks::VK_ERROR_INITIALIZATION_FAILED as i32,
    ErrorDeviceLost = vks::VK_ERROR_DEVICE_LOST as i32,
    ErrorMemoryMapFailed = vks::VK_ERROR_MEMORY_MAP_FAILED as i32,
    ErrorLayerNotPresent = vks::VK_ERROR_LAYER_NOT_PRESENT as i32,
    ErrorExtensionNotPresent = vks::VK_ERROR_EXTENSION_NOT_PRESENT as i32,
    ErrorFeatureNotPresent = vks::VK_ERROR_FEATURE_NOT_PRESENT as i32,
    ErrorIncompatibleDriver = vks::VK_ERROR_INCOMPATIBLE_DRIVER as i32,
    ErrorTooManyObjects = vks::VK_ERROR_TOO_MANY_OBJECTS as i32,
    ErrorFormatNotSupported = vks::VK_ERROR_FORMAT_NOT_SUPPORTED as i32,
    ErrorFragmentedPool = vks::VK_ERROR_FRAGMENTED_POOL as i32,
    ErrorSurfaceLostKhr = vks::VK_ERROR_SURFACE_LOST_KHR as i32,
    ErrorNativeWindowInUseKhr = vks::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR as i32,
    SuboptimalKhr = vks::VK_SUBOPTIMAL_KHR as i32,
    ErrorOutOfDateKhr = vks::VK_ERROR_OUT_OF_DATE_KHR as i32,
    ErrorIncompatibleDisplayKhr = vks::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR as i32,
    ErrorValidationFailedExt = vks::VK_ERROR_VALIDATION_FAILED_EXT as i32,
    ErrorInvalidShaderNv = vks::VK_ERROR_INVALID_SHADER_NV as i32,
    ErrorOutOfPoolMemoryKhr = vks::VK_ERROR_OUT_OF_POOL_MEMORY_KHR as i32,
    ErrorInvalidExternalHandleKhr = vks::VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR as i32,
}

impl From<CallResult> for i32 {
    fn from(f: CallResult) -> i32 {
        f as i32
    }
}

impl From<CallResult> for u32 {
    fn from(f: CallResult) -> u32 {
        f as u32
    }
}

impl From<u32> for CallResult {
    fn from(f: u32) -> CallResult {
        CallResult::from_u32(f).unwrap()
    }
}

impl From<i32> for CallResult {
    fn from(f: i32) -> CallResult {
        CallResult::from_i32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum StructureType {
    ApplicationInfo = vks::VK_STRUCTURE_TYPE_APPLICATION_INFO as i32,
    InstanceCreateInfo = vks::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO as i32,
    DeviceQueueCreateInfo = vks::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO as i32,
    DeviceCreateInfo = vks::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO as i32,
    SubmitInfo = vks::VK_STRUCTURE_TYPE_SUBMIT_INFO as i32,
    MemoryAllocateInfo = vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO as i32,
    MappedMemoryRange = vks::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE as i32,
    BindSparseInfo = vks::VK_STRUCTURE_TYPE_BIND_SPARSE_INFO as i32,
    FenceCreateInfo = vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO as i32,
    SemaphoreCreateInfo = vks::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO as i32,
    EventCreateInfo = vks::VK_STRUCTURE_TYPE_EVENT_CREATE_INFO as i32,
    QueryPoolCreateInfo = vks::VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO as i32,
    BufferCreateInfo = vks::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO as i32,
    BufferViewCreateInfo = vks::VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO as i32,
    ImageCreateInfo = vks::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO as i32,
    ImageViewCreateInfo = vks::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO as i32,
    ShaderModuleCreateInfo = vks::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO as i32,
    PipelineCacheCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO as i32,
    PipelineShaderStageCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO as i32,
    PipelineVertexInputStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO as i32,
    PipelineInputAssemblyStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO as i32,
    PipelineTessellationStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO as i32,
    PipelineViewportStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO as i32,
    PipelineRasterizationStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO as i32,
    PipelineMultisampleStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO as i32,
    PipelineDepthStencilStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO as i32,
    PipelineColorBlendStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO as i32,
    PipelineDynamicStateCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO as i32,
    GraphicsPipelineCreateInfo = vks::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO as i32,
    ComputePipelineCreateInfo = vks::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO as i32,
    PipelineLayoutCreateInfo = vks::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO as i32,
    SamplerCreateInfo = vks::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO as i32,
    DescriptorSetLayoutCreateInfo = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO as i32,
    DescriptorPoolCreateInfo = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO as i32,
    DescriptorSetAllocateInfo = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO as i32,
    WriteDescriptorSet = vks::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET as i32,
    CopyDescriptorSet = vks::VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET as i32,
    FramebufferCreateInfo = vks::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO as i32,
    RenderPassCreateInfo = vks::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO as i32,
    CommandPoolCreateInfo = vks::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO as i32,
    CommandBufferAllocateInfo = vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO as i32,
    CommandBufferInheritanceInfo = vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO as i32,
    CommandBufferBeginInfo = vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO as i32,
    RenderPassBeginInfo = vks::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO as i32,
    BufferMemoryBarrier = vks::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER as i32,
    ImageMemoryBarrier = vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER as i32,
    MemoryBarrier = vks::VK_STRUCTURE_TYPE_MEMORY_BARRIER as i32,
    LoaderInstanceCreateInfo = vks::VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO as i32,
    LoaderDeviceCreateInfo = vks::VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO as i32,
    SwapchainCreateInfoKhr = vks::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR as i32,
    PresentInfoKhr = vks::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR as i32,
    DisplayModeCreateInfoKhr = vks::VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR as i32,
    DisplaySurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR as i32,
    DisplayPresentInfoKhr = vks::VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR as i32,
    XlibSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR as i32,
    XcbSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR as i32,
    WaylandSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR as i32,
    MirSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR as i32,
    AndroidSurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR as i32,
    Win32SurfaceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR as i32,
    DebugReportCreateInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT as i32,
    // VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT as i32,
    PipelineRasterizationStateRasterizationOrderAmd = vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD as i32,
    DebugMarkerObjectNameInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT as i32,
    DebugMarkerObjectTagInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT as i32,
    DebugMarkerMarkerInfoExt = vks::VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT as i32,
    DedicatedAllocationImageCreateInfoNv = vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV as i32,
    DedicatedAllocationBufferCreateInfoNv = vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV as i32,
    DedicatedAllocationMemoryAllocateInfoNv = vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV as i32,
    TextureLodGatherFormatPropertiesAmd = vks::VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD as i32,
    #[cfg(feature = "experimental")]
    RenderPassMultiviewCreateInfoKhx = vks::VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    PhysicalDeviceMultiviewFeaturesKhx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX as i32,
    #[cfg(feature = "experimental")]
    PhysicalDeviceMultiviewPropertiesKhx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX as i32,
    ExternalMemoryImageCreateInfoNv = vks::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV as i32,
    ExportMemoryAllocateInfoNv = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV as i32,
    ImportMemoryWin32HandleInfoNv = vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV as i32,
    ExportMemoryWin32HandleInfoNv = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV as i32,
    Win32KeyedMutexAcquireReleaseInfoNv = vks::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV as i32,
    PhysicalDeviceFeatures2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR as i32,
    PhysicalDeviceProperties2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR as i32,
    FormatProperties2Khr = vks::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR as i32,
    ImageFormatProperties2Khr = vks::VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR as i32,
    PhysicalDeviceImageFormatInfo2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR as i32,
    QueueFamilyProperties2Khr = vks::VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR as i32,
    PhysicalDeviceMemoryProperties2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR as i32,
    SparseImageFormatProperties2Khr = vks::VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR as i32,
    PhysicalDeviceSparseImageFormatInfo2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR as i32,
    #[cfg(feature = "experimental")]
    MemoryAllocateFlagsInfoKhx = vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    BindBufferMemoryInfoKhx = vks::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    BindImageMemoryInfoKhx = vks::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupRenderPassBeginInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupCommandBufferBeginInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupSubmitInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupBindSparseInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupPresentCapabilitiesKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX as i32,
    #[cfg(feature = "experimental")]
    ImageSwapchainCreateInfoKhx = vks::VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    BindImageMemorySwapchainInfoKhx = vks::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    AcquireNextImageInfoKhx = vks::VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupPresentInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupSwapchainCreateInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX as i32,
    ValidationFlagsExt = vks::VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT as i32,
    ViSurfaceCreateInfoNn = vks::VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN as i32,
    #[cfg(feature = "experimental")]
    PhysicalDeviceGroupPropertiesKhx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX as i32,
    #[cfg(feature = "experimental")]
    DeviceGroupDeviceCreateInfoKhx = vks::VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX as i32,
    PhysicalDeviceExternalImageFormatInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR as i32,
    ExternalImageFormatPropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR as i32,
    PhysicalDeviceExternalBufferInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR as i32,
    ExternalBufferPropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR as i32,
    PhysicalDeviceIDPropertiesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR as i32,
    ExternalMemoryBufferCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR as i32,
    ExternalMemoryImageCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR as i32,
    ExportMemoryAllocateInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR as i32,
    ImportMemoryWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR as i32,
    ExportMemoryWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR as i32,
    MemoryWin32HandlePropertiesKhr = vks::VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR as i32,
    MemoryGetWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR as i32,
    ImportMemoryFdInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR as i32,
    MemoryFdPropertiesKhr = vks::VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR as i32,
    MemoryGetFdInfoKhr = vks::VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR as i32,
    Win32KeyedMutexAcquireReleaseInfoKhr = vks::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR as i32,
    PhysicalDeviceExternalSemaphoreInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR as i32,
    ExternalSemaphorePropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR as i32,
    ExportSemaphoreCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR as i32,
    ImportSemaphoreWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR as i32,
    ExportSemaphoreWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR as i32,
    D3d12FenceSubmitInfoKhr = vks::VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR as i32,
    SemaphoreGetWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR as i32,
    ImportSemaphoreFdInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR as i32,
    SemaphoreGetFdInfoKhr = vks::VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR as i32,
    PhysicalDevicePushDescriptorPropertiesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR as i32,
    PhysicalDevice16bitStorageFeaturesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR as i32,
    PresentRegionsKhr = vks::VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR as i32,
    DescriptorUpdateTemplateCreateInfoKhr = vks::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR as i32,
    #[cfg(feature = "experimental")]
    ObjectTableCreateInfoNvx = vks::VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX as i32,
    #[cfg(feature = "experimental")]
    IndirectCommandsLayoutCreateInfoNvx = vks::VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX as i32,
    #[cfg(feature = "experimental")]
    CmdProcessCommandsInfoNvx = vks::VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX as i32,
    #[cfg(feature = "experimental")]
    CmdReserveSpaceForCommandsInfoNvx = vks::VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX as i32,
    #[cfg(feature = "experimental")]
    DeviceGeneratedCommandsLimitsNvx = vks::VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX as i32,
    #[cfg(feature = "experimental")]
    DeviceGeneratedCommandsFeaturesNvx = vks::VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX as i32,
    PipelineViewportWScalingStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV as i32,
    #[cfg(feature = "unimplemented")]
    SurfaceCapabilities2Ext = vks::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT as i32,
    // VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT = vks::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT as i32,
    DisplayPowerInfoExt = vks::VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT as i32,
    DeviceEventInfoExt = vks::VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT as i32,
    DisplayEventInfoExt = vks::VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT as i32,
    SwapchainCounterCreateInfoExt = vks::VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT as i32,
    PresentTimesInfoGoogle = vks::VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE as i32,
    #[cfg(feature = "experimental")]
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX as i32,
    PipelineViewportSwizzleStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV as i32,
    PhysicalDeviceDiscardRectanglePropertiesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT as i32,
    PipelineDiscardRectangleStateCreateInfoExt = vks::VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT as i32,
    HdrMetadataExt = vks::VK_STRUCTURE_TYPE_HDR_METADATA_EXT as i32,
    SharedPresentSurfaceCapabilitiesKhr = vks::VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR as i32,
    PhysicalDeviceExternalFenceInfoKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR as i32,
    ExternalFencePropertiesKhr = vks::VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR as i32,
    ExportFenceCreateInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR as i32,
    ImportFenceWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR as i32,
    ExportFenceWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR as i32,
    FenceGetWin32HandleInfoKhr = vks::VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR as i32,
    ImportFenceFdInfoKhr = vks::VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR as i32,
    FenceGetFdInfoKhr = vks::VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR as i32,
    PhysicalDeviceSurfaceInfo2Khr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR as i32,
    SurfaceCapabilities2Khr = vks::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR as i32,
    SurfaceFormat2Khr = vks::VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR as i32,
    PhysicalDeviceVariablePointerFeaturesKhr = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR as i32,
    IosSurfaceCreateInfoMvk = vks::VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK as i32,
    MacosSurfaceCreateInfoMvk = vks::VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK as i32,
    MemoryDedicatedRequirementsKhr = vks::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR as i32,
    MemoryDedicatedAllocateInfoKhr = vks::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR as i32,
    PhysicalDeviceSamplerFilterMinmaxPropertiesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT as i32,
    SamplerReductionModeCreateInfoExt = vks::VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT as i32,
    BufferMemoryRequirementsInfo2Khr = vks::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR as i32,
    ImageMemoryRequirementsInfo2Khr = vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR as i32,
    ImageSparseMemoryRequirementsInfo2Khr = vks::VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR as i32,
    MemoryRequirements2Khr = vks::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR as i32,
    SparseImageMemoryRequirements2Khr = vks::VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR as i32,
    PhysicalDeviceBlendOperationAdvancedFeaturesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT as i32,
    PhysicalDeviceBlendOperationAdvancedPropertiesExt = vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT as i32,
    PipelineColorBlendAdvancedStateCreateInfoExt = vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT as i32,
    PipelineCoverageToColorStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV as i32,
    PipelineCoverageModulationStateCreateInfoNv = vks::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV as i32,
}

impl From<StructureType> for i32 {
    fn from(f: StructureType) -> i32 {
        f as i32
    }
}

impl From<StructureType> for u32 {
    fn from(f: StructureType) -> u32 {
        f as u32
    }
}

impl From<u32> for StructureType {
    fn from(f: u32) -> StructureType {
        StructureType::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SystemAllocationScope {
    Command = vks::VK_SYSTEM_ALLOCATION_SCOPE_COMMAND as i32,
    Object = vks::VK_SYSTEM_ALLOCATION_SCOPE_OBJECT as i32,
    Cache = vks::VK_SYSTEM_ALLOCATION_SCOPE_CACHE as i32,
    Device = vks::VK_SYSTEM_ALLOCATION_SCOPE_DEVICE as i32,
    Instance = vks::VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE as i32,
}

impl From<SystemAllocationScope> for i32 {
    fn from(f: SystemAllocationScope) -> i32 {
        f as i32
    }
}

impl From<SystemAllocationScope> for u32 {
    fn from(f: SystemAllocationScope) -> u32 {
        f as u32
    }
}

impl From<u32> for SystemAllocationScope {
    fn from(f: u32) -> SystemAllocationScope {
        SystemAllocationScope::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum InternalAllocationType {
    Executable = vks::VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE as i32,
}

impl From<InternalAllocationType> for i32 {
    fn from(f: InternalAllocationType) -> i32 {
        f as i32
    }
}

impl From<InternalAllocationType> for u32 {
    fn from(f: InternalAllocationType) -> u32 {
        f as u32
    }
}

impl From<u32> for InternalAllocationType {
    fn from(f: u32) -> InternalAllocationType {
        InternalAllocationType::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum Format {
    Undefined = vks::VK_FORMAT_UNDEFINED as i32,
    R4G4UnormPack8 = vks::VK_FORMAT_R4G4_UNORM_PACK8 as i32,
    R4G4B4B4UnormPack16 = vks::VK_FORMAT_R4G4B4A4_UNORM_PACK16 as i32,
    B4G4R4A4UnormPack16 = vks::VK_FORMAT_B4G4R4A4_UNORM_PACK16 as i32,
    R5G6B5UnormPack16 = vks::VK_FORMAT_R5G6B5_UNORM_PACK16 as i32,
    B5G6R5UnormPack16 = vks::VK_FORMAT_B5G6R5_UNORM_PACK16 as i32,
    R5G5B5A1UnormPack16 = vks::VK_FORMAT_R5G5B5A1_UNORM_PACK16 as i32,
    B5G5R5A1UnormPack16 = vks::VK_FORMAT_B5G5R5A1_UNORM_PACK16 as i32,
    A1R5G5B5UnormPack16 = vks::VK_FORMAT_A1R5G5B5_UNORM_PACK16 as i32,
    R8Unorm = vks::VK_FORMAT_R8_UNORM as i32,
    R8Snorm = vks::VK_FORMAT_R8_SNORM as i32,
    R8Uscaled = vks::VK_FORMAT_R8_USCALED as i32,
    R8Sscaled = vks::VK_FORMAT_R8_SSCALED as i32,
    R8Uint = vks::VK_FORMAT_R8_UINT as i32,
    R8Sint = vks::VK_FORMAT_R8_SINT as i32,
    R8Srgb = vks::VK_FORMAT_R8_SRGB as i32,
    R8G8Unorm = vks::VK_FORMAT_R8G8_UNORM as i32,
    R8G8Snorm = vks::VK_FORMAT_R8G8_SNORM as i32,
    R8G8Uscaled = vks::VK_FORMAT_R8G8_USCALED as i32,
    R8G8Sscaled = vks::VK_FORMAT_R8G8_SSCALED as i32,
    R8G8Uint = vks::VK_FORMAT_R8G8_UINT as i32,
    R8G8Sint = vks::VK_FORMAT_R8G8_SINT as i32,
    R8G8Srgb = vks::VK_FORMAT_R8G8_SRGB as i32,
    R8G8B8Unorm = vks::VK_FORMAT_R8G8B8_UNORM as i32,
    R8G8B8Snorm = vks::VK_FORMAT_R8G8B8_SNORM as i32,
    R8G8B8Uscaled = vks::VK_FORMAT_R8G8B8_USCALED as i32,
    R8G8B8Sscaled = vks::VK_FORMAT_R8G8B8_SSCALED as i32,
    R8G8B8Uint = vks::VK_FORMAT_R8G8B8_UINT as i32,
    R8G8B8Sint = vks::VK_FORMAT_R8G8B8_SINT as i32,
    R8G8B8Srgb = vks::VK_FORMAT_R8G8B8_SRGB as i32,
    B8G8R8Unorm = vks::VK_FORMAT_B8G8R8_UNORM as i32,
    B8G8R8Snorm = vks::VK_FORMAT_B8G8R8_SNORM as i32,
    B8G8R8Uscaled = vks::VK_FORMAT_B8G8R8_USCALED as i32,
    B8G8R8Sscaled = vks::VK_FORMAT_B8G8R8_SSCALED as i32,
    B8G8R8Uint = vks::VK_FORMAT_B8G8R8_UINT as i32,
    B8G8R8Sint = vks::VK_FORMAT_B8G8R8_SINT as i32,
    B8G8R8Srgb = vks::VK_FORMAT_B8G8R8_SRGB as i32,
    R8G8B8A8Unorm = vks::VK_FORMAT_R8G8B8A8_UNORM as i32,
    R8G8B8A8Snorm = vks::VK_FORMAT_R8G8B8A8_SNORM as i32,
    R8G8B8A8Uscaled = vks::VK_FORMAT_R8G8B8A8_USCALED as i32,
    R8G8B8A8Sscaled = vks::VK_FORMAT_R8G8B8A8_SSCALED as i32,
    R8G8B8A8Uint = vks::VK_FORMAT_R8G8B8A8_UINT as i32,
    R8G8B8A8Sint = vks::VK_FORMAT_R8G8B8A8_SINT as i32,
    R8G8B8A8Srgb = vks::VK_FORMAT_R8G8B8A8_SRGB as i32,
    B8G8R8A8Unorm = vks::VK_FORMAT_B8G8R8A8_UNORM as i32,
    B8G8R8A8Snorm = vks::VK_FORMAT_B8G8R8A8_SNORM as i32,
    B8G8R8A8Uscaled = vks::VK_FORMAT_B8G8R8A8_USCALED as i32,
    B8G8R8A8Sscaled = vks::VK_FORMAT_B8G8R8A8_SSCALED as i32,
    B8G8R8A8Uint = vks::VK_FORMAT_B8G8R8A8_UINT as i32,
    B8G8R8A8Sint = vks::VK_FORMAT_B8G8R8A8_SINT as i32,
    B8G8R8A8Srgb = vks::VK_FORMAT_B8G8R8A8_SRGB as i32,
    A8B8G8R8UnormPack32 = vks::VK_FORMAT_A8B8G8R8_UNORM_PACK32 as i32,
    A8B8G8R8SnormPack32 = vks::VK_FORMAT_A8B8G8R8_SNORM_PACK32 as i32,
    A8B8G8R8UscaledPack32 = vks::VK_FORMAT_A8B8G8R8_USCALED_PACK32 as i32,
    A8B8G8R8SscaledPack32 = vks::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 as i32,
    A8B8G8R8UintPack32 = vks::VK_FORMAT_A8B8G8R8_UINT_PACK32 as i32,
    A8B8G8R8SintPack32 = vks::VK_FORMAT_A8B8G8R8_SINT_PACK32 as i32,
    A8B8G8R8SrgbPack32 = vks::VK_FORMAT_A8B8G8R8_SRGB_PACK32 as i32,
    A2R10G10B10UnormPack32 = vks::VK_FORMAT_A2R10G10B10_UNORM_PACK32 as i32,
    A2R10G10B10SnormPack32 = vks::VK_FORMAT_A2R10G10B10_SNORM_PACK32 as i32,
    A2R10G10B10UscaledPack32 = vks::VK_FORMAT_A2R10G10B10_USCALED_PACK32 as i32,
    A2R10G10B10SscaledPack32 = vks::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 as i32,
    A2R10G10B10UintPack32 = vks::VK_FORMAT_A2R10G10B10_UINT_PACK32 as i32,
    A2R10G10B10SintPack32 = vks::VK_FORMAT_A2R10G10B10_SINT_PACK32 as i32,
    A2B10G10R10UnormPack32 = vks::VK_FORMAT_A2B10G10R10_UNORM_PACK32 as i32,
    A2B10G10R10SnormPack32 = vks::VK_FORMAT_A2B10G10R10_SNORM_PACK32 as i32,
    A2B10G10R10UscaledPack32 = vks::VK_FORMAT_A2B10G10R10_USCALED_PACK32 as i32,
    A2B10G10R10SscaledPack32 = vks::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 as i32,
    A2B10G10R10UintPack32 = vks::VK_FORMAT_A2B10G10R10_UINT_PACK32 as i32,
    A2B10G10R10SintPack32 = vks::VK_FORMAT_A2B10G10R10_SINT_PACK32 as i32,
    R16Unorm = vks::VK_FORMAT_R16_UNORM as i32,
    R16Snorm = vks::VK_FORMAT_R16_SNORM as i32,
    R16Uscaled = vks::VK_FORMAT_R16_USCALED as i32,
    R16Sscaled = vks::VK_FORMAT_R16_SSCALED as i32,
    R16Uint = vks::VK_FORMAT_R16_UINT as i32,
    R16Sint = vks::VK_FORMAT_R16_SINT as i32,
    R16Sfloat = vks::VK_FORMAT_R16_SFLOAT as i32,
    R16G16Unorm = vks::VK_FORMAT_R16G16_UNORM as i32,
    R16G16Snorm = vks::VK_FORMAT_R16G16_SNORM as i32,
    R16G16Uscaled = vks::VK_FORMAT_R16G16_USCALED as i32,
    R16G16Sscaled = vks::VK_FORMAT_R16G16_SSCALED as i32,
    R16G16Uint = vks::VK_FORMAT_R16G16_UINT as i32,
    R16G16Sint = vks::VK_FORMAT_R16G16_SINT as i32,
    R16G16Sfloat = vks::VK_FORMAT_R16G16_SFLOAT as i32,
    R16G16B16Unorm = vks::VK_FORMAT_R16G16B16_UNORM as i32,
    R16G16B16Snorm = vks::VK_FORMAT_R16G16B16_SNORM as i32,
    R16G16B16Uscaled = vks::VK_FORMAT_R16G16B16_USCALED as i32,
    R16G16B16Sscaled = vks::VK_FORMAT_R16G16B16_SSCALED as i32,
    R16G16B16Uint = vks::VK_FORMAT_R16G16B16_UINT as i32,
    R16G16B16Sint = vks::VK_FORMAT_R16G16B16_SINT as i32,
    R16G16B16Sfloat = vks::VK_FORMAT_R16G16B16_SFLOAT as i32,
    R16G16B16A16Unorm = vks::VK_FORMAT_R16G16B16A16_UNORM as i32,
    R16G16B16A16Snorm = vks::VK_FORMAT_R16G16B16A16_SNORM as i32,
    R16G16B16A16Uscaled = vks::VK_FORMAT_R16G16B16A16_USCALED as i32,
    R16G16B16A16Sscaled = vks::VK_FORMAT_R16G16B16A16_SSCALED as i32,
    R16G16B16A16Uint = vks::VK_FORMAT_R16G16B16A16_UINT as i32,
    R16G16B16A16Sint = vks::VK_FORMAT_R16G16B16A16_SINT as i32,
    R16G16B16A16Sfloat = vks::VK_FORMAT_R16G16B16A16_SFLOAT as i32,
    R32Uint = vks::VK_FORMAT_R32_UINT as i32,
    R32Sint = vks::VK_FORMAT_R32_SINT as i32,
    R32Sfloat = vks::VK_FORMAT_R32_SFLOAT as i32,
    R32G32Uint = vks::VK_FORMAT_R32G32_UINT as i32,
    R32G32Sint = vks::VK_FORMAT_R32G32_SINT as i32,
    R32G32Sfloat = vks::VK_FORMAT_R32G32_SFLOAT as i32,
    R32G32B32Uint = vks::VK_FORMAT_R32G32B32_UINT as i32,
    R32G32B32Sint = vks::VK_FORMAT_R32G32B32_SINT as i32,
    R32G32B32Sfloat = vks::VK_FORMAT_R32G32B32_SFLOAT as i32,
    R32G32B32A32Uint = vks::VK_FORMAT_R32G32B32A32_UINT as i32,
    R32G32B32A32Sint = vks::VK_FORMAT_R32G32B32A32_SINT as i32,
    R32G32B32A32Sfloat = vks::VK_FORMAT_R32G32B32A32_SFLOAT as i32,
    R64Uint = vks::VK_FORMAT_R64_UINT as i32,
    R64Sint = vks::VK_FORMAT_R64_SINT as i32,
    R64Sfloat = vks::VK_FORMAT_R64_SFLOAT as i32,
    R64G64Uint = vks::VK_FORMAT_R64G64_UINT as i32,
    R64G64Sint = vks::VK_FORMAT_R64G64_SINT as i32,
    R64G64Sfloat = vks::VK_FORMAT_R64G64_SFLOAT as i32,
    R64G64B64Uint = vks::VK_FORMAT_R64G64B64_UINT as i32,
    R64G64B64Sint = vks::VK_FORMAT_R64G64B64_SINT as i32,
    R64G64B64Sfloat = vks::VK_FORMAT_R64G64B64_SFLOAT as i32,
    R64G64B64A64Uint = vks::VK_FORMAT_R64G64B64A64_UINT as i32,
    R64G64B64A64Sint = vks::VK_FORMAT_R64G64B64A64_SINT as i32,
    R64G64B64A64Sfloat = vks::VK_FORMAT_R64G64B64A64_SFLOAT as i32,
    B10G11R11UfloatPack32 = vks::VK_FORMAT_B10G11R11_UFLOAT_PACK32 as i32,
    E5B9G9R9UfloatPack32 = vks::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 as i32,
    D16Unorm = vks::VK_FORMAT_D16_UNORM as i32,
    X8D24UnormPack32 = vks::VK_FORMAT_X8_D24_UNORM_PACK32 as i32,
    D32Sfloat = vks::VK_FORMAT_D32_SFLOAT as i32,
    S8Uint = vks::VK_FORMAT_S8_UINT as i32,
    D16UnormS8Uint = vks::VK_FORMAT_D16_UNORM_S8_UINT as i32,
    D24UnormS8Uint = vks::VK_FORMAT_D24_UNORM_S8_UINT as i32,
    D32SfloatS8Uint = vks::VK_FORMAT_D32_SFLOAT_S8_UINT as i32,
    Bc1RgbUnormBlock = vks::VK_FORMAT_BC1_RGB_UNORM_BLOCK as i32,
    Bc1RgbSrgbBlock = vks::VK_FORMAT_BC1_RGB_SRGB_BLOCK as i32,
    Bc1RgbaUnormBlock = vks::VK_FORMAT_BC1_RGBA_UNORM_BLOCK as i32,
    Bc1RgbaSrgbBlock = vks::VK_FORMAT_BC1_RGBA_SRGB_BLOCK as i32,
    Bc2UnormBlock = vks::VK_FORMAT_BC2_UNORM_BLOCK as i32,
    Bc2SrgbBlock = vks::VK_FORMAT_BC2_SRGB_BLOCK as i32,
    Bc3UnormBlock = vks::VK_FORMAT_BC3_UNORM_BLOCK as i32,
    Bc3SrgbBlock = vks::VK_FORMAT_BC3_SRGB_BLOCK as i32,
    Bc4UnormBlock = vks::VK_FORMAT_BC4_UNORM_BLOCK as i32,
    Bc4SnormBlock = vks::VK_FORMAT_BC4_SNORM_BLOCK as i32,
    Bc5UnormBlock = vks::VK_FORMAT_BC5_UNORM_BLOCK as i32,
    Bc5SnormBlock = vks::VK_FORMAT_BC5_SNORM_BLOCK as i32,
    Bc6hUfloatBlock = vks::VK_FORMAT_BC6H_UFLOAT_BLOCK as i32,
    Bc6hSfloatBlock = vks::VK_FORMAT_BC6H_SFLOAT_BLOCK as i32,
    Bc7UnormBlock = vks::VK_FORMAT_BC7_UNORM_BLOCK as i32,
    Bc7SrgbBlock = vks::VK_FORMAT_BC7_SRGB_BLOCK as i32,
    Etc2R8G8B8UnormBlock = vks::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK as i32,
    Etc2R8G8B8SrgbBlock = vks::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK as i32,
    Etc2R8G8B8A1UnormBlock = vks::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK as i32,
    Etc2R8G8B8A1SrgbBlock = vks::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK as i32,
    Etc2R8G8B8A8UnormBlock = vks::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK as i32,
    Etc2R8G8B8A8SrgbBlock = vks::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK as i32,
    EacR11UnormBlock = vks::VK_FORMAT_EAC_R11_UNORM_BLOCK as i32,
    EacR11SnormBlock = vks::VK_FORMAT_EAC_R11_SNORM_BLOCK as i32,
    EacR11G11UnormBlock = vks::VK_FORMAT_EAC_R11G11_UNORM_BLOCK as i32,
    EacR11G11SnormBlock = vks::VK_FORMAT_EAC_R11G11_SNORM_BLOCK as i32,
    Astc4x4UnormBlock = vks::VK_FORMAT_ASTC_4x4_UNORM_BLOCK as i32,
    Astc4x4SrgbBlock = vks::VK_FORMAT_ASTC_4x4_SRGB_BLOCK as i32,
    Astc5x4UnormBlock = vks::VK_FORMAT_ASTC_5x4_UNORM_BLOCK as i32,
    Astc5x4SrgbBlock = vks::VK_FORMAT_ASTC_5x4_SRGB_BLOCK as i32,
    Astc5x5UnormBlock = vks::VK_FORMAT_ASTC_5x5_UNORM_BLOCK as i32,
    Astc5x5SrgbBlock = vks::VK_FORMAT_ASTC_5x5_SRGB_BLOCK as i32,
    Astc6x5UnormBlock = vks::VK_FORMAT_ASTC_6x5_UNORM_BLOCK as i32,
    Astc6x5SrgbBlock = vks::VK_FORMAT_ASTC_6x5_SRGB_BLOCK as i32,
    Astc6x6UnormBlock = vks::VK_FORMAT_ASTC_6x6_UNORM_BLOCK as i32,
    Astc6x6SrgbBlock = vks::VK_FORMAT_ASTC_6x6_SRGB_BLOCK as i32,
    Astc8x5UnormBlock = vks::VK_FORMAT_ASTC_8x5_UNORM_BLOCK as i32,
    Astc8x5SrgbBlock = vks::VK_FORMAT_ASTC_8x5_SRGB_BLOCK as i32,
    Astc8x6UnormBlock = vks::VK_FORMAT_ASTC_8x6_UNORM_BLOCK as i32,
    Astc8x6SrgbBlock = vks::VK_FORMAT_ASTC_8x6_SRGB_BLOCK as i32,
    Astc8x8UnormBlock = vks::VK_FORMAT_ASTC_8x8_UNORM_BLOCK as i32,
    Astc8x8SrgbBlock = vks::VK_FORMAT_ASTC_8x8_SRGB_BLOCK as i32,
    Astc10x5UnormBlock = vks::VK_FORMAT_ASTC_10x5_UNORM_BLOCK as i32,
    Astc10x5SrgbBlock = vks::VK_FORMAT_ASTC_10x5_SRGB_BLOCK as i32,
    Astc10x6UnormBlock = vks::VK_FORMAT_ASTC_10x6_UNORM_BLOCK as i32,
    Astc10x6SrgbBlock = vks::VK_FORMAT_ASTC_10x6_SRGB_BLOCK as i32,
    Astc10x8UnormBlock = vks::VK_FORMAT_ASTC_10x8_UNORM_BLOCK as i32,
    Astc10x8SrgbBlock = vks::VK_FORMAT_ASTC_10x8_SRGB_BLOCK as i32,
    Astc10x10UnormBlock = vks::VK_FORMAT_ASTC_10x10_UNORM_BLOCK as i32,
    Astc10x10SrgbBlock = vks::VK_FORMAT_ASTC_10x10_SRGB_BLOCK as i32,
    Astc12x10UnormBlock = vks::VK_FORMAT_ASTC_12x10_UNORM_BLOCK as i32,
    Astc12x10SrgbBlock = vks::VK_FORMAT_ASTC_12x10_SRGB_BLOCK as i32,
    Astc12x12UnormBlock = vks::VK_FORMAT_ASTC_12x12_UNORM_BLOCK as i32,
    Astc12x12SrgbBlock = vks::VK_FORMAT_ASTC_12x12_SRGB_BLOCK as i32,
    Pvrtc12bppUnormBlockImg = vks::VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG as i32,
    Pvrtc14bppUnormBlockImg = vks::VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG as i32,
    Pvrtc22bppUnormBlockImg = vks::VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG as i32,
    Pvrtc24bppUnormBlockImg = vks::VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG as i32,
    Pvrtc12bppSrgbBlockImg = vks::VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG as i32,
    Pvrtc14bppSrgbBlockImg = vks::VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG as i32,
    Pvrtc22bppSrgbBlockImg = vks::VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG as i32,
    Pvrtc24bppSrgbBlockImg = vks::VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG as i32,
}

impl From<Format> for i32 {
    fn from(f: Format) -> i32 {
        f as i32
    }
}

impl From<Format> for u32 {
    fn from(f: Format) -> u32 {
        f as u32
    }
}

impl From<u32> for Format {
    fn from(f: u32) -> Format {
        Format::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageType {
    Type1d = vks::VK_IMAGE_TYPE_1D as i32,
    Type2d = vks::VK_IMAGE_TYPE_2D as i32,
    Type3d = vks::VK_IMAGE_TYPE_3D as i32,
}

impl From<ImageType> for i32 {
    fn from(f: ImageType) -> i32 {
        f as i32
    }
}

impl From<ImageType> for u32 {
    fn from(f: ImageType) -> u32 {
        f as u32
    }
}

impl From<u32> for ImageType {
    fn from(f: u32) -> ImageType {
        ImageType::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageTiling {
    Optimal = vks::VK_IMAGE_TILING_OPTIMAL as i32,
    Linear = vks::VK_IMAGE_TILING_LINEAR as i32,
}

impl From<ImageTiling> for i32 {
    fn from(f: ImageTiling) -> i32 {
        f as i32
    }
}

impl From<ImageTiling> for u32 {
    fn from(f: ImageTiling) -> u32 {
        f as u32
    }
}

impl From<u32> for ImageTiling {
    fn from(f: u32) -> ImageTiling {
        ImageTiling::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PhysicalDeviceType {
    Other = vks::VK_PHYSICAL_DEVICE_TYPE_OTHER as i32,
    IntegratedGpu = vks::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU as i32,
    DiscreteGpu = vks::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU as i32,
    VirtualGpu = vks::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU as i32,
    Cpu = vks::VK_PHYSICAL_DEVICE_TYPE_CPU as i32,
}

impl From<PhysicalDeviceType> for i32 {
    fn from(f: PhysicalDeviceType) -> i32 {
        f as i32
    }
}

impl From<PhysicalDeviceType> for u32 {
    fn from(f: PhysicalDeviceType) -> u32 {
        f as u32
    }
}

impl From<u32> for PhysicalDeviceType {
    fn from(f: u32) -> PhysicalDeviceType {
        PhysicalDeviceType::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum QueryType {
    Occlusion = vks::VK_QUERY_TYPE_OCCLUSION as i32,
    PipelineStatistics = vks::VK_QUERY_TYPE_PIPELINE_STATISTICS as i32,
    Timestamp = vks::VK_QUERY_TYPE_TIMESTAMP as i32,
}

impl From<QueryType> for i32 {
    fn from(f: QueryType) -> i32 {
        f as i32
    }
}

impl From<QueryType> for u32 {
    fn from(f: QueryType) -> u32 {
        f as u32
    }
}

impl From<u32> for QueryType {
    fn from(f: u32) -> QueryType {
        QueryType::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SharingMode {
    Exclusive = vks::VK_SHARING_MODE_EXCLUSIVE as i32,
    Concurrent = vks::VK_SHARING_MODE_CONCURRENT as i32,
}

impl From<SharingMode> for i32 {
    fn from(f: SharingMode) -> i32 {
        f as i32
    }
}

impl From<SharingMode> for u32 {
    fn from(f: SharingMode) -> u32 {
        f as u32
    }
}

impl From<u32> for SharingMode {
    fn from(f: u32) -> SharingMode {
        SharingMode::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageLayout {
    Undefined = vks::VK_IMAGE_LAYOUT_UNDEFINED as i32,
    General = vks::VK_IMAGE_LAYOUT_GENERAL as i32,
    ColorAttachmentOptimal = vks::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL as i32,
    DepthStencilAttachmentOptimal = vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL as i32,
    DepthStencilReadOnlyOptimal = vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL as i32,
    ShaderReadOnlyOptimal = vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL as i32,
    TransferSrcOptimal = vks::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL as i32,
    TransferDstOptimal = vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as i32,
    Preinitialized = vks::VK_IMAGE_LAYOUT_PREINITIALIZED as i32,
    PresentSrcKhr = vks::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR as i32,
    SharedPresentKhr = vks::VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR as i32,
}

impl From<ImageLayout> for i32 {
    fn from(f: ImageLayout) -> i32 {
        f as i32
    }
}

impl From<ImageLayout> for u32 {
    fn from(f: ImageLayout) -> u32 {
        f as u32
    }
}

impl From<u32> for ImageLayout {
    fn from(f: u32) -> ImageLayout {
        ImageLayout::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ImageViewType {
    Type1d = vks::VK_IMAGE_VIEW_TYPE_1D as i32,
    Type2d = vks::VK_IMAGE_VIEW_TYPE_2D as i32,
    Type3d = vks::VK_IMAGE_VIEW_TYPE_3D as i32,
    Cube = vks::VK_IMAGE_VIEW_TYPE_CUBE as i32,
    Type1dArray = vks::VK_IMAGE_VIEW_TYPE_1D_ARRAY as i32,
    Type2dArray = vks::VK_IMAGE_VIEW_TYPE_2D_ARRAY as i32,
    CubeArray = vks::VK_IMAGE_VIEW_TYPE_CUBE_ARRAY as i32,
}

impl From<ImageViewType> for i32 {
    fn from(f: ImageViewType) -> i32 {
        f as i32
    }
}

impl From<ImageViewType> for u32 {
    fn from(f: ImageViewType) -> u32 {
        f as u32
    }
}

impl From<u32> for ImageViewType {
    fn from(f: u32) -> ImageViewType {
        ImageViewType::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ComponentSwizzle {
    Identity = vks::VK_COMPONENT_SWIZZLE_IDENTITY as i32,
    Zero = vks::VK_COMPONENT_SWIZZLE_ZERO as i32,
    One = vks::VK_COMPONENT_SWIZZLE_ONE as i32,
    R = vks::VK_COMPONENT_SWIZZLE_R as i32,
    G = vks::VK_COMPONENT_SWIZZLE_G as i32,
    B = vks::VK_COMPONENT_SWIZZLE_B as i32,
    A = vks::VK_COMPONENT_SWIZZLE_A as i32,
}

impl From<ComponentSwizzle> for i32 {
    fn from(f: ComponentSwizzle) -> i32 {
        f as i32
    }
}

impl From<ComponentSwizzle> for u32 {
    fn from(f: ComponentSwizzle) -> u32 {
        f as u32
    }
}

impl From<u32> for ComponentSwizzle {
    fn from(f: u32) -> ComponentSwizzle {
        ComponentSwizzle::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum VertexInputRate {
    Vertex = vks::VK_VERTEX_INPUT_RATE_VERTEX as i32,
    Instance = vks::VK_VERTEX_INPUT_RATE_INSTANCE as i32,
}

impl From<VertexInputRate> for i32 {
    fn from(f: VertexInputRate) -> i32 {
        f as i32
    }
}

impl From<VertexInputRate> for u32 {
    fn from(f: VertexInputRate) -> u32 {
        f as u32
    }
}

impl From<u32> for VertexInputRate {
    fn from(f: u32) -> VertexInputRate {
        VertexInputRate::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PrimitiveTopology {
    PointList = vks::VK_PRIMITIVE_TOPOLOGY_POINT_LIST as i32,
    LineList = vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST as i32,
    LineStrip = vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP as i32,
    TriangleList = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST as i32,
    TriangleStrip = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP as i32,
    TriangleFan = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN as i32,
    LineListWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY as i32,
    LineStripWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY as i32,
    TriangleListWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY as i32,
    TriangleStripWithAdjacency = vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY as i32,
    PatchList = vks::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST as i32,
}

impl From<PrimitiveTopology> for i32 {
    fn from(f: PrimitiveTopology) -> i32 {
        f as i32
    }
}

impl From<PrimitiveTopology> for u32 {
    fn from(f: PrimitiveTopology) -> u32 {
        f as u32
    }
}

impl From<u32> for PrimitiveTopology {
    fn from(f: u32) -> PrimitiveTopology {
        PrimitiveTopology::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PolygonMode {
    Fill = vks::VK_POLYGON_MODE_FILL as i32,
    Line = vks::VK_POLYGON_MODE_LINE as i32,
    Point = vks::VK_POLYGON_MODE_POINT as i32,
    FillRectangleNv = vks::VK_POLYGON_MODE_FILL_RECTANGLE_NV as i32,
}

impl From<PolygonMode> for i32 {
    fn from(f: PolygonMode) -> i32 {
        f as i32
    }
}

impl From<PolygonMode> for u32 {
    fn from(f: PolygonMode) -> u32 {
        f as u32
    }
}

impl From<u32> for PolygonMode {
    fn from(f: u32) -> PolygonMode {
        PolygonMode::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum FrontFace {
    CounterClockwise = vks::VK_FRONT_FACE_COUNTER_CLOCKWISE as i32,
    Clockwise = vks::VK_FRONT_FACE_CLOCKWISE as i32,
}

impl From<FrontFace> for i32 {
    fn from(f: FrontFace) -> i32 {
        f as i32
    }
}

impl From<FrontFace> for u32 {
    fn from(f: FrontFace) -> u32 {
        f as u32
    }
}

impl From<u32> for FrontFace {
    fn from(f: u32) -> FrontFace {
        FrontFace::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum CompareOp {
    Never = vks::VK_COMPARE_OP_NEVER as i32,
    Less = vks::VK_COMPARE_OP_LESS as i32,
    Equal = vks::VK_COMPARE_OP_EQUAL as i32,
    LessOrEqual = vks::VK_COMPARE_OP_LESS_OR_EQUAL as i32,
    Greater = vks::VK_COMPARE_OP_GREATER as i32,
    NotEqual = vks::VK_COMPARE_OP_NOT_EQUAL as i32,
    GreaterOrEqual = vks::VK_COMPARE_OP_GREATER_OR_EQUAL as i32,
    Always = vks::VK_COMPARE_OP_ALWAYS as i32,
}

impl From<CompareOp> for i32 {
    fn from(f: CompareOp) -> i32 {
        f as i32
    }
}

impl From<CompareOp> for u32 {
    fn from(f: CompareOp) -> u32 {
        f as u32
    }
}

impl From<u32> for CompareOp {
    fn from(f: u32) -> CompareOp {
        CompareOp::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum StencilOp {
    Keep = vks::VK_STENCIL_OP_KEEP as i32,
    Zero = vks::VK_STENCIL_OP_ZERO as i32,
    Replace = vks::VK_STENCIL_OP_REPLACE as i32,
    IncrementAndClamp = vks::VK_STENCIL_OP_INCREMENT_AND_CLAMP as i32,
    DecrementAndClamp = vks::VK_STENCIL_OP_DECREMENT_AND_CLAMP as i32,
    Invert = vks::VK_STENCIL_OP_INVERT as i32,
    IncrementAndWrap = vks::VK_STENCIL_OP_INCREMENT_AND_WRAP as i32,
    DecrementAndWrap = vks::VK_STENCIL_OP_DECREMENT_AND_WRAP as i32,
}

impl From<StencilOp> for i32 {
    fn from(f: StencilOp) -> i32 {
        f as i32
    }
}

impl From<StencilOp> for u32 {
    fn from(f: StencilOp) -> u32 {
        f as u32
    }
}

impl From<u32> for StencilOp {
    fn from(f: u32) -> StencilOp {
        StencilOp::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum LogicOp {
    Clear = vks::VK_LOGIC_OP_CLEAR as i32,
    And = vks::VK_LOGIC_OP_AND as i32,
    AndReverse = vks::VK_LOGIC_OP_AND_REVERSE as i32,
    Copy = vks::VK_LOGIC_OP_COPY as i32,
    AndInverted = vks::VK_LOGIC_OP_AND_INVERTED as i32,
    NoOp = vks::VK_LOGIC_OP_NO_OP as i32,
    Xor = vks::VK_LOGIC_OP_XOR as i32,
    Or = vks::VK_LOGIC_OP_OR as i32,
    Nor = vks::VK_LOGIC_OP_NOR as i32,
    Equivalent = vks::VK_LOGIC_OP_EQUIVALENT as i32,
    Invert = vks::VK_LOGIC_OP_INVERT as i32,
    OrReverse = vks::VK_LOGIC_OP_OR_REVERSE as i32,
    CopyInverted = vks::VK_LOGIC_OP_COPY_INVERTED as i32,
    OrInverted = vks::VK_LOGIC_OP_OR_INVERTED as i32,
    Nand = vks::VK_LOGIC_OP_NAND as i32,
    Set = vks::VK_LOGIC_OP_SET as i32,
}

impl From<LogicOp> for i32 {
    fn from(f: LogicOp) -> i32 {
        f as i32
    }
}

impl From<LogicOp> for u32 {
    fn from(f: LogicOp) -> u32 {
        f as u32
    }
}

impl From<u32> for LogicOp {
    fn from(f: u32) -> LogicOp {
        LogicOp::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum BlendFactor {
    Zero = vks::VK_BLEND_FACTOR_ZERO as i32,
    One = vks::VK_BLEND_FACTOR_ONE as i32,
    SrcColor = vks::VK_BLEND_FACTOR_SRC_COLOR as i32,
    OneMinusSrcColor = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR as i32,
    DstColor = vks::VK_BLEND_FACTOR_DST_COLOR as i32,
    OneMinusDstColor = vks::VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR as i32,
    SrcAlpha = vks::VK_BLEND_FACTOR_SRC_ALPHA as i32,
    OneMinusSrcAlpha = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA as i32,
    DstAlpha = vks::VK_BLEND_FACTOR_DST_ALPHA as i32,
    OneMinusDstAlpha = vks::VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA as i32,
    ConstantColor = vks::VK_BLEND_FACTOR_CONSTANT_COLOR as i32,
    OneMinusConstantColor = vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR as i32,
    ConstantAlpha = vks::VK_BLEND_FACTOR_CONSTANT_ALPHA as i32,
    OneMinusConstantAlpha = vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA as i32,
    SrcAlphaSaturate = vks::VK_BLEND_FACTOR_SRC_ALPHA_SATURATE as i32,
    Src1Color = vks::VK_BLEND_FACTOR_SRC1_COLOR as i32,
    OneMinusSrc1Color = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR as i32,
    Src1Alpha = vks::VK_BLEND_FACTOR_SRC1_ALPHA as i32,
    OneMinusSrc1Alpha = vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA as i32,
}

impl From<BlendFactor> for i32 {
    fn from(f: BlendFactor) -> i32 {
        f as i32
    }
}

impl From<BlendFactor> for u32 {
    fn from(f: BlendFactor) -> u32 {
        f as u32
    }
}

impl From<u32> for BlendFactor {
    fn from(f: u32) -> BlendFactor {
        BlendFactor::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum BlendOp {
    Add = vks::VK_BLEND_OP_ADD as i32,
    Subtract = vks::VK_BLEND_OP_SUBTRACT as i32,
    ReverseSubtract = vks::VK_BLEND_OP_REVERSE_SUBTRACT as i32,
    Min = vks::VK_BLEND_OP_MIN as i32,
    Max = vks::VK_BLEND_OP_MAX as i32,
    ZeroExt = vks::VK_BLEND_OP_ZERO_EXT as i32,
    SrcExt = vks::VK_BLEND_OP_SRC_EXT as i32,
    DstExt = vks::VK_BLEND_OP_DST_EXT as i32,
    SrcOverExt = vks::VK_BLEND_OP_SRC_OVER_EXT as i32,
    DstOverExt = vks::VK_BLEND_OP_DST_OVER_EXT as i32,
    SrcInExt = vks::VK_BLEND_OP_SRC_IN_EXT as i32,
    DstInExt = vks::VK_BLEND_OP_DST_IN_EXT as i32,
    SrcOutExt = vks::VK_BLEND_OP_SRC_OUT_EXT as i32,
    DstOutExt = vks::VK_BLEND_OP_DST_OUT_EXT as i32,
    SrcAtopExt = vks::VK_BLEND_OP_SRC_ATOP_EXT as i32,
    DstAtopExt = vks::VK_BLEND_OP_DST_ATOP_EXT as i32,
    XorExt = vks::VK_BLEND_OP_XOR_EXT as i32,
    MultiplyExt = vks::VK_BLEND_OP_MULTIPLY_EXT as i32,
    ScreenExt = vks::VK_BLEND_OP_SCREEN_EXT as i32,
    OverlayExt = vks::VK_BLEND_OP_OVERLAY_EXT as i32,
    DarkenExt = vks::VK_BLEND_OP_DARKEN_EXT as i32,
    LightenExt = vks::VK_BLEND_OP_LIGHTEN_EXT as i32,
    ColordodgeExt = vks::VK_BLEND_OP_COLORDODGE_EXT as i32,
    ColorburnExt = vks::VK_BLEND_OP_COLORBURN_EXT as i32,
    HardlightExt = vks::VK_BLEND_OP_HARDLIGHT_EXT as i32,
    SoftlightExt = vks::VK_BLEND_OP_SOFTLIGHT_EXT as i32,
    DifferenceExt = vks::VK_BLEND_OP_DIFFERENCE_EXT as i32,
    ExclusionExt = vks::VK_BLEND_OP_EXCLUSION_EXT as i32,
    InvertExt = vks::VK_BLEND_OP_INVERT_EXT as i32,
    InvertRgbExt = vks::VK_BLEND_OP_INVERT_RGB_EXT as i32,
    LineardodgeExt = vks::VK_BLEND_OP_LINEARDODGE_EXT as i32,
    LinearburnExt = vks::VK_BLEND_OP_LINEARBURN_EXT as i32,
    VividlightExt = vks::VK_BLEND_OP_VIVIDLIGHT_EXT as i32,
    LinearlightExt = vks::VK_BLEND_OP_LINEARLIGHT_EXT as i32,
    PinlightExt = vks::VK_BLEND_OP_PINLIGHT_EXT as i32,
    HardmixExt = vks::VK_BLEND_OP_HARDMIX_EXT as i32,
    HslHueExt = vks::VK_BLEND_OP_HSL_HUE_EXT as i32,
    HslSaturationExt = vks::VK_BLEND_OP_HSL_SATURATION_EXT as i32,
    HslColorExt = vks::VK_BLEND_OP_HSL_COLOR_EXT as i32,
    HslLuminosityExt = vks::VK_BLEND_OP_HSL_LUMINOSITY_EXT as i32,
    PlusExt = vks::VK_BLEND_OP_PLUS_EXT as i32,
    PlusClampedExt = vks::VK_BLEND_OP_PLUS_CLAMPED_EXT as i32,
    PlusClampedAlphaExt = vks::VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT as i32,
    PlusDarkerExt = vks::VK_BLEND_OP_PLUS_DARKER_EXT as i32,
    MinusExt = vks::VK_BLEND_OP_MINUS_EXT as i32,
    MinusClampedExt = vks::VK_BLEND_OP_MINUS_CLAMPED_EXT as i32,
    ContrastExt = vks::VK_BLEND_OP_CONTRAST_EXT as i32,
    InvertOvgExt = vks::VK_BLEND_OP_INVERT_OVG_EXT as i32,
    RedExt = vks::VK_BLEND_OP_RED_EXT as i32,
    GreenExt = vks::VK_BLEND_OP_GREEN_EXT as i32,
    BlueExt = vks::VK_BLEND_OP_BLUE_EXT as i32,
}

impl From<BlendOp> for i32 {
    fn from(f: BlendOp) -> i32 {
        f as i32
    }
}

impl From<BlendOp> for u32 {
    fn from(f: BlendOp) -> u32 {
        f as u32
    }
}

impl From<u32> for BlendOp {
    fn from(f: u32) -> BlendOp {
        BlendOp::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum DynamicState {
    Viewport = vks::VK_DYNAMIC_STATE_VIEWPORT as i32,
    Scissor = vks::VK_DYNAMIC_STATE_SCISSOR as i32,
    LineWidth = vks::VK_DYNAMIC_STATE_LINE_WIDTH as i32,
    DepthBias = vks::VK_DYNAMIC_STATE_DEPTH_BIAS as i32,
    BlendConstants = vks::VK_DYNAMIC_STATE_BLEND_CONSTANTS as i32,
    DepthBounds = vks::VK_DYNAMIC_STATE_DEPTH_BOUNDS as i32,
    StencilCompareMask = vks::VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK as i32,
    StencilWriteMask = vks::VK_DYNAMIC_STATE_STENCIL_WRITE_MASK as i32,
    StencilReference = vks::VK_DYNAMIC_STATE_STENCIL_REFERENCE as i32,
    ViewportWScalingNv = vks::VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV as i32,
    DiscardRectangleExt = vks::VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT as i32,
}

impl From<DynamicState> for i32 {
    fn from(f: DynamicState) -> i32 {
        f as i32
    }
}

impl From<DynamicState> for u32 {
    fn from(f: DynamicState) -> u32 {
        f as u32
    }
}

impl From<u32> for DynamicState {
    fn from(f: u32) -> DynamicState {
        DynamicState::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum Filter {
    Nearest = vks::VK_FILTER_NEAREST as i32,
    Linear = vks::VK_FILTER_LINEAR as i32,
    CubicImg = vks::VK_FILTER_CUBIC_IMG as i32,
}

impl From<Filter> for i32 {
    fn from(f: Filter) -> i32 {
        f as i32
    }
}

impl From<Filter> for u32 {
    fn from(f: Filter) -> u32 {
        f as u32
    }
}

impl From<u32> for Filter {
    fn from(f: u32) -> Filter {
        Filter::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SamplerMipmapMode {
    Nearest = vks::VK_SAMPLER_MIPMAP_MODE_NEAREST as i32,
    Linear = vks::VK_SAMPLER_MIPMAP_MODE_LINEAR as i32,
}

impl From<SamplerMipmapMode> for i32 {
    fn from(f: SamplerMipmapMode) -> i32 {
        f as i32
    }
}

impl From<SamplerMipmapMode> for u32 {
    fn from(f: SamplerMipmapMode) -> u32 {
        f as u32
    }
}

impl From<u32> for SamplerMipmapMode {
    fn from(f: u32) -> SamplerMipmapMode {
        SamplerMipmapMode::from_u32(f).unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SamplerAddressMode {
    Repeat = vks::VK_SAMPLER_ADDRESS_MODE_REPEAT as i32,
    MirroredRepeat = vks::VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as i32,
    ClampToEdge = vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE as i32,
    ClampToBorder = vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as i32,
    MirrorClampToEdge = vks::VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE as i32,
}

impl From<SamplerAddressMode> for i32 {
    fn from(f: SamplerAddressMode) -> i32 {
        f as i32
    }
}

impl From<SamplerAddressMode> for u32 {
    fn from(f: SamplerAddressMode) -> u32 {
        f as u32
    }
}

impl From<u32> for SamplerAddressMode {
    fn from(f: u32) -> SamplerAddressMode {
        SamplerAddressMode::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum BorderColor {
    FloatTransparentBlack = vks::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK as i32,
    IntTransparentBlack = vks::VK_BORDER_COLOR_INT_TRANSPARENT_BLACK as i32,
    FloatOpaqueBlack = vks::VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK as i32,
    IntOpaqueBlack = vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK as i32,
    FloatOpaqueWhite = vks::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE as i32,
    IntOpaqueWhite = vks::VK_BORDER_COLOR_INT_OPAQUE_WHITE as i32,
}

impl From<BorderColor> for i32 {
    fn from(f: BorderColor) -> i32 {
        f as i32
    }
}

impl From<BorderColor> for u32 {
    fn from(f: BorderColor) -> u32 {
        f as u32
    }
}

impl From<u32> for BorderColor {
    fn from(f: u32) -> BorderColor {
        BorderColor::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum DescriptorType {
    Sampler = vks::VK_DESCRIPTOR_TYPE_SAMPLER as i32,
    CombinedImageSampler = vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER as i32,
    SampledImage = vks::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE as i32,
    StorageImage = vks::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE as i32,
    UniformTexelBuffer = vks::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER as i32,
    StorageTexelBuffer = vks::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER as i32,
    UniformBuffer = vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER as i32,
    StorageBuffer = vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER as i32,
    UniformBufferDynamic = vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC as i32,
    StorageBufferDynamic = vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC as i32,
    InputAttachment = vks::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as i32,
}

impl From<DescriptorType> for i32 {
    fn from(f: DescriptorType) -> i32 {
        f as i32
    }
}

impl From<DescriptorType> for u32 {
    fn from(f: DescriptorType) -> u32 {
        f as u32
    }
}

impl From<u32> for DescriptorType {
    fn from(f: u32) -> DescriptorType {
        DescriptorType::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum AttachmentLoadOp {
    Load = vks::VK_ATTACHMENT_LOAD_OP_LOAD as i32,
    Clear = vks::VK_ATTACHMENT_LOAD_OP_CLEAR as i32,
    DontCare = vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE as i32,
}

impl From<AttachmentLoadOp> for i32 {
    fn from(f: AttachmentLoadOp) -> i32 {
        f as i32
    }
}

impl From<AttachmentLoadOp> for u32 {
    fn from(f: AttachmentLoadOp) -> u32 {
        f as u32
    }
}

impl From<u32> for AttachmentLoadOp {
    fn from(f: u32) -> AttachmentLoadOp {
        AttachmentLoadOp::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum AttachmentStoreOp {
    Store = vks::VK_ATTACHMENT_STORE_OP_STORE as i32,
    DontCare = vks::VK_ATTACHMENT_STORE_OP_DONT_CARE as i32,
}

impl From<AttachmentStoreOp> for i32 {
    fn from(f: AttachmentStoreOp) -> i32 {
        f as i32
    }
}

impl From<AttachmentStoreOp> for u32 {
    fn from(f: AttachmentStoreOp) -> u32 {
        f as u32
    }
}

impl From<u32> for AttachmentStoreOp {
    fn from(f: u32) -> AttachmentStoreOp {
        AttachmentStoreOp::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum PipelineBindPoint {
    Graphics = vks::VK_PIPELINE_BIND_POINT_GRAPHICS as i32,
    Compute = vks::VK_PIPELINE_BIND_POINT_COMPUTE as i32,
}

impl From<PipelineBindPoint> for i32 {
    fn from(f: PipelineBindPoint) -> i32 {
        f as i32
    }
}

impl From<PipelineBindPoint> for u32 {
    fn from(f: PipelineBindPoint) -> u32 {
        f as u32
    }
}

impl From<u32> for PipelineBindPoint {
    fn from(f: u32) -> PipelineBindPoint {
        PipelineBindPoint::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum CommandBufferLevel {
    Primary = vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY as i32,
    Secondary = vks::VK_COMMAND_BUFFER_LEVEL_SECONDARY as i32,
}

impl From<CommandBufferLevel> for i32 {
    fn from(f: CommandBufferLevel) -> i32 {
        f as i32
    }
}

impl From<CommandBufferLevel> for u32 {
    fn from(f: CommandBufferLevel) -> u32 {
        f as u32
    }
}

impl From<u32> for CommandBufferLevel {
    fn from(f: u32) -> CommandBufferLevel {
        CommandBufferLevel::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum IndexType {
    Uint16 = vks::VK_INDEX_TYPE_UINT16 as i32,
    Uint32 = vks::VK_INDEX_TYPE_UINT32 as i32,
}

impl From<IndexType> for i32 {
    fn from(f: IndexType) -> i32 {
        f as i32
    }
}

impl From<IndexType> for u32 {
    fn from(f: IndexType) -> u32 {
        f as u32
    }
}

impl From<u32> for IndexType {
    fn from(f: u32) -> IndexType {
        IndexType::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum SubpassContents {
    Inline = vks::VK_SUBPASS_CONTENTS_INLINE as i32,
    SecondaryCommandBuffers = vks::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS as i32,
}

impl From<SubpassContents> for i32 {
    fn from(f: SubpassContents) -> i32 {
        f as i32
    }
}

impl From<SubpassContents> for u32 {
    fn from(f: SubpassContents) -> u32 {
        f as u32
    }
}

impl From<u32> for SubpassContents {
    fn from(f: u32) -> SubpassContents {
        SubpassContents::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive, Hash)]
pub enum ObjectType {
    Unknown = vks::VK_OBJECT_TYPE_UNKNOWN as i32,
    Instance = vks::VK_OBJECT_TYPE_INSTANCE as i32,
    PhysicalDevice = vks::VK_OBJECT_TYPE_PHYSICAL_DEVICE as i32,
    Device = vks::VK_OBJECT_TYPE_DEVICE as i32,
    Queue = vks::VK_OBJECT_TYPE_QUEUE as i32,
    Semaphore = vks::VK_OBJECT_TYPE_SEMAPHORE as i32,
    CommandBuffer = vks::VK_OBJECT_TYPE_COMMAND_BUFFER as i32,
    Fence = vks::VK_OBJECT_TYPE_FENCE as i32,
    DeviceMemory = vks::VK_OBJECT_TYPE_DEVICE_MEMORY as i32,
    Buffer = vks::VK_OBJECT_TYPE_BUFFER as i32,
    Image = vks::VK_OBJECT_TYPE_IMAGE as i32,
    Event = vks::VK_OBJECT_TYPE_EVENT as i32,
    QueryPool = vks::VK_OBJECT_TYPE_QUERY_POOL as i32,
    BufferView = vks::VK_OBJECT_TYPE_BUFFER_VIEW as i32,
    ImageView = vks::VK_OBJECT_TYPE_IMAGE_VIEW as i32,
    ShaderModule = vks::VK_OBJECT_TYPE_SHADER_MODULE as i32,
    PipelineCache = vks::VK_OBJECT_TYPE_PIPELINE_CACHE as i32,
    PipelineLayout = vks::VK_OBJECT_TYPE_PIPELINE_LAYOUT as i32,
    RenderPass = vks::VK_OBJECT_TYPE_RENDER_PASS as i32,
    Pipeline = vks::VK_OBJECT_TYPE_PIPELINE as i32,
    DescriptorSetLayout = vks::VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT as i32,
    Sampler = vks::VK_OBJECT_TYPE_SAMPLER as i32,
    DescriptorPool = vks::VK_OBJECT_TYPE_DESCRIPTOR_POOL as i32,
    DescriptorSet = vks::VK_OBJECT_TYPE_DESCRIPTOR_SET as i32,
    Framebuffer = vks::VK_OBJECT_TYPE_FRAMEBUFFER as i32,
    CommandPool = vks::VK_OBJECT_TYPE_COMMAND_POOL as i32,
    SurfaceKhr = vks::VK_OBJECT_TYPE_SURFACE_KHR as i32,
    SwapchainKhr = vks::VK_OBJECT_TYPE_SWAPCHAIN_KHR as i32,
    DisplayKhr = vks::VK_OBJECT_TYPE_DISPLAY_KHR as i32,
    DisplayModeKhr = vks::VK_OBJECT_TYPE_DISPLAY_MODE_KHR as i32,
    DebugReportCallbackExt = vks::VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT as i32,
    DescriptorUpdateTemplateKhr = 1000085000 as i32,
    #[cfg(feature = "experimental")]
    ObjectTableNvx = vks::VK_OBJECT_TYPE_OBJECT_TABLE_NVX as i32,
    #[cfg(feature = "experimental")]
    IndirectCommandsLayoutNvx = vks::VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX as i32,
}

impl From<ObjectType> for i32 {
    fn from(f: ObjectType) -> i32 {
        f as i32
    }
}

impl From<ObjectType> for u32 {
    fn from(f: ObjectType) -> u32 {
        f as u32
    }
}

impl From<u32> for ObjectType {
    fn from(f: u32) -> ObjectType {
        ObjectType::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ColorSpaceKhr {
    SrgbNonlinearKhr = vks::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR as i32,
    DisplayP3NonlinearExt = vks::VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT as i32,
    ExtendedSrgbLinearExt = vks::VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as i32,
    DciP3LinearExt = vks::VK_COLOR_SPACE_DCI_P3_LINEAR_EXT as i32,
    DciP3NonlinearExt = vks::VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT as i32,
    Bt709LinearExt = vks::VK_COLOR_SPACE_BT709_LINEAR_EXT as i32,
    Bt709NonlinearExt = vks::VK_COLOR_SPACE_BT709_NONLINEAR_EXT as i32,
    Bt2020LinearExt = vks::VK_COLOR_SPACE_BT2020_LINEAR_EXT as i32,
    Hdr10St2084Ext = vks::VK_COLOR_SPACE_HDR10_ST2084_EXT as i32,
    DolbyvisionExt = vks::VK_COLOR_SPACE_DOLBYVISION_EXT as i32,
    Hdr10HlgExt = vks::VK_COLOR_SPACE_HDR10_HLG_EXT as i32,
    AdobergbLinearExt = vks::VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT as i32,
    AdobergbNonlinearExt = vks::VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT as i32,
    PassThroughExt = vks::VK_COLOR_SPACE_PASS_THROUGH_EXT as i32,
    ExtendedSrgbNonlinearExt = vks::VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeKhr = vks::VK_COLOR_SPACE_RANGE_SIZE_KHR as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumKhr = vks::VK_COLOR_SPACE_MAX_ENUM_KHR as i32,
}

impl From<ColorSpaceKhr> for i32 {
    fn from(f: ColorSpaceKhr) -> i32 {
        f as i32
    }
}

impl From<ColorSpaceKhr> for u32 {
    fn from(f: ColorSpaceKhr) -> u32 {
        f as u32
    }
}

impl From<u32> for ColorSpaceKhr {
    fn from(f: u32) -> ColorSpaceKhr {
        ColorSpaceKhr::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum PresentModeKhr {
    ImmediateKhr = vks::VK_PRESENT_MODE_IMMEDIATE_KHR as i32,
    MailboxKhr = vks::VK_PRESENT_MODE_MAILBOX_KHR as i32,
    FifoKhr = vks::VK_PRESENT_MODE_FIFO_KHR as i32,
    FifoRelaxedKhr = vks::VK_PRESENT_MODE_FIFO_RELAXED_KHR as i32,
    SharedDemandRefreshKhr = vks::VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR as i32,
    SharedContinuousRefreshKhr = vks::VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeKhr = vks::VK_PRESENT_MODE_RANGE_SIZE_KHR as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumKhr = vks::VK_PRESENT_MODE_MAX_ENUM_KHR as i32,
}

impl From<PresentModeKhr> for i32 {
    fn from(f: PresentModeKhr) -> i32 {
        f as i32
    }
}

impl From<PresentModeKhr> for u32 {
    fn from(f: PresentModeKhr) -> u32 {
        f as u32
    }
}

impl From<u32> for PresentModeKhr {
    fn from(f: u32) -> PresentModeKhr {
        PresentModeKhr::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DescriptorUpdateTemplateTypeKhr {
    DescriptorSetKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR as i32,
    PushDescriptorsKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_RANGE_SIZE_KHR as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_MAX_ENUM_KHR as i32,
}

impl From<DescriptorUpdateTemplateTypeKhr> for i32 {
    fn from(f: DescriptorUpdateTemplateTypeKhr) -> i32 {
        f as i32
    }
}

impl From<DescriptorUpdateTemplateTypeKhr> for u32 {
    fn from(f: DescriptorUpdateTemplateTypeKhr) -> u32 {
        f as u32
    }
}

impl From<u32> for DescriptorUpdateTemplateTypeKhr {
    fn from(f: u32) -> DescriptorUpdateTemplateTypeKhr {
        DescriptorUpdateTemplateTypeKhr::from_u32(f).unwrap()
    }
}

pub type DescriptorUpdateTemplateCreateFlagsKHR = i32;

#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum PointClippingBehaviorKhr {
    AllClipPlanesKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR as i32,
    UserClipPlanesOnlyKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR as i32,
    RangeSizeKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_MAX_ENUM_KHR as i32,
}

#[cfg(feature = "unimplemented")]
impl From<PointClippingBehaviorKhr> for i32 {
    fn from(f: PointClippingBehaviorKhr) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<PointClippingBehaviorKhr> for u32 {
    fn from(f: PointClippingBehaviorKhr) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for PointClippingBehaviorKhr {
    fn from(f: u32) -> PointClippingBehaviorKhr {
        PointClippingBehaviorKhr::from_u32(f).unwrap()
    }
}


#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum TessellationDomainOriginKhr {
    UpperLeftKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR as i32,
    LowerLeftKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR as i32,
    RangeSizeKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_MAX_ENUM_KHR as i32,
}

#[cfg(feature = "unimplemented")]
impl From<TessellationDomainOriginKhr> for i32 {
    fn from(f: TessellationDomainOriginKhr) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<TessellationDomainOriginKhr> for u32 {
    fn from(f: TessellationDomainOriginKhr) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for TessellationDomainOriginKhr {
    fn from(f: u32) -> TessellationDomainOriginKhr {
        TessellationDomainOriginKhr::from_u32(f).unwrap()
    }
}


#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum SamplerYcbcrModelConversionKhr {
    RgbIdentityKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR as i32,
    YcbcrIdentityKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR as i32,
    Ycbcr709Khr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR as i32,
    Ycbcr601Khr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR as i32,
    Ycbcr2020Khr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR as i32,
    RangeSizeKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_MAX_ENUM_KHR as i32,
}

#[cfg(feature = "unimplemented")]
impl From<SamplerYcbcrModelConversionKhr> for i32 {
    fn from(f: SamplerYcbcrModelConversionKhr) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<SamplerYcbcrModelConversionKhr> for u32 {
    fn from(f: SamplerYcbcrModelConversionKhr) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for SamplerYcbcrModelConversionKhr {
    fn from(f: u32) -> SamplerYcbcrModelConversionKhr {
        SamplerYcbcrModelConversionKhr::from_u32(f).unwrap()
    }
}


#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum SamplerYcbcrRangeKhr {
    ItuFullKhr = vks::VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR as i32,
    ItuNarrowKhr = vks::VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR as i32,
    RangeSizeKhr = vks::VK_SAMPLER_YCBCR_RANGE_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_SAMPLER_YCBCR_RANGE_MAX_ENUM_KHR as i32,
}

#[cfg(feature = "unimplemented")]
impl From<SamplerYcbcrRangeKhr> for i32 {
    fn from(f: SamplerYcbcrRangeKhr) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<SamplerYcbcrRangeKhr> for u32 {
    fn from(f: SamplerYcbcrRangeKhr) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for SamplerYcbcrRangeKhr {
    fn from(f: u32) -> SamplerYcbcrRangeKhr {
        SamplerYcbcrRangeKhr::from_u32(f).unwrap()
    }
}


#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ChromaLocationKhr {
    CositedEvenKhr = vks::VK_CHROMA_LOCATION_COSITED_EVEN_KHR as i32,
    MidpointKhr = vks::VK_CHROMA_LOCATION_MIDPOINT_KHR as i32,
    RangeSizeKhr = vks::VK_CHROMA_LOCATION_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_CHROMA_LOCATION_MAX_ENUM_KHR as i32,
}

#[cfg(feature = "unimplemented")]
impl From<ChromaLocationKhr> for i32 {
    fn from(f: ChromaLocationKhr) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<ChromaLocationKhr> for u32 {
    fn from(f: ChromaLocationKhr) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for ChromaLocationKhr {
    fn from(f: u32) -> ChromaLocationKhr {
        ChromaLocationKhr::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DebugReportObjectTypeExt {
    UnknownExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT as i32,
    InstanceExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT as i32,
    PhysicalDeviceExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT as i32,
    DeviceExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT as i32,
    QueueExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT as i32,
    SemaphoreExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT as i32,
    CommandBufferExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT as i32,
    FenceExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT as i32,
    DeviceMemoryExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT as i32,
    BufferExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT as i32,
    ImageExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT as i32,
    EventExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT as i32,
    QueryPoolExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT as i32,
    BufferViewExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT as i32,
    ImageViewExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT as i32,
    ShaderModuleExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT as i32,
    PipelineCacheExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT as i32,
    PipelineLayoutExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT as i32,
    RenderPassExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT as i32,
    PipelineExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT as i32,
    DescriptorSetLayoutExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT as i32,
    SamplerExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT as i32,
    DescriptorPoolExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT as i32,
    DescriptorSetExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT as i32,
    FramebufferExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT as i32,
    CommandPoolExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT as i32,
    SurfaceKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT as i32,
    SwapchainKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT as i32,
    DebugReportCallbackExtExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT as i32,
    DisplayKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT as i32,
    DisplayModeKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT as i32,
    #[cfg(feature = "experimental")]
    ObjectTableNvxExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT as i32,
    #[cfg(feature = "experimental")]
    IndirectCommandsLayoutNvxExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT as i32,
    #[cfg(feature = "unimplemented")]
    ValidationCacheExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT as i32,
    DescriptorUpdateTemplateKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT as i32,
    #[cfg(feature = "unimplemented")]
    SamplerYcbcrConversionKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_MAX_ENUM_EXT as i32,
}

impl From<DebugReportObjectTypeExt> for i32 {
    fn from(f: DebugReportObjectTypeExt) -> i32 {
        f as i32
    }
}

impl From<DebugReportObjectTypeExt> for u32 {
    fn from(f: DebugReportObjectTypeExt) -> u32 {
        f as u32
    }
}

impl From<u32> for DebugReportObjectTypeExt {
    fn from(f: u32) -> DebugReportObjectTypeExt {
        DebugReportObjectTypeExt::from_u32(f).unwrap()
    }
}



#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum RasterizationOrderAmd {
    StrictAmd = vks::VK_RASTERIZATION_ORDER_STRICT_AMD as i32,
    RelaxedAmd = vks::VK_RASTERIZATION_ORDER_RELAXED_AMD as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeAmd = vks::VK_RASTERIZATION_ORDER_RANGE_SIZE_AMD as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumAmd = vks::VK_RASTERIZATION_ORDER_MAX_ENUM_AMD as i32,
}

impl From<RasterizationOrderAmd> for i32 {
    fn from(f: RasterizationOrderAmd) -> i32 {
        f as i32
    }
}

impl From<RasterizationOrderAmd> for u32 {
    fn from(f: RasterizationOrderAmd) -> u32 {
        f as u32
    }
}

impl From<u32> for RasterizationOrderAmd {
    fn from(f: u32) -> RasterizationOrderAmd {
        RasterizationOrderAmd::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ValidationCheckExt {
    AllExt = vks::VK_VALIDATION_CHECK_ALL_EXT as i32,
    ShadersExt = vks::VK_VALIDATION_CHECK_SHADERS_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_VALIDATION_CHECK_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_VALIDATION_CHECK_MAX_ENUM_EXT as i32,
}

impl From<ValidationCheckExt> for i32 {
    fn from(f: ValidationCheckExt) -> i32 {
        f as i32
    }
}

impl From<ValidationCheckExt> for u32 {
    fn from(f: ValidationCheckExt) -> u32 {
        f as u32
    }
}

impl From<u32> for ValidationCheckExt {
    fn from(f: u32) -> ValidationCheckExt {
        ValidationCheckExt::from_u32(f).unwrap()
    }
}


#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum IndirectCommandsTokenTypeNvx {
    PipelineNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX as i32,
    DescriptorSetNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX as i32,
    IndexBufferNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX as i32,
    VertexBufferNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX as i32,
    PushConstantNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX as i32,
    DrawIndexedNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX as i32,
    DrawNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NVX as i32,
    DispatchNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX as i32,
    RangeSizeNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_RANGE_SIZE_NVX as i32,
    MaxEnumNvx = vks::VK_INDIRECT_COMMANDS_TOKEN_TYPE_MAX_ENUM_NVX as i32,
}

#[cfg(feature = "unimplemented")]
impl From<IndirectCommandsTokenTypeNvx> for i32 {
    fn from(f: IndirectCommandsTokenTypeNvx) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<IndirectCommandsTokenTypeNvx> for u32 {
    fn from(f: IndirectCommandsTokenTypeNvx) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for IndirectCommandsTokenTypeNvx {
    fn from(f: u32) -> IndirectCommandsTokenTypeNvx {
        IndirectCommandsTokenTypeNvx::from_u32(f).unwrap()
    }
}


#[cfg(feature = "experimental")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ObjectEntryTypeNvx {
    DescriptorSetNvx = vks::VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX as i32,
    PipelineNvx = vks::VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX as i32,
    IndexBufferNvx = vks::VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX as i32,
    VertexBufferNvx = vks::VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX as i32,
    PushConstantNvx = vks::VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeNvx = vks::VK_OBJECT_ENTRY_TYPE_RANGE_SIZE_NVX as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumNvx = vks::VK_OBJECT_ENTRY_TYPE_MAX_ENUM_NVX as i32,
}

#[cfg(feature = "experimental")]
impl From<ObjectEntryTypeNvx> for i32 {
    fn from(f: ObjectEntryTypeNvx) -> i32 {
        f as i32
    }
}

#[cfg(feature = "experimental")]
impl From<ObjectEntryTypeNvx> for u32 {
    fn from(f: ObjectEntryTypeNvx) -> u32 {
        f as u32
    }
}

#[cfg(feature = "experimental")]
impl From<u32> for ObjectEntryTypeNvx {
    fn from(f: u32) -> ObjectEntryTypeNvx {
        ObjectEntryTypeNvx::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DisplayPowerStateExt {
    OffExt = vks::VK_DISPLAY_POWER_STATE_OFF_EXT as i32,
    SuspendExt = vks::VK_DISPLAY_POWER_STATE_SUSPEND_EXT as i32,
    OnExt = vks::VK_DISPLAY_POWER_STATE_ON_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_DISPLAY_POWER_STATE_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_DISPLAY_POWER_STATE_MAX_ENUM_EXT as i32,
}

impl From<DisplayPowerStateExt> for i32 {
    fn from(f: DisplayPowerStateExt) -> i32 {
        f as i32
    }
}

impl From<DisplayPowerStateExt> for u32 {
    fn from(f: DisplayPowerStateExt) -> u32 {
        f as u32
    }
}

impl From<u32> for DisplayPowerStateExt {
    fn from(f: u32) -> DisplayPowerStateExt {
        DisplayPowerStateExt::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DeviceEventTypeExt {
    DisplayHotplugExt = vks::VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_DEVICE_EVENT_TYPE_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_DEVICE_EVENT_TYPE_MAX_ENUM_EXT as i32,
}

impl From<DeviceEventTypeExt> for i32 {
    fn from(f: DeviceEventTypeExt) -> i32 {
        f as i32
    }
}

impl From<DeviceEventTypeExt> for u32 {
    fn from(f: DeviceEventTypeExt) -> u32 {
        f as u32
    }
}

impl From<u32> for DeviceEventTypeExt {
    fn from(f: u32) -> DeviceEventTypeExt {
        DeviceEventTypeExt::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DisplayEventTypeExt {
    FirstPixelOutExt = vks::VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_DISPLAY_EVENT_TYPE_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_DISPLAY_EVENT_TYPE_MAX_ENUM_EXT as i32,
}

impl From<DisplayEventTypeExt> for i32 {
    fn from(f: DisplayEventTypeExt) -> i32 {
        f as i32
    }
}

impl From<DisplayEventTypeExt> for u32 {
    fn from(f: DisplayEventTypeExt) -> u32 {
        f as u32
    }
}

impl From<u32> for DisplayEventTypeExt {
    fn from(f: u32) -> DisplayEventTypeExt {
        DisplayEventTypeExt::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ViewportCoordinateSwizzleNv {
    PositiveXNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV as i32,
    NegativeXNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV as i32,
    PositiveYNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV as i32,
    NegativeYNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV as i32,
    PositiveZNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV as i32,
    NegativeZNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV as i32,
    PositiveWNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV as i32,
    NegativeWNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_RANGE_SIZE_NV as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_MAX_ENUM_NV as i32,
}

impl From<ViewportCoordinateSwizzleNv> for i32 {
    fn from(f: ViewportCoordinateSwizzleNv) -> i32 {
        f as i32
    }
}

impl From<ViewportCoordinateSwizzleNv> for u32 {
    fn from(f: ViewportCoordinateSwizzleNv) -> u32 {
        f as u32
    }
}

impl From<u32> for ViewportCoordinateSwizzleNv {
    fn from(f: u32) -> ViewportCoordinateSwizzleNv {
        ViewportCoordinateSwizzleNv::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DiscardRectangleModeExt {
    InclusiveExt = vks::VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT as i32,
    ExclusiveExt = vks::VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_DISCARD_RECTANGLE_MODE_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_DISCARD_RECTANGLE_MODE_MAX_ENUM_EXT as i32,
}

impl From<DiscardRectangleModeExt> for i32 {
    fn from(f: DiscardRectangleModeExt) -> i32 {
        f as i32
    }
}

impl From<DiscardRectangleModeExt> for u32 {
    fn from(f: DiscardRectangleModeExt) -> u32 {
        f as u32
    }
}

impl From<u32> for DiscardRectangleModeExt {
    fn from(f: u32) -> DiscardRectangleModeExt {
        DiscardRectangleModeExt::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum SamplerReductionModeExt {
    WeightedAverageExt = vks::VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT as i32,
    MinExt = vks::VK_SAMPLER_REDUCTION_MODE_MIN_EXT as i32,
    MaxExt = vks::VK_SAMPLER_REDUCTION_MODE_MAX_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_SAMPLER_REDUCTION_MODE_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_SAMPLER_REDUCTION_MODE_MAX_ENUM_EXT as i32,
}

impl From<SamplerReductionModeExt> for i32 {
    fn from(f: SamplerReductionModeExt) -> i32 {
        f as i32
    }
}

impl From<SamplerReductionModeExt> for u32 {
    fn from(f: SamplerReductionModeExt) -> u32 {
        f as u32
    }
}

impl From<u32> for SamplerReductionModeExt {
    fn from(f: u32) -> SamplerReductionModeExt {
        SamplerReductionModeExt::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum BlendOverlapExt {
    UncorrelatedExt = vks::VK_BLEND_OVERLAP_UNCORRELATED_EXT as i32,
    DisjointExt = vks::VK_BLEND_OVERLAP_DISJOINT_EXT as i32,
    ConjointExt = vks::VK_BLEND_OVERLAP_CONJOINT_EXT as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeExt = vks::VK_BLEND_OVERLAP_RANGE_SIZE_EXT as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumExt = vks::VK_BLEND_OVERLAP_MAX_ENUM_EXT as i32,
}

impl From<BlendOverlapExt> for i32 {
    fn from(f: BlendOverlapExt) -> i32 {
        f as i32
    }
}

impl From<BlendOverlapExt> for u32 {
    fn from(f: BlendOverlapExt) -> u32 {
        f as u32
    }
}

impl From<u32> for BlendOverlapExt {
    fn from(f: u32) -> BlendOverlapExt {
        BlendOverlapExt::from_u32(f).unwrap()
    }
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum CoverageModulationModeNv {
    NoneNv = vks::VK_COVERAGE_MODULATION_MODE_NONE_NV as i32,
    RgbNv = vks::VK_COVERAGE_MODULATION_MODE_RGB_NV as i32,
    AlphaNv = vks::VK_COVERAGE_MODULATION_MODE_ALPHA_NV as i32,
    RgbaNv = vks::VK_COVERAGE_MODULATION_MODE_RGBA_NV as i32,
    #[cfg(feature = "unimplemented")]
    RangeSizeNv = vks::VK_COVERAGE_MODULATION_MODE_RANGE_SIZE_NV as i32,
    #[cfg(feature = "unimplemented")]
    MaxEnumNv = vks::VK_COVERAGE_MODULATION_MODE_MAX_ENUM_NV as i32,
}

impl From<CoverageModulationModeNv> for i32 {
    fn from(f: CoverageModulationModeNv) -> i32 {
        f as i32
    }
}

impl From<CoverageModulationModeNv> for u32 {
    fn from(f: CoverageModulationModeNv) -> u32 {
        f as u32
    }
}

impl From<u32> for CoverageModulationModeNv {
    fn from(f: u32) -> CoverageModulationModeNv {
        CoverageModulationModeNv::from_u32(f).unwrap()
    }
}


#[cfg(feature = "unimplemented")]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ValidationCacheHeaderVersionExt {
    OneExt = vks::VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT as i32,
    MaxEnumExt = vks::VK_VALIDATION_CACHE_HEADER_VERSION_MAX_ENUM_EXT as i32,
}

#[cfg(feature = "unimplemented")]
impl From<ValidationCacheHeaderVersionExt> for i32 {
    fn from(f: ValidationCacheHeaderVersionExt) -> i32 {
        f as i32
    }
}

#[cfg(feature = "unimplemented")]
impl From<ValidationCacheHeaderVersionExt> for u32 {
    fn from(f: ValidationCacheHeaderVersionExt) -> u32 {
        f as u32
    }
}

#[cfg(feature = "unimplemented")]
impl From<u32> for ValidationCacheHeaderVersionExt {
    fn from(f: u32) -> ValidationCacheHeaderVersionExt {
        ValidationCacheHeaderVersionExt::from_u32(f).unwrap()
    }
}
