bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkSurfaceTransformFlagBitsKHR: u32 {
        const IDENTITY = 1,
        const ROTATE_90 = 2,
        const ROTATE_180 = 4,
        const ROTATE_270 = 8,
        const HORIZONTAL_MIRROR = 16,
        const HORIZONTAL_MIRROR_ROTATE_90 = 32,
        const HORIZONTAL_MIRROR_ROTATE_180 = 64,
        const HORIZONTAL_MIRROR_ROTATE_270 = 128,
        const INHERIT = 256,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkSurfaceTransformFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkCompositeAlphaFlagBitsKHR: u32 {
        const OPAQUE = 1,
        const PRE_MULTIPLIED = 2,
        const POST_MULTIPLIED = 4,
        const INHERIT = 8,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkCompositeAlphaFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkSwapchainCreateFlagBitsKHR: u32 {
        const BIND_SFR = 1,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkSwapchainCreateFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDisplayPlaneAlphaFlagBitsKHR: u32 {
        const OPAQUE = 1,
        const GLOBAL = 2,
        const PER_PIXEL = 4,
        const PER_PIXEL_PREMULTIPLIED = 8,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkDisplayPlaneAlphaFlagsKHR = u32;
pub type VkDisplayModeCreateFlagsKHR = u32;
pub type VkDisplaySurfaceCreateFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkSwapchainCreateFlagBitsKHR: u32 {
        const BIND_SFR = 1,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
    pub type VkSwapchainCreateFlagsKHR = u32;
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDisplayPlaneAlphaFlagBitsKHR: u32 {
        const OPAQUE = 1,
        const GLOBAL = 2,
        const PER_PIXEL = 4,
        const PER_PIXEL_PREMULTIPLIED = 8,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkDisplayPlaneAlphaFlagsKHR = u32;
pub type VkDisplayModeCreateFlagsKHR = u32;
pub type VkDisplaySurfaceCreateFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalMemoryHandleTypeFlagBitsKHR: u32 {
        const OPAQUE_FD = 1,
        const OPAQUE_WIN32 = 2,
        const OPAQUE_WIN32_KMT = 4,
        const D3D11_TEXTURE = 8,
        const D3D11_TEXTURE_KMT = 16,
        const D3D12_HEAP = 32,
        const D3D12_RESOURCE = 64,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkExternalMemoryHandleTypeFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalMemoryFeatureFlagBitsKHR: u32 {
        const DEDICATED_ONLY = 1,
        const EXPORTABLE = 2,
        const IMPORTABLE = 4,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkExternalMemoryFeatureFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalSemaphoreHandleTypeFlagBitsKHR: u32 {
        const OPAQUE_FD = 1,
        const OPAQUE_WIN32 = 2,
        const OPAQUE_WIN32_KMT = 4,
        const D3D12_FENCE = 8,
        const SYNC_FD = 16,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkExternalSemaphoreHandleTypeFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalSemaphoreFeatureFlagBitsKHR: u32 {
        const EXPORTABLE = 1,
        const IMPORTABLE = 2,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkExternalSemaphoreFeatureFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkSemaphoreImportFlagBitsKHR: u32 {
        const TEMPORARY = 1,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}

pub type VkSemaphoreImportFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalFenceHandleTypeFlagBitsKHR: u32 {
        const OPAQUE_FD = 1,
        const OPAQUE_WIN32 = 2,
        const OPAQUE_WIN32_KMT = 4,
        const SYNC_FD = 8,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}
pub type VkExternalFenceHandleTypeFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalFenceFeatureFlagBitsKHR: u32 {
        const EXPORTABLE = 1,
        const IMPORTABLE = 2,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}
pub type VkExternalFenceFeatureFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkFenceImportFlagBitsKHR: u32 {
        const TEMPORARY = 1,
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647,
    }
}
pub type VkFenceImportFlagsKHR = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDebugReportFlagBitsEXT: u32 {
        const INFORMATION = 1,
        const WARNING = 2,
        const PERFORMANCE_WARNING = 4,
        const ERROR = 8,
        const DEBUG = 16,
        const FLAG_BITS_MAX_ENUM_EXT = 2147483647,
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalMemoryHandleTypeFlagBitsNV: u32 {
        const OPAQUE_WIN32 = 1,
        const OPAQUE_WIN32_KMT = 2,
        const D3D11_IMAGE = 4,
        const D3D11_IMAGE_KMT = 8,
        const FLAG_BITS_MAX_ENUM_NV = 2147483647,
    }
}
pub type VkExternalMemoryHandleTypeFlagsNV = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkExternalMemoryFeatureFlagBitsNV: u32 {
        const DEDICATED_ONLY = 1,
        const EXPORTABLE = 2,
        const IMPORTABLE = 4,
        const FLAG_BITS_MAX_ENUM_NV = 2147483647,
    }
}
pub type VkExternalMemoryFeatureFlagsNV = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkPeerMemoryFeatureFlagBitsKHX: u32 {
        const COPY_SRC = 1,
        const COPY_DST = 2,
        const GENERIC_SRC = 4,
        const GENERIC_DST = 8,
        const FLAG_BITS_MAX_ENUM_KHX = 2147483647,
    }
}

pub type VkPeerMemoryFeatureFlagsKHX = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkMemoryAllocateFlagBitsKHX: u32 {
        const DEVICE_MASK = 1,
        const FLAG_BITS_MAX_ENUM_KHX = 2147483647,
    }
}

pub type VkMemoryAllocateFlagsKHX = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkDeviceGroupPresentModeFlagBitsKHX: u32 {
        const LOCAL = 1,
        const REMOTE = 2,
        const SUM = 4,
        const LOCAL_MULTI_DEVICE = 8,
        const FLAG_BITS_MAX_ENUM_KHX = 2147483647,
    }
}
pub type VkDeviceGroupPresentModeFlagsKHX = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkIndirectCommandsLayoutUsageFlagBitsNVX: u32 {
        const UNORDERED_SEQUENCES = 1,
        const SPARSE_SEQUENCES = 2,
        const EMPTY_EXECUTIONS = 4,
        const INDEXED_SEQUENCES = 8,
        const FLAG_BITS_MAX_ENUM_NVX = 2147483647,
    }
}

pub type VkIndirectCommandsLayoutUsageFlagsNVX = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkObjectEntryUsageFlagBitsNVX: u32 {
        const GRAPHICS = 1,
        const COMPUTE = 2,
        const FLAG_BITS_MAX_ENUM_NVX = 2147483647,
    }
}

pub type VkObjectEntryUsageFlagsNVX = u32;

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct VkSurfaceCounterFlagBitsEXT: u32 {
        const VBLANK_EXT = 1,
        const FLAG_BITS_MAX_ENUM_EXT = 2147483647,
    }
}

pub type VkSurfaceCounterFlagsEXT = u32;