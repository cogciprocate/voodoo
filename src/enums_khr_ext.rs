

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ColorSpaceKHR {
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
    RangeSizeKhr = vks::VK_COLOR_SPACE_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_COLOR_SPACE_MAX_ENUM_KHR as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum PresentModeKHR {
    ImmediateKhr = vks::VK_PRESENT_MODE_IMMEDIATE_KHR as i32,
    MailboxKhr = vks::VK_PRESENT_MODE_MAILBOX_KHR as i32,
    FifoKhr = vks::VK_PRESENT_MODE_FIFO_KHR as i32,
    FifoRelaxedKhr = vks::VK_PRESENT_MODE_FIFO_RELAXED_KHR as i32,
    SharedDemandRefreshKhr = vks::VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR as i32,
    SharedContinuousRefreshKhr = vks::VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR as i32,
    RangeSizeKhr = vks::VK_PRESENT_MODE_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_PRESENT_MODE_MAX_ENUM_KHR as i32,
}




#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DescriptorUpdateTemplateTypeKHR {
    DescriptorSetKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR as i32,
    PushDescriptorsKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR as i32,
    RangeSizeKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_MAX_ENUM_KHR as i32,
}
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkFlags;



#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum PointClippingBehaviorKHR {
    AllClipPlanesKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR as i32,
    UserClipPlanesOnlyKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR as i32,
    RangeSizeKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_POINT_CLIPPING_BEHAVIOR_MAX_ENUM_KHR as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum TessellationDomainOriginKHR {
    UpperLeftKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR as i32,
    LowerLeftKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR as i32,
    RangeSizeKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_TESSELLATION_DOMAIN_ORIGIN_MAX_ENUM_KHR as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum SamplerYcbcrModelConversionKHR {
    RgbIdentityKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR as i32,
    YcbcrIdentityKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR as i32,
    Ycbcr709Khr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR as i32,
    Ycbcr601Khr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR as i32,
    Ycbcr2020Khr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR as i32,
    RangeSizeKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_SAMPLER_YCBCR_MODEL_CONVERSION_MAX_ENUM_KHR as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum SamplerYcbcrRangeKHR {
    ItuFullKhr = vks::VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR as i32,
    ItuNarrowKhr = vks::VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR as i32,
    RangeSizeKhr = vks::VK_SAMPLER_YCBCR_RANGE_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_SAMPLER_YCBCR_RANGE_MAX_ENUM_KHR as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ChromaLocationKHR {
    CositedEvenKhr = vks::VK_CHROMA_LOCATION_COSITED_EVEN_KHR as i32,
    MidpointKhr = vks::VK_CHROMA_LOCATION_MIDPOINT_KHR as i32,
    RangeSizeKhr = vks::VK_CHROMA_LOCATION_RANGE_SIZE_KHR as i32,
    MaxEnumKhr = vks::VK_CHROMA_LOCATION_MAX_ENUM_KHR as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DebugReportObjectTypeEXT {
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
    ObjectTableNvxExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT as i32,
    IndirectCommandsLayoutNvxExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT as i32,
    ValidationCacheExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT as i32,
    DescriptorUpdateTemplateKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT as i32,
    SamplerYcbcrConversionKhrExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT as i32,
    RangeSizeExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_DEBUG_REPORT_OBJECT_TYPE_MAX_ENUM_EXT as i32,
}



#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum RasterizationOrderAMD {
    StrictAmd = vks::VK_RASTERIZATION_ORDER_STRICT_AMD as i32,
    RelaxedAmd = vks::VK_RASTERIZATION_ORDER_RELAXED_AMD as i32,
    RangeSizeAmd = vks::VK_RASTERIZATION_ORDER_RANGE_SIZE_AMD as i32,
    MaxEnumAmd = vks::VK_RASTERIZATION_ORDER_MAX_ENUM_AMD as i32,
}



#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ValidationCheckEXT {
    AllExt = vks::VK_VALIDATION_CHECK_ALL_EXT as i32,
    ShadersExt = vks::VK_VALIDATION_CHECK_SHADERS_EXT as i32,
    RangeSizeExt = vks::VK_VALIDATION_CHECK_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_VALIDATION_CHECK_MAX_ENUM_EXT as i32,
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum IndirectCommandsTokenTypeNVX {
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

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ObjectEntryTypeNVX {
    DescriptorSetNvx = vks::VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX as i32,
    PipelineNvx = vks::VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX as i32,
    IndexBufferNvx = vks::VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX as i32,
    VertexBufferNvx = vks::VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX as i32,
    PushConstantNvx = vks::VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX as i32,
    RangeSizeNvx = vks::VK_OBJECT_ENTRY_TYPE_RANGE_SIZE_NVX as i32,
    MaxEnumNvx = vks::VK_OBJECT_ENTRY_TYPE_MAX_ENUM_NVX as i32,
}


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DisplayPowerStateEXT {
    OffExt = vks::VK_DISPLAY_POWER_STATE_OFF_EXT as i32,
    SuspendExt = vks::VK_DISPLAY_POWER_STATE_SUSPEND_EXT as i32,
    OnExt = vks::VK_DISPLAY_POWER_STATE_ON_EXT as i32,
    RangeSizeExt = vks::VK_DISPLAY_POWER_STATE_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_DISPLAY_POWER_STATE_MAX_ENUM_EXT as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DeviceEventTypeEXT {
    DisplayHotplugExt = vks::VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT as i32,
    RangeSizeExt = vks::VK_DEVICE_EVENT_TYPE_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_DEVICE_EVENT_TYPE_MAX_ENUM_EXT as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DisplayEventTypeEXT {
    FirstPixelOutExt = vks::VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT as i32,
    RangeSizeExt = vks::VK_DISPLAY_EVENT_TYPE_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_DISPLAY_EVENT_TYPE_MAX_ENUM_EXT as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ViewportCoordinateSwizzleNV {
    PositiveXNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV as i32,
    NegativeXNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV as i32,
    PositiveYNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV as i32,
    NegativeYNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV as i32,
    PositiveZNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV as i32,
    NegativeZNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV as i32,
    PositiveWNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV as i32,
    NegativeWNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV as i32,
    RangeSizeNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_RANGE_SIZE_NV as i32,
    MaxEnumNv = vks::VK_VIEWPORT_COORDINATE_SWIZZLE_MAX_ENUM_NV as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum DiscardRectangleModeEXT {
    InclusiveExt = vks::VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT as i32,
    ExclusiveExt = vks::VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT as i32,
    RangeSizeExt = vks::VK_DISCARD_RECTANGLE_MODE_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_DISCARD_RECTANGLE_MODE_MAX_ENUM_EXT as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum SamplerReductionModeEXT {
    WeightedAverageExt = vks::VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT as i32,
    MinExt = vks::VK_SAMPLER_REDUCTION_MODE_MIN_EXT as i32,
    MaxExt = vks::VK_SAMPLER_REDUCTION_MODE_MAX_EXT as i32,
    RangeSizeExt = vks::VK_SAMPLER_REDUCTION_MODE_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_SAMPLER_REDUCTION_MODE_MAX_ENUM_EXT as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum BlendOverlapEXT {
    UncorrelatedExt = vks::VK_BLEND_OVERLAP_UNCORRELATED_EXT as i32,
    DisjointExt = vks::VK_BLEND_OVERLAP_DISJOINT_EXT as i32,
    ConjointExt = vks::VK_BLEND_OVERLAP_CONJOINT_EXT as i32,
    RangeSizeExt = vks::VK_BLEND_OVERLAP_RANGE_SIZE_EXT as i32,
    MaxEnumExt = vks::VK_BLEND_OVERLAP_MAX_ENUM_EXT as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum CoverageModulationModeNV {
    NoneNv = vks::VK_COVERAGE_MODULATION_MODE_NONE_NV as i32,
    RgbNv = vks::VK_COVERAGE_MODULATION_MODE_RGB_NV as i32,
    AlphaNv = vks::VK_COVERAGE_MODULATION_MODE_ALPHA_NV as i32,
    RgbaNv = vks::VK_COVERAGE_MODULATION_MODE_RGBA_NV as i32,
    RangeSizeNv = vks::VK_COVERAGE_MODULATION_MODE_RANGE_SIZE_NV as i32,
    MaxEnumNv = vks::VK_COVERAGE_MODULATION_MODE_MAX_ENUM_NV as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive, Hash)]
pub enum ValidationCacheHeaderVersionEXT {
    OneExt = vks::VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT as i32,
    MaxEnumExt = vks::VK_VALIDATION_CACHE_HEADER_VERSION_MAX_ENUM_EXT as i32,
}