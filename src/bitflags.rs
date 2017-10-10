

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct InstanceCreateFlags: u32 {
        const CREATE_FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct FormatFeatureFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const SAMPLED_IMAGE = 0x00000001;
        const STORAGE_IMAGE = 0x00000002;
        const STORAGE_IMAGE_ATOMIC = 0x00000004;
        const UNIFORM_TEXEL_BUFFER = 0x00000008;
        const STORAGE_TEXEL_BUFFER = 0x00000010;
        const STORAGE_TEXEL_BUFFER_ATOMIC = 0x00000020;
        const VERTEX_BUFFER = 0x00000040;
        const COLOR_ATTACHMENT = 0x00000080;
        const COLOR_ATTACHMENT_BLEND = 0x00000100;
        const DEPTH_STENCIL_ATTACHMENT = 0x00000200;
        const BLIT_SRC = 0x00000400;
        const BLIT_DST = 0x00000800;
        const SAMPLED_IMAGE_FILTER_LINEAR = 0x00001000;
        const SAMPLED_IMAGE_FILTER_CUBIC_IMG = 0x00002000;
        const TRANSFER_SRC_KHR = 0x00004000;
        const TRANSFER_DST_KHR = 0x00008000;
        const SAMPLED_IMAGE_FILTER_MINMAX_EXT = 0x00010000;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ImageUsageFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const TRANSFER_SRC = 0x00000001;
        const TRANSFER_DST = 0x00000002;
        const SAMPLED = 0x00000004;
        const STORAGE = 0x00000008;
        const COLOR_ATTACHMENT = 0x00000010;
        const DEPTH_STENCIL_ATTACHMENT = 0x00000020;
        const TRANSIENT_ATTACHMENT = 0x00000040;
        const INPUT_ATTACHMENT = 0x00000080;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ImageCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const SPARSE_BINDING = 0x00000001;
        const SPARSE_RESIDENCY = 0x00000002;
        const SPARSE_ALIASED = 0x00000004;
        const MUTABLE_FORMAT = 0x00000008;
        const CUBE_COMPATIBLE = 0x00000010;

        #[cfg(feature = "experimental")]
        const IMAGE_CREATE_BIND_SFR = 0x00000040;
        const IMAGE_CREATE_2D_ARRAY_COMPATIBLE_KHR = 0x00000020;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SampleCountFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const COUNT_1 = 0x00000001;
        const COUNT_2 = 0x00000002;
        const COUNT_4 = 0x00000004;
        const COUNT_8 = 0x00000008;
        const COUNT_16 = 0x00000010;
        const COUNT_32 = 0x00000020;
        const COUNT_64 = 0x00000040;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct QueueFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const GRAPHICS = 0x00000001;
        const COMPUTE = 0x00000002;
        const TRANSFER = 0x00000004;
        const SPARSE_BINDING = 0x00000008;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryPropertyFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const DEVICE_LOCAL = 0x00000001;
        const HOST_VISIBLE = 0x00000002;
        const HOST_COHERENT = 0x00000004;
        const HOST_CACHED = 0x00000008;
        const LAZILY_ALLOCATED = 0x00000010;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryHeapFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const DEVICE_LOCAL = 0x00000001;

        #[cfg(feature = "experimental")]
        const MULTI_INSTANCE = 0x00000002;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DeviceCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DeviceQueueCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineStageFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const TOP_OF_PIPE = 0x00000001;
        const DRAW_INDIRECT = 0x00000002;
        const VERTEX_INPUT = 0x00000004;
        const VERTEX_SHADER = 0x00000008;
        const TESSELLATION_CONTROL_SHADER = 0x00000010;
        const TESSELLATION_EVALUATION_SHADER = 0x00000020;
        const GEOMETRY_SHADER = 0x00000040;
        const FRAGMENT_SHADER = 0x00000080;
        const EARLY_FRAGMENT_TESTS = 0x00000100;
        const LATE_FRAGMENT_TESTS = 0x00000200;
        const COLOR_ATTACHMENT_OUTPUT = 0x00000400;
        const COMPUTE_SHADER = 0x00000800;
        const TRANSFER = 0x00001000;
        const BOTTOM_OF_PIPE = 0x00002000;
        const HOST = 0x00004000;
        const ALL_GRAPHICS = 0x00008000;
        const ALL_COMMANDS = 0x00010000;

        #[cfg(feature = "experimental")]
        const COMMAND_PROCESS = 0x00020000;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryMapFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ImageAspectFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const COLOR = 0x00000001;
        const DEPTH = 0x00000002;
        const STENCIL = 0x00000004;
        const METADATA = 0x00000008;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SparseImageFormatFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const SINGLE_MIPTAIL = 0x00000001;
        const ALIGNED_MIP_SIZE = 0x00000002;
        const NONSTANDARD_BLOCK_SIZE = 0x00000004;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SparseMemoryBindFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const METADATA = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct FenceCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const SIGNALED = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SemaphoreCreateFlags: u32 {
        const SEMAPHORE_CREATE_FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct EventCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct QueryPoolCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct QueryPipelineStatisticFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const INPUT_ASSEMBLY_VERTICES = 0x00000001;
        const INPUT_ASSEMBLY_PRIMITIVES = 0x00000002;
        const VERTEX_SHADER_INVOCATIONS = 0x00000004;
        const GEOMETRY_SHADER_INVOCATIONS = 0x00000008;
        const GEOMETRY_SHADER_PRIMITIVES = 0x00000010;
        const CLIPPING_INVOCATIONS = 0x00000020;
        const CLIPPING_PRIMITIVES = 0x00000040;
        const FRAGMENT_SHADER_INVOCATIONS = 0x00000080;
        const TESSELLATION_CONTROL_SHADER_PATCHES = 0x00000100;
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS = 0x00000200;
        const COMPUTE_SHADER_INVOCATIONS = 0x00000400;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct QueryResultFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const RESULT_64 = 0x00000001;
        const WAIT = 0x00000002;
        const WITH_AVAILABILITY = 0x00000004;
        const PARTIAL = 0x00000008;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct BufferCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const SPARSE_BINDING = 0x00000001;
        const SPARSE_RESIDENCY = 0x00000002;
        const SPARSE_ALIASED = 0x00000004;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct BufferUsageFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const TRANSFER_SRC = 0x00000001;
        const TRANSFER_DST = 0x00000002;
        const UNIFORM_TEXEL_BUFFER = 0x00000004;
        const STORAGE_TEXEL_BUFFER = 0x00000008;
        const UNIFORM_BUFFER = 0x00000010;
        const STORAGE_BUFFER = 0x00000020;
        const INDEX_BUFFER = 0x00000040;
        const VERTEX_BUFFER = 0x00000080;
        const INDIRECT_BUFFER = 0x00000100;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct BufferViewCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ImageViewCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ShaderModuleCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineCacheCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const DISABLE_OPTIMIZATION = 0x00000001;
        const ALLOW_DERIVATIVES = 0x00000002;
        const DERIVATIVE = 0x00000004;

        #[cfg(feature = "experimental")]
        const VIEW_INDEX_FROM_DEVICE_INDEX = 0x00000008;
        #[cfg(feature = "experimental")]
        const DISPATCH_BASE = 0x00000010;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineShaderStageCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ShaderStageFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const VERTEX = 0x00000001;
        const TESSELLATION_CONTROL = 0x00000002;
        const TESSELLATION_EVALUATION = 0x00000004;
        const GEOMETRY = 0x00000008;
        const FRAGMENT = 0x00000010;
        const COMPUTE = 0x00000020;
        const ALL_GRAPHICS = 0x0000001f;
        const ALL = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineVertexInputStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineInputAssemblyStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineTessellationStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineViewportStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineRasterizationStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct CullModeFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const NONE = 0;
        const FRONT = 0x00000001;
        const BACK = 0x00000002;
        const FRONT_AND_BACK = 0x00000003;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineMultisampleStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineDepthStencilStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineColorBlendStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ColorComponentFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const R = 0x00000001;
        const G = 0x00000002;
        const B = 0x00000004;
        const A = 0x00000008;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineDynamicStateCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineLayoutCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SamplerCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const PUSH_DESCRIPTOR_KHR = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DescriptorPoolCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const FREE_DESCRIPTOR_SET = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DescriptorPoolResetFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct FramebufferCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct RenderPassCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct AttachmentDescriptionFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const MAY_ALIAS = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SubpassDescriptionFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;

        #[cfg(feature = "experimental")]
        const PER_VIEW_ATTRIBUTES = 0x00000001;
        #[cfg(feature = "experimental")]
        const PER_VIEW_POSITION_X_ONLY = 0x00000002;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct AccessFlags: u32 {
        const NONE = 0x00000001;
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const INDIRECT_COMMAND_READ = 0x00000001;
        const INDEX_READ = 0x00000002;
        const VERTEX_ATTRIBUTE_READ = 0x00000004;
        const UNIFORM_READ = 0x00000008;
        const INPUT_ATTACHMENT_READ = 0x00000010;
        const SHADER_READ = 0x00000020;
        const SHADER_WRITE = 0x00000040;
        const COLOR_ATTACHMENT_READ = 0x00000080;
        const COLOR_ATTACHMENT_WRITE = 0x00000100;
        const DEPTH_STENCIL_ATTACHMENT_READ = 0x00000200;
        const DEPTH_STENCIL_ATTACHMENT_WRITE = 0x00000400;
        const TRANSFER_READ = 0x00000800;
        const TRANSFER_WRITE = 0x00001000;
        const HOST_READ = 0x00002000;
        const HOST_WRITE = 0x00004000;
        const MEMORY_READ = 0x00008000;
        const MEMORY_WRITE = 0x00010000;

        #[cfg(feature = "experimental")]
        const COMMAND_PROCESS_READ = 0x00020000;
        #[cfg(feature = "experimental")]
        const COMMAND_PROCESS_WRITE = 0x00040000;
        const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = 0x00080000;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DependencyFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const BY_REGION = 0x00000001;

        #[cfg(feature = "experimental")]
        const VIEW_LOCAL = 0x00000002;
        #[cfg(feature = "experimental")]
        const DEVICE_GROUP = 0x00000004;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct CommandPoolCreateFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const TRANSIENT = 0x00000001;
        const RESET_COMMAND_BUFFER = 0x00000002;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct CommandPoolResetFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const RELEASE_RESOURCES = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct CommandBufferUsageFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const ONE_TIME_SUBMIT = 0x00000001;
        const RENDER_PASS_CONTINUE = 0x00000002;
        const SIMULTANEOUS_USE = 0x00000004;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct QueryControlFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const PRECISE = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct CommandBufferResetFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const RELEASE_RESOURCES = 0x00000001;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct StencilFaceFlags: u32 {
        const FLAG_BITS_MAX_ENUM = 0x7fffffff;
        const FRONT = 0x00000001;
        const BACK = 0x00000002;
        const STENCIL_FRONT_AND_BACK = 0x00000003;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SurfaceTransformFlagsKhr: u32 {
        const IDENTITY = 1;
        const ROTATE_90 = 2;
        const ROTATE_180 = 4;
        const ROTATE_270 = 8;
        const HORIZONTAL_MIRROR = 16;
        const HORIZONTAL_MIRROR_ROTATE_90 = 32;
        const HORIZONTAL_MIRROR_ROTATE_180 = 64;
        const HORIZONTAL_MIRROR_ROTATE_270 = 128;
        const INHERIT = 256;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}



bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct CompositeAlphaFlagsKhr: u32 {
        const OPAQUE = 1;
        const PRE_MULTIPLIED = 2;
        const POST_MULTIPLIED = 4;
        const INHERIT = 8;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}



bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SwapchainCreateFlagsKhr: u32 {
        const BIND_SFR = 1;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DisplayPlaneAlphaFlagsKhr: u32 {
        const OPAQUE = 1;
        const GLOBAL = 2;
        const PER_PIXEL = 4;
        const PER_PIXEL_PREMULTIPLIED = 8;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalMemoryHandleTypeFlagsKhr: u32 {
        const OPAQUE_FD = 1;
        const OPAQUE_WIN32 = 2;
        const OPAQUE_WIN32_KMT = 4;
        const D3D11_TEXTURE = 8;
        const D3D11_TEXTURE_KMT = 16;
        const D3D12_HEAP = 32;
        const D3D12_RESOURCE = 64;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalMemoryFeatureFlagsKhr: u32 {
        const DEDICATED_ONLY = 1;
        const EXPORTABLE = 2;
        const IMPORTABLE = 4;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalSemaphoreHandleTypeFlagsKhr: u32 {
        const OPAQUE_FD = 1;
        const OPAQUE_WIN32 = 2;
        const OPAQUE_WIN32_KMT = 4;
        const D3D12_FENCE = 8;
        const SYNC_FD = 16;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalSemaphoreFeatureFlagsKhr: u32 {
        const EXPORTABLE = 1;
        const IMPORTABLE = 2;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SemaphoreImportFlagsKhr: u32 {
        const TEMPORARY = 1;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalFenceHandleTypeFlagsKhr: u32 {
        const OPAQUE_FD = 1;
        const OPAQUE_WIN32 = 2;
        const OPAQUE_WIN32_KMT = 4;
        const SYNC_FD = 8;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalFenceFeatureFlagsKhr: u32 {
        const EXPORTABLE = 1;
        const IMPORTABLE = 2;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct FenceImportFlagsKhr: u32 {
        const TEMPORARY = 1;
        const FLAG_BITS_MAX_ENUM_KHR = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DebugReportFlagsExt:  u32 {
        const INFORMATION = 1;
        const WARNING = 2;
        const PERFORMANCE_WARNING = 4;
        const ERROR = 8;
        const DEBUG = 16;
        const FLAG_BITS_MAX_ENUM_EXT = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalMemoryHandleTypeFlagsNv:  u32 {
        const OPAQUE_WIN32 = 1;
        const OPAQUE_WIN32_KMT = 2;
        const D3D11_IMAGE = 4;
        const D3D11_IMAGE_KMT = 8;
        const FLAG_BITS_MAX_ENUM_NV = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExternalMemoryFeatureFlagsNv:  u32 {
        const DEDICATED_ONLY = 1;
        const EXPORTABLE = 2;
        const IMPORTABLE = 4;
        const FLAG_BITS_MAX_ENUM_NV = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PeerMemoryFeatureFlagsKhx:  u32 {
        const COPY_SRC = 1;
        const COPY_DST = 2;
        const GENERIC_SRC = 4;
        const GENERIC_DST = 8;
        const FLAG_BITS_MAX_ENUM_KHX = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MemoryAllocateFlagsKhx:  u32 {
        const DEVICE_MASK = 1;
        const FLAG_BITS_MAX_ENUM_KHX = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DeviceGroupPresentModeFlagsKhx:  u32 {
        const LOCAL = 1;
        const REMOTE = 2;
        const SUM = 4;
        const LOCAL_MULTI_DEVICE = 8;
        const FLAG_BITS_MAX_ENUM_KHX = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct IndirectCommandsLayoutUsageFlagsNvx:  u32 {
        const UNORDERED_SEQUENCES = 1;
        const SPARSE_SEQUENCES = 2;
        const EMPTY_EXECUTIONS = 4;
        const INDEXED_SEQUENCES = 8;
        const FLAG_BITS_MAX_ENUM_NVX = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ObjectEntryUsageFlagsNvx:  u32 {
        const GRAPHICS = 1;
        const COMPUTE = 2;
        const FLAG_BITS_MAX_ENUM_NVX = 2147483647;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct SurfaceCounterFlagsExt:  u32 {
        const VBLANK_EXT = 1;
        const FLAG_BITS_MAX_ENUM_EXT = 2147483647;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DisplaySurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DisplayModeCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct AndroidSurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MirSurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ViSurfaceCreateFlagsNN: u32 {
        const FLAG_BITS_MAX_ENUM_NN = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct WaylandSurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct Win32SurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct XlibSurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct XcbSurfaceCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ViSurfaceCreateFlagsNn: u32 {
        const FLAG_BITS_MAX_ENUM_NN = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct DescriptorUpdateTemplateCreateFlagsKhr: u32 {
        const FLAG_BITS_MAX_ENUM_KHR = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct IosSurfaceCreateFlagsMvk: u32 {
        const FLAG_BITS_MAX_ENUM_MVK = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct MacOsSurfaceCreateFlagsMvk: u32 {
        const FLAG_BITS_MAX_ENUM_MVK = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineViewportSwizzleStateCreateFlagsNv: u32 {
        const FLAG_BITS_MAX_ENUM_NV = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineDiscardRectangleStateCreateFlagsExt: u32 {
        const FLAG_BITS_MAX_ENUM_EXT = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineCoverageToColorStateCreateFlagsNv: u32 {
        const FLAG_BITS_MAX_ENUM_NV = 0x7fffffff;
    }
}


bitflags! {
    #[repr(C)]
    #[derive(Default)]
    pub struct PipelineCoverageModulationStateCreateFlagsNv: u32 {
        const FLAG_BITS_MAX_ENUM_NV = 0x7fffffff;
    }
}
