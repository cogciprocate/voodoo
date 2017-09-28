#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkSurfaceTransformFlagBitsKHR {
    VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 1,
    VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 2,
    VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 4,
    VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 8,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 16,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 32,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 64,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 128,
    VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 256,
    VK_SURFACE_TRANSFORM_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkSurfaceTransformFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkCompositeAlphaFlagBitsKHR {
    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 1,
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 2,
    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 4,
    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 8,
    VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkCompositeAlphaFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkSwapchainCreateFlagBitsKHR {
    VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX = 1,
    VK_SWAPCHAIN_CREATE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkSwapchainCreateFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkDisplayPlaneAlphaFlagBitsKHR {
    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 1,
    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 2,
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 4,
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8,
    VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkSwapchainCreateFlagBitsKHR {
    VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX = 1,
    VK_SWAPCHAIN_CREATE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkSwapchainCreateFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkDisplayPlaneAlphaFlagBitsKHR {
    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 1,
    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 2,
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 4,
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8,
    VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalMemoryHandleTypeFlagBitsKHR {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = 1,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = 2,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = 4,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR = 8,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR = 16,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR = 32,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR = 64,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkExternalMemoryHandleTypeFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalMemoryFeatureFlagBitsKHR {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR = 1,
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR = 2,
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR = 4,
    VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkExternalMemoryFeatureFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalSemaphoreHandleTypeFlagBitsKHR {
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = 1,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = 2,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = 4,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR = 8,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR = 16,
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalSemaphoreFeatureFlagBitsKHR {
    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR = 1,
    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR = 2,
    VK_EXTERNAL_SEMAPHORE_FEATURE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkExternalSemaphoreFeatureFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkSemaphoreImportFlagBitsKHR {
    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR = 1,
    VK_SEMAPHORE_IMPORT_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkSemaphoreImportFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalFenceHandleTypeFlagBitsKHR {
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR = 1,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR = 2,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR = 4,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR = 8,
    VK_EXTERNAL_FENCE_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkExternalFenceHandleTypeFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalFenceFeatureFlagBitsKHR {
    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR = 1,
    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR = 2,
    VK_EXTERNAL_FENCE_FEATURE_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkExternalFenceFeatureFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkFenceImportFlagBitsKHR {
    VK_FENCE_IMPORT_TEMPORARY_BIT_KHR = 1,
    VK_FENCE_IMPORT_FLAG_BITS_MAX_ENUM_KHR = 2147483647,
}
pub type VkFenceImportFlagsKHR = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkDebugReportFlagBitsEXT {
    VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 1,
    VK_DEBUG_REPORT_WARNING_BIT_EXT = 2,
    VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 4,
    VK_DEBUG_REPORT_ERROR_BIT_EXT = 8,
    VK_DEBUG_REPORT_DEBUG_BIT_EXT = 16,
    VK_DEBUG_REPORT_FLAG_BITS_MAX_ENUM_EXT = 2147483647,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalMemoryHandleTypeFlagBitsNV {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 1,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 2,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 4,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 8,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_NV = 2147483647,
}
pub type VkExternalMemoryHandleTypeFlagsNV = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkExternalMemoryFeatureFlagBitsNV {
    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 1,
    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 2,
    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 4,
    VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_NV = 2147483647,
}
pub type VkExternalMemoryFeatureFlagsNV = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkPeerMemoryFeatureFlagBitsKHX {
    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHX = 1,
    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHX = 2,
    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHX = 4,
    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHX = 8,
    VK_PEER_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_KHX = 2147483647,
}
pub type VkPeerMemoryFeatureFlagsKHX = VkFlags;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkMemoryAllocateFlagBitsKHX {
    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHX = 1,
    VK_MEMORY_ALLOCATE_FLAG_BITS_MAX_ENUM_KHX = 2147483647,
}
pub type VkMemoryAllocateFlagsKHX = VkFlags;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkDeviceGroupPresentModeFlagBitsKHX {
    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX = 1,
    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX = 2,
    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX = 4,
    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX = 8,
    VK_DEVICE_GROUP_PRESENT_MODE_FLAG_BITS_MAX_ENUM_KHX = 2147483647,
}
pub type VkDeviceGroupPresentModeFlagsKHX = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkIndirectCommandsLayoutUsageFlagBitsNVX {
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX = 1,
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX = 2,
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX = 4,
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX = 8,
    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_FLAG_BITS_MAX_ENUM_NVX = 2147483647,
}
pub type VkIndirectCommandsLayoutUsageFlagsNVX = VkFlags;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkObjectEntryUsageFlagBitsNVX {
    VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX = 1,
    VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX = 2,
    VK_OBJECT_ENTRY_USAGE_FLAG_BITS_MAX_ENUM_NVX = 2147483647,
}
pub type VkObjectEntryUsageFlagsNVX = VkFlags;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VkSurfaceCounterFlagBitsEXT {
    VK_SURFACE_COUNTER_VBLANK_EXT = 1,
    VK_SURFACE_COUNTER_FLAG_BITS_MAX_ENUM_EXT = 2147483647,
}
pub type VkSurfaceCounterFlagsEXT = VkFlags;