use std::ptr;
use std::ffi::{CString, CStr};
use std::ops::Deref;
use std::marker::PhantomData;
use ::{Version, CharStr};
use vks;


// typedef struct VkApplicationInfo
/// Application information.
///
/// # Example
/// ```text
/// let app_name = CString::new("Hello Triangle").unwrap();
///    let engine_name = CString::new("No Engine").unwrap();
///
///  let app_info = voo::ApplicationInfo::new()
///        .application_name_c_str(app_name)
///        .application_version((1, 0, 0))
///        .engine_name_c_str(engine_name)
///        .engine_version((1, 0, 0))
///        .api_version((1, 0, 51));
/// ```
//
#[derive(Debug, Clone, Default)]
pub struct ApplicationInfo<'a> {
    pub raw: vks::VkApplicationInfo,
    // application_name: Option<CString>,
    application_name: Option<CharStr<'a>>,
    // engine_name: Option<CString>,
    engine_name: Option<CharStr<'a>>,
    _names: PhantomData<&'a CStr>
}

impl<'a> ApplicationInfo<'a> {
    /// Returns a new `ApplicationInfo` with default values.
    pub fn new() -> ApplicationInfo<'a> {
        ApplicationInfo {
            raw: vks::VkApplicationInfo::default(),
            application_name: None,
            engine_name: None,
            _names: PhantomData,
        }
    }

    /// Specifies the application name.
    ///
    /// Use `CStr::from_bytes_with_nul` to avoid any extra allocation. (e.g.:
    /// `.application_name(CStr::from_bytes_with_nul(b"Application
    /// Name\0").unwrap())`).
    pub fn application_name<'c, S>(mut self, application_name: S)
            -> ApplicationInfo<'a>
            where 'c: 'a, S: Into<CharStr<'c>> {
        self.application_name = Some(application_name.into());
        self.raw.pApplicationName = self.application_name.as_ref().unwrap().as_ptr();
        self
    }

    /// Specifies the application version.
    pub fn application_version<V: Into<Version>>(mut self, application_version: V)
            -> ApplicationInfo<'a> {
        self.raw.applicationVersion = application_version.into().into();
        self
    }

    /// Specifies the engine name.
    ///
    /// Use `CStr::from_bytes_with_nul` to avoid any extra allocation. (e.g.:
    /// `.engine_name(CStr::from_bytes_with_nul(b"Engine Name\0").unwrap())`).
    pub fn engine_name<'c, S>(mut self, engine_name: S)
            -> ApplicationInfo<'a>
            where 'c: 'a, S: Into<CharStr<'c>> {
        self.engine_name = Some(engine_name.into());
        self.raw.pApplicationName = self.engine_name.as_ref().unwrap().as_ptr();
        self
    }

    /// Specifies the engine version.
    pub fn engine_version<V: Into<Version>>(mut self, engine_version: V)
            -> ApplicationInfo<'a> {
        self.raw.engineVersion = engine_version.into().into();
        self
    }

    /// Specifies the API version.
    pub fn api_version<V: Into<Version>>(mut self, api_version: V)
            -> ApplicationInfo<'a> {
        self.raw.apiVersion = api_version.into().into();
        self
    }

    /// Returns the specified application name.
    pub fn get_application_name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    /// Returns the specified application version.
    pub fn get_application_version(&self) -> Version {
        self.raw.applicationVersion.into()
    }

    /// Returns the specified engine name.
    pub fn get_engine_name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    /// Returns the specified engine version.
    pub fn get_engine_version(&self) -> Version {
        self.raw.engineVersion.into()
    }

    /// Returns the specified API version.
    pub fn get_api_version(&self) -> Version {
        self.raw.apiVersion.into()
    }

    /// Returns a reference to the internal `vks::VkApplicationInfo` struct.
    pub fn raw(&self) -> &vks::VkApplicationInfo {
        &self.raw
    }
}



// typedef struct VkInstanceCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkInstanceCreateFlags       flags;
//     const VkApplicationInfo*    pApplicationInfo;
//     uint32_t                    enabledLayerCount;
//     const char* const*          ppEnabledLayerNames;
//     uint32_t                    enabledExtensionCount;
//     const char* const*          ppEnabledExtensionNames;
// } VkInstanceCreateInfo;
pub struct InstanceCreateInfo/*<'s>*/ {
    pub raw: vks::VkInstanceCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkAllocationCallbacks {
//     void*                                   pUserData;
//     PFN_vkAllocationFunction                pfnAllocation;
//     PFN_vkReallocationFunction              pfnReallocation;
//     PFN_vkFreeFunction                      pfnFree;
//     PFN_vkInternalAllocationNotification    pfnInternalAllocation;
//     PFN_vkInternalFreeNotification          pfnInternalFree;
// } VkAllocationCallbacks;
pub struct AllocationCallbacks/*<'s>*/ {
    pub raw: vks::VkAllocationCallbacks,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceFeatures {
//     VkBool32    robustBufferAccess;
//     VkBool32    fullDrawIndexUint32;
//     VkBool32    imageCubeArray;
//     VkBool32    independentBlend;
//     VkBool32    geometryShader;
//     VkBool32    tessellationShader;
//     VkBool32    sampleRateShading;
//     VkBool32    dualSrcBlend;
//     VkBool32    logicOp;
//     VkBool32    multiDrawIndirect;
//     VkBool32    drawIndirectFirstInstance;
//     VkBool32    depthClamp;
//     VkBool32    depthBiasClamp;
//     VkBool32    fillModeNonSolid;
//     VkBool32    depthBounds;
//     VkBool32    wideLines;
//     VkBool32    largePoints;
//     VkBool32    alphaToOne;
//     VkBool32    multiViewport;
//     VkBool32    samplerAnisotropy;
//     VkBool32    textureCompressionETC2;
//     VkBool32    textureCompressionASTC_LDR;
//     VkBool32    textureCompressionBC;
//     VkBool32    occlusionQueryPrecise;
//     VkBool32    pipelineStatisticsQuery;
//     VkBool32    vertexPipelineStoresAndAtomics;
//     VkBool32    fragmentStoresAndAtomics;
//     VkBool32    shaderTessellationAndGeometryPointSize;
//     VkBool32    shaderImageGatherExtended;
//     VkBool32    shaderStorageImageExtendedFormats;
//     VkBool32    shaderStorageImageMultisample;
//     VkBool32    shaderStorageImageReadWithoutFormat;
//     VkBool32    shaderStorageImageWriteWithoutFormat;
//     VkBool32    shaderUniformBufferArrayDynamicIndexing;
//     VkBool32    shaderSampledImageArrayDynamicIndexing;
//     VkBool32    shaderStorageBufferArrayDynamicIndexing;
//     VkBool32    shaderStorageImageArrayDynamicIndexing;
//     VkBool32    shaderClipDistance;
//     VkBool32    shaderCullDistance;
//     VkBool32    shaderFloat64;
//     VkBool32    shaderInt64;
//     VkBool32    shaderInt16;
//     VkBool32    shaderResourceResidency;
//     VkBool32    shaderResourceMinLod;
//     VkBool32    sparseBinding;
//     VkBool32    sparseResidencyBuffer;
//     VkBool32    sparseResidencyImage2D;
//     VkBool32    sparseResidencyImage3D;
//     VkBool32    sparseResidency2Samples;
//     VkBool32    sparseResidency4Samples;
//     VkBool32    sparseResidency8Samples;
//     VkBool32    sparseResidency16Samples;
//     VkBool32    sparseResidencyAliased;
//     VkBool32    variableMultisampleRate;
//     VkBool32    inheritedQueries;
// } VkPhysicalDeviceFeatures;
pub struct PhysicalDeviceFeatures/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceFeatures,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkFormatProperties {
//     VkFormatFeatureFlags    linearTilingFeatures;
//     VkFormatFeatureFlags    optimalTilingFeatures;
//     VkFormatFeatureFlags    bufferFeatures;
// } VkFormatProperties;
pub struct FormatProperties/*<'s>*/ {
    pub raw: vks::VkFormatProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExtent3D {
//     uint32_t    width;
//     uint32_t    height;
//     uint32_t    depth;
// } VkExtent3D;
pub struct Extent3D/*<'s>*/ {
    pub raw: vks::VkExtent3D,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageFormatProperties {
//     VkExtent3D            maxExtent;
//     uint32_t              maxMipLevels;
//     uint32_t              maxArrayLayers;
//     VkSampleCountFlags    sampleCounts;
//     VkDeviceSize          maxResourceSize;
// } VkImageFormatProperties;
pub struct ImageFormatProperties/*<'s>*/ {
    pub raw: vks::VkImageFormatProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceLimits {
//     uint32_t              maxImageDimension1D;
//     uint32_t              maxImageDimension2D;
//     uint32_t              maxImageDimension3D;
//     uint32_t              maxImageDimensionCube;
//     uint32_t              maxImageArrayLayers;
//     uint32_t              maxTexelBufferElements;
//     uint32_t              maxUniformBufferRange;
//     uint32_t              maxStorageBufferRange;
//     uint32_t              maxPushConstantsSize;
//     uint32_t              maxMemoryAllocationCount;
//     uint32_t              maxSamplerAllocationCount;
//     VkDeviceSize          bufferImageGranularity;
//     VkDeviceSize          sparseAddressSpaceSize;
//     uint32_t              maxBoundDescriptorSets;
//     uint32_t              maxPerStageDescriptorSamplers;
//     uint32_t              maxPerStageDescriptorUniformBuffers;
//     uint32_t              maxPerStageDescriptorStorageBuffers;
//     uint32_t              maxPerStageDescriptorSampledImages;
//     uint32_t              maxPerStageDescriptorStorageImages;
//     uint32_t              maxPerStageDescriptorInputAttachments;
//     uint32_t              maxPerStageResources;
//     uint32_t              maxDescriptorSetSamplers;
//     uint32_t              maxDescriptorSetUniformBuffers;
//     uint32_t              maxDescriptorSetUniformBuffersDynamic;
//     uint32_t              maxDescriptorSetStorageBuffers;
//     uint32_t              maxDescriptorSetStorageBuffersDynamic;
//     uint32_t              maxDescriptorSetSampledImages;
//     uint32_t              maxDescriptorSetStorageImages;
//     uint32_t              maxDescriptorSetInputAttachments;
//     uint32_t              maxVertexInputAttributes;
//     uint32_t              maxVertexInputBindings;
//     uint32_t              maxVertexInputAttributeOffset;
//     uint32_t              maxVertexInputBindingStride;
//     uint32_t              maxVertexOutputComponents;
//     uint32_t              maxTessellationGenerationLevel;
//     uint32_t              maxTessellationPatchSize;
//     uint32_t              maxTessellationControlPerVertexInputComponents;
//     uint32_t              maxTessellationControlPerVertexOutputComponents;
//     uint32_t              maxTessellationControlPerPatchOutputComponents;
//     uint32_t              maxTessellationControlTotalOutputComponents;
//     uint32_t              maxTessellationEvaluationInputComponents;
//     uint32_t              maxTessellationEvaluationOutputComponents;
//     uint32_t              maxGeometryShaderInvocations;
//     uint32_t              maxGeometryInputComponents;
//     uint32_t              maxGeometryOutputComponents;
//     uint32_t              maxGeometryOutputVertices;
//     uint32_t              maxGeometryTotalOutputComponents;
//     uint32_t              maxFragmentInputComponents;
//     uint32_t              maxFragmentOutputAttachments;
//     uint32_t              maxFragmentDualSrcAttachments;
//     uint32_t              maxFragmentCombinedOutputResources;
//     uint32_t              maxComputeSharedMemorySize;
//     uint32_t              maxComputeWorkGroupCount[3];
//     uint32_t              maxComputeWorkGroupInvocations;
//     uint32_t              maxComputeWorkGroupSize[3];
//     uint32_t              subPixelPrecisionBits;
//     uint32_t              subTexelPrecisionBits;
//     uint32_t              mipmapPrecisionBits;
//     uint32_t              maxDrawIndexedIndexValue;
//     uint32_t              maxDrawIndirectCount;
//     float                 maxSamplerLodBias;
//     float                 maxSamplerAnisotropy;
//     uint32_t              maxViewports;
//     uint32_t              maxViewportDimensions[2];
//     float                 viewportBoundsRange[2];
//     uint32_t              viewportSubPixelBits;
//     size_t                minMemoryMapAlignment;
//     VkDeviceSize          minTexelBufferOffsetAlignment;
//     VkDeviceSize          minUniformBufferOffsetAlignment;
//     VkDeviceSize          minStorageBufferOffsetAlignment;
//     int32_t               minTexelOffset;
//     uint32_t              maxTexelOffset;
//     int32_t               minTexelGatherOffset;
//     uint32_t              maxTexelGatherOffset;
//     float                 minInterpolationOffset;
//     float                 maxInterpolationOffset;
//     uint32_t              subPixelInterpolationOffsetBits;
//     uint32_t              maxFramebufferWidth;
//     uint32_t              maxFramebufferHeight;
//     uint32_t              maxFramebufferLayers;
//     VkSampleCountFlags    framebufferColorSampleCounts;
//     VkSampleCountFlags    framebufferDepthSampleCounts;
//     VkSampleCountFlags    framebufferStencilSampleCounts;
//     VkSampleCountFlags    framebufferNoAttachmentsSampleCounts;
//     uint32_t              maxColorAttachments;
//     VkSampleCountFlags    sampledImageColorSampleCounts;
//     VkSampleCountFlags    sampledImageIntegerSampleCounts;
//     VkSampleCountFlags    sampledImageDepthSampleCounts;
//     VkSampleCountFlags    sampledImageStencilSampleCounts;
//     VkSampleCountFlags    storageImageSampleCounts;
//     uint32_t              maxSampleMaskWords;
//     VkBool32              timestampComputeAndGraphics;
//     float                 timestampPeriod;
//     uint32_t              maxClipDistances;
//     uint32_t              maxCullDistances;
//     uint32_t              maxCombinedClipAndCullDistances;
//     uint32_t              discreteQueuePriorities;
//     float                 pointSizeRange[2];
//     float                 lineWidthRange[2];
//     float                 pointSizeGranularity;
//     float                 lineWidthGranularity;
//     VkBool32              strictLines;
//     VkBool32              standardSampleLocations;
//     VkDeviceSize          optimalBufferCopyOffsetAlignment;
//     VkDeviceSize          optimalBufferCopyRowPitchAlignment;
//     VkDeviceSize          nonCoherentAtomSize;
// } VkPhysicalDeviceLimits;
pub struct PhysicalDeviceLimits/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceLimits,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceSparseProperties {
//     VkBool32    residencyStandard2DBlockShape;
//     VkBool32    residencyStandard2DMultisampleBlockShape;
//     VkBool32    residencyStandard3DBlockShape;
//     VkBool32    residencyAlignedMipSize;
//     VkBool32    residencyNonResidentStrict;
// } VkPhysicalDeviceSparseProperties;
pub struct PhysicalDeviceSparseProperties/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceSparseProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceProperties {
//     uint32_t                            apiVersion;
//     uint32_t                            driverVersion;
//     uint32_t                            vendorID;
//     uint32_t                            deviceID;
//     VkPhysicalDeviceType                deviceType;
//     char                                deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE];
//     uint8_t                             pipelineCacheUUID[VK_UUID_SIZE];
//     VkPhysicalDeviceLimits              limits;
//     VkPhysicalDeviceSparseProperties    sparseProperties;
// } VkPhysicalDeviceProperties;
pub struct PhysicalDeviceProperties/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkQueueFamilyProperties {
//     VkQueueFlags    queueFlags;
//     uint32_t        queueCount;
//     uint32_t        timestampValidBits;
//     VkExtent3D      minImageTransferGranularity;
// } VkQueueFamilyProperties;
pub struct QueueFamilyProperties/*<'s>*/ {
    pub raw: vks::VkQueueFamilyProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryType {
//     VkMemoryPropertyFlags    propertyFlags;
//     uint32_t                 heapIndex;
// } VkMemoryType;
pub struct MemoryType/*<'s>*/ {
    pub raw: vks::VkMemoryType,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryHeap {
//     VkDeviceSize         size;
//     VkMemoryHeapFlags    flags;
// } VkMemoryHeap;
pub struct MemoryHeap/*<'s>*/ {
    pub raw: vks::VkMemoryHeap,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceMemoryProperties {
//     uint32_t        memoryTypeCount;
//     VkMemoryType    memoryTypes[VK_MAX_MEMORY_TYPES];
//     uint32_t        memoryHeapCount;
//     VkMemoryHeap    memoryHeaps[VK_MAX_MEMORY_HEAPS];
// } VkPhysicalDeviceMemoryProperties;
pub struct PhysicalDeviceMemoryProperties/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceMemoryProperties,
    // _p: PhantomData<&'s ()>,
}

// typedef struct VkDeviceQueueCreateInfo
/// Device queue creation information.
///
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VkDeviceQueueCreateInfo {
//     pub sType: VkStructureType,
//     pub pNext: *const c_void,
//     pub flags: VkDeviceQueueCreateFlags,
//     pub queueFamilyIndex: u32,
//     pub queueCount: u32,
//     pub pQueuePriorities: *const f32,
// }
#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct DeviceQueueCreateInfo<'ci> {
    raw: vks::VkDeviceQueueCreateInfo,
    _qp: PhantomData<&'ci [f32]>,
}

impl<'ci> DeviceQueueCreateInfo<'ci> {
    /// Returns a new `DeviceQueueCreateInfo`.
    pub fn new() -> DeviceQueueCreateInfo<'ci> {
        DeviceQueueCreateInfo {
            raw: vks::VkDeviceQueueCreateInfo::default(),
            _qp: PhantomData,
        }
    }

    /// Specifies the queue family index to use.
    pub fn queue_family_index(mut self, queue_family_index: u32) -> DeviceQueueCreateInfo<'ci> {
        self.raw.queueFamilyIndex = queue_family_index;
        self
    }

    /// Specifies a list of priority ranking from `0.0` to `1.0` for each
    /// queue.
    ///
    /// Passing `&[1.0]` will create a single queue.
    pub fn queue_priorities<'qp>(mut self, queue_priorities: &'qp [f32])
            -> DeviceQueueCreateInfo<'ci>
            where 'qp: 'ci {
        self.raw.queueCount = queue_priorities.len() as u32;
        self.raw.pQueuePriorities = queue_priorities.as_ptr();
        self
    }

    pub fn get_queue_family_index(&self) -> u32 {
        self.raw.queueFamilyIndex
    }

    /// Returns a reference to the internal `vks::VkDeviceQueueCreateInfo`
    /// struct.
    pub fn raw(&self) -> &vks::VkDeviceQueueCreateInfo {
        &self.raw
    }
}




// typedef struct VkDeviceCreateInfo
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
pub struct DeviceCreateInfo/*<'s>*/ {
    pub raw: vks::VkDeviceCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExtensionProperties {
//     char        extensionName[VK_MAX_EXTENSION_NAME_SIZE];
//     uint32_t    specVersion;
// } VkExtensionProperties;
pub struct ExtensionProperties/*<'s>*/ {
    pub raw: vks::VkExtensionProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkLayerProperties {
//     char        layerName[VK_MAX_EXTENSION_NAME_SIZE];
//     uint32_t    specVersion;
//     uint32_t    implementationVersion;
//     char        description[VK_MAX_DESCRIPTION_SIZE];
// } VkLayerProperties;
pub struct LayerProperties/*<'s>*/ {
    pub raw: vks::VkLayerProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSubmitInfo {
//     VkStructureType                sType;
//     const void*                    pNext;
//     uint32_t                       waitSemaphoreCount;
//     const VkSemaphore*             pWaitSemaphores;
//     const VkPipelineStageFlags*    pWaitDstStageMask;
//     uint32_t                       commandBufferCount;
//     const VkCommandBuffer*         pCommandBuffers;
//     uint32_t                       signalSemaphoreCount;
//     const VkSemaphore*             pSignalSemaphores;
// } VkSubmitInfo;
pub struct SubmitInfo/*<'s>*/ {
    pub raw: vks::VkSubmitInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryAllocateInfo {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkDeviceSize       allocationSize;
//     uint32_t           memoryTypeIndex;
// } VkMemoryAllocateInfo;
pub struct MemoryAllocateInfo/*<'s>*/ {
    pub raw: vks::VkMemoryAllocateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMappedMemoryRange {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkDeviceMemory     memory;
//     VkDeviceSize       offset;
//     VkDeviceSize       size;
// } VkMappedMemoryRange;
pub struct MappedMemoryRange/*<'s>*/ {
    pub raw: vks::VkMappedMemoryRange,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryRequirements {
//     VkDeviceSize    size;
//     VkDeviceSize    alignment;
//     uint32_t        memoryTypeBits;
// } VkMemoryRequirements;
pub struct MemoryRequirements/*<'s>*/ {
    pub raw: vks::VkMemoryRequirements,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageFormatProperties {
//     VkImageAspectFlags          aspectMask;
//     VkExtent3D                  imageGranularity;
//     VkSparseImageFormatFlags    flags;
// } VkSparseImageFormatProperties;
pub struct SparseImageFormatProperties/*<'s>*/ {
    pub raw: vks::VkSparseImageFormatProperties,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageMemoryRequirements {
//     VkSparseImageFormatProperties    formatProperties;
//     uint32_t                         imageMipTailFirstLod;
//     VkDeviceSize                     imageMipTailSize;
//     VkDeviceSize                     imageMipTailOffset;
//     VkDeviceSize                     imageMipTailStride;
// } VkSparseImageMemoryRequirements;
pub struct SparseImageMemoryRequirements/*<'s>*/ {
    pub raw: vks::VkSparseImageMemoryRequirements,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseMemoryBind {
//     VkDeviceSize               resourceOffset;
//     VkDeviceSize               size;
//     VkDeviceMemory             memory;
//     VkDeviceSize               memoryOffset;
//     VkSparseMemoryBindFlags    flags;
// } VkSparseMemoryBind;
pub struct SparseMemoryBind/*<'s>*/ {
    pub raw: vks::VkSparseMemoryBind,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseBufferMemoryBindInfo {
//     VkBuffer                     buffer;
//     uint32_t                     bindCount;
//     const VkSparseMemoryBind*    pBinds;
// } VkSparseBufferMemoryBindInfo;
pub struct SparseBufferMemoryBindInfo/*<'s>*/ {
    pub raw: vks::VkSparseBufferMemoryBindInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageOpaqueMemoryBindInfo {
//     VkImage                      image;
//     uint32_t                     bindCount;
//     const VkSparseMemoryBind*    pBinds;
// } VkSparseImageOpaqueMemoryBindInfo;
pub struct SparseImageOpaqueMemoryBindInfo/*<'s>*/ {
    pub raw: vks::VkSparseImageOpaqueMemoryBindInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageSubresource {
//     VkImageAspectFlags    aspectMask;
//     uint32_t              mipLevel;
//     uint32_t              arrayLayer;
// } VkImageSubresource;
pub struct ImageSubresource/*<'s>*/ {
    pub raw: vks::VkImageSubresource,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkOffset3D {
//     int32_t    x;
//     int32_t    y;
//     int32_t    z;
// } VkOffset3D;
pub struct Offset3D/*<'s>*/ {
    pub raw: vks::VkOffset3D,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageMemoryBind {
//     VkImageSubresource         subresource;
//     VkOffset3D                 offset;
//     VkExtent3D                 extent;
//     VkDeviceMemory             memory;
//     VkDeviceSize               memoryOffset;
//     VkSparseMemoryBindFlags    flags;
// } VkSparseImageMemoryBind;
pub struct SparseImageMemoryBind/*<'s>*/ {
    pub raw: vks::VkSparseImageMemoryBind,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageMemoryBindInfo {
//     VkImage                           image;
//     uint32_t                          bindCount;
//     const VkSparseImageMemoryBind*    pBinds;
// } VkSparseImageMemoryBindInfo;
pub struct SparseImageMemoryBindInfo/*<'s>*/ {
    pub raw: vks::VkSparseImageMemoryBindInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindSparseInfo {
//     VkStructureType                             sType;
//     const void*                                 pNext;
//     uint32_t                                    waitSemaphoreCount;
//     const VkSemaphore*                          pWaitSemaphores;
//     uint32_t                                    bufferBindCount;
//     const VkSparseBufferMemoryBindInfo*         pBufferBinds;
//     uint32_t                                    imageOpaqueBindCount;
//     const VkSparseImageOpaqueMemoryBindInfo*    pImageOpaqueBinds;
//     uint32_t                                    imageBindCount;
//     const VkSparseImageMemoryBindInfo*          pImageBinds;
//     uint32_t                                    signalSemaphoreCount;
//     const VkSemaphore*                          pSignalSemaphores;
// } VkBindSparseInfo;
pub struct BindSparseInfo/*<'s>*/ {
    pub raw: vks::VkBindSparseInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkFenceCreateInfo {
//     VkStructureType       sType;
//     const void*           pNext;
//     VkFenceCreateFlags    flags;
// } VkFenceCreateInfo;
pub struct FenceCreateInfo/*<'s>*/ {
    pub raw: vks::VkFenceCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSemaphoreCreateInfo {
//     VkStructureType           sType;
//     const void*               pNext;
//     VkSemaphoreCreateFlags    flags;
// } VkSemaphoreCreateInfo;
pub struct SemaphoreCreateInfo/*<'s>*/ {
    pub raw: vks::VkSemaphoreCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkEventCreateInfo {
//     VkStructureType       sType;
//     const void*           pNext;
//     VkEventCreateFlags    flags;
// } VkEventCreateInfo;
pub struct EventCreateInfo/*<'s>*/ {
    pub raw: vks::VkEventCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkQueryPoolCreateInfo {
//     VkStructureType                  sType;
//     const void*                      pNext;
//     VkQueryPoolCreateFlags           flags;
//     VkQueryType                      queryType;
//     uint32_t                         queryCount;
//     VkQueryPipelineStatisticFlags    pipelineStatistics;
// } VkQueryPoolCreateInfo;
pub struct QueryPoolCreateInfo/*<'s>*/ {
    pub raw: vks::VkQueryPoolCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBufferCreateInfo {
//     VkStructureType        sType;
//     const void*            pNext;
//     VkBufferCreateFlags    flags;
//     VkDeviceSize           size;
//     VkBufferUsageFlags     usage;
//     VkSharingMode          sharingMode;
//     uint32_t               queueFamilyIndexCount;
//     const uint32_t*        pQueueFamilyIndices;
// } VkBufferCreateInfo;
pub struct BufferCreateInfo/*<'s>*/ {
    pub raw: vks::VkBufferCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBufferViewCreateInfo {
//     VkStructureType            sType;
//     const void*                pNext;
//     VkBufferViewCreateFlags    flags;
//     VkBuffer                   buffer;
//     VkFormat                   format;
//     VkDeviceSize               offset;
//     VkDeviceSize               range;
// } VkBufferViewCreateInfo;
pub struct BufferViewCreateInfo/*<'s>*/ {
    pub raw: vks::VkBufferViewCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageCreateInfo {
//     VkStructureType          sType;
//     const void*              pNext;
//     VkImageCreateFlags       flags;
//     VkImageType              imageType;
//     VkFormat                 format;
//     VkExtent3D               extent;
//     uint32_t                 mipLevels;
//     uint32_t                 arrayLayers;
//     VkSampleCountFlagBits    samples;
//     VkImageTiling            tiling;
//     VkImageUsageFlags        usage;
//     VkSharingMode            sharingMode;
//     uint32_t                 queueFamilyIndexCount;
//     const uint32_t*          pQueueFamilyIndices;
//     VkImageLayout            initialLayout;
// } VkImageCreateInfo;
pub struct ImageCreateInfo/*<'s>*/ {
    pub raw: vks::VkImageCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSubresourceLayout {
//     VkDeviceSize    offset;
//     VkDeviceSize    size;
//     VkDeviceSize    rowPitch;
//     VkDeviceSize    arrayPitch;
//     VkDeviceSize    depthPitch;
// } VkSubresourceLayout;
pub struct SubresourceLayout/*<'s>*/ {
    pub raw: vks::VkSubresourceLayout,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkComponentMapping {
//     VkComponentSwizzle    r;
//     VkComponentSwizzle    g;
//     VkComponentSwizzle    b;
//     VkComponentSwizzle    a;
// } VkComponentMapping;
pub struct ComponentMapping/*<'s>*/ {
    pub raw: vks::VkComponentMapping,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageSubresourceRange {
//     VkImageAspectFlags    aspectMask;
//     uint32_t              baseMipLevel;
//     uint32_t              levelCount;
//     uint32_t              baseArrayLayer;
//     uint32_t              layerCount;
// } VkImageSubresourceRange;
pub struct ImageSubresourceRange/*<'s>*/ {
    pub raw: vks::VkImageSubresourceRange,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageViewCreateInfo {
//     VkStructureType            sType;
//     const void*                pNext;
//     VkImageViewCreateFlags     flags;
//     VkImage                    image;
//     VkImageViewType            viewType;
//     VkFormat                   format;
//     VkComponentMapping         components;
//     VkImageSubresourceRange    subresourceRange;
// } VkImageViewCreateInfo;
pub struct ImageViewCreateInfo/*<'s>*/ {
    pub raw: vks::VkImageViewCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkShaderModuleCreateInfo {
//     VkStructureType              sType;
//     const void*                  pNext;
//     VkShaderModuleCreateFlags    flags;
//     size_t                       codeSize;
//     const uint32_t*              pCode;
// } VkShaderModuleCreateInfo;
pub struct ShaderModuleCreateInfo/*<'s>*/ {
    pub raw: vks::VkShaderModuleCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineCacheCreateInfo {
//     VkStructureType               sType;
//     const void*                   pNext;
//     VkPipelineCacheCreateFlags    flags;
//     size_t                        initialDataSize;
//     const void*                   pInitialData;
// } VkPipelineCacheCreateInfo;
pub struct PipelineCacheCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineCacheCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSpecializationMapEntry {
//     uint32_t    constantID;
//     uint32_t    offset;
//     size_t      size;
// } VkSpecializationMapEntry;
pub struct SpecializationMapEntry/*<'s>*/ {
    pub raw: vks::VkSpecializationMapEntry,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSpecializationInfo {
//     uint32_t                           mapEntryCount;
//     const VkSpecializationMapEntry*    pMapEntries;
//     size_t                             dataSize;
//     const void*                        pData;
// } VkSpecializationInfo;
pub struct SpecializationInfo/*<'s>*/ {
    pub raw: vks::VkSpecializationInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineShaderStageCreateInfo {
//     VkStructureType                     sType;
//     const void*                         pNext;
//     VkPipelineShaderStageCreateFlags    flags;
//     VkShaderStageFlagBits               stage;
//     VkShaderModule                      module;
//     const char*                         pName;
//     const VkSpecializationInfo*         pSpecializationInfo;
// } VkPipelineShaderStageCreateInfo;
pub struct PipelineShaderStageCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineShaderStageCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkVertexInputBindingDescription {
//     uint32_t             binding;
//     uint32_t             stride;
//     VkVertexInputRate    inputRate;
// } VkVertexInputBindingDescription;
pub struct VertexInputBindingDescription/*<'s>*/ {
    pub raw: vks::VkVertexInputBindingDescription,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkVertexInputAttributeDescription {
//     uint32_t    location;
//     uint32_t    binding;
//     VkFormat    format;
//     uint32_t    offset;
// } VkVertexInputAttributeDescription;
pub struct VertexInputAttributeDescription/*<'s>*/ {
    pub raw: vks::VkVertexInputAttributeDescription,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineVertexInputStateCreateInfo {
//     VkStructureType                             sType;
//     const void*                                 pNext;
//     VkPipelineVertexInputStateCreateFlags       flags;
//     uint32_t                                    vertexBindingDescriptionCount;
//     const VkVertexInputBindingDescription*      pVertexBindingDescriptions;
//     uint32_t                                    vertexAttributeDescriptionCount;
//     const VkVertexInputAttributeDescription*    pVertexAttributeDescriptions;
// } VkPipelineVertexInputStateCreateInfo;
pub struct PipelineVertexInputStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineVertexInputStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineInputAssemblyStateCreateInfo {
//     VkStructureType                            sType;
//     const void*                                pNext;
//     VkPipelineInputAssemblyStateCreateFlags    flags;
//     VkPrimitiveTopology                        topology;
//     VkBool32                                   primitiveRestartEnable;
// } VkPipelineInputAssemblyStateCreateInfo;
pub struct PipelineInputAssemblyStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineInputAssemblyStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineTessellationStateCreateInfo {
//     VkStructureType                           sType;
//     const void*                               pNext;
//     VkPipelineTessellationStateCreateFlags    flags;
//     uint32_t                                  patchControlPoints;
// } VkPipelineTessellationStateCreateInfo;
pub struct PipelineTessellationStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineTessellationStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkViewport {
//     float    x;
//     float    y;
//     float    width;
//     float    height;
//     float    minDepth;
//     float    maxDepth;
// } VkViewport;
pub struct Viewport/*<'s>*/ {
    pub raw: vks::VkViewport,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkOffset2D {
//     int32_t    x;
//     int32_t    y;
// } VkOffset2D;
pub struct Offset2D/*<'s>*/ {
    pub raw: vks::VkOffset2D,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExtent2D {
//     uint32_t    width;
//     uint32_t    height;
// } VkExtent2D;
pub struct Extent2D/*<'s>*/ {
    pub raw: vks::VkExtent2D,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRect2D {
//     VkOffset2D    offset;
//     VkExtent2D    extent;
// } VkRect2D;
pub struct Rect2D/*<'s>*/ {
    pub raw: vks::VkRect2D,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineViewportStateCreateInfo {
//     VkStructureType                       sType;
//     const void*                           pNext;
//     VkPipelineViewportStateCreateFlags    flags;
//     uint32_t                              viewportCount;
//     const VkViewport*                     pViewports;
//     uint32_t                              scissorCount;
//     const VkRect2D*                       pScissors;
// } VkPipelineViewportStateCreateInfo;
pub struct PipelineViewportStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineViewportStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineRasterizationStateCreateInfo {
//     VkStructureType                            sType;
//     const void*                                pNext;
//     VkPipelineRasterizationStateCreateFlags    flags;
//     VkBool32                                   depthClampEnable;
//     VkBool32                                   rasterizerDiscardEnable;
//     VkPolygonMode                              polygonMode;
//     VkCullModeFlags                            cullMode;
//     VkFrontFace                                frontFace;
//     VkBool32                                   depthBiasEnable;
//     float                                      depthBiasConstantFactor;
//     float                                      depthBiasClamp;
//     float                                      depthBiasSlopeFactor;
//     float                                      lineWidth;
// } VkPipelineRasterizationStateCreateInfo;
pub struct PipelineRasterizationStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineRasterizationStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineMultisampleStateCreateInfo {
//     VkStructureType                          sType;
//     const void*                              pNext;
//     VkPipelineMultisampleStateCreateFlags    flags;
//     VkSampleCountFlagBits                    rasterizationSamples;
//     VkBool32                                 sampleShadingEnable;
//     float                                    minSampleShading;
//     const VkSampleMask*                      pSampleMask;
//     VkBool32                                 alphaToCoverageEnable;
//     VkBool32                                 alphaToOneEnable;
// } VkPipelineMultisampleStateCreateInfo;
pub struct PipelineMultisampleStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineMultisampleStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkStencilOpState {
//     VkStencilOp    failOp;
//     VkStencilOp    passOp;
//     VkStencilOp    depthFailOp;
//     VkCompareOp    compareOp;
//     uint32_t       compareMask;
//     uint32_t       writeMask;
//     uint32_t       reference;
// } VkStencilOpState;
pub struct StencilOpState/*<'s>*/ {
    pub raw: vks::VkStencilOpState,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineDepthStencilStateCreateInfo {
//     VkStructureType                           sType;
//     const void*                               pNext;
//     VkPipelineDepthStencilStateCreateFlags    flags;
//     VkBool32                                  depthTestEnable;
//     VkBool32                                  depthWriteEnable;
//     VkCompareOp                               depthCompareOp;
//     VkBool32                                  depthBoundsTestEnable;
//     VkBool32                                  stencilTestEnable;
//     VkStencilOpState                          front;
//     VkStencilOpState                          back;
//     float                                     minDepthBounds;
//     float                                     maxDepthBounds;
// } VkPipelineDepthStencilStateCreateInfo;
pub struct PipelineDepthStencilStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineDepthStencilStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineColorBlendAttachmentState {
//     VkBool32                 blendEnable;
//     VkBlendFactor            srcColorBlendFactor;
//     VkBlendFactor            dstColorBlendFactor;
//     VkBlendOp                colorBlendOp;
//     VkBlendFactor            srcAlphaBlendFactor;
//     VkBlendFactor            dstAlphaBlendFactor;
//     VkBlendOp                alphaBlendOp;
//     VkColorComponentFlags    colorWriteMask;
// } VkPipelineColorBlendAttachmentState;
pub struct PipelineColorBlendAttachmentState/*<'s>*/ {
    pub raw: vks::VkPipelineColorBlendAttachmentState,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineColorBlendStateCreateInfo {
//     VkStructureType                               sType;
//     const void*                                   pNext;
//     VkPipelineColorBlendStateCreateFlags          flags;
//     VkBool32                                      logicOpEnable;
//     VkLogicOp                                     logicOp;
//     uint32_t                                      attachmentCount;
//     const VkPipelineColorBlendAttachmentState*    pAttachments;
//     float                                         blendConstants[4];
// } VkPipelineColorBlendStateCreateInfo;
pub struct PipelineColorBlendStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineColorBlendStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineDynamicStateCreateInfo {
//     VkStructureType                      sType;
//     const void*                          pNext;
//     VkPipelineDynamicStateCreateFlags    flags;
//     uint32_t                             dynamicStateCount;
//     const VkDynamicState*                pDynamicStates;
// } VkPipelineDynamicStateCreateInfo;
pub struct PipelineDynamicStateCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineDynamicStateCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkGraphicsPipelineCreateInfo {
//     VkStructureType                                  sType;
//     const void*                                      pNext;
//     VkPipelineCreateFlags                            flags;
//     uint32_t                                         stageCount;
//     const VkPipelineShaderStageCreateInfo*           pStages;
//     const VkPipelineVertexInputStateCreateInfo*      pVertexInputState;
//     const VkPipelineInputAssemblyStateCreateInfo*    pInputAssemblyState;
//     const VkPipelineTessellationStateCreateInfo*     pTessellationState;
//     const VkPipelineViewportStateCreateInfo*         pViewportState;
//     const VkPipelineRasterizationStateCreateInfo*    pRasterizationState;
//     const VkPipelineMultisampleStateCreateInfo*      pMultisampleState;
//     const VkPipelineDepthStencilStateCreateInfo*     pDepthStencilState;
//     const VkPipelineColorBlendStateCreateInfo*       pColorBlendState;
//     const VkPipelineDynamicStateCreateInfo*          pDynamicState;
//     VkPipelineLayout                                 layout;
//     VkRenderPass                                     renderPass;
//     uint32_t                                         subpass;
//     VkPipeline                                       basePipelineHandle;
//     int32_t                                          basePipelineIndex;
// } VkGraphicsPipelineCreateInfo;
pub struct GraphicsPipelineCreateInfo/*<'s>*/ {
    pub raw: vks::VkGraphicsPipelineCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkComputePipelineCreateInfo {
//     VkStructureType                    sType;
//     const void*                        pNext;
//     VkPipelineCreateFlags              flags;
//     VkPipelineShaderStageCreateInfo    stage;
//     VkPipelineLayout                   layout;
//     VkPipeline                         basePipelineHandle;
//     int32_t                            basePipelineIndex;
// } VkComputePipelineCreateInfo;
pub struct ComputePipelineCreateInfo/*<'s>*/ {
    pub raw: vks::VkComputePipelineCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPushConstantRange {
//     VkShaderStageFlags    stageFlags;
//     uint32_t              offset;
//     uint32_t              size;
// } VkPushConstantRange;
pub struct PushConstantRange/*<'s>*/ {
    pub raw: vks::VkPushConstantRange,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineLayoutCreateInfo {
//     VkStructureType                 sType;
//     const void*                     pNext;
//     VkPipelineLayoutCreateFlags     flags;
//     uint32_t                        setLayoutCount;
//     const VkDescriptorSetLayout*    pSetLayouts;
//     uint32_t                        pushConstantRangeCount;
//     const VkPushConstantRange*      pPushConstantRanges;
// } VkPipelineLayoutCreateInfo;
pub struct PipelineLayoutCreateInfo/*<'s>*/ {
    pub raw: vks::VkPipelineLayoutCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSamplerCreateInfo {
//     VkStructureType         sType;
//     const void*             pNext;
//     VkSamplerCreateFlags    flags;
//     VkFilter                magFilter;
//     VkFilter                minFilter;
//     VkSamplerMipmapMode     mipmapMode;
//     VkSamplerAddressMode    addressModeU;
//     VkSamplerAddressMode    addressModeV;
//     VkSamplerAddressMode    addressModeW;
//     float                   mipLodBias;
//     VkBool32                anisotropyEnable;
//     float                   maxAnisotropy;
//     VkBool32                compareEnable;
//     VkCompareOp             compareOp;
//     float                   minLod;
//     float                   maxLod;
//     VkBorderColor           borderColor;
//     VkBool32                unnormalizedCoordinates;
// } VkSamplerCreateInfo;
pub struct SamplerCreateInfo/*<'s>*/ {
    pub raw: vks::VkSamplerCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorSetLayoutBinding {
//     uint32_t              binding;
//     VkDescriptorType      descriptorType;
//     uint32_t              descriptorCount;
//     VkShaderStageFlags    stageFlags;
//     const VkSampler*      pImmutableSamplers;
// } VkDescriptorSetLayoutBinding;
pub struct DescriptorSetLayoutBinding/*<'s>*/ {
    pub raw: vks::VkDescriptorSetLayoutBinding,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorSetLayoutCreateInfo {
//     VkStructureType                        sType;
//     const void*                            pNext;
//     VkDescriptorSetLayoutCreateFlags       flags;
//     uint32_t                               bindingCount;
//     const VkDescriptorSetLayoutBinding*    pBindings;
// } VkDescriptorSetLayoutCreateInfo;
pub struct DescriptorSetLayoutCreateInfo/*<'s>*/ {
    pub raw: vks::VkDescriptorSetLayoutCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorPoolSize {
//     VkDescriptorType    type;
//     uint32_t            descriptorCount;
// } VkDescriptorPoolSize;
pub struct DescriptorPoolSize/*<'s>*/ {
    pub raw: vks::VkDescriptorPoolSize,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorPoolCreateInfo {
//     VkStructureType                sType;
//     const void*                    pNext;
//     VkDescriptorPoolCreateFlags    flags;
//     uint32_t                       maxSets;
//     uint32_t                       poolSizeCount;
//     const VkDescriptorPoolSize*    pPoolSizes;
// } VkDescriptorPoolCreateInfo;
pub struct DescriptorPoolCreateInfo/*<'s>*/ {
    pub raw: vks::VkDescriptorPoolCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorSetAllocateInfo {
//     VkStructureType                 sType;
//     const void*                     pNext;
//     VkDescriptorPool                descriptorPool;
//     uint32_t                        descriptorSetCount;
//     const VkDescriptorSetLayout*    pSetLayouts;
// } VkDescriptorSetAllocateInfo;
pub struct DescriptorSetAllocateInfo/*<'s>*/ {
    pub raw: vks::VkDescriptorSetAllocateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorImageInfo {
//     VkSampler        sampler;
//     VkImageView      imageView;
//     VkImageLayout    imageLayout;
// } VkDescriptorImageInfo;
pub struct DescriptorImageInfo/*<'s>*/ {
    pub raw: vks::VkDescriptorImageInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorBufferInfo {
//     VkBuffer        buffer;
//     VkDeviceSize    offset;
//     VkDeviceSize    range;
// } VkDescriptorBufferInfo;
pub struct DescriptorBufferInfo/*<'s>*/ {
    pub raw: vks::VkDescriptorBufferInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkWriteDescriptorSet {
//     VkStructureType                  sType;
//     const void*                      pNext;
//     VkDescriptorSet                  dstSet;
//     uint32_t                         dstBinding;
//     uint32_t                         dstArrayElement;
//     uint32_t                         descriptorCount;
//     VkDescriptorType                 descriptorType;
//     const VkDescriptorImageInfo*     pImageInfo;
//     const VkDescriptorBufferInfo*    pBufferInfo;
//     const VkBufferView*              pTexelBufferView;
// } VkWriteDescriptorSet;
pub struct WriteDescriptorSet/*<'s>*/ {
    pub raw: vks::VkWriteDescriptorSet,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCopyDescriptorSet {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkDescriptorSet    srcSet;
//     uint32_t           srcBinding;
//     uint32_t           srcArrayElement;
//     VkDescriptorSet    dstSet;
//     uint32_t           dstBinding;
//     uint32_t           dstArrayElement;
//     uint32_t           descriptorCount;
// } VkCopyDescriptorSet;
pub struct CopyDescriptorSet/*<'s>*/ {
    pub raw: vks::VkCopyDescriptorSet,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkFramebufferCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkFramebufferCreateFlags    flags;
//     VkRenderPass                renderPass;
//     uint32_t                    attachmentCount;
//     const VkImageView*          pAttachments;
//     uint32_t                    width;
//     uint32_t                    height;
//     uint32_t                    layers;
// } VkFramebufferCreateInfo;
pub struct FramebufferCreateInfo/*<'s>*/ {
    pub raw: vks::VkFramebufferCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkAttachmentDescription {
//     VkAttachmentDescriptionFlags    flags;
//     VkFormat                        format;
//     VkSampleCountFlagBits           samples;
//     VkAttachmentLoadOp              loadOp;
//     VkAttachmentStoreOp             storeOp;
//     VkAttachmentLoadOp              stencilLoadOp;
//     VkAttachmentStoreOp             stencilStoreOp;
//     VkImageLayout                   initialLayout;
//     VkImageLayout                   finalLayout;
// } VkAttachmentDescription;
pub struct AttachmentDescription/*<'s>*/ {
    pub raw: vks::VkAttachmentDescription,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkAttachmentReference {
//     uint32_t         attachment;
//     VkImageLayout    layout;
// } VkAttachmentReference;
pub struct AttachmentReference/*<'s>*/ {
    pub raw: vks::VkAttachmentReference,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSubpassDescription {
//     VkSubpassDescriptionFlags       flags;
//     VkPipelineBindPoint             pipelineBindPoint;
//     uint32_t                        inputAttachmentCount;
//     const VkAttachmentReference*    pInputAttachments;
//     uint32_t                        colorAttachmentCount;
//     const VkAttachmentReference*    pColorAttachments;
//     const VkAttachmentReference*    pResolveAttachments;
//     const VkAttachmentReference*    pDepthStencilAttachment;
//     uint32_t                        preserveAttachmentCount;
//     const uint32_t*                 pPreserveAttachments;
// } VkSubpassDescription;
pub struct SubpassDescription/*<'s>*/ {
    pub raw: vks::VkSubpassDescription,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSubpassDependency {
//     uint32_t                srcSubpass;
//     uint32_t                dstSubpass;
//     VkPipelineStageFlags    srcStageMask;
//     VkPipelineStageFlags    dstStageMask;
//     VkAccessFlags           srcAccessMask;
//     VkAccessFlags           dstAccessMask;
//     VkDependencyFlags       dependencyFlags;
// } VkSubpassDependency;
pub struct SubpassDependency/*<'s>*/ {
    pub raw: vks::VkSubpassDependency,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRenderPassCreateInfo {
//     VkStructureType                   sType;
//     const void*                       pNext;
//     VkRenderPassCreateFlags           flags;
//     uint32_t                          attachmentCount;
//     const VkAttachmentDescription*    pAttachments;
//     uint32_t                          subpassCount;
//     const VkSubpassDescription*       pSubpasses;
//     uint32_t                          dependencyCount;
//     const VkSubpassDependency*        pDependencies;
// } VkRenderPassCreateInfo;
pub struct RenderPassCreateInfo/*<'s>*/ {
    pub raw: vks::VkRenderPassCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCommandPoolCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkCommandPoolCreateFlags    flags;
//     uint32_t                    queueFamilyIndex;
// } VkCommandPoolCreateInfo;
pub struct CommandPoolCreateInfo/*<'s>*/ {
    pub raw: vks::VkCommandPoolCreateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCommandBufferAllocateInfo {
//     VkStructureType         sType;
//     const void*             pNext;
//     VkCommandPool           commandPool;
//     VkCommandBufferLevel    level;
//     uint32_t                commandBufferCount;
// } VkCommandBufferAllocateInfo;
pub struct CommandBufferAllocateInfo/*<'s>*/ {
    pub raw: vks::VkCommandBufferAllocateInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCommandBufferInheritanceInfo {
//     VkStructureType                  sType;
//     const void*                      pNext;
//     VkRenderPass                     renderPass;
//     uint32_t                         subpass;
//     VkFramebuffer                    framebuffer;
//     VkBool32                         occlusionQueryEnable;
//     VkQueryControlFlags              queryFlags;
//     VkQueryPipelineStatisticFlags    pipelineStatistics;
// } VkCommandBufferInheritanceInfo;
pub struct CommandBufferInheritanceInfo/*<'s>*/ {
    pub raw: vks::VkCommandBufferInheritanceInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCommandBufferBeginInfo {
//     VkStructureType                          sType;
//     const void*                              pNext;
//     VkCommandBufferUsageFlags                flags;
//     const VkCommandBufferInheritanceInfo*    pInheritanceInfo;
// } VkCommandBufferBeginInfo;
pub struct CommandBufferBeginInfo/*<'s>*/ {
    pub raw: vks::VkCommandBufferBeginInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBufferCopy {
//     VkDeviceSize    srcOffset;
//     VkDeviceSize    dstOffset;
//     VkDeviceSize    size;
// } VkBufferCopy;
pub struct BufferCopy/*<'s>*/ {
    pub raw: vks::VkBufferCopy,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageSubresourceLayers {
//     VkImageAspectFlags    aspectMask;
//     uint32_t              mipLevel;
//     uint32_t              baseArrayLayer;
//     uint32_t              layerCount;
// } VkImageSubresourceLayers;
pub struct ImageSubresourceLayers/*<'s>*/ {
    pub raw: vks::VkImageSubresourceLayers,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageCopy {
//     VkImageSubresourceLayers    srcSubresource;
//     VkOffset3D                  srcOffset;
//     VkImageSubresourceLayers    dstSubresource;
//     VkOffset3D                  dstOffset;
//     VkExtent3D                  extent;
// } VkImageCopy;
pub struct ImageCopy/*<'s>*/ {
    pub raw: vks::VkImageCopy,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageBlit {
//     VkImageSubresourceLayers    srcSubresource;
//     VkOffset3D                  srcOffsets[2];
//     VkImageSubresourceLayers    dstSubresource;
//     VkOffset3D                  dstOffsets[2];
// } VkImageBlit;
pub struct ImageBlit/*<'s>*/ {
    pub raw: vks::VkImageBlit,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBufferImageCopy {
//     VkDeviceSize                bufferOffset;
//     uint32_t                    bufferRowLength;
//     uint32_t                    bufferImageHeight;
//     VkImageSubresourceLayers    imageSubresource;
//     VkOffset3D                  imageOffset;
//     VkExtent3D                  imageExtent;
// } VkBufferImageCopy;
pub struct BufferImageCopy/*<'s>*/ {
    pub raw: vks::VkBufferImageCopy,
    // _p: PhantomData<&'s ()>,
}


// typedef union VkClearColorValue {
//     float       float32[4];
//     int32_t     int32[4];
//     uint32_t    uint32[4];
// } VkClearColorValue;
pub enum ClearColorValue {
    Float32([f32; 4]),
    Int32([i32; 4]),
    Uint32([u32; 4]),
}


// typedef struct VkClearDepthStencilValue {
//     float       depth;
//     uint32_t    stencil;
// } VkClearDepthStencilValue;
pub struct ClearDepthStencilValue/*<'s>*/ {
    pub raw: vks::VkClearDepthStencilValue,
    // _p: PhantomData<&'s ()>,
}


// typedef union VkClearValue {
//     VkClearColorValue           color;
//     VkClearDepthStencilValue    depthStencil;
// } VkClearValue;
pub enum ClearValue/*<'s>*/ {
    Color(ClearColorValue),
    DepthStencil(ClearDepthStencilValue/*<'s>*/),
}

// typedef struct VkClearAttachment {
//     VkImageAspectFlags    aspectMask;
//     uint32_t              colorAttachment;
//     VkClearValue          clearValue;
// } VkClearAttachment;
pub struct ClearAttachment/*<'s>*/ {
    pub raw: vks::VkClearAttachment,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkClearRect {
//     VkRect2D    rect;
//     uint32_t    baseArrayLayer;
//     uint32_t    layerCount;
// } VkClearRect;
pub struct ClearRect/*<'s>*/ {
    pub raw: vks::VkClearRect,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageResolve {
//     VkImageSubresourceLayers    srcSubresource;
//     VkOffset3D                  srcOffset;
//     VkImageSubresourceLayers    dstSubresource;
//     VkOffset3D                  dstOffset;
//     VkExtent3D                  extent;
// } VkImageResolve;
pub struct ImageResolve/*<'s>*/ {
    pub raw: vks::VkImageResolve,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryBarrier {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkAccessFlags      srcAccessMask;
//     VkAccessFlags      dstAccessMask;
// } VkMemoryBarrier;
pub struct MemoryBarrier/*<'s>*/ {
    pub raw: vks::VkMemoryBarrier,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBufferMemoryBarrier {
//     VkStructureType    sType;
//     const void*        pNext;
//     VkAccessFlags      srcAccessMask;
//     VkAccessFlags      dstAccessMask;
//     uint32_t           srcQueueFamilyIndex;
//     uint32_t           dstQueueFamilyIndex;
//     VkBuffer           buffer;
//     VkDeviceSize       offset;
//     VkDeviceSize       size;
// } VkBufferMemoryBarrier;
pub struct BufferMemoryBarrier/*<'s>*/ {
    pub raw: vks::VkBufferMemoryBarrier,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageMemoryBarrier {
//     VkStructureType            sType;
//     const void*                pNext;
//     VkAccessFlags              srcAccessMask;
//     VkAccessFlags              dstAccessMask;
//     VkImageLayout              oldLayout;
//     VkImageLayout              newLayout;
//     uint32_t                   srcQueueFamilyIndex;
//     uint32_t                   dstQueueFamilyIndex;
//     VkImage                    image;
//     VkImageSubresourceRange    subresourceRange;
// } VkImageMemoryBarrier;
pub struct ImageMemoryBarrier/*<'s>*/ {
    pub raw: vks::VkImageMemoryBarrier,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRenderPassBeginInfo {
//     VkStructureType        sType;
//     const void*            pNext;
//     VkRenderPass           renderPass;
//     VkFramebuffer          framebuffer;
//     VkRect2D               renderArea;
//     uint32_t               clearValueCount;
//     const VkClearValue*    pClearValues;
// } VkRenderPassBeginInfo;
pub struct RenderPassBeginInfo/*<'s>*/ {
    pub raw: vks::VkRenderPassBeginInfo,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDispatchIndirectCommand {
//     uint32_t    x;
//     uint32_t    y;
//     uint32_t    z;
// } VkDispatchIndirectCommand;
pub struct DispatchIndirectCommand/*<'s>*/ {
    pub raw: vks::VkDispatchIndirectCommand,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDrawIndexedIndirectCommand {
//     uint32_t    indexCount;
//     uint32_t    instanceCount;
//     uint32_t    firstIndex;
//     int32_t     vertexOffset;
//     uint32_t    firstInstance;
// } VkDrawIndexedIndirectCommand;
pub struct DrawIndexedIndirectCommand/*<'s>*/ {
    pub raw: vks::VkDrawIndexedIndirectCommand,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDrawIndirectCommand {
//     uint32_t    vertexCount;
//     uint32_t    instanceCount;
//     uint32_t    firstVertex;
//     uint32_t    firstInstance;
// } VkDrawIndirectCommand;
pub struct DrawIndirectCommand/*<'s>*/ {
    pub raw: vks::VkDrawIndirectCommand,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSurfaceCapabilitiesKHR
pub struct SurfaceCapabilitiesKHR/*<'s>*/ {
    pub raw: vks::VkSurfaceCapabilitiesKHR,
    // _p: PhantomData<&'s ()>,
}


// // typedef struct VkSurfaceFormatKHR
pub struct SurfaceFormatKHR/*<'s>*/ {
    pub raw: vks::VkSurfaceFormatKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSwapchainCreateInfoKHR
pub struct SwapchainCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkSwapchainCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPresentInfoKHR
pub struct PresentInfoKHR/*<'s>*/ {
    pub raw: vks::VkPresentInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayPropertiesKHR
pub struct DisplayPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkDisplayPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayModeParametersKHR
pub struct DisplayModeParametersKHR/*<'s>*/ {
    pub raw: vks::VkDisplayModeParametersKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayModePropertiesKHR
pub struct DisplayModePropertiesKHR/*<'s>*/ {
    pub raw: vks::VkDisplayModePropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayModeCreateInfoKHR
pub struct DisplayModeCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkDisplayModeCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayPlaneCapabilitiesKHR
pub struct DisplayPlaneCapabilitiesKHR/*<'s>*/ {
    pub raw: vks::VkDisplayPlaneCapabilitiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayPlanePropertiesKHR
pub struct DisplayPlanePropertiesKHR/*<'s>*/ {
    pub raw: vks::VkDisplayPlanePropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplaySurfaceCreateInfoKHR
pub struct DisplaySurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkDisplaySurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayPresentInfoKHR
pub struct DisplayPresentInfoKHR/*<'s>*/ {
    pub raw: vks::VkDisplayPresentInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkXlibSurfaceCreateInfoKHR
pub struct XlibSurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkXlibSurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkXcbSurfaceCreateInfoKHR
pub struct XcbSurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkXcbSurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkWaylandSurfaceCreateInfoKHR
pub struct WaylandSurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkWaylandSurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMirSurfaceCreateInfoKHR
pub struct MirSurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkMirSurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkAndroidSurfaceCreateInfoKHR
pub struct AndroidSurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkAndroidSurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkWin32SurfaceCreateInfoKHR
pub struct Win32SurfaceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkWin32SurfaceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceFeatures2KHR
pub struct PhysicalDeviceFeatures2KHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceFeatures2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceProperties2KHR
pub struct PhysicalDeviceProperties2KHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceProperties2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkFormatProperties2KHR
pub struct FormatProperties2KHR/*<'s>*/ {
    pub raw: vks::VkFormatProperties2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageFormatProperties2KHR
pub struct ImageFormatProperties2KHR/*<'s>*/ {
    pub raw: vks::VkImageFormatProperties2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceImageFormatInfo2KHR
pub struct PhysicalDeviceImageFormatInfo2KHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceImageFormatInfo2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkQueueFamilyProperties2KHR
pub struct QueueFamilyProperties2KHR/*<'s>*/ {
    pub raw: vks::VkQueueFamilyProperties2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceMemoryProperties2KHR
pub struct PhysicalDeviceMemoryProperties2KHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceMemoryProperties2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageFormatProperties2KHR
pub struct SparseImageFormatProperties2KHR/*<'s>*/ {
    pub raw: vks::VkSparseImageFormatProperties2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceSparseImageFormatInfo2KHR
pub struct PhysicalDeviceSparseImageFormatInfo2KHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalMemoryPropertiesKHR
pub struct ExternalMemoryPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkExternalMemoryPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceExternalImageFormatInfoKHR
pub struct PhysicalDeviceExternalImageFormatInfoKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceExternalImageFormatInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalImageFormatPropertiesKHR
pub struct ExternalImageFormatPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkExternalImageFormatPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceExternalBufferInfoKHR
pub struct PhysicalDeviceExternalBufferInfoKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceExternalBufferInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalBufferPropertiesKHR
pub struct ExternalBufferPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkExternalBufferPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceIDPropertiesKHR
pub struct PhysicalDeviceIDPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceIDPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalMemoryImageCreateInfoKHR
pub struct ExternalMemoryImageCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkExternalMemoryImageCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalMemoryBufferCreateInfoKHR
pub struct ExternalMemoryBufferCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkExternalMemoryBufferCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportMemoryAllocateInfoKHR
pub struct ExportMemoryAllocateInfoKHR/*<'s>*/ {
    pub raw: vks::VkExportMemoryAllocateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportMemoryWin32HandleInfoKHR
pub struct ImportMemoryWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkImportMemoryWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportMemoryWin32HandleInfoKHR
pub struct ExportMemoryWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkExportMemoryWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryWin32HandlePropertiesKHR
pub struct MemoryWin32HandlePropertiesKHR/*<'s>*/ {
    pub raw: vks::VkMemoryWin32HandlePropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryGetWin32HandleInfoKHR
pub struct MemoryGetWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkMemoryGetWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportMemoryFdInfoKHR
pub struct ImportMemoryFdInfoKHR/*<'s>*/ {
    pub raw: vks::VkImportMemoryFdInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryFdPropertiesKHR
pub struct MemoryFdPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkMemoryFdPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryGetFdInfoKHR
pub struct MemoryGetFdInfoKHR/*<'s>*/ {
    pub raw: vks::VkMemoryGetFdInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkWin32KeyedMutexAcquireReleaseInfoKHR
pub struct Win32KeyedMutexAcquireReleaseInfoKHR/*<'s>*/ {
    pub raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceExternalSemaphoreInfoKHR
pub struct PhysicalDeviceExternalSemaphoreInfoKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceExternalSemaphoreInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalSemaphorePropertiesKHR
pub struct ExternalSemaphorePropertiesKHR/*<'s>*/ {
    pub raw: vks::VkExternalSemaphorePropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportSemaphoreCreateInfoKHR
pub struct ExportSemaphoreCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkExportSemaphoreCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportSemaphoreWin32HandleInfoKHR
pub struct ImportSemaphoreWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkImportSemaphoreWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportSemaphoreWin32HandleInfoKHR
pub struct ExportSemaphoreWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkExportSemaphoreWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkD3D12FenceSubmitInfoKHR
pub struct D3D12FenceSubmitInfoKHR/*<'s>*/ {
    pub raw: vks::VkD3D12FenceSubmitInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSemaphoreGetWin32HandleInfoKHR
pub struct SemaphoreGetWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkSemaphoreGetWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportSemaphoreFdInfoKHR
pub struct ImportSemaphoreFdInfoKHR/*<'s>*/ {
    pub raw: vks::VkImportSemaphoreFdInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSemaphoreGetFdInfoKHR
pub struct SemaphoreGetFdInfoKHR/*<'s>*/ {
    pub raw: vks::VkSemaphoreGetFdInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDevicePushDescriptorPropertiesKHR
pub struct PhysicalDevicePushDescriptorPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDevicePushDescriptorPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDevice16BitStorageFeaturesKHR
pub struct PhysicalDevice16BitStorageFeaturesKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDevice16BitStorageFeaturesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRectLayerKHR
pub struct RectLayerKHR/*<'s>*/ {
    pub raw: vks::VkRectLayerKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPresentRegionKHR
pub struct PresentRegionKHR/*<'s>*/ {
    pub raw: vks::VkPresentRegionKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPresentRegionsKHR
pub struct PresentRegionsKHR/*<'s>*/ {
    pub raw: vks::VkPresentRegionsKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorUpdateTemplateEntryKHR
pub struct DescriptorUpdateTemplateEntryKHR/*<'s>*/ {
    pub raw: vks::VkDescriptorUpdateTemplateEntryKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDescriptorUpdateTemplateCreateInfoKHR
pub struct DescriptorUpdateTemplateCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkDescriptorUpdateTemplateCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSharedPresentSurfaceCapabilitiesKHR
pub struct SharedPresentSurfaceCapabilitiesKHR/*<'s>*/ {
    pub raw: vks::VkSharedPresentSurfaceCapabilitiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceExternalFenceInfoKHR
pub struct PhysicalDeviceExternalFenceInfoKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceExternalFenceInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalFencePropertiesKHR
pub struct ExternalFencePropertiesKHR/*<'s>*/ {
    pub raw: vks::VkExternalFencePropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportFenceCreateInfoKHR
pub struct ExportFenceCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkExportFenceCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportFenceWin32HandleInfoKHR
pub struct ImportFenceWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkImportFenceWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportFenceWin32HandleInfoKHR
pub struct ExportFenceWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkExportFenceWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkFenceGetWin32HandleInfoKHR
pub struct FenceGetWin32HandleInfoKHR/*<'s>*/ {
    pub raw: vks::VkFenceGetWin32HandleInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportFenceFdInfoKHR
pub struct ImportFenceFdInfoKHR/*<'s>*/ {
    pub raw: vks::VkImportFenceFdInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkFenceGetFdInfoKHR
pub struct FenceGetFdInfoKHR/*<'s>*/ {
    pub raw: vks::VkFenceGetFdInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDevicePointClippingPropertiesKHR
#[cfg(feature = "experimental")]
pub struct PhysicalDevicePointClippingPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDevicePointClippingPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkInputAttachmentAspectReferenceKHR
#[cfg(feature = "experimental")]
pub struct InputAttachmentAspectReferenceKHR/*<'s>*/ {
    pub raw: vks::VkInputAttachmentAspectReferenceKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRenderPassInputAttachmentAspectCreateInfoKHR
#[cfg(feature = "experimental")]
pub struct RenderPassInputAttachmentAspectCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkRenderPassInputAttachmentAspectCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageViewUsageCreateInfoKHR
#[cfg(feature = "experimental")]
pub struct ImageViewUsageCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkImageViewUsageCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineTessellationDomainOriginStateCreateInfoKHR
#[cfg(feature = "experimental")]
pub struct PipelineTessellationDomainOriginStateCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkPipelineTessellationDomainOriginStateCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceSurfaceInfo2KHR
pub struct PhysicalDeviceSurfaceInfo2KHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceSurfaceInfo2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSurfaceCapabilities2KHR
pub struct SurfaceCapabilities2KHR/*<'s>*/ {
    pub raw: vks::VkSurfaceCapabilities2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSurfaceFormat2KHR
pub struct SurfaceFormat2KHR/*<'s>*/ {
    pub raw: vks::VkSurfaceFormat2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceVariablePointerFeaturesKHR
pub struct PhysicalDeviceVariablePointerFeaturesKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceVariablePointerFeaturesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryDedicatedRequirementsKHR
pub struct MemoryDedicatedRequirementsKHR/*<'s>*/ {
    pub raw: vks::VkMemoryDedicatedRequirementsKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryDedicatedAllocateInfoKHR
pub struct MemoryDedicatedAllocateInfoKHR/*<'s>*/ {
    pub raw: vks::VkMemoryDedicatedAllocateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBufferMemoryRequirementsInfo2KHR
pub struct BufferMemoryRequirementsInfo2KHR/*<'s>*/ {
    pub raw: vks::VkBufferMemoryRequirementsInfo2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageMemoryRequirementsInfo2KHR
pub struct ImageMemoryRequirementsInfo2KHR/*<'s>*/ {
    pub raw: vks::VkImageMemoryRequirementsInfo2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageSparseMemoryRequirementsInfo2KHR
pub struct ImageSparseMemoryRequirementsInfo2KHR/*<'s>*/ {
    pub raw: vks::VkImageSparseMemoryRequirementsInfo2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryRequirements2KHR
pub struct MemoryRequirements2KHR/*<'s>*/ {
    pub raw: vks::VkMemoryRequirements2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSparseImageMemoryRequirements2KHR
pub struct SparseImageMemoryRequirements2KHR/*<'s>*/ {
    pub raw: vks::VkSparseImageMemoryRequirements2KHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageFormatListCreateInfoKHR
#[cfg(feature = "experimental")]
pub struct ImageFormatListCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkImageFormatListCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSamplerYcbcrConversionCreateInfoKHR
#[cfg(feature = "experimental")]
pub struct SamplerYcbcrConversionCreateInfoKHR/*<'s>*/ {
    pub raw: vks::VkSamplerYcbcrConversionCreateInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSamplerYcbcrConversionInfoKHR
#[cfg(feature = "experimental")]
pub struct SamplerYcbcrConversionInfoKHR/*<'s>*/ {
    pub raw: vks::VkSamplerYcbcrConversionInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindImagePlaneMemoryInfoKHR
#[cfg(feature = "experimental")]
pub struct BindImagePlaneMemoryInfoKHR/*<'s>*/ {
    pub raw: vks::VkBindImagePlaneMemoryInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImagePlaneMemoryRequirementsInfoKHR
#[cfg(feature = "experimental")]
pub struct ImagePlaneMemoryRequirementsInfoKHR/*<'s>*/ {
    pub raw: vks::VkImagePlaneMemoryRequirementsInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
#[cfg(feature = "experimental")]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesKHR/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSamplerYcbcrConversionImageFormatPropertiesKHR
#[cfg(feature = "experimental")]
pub struct SamplerYcbcrConversionImageFormatPropertiesKHR/*<'s>*/ {
    pub raw: vks::VkSamplerYcbcrConversionImageFormatPropertiesKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindBufferMemoryInfoKHR
#[cfg(feature = "experimental")]
pub struct BindBufferMemoryInfoKHR/*<'s>*/ {
    pub raw: vks::VkBindBufferMemoryInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindImageMemoryInfoKHR
#[cfg(feature = "experimental")]
pub struct BindImageMemoryInfoKHR/*<'s>*/ {
    pub raw: vks::VkBindImageMemoryInfoKHR,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDebugReportCallbackCreateInfoEXT
pub struct DebugReportCallbackCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkDebugReportCallbackCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineRasterizationStateRasterizationOrderAMD
pub struct PipelineRasterizationStateRasterizationOrderAMD/*<'s>*/ {
    pub raw: vks::VkPipelineRasterizationStateRasterizationOrderAMD,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDebugMarkerObjectNameInfoEXT
pub struct DebugMarkerObjectNameInfoEXT/*<'s>*/ {
    pub raw: vks::VkDebugMarkerObjectNameInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDebugMarkerObjectTagInfoEXT
pub struct DebugMarkerObjectTagInfoEXT/*<'s>*/ {
    pub raw: vks::VkDebugMarkerObjectTagInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDebugMarkerMarkerInfoEXT
pub struct DebugMarkerMarkerInfoEXT/*<'s>*/ {
    pub raw: vks::VkDebugMarkerMarkerInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDedicatedAllocationImageCreateInfoNV
pub struct DedicatedAllocationImageCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkDedicatedAllocationImageCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDedicatedAllocationBufferCreateInfoNV
pub struct DedicatedAllocationBufferCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkDedicatedAllocationBufferCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDedicatedAllocationMemoryAllocateInfoNV
pub struct DedicatedAllocationMemoryAllocateInfoNV/*<'s>*/ {
    pub raw: vks::VkDedicatedAllocationMemoryAllocateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkTextureLODGatherFormatPropertiesAMD
pub struct TextureLODGatherFormatPropertiesAMD/*<'s>*/ {
    pub raw: vks::VkTextureLODGatherFormatPropertiesAMD,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRenderPassMultiviewCreateInfoKHX
#[cfg(feature = "experimental")]
pub struct RenderPassMultiviewCreateInfoKHX/*<'s>*/ {
    pub raw: vks::VkRenderPassMultiviewCreateInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceMultiviewFeaturesKHX
#[cfg(feature = "experimental")]
pub struct PhysicalDeviceMultiviewFeaturesKHX/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceMultiviewFeaturesKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceMultiviewPropertiesKHX
#[cfg(feature = "experimental")]
pub struct PhysicalDeviceMultiviewPropertiesKHX/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceMultiviewPropertiesKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalImageFormatPropertiesNV
pub struct ExternalImageFormatPropertiesNV/*<'s>*/ {
    pub raw: vks::VkExternalImageFormatPropertiesNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExternalMemoryImageCreateInfoNV
pub struct ExternalMemoryImageCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkExternalMemoryImageCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportMemoryAllocateInfoNV
pub struct ExportMemoryAllocateInfoNV/*<'s>*/ {
    pub raw: vks::VkExportMemoryAllocateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImportMemoryWin32HandleInfoNV
pub struct ImportMemoryWin32HandleInfoNV/*<'s>*/ {
    pub raw: vks::VkImportMemoryWin32HandleInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkExportMemoryWin32HandleInfoNV
pub struct ExportMemoryWin32HandleInfoNV/*<'s>*/ {
    pub raw: vks::VkExportMemoryWin32HandleInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkWin32KeyedMutexAcquireReleaseInfoNV
pub struct Win32KeyedMutexAcquireReleaseInfoNV/*<'s>*/ {
    pub raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMemoryAllocateFlagsInfoKHX
#[cfg(feature = "experimental")]
pub struct MemoryAllocateFlagsInfoKHX/*<'s>*/ {
    pub raw: vks::VkMemoryAllocateFlagsInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupRenderPassBeginInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupRenderPassBeginInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupRenderPassBeginInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupCommandBufferBeginInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupCommandBufferBeginInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupCommandBufferBeginInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupSubmitInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupSubmitInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupSubmitInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupBindSparseInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupBindSparseInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupBindSparseInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindBufferMemoryDeviceGroupInfoKHX
#[cfg(feature = "experimental")]
pub struct BindBufferMemoryDeviceGroupInfoKHX/*<'s>*/ {
    pub raw: vks::VkBindBufferMemoryDeviceGroupInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindImageMemoryDeviceGroupInfoKHX
#[cfg(feature = "experimental")]
pub struct BindImageMemoryDeviceGroupInfoKHX/*<'s>*/ {
    pub raw: vks::VkBindImageMemoryDeviceGroupInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupPresentCapabilitiesKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupPresentCapabilitiesKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupPresentCapabilitiesKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkImageSwapchainCreateInfoKHX
#[cfg(feature = "experimental")]
pub struct ImageSwapchainCreateInfoKHX/*<'s>*/ {
    pub raw: vks::VkImageSwapchainCreateInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkBindImageMemorySwapchainInfoKHX
#[cfg(feature = "experimental")]
pub struct BindImageMemorySwapchainInfoKHX/*<'s>*/ {
    pub raw: vks::VkBindImageMemorySwapchainInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkAcquireNextImageInfoKHX
#[cfg(feature = "experimental")]
pub struct AcquireNextImageInfoKHX/*<'s>*/ {
    pub raw: vks::VkAcquireNextImageInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupPresentInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupPresentInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupPresentInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupSwapchainCreateInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupSwapchainCreateInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupSwapchainCreateInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkValidationFlagsEXT
pub struct ValidationFlagsEXT/*<'s>*/ {
    pub raw: vks::VkValidationFlagsEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkViSurfaceCreateInfoNN
pub struct ViSurfaceCreateInfoNN/*<'s>*/ {
    pub raw: vks::VkViSurfaceCreateInfoNN,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceGroupPropertiesKHX
#[cfg(feature = "experimental")]
pub struct PhysicalDeviceGroupPropertiesKHX/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceGroupPropertiesKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGroupDeviceCreateInfoKHX
#[cfg(feature = "experimental")]
pub struct DeviceGroupDeviceCreateInfoKHX/*<'s>*/ {
    pub raw: vks::VkDeviceGroupDeviceCreateInfoKHX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGeneratedCommandsFeaturesNVX
#[cfg(feature = "experimental")]
pub struct DeviceGeneratedCommandsFeaturesNVX/*<'s>*/ {
    pub raw: vks::VkDeviceGeneratedCommandsFeaturesNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceGeneratedCommandsLimitsNVX
#[cfg(feature = "experimental")]
pub struct DeviceGeneratedCommandsLimitsNVX/*<'s>*/ {
    pub raw: vks::VkDeviceGeneratedCommandsLimitsNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkIndirectCommandsTokenNVX
#[cfg(feature = "experimental")]
pub struct IndirectCommandsTokenNVX/*<'s>*/ {
    pub raw: vks::VkIndirectCommandsTokenNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkIndirectCommandsLayoutTokenNVX
#[cfg(feature = "experimental")]
pub struct IndirectCommandsLayoutTokenNVX/*<'s>*/ {
    pub raw: vks::VkIndirectCommandsLayoutTokenNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkIndirectCommandsLayoutCreateInfoNVX
#[cfg(feature = "experimental")]
pub struct IndirectCommandsLayoutCreateInfoNVX/*<'s>*/ {
    pub raw: vks::VkIndirectCommandsLayoutCreateInfoNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCmdProcessCommandsInfoNVX
#[cfg(feature = "experimental")]
pub struct CmdProcessCommandsInfoNVX/*<'s>*/ {
    pub raw: vks::VkCmdProcessCommandsInfoNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkCmdReserveSpaceForCommandsInfoNVX
#[cfg(feature = "experimental")]
pub struct CmdReserveSpaceForCommandsInfoNVX/*<'s>*/ {
    pub raw: vks::VkCmdReserveSpaceForCommandsInfoNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTableCreateInfoNVX
#[cfg(feature = "experimental")]
pub struct ObjectTableCreateInfoNVX/*<'s>*/ {
    pub raw: vks::VkObjectTableCreateInfoNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTableEntryNVX
#[cfg(feature = "experimental")]
pub struct ObjectTableEntryNVX/*<'s>*/ {
    pub raw: vks::VkObjectTableEntryNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTablePipelineEntryNVX
#[cfg(feature = "experimental")]
pub struct ObjectTablePipelineEntryNVX/*<'s>*/ {
    pub raw: vks::VkObjectTablePipelineEntryNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTableDescriptorSetEntryNVX
#[cfg(feature = "experimental")]
pub struct ObjectTableDescriptorSetEntryNVX/*<'s>*/ {
    pub raw: vks::VkObjectTableDescriptorSetEntryNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTableVertexBufferEntryNVX
#[cfg(feature = "experimental")]
pub struct ObjectTableVertexBufferEntryNVX/*<'s>*/ {
    pub raw: vks::VkObjectTableVertexBufferEntryNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTableIndexBufferEntryNVX
#[cfg(feature = "experimental")]
pub struct ObjectTableIndexBufferEntryNVX/*<'s>*/ {
    pub raw: vks::VkObjectTableIndexBufferEntryNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkObjectTablePushConstantEntryNVX
#[cfg(feature = "experimental")]
pub struct ObjectTablePushConstantEntryNVX/*<'s>*/ {
    pub raw: vks::VkObjectTablePushConstantEntryNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkViewportWScalingNV
pub struct ViewportWScalingNV/*<'s>*/ {
    pub raw: vks::VkViewportWScalingNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineViewportWScalingStateCreateInfoNV
pub struct PipelineViewportWScalingStateCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkPipelineViewportWScalingStateCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSurfaceCapabilities2EXT
pub struct SurfaceCapabilities2EXT/*<'s>*/ {
    pub raw: vks::VkSurfaceCapabilities2EXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayPowerInfoEXT
pub struct DisplayPowerInfoEXT/*<'s>*/ {
    pub raw: vks::VkDisplayPowerInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDeviceEventInfoEXT
pub struct DeviceEventInfoEXT/*<'s>*/ {
    pub raw: vks::VkDeviceEventInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkDisplayEventInfoEXT
pub struct DisplayEventInfoEXT/*<'s>*/ {
    pub raw: vks::VkDisplayEventInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSwapchainCounterCreateInfoEXT
pub struct SwapchainCounterCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkSwapchainCounterCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRefreshCycleDurationGOOGLE
pub struct RefreshCycleDurationGOOGLE/*<'s>*/ {
    pub raw: vks::VkRefreshCycleDurationGOOGLE,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPastPresentationTimingGOOGLE
pub struct PastPresentationTimingGOOGLE/*<'s>*/ {
    pub raw: vks::VkPastPresentationTimingGOOGLE,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPresentTimeGOOGLE
pub struct PresentTimeGOOGLE/*<'s>*/ {
    pub raw: vks::VkPresentTimeGOOGLE,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPresentTimesInfoGOOGLE
pub struct PresentTimesInfoGOOGLE/*<'s>*/ {
    pub raw: vks::VkPresentTimesInfoGOOGLE,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
#[cfg(feature = "experimental")]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkViewportSwizzleNV
pub struct ViewportSwizzleNV/*<'s>*/ {
    pub raw: vks::VkViewportSwizzleNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineViewportSwizzleStateCreateInfoNV
pub struct PipelineViewportSwizzleStateCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkPipelineViewportSwizzleStateCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceDiscardRectanglePropertiesEXT
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineDiscardRectangleStateCreateInfoEXT
pub struct PipelineDiscardRectangleStateCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkPipelineDiscardRectangleStateCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkXYColorEXT
pub struct XYColorEXT/*<'s>*/ {
    pub raw: vks::VkXYColorEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkHdrMetadataEXT
pub struct HdrMetadataEXT/*<'s>*/ {
    pub raw: vks::VkHdrMetadataEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkIOSSurfaceCreateInfoMVK
pub struct IOSSurfaceCreateInfoMVK/*<'s>*/ {
    pub raw: vks::VkIOSSurfaceCreateInfoMVK,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMacOSSurfaceCreateInfoMVK
pub struct MacOSSurfaceCreateInfoMVK/*<'s>*/ {
    pub raw: vks::VkMacOSSurfaceCreateInfoMVK,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSamplerReductionModeCreateInfoEXT
pub struct SamplerReductionModeCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkSamplerReductionModeCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
pub struct PhysicalDeviceSamplerFilterMinmaxPropertiesEXT/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSampleLocationEXT
#[cfg(feature = "experimental")]
pub struct SampleLocationEXT/*<'s>*/ {
    pub raw: vks::VkSampleLocationEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSampleLocationsInfoEXT
#[cfg(feature = "experimental")]
pub struct SampleLocationsInfoEXT/*<'s>*/ {
    pub raw: vks::VkSampleLocationsInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkAttachmentSampleLocationsEXT
#[cfg(feature = "experimental")]
pub struct AttachmentSampleLocationsEXT/*<'s>*/ {
    pub raw: vks::VkAttachmentSampleLocationsEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkSubpassSampleLocationsEXT
#[cfg(feature = "experimental")]
pub struct SubpassSampleLocationsEXT/*<'s>*/ {
    pub raw: vks::VkSubpassSampleLocationsEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkRenderPassSampleLocationsBeginInfoEXT
#[cfg(feature = "experimental")]
pub struct RenderPassSampleLocationsBeginInfoEXT/*<'s>*/ {
    pub raw: vks::VkRenderPassSampleLocationsBeginInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineSampleLocationsStateCreateInfoEXT
#[cfg(feature = "experimental")]
pub struct PipelineSampleLocationsStateCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkPipelineSampleLocationsStateCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceSampleLocationsPropertiesEXT
#[cfg(feature = "experimental")]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceSampleLocationsPropertiesEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkMultisamplePropertiesEXT
#[cfg(feature = "experimental")]
pub struct MultisamplePropertiesEXT/*<'s>*/ {
    pub raw: vks::VkMultisamplePropertiesEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT/*<'s>*/ {
    pub raw: vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineColorBlendAdvancedStateCreateInfoEXT
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineCoverageToColorStateCreateInfoNV
pub struct PipelineCoverageToColorStateCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkPipelineCoverageToColorStateCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkPipelineCoverageModulationStateCreateInfoNV
pub struct PipelineCoverageModulationStateCreateInfoNV/*<'s>*/ {
    pub raw: vks::VkPipelineCoverageModulationStateCreateInfoNV,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkValidationCacheCreateInfoEXT
#[cfg(feature = "experimental")]
pub struct ValidationCacheCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkValidationCacheCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}


// typedef struct VkShaderModuleValidationCacheCreateInfoEXT
#[cfg(feature = "experimental")]
pub struct ShaderModuleValidationCacheCreateInfoEXT/*<'s>*/ {
    pub raw: vks::VkShaderModuleValidationCacheCreateInfoEXT,
    // _p: PhantomData<&'s ()>,
}

