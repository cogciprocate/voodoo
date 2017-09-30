//! Structs.

use std::ptr;
use std::ffi::{CString, CStr};
use std::marker::PhantomData;
use ::*;
use vks;
use vks::{PFN_vkAllocationFunction, PFN_vkReallocationFunction, PFN_vkFreeFunction, 
    PFN_vkInternalAllocationNotification, PFN_vkInternalFreeNotification, 
    PFN_vkDebugReportCallbackEXT};



/// A `VkOffset2D`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Offset2d {
    raw: vks::VkOffset2D,
}

impl Offset2d {
    pub fn builder() -> Offset2dBuilder {
        Offset2dBuilder::new()
    }

    pub fn x(&self) {
    }

    pub fn y(&self) {
    }

    pub fn raw(&self) -> &vks::VkOffset2D {
        &self.raw
    }
}


/// A builder for `VkOffset2D`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Offset2dBuilder {
    raw: vks::VkOffset2D,
}

impl Offset2dBuilder {
    pub fn new() -> Offset2dBuilder {
        Offset2dBuilder {
            raw: vks::VkOffset2D::default(),
        }
    }

    pub fn x<'m>(&'m mut self, x: i32) -> &'m mut Offset2dBuilder {
        self
    }

    pub fn y<'m>(&'m mut self, y: i32) -> &'m mut Offset2dBuilder {
        self
    }

}


/// A `VkOffset3D`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Offset3d {
    raw: vks::VkOffset3D,
}

impl Offset3d {
    pub fn builder() -> Offset3dBuilder {
        Offset3dBuilder::new()
    }

    pub fn x(&self) {
    }

    pub fn y(&self) {
    }

    pub fn z(&self) {
    }

    pub fn raw(&self) -> &vks::VkOffset3D {
        &self.raw
    }
}


/// A builder for `VkOffset3D`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Offset3dBuilder {
    raw: vks::VkOffset3D,
}

impl Offset3dBuilder {
    pub fn new() -> Offset3dBuilder {
        Offset3dBuilder {
            raw: vks::VkOffset3D::default(),
        }
    }

    pub fn x<'m>(&'m mut self, x: i32) -> &'m mut Offset3dBuilder {
        self
    }

    pub fn y<'m>(&'m mut self, y: i32) -> &'m mut Offset3dBuilder {
        self
    }

    pub fn z<'m>(&'m mut self, z: i32) -> &'m mut Offset3dBuilder {
        self
    }

}


/// A `VkExtent2D`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Extent2d {
    raw: vks::VkExtent2D,
}

impl Extent2d {
    pub fn builder() -> Extent2dBuilder {
        Extent2dBuilder::new()
    }

    pub fn width(&self) {
    }

    pub fn height(&self) {
    }

    pub fn raw(&self) -> &vks::VkExtent2D {
        &self.raw
    }
}


/// A builder for `VkExtent2D`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Extent2dBuilder {
    raw: vks::VkExtent2D,
}

impl Extent2dBuilder {
    pub fn new() -> Extent2dBuilder {
        Extent2dBuilder {
            raw: vks::VkExtent2D::default(),
        }
    }

    pub fn width<'m>(&'m mut self, width: u32) -> &'m mut Extent2dBuilder {
        self
    }

    pub fn height<'m>(&'m mut self, height: u32) -> &'m mut Extent2dBuilder {
        self
    }

}


/// A `VkExtent3D`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Extent3d {
    raw: vks::VkExtent3D,
}

impl Extent3d {
    pub fn builder() -> Extent3dBuilder {
        Extent3dBuilder::new()
    }

    pub fn width(&self) {
    }

    pub fn height(&self) {
    }

    pub fn depth(&self) {
    }

    pub fn raw(&self) -> &vks::VkExtent3D {
        &self.raw
    }
}


/// A builder for `VkExtent3D`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Extent3dBuilder {
    raw: vks::VkExtent3D,
}

impl Extent3dBuilder {
    pub fn new() -> Extent3dBuilder {
        Extent3dBuilder {
            raw: vks::VkExtent3D::default(),
        }
    }

    pub fn width<'m>(&'m mut self, width: u32) -> &'m mut Extent3dBuilder {
        self
    }

    pub fn height<'m>(&'m mut self, height: u32) -> &'m mut Extent3dBuilder {
        self
    }

    pub fn depth<'m>(&'m mut self, depth: u32) -> &'m mut Extent3dBuilder {
        self
    }

}


/// A `VkViewport`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Viewport {
    raw: vks::VkViewport,
}

impl Viewport {
    pub fn builder() -> ViewportBuilder {
        ViewportBuilder::new()
    }

    pub fn x(&self) {
    }

    pub fn y(&self) {
    }

    pub fn width(&self) {
    }

    pub fn height(&self) {
    }

    pub fn min_depth(&self) {
    }

    pub fn max_depth(&self) {
    }

    pub fn raw(&self) -> &vks::VkViewport {
        &self.raw
    }
}


/// A builder for `VkViewport`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ViewportBuilder {
    raw: vks::VkViewport,
}

impl ViewportBuilder {
    pub fn new() -> ViewportBuilder {
        ViewportBuilder {
            raw: vks::VkViewport::default(),
        }
    }

    pub fn x<'m>(&'m mut self, x: f32) -> &'m mut ViewportBuilder {
        self
    }

    pub fn y<'m>(&'m mut self, y: f32) -> &'m mut ViewportBuilder {
        self
    }

    pub fn width<'m>(&'m mut self, width: f32) -> &'m mut ViewportBuilder {
        self
    }

    pub fn height<'m>(&'m mut self, height: f32) -> &'m mut ViewportBuilder {
        self
    }

    pub fn min_depth<'m>(&'m mut self, min_depth: f32) -> &'m mut ViewportBuilder {
        self
    }

    pub fn max_depth<'m>(&'m mut self, max_depth: f32) -> &'m mut ViewportBuilder {
        self
    }

}


/// A `VkRect2D`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Rect2d {
    raw: vks::VkRect2D,
}

impl Rect2d {
    pub fn builder() -> Rect2dBuilder {
        Rect2dBuilder::new()
    }

    pub fn offset(&self) {
    }

    pub fn extent(&self) {
    }

    pub fn raw(&self) -> &vks::VkRect2D {
        &self.raw
    }
}


/// A builder for `VkRect2D`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Rect2dBuilder {
    raw: vks::VkRect2D,
}

impl Rect2dBuilder {
    pub fn new() -> Rect2dBuilder {
        Rect2dBuilder {
            raw: vks::VkRect2D::default(),
        }
    }

    pub fn offset<'m>(&'m mut self, offset: Offset2d) -> &'m mut Rect2dBuilder {
        self
    }

    pub fn extent<'m>(&'m mut self, extent: Extent2d) -> &'m mut Rect2dBuilder {
        self
    }

}


/// A `VkClearRect`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ClearRect {
    raw: vks::VkClearRect,
}

impl ClearRect {
    pub fn builder() -> ClearRectBuilder {
        ClearRectBuilder::new()
    }

    pub fn rect(&self) {
    }

    pub fn base_array_layer(&self) {
    }

    pub fn layer_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkClearRect {
        &self.raw
    }
}


/// A builder for `VkClearRect`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ClearRectBuilder {
    raw: vks::VkClearRect,
}

impl ClearRectBuilder {
    pub fn new() -> ClearRectBuilder {
        ClearRectBuilder {
            raw: vks::VkClearRect::default(),
        }
    }

    pub fn rect<'m>(&'m mut self, rect: Rect2d) -> &'m mut ClearRectBuilder {
        self
    }

    pub fn base_array_layer<'m>(&'m mut self, base_array_layer: u32) -> &'m mut ClearRectBuilder {
        self
    }

    pub fn layer_count<'m>(&'m mut self, layer_count: u32) -> &'m mut ClearRectBuilder {
        self
    }

}


/// A `VkComponentMapping`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ComponentMapping {
    raw: vks::VkComponentMapping,
}

impl ComponentMapping {
    pub fn builder() -> ComponentMappingBuilder {
        ComponentMappingBuilder::new()
    }

    pub fn r(&self) {
    }

    pub fn g(&self) {
    }

    pub fn b(&self) {
    }

    pub fn a(&self) {
    }

    pub fn raw(&self) -> &vks::VkComponentMapping {
        &self.raw
    }
}


/// A builder for `VkComponentMapping`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ComponentMappingBuilder {
    raw: vks::VkComponentMapping,
}

impl ComponentMappingBuilder {
    pub fn new() -> ComponentMappingBuilder {
        ComponentMappingBuilder {
            raw: vks::VkComponentMapping::default(),
        }
    }

    pub fn r<'m>(&'m mut self, r: ComponentSwizzle) -> &'m mut ComponentMappingBuilder {
        self
    }

    pub fn g<'m>(&'m mut self, g: ComponentSwizzle) -> &'m mut ComponentMappingBuilder {
        self
    }

    pub fn b<'m>(&'m mut self, b: ComponentSwizzle) -> &'m mut ComponentMappingBuilder {
        self
    }

    pub fn a<'m>(&'m mut self, a: ComponentSwizzle) -> &'m mut ComponentMappingBuilder {
        self
    }

}


/// A `VkPhysicalDeviceProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceProperties {
    raw: vks::VkPhysicalDeviceProperties,
}

impl PhysicalDeviceProperties {
    pub fn api_version(&self) {
    }

    pub fn driver_version(&self) {
    }

    pub fn vendor_id(&self) {
    }

    pub fn device_id(&self) {
    }

    pub fn device_type(&self) {
    }

    pub fn device_name(&self) {
    }

    pub fn pipeline_cache_uu_id(&self) {
    }

    pub fn limits(&self) {
    }

    pub fn sparse_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceProperties {
        &self.raw
    }
}


/// A `VkExtensionProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExtensionProperties {
    raw: vks::VkExtensionProperties,
}

impl ExtensionProperties {
    pub fn extension_name(&self) {
    }

    pub fn spec_version(&self) {
    }

    pub fn raw(&self) -> &vks::VkExtensionProperties {
        &self.raw
    }
}


/// A `VkLayerProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct LayerProperties {
    raw: vks::VkLayerProperties,
}

impl LayerProperties {
    pub fn layer_name(&self) {
    }

    pub fn spec_version(&self) {
    }

    pub fn implementation_version(&self) {
    }

    pub fn description(&self) {
    }

    pub fn raw(&self) -> &vks::VkLayerProperties {
        &self.raw
    }
}


/// A `VkApplicationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ApplicationInfo<'s> {
    raw: vks::VkApplicationInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> ApplicationInfo<'s> {
    pub fn builder<'b>() -> ApplicationInfoBuilder<'b> {
        ApplicationInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn application_name(&self) {
    }

    pub fn application_version(&self) {
    }

    pub fn engine_name(&self) {
    }

    pub fn engine_version(&self) {
    }

    pub fn api_version(&self) {
    }

    pub fn raw(&self) -> &vks::VkApplicationInfo {
        &self.raw
    }
}


/// A builder for `VkApplicationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ApplicationInfoBuilder<'b> {
    raw: vks::VkApplicationInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> ApplicationInfoBuilder<'b> {
    pub fn new() -> ApplicationInfoBuilder<'b> {
        ApplicationInfoBuilder {
            raw: vks::VkApplicationInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ApplicationInfoBuilder<'b> {
        self
    }

    pub fn application_name<'m, 'a>(&'m mut self, application_name: &'a i8) -> &'m mut ApplicationInfoBuilder<'b> {
        self
    }

    pub fn application_version<'m>(&'m mut self, application_version: u32) -> &'m mut ApplicationInfoBuilder<'b> {
        self
    }

    pub fn engine_name<'m, 'a>(&'m mut self, engine_name: &'a i8) -> &'m mut ApplicationInfoBuilder<'b> {
        self
    }

    pub fn engine_version<'m>(&'m mut self, engine_version: u32) -> &'m mut ApplicationInfoBuilder<'b> {
        self
    }

    pub fn api_version<'m>(&'m mut self, api_version: u32) -> &'m mut ApplicationInfoBuilder<'b> {
        self
    }

}


/// A `VkAllocationCallbacks`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AllocationCallbacks<'s> {
    raw: vks::VkAllocationCallbacks,
    _p: PhantomData<&'s ()>,
}

impl<'s> AllocationCallbacks<'s> {
    pub fn builder<'b>() -> AllocationCallbacksBuilder<'b> {
        AllocationCallbacksBuilder::new()
    }

    pub fn user_data(&self) {
    }

    pub fn pfn_allocation(&self) {
    }

    pub fn pfn_reallocation(&self) {
    }

    pub fn pfn_free(&self) {
    }

    pub fn pfn_internal_allocation(&self) {
    }

    pub fn pfn_internal_free(&self) {
    }

    pub fn raw(&self) -> &vks::VkAllocationCallbacks {
        &self.raw
    }
}


/// A builder for `VkAllocationCallbacks`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct AllocationCallbacksBuilder<'b> {
    raw: vks::VkAllocationCallbacks,
    _p: PhantomData<&'b ()>,
}

impl<'b> AllocationCallbacksBuilder<'b> {
    pub fn new() -> AllocationCallbacksBuilder<'b> {
        AllocationCallbacksBuilder {
            raw: vks::VkAllocationCallbacks::default(),
            _p: PhantomData,
        }
    }

    pub fn user_data<'m, 'a>(&'m mut self, user_data: &'a ()) -> &'m mut AllocationCallbacksBuilder<'b> {
        self
    }

    pub fn pfn_allocation<'m>(&'m mut self, pfn_allocation: PFN_vkAllocationFunction) -> &'m mut AllocationCallbacksBuilder<'b> {
        self
    }

    pub fn pfn_reallocation<'m>(&'m mut self, pfn_reallocation: PFN_vkReallocationFunction) -> &'m mut AllocationCallbacksBuilder<'b> {
        self
    }

    pub fn pfn_free<'m>(&'m mut self, pfn_free: PFN_vkFreeFunction) -> &'m mut AllocationCallbacksBuilder<'b> {
        self
    }

    pub fn pfn_internal_allocation<'m>(&'m mut self, pfn_internal_allocation: PFN_vkInternalAllocationNotification) -> &'m mut AllocationCallbacksBuilder<'b> {
        self
    }

    pub fn pfn_internal_free<'m>(&'m mut self, pfn_internal_free: PFN_vkInternalFreeNotification) -> &'m mut AllocationCallbacksBuilder<'b> {
        self
    }

}


/// A `VkDeviceQueueCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceQueueCreateInfo<'s> {
    raw: vks::VkDeviceQueueCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> DeviceQueueCreateInfo<'s> {
    pub fn builder<'b>() -> DeviceQueueCreateInfoBuilder<'b> {
        DeviceQueueCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn queue_family_index(&self) {
    }

    pub fn queue_count(&self) {
    }

    pub fn queue_priorities(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceQueueCreateInfo {
        &self.raw
    }
}


/// A builder for `VkDeviceQueueCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DeviceQueueCreateInfoBuilder<'b> {
    raw: vks::VkDeviceQueueCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> DeviceQueueCreateInfoBuilder<'b> {
    pub fn new() -> DeviceQueueCreateInfoBuilder<'b> {
        DeviceQueueCreateInfoBuilder {
            raw: vks::VkDeviceQueueCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceQueueCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DeviceQueueCreateFlags) -> &'m mut DeviceQueueCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_family_index<'m>(&'m mut self, queue_family_index: u32) -> &'m mut DeviceQueueCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_count<'m>(&'m mut self, queue_count: u32) -> &'m mut DeviceQueueCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_priorities<'m, 'a>(&'m mut self, queue_priorities: &'a f32) -> &'m mut DeviceQueueCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkDeviceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceCreateInfo<'s> {
    raw: vks::VkDeviceCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> DeviceCreateInfo<'s> {
    pub fn builder<'b>() -> DeviceCreateInfoBuilder<'b> {
        DeviceCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn queue_create_info_count(&self) {
    }

    pub fn queue_create_infos(&self) {
    }

    pub fn enabled_layer_count(&self) {
    }

    pub fn enabled_layer_names(&self) {
    }

    pub fn enabled_extension_count(&self) {
    }

    pub fn enabled_extension_names(&self) {
    }

    pub fn enabled_features(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceCreateInfo {
        &self.raw
    }
}


/// A builder for `VkDeviceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DeviceCreateInfoBuilder<'b> {
    raw: vks::VkDeviceCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> DeviceCreateInfoBuilder<'b> {
    pub fn new() -> DeviceCreateInfoBuilder<'b> {
        DeviceCreateInfoBuilder {
            raw: vks::VkDeviceCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DeviceCreateFlags) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_create_info_count<'m>(&'m mut self, queue_create_info_count: u32) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_create_infos<'m, 'a>(&'m mut self, queue_create_infos: &'a DeviceQueueCreateInfo) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_layer_count<'m>(&'m mut self, enabled_layer_count: u32) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_layer_names<'m>(&'m mut self, enabled_layer_names: i8) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_extension_count<'m>(&'m mut self, enabled_extension_count: u32) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_extension_names<'m>(&'m mut self, enabled_extension_names: i8) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_features<'m, 'a>(&'m mut self, enabled_features: &'a PhysicalDeviceFeatures) -> &'m mut DeviceCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkInstanceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct InstanceCreateInfo<'s> {
    raw: vks::VkInstanceCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> InstanceCreateInfo<'s> {
    pub fn builder<'b>() -> InstanceCreateInfoBuilder<'b> {
        InstanceCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn application_info(&self) {
    }

    pub fn enabled_layer_count(&self) {
    }

    pub fn enabled_layer_names(&self) {
    }

    pub fn enabled_extension_count(&self) {
    }

    pub fn enabled_extension_names(&self) {
    }

    pub fn raw(&self) -> &vks::VkInstanceCreateInfo {
        &self.raw
    }
}


/// A builder for `VkInstanceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct InstanceCreateInfoBuilder<'b> {
    raw: vks::VkInstanceCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> InstanceCreateInfoBuilder<'b> {
    pub fn new() -> InstanceCreateInfoBuilder<'b> {
        InstanceCreateInfoBuilder {
            raw: vks::VkInstanceCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: InstanceCreateFlags) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

    pub fn application_info<'m, 'a>(&'m mut self, application_info: &'a ApplicationInfo) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_layer_count<'m>(&'m mut self, enabled_layer_count: u32) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_layer_names<'m>(&'m mut self, enabled_layer_names: i8) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_extension_count<'m>(&'m mut self, enabled_extension_count: u32) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

    pub fn enabled_extension_names<'m>(&'m mut self, enabled_extension_names: i8) -> &'m mut InstanceCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkQueueFamilyProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct QueueFamilyProperties {
    raw: vks::VkQueueFamilyProperties,
}

impl QueueFamilyProperties {
    pub fn queue_flags(&self) {
    }

    pub fn queue_count(&self) {
    }

    pub fn timestamp_valid_bits(&self) {
    }

    pub fn min_image_transfer_granularity(&self) {
    }

    pub fn raw(&self) -> &vks::VkQueueFamilyProperties {
        &self.raw
    }
}


/// A `VkPhysicalDeviceMemoryProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    raw: vks::VkPhysicalDeviceMemoryProperties,
}

impl PhysicalDeviceMemoryProperties {
    pub fn memory_type_count(&self) {
    }

    pub fn memory_types(&self) {
    }

    pub fn memory_heap_count(&self) {
    }

    pub fn memory_heaps(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceMemoryProperties {
        &self.raw
    }
}


/// A `VkMemoryAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryAllocateInfo<'s> {
    raw: vks::VkMemoryAllocateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryAllocateInfo<'s> {
    pub fn builder<'b>() -> MemoryAllocateInfoBuilder<'b> {
        MemoryAllocateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn allocation_size(&self) {
    }

    pub fn memory_type_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryAllocateInfo {
        &self.raw
    }
}


/// A builder for `VkMemoryAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MemoryAllocateInfoBuilder<'b> {
    raw: vks::VkMemoryAllocateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> MemoryAllocateInfoBuilder<'b> {
    pub fn new() -> MemoryAllocateInfoBuilder<'b> {
        MemoryAllocateInfoBuilder {
            raw: vks::VkMemoryAllocateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MemoryAllocateInfoBuilder<'b> {
        self
    }

    pub fn allocation_size<'m>(&'m mut self, allocation_size: u64) -> &'m mut MemoryAllocateInfoBuilder<'b> {
        self
    }

    pub fn memory_type_index<'m>(&'m mut self, memory_type_index: u32) -> &'m mut MemoryAllocateInfoBuilder<'b> {
        self
    }

}


/// A `VkMemoryRequirements`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryRequirements {
    raw: vks::VkMemoryRequirements,
}

impl MemoryRequirements {
    pub fn size(&self) {
    }

    pub fn alignment(&self) {
    }

    pub fn memory_type_bits(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryRequirements {
        &self.raw
    }
}


/// A `VkSparseImageFormatProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageFormatProperties {
    raw: vks::VkSparseImageFormatProperties,
}

impl SparseImageFormatProperties {
    pub fn aspect_mask(&self) {
    }

    pub fn image_granularity(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageFormatProperties {
        &self.raw
    }
}


/// A `VkSparseImageMemoryRequirements`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageMemoryRequirements {
    raw: vks::VkSparseImageMemoryRequirements,
}

impl SparseImageMemoryRequirements {
    pub fn format_properties(&self) {
    }

    pub fn image_mip_tail_first_lod(&self) {
    }

    pub fn image_mip_tail_size(&self) {
    }

    pub fn image_mip_tail_offset(&self) {
    }

    pub fn image_mip_tail_stride(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageMemoryRequirements {
        &self.raw
    }
}


/// A `VkMemoryType`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryType {
    raw: vks::VkMemoryType,
}

impl MemoryType {
    pub fn property_flags(&self) {
    }

    pub fn heap_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryType {
        &self.raw
    }
}


/// A `VkMemoryHeap`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryHeap {
    raw: vks::VkMemoryHeap,
}

impl MemoryHeap {
    pub fn size(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryHeap {
        &self.raw
    }
}


/// A `VkMappedMemoryRange`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MappedMemoryRange<'s> {
    raw: vks::VkMappedMemoryRange,
    _p: PhantomData<&'s ()>,
}

impl<'s> MappedMemoryRange<'s> {
    pub fn builder<'b>() -> MappedMemoryRangeBuilder<'b> {
        MappedMemoryRangeBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn memory(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn raw(&self) -> &vks::VkMappedMemoryRange {
        &self.raw
    }
}


/// A builder for `VkMappedMemoryRange`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MappedMemoryRangeBuilder<'b> {
    raw: vks::VkMappedMemoryRange,
    _p: PhantomData<&'b ()>,
}

impl<'b> MappedMemoryRangeBuilder<'b> {
    pub fn new() -> MappedMemoryRangeBuilder<'b> {
        MappedMemoryRangeBuilder {
            raw: vks::VkMappedMemoryRange::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MappedMemoryRangeBuilder<'b> {
        self
    }

    pub fn memory<'m>(&'m mut self, memory: DeviceMemory) -> &'m mut MappedMemoryRangeBuilder<'b> {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u64) -> &'m mut MappedMemoryRangeBuilder<'b> {
        self
    }

    pub fn size<'m>(&'m mut self, size: u64) -> &'m mut MappedMemoryRangeBuilder<'b> {
        self
    }

}


/// A `VkFormatProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FormatProperties {
    raw: vks::VkFormatProperties,
}

impl FormatProperties {
    pub fn linear_tiling_features(&self) {
    }

    pub fn optimal_tiling_features(&self) {
    }

    pub fn buffer_features(&self) {
    }

    pub fn raw(&self) -> &vks::VkFormatProperties {
        &self.raw
    }
}


/// A `VkImageFormatProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageFormatProperties {
    raw: vks::VkImageFormatProperties,
}

impl ImageFormatProperties {
    pub fn max_extent(&self) {
    }

    pub fn max_mip_levels(&self) {
    }

    pub fn max_array_layers(&self) {
    }

    pub fn sample_counts(&self) {
    }

    pub fn max_resource_size(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageFormatProperties {
        &self.raw
    }
}


/// A `VkDescriptorBufferInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorBufferInfo {
    raw: vks::VkDescriptorBufferInfo,
}

impl DescriptorBufferInfo {
    pub fn builder() -> DescriptorBufferInfoBuilder {
        DescriptorBufferInfoBuilder::new()
    }

    pub fn buffer(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn range(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorBufferInfo {
        &self.raw
    }
}


/// A builder for `VkDescriptorBufferInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorBufferInfoBuilder {
    raw: vks::VkDescriptorBufferInfo,
}

impl DescriptorBufferInfoBuilder {
    pub fn new() -> DescriptorBufferInfoBuilder {
        DescriptorBufferInfoBuilder {
            raw: vks::VkDescriptorBufferInfo::default(),
        }
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut DescriptorBufferInfoBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u64) -> &'m mut DescriptorBufferInfoBuilder {
        self
    }

    pub fn range<'m>(&'m mut self, range: u64) -> &'m mut DescriptorBufferInfoBuilder {
        self
    }

}


/// A `VkDescriptorImageInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorImageInfo {
    raw: vks::VkDescriptorImageInfo,
}

impl DescriptorImageInfo {
    pub fn builder() -> DescriptorImageInfoBuilder {
        DescriptorImageInfoBuilder::new()
    }

    pub fn sampler(&self) {
    }

    pub fn image_view(&self) {
    }

    pub fn image_layout(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorImageInfo {
        &self.raw
    }
}


/// A builder for `VkDescriptorImageInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorImageInfoBuilder {
    raw: vks::VkDescriptorImageInfo,
}

impl DescriptorImageInfoBuilder {
    pub fn new() -> DescriptorImageInfoBuilder {
        DescriptorImageInfoBuilder {
            raw: vks::VkDescriptorImageInfo::default(),
        }
    }

    pub fn sampler<'m>(&'m mut self, sampler: Sampler) -> &'m mut DescriptorImageInfoBuilder {
        self
    }

    pub fn image_view<'m>(&'m mut self, image_view: ImageView) -> &'m mut DescriptorImageInfoBuilder {
        self
    }

    pub fn image_layout<'m>(&'m mut self, image_layout: ImageLayout) -> &'m mut DescriptorImageInfoBuilder {
        self
    }

}


/// A `VkWriteDescriptorSet`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct WriteDescriptorSet<'s> {
    raw: vks::VkWriteDescriptorSet,
    _p: PhantomData<&'s ()>,
}

impl<'s> WriteDescriptorSet<'s> {
    pub fn builder<'b>() -> WriteDescriptorSetBuilder<'b> {
        WriteDescriptorSetBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn dst_set(&self) {
    }

    pub fn dst_binding(&self) {
    }

    pub fn dst_array_element(&self) {
    }

    pub fn descriptor_count(&self) {
    }

    pub fn descriptor_type(&self) {
    }

    pub fn image_info(&self) {
    }

    pub fn buffer_info(&self) {
    }

    pub fn texel_buffer_view(&self) {
    }

    pub fn raw(&self) -> &vks::VkWriteDescriptorSet {
        &self.raw
    }
}


/// A builder for `VkWriteDescriptorSet`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct WriteDescriptorSetBuilder<'b> {
    raw: vks::VkWriteDescriptorSet,
    _p: PhantomData<&'b ()>,
}

impl<'b> WriteDescriptorSetBuilder<'b> {
    pub fn new() -> WriteDescriptorSetBuilder<'b> {
        WriteDescriptorSetBuilder {
            raw: vks::VkWriteDescriptorSet::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn dst_set<'m>(&'m mut self, dst_set: DescriptorSet) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn dst_binding<'m>(&'m mut self, dst_binding: u32) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn dst_array_element<'m>(&'m mut self, dst_array_element: u32) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn descriptor_count<'m>(&'m mut self, descriptor_count: u32) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn descriptor_type<'m>(&'m mut self, descriptor_type: DescriptorType) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn image_info<'m, 'a>(&'m mut self, image_info: &'a DescriptorImageInfo) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn buffer_info<'m, 'a>(&'m mut self, buffer_info: &'a DescriptorBufferInfo) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

    pub fn texel_buffer_view<'m, 'a>(&'m mut self, texel_buffer_view: &'a BufferView) -> &'m mut WriteDescriptorSetBuilder<'b> {
        self
    }

}


/// A `VkCopyDescriptorSet`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CopyDescriptorSet<'s> {
    raw: vks::VkCopyDescriptorSet,
    _p: PhantomData<&'s ()>,
}

impl<'s> CopyDescriptorSet<'s> {
    pub fn builder<'b>() -> CopyDescriptorSetBuilder<'b> {
        CopyDescriptorSetBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn src_set(&self) {
    }

    pub fn src_binding(&self) {
    }

    pub fn src_array_element(&self) {
    }

    pub fn dst_set(&self) {
    }

    pub fn dst_binding(&self) {
    }

    pub fn dst_array_element(&self) {
    }

    pub fn descriptor_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkCopyDescriptorSet {
        &self.raw
    }
}


/// A builder for `VkCopyDescriptorSet`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct CopyDescriptorSetBuilder<'b> {
    raw: vks::VkCopyDescriptorSet,
    _p: PhantomData<&'b ()>,
}

impl<'b> CopyDescriptorSetBuilder<'b> {
    pub fn new() -> CopyDescriptorSetBuilder<'b> {
        CopyDescriptorSetBuilder {
            raw: vks::VkCopyDescriptorSet::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn src_set<'m>(&'m mut self, src_set: DescriptorSet) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn src_binding<'m>(&'m mut self, src_binding: u32) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn src_array_element<'m>(&'m mut self, src_array_element: u32) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn dst_set<'m>(&'m mut self, dst_set: DescriptorSet) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn dst_binding<'m>(&'m mut self, dst_binding: u32) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn dst_array_element<'m>(&'m mut self, dst_array_element: u32) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

    pub fn descriptor_count<'m>(&'m mut self, descriptor_count: u32) -> &'m mut CopyDescriptorSetBuilder<'b> {
        self
    }

}


/// A `VkBufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BufferCreateInfo<'s> {
    raw: vks::VkBufferCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> BufferCreateInfo<'s> {
    pub fn builder<'b>() -> BufferCreateInfoBuilder<'b> {
        BufferCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn size(&self) {
    }

    pub fn usage(&self) {
    }

    pub fn sharing_mode(&self) {
    }

    pub fn queue_family_index_count(&self) {
    }

    pub fn queue_family_indices(&self) {
    }

    pub fn raw(&self) -> &vks::VkBufferCreateInfo {
        &self.raw
    }
}


/// A builder for `VkBufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BufferCreateInfoBuilder<'b> {
    raw: vks::VkBufferCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> BufferCreateInfoBuilder<'b> {
    pub fn new() -> BufferCreateInfoBuilder<'b> {
        BufferCreateInfoBuilder {
            raw: vks::VkBufferCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: BufferCreateFlags) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

    pub fn size<'m>(&'m mut self, size: u64) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

    pub fn usage<'m>(&'m mut self, usage: BufferUsageFlags) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

    pub fn sharing_mode<'m>(&'m mut self, sharing_mode: SharingMode) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_family_index_count<'m>(&'m mut self, queue_family_index_count: u32) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_family_indices<'m, 'a>(&'m mut self, queue_family_indices: &'a u32) -> &'m mut BufferCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkBufferViewCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BufferViewCreateInfo<'s> {
    raw: vks::VkBufferViewCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> BufferViewCreateInfo<'s> {
    pub fn builder<'b>() -> BufferViewCreateInfoBuilder<'b> {
        BufferViewCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn format(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn range(&self) {
    }

    pub fn raw(&self) -> &vks::VkBufferViewCreateInfo {
        &self.raw
    }
}


/// A builder for `VkBufferViewCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BufferViewCreateInfoBuilder<'b> {
    raw: vks::VkBufferViewCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> BufferViewCreateInfoBuilder<'b> {
    pub fn new() -> BufferViewCreateInfoBuilder<'b> {
        BufferViewCreateInfoBuilder {
            raw: vks::VkBufferViewCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BufferViewCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: BufferViewCreateFlags) -> &'m mut BufferViewCreateInfoBuilder<'b> {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut BufferViewCreateInfoBuilder<'b> {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut BufferViewCreateInfoBuilder<'b> {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u64) -> &'m mut BufferViewCreateInfoBuilder<'b> {
        self
    }

    pub fn range<'m>(&'m mut self, range: u64) -> &'m mut BufferViewCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkImageSubresource`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageSubresource {
    raw: vks::VkImageSubresource,
}

impl ImageSubresource {
    pub fn builder() -> ImageSubresourceBuilder {
        ImageSubresourceBuilder::new()
    }

    pub fn aspect_mask(&self) {
    }

    pub fn mip_level(&self) {
    }

    pub fn array_layer(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageSubresource {
        &self.raw
    }
}


/// A builder for `VkImageSubresource`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageSubresourceBuilder {
    raw: vks::VkImageSubresource,
}

impl ImageSubresourceBuilder {
    pub fn new() -> ImageSubresourceBuilder {
        ImageSubresourceBuilder {
            raw: vks::VkImageSubresource::default(),
        }
    }

    pub fn aspect_mask<'m>(&'m mut self, aspect_mask: ImageAspectFlags) -> &'m mut ImageSubresourceBuilder {
        self
    }

    pub fn mip_level<'m>(&'m mut self, mip_level: u32) -> &'m mut ImageSubresourceBuilder {
        self
    }

    pub fn array_layer<'m>(&'m mut self, array_layer: u32) -> &'m mut ImageSubresourceBuilder {
        self
    }

}


/// A `VkImageSubresourceLayers`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageSubresourceLayers {
    raw: vks::VkImageSubresourceLayers,
}

impl ImageSubresourceLayers {
    pub fn builder() -> ImageSubresourceLayersBuilder {
        ImageSubresourceLayersBuilder::new()
    }

    pub fn aspect_mask(&self) {
    }

    pub fn mip_level(&self) {
    }

    pub fn base_array_layer(&self) {
    }

    pub fn layer_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageSubresourceLayers {
        &self.raw
    }
}


/// A builder for `VkImageSubresourceLayers`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageSubresourceLayersBuilder {
    raw: vks::VkImageSubresourceLayers,
}

impl ImageSubresourceLayersBuilder {
    pub fn new() -> ImageSubresourceLayersBuilder {
        ImageSubresourceLayersBuilder {
            raw: vks::VkImageSubresourceLayers::default(),
        }
    }

    pub fn aspect_mask<'m>(&'m mut self, aspect_mask: ImageAspectFlags) -> &'m mut ImageSubresourceLayersBuilder {
        self
    }

    pub fn mip_level<'m>(&'m mut self, mip_level: u32) -> &'m mut ImageSubresourceLayersBuilder {
        self
    }

    pub fn base_array_layer<'m>(&'m mut self, base_array_layer: u32) -> &'m mut ImageSubresourceLayersBuilder {
        self
    }

    pub fn layer_count<'m>(&'m mut self, layer_count: u32) -> &'m mut ImageSubresourceLayersBuilder {
        self
    }

}


/// A `VkImageSubresourceRange`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageSubresourceRange {
    raw: vks::VkImageSubresourceRange,
}

impl ImageSubresourceRange {
    pub fn builder() -> ImageSubresourceRangeBuilder {
        ImageSubresourceRangeBuilder::new()
    }

    pub fn aspect_mask(&self) {
    }

    pub fn base_mip_level(&self) {
    }

    pub fn level_count(&self) {
    }

    pub fn base_array_layer(&self) {
    }

    pub fn layer_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageSubresourceRange {
        &self.raw
    }
}


/// A builder for `VkImageSubresourceRange`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageSubresourceRangeBuilder {
    raw: vks::VkImageSubresourceRange,
}

impl ImageSubresourceRangeBuilder {
    pub fn new() -> ImageSubresourceRangeBuilder {
        ImageSubresourceRangeBuilder {
            raw: vks::VkImageSubresourceRange::default(),
        }
    }

    pub fn aspect_mask<'m>(&'m mut self, aspect_mask: ImageAspectFlags) -> &'m mut ImageSubresourceRangeBuilder {
        self
    }

    pub fn base_mip_level<'m>(&'m mut self, base_mip_level: u32) -> &'m mut ImageSubresourceRangeBuilder {
        self
    }

    pub fn level_count<'m>(&'m mut self, level_count: u32) -> &'m mut ImageSubresourceRangeBuilder {
        self
    }

    pub fn base_array_layer<'m>(&'m mut self, base_array_layer: u32) -> &'m mut ImageSubresourceRangeBuilder {
        self
    }

    pub fn layer_count<'m>(&'m mut self, layer_count: u32) -> &'m mut ImageSubresourceRangeBuilder {
        self
    }

}


/// A `VkMemoryBarrier`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryBarrier<'s> {
    raw: vks::VkMemoryBarrier,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryBarrier<'s> {
    pub fn builder<'b>() -> MemoryBarrierBuilder<'b> {
        MemoryBarrierBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn src_access_mask(&self) {
    }

    pub fn dst_access_mask(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryBarrier {
        &self.raw
    }
}


/// A builder for `VkMemoryBarrier`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MemoryBarrierBuilder<'b> {
    raw: vks::VkMemoryBarrier,
    _p: PhantomData<&'b ()>,
}

impl<'b> MemoryBarrierBuilder<'b> {
    pub fn new() -> MemoryBarrierBuilder<'b> {
        MemoryBarrierBuilder {
            raw: vks::VkMemoryBarrier::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MemoryBarrierBuilder<'b> {
        self
    }

    pub fn src_access_mask<'m>(&'m mut self, src_access_mask: AccessFlags) -> &'m mut MemoryBarrierBuilder<'b> {
        self
    }

    pub fn dst_access_mask<'m>(&'m mut self, dst_access_mask: AccessFlags) -> &'m mut MemoryBarrierBuilder<'b> {
        self
    }

}


/// A `VkBufferMemoryBarrier`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BufferMemoryBarrier<'s> {
    raw: vks::VkBufferMemoryBarrier,
    _p: PhantomData<&'s ()>,
}

impl<'s> BufferMemoryBarrier<'s> {
    pub fn builder<'b>() -> BufferMemoryBarrierBuilder<'b> {
        BufferMemoryBarrierBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn src_access_mask(&self) {
    }

    pub fn dst_access_mask(&self) {
    }

    pub fn src_queue_family_index(&self) {
    }

    pub fn dst_queue_family_index(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn raw(&self) -> &vks::VkBufferMemoryBarrier {
        &self.raw
    }
}


/// A builder for `VkBufferMemoryBarrier`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BufferMemoryBarrierBuilder<'b> {
    raw: vks::VkBufferMemoryBarrier,
    _p: PhantomData<&'b ()>,
}

impl<'b> BufferMemoryBarrierBuilder<'b> {
    pub fn new() -> BufferMemoryBarrierBuilder<'b> {
        BufferMemoryBarrierBuilder {
            raw: vks::VkBufferMemoryBarrier::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn src_access_mask<'m>(&'m mut self, src_access_mask: AccessFlags) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn dst_access_mask<'m>(&'m mut self, dst_access_mask: AccessFlags) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn src_queue_family_index<'m>(&'m mut self, src_queue_family_index: u32) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn dst_queue_family_index<'m>(&'m mut self, dst_queue_family_index: u32) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u64) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

    pub fn size<'m>(&'m mut self, size: u64) -> &'m mut BufferMemoryBarrierBuilder<'b> {
        self
    }

}


/// A `VkImageMemoryBarrier`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageMemoryBarrier<'s> {
    raw: vks::VkImageMemoryBarrier,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImageMemoryBarrier<'s> {
    pub fn builder<'b>() -> ImageMemoryBarrierBuilder<'b> {
        ImageMemoryBarrierBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn src_access_mask(&self) {
    }

    pub fn dst_access_mask(&self) {
    }

    pub fn old_layout(&self) {
    }

    pub fn new_layout(&self) {
    }

    pub fn src_queue_family_index(&self) {
    }

    pub fn dst_queue_family_index(&self) {
    }

    pub fn image(&self) {
    }

    pub fn subresource_range(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageMemoryBarrier {
        &self.raw
    }
}


/// A builder for `VkImageMemoryBarrier`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageMemoryBarrierBuilder<'b> {
    raw: vks::VkImageMemoryBarrier,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageMemoryBarrierBuilder<'b> {
    pub fn new() -> ImageMemoryBarrierBuilder<'b> {
        ImageMemoryBarrierBuilder {
            raw: vks::VkImageMemoryBarrier::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn src_access_mask<'m>(&'m mut self, src_access_mask: AccessFlags) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn dst_access_mask<'m>(&'m mut self, dst_access_mask: AccessFlags) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn old_layout<'m>(&'m mut self, old_layout: ImageLayout) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn new_layout<'m>(&'m mut self, new_layout: ImageLayout) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn src_queue_family_index<'m>(&'m mut self, src_queue_family_index: u32) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn dst_queue_family_index<'m>(&'m mut self, dst_queue_family_index: u32) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

    pub fn subresource_range<'m>(&'m mut self, subresource_range: ImageSubresourceRange) -> &'m mut ImageMemoryBarrierBuilder<'b> {
        self
    }

}


/// A `VkImageCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageCreateInfo<'s> {
    raw: vks::VkImageCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImageCreateInfo<'s> {
    pub fn builder<'b>() -> ImageCreateInfoBuilder<'b> {
        ImageCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn image_type(&self) {
    }

    pub fn format(&self) {
    }

    pub fn extent(&self) {
    }

    pub fn mip_levels(&self) {
    }

    pub fn array_layers(&self) {
    }

    pub fn samples(&self) {
    }

    pub fn tiling(&self) {
    }

    pub fn usage(&self) {
    }

    pub fn sharing_mode(&self) {
    }

    pub fn queue_family_index_count(&self) {
    }

    pub fn queue_family_indices(&self) {
    }

    pub fn initial_layout(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageCreateInfo {
        &self.raw
    }
}


/// A builder for `VkImageCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageCreateInfoBuilder<'b> {
    raw: vks::VkImageCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageCreateInfoBuilder<'b> {
    pub fn new() -> ImageCreateInfoBuilder<'b> {
        ImageCreateInfoBuilder {
            raw: vks::VkImageCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ImageCreateFlags) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn image_type<'m>(&'m mut self, image_type: ImageType) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn extent<'m>(&'m mut self, extent: Extent3d) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn mip_levels<'m>(&'m mut self, mip_levels: u32) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn array_layers<'m>(&'m mut self, array_layers: u32) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn samples<'m>(&'m mut self, samples: SampleCountFlags) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn tiling<'m>(&'m mut self, tiling: ImageTiling) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn usage<'m>(&'m mut self, usage: ImageUsageFlags) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn sharing_mode<'m>(&'m mut self, sharing_mode: SharingMode) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_family_index_count<'m>(&'m mut self, queue_family_index_count: u32) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_family_indices<'m, 'a>(&'m mut self, queue_family_indices: &'a u32) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

    pub fn initial_layout<'m>(&'m mut self, initial_layout: ImageLayout) -> &'m mut ImageCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkSubresourceLayout`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SubresourceLayout {
    raw: vks::VkSubresourceLayout,
}

impl SubresourceLayout {
    pub fn offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn row_pitch(&self) {
    }

    pub fn array_pitch(&self) {
    }

    pub fn depth_pitch(&self) {
    }

    pub fn raw(&self) -> &vks::VkSubresourceLayout {
        &self.raw
    }
}


/// A `VkImageViewCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageViewCreateInfo<'s> {
    raw: vks::VkImageViewCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImageViewCreateInfo<'s> {
    pub fn builder<'b>() -> ImageViewCreateInfoBuilder<'b> {
        ImageViewCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn image(&self) {
    }

    pub fn view_type(&self) {
    }

    pub fn format(&self) {
    }

    pub fn components(&self) {
    }

    pub fn subresource_range(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageViewCreateInfo {
        &self.raw
    }
}


/// A builder for `VkImageViewCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageViewCreateInfoBuilder<'b> {
    raw: vks::VkImageViewCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageViewCreateInfoBuilder<'b> {
    pub fn new() -> ImageViewCreateInfoBuilder<'b> {
        ImageViewCreateInfoBuilder {
            raw: vks::VkImageViewCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ImageViewCreateFlags) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

    pub fn view_type<'m>(&'m mut self, view_type: ImageViewType) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

    pub fn components<'m>(&'m mut self, components: ComponentMapping) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

    pub fn subresource_range<'m>(&'m mut self, subresource_range: ImageSubresourceRange) -> &'m mut ImageViewCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkBufferCopy`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BufferCopy {
    raw: vks::VkBufferCopy,
}

impl BufferCopy {
    pub fn builder() -> BufferCopyBuilder {
        BufferCopyBuilder::new()
    }

    pub fn src_offset(&self) {
    }

    pub fn dst_offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn raw(&self) -> &vks::VkBufferCopy {
        &self.raw
    }
}


/// A builder for `VkBufferCopy`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BufferCopyBuilder {
    raw: vks::VkBufferCopy,
}

impl BufferCopyBuilder {
    pub fn new() -> BufferCopyBuilder {
        BufferCopyBuilder {
            raw: vks::VkBufferCopy::default(),
        }
    }

    pub fn src_offset<'m>(&'m mut self, src_offset: u64) -> &'m mut BufferCopyBuilder {
        self
    }

    pub fn dst_offset<'m>(&'m mut self, dst_offset: u64) -> &'m mut BufferCopyBuilder {
        self
    }

    pub fn size<'m>(&'m mut self, size: u64) -> &'m mut BufferCopyBuilder {
        self
    }

}


/// A `VkSparseMemoryBind`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseMemoryBind {
    raw: vks::VkSparseMemoryBind,
}

impl SparseMemoryBind {
    pub fn builder() -> SparseMemoryBindBuilder {
        SparseMemoryBindBuilder::new()
    }

    pub fn resource_offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn memory(&self) {
    }

    pub fn memory_offset(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseMemoryBind {
        &self.raw
    }
}


/// A builder for `VkSparseMemoryBind`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SparseMemoryBindBuilder {
    raw: vks::VkSparseMemoryBind,
}

impl SparseMemoryBindBuilder {
    pub fn new() -> SparseMemoryBindBuilder {
        SparseMemoryBindBuilder {
            raw: vks::VkSparseMemoryBind::default(),
        }
    }

    pub fn resource_offset<'m>(&'m mut self, resource_offset: u64) -> &'m mut SparseMemoryBindBuilder {
        self
    }

    pub fn size<'m>(&'m mut self, size: u64) -> &'m mut SparseMemoryBindBuilder {
        self
    }

    pub fn memory<'m>(&'m mut self, memory: DeviceMemory) -> &'m mut SparseMemoryBindBuilder {
        self
    }

    pub fn memory_offset<'m>(&'m mut self, memory_offset: u64) -> &'m mut SparseMemoryBindBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SparseMemoryBindFlags) -> &'m mut SparseMemoryBindBuilder {
        self
    }

}


/// A `VkSparseImageMemoryBind`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageMemoryBind {
    raw: vks::VkSparseImageMemoryBind,
}

impl SparseImageMemoryBind {
    pub fn builder() -> SparseImageMemoryBindBuilder {
        SparseImageMemoryBindBuilder::new()
    }

    pub fn subresource(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn extent(&self) {
    }

    pub fn memory(&self) {
    }

    pub fn memory_offset(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageMemoryBind {
        &self.raw
    }
}


/// A builder for `VkSparseImageMemoryBind`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SparseImageMemoryBindBuilder {
    raw: vks::VkSparseImageMemoryBind,
}

impl SparseImageMemoryBindBuilder {
    pub fn new() -> SparseImageMemoryBindBuilder {
        SparseImageMemoryBindBuilder {
            raw: vks::VkSparseImageMemoryBind::default(),
        }
    }

    pub fn subresource<'m>(&'m mut self, subresource: ImageSubresource) -> &'m mut SparseImageMemoryBindBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: Offset3d) -> &'m mut SparseImageMemoryBindBuilder {
        self
    }

    pub fn extent<'m>(&'m mut self, extent: Extent3d) -> &'m mut SparseImageMemoryBindBuilder {
        self
    }

    pub fn memory<'m>(&'m mut self, memory: DeviceMemory) -> &'m mut SparseImageMemoryBindBuilder {
        self
    }

    pub fn memory_offset<'m>(&'m mut self, memory_offset: u64) -> &'m mut SparseImageMemoryBindBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SparseMemoryBindFlags) -> &'m mut SparseImageMemoryBindBuilder {
        self
    }

}


/// A `VkSparseBufferMemoryBindInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseBufferMemoryBindInfo<'s> {
    raw: vks::VkSparseBufferMemoryBindInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SparseBufferMemoryBindInfo<'s> {
    pub fn builder<'b>() -> SparseBufferMemoryBindInfoBuilder<'b> {
        SparseBufferMemoryBindInfoBuilder::new()
    }

    pub fn buffer(&self) {
    }

    pub fn bind_count(&self) {
    }

    pub fn binds(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseBufferMemoryBindInfo {
        &self.raw
    }
}


/// A builder for `VkSparseBufferMemoryBindInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SparseBufferMemoryBindInfoBuilder<'b> {
    raw: vks::VkSparseBufferMemoryBindInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SparseBufferMemoryBindInfoBuilder<'b> {
    pub fn new() -> SparseBufferMemoryBindInfoBuilder<'b> {
        SparseBufferMemoryBindInfoBuilder {
            raw: vks::VkSparseBufferMemoryBindInfo::default(),
            _p: PhantomData,
        }
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut SparseBufferMemoryBindInfoBuilder<'b> {
        self
    }

    pub fn bind_count<'m>(&'m mut self, bind_count: u32) -> &'m mut SparseBufferMemoryBindInfoBuilder<'b> {
        self
    }

    pub fn binds<'m, 'a>(&'m mut self, binds: &'a SparseMemoryBind) -> &'m mut SparseBufferMemoryBindInfoBuilder<'b> {
        self
    }

}


/// A `VkSparseImageOpaqueMemoryBindInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo<'s> {
    raw: vks::VkSparseImageOpaqueMemoryBindInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SparseImageOpaqueMemoryBindInfo<'s> {
    pub fn builder<'b>() -> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        SparseImageOpaqueMemoryBindInfoBuilder::new()
    }

    pub fn image(&self) {
    }

    pub fn bind_count(&self) {
    }

    pub fn binds(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageOpaqueMemoryBindInfo {
        &self.raw
    }
}


/// A builder for `VkSparseImageOpaqueMemoryBindInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SparseImageOpaqueMemoryBindInfoBuilder<'b> {
    raw: vks::VkSparseImageOpaqueMemoryBindInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
    pub fn new() -> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        SparseImageOpaqueMemoryBindInfoBuilder {
            raw: vks::VkSparseImageOpaqueMemoryBindInfo::default(),
            _p: PhantomData,
        }
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        self
    }

    pub fn bind_count<'m>(&'m mut self, bind_count: u32) -> &'m mut SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        self
    }

    pub fn binds<'m, 'a>(&'m mut self, binds: &'a SparseMemoryBind) -> &'m mut SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        self
    }

}


/// A `VkSparseImageMemoryBindInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageMemoryBindInfo<'s> {
    raw: vks::VkSparseImageMemoryBindInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SparseImageMemoryBindInfo<'s> {
    pub fn builder<'b>() -> SparseImageMemoryBindInfoBuilder<'b> {
        SparseImageMemoryBindInfoBuilder::new()
    }

    pub fn image(&self) {
    }

    pub fn bind_count(&self) {
    }

    pub fn binds(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageMemoryBindInfo {
        &self.raw
    }
}


/// A builder for `VkSparseImageMemoryBindInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SparseImageMemoryBindInfoBuilder<'b> {
    raw: vks::VkSparseImageMemoryBindInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SparseImageMemoryBindInfoBuilder<'b> {
    pub fn new() -> SparseImageMemoryBindInfoBuilder<'b> {
        SparseImageMemoryBindInfoBuilder {
            raw: vks::VkSparseImageMemoryBindInfo::default(),
            _p: PhantomData,
        }
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut SparseImageMemoryBindInfoBuilder<'b> {
        self
    }

    pub fn bind_count<'m>(&'m mut self, bind_count: u32) -> &'m mut SparseImageMemoryBindInfoBuilder<'b> {
        self
    }

    pub fn binds<'m, 'a>(&'m mut self, binds: &'a SparseImageMemoryBind) -> &'m mut SparseImageMemoryBindInfoBuilder<'b> {
        self
    }

}


/// A `VkBindSparseInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BindSparseInfo<'s> {
    raw: vks::VkBindSparseInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> BindSparseInfo<'s> {
    pub fn builder<'b>() -> BindSparseInfoBuilder<'b> {
        BindSparseInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn wait_semaphore_count(&self) {
    }

    pub fn wait_semaphores(&self) {
    }

    pub fn buffer_bind_count(&self) {
    }

    pub fn buffer_binds(&self) {
    }

    pub fn image_opaque_bind_count(&self) {
    }

    pub fn image_opaque_binds(&self) {
    }

    pub fn image_bind_count(&self) {
    }

    pub fn image_binds(&self) {
    }

    pub fn signal_semaphore_count(&self) {
    }

    pub fn signal_semaphores(&self) {
    }

    pub fn raw(&self) -> &vks::VkBindSparseInfo {
        &self.raw
    }
}


/// A builder for `VkBindSparseInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BindSparseInfoBuilder<'b> {
    raw: vks::VkBindSparseInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> BindSparseInfoBuilder<'b> {
    pub fn new() -> BindSparseInfoBuilder<'b> {
        BindSparseInfoBuilder {
            raw: vks::VkBindSparseInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn wait_semaphore_count<'m>(&'m mut self, wait_semaphore_count: u32) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn wait_semaphores<'m, 'a>(&'m mut self, wait_semaphores: &'a Semaphore) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn buffer_bind_count<'m>(&'m mut self, buffer_bind_count: u32) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn buffer_binds<'m, 'a>(&'m mut self, buffer_binds: &'a SparseBufferMemoryBindInfo) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn image_opaque_bind_count<'m>(&'m mut self, image_opaque_bind_count: u32) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn image_opaque_binds<'m, 'a>(&'m mut self, image_opaque_binds: &'a SparseImageOpaqueMemoryBindInfo) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn image_bind_count<'m>(&'m mut self, image_bind_count: u32) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn image_binds<'m, 'a>(&'m mut self, image_binds: &'a SparseImageMemoryBindInfo) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn signal_semaphore_count<'m>(&'m mut self, signal_semaphore_count: u32) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

    pub fn signal_semaphores<'m, 'a>(&'m mut self, signal_semaphores: &'a Semaphore) -> &'m mut BindSparseInfoBuilder<'b> {
        self
    }

}


/// A `VkImageCopy`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageCopy {
    raw: vks::VkImageCopy,
}

impl ImageCopy {
    pub fn builder() -> ImageCopyBuilder {
        ImageCopyBuilder::new()
    }

    pub fn src_subresource(&self) {
    }

    pub fn src_offset(&self) {
    }

    pub fn dst_subresource(&self) {
    }

    pub fn dst_offset(&self) {
    }

    pub fn extent(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageCopy {
        &self.raw
    }
}


/// A builder for `VkImageCopy`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageCopyBuilder {
    raw: vks::VkImageCopy,
}

impl ImageCopyBuilder {
    pub fn new() -> ImageCopyBuilder {
        ImageCopyBuilder {
            raw: vks::VkImageCopy::default(),
        }
    }

    pub fn src_subresource<'m>(&'m mut self, src_subresource: ImageSubresourceLayers) -> &'m mut ImageCopyBuilder {
        self
    }

    pub fn src_offset<'m>(&'m mut self, src_offset: Offset3d) -> &'m mut ImageCopyBuilder {
        self
    }

    pub fn dst_subresource<'m>(&'m mut self, dst_subresource: ImageSubresourceLayers) -> &'m mut ImageCopyBuilder {
        self
    }

    pub fn dst_offset<'m>(&'m mut self, dst_offset: Offset3d) -> &'m mut ImageCopyBuilder {
        self
    }

    pub fn extent<'m>(&'m mut self, extent: Extent3d) -> &'m mut ImageCopyBuilder {
        self
    }

}


/// A `VkImageBlit`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageBlit {
    raw: vks::VkImageBlit,
}

impl ImageBlit {
    pub fn builder() -> ImageBlitBuilder {
        ImageBlitBuilder::new()
    }

    pub fn src_subresource(&self) {
    }

    pub fn src_offsets(&self) {
    }

    pub fn dst_subresource(&self) {
    }

    pub fn dst_offsets(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageBlit {
        &self.raw
    }
}


/// A builder for `VkImageBlit`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageBlitBuilder {
    raw: vks::VkImageBlit,
}

impl ImageBlitBuilder {
    pub fn new() -> ImageBlitBuilder {
        ImageBlitBuilder {
            raw: vks::VkImageBlit::default(),
        }
    }

    pub fn src_subresource<'m>(&'m mut self, src_subresource: ImageSubresourceLayers) -> &'m mut ImageBlitBuilder {
        self
    }

    pub fn src_offsets<'m>(&'m mut self, src_offsets: Offset3d) -> &'m mut ImageBlitBuilder {
        self
    }

    pub fn dst_subresource<'m>(&'m mut self, dst_subresource: ImageSubresourceLayers) -> &'m mut ImageBlitBuilder {
        self
    }

    pub fn dst_offsets<'m>(&'m mut self, dst_offsets: Offset3d) -> &'m mut ImageBlitBuilder {
        self
    }

}


/// A `VkBufferImageCopy`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BufferImageCopy {
    raw: vks::VkBufferImageCopy,
}

impl BufferImageCopy {
    pub fn builder() -> BufferImageCopyBuilder {
        BufferImageCopyBuilder::new()
    }

    pub fn buffer_offset(&self) {
    }

    pub fn buffer_row_length(&self) {
    }

    pub fn buffer_image_height(&self) {
    }

    pub fn image_subresource(&self) {
    }

    pub fn image_offset(&self) {
    }

    pub fn image_extent(&self) {
    }

    pub fn raw(&self) -> &vks::VkBufferImageCopy {
        &self.raw
    }
}


/// A builder for `VkBufferImageCopy`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BufferImageCopyBuilder {
    raw: vks::VkBufferImageCopy,
}

impl BufferImageCopyBuilder {
    pub fn new() -> BufferImageCopyBuilder {
        BufferImageCopyBuilder {
            raw: vks::VkBufferImageCopy::default(),
        }
    }

    pub fn buffer_offset<'m>(&'m mut self, buffer_offset: u64) -> &'m mut BufferImageCopyBuilder {
        self
    }

    pub fn buffer_row_length<'m>(&'m mut self, buffer_row_length: u32) -> &'m mut BufferImageCopyBuilder {
        self
    }

    pub fn buffer_image_height<'m>(&'m mut self, buffer_image_height: u32) -> &'m mut BufferImageCopyBuilder {
        self
    }

    pub fn image_subresource<'m>(&'m mut self, image_subresource: ImageSubresourceLayers) -> &'m mut BufferImageCopyBuilder {
        self
    }

    pub fn image_offset<'m>(&'m mut self, image_offset: Offset3d) -> &'m mut BufferImageCopyBuilder {
        self
    }

    pub fn image_extent<'m>(&'m mut self, image_extent: Extent3d) -> &'m mut BufferImageCopyBuilder {
        self
    }

}


/// A `VkImageResolve`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageResolve {
    raw: vks::VkImageResolve,
}

impl ImageResolve {
    pub fn builder() -> ImageResolveBuilder {
        ImageResolveBuilder::new()
    }

    pub fn src_subresource(&self) {
    }

    pub fn src_offset(&self) {
    }

    pub fn dst_subresource(&self) {
    }

    pub fn dst_offset(&self) {
    }

    pub fn extent(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageResolve {
        &self.raw
    }
}


/// A builder for `VkImageResolve`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageResolveBuilder {
    raw: vks::VkImageResolve,
}

impl ImageResolveBuilder {
    pub fn new() -> ImageResolveBuilder {
        ImageResolveBuilder {
            raw: vks::VkImageResolve::default(),
        }
    }

    pub fn src_subresource<'m>(&'m mut self, src_subresource: ImageSubresourceLayers) -> &'m mut ImageResolveBuilder {
        self
    }

    pub fn src_offset<'m>(&'m mut self, src_offset: Offset3d) -> &'m mut ImageResolveBuilder {
        self
    }

    pub fn dst_subresource<'m>(&'m mut self, dst_subresource: ImageSubresourceLayers) -> &'m mut ImageResolveBuilder {
        self
    }

    pub fn dst_offset<'m>(&'m mut self, dst_offset: Offset3d) -> &'m mut ImageResolveBuilder {
        self
    }

    pub fn extent<'m>(&'m mut self, extent: Extent3d) -> &'m mut ImageResolveBuilder {
        self
    }

}


/// A `VkShaderModuleCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ShaderModuleCreateInfo<'s> {
    raw: vks::VkShaderModuleCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> ShaderModuleCreateInfo<'s> {
    pub fn builder<'b>() -> ShaderModuleCreateInfoBuilder<'b> {
        ShaderModuleCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn code_size(&self) {
    }

    pub fn code(&self) {
    }

    pub fn raw(&self) -> &vks::VkShaderModuleCreateInfo {
        &self.raw
    }
}


/// A builder for `VkShaderModuleCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ShaderModuleCreateInfoBuilder<'b> {
    raw: vks::VkShaderModuleCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> ShaderModuleCreateInfoBuilder<'b> {
    pub fn new() -> ShaderModuleCreateInfoBuilder<'b> {
        ShaderModuleCreateInfoBuilder {
            raw: vks::VkShaderModuleCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ShaderModuleCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ShaderModuleCreateFlags) -> &'m mut ShaderModuleCreateInfoBuilder<'b> {
        self
    }

    pub fn code_size<'m>(&'m mut self, code_size: usize) -> &'m mut ShaderModuleCreateInfoBuilder<'b> {
        self
    }

    pub fn code<'m, 'a>(&'m mut self, code: &'a u32) -> &'m mut ShaderModuleCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkDescriptorSetLayoutBinding`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorSetLayoutBinding<'s> {
    raw: vks::VkDescriptorSetLayoutBinding,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorSetLayoutBinding<'s> {
    pub fn builder<'b>() -> DescriptorSetLayoutBindingBuilder<'b> {
        DescriptorSetLayoutBindingBuilder::new()
    }

    pub fn binding(&self) {
    }

    pub fn descriptor_type(&self) {
    }

    pub fn descriptor_count(&self) {
    }

    pub fn stage_flags(&self) {
    }

    pub fn immutable_samplers(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorSetLayoutBinding {
        &self.raw
    }
}


/// A builder for `VkDescriptorSetLayoutBinding`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutBindingBuilder<'b> {
    raw: vks::VkDescriptorSetLayoutBinding,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetLayoutBindingBuilder<'b> {
    pub fn new() -> DescriptorSetLayoutBindingBuilder<'b> {
        DescriptorSetLayoutBindingBuilder {
            raw: vks::VkDescriptorSetLayoutBinding::default(),
            _p: PhantomData,
        }
    }

    pub fn binding<'m>(&'m mut self, binding: u32) -> &'m mut DescriptorSetLayoutBindingBuilder<'b> {
        self
    }

    pub fn descriptor_type<'m>(&'m mut self, descriptor_type: DescriptorType) -> &'m mut DescriptorSetLayoutBindingBuilder<'b> {
        self
    }

    pub fn descriptor_count<'m>(&'m mut self, descriptor_count: u32) -> &'m mut DescriptorSetLayoutBindingBuilder<'b> {
        self
    }

    pub fn stage_flags<'m>(&'m mut self, stage_flags: ShaderStageFlags) -> &'m mut DescriptorSetLayoutBindingBuilder<'b> {
        self
    }

    pub fn immutable_samplers<'m, 'a>(&'m mut self, immutable_samplers: &'a Sampler) -> &'m mut DescriptorSetLayoutBindingBuilder<'b> {
        self
    }

}


/// A `VkDescriptorSetLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo<'s> {
    raw: vks::VkDescriptorSetLayoutCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorSetLayoutCreateInfo<'s> {
    pub fn builder<'b>() -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        DescriptorSetLayoutCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn binding_count(&self) {
    }

    pub fn bindings(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorSetLayoutCreateInfo {
        &self.raw
    }
}


/// A builder for `VkDescriptorSetLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'b> {
    raw: vks::VkDescriptorSetLayoutCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetLayoutCreateInfoBuilder<'b> {
    pub fn new() -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        DescriptorSetLayoutCreateInfoBuilder {
            raw: vks::VkDescriptorSetLayoutCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DescriptorSetLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DescriptorSetLayoutCreateFlags) -> &'m mut DescriptorSetLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn binding_count<'m>(&'m mut self, binding_count: u32) -> &'m mut DescriptorSetLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn bindings<'m, 'a>(&'m mut self, bindings: &'a DescriptorSetLayoutBinding) -> &'m mut DescriptorSetLayoutCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkDescriptorPoolSize`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorPoolSize {
    raw: vks::VkDescriptorPoolSize,
}

impl DescriptorPoolSize {
    pub fn builder() -> DescriptorPoolSizeBuilder {
        DescriptorPoolSizeBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn descriptor_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorPoolSize {
        &self.raw
    }
}


/// A builder for `VkDescriptorPoolSize`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorPoolSizeBuilder {
    raw: vks::VkDescriptorPoolSize,
}

impl DescriptorPoolSizeBuilder {
    pub fn new() -> DescriptorPoolSizeBuilder {
        DescriptorPoolSizeBuilder {
            raw: vks::VkDescriptorPoolSize::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: DescriptorType) -> &'m mut DescriptorPoolSizeBuilder {
        self
    }

    pub fn descriptor_count<'m>(&'m mut self, descriptor_count: u32) -> &'m mut DescriptorPoolSizeBuilder {
        self
    }

}


/// A `VkDescriptorPoolCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorPoolCreateInfo<'s> {
    raw: vks::VkDescriptorPoolCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorPoolCreateInfo<'s> {
    pub fn builder<'b>() -> DescriptorPoolCreateInfoBuilder<'b> {
        DescriptorPoolCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn max_sets(&self) {
    }

    pub fn pool_size_count(&self) {
    }

    pub fn pool_sizes(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorPoolCreateInfo {
        &self.raw
    }
}


/// A builder for `VkDescriptorPoolCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorPoolCreateInfoBuilder<'b> {
    raw: vks::VkDescriptorPoolCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorPoolCreateInfoBuilder<'b> {
    pub fn new() -> DescriptorPoolCreateInfoBuilder<'b> {
        DescriptorPoolCreateInfoBuilder {
            raw: vks::VkDescriptorPoolCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DescriptorPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DescriptorPoolCreateFlags) -> &'m mut DescriptorPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn max_sets<'m>(&'m mut self, max_sets: u32) -> &'m mut DescriptorPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn pool_size_count<'m>(&'m mut self, pool_size_count: u32) -> &'m mut DescriptorPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn pool_sizes<'m, 'a>(&'m mut self, pool_sizes: &'a DescriptorPoolSize) -> &'m mut DescriptorPoolCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkDescriptorSetAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorSetAllocateInfo<'s> {
    raw: vks::VkDescriptorSetAllocateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorSetAllocateInfo<'s> {
    pub fn builder<'b>() -> DescriptorSetAllocateInfoBuilder<'b> {
        DescriptorSetAllocateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn descriptor_pool(&self) {
    }

    pub fn descriptor_set_count(&self) {
    }

    pub fn set_layouts(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorSetAllocateInfo {
        &self.raw
    }
}


/// A builder for `VkDescriptorSetAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetAllocateInfoBuilder<'b> {
    raw: vks::VkDescriptorSetAllocateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetAllocateInfoBuilder<'b> {
    pub fn new() -> DescriptorSetAllocateInfoBuilder<'b> {
        DescriptorSetAllocateInfoBuilder {
            raw: vks::VkDescriptorSetAllocateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DescriptorSetAllocateInfoBuilder<'b> {
        self
    }

    pub fn descriptor_pool<'m>(&'m mut self, descriptor_pool: DescriptorPool) -> &'m mut DescriptorSetAllocateInfoBuilder<'b> {
        self
    }

    pub fn descriptor_set_count<'m>(&'m mut self, descriptor_set_count: u32) -> &'m mut DescriptorSetAllocateInfoBuilder<'b> {
        self
    }

    pub fn set_layouts<'m, 'a>(&'m mut self, set_layouts: &'a DescriptorSetLayout) -> &'m mut DescriptorSetAllocateInfoBuilder<'b> {
        self
    }

}


/// A `VkSpecializationMapEntry`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SpecializationMapEntry {
    raw: vks::VkSpecializationMapEntry,
}

impl SpecializationMapEntry {
    pub fn builder() -> SpecializationMapEntryBuilder {
        SpecializationMapEntryBuilder::new()
    }

    pub fn constant_id(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn raw(&self) -> &vks::VkSpecializationMapEntry {
        &self.raw
    }
}


/// A builder for `VkSpecializationMapEntry`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SpecializationMapEntryBuilder {
    raw: vks::VkSpecializationMapEntry,
}

impl SpecializationMapEntryBuilder {
    pub fn new() -> SpecializationMapEntryBuilder {
        SpecializationMapEntryBuilder {
            raw: vks::VkSpecializationMapEntry::default(),
        }
    }

    pub fn constant_id<'m>(&'m mut self, constant_id: u32) -> &'m mut SpecializationMapEntryBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u32) -> &'m mut SpecializationMapEntryBuilder {
        self
    }

    pub fn size<'m>(&'m mut self, size: usize) -> &'m mut SpecializationMapEntryBuilder {
        self
    }

}


/// A `VkSpecializationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SpecializationInfo<'s> {
    raw: vks::VkSpecializationInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SpecializationInfo<'s> {
    pub fn builder<'b>() -> SpecializationInfoBuilder<'b> {
        SpecializationInfoBuilder::new()
    }

    pub fn map_entry_count(&self) {
    }

    pub fn map_entries(&self) {
    }

    pub fn data_size(&self) {
    }

    pub fn data(&self) {
    }

    pub fn raw(&self) -> &vks::VkSpecializationInfo {
        &self.raw
    }
}


/// A builder for `VkSpecializationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SpecializationInfoBuilder<'b> {
    raw: vks::VkSpecializationInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SpecializationInfoBuilder<'b> {
    pub fn new() -> SpecializationInfoBuilder<'b> {
        SpecializationInfoBuilder {
            raw: vks::VkSpecializationInfo::default(),
            _p: PhantomData,
        }
    }

    pub fn map_entry_count<'m>(&'m mut self, map_entry_count: u32) -> &'m mut SpecializationInfoBuilder<'b> {
        self
    }

    pub fn map_entries<'m, 'a>(&'m mut self, map_entries: &'a SpecializationMapEntry) -> &'m mut SpecializationInfoBuilder<'b> {
        self
    }

    pub fn data_size<'m>(&'m mut self, data_size: usize) -> &'m mut SpecializationInfoBuilder<'b> {
        self
    }

    pub fn data<'m, 'a>(&'m mut self, data: &'a ()) -> &'m mut SpecializationInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineShaderStageCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo<'s> {
    raw: vks::VkPipelineShaderStageCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineShaderStageCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineShaderStageCreateInfoBuilder<'b> {
        PipelineShaderStageCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn stage(&self) {
    }

    pub fn module(&self) {
    }

    pub fn name(&self) {
    }

    pub fn specialization_info(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineShaderStageCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineShaderStageCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineShaderStageCreateInfoBuilder<'b> {
    raw: vks::VkPipelineShaderStageCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineShaderStageCreateInfoBuilder<'b> {
    pub fn new() -> PipelineShaderStageCreateInfoBuilder<'b> {
        PipelineShaderStageCreateInfoBuilder {
            raw: vks::VkPipelineShaderStageCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineShaderStageCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineShaderStageCreateFlags) -> &'m mut PipelineShaderStageCreateInfoBuilder<'b> {
        self
    }

    pub fn stage<'m>(&'m mut self, stage: ShaderStageFlags) -> &'m mut PipelineShaderStageCreateInfoBuilder<'b> {
        self
    }

    pub fn module<'m>(&'m mut self, module: ShaderModule) -> &'m mut PipelineShaderStageCreateInfoBuilder<'b> {
        self
    }

    pub fn name<'m, 'a>(&'m mut self, name: &'a i8) -> &'m mut PipelineShaderStageCreateInfoBuilder<'b> {
        self
    }

    pub fn specialization_info<'m, 'a>(&'m mut self, specialization_info: &'a SpecializationInfo) -> &'m mut PipelineShaderStageCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkComputePipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ComputePipelineCreateInfo<'s> {
    raw: vks::VkComputePipelineCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> ComputePipelineCreateInfo<'s> {
    pub fn builder<'b>() -> ComputePipelineCreateInfoBuilder<'b> {
        ComputePipelineCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn stage(&self) {
    }

    pub fn layout(&self) {
    }

    pub fn base_pipeline_handle(&self) {
    }

    pub fn base_pipeline_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkComputePipelineCreateInfo {
        &self.raw
    }
}


/// A builder for `VkComputePipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ComputePipelineCreateInfoBuilder<'b> {
    raw: vks::VkComputePipelineCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> ComputePipelineCreateInfoBuilder<'b> {
    pub fn new() -> ComputePipelineCreateInfoBuilder<'b> {
        ComputePipelineCreateInfoBuilder {
            raw: vks::VkComputePipelineCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ComputePipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineCreateFlags) -> &'m mut ComputePipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn stage<'m>(&'m mut self, stage: PipelineShaderStageCreateInfo) -> &'m mut ComputePipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn layout<'m>(&'m mut self, layout: PipelineLayout) -> &'m mut ComputePipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn base_pipeline_handle<'m>(&'m mut self, base_pipeline_handle: Pipeline) -> &'m mut ComputePipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn base_pipeline_index<'m>(&'m mut self, base_pipeline_index: i32) -> &'m mut ComputePipelineCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkVertexInputBindingDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct VertexInputBindingDescription {
    raw: vks::VkVertexInputBindingDescription,
}

impl VertexInputBindingDescription {
    pub fn builder() -> VertexInputBindingDescriptionBuilder {
        VertexInputBindingDescriptionBuilder::new()
    }

    pub fn binding(&self) {
    }

    pub fn stride(&self) {
    }

    pub fn input_rate(&self) {
    }

    pub fn raw(&self) -> &vks::VkVertexInputBindingDescription {
        &self.raw
    }
}


/// A builder for `VkVertexInputBindingDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct VertexInputBindingDescriptionBuilder {
    raw: vks::VkVertexInputBindingDescription,
}

impl VertexInputBindingDescriptionBuilder {
    pub fn new() -> VertexInputBindingDescriptionBuilder {
        VertexInputBindingDescriptionBuilder {
            raw: vks::VkVertexInputBindingDescription::default(),
        }
    }

    pub fn binding<'m>(&'m mut self, binding: u32) -> &'m mut VertexInputBindingDescriptionBuilder {
        self
    }

    pub fn stride<'m>(&'m mut self, stride: u32) -> &'m mut VertexInputBindingDescriptionBuilder {
        self
    }

    pub fn input_rate<'m>(&'m mut self, input_rate: VertexInputRate) -> &'m mut VertexInputBindingDescriptionBuilder {
        self
    }

}


/// A `VkVertexInputAttributeDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct VertexInputAttributeDescription {
    raw: vks::VkVertexInputAttributeDescription,
}

impl VertexInputAttributeDescription {
    pub fn builder() -> VertexInputAttributeDescriptionBuilder {
        VertexInputAttributeDescriptionBuilder::new()
    }

    pub fn location(&self) {
    }

    pub fn binding(&self) {
    }

    pub fn format(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn raw(&self) -> &vks::VkVertexInputAttributeDescription {
        &self.raw
    }
}


/// A builder for `VkVertexInputAttributeDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct VertexInputAttributeDescriptionBuilder {
    raw: vks::VkVertexInputAttributeDescription,
}

impl VertexInputAttributeDescriptionBuilder {
    pub fn new() -> VertexInputAttributeDescriptionBuilder {
        VertexInputAttributeDescriptionBuilder {
            raw: vks::VkVertexInputAttributeDescription::default(),
        }
    }

    pub fn location<'m>(&'m mut self, location: u32) -> &'m mut VertexInputAttributeDescriptionBuilder {
        self
    }

    pub fn binding<'m>(&'m mut self, binding: u32) -> &'m mut VertexInputAttributeDescriptionBuilder {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut VertexInputAttributeDescriptionBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u32) -> &'m mut VertexInputAttributeDescriptionBuilder {
        self
    }

}


/// A `VkPipelineVertexInputStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo<'s> {
    raw: vks::VkPipelineVertexInputStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineVertexInputStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        PipelineVertexInputStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn vertex_binding_description_count(&self) {
    }

    pub fn vertex_binding_descriptions(&self) {
    }

    pub fn vertex_attribute_description_count(&self) {
    }

    pub fn vertex_attribute_descriptions(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineVertexInputStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineVertexInputStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineVertexInputStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineVertexInputStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineVertexInputStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        PipelineVertexInputStateCreateInfoBuilder {
            raw: vks::VkPipelineVertexInputStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineVertexInputStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineVertexInputStateCreateFlags) -> &'m mut PipelineVertexInputStateCreateInfoBuilder<'b> {
        self
    }

    pub fn vertex_binding_description_count<'m>(&'m mut self, vertex_binding_description_count: u32) -> &'m mut PipelineVertexInputStateCreateInfoBuilder<'b> {
        self
    }

    pub fn vertex_binding_descriptions<'m, 'a>(&'m mut self, vertex_binding_descriptions: &'a VertexInputBindingDescription) -> &'m mut PipelineVertexInputStateCreateInfoBuilder<'b> {
        self
    }

    pub fn vertex_attribute_description_count<'m>(&'m mut self, vertex_attribute_description_count: u32) -> &'m mut PipelineVertexInputStateCreateInfoBuilder<'b> {
        self
    }

    pub fn vertex_attribute_descriptions<'m, 'a>(&'m mut self, vertex_attribute_descriptions: &'a VertexInputAttributeDescription) -> &'m mut PipelineVertexInputStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineInputAssemblyStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo<'s> {
    raw: vks::VkPipelineInputAssemblyStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineInputAssemblyStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        PipelineInputAssemblyStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn topology(&self) {
    }

    pub fn primitive_restart_enable(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineInputAssemblyStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineInputAssemblyStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineInputAssemblyStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineInputAssemblyStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        PipelineInputAssemblyStateCreateInfoBuilder {
            raw: vks::VkPipelineInputAssemblyStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineInputAssemblyStateCreateFlags) -> &'m mut PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self
    }

    pub fn topology<'m>(&'m mut self, topology: PrimitiveTopology) -> &'m mut PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self
    }

    pub fn primitive_restart_enable<'m>(&'m mut self, primitive_restart_enable: bool) -> &'m mut PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineTessellationStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineTessellationStateCreateInfo<'s> {
    raw: vks::VkPipelineTessellationStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineTessellationStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineTessellationStateCreateInfoBuilder<'b> {
        PipelineTessellationStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn patch_control_points(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineTessellationStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineTessellationStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineTessellationStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineTessellationStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineTessellationStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineTessellationStateCreateInfoBuilder<'b> {
        PipelineTessellationStateCreateInfoBuilder {
            raw: vks::VkPipelineTessellationStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineTessellationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineTessellationStateCreateFlags) -> &'m mut PipelineTessellationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn patch_control_points<'m>(&'m mut self, patch_control_points: u32) -> &'m mut PipelineTessellationStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineViewportStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo<'s> {
    raw: vks::VkPipelineViewportStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineViewportStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineViewportStateCreateInfoBuilder<'b> {
        PipelineViewportStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn viewport_count(&self) {
    }

    pub fn viewports(&self) {
    }

    pub fn scissor_count(&self) {
    }

    pub fn scissors(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineViewportStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineViewportStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineViewportStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineViewportStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineViewportStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineViewportStateCreateInfoBuilder<'b> {
        PipelineViewportStateCreateInfoBuilder {
            raw: vks::VkPipelineViewportStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineViewportStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineViewportStateCreateFlags) -> &'m mut PipelineViewportStateCreateInfoBuilder<'b> {
        self
    }

    pub fn viewport_count<'m>(&'m mut self, viewport_count: u32) -> &'m mut PipelineViewportStateCreateInfoBuilder<'b> {
        self
    }

    pub fn viewports<'m, 'a>(&'m mut self, viewports: &'a Viewport) -> &'m mut PipelineViewportStateCreateInfoBuilder<'b> {
        self
    }

    pub fn scissor_count<'m>(&'m mut self, scissor_count: u32) -> &'m mut PipelineViewportStateCreateInfoBuilder<'b> {
        self
    }

    pub fn scissors<'m, 'a>(&'m mut self, scissors: &'a Rect2d) -> &'m mut PipelineViewportStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineRasterizationStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo<'s> {
    raw: vks::VkPipelineRasterizationStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineRasterizationStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        PipelineRasterizationStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn depth_clamp_enable(&self) {
    }

    pub fn rasterizer_discard_enable(&self) {
    }

    pub fn polygon_mode(&self) {
    }

    pub fn cull_mode(&self) {
    }

    pub fn front_face(&self) {
    }

    pub fn depth_bias_enable(&self) {
    }

    pub fn depth_bias_constant_factor(&self) {
    }

    pub fn depth_bias_clamp(&self) {
    }

    pub fn depth_bias_slope_factor(&self) {
    }

    pub fn line_width(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineRasterizationStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineRasterizationStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineRasterizationStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineRasterizationStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineRasterizationStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        PipelineRasterizationStateCreateInfoBuilder {
            raw: vks::VkPipelineRasterizationStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineRasterizationStateCreateFlags) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_clamp_enable<'m>(&'m mut self, depth_clamp_enable: bool) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn rasterizer_discard_enable<'m>(&'m mut self, rasterizer_discard_enable: bool) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn polygon_mode<'m>(&'m mut self, polygon_mode: PolygonMode) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn cull_mode<'m>(&'m mut self, cull_mode: CullModeFlags) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn front_face<'m>(&'m mut self, front_face: FrontFace) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_bias_enable<'m>(&'m mut self, depth_bias_enable: bool) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_bias_constant_factor<'m>(&'m mut self, depth_bias_constant_factor: f32) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_bias_clamp<'m>(&'m mut self, depth_bias_clamp: f32) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_bias_slope_factor<'m>(&'m mut self, depth_bias_slope_factor: f32) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

    pub fn line_width<'m>(&'m mut self, line_width: f32) -> &'m mut PipelineRasterizationStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineMultisampleStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo<'s> {
    raw: vks::VkPipelineMultisampleStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineMultisampleStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        PipelineMultisampleStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn rasterization_samples(&self) {
    }

    pub fn sample_shading_enable(&self) {
    }

    pub fn min_sample_shading(&self) {
    }

    pub fn sample_mask(&self) {
    }

    pub fn alpha_to_coverage_enable(&self) {
    }

    pub fn alpha_to_one_enable(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineMultisampleStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineMultisampleStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineMultisampleStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineMultisampleStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineMultisampleStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        PipelineMultisampleStateCreateInfoBuilder {
            raw: vks::VkPipelineMultisampleStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineMultisampleStateCreateFlags) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn rasterization_samples<'m>(&'m mut self, rasterization_samples: SampleCountFlags) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn sample_shading_enable<'m>(&'m mut self, sample_shading_enable: bool) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn min_sample_shading<'m>(&'m mut self, min_sample_shading: f32) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn sample_mask<'m, 'a>(&'m mut self, sample_mask: &'a u32) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn alpha_to_coverage_enable<'m>(&'m mut self, alpha_to_coverage_enable: bool) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

    pub fn alpha_to_one_enable<'m>(&'m mut self, alpha_to_one_enable: bool) -> &'m mut PipelineMultisampleStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineColorBlendAttachmentState`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    raw: vks::VkPipelineColorBlendAttachmentState,
}

impl PipelineColorBlendAttachmentState {
    pub fn builder() -> PipelineColorBlendAttachmentStateBuilder {
        PipelineColorBlendAttachmentStateBuilder::new()
    }

    pub fn blend_enable(&self) {
    }

    pub fn src_color_blend_factor(&self) {
    }

    pub fn dst_color_blend_factor(&self) {
    }

    pub fn color_blend_op(&self) {
    }

    pub fn src_alpha_blend_factor(&self) {
    }

    pub fn dst_alpha_blend_factor(&self) {
    }

    pub fn alpha_blend_op(&self) {
    }

    pub fn color_write_mask(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineColorBlendAttachmentState {
        &self.raw
    }
}


/// A builder for `VkPipelineColorBlendAttachmentState`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineColorBlendAttachmentStateBuilder {
    raw: vks::VkPipelineColorBlendAttachmentState,
}

impl PipelineColorBlendAttachmentStateBuilder {
    pub fn new() -> PipelineColorBlendAttachmentStateBuilder {
        PipelineColorBlendAttachmentStateBuilder {
            raw: vks::VkPipelineColorBlendAttachmentState::default(),
        }
    }

    pub fn blend_enable<'m>(&'m mut self, blend_enable: bool) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn src_color_blend_factor<'m>(&'m mut self, src_color_blend_factor: BlendFactor) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn dst_color_blend_factor<'m>(&'m mut self, dst_color_blend_factor: BlendFactor) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn color_blend_op<'m>(&'m mut self, color_blend_op: BlendOp) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn src_alpha_blend_factor<'m>(&'m mut self, src_alpha_blend_factor: BlendFactor) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn dst_alpha_blend_factor<'m>(&'m mut self, dst_alpha_blend_factor: BlendFactor) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn alpha_blend_op<'m>(&'m mut self, alpha_blend_op: BlendOp) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

    pub fn color_write_mask<'m>(&'m mut self, color_write_mask: ColorComponentFlags) -> &'m mut PipelineColorBlendAttachmentStateBuilder {
        self
    }

}


/// A `VkPipelineColorBlendStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo<'s> {
    raw: vks::VkPipelineColorBlendStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineColorBlendStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        PipelineColorBlendStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn logic_op_enable(&self) {
    }

    pub fn logic_op(&self) {
    }

    pub fn attachment_count(&self) {
    }

    pub fn attachments(&self) {
    }

    pub fn blend_constants(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineColorBlendStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineColorBlendStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineColorBlendStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineColorBlendStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineColorBlendStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        PipelineColorBlendStateCreateInfoBuilder {
            raw: vks::VkPipelineColorBlendStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineColorBlendStateCreateFlags) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

    pub fn logic_op_enable<'m>(&'m mut self, logic_op_enable: bool) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

    pub fn logic_op<'m>(&'m mut self, logic_op: LogicOp) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

    pub fn attachment_count<'m>(&'m mut self, attachment_count: u32) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

    pub fn attachments<'m, 'a>(&'m mut self, attachments: &'a PipelineColorBlendAttachmentState) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

    pub fn blend_constants<'m>(&'m mut self, blend_constants: f32) -> &'m mut PipelineColorBlendStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineDynamicStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo<'s> {
    raw: vks::VkPipelineDynamicStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineDynamicStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineDynamicStateCreateInfoBuilder<'b> {
        PipelineDynamicStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn dynamic_state_count(&self) {
    }

    pub fn dynamic_states(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineDynamicStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineDynamicStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineDynamicStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineDynamicStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineDynamicStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineDynamicStateCreateInfoBuilder<'b> {
        PipelineDynamicStateCreateInfoBuilder {
            raw: vks::VkPipelineDynamicStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineDynamicStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineDynamicStateCreateFlags) -> &'m mut PipelineDynamicStateCreateInfoBuilder<'b> {
        self
    }

    pub fn dynamic_state_count<'m>(&'m mut self, dynamic_state_count: u32) -> &'m mut PipelineDynamicStateCreateInfoBuilder<'b> {
        self
    }

    pub fn dynamic_states<'m, 'a>(&'m mut self, dynamic_states: &'a DynamicState) -> &'m mut PipelineDynamicStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkStencilOpState`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct StencilOpState {
    raw: vks::VkStencilOpState,
}

impl StencilOpState {
    pub fn builder() -> StencilOpStateBuilder {
        StencilOpStateBuilder::new()
    }

    pub fn fail_op(&self) {
    }

    pub fn pass_op(&self) {
    }

    pub fn depth_fail_op(&self) {
    }

    pub fn compare_op(&self) {
    }

    pub fn compare_mask(&self) {
    }

    pub fn write_mask(&self) {
    }

    pub fn reference(&self) {
    }

    pub fn raw(&self) -> &vks::VkStencilOpState {
        &self.raw
    }
}


/// A builder for `VkStencilOpState`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct StencilOpStateBuilder {
    raw: vks::VkStencilOpState,
}

impl StencilOpStateBuilder {
    pub fn new() -> StencilOpStateBuilder {
        StencilOpStateBuilder {
            raw: vks::VkStencilOpState::default(),
        }
    }

    pub fn fail_op<'m>(&'m mut self, fail_op: StencilOp) -> &'m mut StencilOpStateBuilder {
        self
    }

    pub fn pass_op<'m>(&'m mut self, pass_op: StencilOp) -> &'m mut StencilOpStateBuilder {
        self
    }

    pub fn depth_fail_op<'m>(&'m mut self, depth_fail_op: StencilOp) -> &'m mut StencilOpStateBuilder {
        self
    }

    pub fn compare_op<'m>(&'m mut self, compare_op: CompareOp) -> &'m mut StencilOpStateBuilder {
        self
    }

    pub fn compare_mask<'m>(&'m mut self, compare_mask: u32) -> &'m mut StencilOpStateBuilder {
        self
    }

    pub fn write_mask<'m>(&'m mut self, write_mask: u32) -> &'m mut StencilOpStateBuilder {
        self
    }

    pub fn reference<'m>(&'m mut self, reference: u32) -> &'m mut StencilOpStateBuilder {
        self
    }

}


/// A `VkPipelineDepthStencilStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo<'s> {
    raw: vks::VkPipelineDepthStencilStateCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineDepthStencilStateCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        PipelineDepthStencilStateCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn depth_test_enable(&self) {
    }

    pub fn depth_write_enable(&self) {
    }

    pub fn depth_compare_op(&self) {
    }

    pub fn depth_bounds_test_enable(&self) {
    }

    pub fn stencil_test_enable(&self) {
    }

    pub fn front(&self) {
    }

    pub fn back(&self) {
    }

    pub fn min_depth_bounds(&self) {
    }

    pub fn max_depth_bounds(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineDepthStencilStateCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineDepthStencilStateCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineDepthStencilStateCreateInfoBuilder<'b> {
    raw: vks::VkPipelineDepthStencilStateCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineDepthStencilStateCreateInfoBuilder<'b> {
    pub fn new() -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        PipelineDepthStencilStateCreateInfoBuilder {
            raw: vks::VkPipelineDepthStencilStateCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineDepthStencilStateCreateFlags) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_test_enable<'m>(&'m mut self, depth_test_enable: bool) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_write_enable<'m>(&'m mut self, depth_write_enable: bool) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_compare_op<'m>(&'m mut self, depth_compare_op: CompareOp) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_bounds_test_enable<'m>(&'m mut self, depth_bounds_test_enable: bool) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn stencil_test_enable<'m>(&'m mut self, stencil_test_enable: bool) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn front<'m>(&'m mut self, front: StencilOpState) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn back<'m>(&'m mut self, back: StencilOpState) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn min_depth_bounds<'m>(&'m mut self, min_depth_bounds: f32) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

    pub fn max_depth_bounds<'m>(&'m mut self, max_depth_bounds: f32) -> &'m mut PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkGraphicsPipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo<'s> {
    raw: vks::VkGraphicsPipelineCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> GraphicsPipelineCreateInfo<'s> {
    pub fn builder<'b>() -> GraphicsPipelineCreateInfoBuilder<'b> {
        GraphicsPipelineCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn stage_count(&self) {
    }

    pub fn stages(&self) {
    }

    pub fn vertex_input_state(&self) {
    }

    pub fn input_assembly_state(&self) {
    }

    pub fn tessellation_state(&self) {
    }

    pub fn viewport_state(&self) {
    }

    pub fn rasterization_state(&self) {
    }

    pub fn multisample_state(&self) {
    }

    pub fn depth_stencil_state(&self) {
    }

    pub fn color_blend_state(&self) {
    }

    pub fn dynamic_state(&self) {
    }

    pub fn layout(&self) {
    }

    pub fn render_pass(&self) {
    }

    pub fn subpass(&self) {
    }

    pub fn base_pipeline_handle(&self) {
    }

    pub fn base_pipeline_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkGraphicsPipelineCreateInfo {
        &self.raw
    }
}


/// A builder for `VkGraphicsPipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'b> {
    raw: vks::VkGraphicsPipelineCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> GraphicsPipelineCreateInfoBuilder<'b> {
    pub fn new() -> GraphicsPipelineCreateInfoBuilder<'b> {
        GraphicsPipelineCreateInfoBuilder {
            raw: vks::VkGraphicsPipelineCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineCreateFlags) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn stage_count<'m>(&'m mut self, stage_count: u32) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn stages<'m, 'a>(&'m mut self, stages: &'a PipelineShaderStageCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn vertex_input_state<'m, 'a>(&'m mut self, vertex_input_state: &'a PipelineVertexInputStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn input_assembly_state<'m, 'a>(&'m mut self, input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn tessellation_state<'m, 'a>(&'m mut self, tessellation_state: &'a PipelineTessellationStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn viewport_state<'m, 'a>(&'m mut self, viewport_state: &'a PipelineViewportStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn rasterization_state<'m, 'a>(&'m mut self, rasterization_state: &'a PipelineRasterizationStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn multisample_state<'m, 'a>(&'m mut self, multisample_state: &'a PipelineMultisampleStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn depth_stencil_state<'m, 'a>(&'m mut self, depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn color_blend_state<'m, 'a>(&'m mut self, color_blend_state: &'a PipelineColorBlendStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn dynamic_state<'m, 'a>(&'m mut self, dynamic_state: &'a PipelineDynamicStateCreateInfo) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn layout<'m>(&'m mut self, layout: PipelineLayout) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn render_pass<'m>(&'m mut self, render_pass: RenderPass) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn subpass<'m>(&'m mut self, subpass: u32) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn base_pipeline_handle<'m>(&'m mut self, base_pipeline_handle: Pipeline) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

    pub fn base_pipeline_index<'m>(&'m mut self, base_pipeline_index: i32) -> &'m mut GraphicsPipelineCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPipelineCacheCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineCacheCreateInfo<'s> {
    raw: vks::VkPipelineCacheCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineCacheCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineCacheCreateInfoBuilder<'b> {
        PipelineCacheCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn initial_data_size(&self) {
    }

    pub fn initial_data(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineCacheCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineCacheCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineCacheCreateInfoBuilder<'b> {
    raw: vks::VkPipelineCacheCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineCacheCreateInfoBuilder<'b> {
    pub fn new() -> PipelineCacheCreateInfoBuilder<'b> {
        PipelineCacheCreateInfoBuilder {
            raw: vks::VkPipelineCacheCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineCacheCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineCacheCreateFlags) -> &'m mut PipelineCacheCreateInfoBuilder<'b> {
        self
    }

    pub fn initial_data_size<'m>(&'m mut self, initial_data_size: usize) -> &'m mut PipelineCacheCreateInfoBuilder<'b> {
        self
    }

    pub fn initial_data<'m, 'a>(&'m mut self, initial_data: &'a ()) -> &'m mut PipelineCacheCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPushConstantRange`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PushConstantRange {
    raw: vks::VkPushConstantRange,
}

impl PushConstantRange {
    pub fn builder() -> PushConstantRangeBuilder {
        PushConstantRangeBuilder::new()
    }

    pub fn stage_flags(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn size(&self) {
    }

    pub fn raw(&self) -> &vks::VkPushConstantRange {
        &self.raw
    }
}


/// A builder for `VkPushConstantRange`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PushConstantRangeBuilder {
    raw: vks::VkPushConstantRange,
}

impl PushConstantRangeBuilder {
    pub fn new() -> PushConstantRangeBuilder {
        PushConstantRangeBuilder {
            raw: vks::VkPushConstantRange::default(),
        }
    }

    pub fn stage_flags<'m>(&'m mut self, stage_flags: ShaderStageFlags) -> &'m mut PushConstantRangeBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u32) -> &'m mut PushConstantRangeBuilder {
        self
    }

    pub fn size<'m>(&'m mut self, size: u32) -> &'m mut PushConstantRangeBuilder {
        self
    }

}


/// A `VkPipelineLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo<'s> {
    raw: vks::VkPipelineLayoutCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineLayoutCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineLayoutCreateInfoBuilder<'b> {
        PipelineLayoutCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn set_layout_count(&self) {
    }

    pub fn set_layouts(&self) {
    }

    pub fn push_constant_range_count(&self) {
    }

    pub fn push_constant_ranges(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineLayoutCreateInfo {
        &self.raw
    }
}


/// A builder for `VkPipelineLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineLayoutCreateInfoBuilder<'b> {
    raw: vks::VkPipelineLayoutCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineLayoutCreateInfoBuilder<'b> {
    pub fn new() -> PipelineLayoutCreateInfoBuilder<'b> {
        PipelineLayoutCreateInfoBuilder {
            raw: vks::VkPipelineLayoutCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineLayoutCreateFlags) -> &'m mut PipelineLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn set_layout_count<'m>(&'m mut self, set_layout_count: u32) -> &'m mut PipelineLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn set_layouts<'m, 'a>(&'m mut self, set_layouts: &'a DescriptorSetLayout) -> &'m mut PipelineLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn push_constant_range_count<'m>(&'m mut self, push_constant_range_count: u32) -> &'m mut PipelineLayoutCreateInfoBuilder<'b> {
        self
    }

    pub fn push_constant_ranges<'m, 'a>(&'m mut self, push_constant_ranges: &'a PushConstantRange) -> &'m mut PipelineLayoutCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkSamplerCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SamplerCreateInfo<'s> {
    raw: vks::VkSamplerCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SamplerCreateInfo<'s> {
    pub fn builder<'b>() -> SamplerCreateInfoBuilder<'b> {
        SamplerCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn mag_filter(&self) {
    }

    pub fn min_filter(&self) {
    }

    pub fn mipmap_mode(&self) {
    }

    pub fn address_mode_u(&self) {
    }

    pub fn address_mode_v(&self) {
    }

    pub fn address_mode_w(&self) {
    }

    pub fn mip_lod_bias(&self) {
    }

    pub fn anisotropy_enable(&self) {
    }

    pub fn max_anisotropy(&self) {
    }

    pub fn compare_enable(&self) {
    }

    pub fn compare_op(&self) {
    }

    pub fn min_lod(&self) {
    }

    pub fn max_lod(&self) {
    }

    pub fn border_color(&self) {
    }

    pub fn unnormalized_coordinates(&self) {
    }

    pub fn raw(&self) -> &vks::VkSamplerCreateInfo {
        &self.raw
    }
}


/// A builder for `VkSamplerCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SamplerCreateInfoBuilder<'b> {
    raw: vks::VkSamplerCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SamplerCreateInfoBuilder<'b> {
    pub fn new() -> SamplerCreateInfoBuilder<'b> {
        SamplerCreateInfoBuilder {
            raw: vks::VkSamplerCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SamplerCreateFlags) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn mag_filter<'m>(&'m mut self, mag_filter: Filter) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn min_filter<'m>(&'m mut self, min_filter: Filter) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn mipmap_mode<'m>(&'m mut self, mipmap_mode: SamplerMipmapMode) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn address_mode_u<'m>(&'m mut self, address_mode_u: SamplerAddressMode) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn address_mode_v<'m>(&'m mut self, address_mode_v: SamplerAddressMode) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn address_mode_w<'m>(&'m mut self, address_mode_w: SamplerAddressMode) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn mip_lod_bias<'m>(&'m mut self, mip_lod_bias: f32) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn anisotropy_enable<'m>(&'m mut self, anisotropy_enable: bool) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn max_anisotropy<'m>(&'m mut self, max_anisotropy: f32) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn compare_enable<'m>(&'m mut self, compare_enable: bool) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn compare_op<'m>(&'m mut self, compare_op: CompareOp) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn min_lod<'m>(&'m mut self, min_lod: f32) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn max_lod<'m>(&'m mut self, max_lod: f32) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn border_color<'m>(&'m mut self, border_color: BorderColor) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

    pub fn unnormalized_coordinates<'m>(&'m mut self, unnormalized_coordinates: bool) -> &'m mut SamplerCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkCommandPoolCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CommandPoolCreateInfo<'s> {
    raw: vks::VkCommandPoolCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> CommandPoolCreateInfo<'s> {
    pub fn builder<'b>() -> CommandPoolCreateInfoBuilder<'b> {
        CommandPoolCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn queue_family_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkCommandPoolCreateInfo {
        &self.raw
    }
}


/// A builder for `VkCommandPoolCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct CommandPoolCreateInfoBuilder<'b> {
    raw: vks::VkCommandPoolCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> CommandPoolCreateInfoBuilder<'b> {
    pub fn new() -> CommandPoolCreateInfoBuilder<'b> {
        CommandPoolCreateInfoBuilder {
            raw: vks::VkCommandPoolCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CommandPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: CommandPoolCreateFlags) -> &'m mut CommandPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn queue_family_index<'m>(&'m mut self, queue_family_index: u32) -> &'m mut CommandPoolCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkCommandBufferAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CommandBufferAllocateInfo<'s> {
    raw: vks::VkCommandBufferAllocateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> CommandBufferAllocateInfo<'s> {
    pub fn builder<'b>() -> CommandBufferAllocateInfoBuilder<'b> {
        CommandBufferAllocateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn command_pool(&self) {
    }

    pub fn level(&self) {
    }

    pub fn command_buffer_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkCommandBufferAllocateInfo {
        &self.raw
    }
}


/// A builder for `VkCommandBufferAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct CommandBufferAllocateInfoBuilder<'b> {
    raw: vks::VkCommandBufferAllocateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> CommandBufferAllocateInfoBuilder<'b> {
    pub fn new() -> CommandBufferAllocateInfoBuilder<'b> {
        CommandBufferAllocateInfoBuilder {
            raw: vks::VkCommandBufferAllocateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CommandBufferAllocateInfoBuilder<'b> {
        self
    }

    pub fn command_pool<'m>(&'m mut self, command_pool: CommandPool) -> &'m mut CommandBufferAllocateInfoBuilder<'b> {
        self
    }

    pub fn level<'m>(&'m mut self, level: CommandBufferLevel) -> &'m mut CommandBufferAllocateInfoBuilder<'b> {
        self
    }

    pub fn command_buffer_count<'m>(&'m mut self, command_buffer_count: u32) -> &'m mut CommandBufferAllocateInfoBuilder<'b> {
        self
    }

}


/// A `VkCommandBufferInheritanceInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CommandBufferInheritanceInfo<'s> {
    raw: vks::VkCommandBufferInheritanceInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> CommandBufferInheritanceInfo<'s> {
    pub fn builder<'b>() -> CommandBufferInheritanceInfoBuilder<'b> {
        CommandBufferInheritanceInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn render_pass(&self) {
    }

    pub fn subpass(&self) {
    }

    pub fn framebuffer(&self) {
    }

    pub fn occlusion_query_enable(&self) {
    }

    pub fn query_flags(&self) {
    }

    pub fn pipeline_statistics(&self) {
    }

    pub fn raw(&self) -> &vks::VkCommandBufferInheritanceInfo {
        &self.raw
    }
}


/// A builder for `VkCommandBufferInheritanceInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct CommandBufferInheritanceInfoBuilder<'b> {
    raw: vks::VkCommandBufferInheritanceInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> CommandBufferInheritanceInfoBuilder<'b> {
    pub fn new() -> CommandBufferInheritanceInfoBuilder<'b> {
        CommandBufferInheritanceInfoBuilder {
            raw: vks::VkCommandBufferInheritanceInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

    pub fn render_pass<'m>(&'m mut self, render_pass: RenderPass) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

    pub fn subpass<'m>(&'m mut self, subpass: u32) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

    pub fn framebuffer<'m>(&'m mut self, framebuffer: Framebuffer) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

    pub fn occlusion_query_enable<'m>(&'m mut self, occlusion_query_enable: bool) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

    pub fn query_flags<'m>(&'m mut self, query_flags: QueryControlFlags) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

    pub fn pipeline_statistics<'m>(&'m mut self, pipeline_statistics: QueryPipelineStatisticFlags) -> &'m mut CommandBufferInheritanceInfoBuilder<'b> {
        self
    }

}


/// A `VkCommandBufferBeginInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CommandBufferBeginInfo<'s> {
    raw: vks::VkCommandBufferBeginInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> CommandBufferBeginInfo<'s> {
    pub fn builder<'b>() -> CommandBufferBeginInfoBuilder<'b> {
        CommandBufferBeginInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn inheritance_info(&self) {
    }

    pub fn raw(&self) -> &vks::VkCommandBufferBeginInfo {
        &self.raw
    }
}


/// A builder for `VkCommandBufferBeginInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct CommandBufferBeginInfoBuilder<'b> {
    raw: vks::VkCommandBufferBeginInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> CommandBufferBeginInfoBuilder<'b> {
    pub fn new() -> CommandBufferBeginInfoBuilder<'b> {
        CommandBufferBeginInfoBuilder {
            raw: vks::VkCommandBufferBeginInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CommandBufferBeginInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: CommandBufferUsageFlags) -> &'m mut CommandBufferBeginInfoBuilder<'b> {
        self
    }

    pub fn inheritance_info<'m, 'a>(&'m mut self, inheritance_info: &'a CommandBufferInheritanceInfo) -> &'m mut CommandBufferBeginInfoBuilder<'b> {
        self
    }

}


/// A `VkRenderPassBeginInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RenderPassBeginInfo<'s> {
    raw: vks::VkRenderPassBeginInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> RenderPassBeginInfo<'s> {
    pub fn builder<'b>() -> RenderPassBeginInfoBuilder<'b> {
        RenderPassBeginInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn render_pass(&self) {
    }

    pub fn framebuffer(&self) {
    }

    pub fn render_area(&self) {
    }

    pub fn clear_value_count(&self) {
    }

    pub fn clear_values(&self) {
    }

    pub fn raw(&self) -> &vks::VkRenderPassBeginInfo {
        &self.raw
    }
}


/// A builder for `VkRenderPassBeginInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct RenderPassBeginInfoBuilder<'b> {
    raw: vks::VkRenderPassBeginInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> RenderPassBeginInfoBuilder<'b> {
    pub fn new() -> RenderPassBeginInfoBuilder<'b> {
        RenderPassBeginInfoBuilder {
            raw: vks::VkRenderPassBeginInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut RenderPassBeginInfoBuilder<'b> {
        self
    }

    pub fn render_pass<'m>(&'m mut self, render_pass: RenderPass) -> &'m mut RenderPassBeginInfoBuilder<'b> {
        self
    }

    pub fn framebuffer<'m>(&'m mut self, framebuffer: Framebuffer) -> &'m mut RenderPassBeginInfoBuilder<'b> {
        self
    }

    pub fn render_area<'m>(&'m mut self, render_area: Rect2d) -> &'m mut RenderPassBeginInfoBuilder<'b> {
        self
    }

    pub fn clear_value_count<'m>(&'m mut self, clear_value_count: u32) -> &'m mut RenderPassBeginInfoBuilder<'b> {
        self
    }

    pub fn clear_values<'m, 'a>(&'m mut self, clear_values: &'a ClearValue) -> &'m mut RenderPassBeginInfoBuilder<'b> {
        self
    }

}


/// A `VkClearDepthStencilValue`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ClearDepthStencilValue {
    raw: vks::VkClearDepthStencilValue,
}

impl ClearDepthStencilValue {
    pub fn builder() -> ClearDepthStencilValueBuilder {
        ClearDepthStencilValueBuilder::new()
    }

    pub fn depth(&self) {
    }

    pub fn stencil(&self) {
    }

    pub fn raw(&self) -> &vks::VkClearDepthStencilValue {
        &self.raw
    }
}


/// A builder for `VkClearDepthStencilValue`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ClearDepthStencilValueBuilder {
    raw: vks::VkClearDepthStencilValue,
}

impl ClearDepthStencilValueBuilder {
    pub fn new() -> ClearDepthStencilValueBuilder {
        ClearDepthStencilValueBuilder {
            raw: vks::VkClearDepthStencilValue::default(),
        }
    }

    pub fn depth<'m>(&'m mut self, depth: f32) -> &'m mut ClearDepthStencilValueBuilder {
        self
    }

    pub fn stencil<'m>(&'m mut self, stencil: u32) -> &'m mut ClearDepthStencilValueBuilder {
        self
    }

}


/// A `VkClearAttachment`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ClearAttachment {
    raw: vks::VkClearAttachment,
}

impl ClearAttachment {
    pub fn builder() -> ClearAttachmentBuilder {
        ClearAttachmentBuilder::new()
    }

    pub fn aspect_mask(&self) {
    }

    pub fn color_attachment(&self) {
    }

    pub fn clear_value(&self) {
    }

    pub fn raw(&self) -> &vks::VkClearAttachment {
        &self.raw
    }
}


/// A builder for `VkClearAttachment`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ClearAttachmentBuilder {
    raw: vks::VkClearAttachment,
}

impl ClearAttachmentBuilder {
    pub fn new() -> ClearAttachmentBuilder {
        ClearAttachmentBuilder {
            raw: vks::VkClearAttachment::default(),
        }
    }

    pub fn aspect_mask<'m>(&'m mut self, aspect_mask: ImageAspectFlags) -> &'m mut ClearAttachmentBuilder {
        self
    }

    pub fn color_attachment<'m>(&'m mut self, color_attachment: u32) -> &'m mut ClearAttachmentBuilder {
        self
    }

    pub fn clear_value<'m>(&'m mut self, clear_value: ClearValue) -> &'m mut ClearAttachmentBuilder {
        self
    }

}


/// A `VkAttachmentDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AttachmentDescription {
    raw: vks::VkAttachmentDescription,
}

impl AttachmentDescription {
    pub fn builder() -> AttachmentDescriptionBuilder {
        AttachmentDescriptionBuilder::new()
    }

    pub fn flags(&self) {
    }

    pub fn format(&self) {
    }

    pub fn samples(&self) {
    }

    pub fn load_op(&self) {
    }

    pub fn store_op(&self) {
    }

    pub fn stencil_load_op(&self) {
    }

    pub fn stencil_store_op(&self) {
    }

    pub fn initial_layout(&self) {
    }

    pub fn final_layout(&self) {
    }

    pub fn raw(&self) -> &vks::VkAttachmentDescription {
        &self.raw
    }
}


/// A builder for `VkAttachmentDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct AttachmentDescriptionBuilder {
    raw: vks::VkAttachmentDescription,
}

impl AttachmentDescriptionBuilder {
    pub fn new() -> AttachmentDescriptionBuilder {
        AttachmentDescriptionBuilder {
            raw: vks::VkAttachmentDescription::default(),
        }
    }

    pub fn flags<'m>(&'m mut self, flags: AttachmentDescriptionFlags) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn samples<'m>(&'m mut self, samples: SampleCountFlags) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn load_op<'m>(&'m mut self, load_op: AttachmentLoadOp) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn store_op<'m>(&'m mut self, store_op: AttachmentStoreOp) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn stencil_load_op<'m>(&'m mut self, stencil_load_op: AttachmentLoadOp) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn stencil_store_op<'m>(&'m mut self, stencil_store_op: AttachmentStoreOp) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn initial_layout<'m>(&'m mut self, initial_layout: ImageLayout) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

    pub fn final_layout<'m>(&'m mut self, final_layout: ImageLayout) -> &'m mut AttachmentDescriptionBuilder {
        self
    }

}


/// A `VkAttachmentReference`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AttachmentReference {
    raw: vks::VkAttachmentReference,
}

impl AttachmentReference {
    pub fn builder() -> AttachmentReferenceBuilder {
        AttachmentReferenceBuilder::new()
    }

    pub fn attachment(&self) {
    }

    pub fn layout(&self) {
    }

    pub fn raw(&self) -> &vks::VkAttachmentReference {
        &self.raw
    }
}


/// A builder for `VkAttachmentReference`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct AttachmentReferenceBuilder {
    raw: vks::VkAttachmentReference,
}

impl AttachmentReferenceBuilder {
    pub fn new() -> AttachmentReferenceBuilder {
        AttachmentReferenceBuilder {
            raw: vks::VkAttachmentReference::default(),
        }
    }

    pub fn attachment<'m>(&'m mut self, attachment: u32) -> &'m mut AttachmentReferenceBuilder {
        self
    }

    pub fn layout<'m>(&'m mut self, layout: ImageLayout) -> &'m mut AttachmentReferenceBuilder {
        self
    }

}


/// A `VkSubpassDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SubpassDescription<'s> {
    raw: vks::VkSubpassDescription,
    _p: PhantomData<&'s ()>,
}

impl<'s> SubpassDescription<'s> {
    pub fn builder<'b>() -> SubpassDescriptionBuilder<'b> {
        SubpassDescriptionBuilder::new()
    }

    pub fn flags(&self) {
    }

    pub fn pipeline_bind_point(&self) {
    }

    pub fn input_attachment_count(&self) {
    }

    pub fn input_attachments(&self) {
    }

    pub fn color_attachment_count(&self) {
    }

    pub fn color_attachments(&self) {
    }

    pub fn resolve_attachments(&self) {
    }

    pub fn depth_stencil_attachment(&self) {
    }

    pub fn preserve_attachment_count(&self) {
    }

    pub fn preserve_attachments(&self) {
    }

    pub fn raw(&self) -> &vks::VkSubpassDescription {
        &self.raw
    }
}


/// A builder for `VkSubpassDescription`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SubpassDescriptionBuilder<'b> {
    raw: vks::VkSubpassDescription,
    _p: PhantomData<&'b ()>,
}

impl<'b> SubpassDescriptionBuilder<'b> {
    pub fn new() -> SubpassDescriptionBuilder<'b> {
        SubpassDescriptionBuilder {
            raw: vks::VkSubpassDescription::default(),
            _p: PhantomData,
        }
    }

    pub fn flags<'m>(&'m mut self, flags: SubpassDescriptionFlags) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn pipeline_bind_point<'m>(&'m mut self, pipeline_bind_point: PipelineBindPoint) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn input_attachment_count<'m>(&'m mut self, input_attachment_count: u32) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn input_attachments<'m, 'a>(&'m mut self, input_attachments: &'a AttachmentReference) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn color_attachment_count<'m>(&'m mut self, color_attachment_count: u32) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn color_attachments<'m, 'a>(&'m mut self, color_attachments: &'a AttachmentReference) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn resolve_attachments<'m, 'a>(&'m mut self, resolve_attachments: &'a AttachmentReference) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn depth_stencil_attachment<'m, 'a>(&'m mut self, depth_stencil_attachment: &'a AttachmentReference) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn preserve_attachment_count<'m>(&'m mut self, preserve_attachment_count: u32) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

    pub fn preserve_attachments<'m, 'a>(&'m mut self, preserve_attachments: &'a u32) -> &'m mut SubpassDescriptionBuilder<'b> {
        self
    }

}


/// A `VkSubpassDependency`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SubpassDependency {
    raw: vks::VkSubpassDependency,
}

impl SubpassDependency {
    pub fn builder() -> SubpassDependencyBuilder {
        SubpassDependencyBuilder::new()
    }

    pub fn src_subpass(&self) {
    }

    pub fn dst_subpass(&self) {
    }

    pub fn src_stage_mask(&self) {
    }

    pub fn dst_stage_mask(&self) {
    }

    pub fn src_access_mask(&self) {
    }

    pub fn dst_access_mask(&self) {
    }

    pub fn dependency_flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSubpassDependency {
        &self.raw
    }
}


/// A builder for `VkSubpassDependency`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SubpassDependencyBuilder {
    raw: vks::VkSubpassDependency,
}

impl SubpassDependencyBuilder {
    pub fn new() -> SubpassDependencyBuilder {
        SubpassDependencyBuilder {
            raw: vks::VkSubpassDependency::default(),
        }
    }

    pub fn src_subpass<'m>(&'m mut self, src_subpass: u32) -> &'m mut SubpassDependencyBuilder {
        self
    }

    pub fn dst_subpass<'m>(&'m mut self, dst_subpass: u32) -> &'m mut SubpassDependencyBuilder {
        self
    }

    pub fn src_stage_mask<'m>(&'m mut self, src_stage_mask: PipelineStageFlags) -> &'m mut SubpassDependencyBuilder {
        self
    }

    pub fn dst_stage_mask<'m>(&'m mut self, dst_stage_mask: PipelineStageFlags) -> &'m mut SubpassDependencyBuilder {
        self
    }

    pub fn src_access_mask<'m>(&'m mut self, src_access_mask: AccessFlags) -> &'m mut SubpassDependencyBuilder {
        self
    }

    pub fn dst_access_mask<'m>(&'m mut self, dst_access_mask: AccessFlags) -> &'m mut SubpassDependencyBuilder {
        self
    }

    pub fn dependency_flags<'m>(&'m mut self, dependency_flags: DependencyFlags) -> &'m mut SubpassDependencyBuilder {
        self
    }

}


/// A `VkRenderPassCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RenderPassCreateInfo<'s> {
    raw: vks::VkRenderPassCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> RenderPassCreateInfo<'s> {
    pub fn builder<'b>() -> RenderPassCreateInfoBuilder<'b> {
        RenderPassCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn attachment_count(&self) {
    }

    pub fn attachments(&self) {
    }

    pub fn subpass_count(&self) {
    }

    pub fn subpasses(&self) {
    }

    pub fn dependency_count(&self) {
    }

    pub fn dependencies(&self) {
    }

    pub fn raw(&self) -> &vks::VkRenderPassCreateInfo {
        &self.raw
    }
}


/// A builder for `VkRenderPassCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct RenderPassCreateInfoBuilder<'b> {
    raw: vks::VkRenderPassCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> RenderPassCreateInfoBuilder<'b> {
    pub fn new() -> RenderPassCreateInfoBuilder<'b> {
        RenderPassCreateInfoBuilder {
            raw: vks::VkRenderPassCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: RenderPassCreateFlags) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn attachment_count<'m>(&'m mut self, attachment_count: u32) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn attachments<'m, 'a>(&'m mut self, attachments: &'a AttachmentDescription) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn subpass_count<'m>(&'m mut self, subpass_count: u32) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn subpasses<'m, 'a>(&'m mut self, subpasses: &'a SubpassDescription) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn dependency_count<'m>(&'m mut self, dependency_count: u32) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

    pub fn dependencies<'m, 'a>(&'m mut self, dependencies: &'a SubpassDependency) -> &'m mut RenderPassCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkEventCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct EventCreateInfo<'s> {
    raw: vks::VkEventCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> EventCreateInfo<'s> {
    pub fn builder<'b>() -> EventCreateInfoBuilder<'b> {
        EventCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkEventCreateInfo {
        &self.raw
    }
}


/// A builder for `VkEventCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct EventCreateInfoBuilder<'b> {
    raw: vks::VkEventCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> EventCreateInfoBuilder<'b> {
    pub fn new() -> EventCreateInfoBuilder<'b> {
        EventCreateInfoBuilder {
            raw: vks::VkEventCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut EventCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: EventCreateFlags) -> &'m mut EventCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkFenceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FenceCreateInfo<'s> {
    raw: vks::VkFenceCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> FenceCreateInfo<'s> {
    pub fn builder<'b>() -> FenceCreateInfoBuilder<'b> {
        FenceCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkFenceCreateInfo {
        &self.raw
    }
}


/// A builder for `VkFenceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FenceCreateInfoBuilder<'b> {
    raw: vks::VkFenceCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> FenceCreateInfoBuilder<'b> {
    pub fn new() -> FenceCreateInfoBuilder<'b> {
        FenceCreateInfoBuilder {
            raw: vks::VkFenceCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut FenceCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: FenceCreateFlags) -> &'m mut FenceCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceFeatures`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceFeatures {
    raw: vks::VkPhysicalDeviceFeatures,
}

impl PhysicalDeviceFeatures {
    pub fn builder() -> PhysicalDeviceFeaturesBuilder {
        PhysicalDeviceFeaturesBuilder::new()
    }

    pub fn robust_buffer_access(&self) {
    }

    pub fn full_draw_index_uint_32(&self) {
    }

    pub fn image_cube_array(&self) {
    }

    pub fn independent_blend(&self) {
    }

    pub fn geometry_shader(&self) {
    }

    pub fn tessellation_shader(&self) {
    }

    pub fn sample_rate_shading(&self) {
    }

    pub fn dual_src_blend(&self) {
    }

    pub fn logic_op(&self) {
    }

    pub fn multi_draw_indirect(&self) {
    }

    pub fn draw_indirect_first_instance(&self) {
    }

    pub fn depth_clamp(&self) {
    }

    pub fn depth_bias_clamp(&self) {
    }

    pub fn fill_mode_non_solid(&self) {
    }

    pub fn depth_bounds(&self) {
    }

    pub fn wide_lines(&self) {
    }

    pub fn large_points(&self) {
    }

    pub fn alpha_to_one(&self) {
    }

    pub fn multi_viewport(&self) {
    }

    pub fn sampler_anisotropy(&self) {
    }

    pub fn texture_compression_et_c2(&self) {
    }

    pub fn texture_compression_as_tc_ld_r(&self) {
    }

    pub fn texture_compression_bc(&self) {
    }

    pub fn occlusion_query_precise(&self) {
    }

    pub fn pipeline_statistics_query(&self) {
    }

    pub fn vertex_pipeline_stores_and_atomics(&self) {
    }

    pub fn fragment_stores_and_atomics(&self) {
    }

    pub fn shader_tessellation_and_geometry_point_size(&self) {
    }

    pub fn shader_image_gather_extended(&self) {
    }

    pub fn shader_storage_image_extended_formats(&self) {
    }

    pub fn shader_storage_image_multisample(&self) {
    }

    pub fn shader_storage_image_read_without_format(&self) {
    }

    pub fn shader_storage_image_write_without_format(&self) {
    }

    pub fn shader_uniform_buffer_array_dynamic_indexing(&self) {
    }

    pub fn shader_sampled_image_array_dynamic_indexing(&self) {
    }

    pub fn shader_storage_buffer_array_dynamic_indexing(&self) {
    }

    pub fn shader_storage_image_array_dynamic_indexing(&self) {
    }

    pub fn shader_clip_distance(&self) {
    }

    pub fn shader_cull_distance(&self) {
    }

    pub fn shader_float_64(&self) {
    }

    pub fn shader_int_64(&self) {
    }

    pub fn shader_int_16(&self) {
    }

    pub fn shader_resource_residency(&self) {
    }

    pub fn shader_resource_min_lod(&self) {
    }

    pub fn sparse_binding(&self) {
    }

    pub fn sparse_residency_buffer(&self) {
    }

    pub fn sparse_residency_image_2d(&self) {
    }

    pub fn sparse_residency_image_3d(&self) {
    }

    pub fn sparse_residency_2samples(&self) {
    }

    pub fn sparse_residency_4samples(&self) {
    }

    pub fn sparse_residency_8samples(&self) {
    }

    pub fn sparse_residency_16_samples(&self) {
    }

    pub fn sparse_residency_aliased(&self) {
    }

    pub fn variable_multisample_rate(&self) {
    }

    pub fn inherited_queries(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceFeatures {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceFeatures`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceFeaturesBuilder {
    raw: vks::VkPhysicalDeviceFeatures,
}

impl PhysicalDeviceFeaturesBuilder {
    pub fn new() -> PhysicalDeviceFeaturesBuilder {
        PhysicalDeviceFeaturesBuilder {
            raw: vks::VkPhysicalDeviceFeatures::default(),
        }
    }

    pub fn robust_buffer_access<'m>(&'m mut self, robust_buffer_access: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn full_draw_index_uint_32<'m>(&'m mut self, full_draw_index_uint_32: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn image_cube_array<'m>(&'m mut self, image_cube_array: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn independent_blend<'m>(&'m mut self, independent_blend: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn geometry_shader<'m>(&'m mut self, geometry_shader: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn tessellation_shader<'m>(&'m mut self, tessellation_shader: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sample_rate_shading<'m>(&'m mut self, sample_rate_shading: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn dual_src_blend<'m>(&'m mut self, dual_src_blend: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn logic_op<'m>(&'m mut self, logic_op: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn multi_draw_indirect<'m>(&'m mut self, multi_draw_indirect: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn draw_indirect_first_instance<'m>(&'m mut self, draw_indirect_first_instance: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn depth_clamp<'m>(&'m mut self, depth_clamp: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn depth_bias_clamp<'m>(&'m mut self, depth_bias_clamp: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn fill_mode_non_solid<'m>(&'m mut self, fill_mode_non_solid: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn depth_bounds<'m>(&'m mut self, depth_bounds: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn wide_lines<'m>(&'m mut self, wide_lines: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn large_points<'m>(&'m mut self, large_points: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn alpha_to_one<'m>(&'m mut self, alpha_to_one: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn multi_viewport<'m>(&'m mut self, multi_viewport: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sampler_anisotropy<'m>(&'m mut self, sampler_anisotropy: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn texture_compression_et_c2<'m>(&'m mut self, texture_compression_et_c2: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn texture_compression_as_tc_ld_r<'m>(&'m mut self, texture_compression_as_tc_ld_r: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn texture_compression_bc<'m>(&'m mut self, texture_compression_bc: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn occlusion_query_precise<'m>(&'m mut self, occlusion_query_precise: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn pipeline_statistics_query<'m>(&'m mut self, pipeline_statistics_query: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn vertex_pipeline_stores_and_atomics<'m>(&'m mut self, vertex_pipeline_stores_and_atomics: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn fragment_stores_and_atomics<'m>(&'m mut self, fragment_stores_and_atomics: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_tessellation_and_geometry_point_size<'m>(&'m mut self, shader_tessellation_and_geometry_point_size: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_image_gather_extended<'m>(&'m mut self, shader_image_gather_extended: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_storage_image_extended_formats<'m>(&'m mut self, shader_storage_image_extended_formats: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_storage_image_multisample<'m>(&'m mut self, shader_storage_image_multisample: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_storage_image_read_without_format<'m>(&'m mut self, shader_storage_image_read_without_format: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_storage_image_write_without_format<'m>(&'m mut self, shader_storage_image_write_without_format: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_uniform_buffer_array_dynamic_indexing<'m>(&'m mut self, shader_uniform_buffer_array_dynamic_indexing: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_sampled_image_array_dynamic_indexing<'m>(&'m mut self, shader_sampled_image_array_dynamic_indexing: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_storage_buffer_array_dynamic_indexing<'m>(&'m mut self, shader_storage_buffer_array_dynamic_indexing: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_storage_image_array_dynamic_indexing<'m>(&'m mut self, shader_storage_image_array_dynamic_indexing: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_clip_distance<'m>(&'m mut self, shader_clip_distance: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_cull_distance<'m>(&'m mut self, shader_cull_distance: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_float_64<'m>(&'m mut self, shader_float_64: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_int_64<'m>(&'m mut self, shader_int_64: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_int_16<'m>(&'m mut self, shader_int_16: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_resource_residency<'m>(&'m mut self, shader_resource_residency: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn shader_resource_min_lod<'m>(&'m mut self, shader_resource_min_lod: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_binding<'m>(&'m mut self, sparse_binding: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_buffer<'m>(&'m mut self, sparse_residency_buffer: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_image_2d<'m>(&'m mut self, sparse_residency_image_2d: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_image_3d<'m>(&'m mut self, sparse_residency_image_3d: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_2samples<'m>(&'m mut self, sparse_residency_2samples: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_4samples<'m>(&'m mut self, sparse_residency_4samples: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_8samples<'m>(&'m mut self, sparse_residency_8samples: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_16_samples<'m>(&'m mut self, sparse_residency_16_samples: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn sparse_residency_aliased<'m>(&'m mut self, sparse_residency_aliased: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn variable_multisample_rate<'m>(&'m mut self, variable_multisample_rate: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

    pub fn inherited_queries<'m>(&'m mut self, inherited_queries: bool) -> &'m mut PhysicalDeviceFeaturesBuilder {
        self
    }

}


/// A `VkPhysicalDeviceSparseProperties`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    raw: vks::VkPhysicalDeviceSparseProperties,
}

impl PhysicalDeviceSparseProperties {
    pub fn residency_standard_2d_block_shape(&self) {
    }

    pub fn residency_standard_2d_multisample_block_shape(&self) {
    }

    pub fn residency_standard_3d_block_shape(&self) {
    }

    pub fn residency_aligned_mip_size(&self) {
    }

    pub fn residency_non_resident_strict(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceSparseProperties {
        &self.raw
    }
}


/// A `VkPhysicalDeviceLimits`.
///
/// compute stage limits
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceLimits {
    raw: vks::VkPhysicalDeviceLimits,
}

impl PhysicalDeviceLimits {
    pub fn max_image_dimension_1d(&self) {
    }

    pub fn max_image_dimension_2d(&self) {
    }

    pub fn max_image_dimension_3d(&self) {
    }

    pub fn max_image_dimension_cube(&self) {
    }

    pub fn max_image_array_layers(&self) {
    }

    pub fn max_texel_buffer_elements(&self) {
    }

    pub fn max_uniform_buffer_range(&self) {
    }

    pub fn max_storage_buffer_range(&self) {
    }

    pub fn max_push_constants_size(&self) {
    }

    pub fn max_memory_allocation_count(&self) {
    }

    pub fn max_sampler_allocation_count(&self) {
    }

    pub fn buffer_image_granularity(&self) {
    }

    pub fn sparse_address_space_size(&self) {
    }

    pub fn max_bound_descriptor_sets(&self) {
    }

    pub fn max_per_stage_descriptor_samplers(&self) {
    }

    pub fn max_per_stage_descriptor_uniform_buffers(&self) {
    }

    pub fn max_per_stage_descriptor_storage_buffers(&self) {
    }

    pub fn max_per_stage_descriptor_sampled_images(&self) {
    }

    pub fn max_per_stage_descriptor_storage_images(&self) {
    }

    pub fn max_per_stage_descriptor_input_attachments(&self) {
    }

    pub fn max_per_stage_resources(&self) {
    }

    pub fn max_descriptor_set_samplers(&self) {
    }

    pub fn max_descriptor_set_uniform_buffers(&self) {
    }

    pub fn max_descriptor_set_uniform_buffers_dynamic(&self) {
    }

    pub fn max_descriptor_set_storage_buffers(&self) {
    }

    pub fn max_descriptor_set_storage_buffers_dynamic(&self) {
    }

    pub fn max_descriptor_set_sampled_images(&self) {
    }

    pub fn max_descriptor_set_storage_images(&self) {
    }

    pub fn max_descriptor_set_input_attachments(&self) {
    }

    pub fn max_vertex_input_attributes(&self) {
    }

    pub fn max_vertex_input_bindings(&self) {
    }

    pub fn max_vertex_input_attribute_offset(&self) {
    }

    pub fn max_vertex_input_binding_stride(&self) {
    }

    pub fn max_vertex_output_components(&self) {
    }

    pub fn max_tessellation_generation_level(&self) {
    }

    pub fn max_tessellation_patch_size(&self) {
    }

    pub fn max_tessellation_control_per_vertex_input_components(&self) {
    }

    pub fn max_tessellation_control_per_vertex_output_components(&self) {
    }

    pub fn max_tessellation_control_per_patch_output_components(&self) {
    }

    pub fn max_tessellation_control_total_output_components(&self) {
    }

    pub fn max_tessellation_evaluation_input_components(&self) {
    }

    pub fn max_tessellation_evaluation_output_components(&self) {
    }

    pub fn max_geometry_shader_invocations(&self) {
    }

    pub fn max_geometry_input_components(&self) {
    }

    pub fn max_geometry_output_components(&self) {
    }

    pub fn max_geometry_output_vertices(&self) {
    }

    pub fn max_geometry_total_output_components(&self) {
    }

    pub fn max_fragment_input_components(&self) {
    }

    pub fn max_fragment_output_attachments(&self) {
    }

    pub fn max_fragment_dual_src_attachments(&self) {
    }

    pub fn max_fragment_combined_output_resources(&self) {
    }

    pub fn max_compute_shared_memory_size(&self) {
    }

    pub fn max_compute_work_group_count(&self) {
    }

    pub fn max_compute_work_group_invocations(&self) {
    }

    pub fn max_compute_work_group_size(&self) {
    }

    pub fn sub_pixel_precision_bits(&self) {
    }

    pub fn sub_texel_precision_bits(&self) {
    }

    pub fn mipmap_precision_bits(&self) {
    }

    pub fn max_draw_indexed_index_value(&self) {
    }

    pub fn max_draw_indirect_count(&self) {
    }

    pub fn max_sampler_lod_bias(&self) {
    }

    pub fn max_sampler_anisotropy(&self) {
    }

    pub fn max_viewports(&self) {
    }

    pub fn max_viewport_dimensions(&self) {
    }

    pub fn viewport_bounds_range(&self) {
    }

    pub fn viewport_sub_pixel_bits(&self) {
    }

    pub fn min_memory_map_alignment(&self) {
    }

    pub fn min_texel_buffer_offset_alignment(&self) {
    }

    pub fn min_uniform_buffer_offset_alignment(&self) {
    }

    pub fn min_storage_buffer_offset_alignment(&self) {
    }

    pub fn min_texel_offset(&self) {
    }

    pub fn max_texel_offset(&self) {
    }

    pub fn min_texel_gather_offset(&self) {
    }

    pub fn max_texel_gather_offset(&self) {
    }

    pub fn min_interpolation_offset(&self) {
    }

    pub fn max_interpolation_offset(&self) {
    }

    pub fn sub_pixel_interpolation_offset_bits(&self) {
    }

    pub fn max_framebuffer_width(&self) {
    }

    pub fn max_framebuffer_height(&self) {
    }

    pub fn max_framebuffer_layers(&self) {
    }

    pub fn framebuffer_color_sample_counts(&self) {
    }

    pub fn framebuffer_depth_sample_counts(&self) {
    }

    pub fn framebuffer_stencil_sample_counts(&self) {
    }

    pub fn framebuffer_no_attachments_sample_counts(&self) {
    }

    pub fn max_color_attachments(&self) {
    }

    pub fn sampled_image_color_sample_counts(&self) {
    }

    pub fn sampled_image_integer_sample_counts(&self) {
    }

    pub fn sampled_image_depth_sample_counts(&self) {
    }

    pub fn sampled_image_stencil_sample_counts(&self) {
    }

    pub fn storage_image_sample_counts(&self) {
    }

    pub fn max_sample_mask_words(&self) {
    }

    pub fn timestamp_compute_and_graphics(&self) {
    }

    pub fn timestamp_period(&self) {
    }

    pub fn max_clip_distances(&self) {
    }

    pub fn max_cull_distances(&self) {
    }

    pub fn max_combined_clip_and_cull_distances(&self) {
    }

    pub fn discrete_queue_priorities(&self) {
    }

    pub fn point_size_range(&self) {
    }

    pub fn line_width_range(&self) {
    }

    pub fn point_size_granularity(&self) {
    }

    pub fn line_width_granularity(&self) {
    }

    pub fn strict_lines(&self) {
    }

    pub fn standard_sample_locations(&self) {
    }

    pub fn optimal_buffer_copy_offset_alignment(&self) {
    }

    pub fn optimal_buffer_copy_row_pitch_alignment(&self) {
    }

    pub fn non_coherent_atom_size(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceLimits {
        &self.raw
    }
}


/// A `VkSemaphoreCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SemaphoreCreateInfo<'s> {
    raw: vks::VkSemaphoreCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SemaphoreCreateInfo<'s> {
    pub fn builder<'b>() -> SemaphoreCreateInfoBuilder<'b> {
        SemaphoreCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSemaphoreCreateInfo {
        &self.raw
    }
}


/// A builder for `VkSemaphoreCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SemaphoreCreateInfoBuilder<'b> {
    raw: vks::VkSemaphoreCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SemaphoreCreateInfoBuilder<'b> {
    pub fn new() -> SemaphoreCreateInfoBuilder<'b> {
        SemaphoreCreateInfoBuilder {
            raw: vks::VkSemaphoreCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SemaphoreCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SemaphoreCreateFlags) -> &'m mut SemaphoreCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkQueryPoolCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct QueryPoolCreateInfo<'s> {
    raw: vks::VkQueryPoolCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> QueryPoolCreateInfo<'s> {
    pub fn builder<'b>() -> QueryPoolCreateInfoBuilder<'b> {
        QueryPoolCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn query_type(&self) {
    }

    pub fn query_count(&self) {
    }

    pub fn pipeline_statistics(&self) {
    }

    pub fn raw(&self) -> &vks::VkQueryPoolCreateInfo {
        &self.raw
    }
}


/// A builder for `VkQueryPoolCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct QueryPoolCreateInfoBuilder<'b> {
    raw: vks::VkQueryPoolCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> QueryPoolCreateInfoBuilder<'b> {
    pub fn new() -> QueryPoolCreateInfoBuilder<'b> {
        QueryPoolCreateInfoBuilder {
            raw: vks::VkQueryPoolCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut QueryPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: QueryPoolCreateFlags) -> &'m mut QueryPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn query_type<'m>(&'m mut self, query_type: QueryType) -> &'m mut QueryPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn query_count<'m>(&'m mut self, query_count: u32) -> &'m mut QueryPoolCreateInfoBuilder<'b> {
        self
    }

    pub fn pipeline_statistics<'m>(&'m mut self, pipeline_statistics: QueryPipelineStatisticFlags) -> &'m mut QueryPoolCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkFramebufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FramebufferCreateInfo<'s> {
    raw: vks::VkFramebufferCreateInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> FramebufferCreateInfo<'s> {
    pub fn builder<'b>() -> FramebufferCreateInfoBuilder<'b> {
        FramebufferCreateInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn render_pass(&self) {
    }

    pub fn attachment_count(&self) {
    }

    pub fn attachments(&self) {
    }

    pub fn width(&self) {
    }

    pub fn height(&self) {
    }

    pub fn layers(&self) {
    }

    pub fn raw(&self) -> &vks::VkFramebufferCreateInfo {
        &self.raw
    }
}


/// A builder for `VkFramebufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FramebufferCreateInfoBuilder<'b> {
    raw: vks::VkFramebufferCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> FramebufferCreateInfoBuilder<'b> {
    pub fn new() -> FramebufferCreateInfoBuilder<'b> {
        FramebufferCreateInfoBuilder {
            raw: vks::VkFramebufferCreateInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: FramebufferCreateFlags) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn render_pass<'m>(&'m mut self, render_pass: RenderPass) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn attachment_count<'m>(&'m mut self, attachment_count: u32) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn attachments<'m, 'a>(&'m mut self, attachments: &'a ImageView) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn width<'m>(&'m mut self, width: u32) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn height<'m>(&'m mut self, height: u32) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

    pub fn layers<'m>(&'m mut self, layers: u32) -> &'m mut FramebufferCreateInfoBuilder<'b> {
        self
    }

}


/// A `VkDrawIndirectCommand`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DrawIndirectCommand {
    raw: vks::VkDrawIndirectCommand,
}

impl DrawIndirectCommand {
    pub fn builder() -> DrawIndirectCommandBuilder {
        DrawIndirectCommandBuilder::new()
    }

    pub fn vertex_count(&self) {
    }

    pub fn instance_count(&self) {
    }

    pub fn first_vertex(&self) {
    }

    pub fn first_instance(&self) {
    }

    pub fn raw(&self) -> &vks::VkDrawIndirectCommand {
        &self.raw
    }
}


/// A builder for `VkDrawIndirectCommand`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DrawIndirectCommandBuilder {
    raw: vks::VkDrawIndirectCommand,
}

impl DrawIndirectCommandBuilder {
    pub fn new() -> DrawIndirectCommandBuilder {
        DrawIndirectCommandBuilder {
            raw: vks::VkDrawIndirectCommand::default(),
        }
    }

    pub fn vertex_count<'m>(&'m mut self, vertex_count: u32) -> &'m mut DrawIndirectCommandBuilder {
        self
    }

    pub fn instance_count<'m>(&'m mut self, instance_count: u32) -> &'m mut DrawIndirectCommandBuilder {
        self
    }

    pub fn first_vertex<'m>(&'m mut self, first_vertex: u32) -> &'m mut DrawIndirectCommandBuilder {
        self
    }

    pub fn first_instance<'m>(&'m mut self, first_instance: u32) -> &'m mut DrawIndirectCommandBuilder {
        self
    }

}


/// A `VkDrawIndexedIndirectCommand`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    raw: vks::VkDrawIndexedIndirectCommand,
}

impl DrawIndexedIndirectCommand {
    pub fn builder() -> DrawIndexedIndirectCommandBuilder {
        DrawIndexedIndirectCommandBuilder::new()
    }

    pub fn index_count(&self) {
    }

    pub fn instance_count(&self) {
    }

    pub fn first_index(&self) {
    }

    pub fn vertex_offset(&self) {
    }

    pub fn first_instance(&self) {
    }

    pub fn raw(&self) -> &vks::VkDrawIndexedIndirectCommand {
        &self.raw
    }
}


/// A builder for `VkDrawIndexedIndirectCommand`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DrawIndexedIndirectCommandBuilder {
    raw: vks::VkDrawIndexedIndirectCommand,
}

impl DrawIndexedIndirectCommandBuilder {
    pub fn new() -> DrawIndexedIndirectCommandBuilder {
        DrawIndexedIndirectCommandBuilder {
            raw: vks::VkDrawIndexedIndirectCommand::default(),
        }
    }

    pub fn index_count<'m>(&'m mut self, index_count: u32) -> &'m mut DrawIndexedIndirectCommandBuilder {
        self
    }

    pub fn instance_count<'m>(&'m mut self, instance_count: u32) -> &'m mut DrawIndexedIndirectCommandBuilder {
        self
    }

    pub fn first_index<'m>(&'m mut self, first_index: u32) -> &'m mut DrawIndexedIndirectCommandBuilder {
        self
    }

    pub fn vertex_offset<'m>(&'m mut self, vertex_offset: i32) -> &'m mut DrawIndexedIndirectCommandBuilder {
        self
    }

    pub fn first_instance<'m>(&'m mut self, first_instance: u32) -> &'m mut DrawIndexedIndirectCommandBuilder {
        self
    }

}


/// A `VkDispatchIndirectCommand`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DispatchIndirectCommand {
    raw: vks::VkDispatchIndirectCommand,
}

impl DispatchIndirectCommand {
    pub fn builder() -> DispatchIndirectCommandBuilder {
        DispatchIndirectCommandBuilder::new()
    }

    pub fn x(&self) {
    }

    pub fn y(&self) {
    }

    pub fn z(&self) {
    }

    pub fn raw(&self) -> &vks::VkDispatchIndirectCommand {
        &self.raw
    }
}


/// A builder for `VkDispatchIndirectCommand`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DispatchIndirectCommandBuilder {
    raw: vks::VkDispatchIndirectCommand,
}

impl DispatchIndirectCommandBuilder {
    pub fn new() -> DispatchIndirectCommandBuilder {
        DispatchIndirectCommandBuilder {
            raw: vks::VkDispatchIndirectCommand::default(),
        }
    }

    pub fn x<'m>(&'m mut self, x: u32) -> &'m mut DispatchIndirectCommandBuilder {
        self
    }

    pub fn y<'m>(&'m mut self, y: u32) -> &'m mut DispatchIndirectCommandBuilder {
        self
    }

    pub fn z<'m>(&'m mut self, z: u32) -> &'m mut DispatchIndirectCommandBuilder {
        self
    }

}


/// A `VkSubmitInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SubmitInfo<'s> {
    raw: vks::VkSubmitInfo,
    _p: PhantomData<&'s ()>,
}

impl<'s> SubmitInfo<'s> {
    pub fn builder<'b>() -> SubmitInfoBuilder<'b> {
        SubmitInfoBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn wait_semaphore_count(&self) {
    }

    pub fn wait_semaphores(&self) {
    }

    pub fn wait_dst_stage_mask(&self) {
    }

    pub fn command_buffer_count(&self) {
    }

    pub fn command_buffers(&self) {
    }

    pub fn signal_semaphore_count(&self) {
    }

    pub fn signal_semaphores(&self) {
    }

    pub fn raw(&self) -> &vks::VkSubmitInfo {
        &self.raw
    }
}


/// A builder for `VkSubmitInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SubmitInfoBuilder<'b> {
    raw: vks::VkSubmitInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SubmitInfoBuilder<'b> {
    pub fn new() -> SubmitInfoBuilder<'b> {
        SubmitInfoBuilder {
            raw: vks::VkSubmitInfo::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn wait_semaphore_count<'m>(&'m mut self, wait_semaphore_count: u32) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn wait_semaphores<'m, 'a>(&'m mut self, wait_semaphores: &'a Semaphore) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn wait_dst_stage_mask<'m, 'a>(&'m mut self, wait_dst_stage_mask: &'a PipelineStageFlags) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn command_buffer_count<'m>(&'m mut self, command_buffer_count: u32) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn command_buffers<'m, 'a>(&'m mut self, command_buffers: &'a CommandBuffer) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn signal_semaphore_count<'m>(&'m mut self, signal_semaphore_count: u32) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

    pub fn signal_semaphores<'m, 'a>(&'m mut self, signal_semaphores: &'a Semaphore) -> &'m mut SubmitInfoBuilder<'b> {
        self
    }

}


/// A `VkDisplayPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayPropertiesKhr<'s> {
    raw: vks::VkDisplayPropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> DisplayPropertiesKhr<'s> {
    pub fn display(&self) {
    }

    pub fn display_name(&self) {
    }

    pub fn physical_dimensions(&self) {
    }

    pub fn physical_resolution(&self) {
    }

    pub fn supported_transforms(&self) {
    }

    pub fn plane_reorder_possible(&self) {
    }

    pub fn persistent_content(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayPropertiesKHR {
        &self.raw
    }
}


/// A `VkDisplayPlanePropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayPlanePropertiesKhr {
    raw: vks::VkDisplayPlanePropertiesKHR,
}

impl DisplayPlanePropertiesKhr {
    pub fn current_display(&self) {
    }

    pub fn current_stack_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayPlanePropertiesKHR {
        &self.raw
    }
}


/// A `VkDisplayModeParametersKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayModeParametersKhr {
    raw: vks::VkDisplayModeParametersKHR,
}

impl DisplayModeParametersKhr {
    pub fn builder() -> DisplayModeParametersKhrBuilder {
        DisplayModeParametersKhrBuilder::new()
    }

    pub fn visible_region(&self) {
    }

    pub fn refresh_rate(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayModeParametersKHR {
        &self.raw
    }
}


/// A builder for `VkDisplayModeParametersKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplayModeParametersKhrBuilder {
    raw: vks::VkDisplayModeParametersKHR,
}

impl DisplayModeParametersKhrBuilder {
    pub fn new() -> DisplayModeParametersKhrBuilder {
        DisplayModeParametersKhrBuilder {
            raw: vks::VkDisplayModeParametersKHR::default(),
        }
    }

    pub fn visible_region<'m>(&'m mut self, visible_region: Extent2d) -> &'m mut DisplayModeParametersKhrBuilder {
        self
    }

    pub fn refresh_rate<'m>(&'m mut self, refresh_rate: u32) -> &'m mut DisplayModeParametersKhrBuilder {
        self
    }

}


/// A `VkDisplayModePropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayModePropertiesKhr {
    raw: vks::VkDisplayModePropertiesKHR,
}

impl DisplayModePropertiesKhr {
    pub fn display_mode(&self) {
    }

    pub fn parameters(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayModePropertiesKHR {
        &self.raw
    }
}


/// A `VkDisplayModeCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayModeCreateInfoKhr<'s> {
    raw: vks::VkDisplayModeCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> DisplayModeCreateInfoKhr<'s> {
    pub fn builder<'b>() -> DisplayModeCreateInfoKhrBuilder<'b> {
        DisplayModeCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn parameters(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayModeCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkDisplayModeCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplayModeCreateInfoKhrBuilder<'b> {
    raw: vks::VkDisplayModeCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> DisplayModeCreateInfoKhrBuilder<'b> {
    pub fn new() -> DisplayModeCreateInfoKhrBuilder<'b> {
        DisplayModeCreateInfoKhrBuilder {
            raw: vks::VkDisplayModeCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DisplayModeCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DisplayModeCreateFlagsKhr) -> &'m mut DisplayModeCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn parameters<'m>(&'m mut self, parameters: DisplayModeParametersKhr) -> &'m mut DisplayModeCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkDisplayPlaneCapabilitiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayPlaneCapabilitiesKhr {
    raw: vks::VkDisplayPlaneCapabilitiesKHR,
}

impl DisplayPlaneCapabilitiesKhr {
    pub fn supported_alpha(&self) {
    }

    pub fn min_src_position(&self) {
    }

    pub fn max_src_position(&self) {
    }

    pub fn min_src_extent(&self) {
    }

    pub fn max_src_extent(&self) {
    }

    pub fn min_dst_position(&self) {
    }

    pub fn max_dst_position(&self) {
    }

    pub fn min_dst_extent(&self) {
    }

    pub fn max_dst_extent(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayPlaneCapabilitiesKHR {
        &self.raw
    }
}


/// A `VkDisplaySurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKhr<'s> {
    raw: vks::VkDisplaySurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> DisplaySurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        DisplaySurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn display_mode(&self) {
    }

    pub fn plane_index(&self) {
    }

    pub fn plane_stack_index(&self) {
    }

    pub fn transform(&self) {
    }

    pub fn global_alpha(&self) {
    }

    pub fn alpha_mode(&self) {
    }

    pub fn image_extent(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplaySurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkDisplaySurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplaySurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkDisplaySurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> DisplaySurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        DisplaySurfaceCreateInfoKhrBuilder {
            raw: vks::VkDisplaySurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DisplaySurfaceCreateFlagsKhr) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn display_mode<'m>(&'m mut self, display_mode: DisplayModeKhr) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn plane_index<'m>(&'m mut self, plane_index: u32) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn plane_stack_index<'m>(&'m mut self, plane_stack_index: u32) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn transform<'m>(&'m mut self, transform: SurfaceTransformFlagsKhr) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn global_alpha<'m>(&'m mut self, global_alpha: f32) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn alpha_mode<'m>(&'m mut self, alpha_mode: DisplayPlaneAlphaFlagsKhr) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_extent<'m>(&'m mut self, image_extent: Extent2d) -> &'m mut DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkDisplayPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayPresentInfoKhr<'s> {
    raw: vks::VkDisplayPresentInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> DisplayPresentInfoKhr<'s> {
    pub fn builder<'b>() -> DisplayPresentInfoKhrBuilder<'b> {
        DisplayPresentInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn src_rect(&self) {
    }

    pub fn dst_rect(&self) {
    }

    pub fn persistent(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayPresentInfoKHR {
        &self.raw
    }
}


/// A builder for `VkDisplayPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplayPresentInfoKhrBuilder<'b> {
    raw: vks::VkDisplayPresentInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> DisplayPresentInfoKhrBuilder<'b> {
    pub fn new() -> DisplayPresentInfoKhrBuilder<'b> {
        DisplayPresentInfoKhrBuilder {
            raw: vks::VkDisplayPresentInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DisplayPresentInfoKhrBuilder<'b> {
        self
    }

    pub fn src_rect<'m>(&'m mut self, src_rect: Rect2d) -> &'m mut DisplayPresentInfoKhrBuilder<'b> {
        self
    }

    pub fn dst_rect<'m>(&'m mut self, dst_rect: Rect2d) -> &'m mut DisplayPresentInfoKhrBuilder<'b> {
        self
    }

    pub fn persistent<'m>(&'m mut self, persistent: bool) -> &'m mut DisplayPresentInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkSurfaceCapabilitiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SurfaceCapabilitiesKhr {
    raw: vks::VkSurfaceCapabilitiesKHR,
}

impl SurfaceCapabilitiesKhr {
    pub fn min_image_count(&self) {
    }

    pub fn max_image_count(&self) {
    }

    pub fn current_extent(&self) {
    }

    pub fn min_image_extent(&self) {
    }

    pub fn max_image_extent(&self) {
    }

    pub fn max_image_array_layers(&self) {
    }

    pub fn supported_transforms(&self) {
    }

    pub fn current_transform(&self) {
    }

    pub fn supported_composite_alpha(&self) {
    }

    pub fn supported_usage_flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSurfaceCapabilitiesKHR {
        &self.raw
    }
}


/// A `VkAndroidSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKhr<'s> {
    raw: vks::VkAndroidSurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> AndroidSurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> AndroidSurfaceCreateInfoKhrBuilder<'b> {
        AndroidSurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkAndroidSurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkAndroidSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct AndroidSurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkAndroidSurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> AndroidSurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> AndroidSurfaceCreateInfoKhrBuilder<'b> {
        AndroidSurfaceCreateInfoKhrBuilder {
            raw: vks::VkAndroidSurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut AndroidSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: AndroidSurfaceCreateFlagsKhr) -> &'m mut AndroidSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn window<'m, 'a>(&'m mut self, window: &'a ANativeWindow) -> &'m mut AndroidSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkMirSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MirSurfaceCreateInfoKhr<'s> {
    raw: vks::VkMirSurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MirSurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> MirSurfaceCreateInfoKhrBuilder<'b> {
        MirSurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn connection(&self) {
    }

    pub fn mir_surface(&self) {
    }

    pub fn raw(&self) -> &vks::VkMirSurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkMirSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MirSurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkMirSurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> MirSurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> MirSurfaceCreateInfoKhrBuilder<'b> {
        MirSurfaceCreateInfoKhrBuilder {
            raw: vks::VkMirSurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MirSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: MirSurfaceCreateFlagsKhr) -> &'m mut MirSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn connection<'m, 'a>(&'m mut self, connection: &'a MirConnection) -> &'m mut MirSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn mir_surface<'m, 'a>(&'m mut self, mir_surface: &'a MirSurface) -> &'m mut MirSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkViSurfaceCreateInfoNN`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ViSurfaceCreateInfoNn<'s> {
    raw: vks::VkViSurfaceCreateInfoNN,
    _p: PhantomData<&'s ()>,
}

impl<'s> ViSurfaceCreateInfoNn<'s> {
    pub fn builder<'b>() -> ViSurfaceCreateInfoNnBuilder<'b> {
        ViSurfaceCreateInfoNnBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkViSurfaceCreateInfoNN {
        &self.raw
    }
}


/// A builder for `VkViSurfaceCreateInfoNN`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ViSurfaceCreateInfoNnBuilder<'b> {
    raw: vks::VkViSurfaceCreateInfoNN,
    _p: PhantomData<&'b ()>,
}

impl<'b> ViSurfaceCreateInfoNnBuilder<'b> {
    pub fn new() -> ViSurfaceCreateInfoNnBuilder<'b> {
        ViSurfaceCreateInfoNnBuilder {
            raw: vks::VkViSurfaceCreateInfoNN::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ViSurfaceCreateInfoNnBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ViSurfaceCreateFlagsNn) -> &'m mut ViSurfaceCreateInfoNnBuilder<'b> {
        self
    }

    pub fn window<'m, 'a>(&'m mut self, window: &'a ()) -> &'m mut ViSurfaceCreateInfoNnBuilder<'b> {
        self
    }

}


/// A `VkWaylandSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKhr<'s> {
    raw: vks::VkWaylandSurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> WaylandSurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> WaylandSurfaceCreateInfoKhrBuilder<'b> {
        WaylandSurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn display(&self) {
    }

    pub fn surface(&self) {
    }

    pub fn raw(&self) -> &vks::VkWaylandSurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkWaylandSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct WaylandSurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkWaylandSurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> WaylandSurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> WaylandSurfaceCreateInfoKhrBuilder<'b> {
        WaylandSurfaceCreateInfoKhrBuilder {
            raw: vks::VkWaylandSurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: WaylandSurfaceCreateFlagsKhr) -> &'m mut WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn display<'m, 'a>(&'m mut self, display: &'a wl_display) -> &'m mut WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn surface<'m, 'a>(&'m mut self, surface: &'a wl_surface) -> &'m mut WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkWin32SurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKhr<'s> {
    raw: vks::VkWin32SurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> Win32SurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> Win32SurfaceCreateInfoKhrBuilder<'b> {
        Win32SurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn hinstance(&self) {
    }

    pub fn hwnd(&self) {
    }

    pub fn raw(&self) -> &vks::VkWin32SurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkWin32SurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32SurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkWin32SurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> Win32SurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> Win32SurfaceCreateInfoKhrBuilder<'b> {
        Win32SurfaceCreateInfoKhrBuilder {
            raw: vks::VkWin32SurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut Win32SurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: Win32SurfaceCreateFlagsKhr) -> &'m mut Win32SurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn hinstance<'m>(&'m mut self, hinstance: HINSTANCE) -> &'m mut Win32SurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn hwnd<'m>(&'m mut self, hwnd: HWND) -> &'m mut Win32SurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkXlibSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKhr<'s> {
    raw: vks::VkXlibSurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> XlibSurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> XlibSurfaceCreateInfoKhrBuilder<'b> {
        XlibSurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn dpy(&self) {
    }

    pub fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkXlibSurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkXlibSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct XlibSurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkXlibSurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> XlibSurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> XlibSurfaceCreateInfoKhrBuilder<'b> {
        XlibSurfaceCreateInfoKhrBuilder {
            raw: vks::VkXlibSurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut XlibSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: XlibSurfaceCreateFlagsKhr) -> &'m mut XlibSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn dpy<'m, 'a>(&'m mut self, dpy: &'a Display) -> &'m mut XlibSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn window<'m>(&'m mut self, window: Window) -> &'m mut XlibSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkXcbSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKhr<'s> {
    raw: vks::VkXcbSurfaceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> XcbSurfaceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> XcbSurfaceCreateInfoKhrBuilder<'b> {
        XcbSurfaceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn connection(&self) {
    }

    pub fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkXcbSurfaceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkXcbSurfaceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct XcbSurfaceCreateInfoKhrBuilder<'b> {
    raw: vks::VkXcbSurfaceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> XcbSurfaceCreateInfoKhrBuilder<'b> {
    pub fn new() -> XcbSurfaceCreateInfoKhrBuilder<'b> {
        XcbSurfaceCreateInfoKhrBuilder {
            raw: vks::VkXcbSurfaceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut XcbSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: XcbSurfaceCreateFlagsKhr) -> &'m mut XcbSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn connection<'m, 'a>(&'m mut self, connection: &'a xcb_connection_t) -> &'m mut XcbSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn window<'m>(&'m mut self, window: xcb_window_t) -> &'m mut XcbSurfaceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkSurfaceFormatKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SurfaceFormatKhr {
    raw: vks::VkSurfaceFormatKHR,
}

impl SurfaceFormatKhr {
    pub fn format(&self) {
    }

    pub fn color_space(&self) {
    }

    pub fn raw(&self) -> &vks::VkSurfaceFormatKHR {
        &self.raw
    }
}


/// A `VkSwapchainCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SwapchainCreateInfoKhr<'s> {
    raw: vks::VkSwapchainCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SwapchainCreateInfoKhr<'s> {
    pub fn builder<'b>() -> SwapchainCreateInfoKhrBuilder<'b> {
        SwapchainCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn surface(&self) {
    }

    pub fn min_image_count(&self) {
    }

    pub fn image_format(&self) {
    }

    pub fn image_color_space(&self) {
    }

    pub fn image_extent(&self) {
    }

    pub fn image_array_layers(&self) {
    }

    pub fn image_usage(&self) {
    }

    pub fn image_sharing_mode(&self) {
    }

    pub fn queue_family_index_count(&self) {
    }

    pub fn queue_family_indices(&self) {
    }

    pub fn pre_transform(&self) {
    }

    pub fn composite_alpha(&self) {
    }

    pub fn present_mode(&self) {
    }

    pub fn clipped(&self) {
    }

    pub fn old_swapchain(&self) {
    }

    pub fn raw(&self) -> &vks::VkSwapchainCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkSwapchainCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SwapchainCreateInfoKhrBuilder<'b> {
    raw: vks::VkSwapchainCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> SwapchainCreateInfoKhrBuilder<'b> {
    pub fn new() -> SwapchainCreateInfoKhrBuilder<'b> {
        SwapchainCreateInfoKhrBuilder {
            raw: vks::VkSwapchainCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SwapchainCreateFlagsKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn surface<'m>(&'m mut self, surface: SurfaceKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn min_image_count<'m>(&'m mut self, min_image_count: u32) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_format<'m>(&'m mut self, image_format: Format) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_color_space<'m>(&'m mut self, image_color_space: ColorSpaceKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_extent<'m>(&'m mut self, image_extent: Extent2d) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_array_layers<'m>(&'m mut self, image_array_layers: u32) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_usage<'m>(&'m mut self, image_usage: ImageUsageFlags) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn image_sharing_mode<'m>(&'m mut self, image_sharing_mode: SharingMode) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn queue_family_index_count<'m>(&'m mut self, queue_family_index_count: u32) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn queue_family_indices<'m, 'a>(&'m mut self, queue_family_indices: &'a u32) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn pre_transform<'m>(&'m mut self, pre_transform: SurfaceTransformFlagsKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn composite_alpha<'m>(&'m mut self, composite_alpha: CompositeAlphaFlagsKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn present_mode<'m>(&'m mut self, present_mode: PresentModeKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn clipped<'m>(&'m mut self, clipped: bool) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn old_swapchain<'m>(&'m mut self, old_swapchain: SwapchainKhr) -> &'m mut SwapchainCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PresentInfoKhr<'s> {
    raw: vks::VkPresentInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PresentInfoKhr<'s> {
    pub fn builder<'b>() -> PresentInfoKhrBuilder<'b> {
        PresentInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn wait_semaphore_count(&self) {
    }

    pub fn wait_semaphores(&self) {
    }

    pub fn swapchain_count(&self) {
    }

    pub fn swapchains(&self) {
    }

    pub fn image_indices(&self) {
    }

    pub fn results(&self) {
    }

    pub fn raw(&self) -> &vks::VkPresentInfoKHR {
        &self.raw
    }
}


/// A builder for `VkPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentInfoKhrBuilder<'b> {
    raw: vks::VkPresentInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PresentInfoKhrBuilder<'b> {
    pub fn new() -> PresentInfoKhrBuilder<'b> {
        PresentInfoKhrBuilder {
            raw: vks::VkPresentInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

    pub fn wait_semaphore_count<'m>(&'m mut self, wait_semaphore_count: u32) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

    pub fn wait_semaphores<'m, 'a>(&'m mut self, wait_semaphores: &'a Semaphore) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

    pub fn swapchain_count<'m>(&'m mut self, swapchain_count: u32) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

    pub fn swapchains<'m, 'a>(&'m mut self, swapchains: &'a SwapchainKhr) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

    pub fn image_indices<'m, 'a>(&'m mut self, image_indices: &'a u32) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

    pub fn results<'m, 'a>(&'m mut self, results: &'a ResultEnum) -> &'m mut PresentInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkDebugReportCallbackCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoExt<'s> {
    raw: vks::VkDebugReportCallbackCreateInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DebugReportCallbackCreateInfoExt<'s> {
    pub fn builder<'b>() -> DebugReportCallbackCreateInfoExtBuilder<'b> {
        DebugReportCallbackCreateInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn pfn_callback(&self) {
    }

    pub fn user_data(&self) {
    }

    pub fn raw(&self) -> &vks::VkDebugReportCallbackCreateInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDebugReportCallbackCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugReportCallbackCreateInfoExtBuilder<'b> {
    raw: vks::VkDebugReportCallbackCreateInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DebugReportCallbackCreateInfoExtBuilder<'b> {
    pub fn new() -> DebugReportCallbackCreateInfoExtBuilder<'b> {
        DebugReportCallbackCreateInfoExtBuilder {
            raw: vks::VkDebugReportCallbackCreateInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DebugReportCallbackCreateInfoExtBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DebugReportFlagsExt) -> &'m mut DebugReportCallbackCreateInfoExtBuilder<'b> {
        self
    }

    pub fn pfn_callback<'m>(&'m mut self, pfn_callback: PFN_vkDebugReportCallbackEXT) -> &'m mut DebugReportCallbackCreateInfoExtBuilder<'b> {
        self
    }

    pub fn user_data<'m, 'a>(&'m mut self, user_data: &'a ()) -> &'m mut DebugReportCallbackCreateInfoExtBuilder<'b> {
        self
    }

}


/// A `VkValidationFlagsEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ValidationFlagsExt<'s> {
    raw: vks::VkValidationFlagsEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> ValidationFlagsExt<'s> {
    pub fn builder<'b>() -> ValidationFlagsExtBuilder<'b> {
        ValidationFlagsExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn disabled_validation_check_count(&self) {
    }

    pub fn disabled_validation_checks(&self) {
    }

    pub fn raw(&self) -> &vks::VkValidationFlagsEXT {
        &self.raw
    }
}


/// A builder for `VkValidationFlagsEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ValidationFlagsExtBuilder<'b> {
    raw: vks::VkValidationFlagsEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> ValidationFlagsExtBuilder<'b> {
    pub fn new() -> ValidationFlagsExtBuilder<'b> {
        ValidationFlagsExtBuilder {
            raw: vks::VkValidationFlagsEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ValidationFlagsExtBuilder<'b> {
        self
    }

    pub fn disabled_validation_check_count<'m>(&'m mut self, disabled_validation_check_count: u32) -> &'m mut ValidationFlagsExtBuilder<'b> {
        self
    }

    pub fn disabled_validation_checks<'m, 'a>(&'m mut self, disabled_validation_checks: &'a ValidationCheckExt) -> &'m mut ValidationFlagsExtBuilder<'b> {
        self
    }

}


/// A `VkPipelineRasterizationStateRasterizationOrderAMD`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAmd<'s> {
    raw: vks::VkPipelineRasterizationStateRasterizationOrderAMD,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineRasterizationStateRasterizationOrderAmd<'s> {
    pub fn builder<'b>() -> PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
        PipelineRasterizationStateRasterizationOrderAmdBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn rasterization_order(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineRasterizationStateRasterizationOrderAMD {
        &self.raw
    }
}


/// A builder for `VkPipelineRasterizationStateRasterizationOrderAMD`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
    raw: vks::VkPipelineRasterizationStateRasterizationOrderAMD,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
    pub fn new() -> PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
        PipelineRasterizationStateRasterizationOrderAmdBuilder {
            raw: vks::VkPipelineRasterizationStateRasterizationOrderAMD::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
        self
    }

    pub fn rasterization_order<'m>(&'m mut self, rasterization_order: RasterizationOrderAmd) -> &'m mut PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
        self
    }

}


/// A `VkDebugMarkerObjectNameInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoExt<'s> {
    raw: vks::VkDebugMarkerObjectNameInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DebugMarkerObjectNameInfoExt<'s> {
    pub fn builder<'b>() -> DebugMarkerObjectNameInfoExtBuilder<'b> {
        DebugMarkerObjectNameInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn object_type(&self) {
    }

    pub fn object(&self) {
    }

    pub fn object_name(&self) {
    }

    pub fn raw(&self) -> &vks::VkDebugMarkerObjectNameInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDebugMarkerObjectNameInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerObjectNameInfoExtBuilder<'b> {
    raw: vks::VkDebugMarkerObjectNameInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DebugMarkerObjectNameInfoExtBuilder<'b> {
    pub fn new() -> DebugMarkerObjectNameInfoExtBuilder<'b> {
        DebugMarkerObjectNameInfoExtBuilder {
            raw: vks::VkDebugMarkerObjectNameInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DebugMarkerObjectNameInfoExtBuilder<'b> {
        self
    }

    pub fn object_type<'m>(&'m mut self, object_type: DebugReportObjectTypeExt) -> &'m mut DebugMarkerObjectNameInfoExtBuilder<'b> {
        self
    }

    pub fn object<'m>(&'m mut self, object: u64) -> &'m mut DebugMarkerObjectNameInfoExtBuilder<'b> {
        self
    }

    pub fn object_name<'m, 'a>(&'m mut self, object_name: &'a i8) -> &'m mut DebugMarkerObjectNameInfoExtBuilder<'b> {
        self
    }

}


/// A `VkDebugMarkerObjectTagInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoExt<'s> {
    raw: vks::VkDebugMarkerObjectTagInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DebugMarkerObjectTagInfoExt<'s> {
    pub fn builder<'b>() -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        DebugMarkerObjectTagInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn object_type(&self) {
    }

    pub fn object(&self) {
    }

    pub fn tag_name(&self) {
    }

    pub fn tag_size(&self) {
    }

    pub fn tag(&self) {
    }

    pub fn raw(&self) -> &vks::VkDebugMarkerObjectTagInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDebugMarkerObjectTagInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerObjectTagInfoExtBuilder<'b> {
    raw: vks::VkDebugMarkerObjectTagInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DebugMarkerObjectTagInfoExtBuilder<'b> {
    pub fn new() -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        DebugMarkerObjectTagInfoExtBuilder {
            raw: vks::VkDebugMarkerObjectTagInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DebugMarkerObjectTagInfoExtBuilder<'b> {
        self
    }

    pub fn object_type<'m>(&'m mut self, object_type: DebugReportObjectTypeExt) -> &'m mut DebugMarkerObjectTagInfoExtBuilder<'b> {
        self
    }

    pub fn object<'m>(&'m mut self, object: u64) -> &'m mut DebugMarkerObjectTagInfoExtBuilder<'b> {
        self
    }

    pub fn tag_name<'m>(&'m mut self, tag_name: u64) -> &'m mut DebugMarkerObjectTagInfoExtBuilder<'b> {
        self
    }

    pub fn tag_size<'m>(&'m mut self, tag_size: usize) -> &'m mut DebugMarkerObjectTagInfoExtBuilder<'b> {
        self
    }

    pub fn tag<'m, 'a>(&'m mut self, tag: &'a ()) -> &'m mut DebugMarkerObjectTagInfoExtBuilder<'b> {
        self
    }

}


/// A `VkDebugMarkerMarkerInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DebugMarkerMarkerInfoExt<'s> {
    raw: vks::VkDebugMarkerMarkerInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DebugMarkerMarkerInfoExt<'s> {
    pub fn builder<'b>() -> DebugMarkerMarkerInfoExtBuilder<'b> {
        DebugMarkerMarkerInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn marker_name(&self) {
    }

    pub fn color(&self) {
    }

    pub fn raw(&self) -> &vks::VkDebugMarkerMarkerInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDebugMarkerMarkerInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerMarkerInfoExtBuilder<'b> {
    raw: vks::VkDebugMarkerMarkerInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DebugMarkerMarkerInfoExtBuilder<'b> {
    pub fn new() -> DebugMarkerMarkerInfoExtBuilder<'b> {
        DebugMarkerMarkerInfoExtBuilder {
            raw: vks::VkDebugMarkerMarkerInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DebugMarkerMarkerInfoExtBuilder<'b> {
        self
    }

    pub fn marker_name<'m, 'a>(&'m mut self, marker_name: &'a i8) -> &'m mut DebugMarkerMarkerInfoExtBuilder<'b> {
        self
    }

    pub fn color<'m>(&'m mut self, color: f32) -> &'m mut DebugMarkerMarkerInfoExtBuilder<'b> {
        self
    }

}


/// A `VkDedicatedAllocationImageCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNv<'s> {
    raw: vks::VkDedicatedAllocationImageCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> DedicatedAllocationImageCreateInfoNv<'s> {
    pub fn builder<'b>() -> DedicatedAllocationImageCreateInfoNvBuilder<'b> {
        DedicatedAllocationImageCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn dedicated_allocation(&self) {
    }

    pub fn raw(&self) -> &vks::VkDedicatedAllocationImageCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkDedicatedAllocationImageCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DedicatedAllocationImageCreateInfoNvBuilder<'b> {
    raw: vks::VkDedicatedAllocationImageCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> DedicatedAllocationImageCreateInfoNvBuilder<'b> {
    pub fn new() -> DedicatedAllocationImageCreateInfoNvBuilder<'b> {
        DedicatedAllocationImageCreateInfoNvBuilder {
            raw: vks::VkDedicatedAllocationImageCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DedicatedAllocationImageCreateInfoNvBuilder<'b> {
        self
    }

    pub fn dedicated_allocation<'m>(&'m mut self, dedicated_allocation: bool) -> &'m mut DedicatedAllocationImageCreateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkDedicatedAllocationBufferCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNv<'s> {
    raw: vks::VkDedicatedAllocationBufferCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> DedicatedAllocationBufferCreateInfoNv<'s> {
    pub fn builder<'b>() -> DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
        DedicatedAllocationBufferCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn dedicated_allocation(&self) {
    }

    pub fn raw(&self) -> &vks::VkDedicatedAllocationBufferCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkDedicatedAllocationBufferCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
    raw: vks::VkDedicatedAllocationBufferCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
    pub fn new() -> DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
        DedicatedAllocationBufferCreateInfoNvBuilder {
            raw: vks::VkDedicatedAllocationBufferCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
        self
    }

    pub fn dedicated_allocation<'m>(&'m mut self, dedicated_allocation: bool) -> &'m mut DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkDedicatedAllocationMemoryAllocateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNv<'s> {
    raw: vks::VkDedicatedAllocationMemoryAllocateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> DedicatedAllocationMemoryAllocateInfoNv<'s> {
    pub fn builder<'b>() -> DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        DedicatedAllocationMemoryAllocateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn image(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn raw(&self) -> &vks::VkDedicatedAllocationMemoryAllocateInfoNV {
        &self.raw
    }
}


/// A builder for `VkDedicatedAllocationMemoryAllocateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
    raw: vks::VkDedicatedAllocationMemoryAllocateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
    pub fn new() -> DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        DedicatedAllocationMemoryAllocateInfoNvBuilder {
            raw: vks::VkDedicatedAllocationMemoryAllocateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        self
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkExternalImageFormatPropertiesNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalImageFormatPropertiesNv {
    raw: vks::VkExternalImageFormatPropertiesNV,
}

impl ExternalImageFormatPropertiesNv {
    pub fn image_format_properties(&self) {
    }

    pub fn external_memory_features(&self) {
    }

    pub fn export_from_imported_handle_types(&self) {
    }

    pub fn compatible_handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalImageFormatPropertiesNV {
        &self.raw
    }
}


/// A `VkExternalMemoryImageCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNv<'s> {
    raw: vks::VkExternalMemoryImageCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalMemoryImageCreateInfoNv<'s> {
    pub fn builder<'b>() -> ExternalMemoryImageCreateInfoNvBuilder<'b> {
        ExternalMemoryImageCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalMemoryImageCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkExternalMemoryImageCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExternalMemoryImageCreateInfoNvBuilder<'b> {
    raw: vks::VkExternalMemoryImageCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExternalMemoryImageCreateInfoNvBuilder<'b> {
    pub fn new() -> ExternalMemoryImageCreateInfoNvBuilder<'b> {
        ExternalMemoryImageCreateInfoNvBuilder {
            raw: vks::VkExternalMemoryImageCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExternalMemoryImageCreateInfoNvBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalMemoryHandleTypeFlagsNv) -> &'m mut ExternalMemoryImageCreateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkExportMemoryAllocateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNv<'s> {
    raw: vks::VkExportMemoryAllocateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportMemoryAllocateInfoNv<'s> {
    pub fn builder<'b>() -> ExportMemoryAllocateInfoNvBuilder<'b> {
        ExportMemoryAllocateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportMemoryAllocateInfoNV {
        &self.raw
    }
}


/// A builder for `VkExportMemoryAllocateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportMemoryAllocateInfoNvBuilder<'b> {
    raw: vks::VkExportMemoryAllocateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportMemoryAllocateInfoNvBuilder<'b> {
    pub fn new() -> ExportMemoryAllocateInfoNvBuilder<'b> {
        ExportMemoryAllocateInfoNvBuilder {
            raw: vks::VkExportMemoryAllocateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportMemoryAllocateInfoNvBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalMemoryHandleTypeFlagsNv) -> &'m mut ExportMemoryAllocateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkImportMemoryWin32HandleInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNv<'s> {
    raw: vks::VkImportMemoryWin32HandleInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportMemoryWin32HandleInfoNv<'s> {
    pub fn builder<'b>() -> ImportMemoryWin32HandleInfoNvBuilder<'b> {
        ImportMemoryWin32HandleInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn handle(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportMemoryWin32HandleInfoNV {
        &self.raw
    }
}


/// A builder for `VkImportMemoryWin32HandleInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportMemoryWin32HandleInfoNvBuilder<'b> {
    raw: vks::VkImportMemoryWin32HandleInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportMemoryWin32HandleInfoNvBuilder<'b> {
    pub fn new() -> ImportMemoryWin32HandleInfoNvBuilder<'b> {
        ImportMemoryWin32HandleInfoNvBuilder {
            raw: vks::VkImportMemoryWin32HandleInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportMemoryWin32HandleInfoNvBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsNv) -> &'m mut ImportMemoryWin32HandleInfoNvBuilder<'b> {
        self
    }

    pub fn handle<'m>(&'m mut self, handle: HANDLE) -> &'m mut ImportMemoryWin32HandleInfoNvBuilder<'b> {
        self
    }

}


/// A `VkExportMemoryWin32HandleInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNv<'s> {
    raw: vks::VkExportMemoryWin32HandleInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportMemoryWin32HandleInfoNv<'s> {
    pub fn builder<'b>() -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        ExportMemoryWin32HandleInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn attributes(&self) {
    }

    pub fn dw_access(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportMemoryWin32HandleInfoNV {
        &self.raw
    }
}


/// A builder for `VkExportMemoryWin32HandleInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportMemoryWin32HandleInfoNvBuilder<'b> {
    raw: vks::VkExportMemoryWin32HandleInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportMemoryWin32HandleInfoNvBuilder<'b> {
    pub fn new() -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        ExportMemoryWin32HandleInfoNvBuilder {
            raw: vks::VkExportMemoryWin32HandleInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self
    }

    pub fn attributes<'m, 'a>(&'m mut self, attributes: &'a SECURITY_ATTRIBUTES) -> &'m mut ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self
    }

    pub fn dw_access<'m>(&'m mut self, dw_access: DWORD) -> &'m mut ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self
    }

}


/// A `VkWin32KeyedMutexAcquireReleaseInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNv<'s> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> Win32KeyedMutexAcquireReleaseInfoNv<'s> {
    pub fn builder<'b>() -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn acquire_count(&self) {
    }

    pub fn acquire_syncs(&self) {
    }

    pub fn acquire_keys(&self) {
    }

    pub fn acquire_timeout_milliseconds(&self) {
    }

    pub fn release_count(&self) {
    }

    pub fn release_syncs(&self) {
    }

    pub fn release_keys(&self) {
    }

    pub fn raw(&self) -> &vks::VkWin32KeyedMutexAcquireReleaseInfoNV {
        &self.raw
    }
}


/// A builder for `VkWin32KeyedMutexAcquireReleaseInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
    pub fn new() -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoNvBuilder {
            raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn acquire_count<'m>(&'m mut self, acquire_count: u32) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn acquire_syncs<'m, 'a>(&'m mut self, acquire_syncs: &'a DeviceMemory) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn acquire_keys<'m, 'a>(&'m mut self, acquire_keys: &'a u64) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn acquire_timeout_milliseconds<'m, 'a>(&'m mut self, acquire_timeout_milliseconds: &'a u32) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn release_count<'m>(&'m mut self, release_count: u32) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn release_syncs<'m, 'a>(&'m mut self, release_syncs: &'a DeviceMemory) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

    pub fn release_keys<'m, 'a>(&'m mut self, release_keys: &'a u64) -> &'m mut Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self
    }

}


/// A `VkDeviceGeneratedCommandsFeaturesNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGeneratedCommandsFeaturesNvx<'s> {
    raw: vks::VkDeviceGeneratedCommandsFeaturesNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGeneratedCommandsFeaturesNvx<'s> {
    pub fn builder<'b>() -> DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
        DeviceGeneratedCommandsFeaturesNvxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn compute_binding_point_support(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGeneratedCommandsFeaturesNVX {
        &self.raw
    }
}


/// A builder for `VkDeviceGeneratedCommandsFeaturesNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
    raw: vks::VkDeviceGeneratedCommandsFeaturesNVX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
    pub fn new() -> DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
        DeviceGeneratedCommandsFeaturesNvxBuilder {
            raw: vks::VkDeviceGeneratedCommandsFeaturesNVX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
        self
    }

    pub fn compute_binding_point_support<'m>(&'m mut self, compute_binding_point_support: bool) -> &'m mut DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGeneratedCommandsLimitsNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGeneratedCommandsLimitsNvx<'s> {
    raw: vks::VkDeviceGeneratedCommandsLimitsNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGeneratedCommandsLimitsNvx<'s> {
    pub fn builder<'b>() -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        DeviceGeneratedCommandsLimitsNvxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn max_indirect_commands_layout_token_count(&self) {
    }

    pub fn max_object_entry_counts(&self) {
    }

    pub fn min_sequence_count_buffer_offset_alignment(&self) {
    }

    pub fn min_sequence_index_buffer_offset_alignment(&self) {
    }

    pub fn min_commands_token_buffer_offset_alignment(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGeneratedCommandsLimitsNVX {
        &self.raw
    }
}


/// A builder for `VkDeviceGeneratedCommandsLimitsNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
    raw: vks::VkDeviceGeneratedCommandsLimitsNVX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
    pub fn new() -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        DeviceGeneratedCommandsLimitsNvxBuilder {
            raw: vks::VkDeviceGeneratedCommandsLimitsNVX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self
    }

    pub fn max_indirect_commands_layout_token_count<'m>(&'m mut self, max_indirect_commands_layout_token_count: u32) -> &'m mut DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self
    }

    pub fn max_object_entry_counts<'m>(&'m mut self, max_object_entry_counts: u32) -> &'m mut DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self
    }

    pub fn min_sequence_count_buffer_offset_alignment<'m>(&'m mut self, min_sequence_count_buffer_offset_alignment: u32) -> &'m mut DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self
    }

    pub fn min_sequence_index_buffer_offset_alignment<'m>(&'m mut self, min_sequence_index_buffer_offset_alignment: u32) -> &'m mut DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self
    }

    pub fn min_commands_token_buffer_offset_alignment<'m>(&'m mut self, min_commands_token_buffer_offset_alignment: u32) -> &'m mut DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self
    }

}


/// A `VkIndirectCommandsTokenNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct IndirectCommandsTokenNvx {
    raw: vks::VkIndirectCommandsTokenNVX,
}

#[cfg(feature = "experimental")]
impl IndirectCommandsTokenNvx {
    pub fn builder() -> IndirectCommandsTokenNvxBuilder {
        IndirectCommandsTokenNvxBuilder::new()
    }

    pub fn token_type(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn raw(&self) -> &vks::VkIndirectCommandsTokenNVX {
        &self.raw
    }
}


/// A builder for `VkIndirectCommandsTokenNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct IndirectCommandsTokenNvxBuilder {
    raw: vks::VkIndirectCommandsTokenNVX,
}

#[cfg(feature = "experimental")]
impl IndirectCommandsTokenNvxBuilder {
    pub fn new() -> IndirectCommandsTokenNvxBuilder {
        IndirectCommandsTokenNvxBuilder {
            raw: vks::VkIndirectCommandsTokenNVX::default(),
        }
    }

    pub fn token_type<'m>(&'m mut self, token_type: IndirectCommandsTokenTypeNvx) -> &'m mut IndirectCommandsTokenNvxBuilder {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut IndirectCommandsTokenNvxBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: u64) -> &'m mut IndirectCommandsTokenNvxBuilder {
        self
    }

}


/// A `VkIndirectCommandsLayoutTokenNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNvx {
    raw: vks::VkIndirectCommandsLayoutTokenNVX,
}

#[cfg(feature = "experimental")]
impl IndirectCommandsLayoutTokenNvx {
    pub fn builder() -> IndirectCommandsLayoutTokenNvxBuilder {
        IndirectCommandsLayoutTokenNvxBuilder::new()
    }

    pub fn token_type(&self) {
    }

    pub fn binding_unit(&self) {
    }

    pub fn dynamic_count(&self) {
    }

    pub fn divisor(&self) {
    }

    pub fn raw(&self) -> &vks::VkIndirectCommandsLayoutTokenNVX {
        &self.raw
    }
}


/// A builder for `VkIndirectCommandsLayoutTokenNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct IndirectCommandsLayoutTokenNvxBuilder {
    raw: vks::VkIndirectCommandsLayoutTokenNVX,
}

#[cfg(feature = "experimental")]
impl IndirectCommandsLayoutTokenNvxBuilder {
    pub fn new() -> IndirectCommandsLayoutTokenNvxBuilder {
        IndirectCommandsLayoutTokenNvxBuilder {
            raw: vks::VkIndirectCommandsLayoutTokenNVX::default(),
        }
    }

    pub fn token_type<'m>(&'m mut self, token_type: IndirectCommandsTokenTypeNvx) -> &'m mut IndirectCommandsLayoutTokenNvxBuilder {
        self
    }

    pub fn binding_unit<'m>(&'m mut self, binding_unit: u32) -> &'m mut IndirectCommandsLayoutTokenNvxBuilder {
        self
    }

    pub fn dynamic_count<'m>(&'m mut self, dynamic_count: u32) -> &'m mut IndirectCommandsLayoutTokenNvxBuilder {
        self
    }

    pub fn divisor<'m>(&'m mut self, divisor: u32) -> &'m mut IndirectCommandsLayoutTokenNvxBuilder {
        self
    }

}


/// A `VkIndirectCommandsLayoutCreateInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNvx<'s> {
    raw: vks::VkIndirectCommandsLayoutCreateInfoNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> IndirectCommandsLayoutCreateInfoNvx<'s> {
    pub fn builder<'b>() -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        IndirectCommandsLayoutCreateInfoNvxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn pipeline_bind_point(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn token_count(&self) {
    }

    pub fn tokens(&self) {
    }

    pub fn raw(&self) -> &vks::VkIndirectCommandsLayoutCreateInfoNVX {
        &self.raw
    }
}


/// A builder for `VkIndirectCommandsLayoutCreateInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
    raw: vks::VkIndirectCommandsLayoutCreateInfoNVX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
    pub fn new() -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        IndirectCommandsLayoutCreateInfoNvxBuilder {
            raw: vks::VkIndirectCommandsLayoutCreateInfoNVX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn pipeline_bind_point<'m>(&'m mut self, pipeline_bind_point: PipelineBindPoint) -> &'m mut IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: IndirectCommandsLayoutUsageFlagsNvx) -> &'m mut IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn token_count<'m>(&'m mut self, token_count: u32) -> &'m mut IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn tokens<'m, 'a>(&'m mut self, tokens: &'a IndirectCommandsLayoutTokenNvx) -> &'m mut IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self
    }

}


/// A `VkCmdProcessCommandsInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CmdProcessCommandsInfoNvx<'s> {
    raw: vks::VkCmdProcessCommandsInfoNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> CmdProcessCommandsInfoNvx<'s> {
    pub fn builder<'b>() -> CmdProcessCommandsInfoNvxBuilder<'b> {
        CmdProcessCommandsInfoNvxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn object_table(&self) {
    }

    pub fn indirect_commands_layout(&self) {
    }

    pub fn indirect_commands_token_count(&self) {
    }

    pub fn indirect_commands_tokens(&self) {
    }

    pub fn max_sequences_count(&self) {
    }

    pub fn target_command_buffer(&self) {
    }

    pub fn sequences_count_buffer(&self) {
    }

    pub fn sequences_count_offset(&self) {
    }

    pub fn sequences_index_buffer(&self) {
    }

    pub fn sequences_index_offset(&self) {
    }

    pub fn raw(&self) -> &vks::VkCmdProcessCommandsInfoNVX {
        &self.raw
    }
}


/// A builder for `VkCmdProcessCommandsInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct CmdProcessCommandsInfoNvxBuilder<'b> {
    raw: vks::VkCmdProcessCommandsInfoNVX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> CmdProcessCommandsInfoNvxBuilder<'b> {
    pub fn new() -> CmdProcessCommandsInfoNvxBuilder<'b> {
        CmdProcessCommandsInfoNvxBuilder {
            raw: vks::VkCmdProcessCommandsInfoNVX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn object_table<'m>(&'m mut self, object_table: ObjectTableNvx) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn indirect_commands_layout<'m>(&'m mut self, indirect_commands_layout: IndirectCommandsLayoutNvx) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn indirect_commands_token_count<'m>(&'m mut self, indirect_commands_token_count: u32) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn indirect_commands_tokens<'m, 'a>(&'m mut self, indirect_commands_tokens: &'a IndirectCommandsTokenNvx) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn max_sequences_count<'m>(&'m mut self, max_sequences_count: u32) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn target_command_buffer<'m>(&'m mut self, target_command_buffer: CommandBuffer) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn sequences_count_buffer<'m>(&'m mut self, sequences_count_buffer: Buffer) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn sequences_count_offset<'m>(&'m mut self, sequences_count_offset: u64) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn sequences_index_buffer<'m>(&'m mut self, sequences_index_buffer: Buffer) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn sequences_index_offset<'m>(&'m mut self, sequences_index_offset: u64) -> &'m mut CmdProcessCommandsInfoNvxBuilder<'b> {
        self
    }

}


/// A `VkCmdReserveSpaceForCommandsInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct CmdReserveSpaceForCommandsInfoNvx<'s> {
    raw: vks::VkCmdReserveSpaceForCommandsInfoNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> CmdReserveSpaceForCommandsInfoNvx<'s> {
    pub fn builder<'b>() -> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        CmdReserveSpaceForCommandsInfoNvxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn object_table(&self) {
    }

    pub fn indirect_commands_layout(&self) {
    }

    pub fn max_sequences_count(&self) {
    }

    pub fn raw(&self) -> &vks::VkCmdReserveSpaceForCommandsInfoNVX {
        &self.raw
    }
}


/// A builder for `VkCmdReserveSpaceForCommandsInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
    raw: vks::VkCmdReserveSpaceForCommandsInfoNVX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
    pub fn new() -> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        CmdReserveSpaceForCommandsInfoNvxBuilder {
            raw: vks::VkCmdReserveSpaceForCommandsInfoNVX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn object_table<'m>(&'m mut self, object_table: ObjectTableNvx) -> &'m mut CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn indirect_commands_layout<'m>(&'m mut self, indirect_commands_layout: IndirectCommandsLayoutNvx) -> &'m mut CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self
    }

    pub fn max_sequences_count<'m>(&'m mut self, max_sequences_count: u32) -> &'m mut CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self
    }

}


/// A `VkObjectTableCreateInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTableCreateInfoNvx<'s> {
    raw: vks::VkObjectTableCreateInfoNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> ObjectTableCreateInfoNvx<'s> {
    pub fn builder<'b>() -> ObjectTableCreateInfoNvxBuilder<'b> {
        ObjectTableCreateInfoNvxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn object_count(&self) {
    }

    pub fn object_entry_types(&self) {
    }

    pub fn object_entry_counts(&self) {
    }

    pub fn object_entry_usage_flags(&self) {
    }

    pub fn max_uniform_buffers_per_descriptor(&self) {
    }

    pub fn max_storage_buffers_per_descriptor(&self) {
    }

    pub fn max_storage_images_per_descriptor(&self) {
    }

    pub fn max_sampled_images_per_descriptor(&self) {
    }

    pub fn max_pipeline_layouts(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTableCreateInfoNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTableCreateInfoNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTableCreateInfoNvxBuilder<'b> {
    raw: vks::VkObjectTableCreateInfoNVX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> ObjectTableCreateInfoNvxBuilder<'b> {
    pub fn new() -> ObjectTableCreateInfoNvxBuilder<'b> {
        ObjectTableCreateInfoNvxBuilder {
            raw: vks::VkObjectTableCreateInfoNVX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn object_count<'m>(&'m mut self, object_count: u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn object_entry_types<'m, 'a>(&'m mut self, object_entry_types: &'a ObjectEntryTypeNvx) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn object_entry_counts<'m, 'a>(&'m mut self, object_entry_counts: &'a u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn object_entry_usage_flags<'m, 'a>(&'m mut self, object_entry_usage_flags: &'a ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn max_uniform_buffers_per_descriptor<'m>(&'m mut self, max_uniform_buffers_per_descriptor: u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn max_storage_buffers_per_descriptor<'m>(&'m mut self, max_storage_buffers_per_descriptor: u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn max_storage_images_per_descriptor<'m>(&'m mut self, max_storage_images_per_descriptor: u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn max_sampled_images_per_descriptor<'m>(&'m mut self, max_sampled_images_per_descriptor: u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

    pub fn max_pipeline_layouts<'m>(&'m mut self, max_pipeline_layouts: u32) -> &'m mut ObjectTableCreateInfoNvxBuilder<'b> {
        self
    }

}


/// A `VkObjectTableEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTableEntryNvx {
    raw: vks::VkObjectTableEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableEntryNvx {
    pub fn builder() -> ObjectTableEntryNvxBuilder {
        ObjectTableEntryNvxBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTableEntryNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTableEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTableEntryNvxBuilder {
    raw: vks::VkObjectTableEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableEntryNvxBuilder {
    pub fn new() -> ObjectTableEntryNvxBuilder {
        ObjectTableEntryNvxBuilder {
            raw: vks::VkObjectTableEntryNVX::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: ObjectEntryTypeNvx) -> &'m mut ObjectTableEntryNvxBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTableEntryNvxBuilder {
        self
    }

}


/// A `VkObjectTablePipelineEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTablePipelineEntryNvx {
    raw: vks::VkObjectTablePipelineEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTablePipelineEntryNvx {
    pub fn builder() -> ObjectTablePipelineEntryNvxBuilder {
        ObjectTablePipelineEntryNvxBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn pipeline(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTablePipelineEntryNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTablePipelineEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTablePipelineEntryNvxBuilder {
    raw: vks::VkObjectTablePipelineEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTablePipelineEntryNvxBuilder {
    pub fn new() -> ObjectTablePipelineEntryNvxBuilder {
        ObjectTablePipelineEntryNvxBuilder {
            raw: vks::VkObjectTablePipelineEntryNVX::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: ObjectEntryTypeNvx) -> &'m mut ObjectTablePipelineEntryNvxBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTablePipelineEntryNvxBuilder {
        self
    }

    pub fn pipeline<'m>(&'m mut self, pipeline: Pipeline) -> &'m mut ObjectTablePipelineEntryNvxBuilder {
        self
    }

}


/// A `VkObjectTableDescriptorSetEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTableDescriptorSetEntryNvx {
    raw: vks::VkObjectTableDescriptorSetEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableDescriptorSetEntryNvx {
    pub fn builder() -> ObjectTableDescriptorSetEntryNvxBuilder {
        ObjectTableDescriptorSetEntryNvxBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn pipeline_layout(&self) {
    }

    pub fn descriptor_set(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTableDescriptorSetEntryNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTableDescriptorSetEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTableDescriptorSetEntryNvxBuilder {
    raw: vks::VkObjectTableDescriptorSetEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableDescriptorSetEntryNvxBuilder {
    pub fn new() -> ObjectTableDescriptorSetEntryNvxBuilder {
        ObjectTableDescriptorSetEntryNvxBuilder {
            raw: vks::VkObjectTableDescriptorSetEntryNVX::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: ObjectEntryTypeNvx) -> &'m mut ObjectTableDescriptorSetEntryNvxBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTableDescriptorSetEntryNvxBuilder {
        self
    }

    pub fn pipeline_layout<'m>(&'m mut self, pipeline_layout: PipelineLayout) -> &'m mut ObjectTableDescriptorSetEntryNvxBuilder {
        self
    }

    pub fn descriptor_set<'m>(&'m mut self, descriptor_set: DescriptorSet) -> &'m mut ObjectTableDescriptorSetEntryNvxBuilder {
        self
    }

}


/// A `VkObjectTableVertexBufferEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTableVertexBufferEntryNvx {
    raw: vks::VkObjectTableVertexBufferEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableVertexBufferEntryNvx {
    pub fn builder() -> ObjectTableVertexBufferEntryNvxBuilder {
        ObjectTableVertexBufferEntryNvxBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTableVertexBufferEntryNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTableVertexBufferEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTableVertexBufferEntryNvxBuilder {
    raw: vks::VkObjectTableVertexBufferEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableVertexBufferEntryNvxBuilder {
    pub fn new() -> ObjectTableVertexBufferEntryNvxBuilder {
        ObjectTableVertexBufferEntryNvxBuilder {
            raw: vks::VkObjectTableVertexBufferEntryNVX::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: ObjectEntryTypeNvx) -> &'m mut ObjectTableVertexBufferEntryNvxBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTableVertexBufferEntryNvxBuilder {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut ObjectTableVertexBufferEntryNvxBuilder {
        self
    }

}


/// A `VkObjectTableIndexBufferEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTableIndexBufferEntryNvx {
    raw: vks::VkObjectTableIndexBufferEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableIndexBufferEntryNvx {
    pub fn builder() -> ObjectTableIndexBufferEntryNvxBuilder {
        ObjectTableIndexBufferEntryNvxBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn index_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTableIndexBufferEntryNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTableIndexBufferEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTableIndexBufferEntryNvxBuilder {
    raw: vks::VkObjectTableIndexBufferEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTableIndexBufferEntryNvxBuilder {
    pub fn new() -> ObjectTableIndexBufferEntryNvxBuilder {
        ObjectTableIndexBufferEntryNvxBuilder {
            raw: vks::VkObjectTableIndexBufferEntryNVX::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: ObjectEntryTypeNvx) -> &'m mut ObjectTableIndexBufferEntryNvxBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTableIndexBufferEntryNvxBuilder {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut ObjectTableIndexBufferEntryNvxBuilder {
        self
    }

    pub fn index_type<'m>(&'m mut self, index_type: IndexType) -> &'m mut ObjectTableIndexBufferEntryNvxBuilder {
        self
    }

}


/// A `VkObjectTablePushConstantEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ObjectTablePushConstantEntryNvx {
    raw: vks::VkObjectTablePushConstantEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTablePushConstantEntryNvx {
    pub fn builder() -> ObjectTablePushConstantEntryNvxBuilder {
        ObjectTablePushConstantEntryNvxBuilder::new()
    }

    pub fn type_of(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn pipeline_layout(&self) {
    }

    pub fn stage_flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkObjectTablePushConstantEntryNVX {
        &self.raw
    }
}


/// A builder for `VkObjectTablePushConstantEntryNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ObjectTablePushConstantEntryNvxBuilder {
    raw: vks::VkObjectTablePushConstantEntryNVX,
}

#[cfg(feature = "experimental")]
impl ObjectTablePushConstantEntryNvxBuilder {
    pub fn new() -> ObjectTablePushConstantEntryNvxBuilder {
        ObjectTablePushConstantEntryNvxBuilder {
            raw: vks::VkObjectTablePushConstantEntryNVX::default(),
        }
    }

    pub fn type_of<'m>(&'m mut self, type_of: ObjectEntryTypeNvx) -> &'m mut ObjectTablePushConstantEntryNvxBuilder {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ObjectEntryUsageFlagsNvx) -> &'m mut ObjectTablePushConstantEntryNvxBuilder {
        self
    }

    pub fn pipeline_layout<'m>(&'m mut self, pipeline_layout: PipelineLayout) -> &'m mut ObjectTablePushConstantEntryNvxBuilder {
        self
    }

    pub fn stage_flags<'m>(&'m mut self, stage_flags: ShaderStageFlags) -> &'m mut ObjectTablePushConstantEntryNvxBuilder {
        self
    }

}


/// A `VkPhysicalDeviceFeatures2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2Khr<'s> {
    raw: vks::VkPhysicalDeviceFeatures2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceFeatures2Khr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceFeatures2KhrBuilder<'b> {
        PhysicalDeviceFeatures2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn features(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceFeatures2KHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceFeatures2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceFeatures2KhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceFeatures2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceFeatures2KhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceFeatures2KhrBuilder<'b> {
        PhysicalDeviceFeatures2KhrBuilder {
            raw: vks::VkPhysicalDeviceFeatures2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceFeatures2KhrBuilder<'b> {
        self
    }

    pub fn features<'m>(&'m mut self, features: PhysicalDeviceFeatures) -> &'m mut PhysicalDeviceFeatures2KhrBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceProperties2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceProperties2Khr<'s> {
    raw: vks::VkPhysicalDeviceProperties2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceProperties2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceProperties2KHR {
        &self.raw
    }
}


/// A `VkFormatProperties2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FormatProperties2Khr<'s> {
    raw: vks::VkFormatProperties2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> FormatProperties2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn format_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkFormatProperties2KHR {
        &self.raw
    }
}


/// A `VkImageFormatProperties2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageFormatProperties2Khr<'s> {
    raw: vks::VkImageFormatProperties2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImageFormatProperties2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn image_format_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageFormatProperties2KHR {
        &self.raw
    }
}


/// A `VkPhysicalDeviceImageFormatInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2Khr<'s> {
    raw: vks::VkPhysicalDeviceImageFormatInfo2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceImageFormatInfo2Khr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        PhysicalDeviceImageFormatInfo2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn format(&self) {
    }

    pub fn type_of(&self) {
    }

    pub fn tiling(&self) {
    }

    pub fn usage(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceImageFormatInfo2KHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceImageFormatInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceImageFormatInfo2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        PhysicalDeviceImageFormatInfo2KhrBuilder {
            raw: vks::VkPhysicalDeviceImageFormatInfo2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn type_of<'m>(&'m mut self, type_of: ImageType) -> &'m mut PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn tiling<'m>(&'m mut self, tiling: ImageTiling) -> &'m mut PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn usage<'m>(&'m mut self, usage: ImageUsageFlags) -> &'m mut PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: ImageCreateFlags) -> &'m mut PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self
    }

}


/// A `VkQueueFamilyProperties2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct QueueFamilyProperties2Khr<'s> {
    raw: vks::VkQueueFamilyProperties2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> QueueFamilyProperties2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn queue_family_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkQueueFamilyProperties2KHR {
        &self.raw
    }
}


/// A `VkPhysicalDeviceMemoryProperties2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2Khr<'s> {
    raw: vks::VkPhysicalDeviceMemoryProperties2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceMemoryProperties2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn memory_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceMemoryProperties2KHR {
        &self.raw
    }
}


/// A `VkSparseImageFormatProperties2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageFormatProperties2Khr<'s> {
    raw: vks::VkSparseImageFormatProperties2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SparseImageFormatProperties2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageFormatProperties2KHR {
        &self.raw
    }
}


/// A `VkPhysicalDeviceSparseImageFormatInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2Khr<'s> {
    raw: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceSparseImageFormatInfo2Khr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        PhysicalDeviceSparseImageFormatInfo2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn format(&self) {
    }

    pub fn type_of(&self) {
    }

    pub fn samples(&self) {
    }

    pub fn usage(&self) {
    }

    pub fn tiling(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceSparseImageFormatInfo2KHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceSparseImageFormatInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        PhysicalDeviceSparseImageFormatInfo2KhrBuilder {
            raw: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn format<'m>(&'m mut self, format: Format) -> &'m mut PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn type_of<'m>(&'m mut self, type_of: ImageType) -> &'m mut PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn samples<'m>(&'m mut self, samples: SampleCountFlags) -> &'m mut PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn usage<'m>(&'m mut self, usage: ImageUsageFlags) -> &'m mut PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self
    }

    pub fn tiling<'m>(&'m mut self, tiling: ImageTiling) -> &'m mut PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDevicePushDescriptorPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKhr<'s> {
    raw: vks::VkPhysicalDevicePushDescriptorPropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDevicePushDescriptorPropertiesKhr<'s> {
    pub fn builder<'b>() -> PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
        PhysicalDevicePushDescriptorPropertiesKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn max_push_descriptors(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDevicePushDescriptorPropertiesKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDevicePushDescriptorPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
    raw: vks::VkPhysicalDevicePushDescriptorPropertiesKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
    pub fn new() -> PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
        PhysicalDevicePushDescriptorPropertiesKhrBuilder {
            raw: vks::VkPhysicalDevicePushDescriptorPropertiesKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
        self
    }

    pub fn max_push_descriptors<'m>(&'m mut self, max_push_descriptors: u32) -> &'m mut PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
        self
    }

}


/// A `VkPresentRegionsKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PresentRegionsKhr<'s> {
    raw: vks::VkPresentRegionsKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PresentRegionsKhr<'s> {
    pub fn builder<'b>() -> PresentRegionsKhrBuilder<'b> {
        PresentRegionsKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn swapchain_count(&self) {
    }

    pub fn regions(&self) {
    }

    pub fn raw(&self) -> &vks::VkPresentRegionsKHR {
        &self.raw
    }
}


/// A builder for `VkPresentRegionsKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentRegionsKhrBuilder<'b> {
    raw: vks::VkPresentRegionsKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PresentRegionsKhrBuilder<'b> {
    pub fn new() -> PresentRegionsKhrBuilder<'b> {
        PresentRegionsKhrBuilder {
            raw: vks::VkPresentRegionsKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PresentRegionsKhrBuilder<'b> {
        self
    }

    pub fn swapchain_count<'m>(&'m mut self, swapchain_count: u32) -> &'m mut PresentRegionsKhrBuilder<'b> {
        self
    }

    pub fn regions<'m, 'a>(&'m mut self, regions: &'a PresentRegionKhr) -> &'m mut PresentRegionsKhrBuilder<'b> {
        self
    }

}


/// A `VkPresentRegionKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PresentRegionKhr<'s> {
    raw: vks::VkPresentRegionKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PresentRegionKhr<'s> {
    pub fn builder<'b>() -> PresentRegionKhrBuilder<'b> {
        PresentRegionKhrBuilder::new()
    }

    pub fn rectangle_count(&self) {
    }

    pub fn rectangles(&self) {
    }

    pub fn raw(&self) -> &vks::VkPresentRegionKHR {
        &self.raw
    }
}


/// A builder for `VkPresentRegionKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentRegionKhrBuilder<'b> {
    raw: vks::VkPresentRegionKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PresentRegionKhrBuilder<'b> {
    pub fn new() -> PresentRegionKhrBuilder<'b> {
        PresentRegionKhrBuilder {
            raw: vks::VkPresentRegionKHR::default(),
            _p: PhantomData,
        }
    }

    pub fn rectangle_count<'m>(&'m mut self, rectangle_count: u32) -> &'m mut PresentRegionKhrBuilder<'b> {
        self
    }

    pub fn rectangles<'m, 'a>(&'m mut self, rectangles: &'a RectLayerKhr) -> &'m mut PresentRegionKhrBuilder<'b> {
        self
    }

}


/// A `VkRectLayerKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RectLayerKhr {
    raw: vks::VkRectLayerKHR,
}

impl RectLayerKhr {
    pub fn builder() -> RectLayerKhrBuilder {
        RectLayerKhrBuilder::new()
    }

    pub fn offset(&self) {
    }

    pub fn extent(&self) {
    }

    pub fn layer(&self) {
    }

    pub fn raw(&self) -> &vks::VkRectLayerKHR {
        &self.raw
    }
}


/// A builder for `VkRectLayerKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct RectLayerKhrBuilder {
    raw: vks::VkRectLayerKHR,
}

impl RectLayerKhrBuilder {
    pub fn new() -> RectLayerKhrBuilder {
        RectLayerKhrBuilder {
            raw: vks::VkRectLayerKHR::default(),
        }
    }

    pub fn offset<'m>(&'m mut self, offset: Offset2d) -> &'m mut RectLayerKhrBuilder {
        self
    }

    pub fn extent<'m>(&'m mut self, extent: Extent2d) -> &'m mut RectLayerKhrBuilder {
        self
    }

    pub fn layer<'m>(&'m mut self, layer: u32) -> &'m mut RectLayerKhrBuilder {
        self
    }

}


/// A `VkPhysicalDeviceVariablePointerFeaturesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointerFeaturesKhr<'s> {
    raw: vks::VkPhysicalDeviceVariablePointerFeaturesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceVariablePointerFeaturesKhr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        PhysicalDeviceVariablePointerFeaturesKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn variable_pointers_storage_buffer(&self) {
    }

    pub fn variable_pointers(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceVariablePointerFeaturesKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceVariablePointerFeaturesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceVariablePointerFeaturesKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        PhysicalDeviceVariablePointerFeaturesKhrBuilder {
            raw: vks::VkPhysicalDeviceVariablePointerFeaturesKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        self
    }

    pub fn variable_pointers_storage_buffer<'m>(&'m mut self, variable_pointers_storage_buffer: bool) -> &'m mut PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        self
    }

    pub fn variable_pointers<'m>(&'m mut self, variable_pointers: bool) -> &'m mut PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        self
    }

}


/// A `VkExternalMemoryPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalMemoryPropertiesKhr {
    raw: vks::VkExternalMemoryPropertiesKHR,
}

impl ExternalMemoryPropertiesKhr {
    pub fn external_memory_features(&self) {
    }

    pub fn export_from_imported_handle_types(&self) {
    }

    pub fn compatible_handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalMemoryPropertiesKHR {
        &self.raw
    }
}


/// A `VkPhysicalDeviceExternalImageFormatInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfoKhr<'s> {
    raw: vks::VkPhysicalDeviceExternalImageFormatInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceExternalImageFormatInfoKhr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
        PhysicalDeviceExternalImageFormatInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceExternalImageFormatInfoKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceExternalImageFormatInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceExternalImageFormatInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
        PhysicalDeviceExternalImageFormatInfoKhrBuilder {
            raw: vks::VkPhysicalDeviceExternalImageFormatInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExternalImageFormatPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalImageFormatPropertiesKhr<'s> {
    raw: vks::VkExternalImageFormatPropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalImageFormatPropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn external_memory_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalImageFormatPropertiesKHR {
        &self.raw
    }
}


/// A `VkPhysicalDeviceExternalBufferInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfoKhr<'s> {
    raw: vks::VkPhysicalDeviceExternalBufferInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceExternalBufferInfoKhr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        PhysicalDeviceExternalBufferInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn usage(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceExternalBufferInfoKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceExternalBufferInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceExternalBufferInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        PhysicalDeviceExternalBufferInfoKhrBuilder {
            raw: vks::VkPhysicalDeviceExternalBufferInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: BufferCreateFlags) -> &'m mut PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self
    }

    pub fn usage<'m>(&'m mut self, usage: BufferUsageFlags) -> &'m mut PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExternalBufferPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalBufferPropertiesKhr<'s> {
    raw: vks::VkExternalBufferPropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalBufferPropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn external_memory_properties(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalBufferPropertiesKHR {
        &self.raw
    }
}


/// A `VkPhysicalDeviceIDPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceIDPropertiesKhr<'s> {
    raw: vks::VkPhysicalDeviceIDPropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceIDPropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn device_uu_id(&self) {
    }

    pub fn driver_uu_id(&self) {
    }

    pub fn device_lu_id(&self) {
    }

    pub fn device_node_mask(&self) {
    }

    pub fn device_lu_id_valid(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceIDPropertiesKHR {
        &self.raw
    }
}


/// A `VkExternalMemoryImageCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoKhr<'s> {
    raw: vks::VkExternalMemoryImageCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalMemoryImageCreateInfoKhr<'s> {
    pub fn builder<'b>() -> ExternalMemoryImageCreateInfoKhrBuilder<'b> {
        ExternalMemoryImageCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalMemoryImageCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExternalMemoryImageCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExternalMemoryImageCreateInfoKhrBuilder<'b> {
    raw: vks::VkExternalMemoryImageCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExternalMemoryImageCreateInfoKhrBuilder<'b> {
    pub fn new() -> ExternalMemoryImageCreateInfoKhrBuilder<'b> {
        ExternalMemoryImageCreateInfoKhrBuilder {
            raw: vks::VkExternalMemoryImageCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExternalMemoryImageCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut ExternalMemoryImageCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExternalMemoryBufferCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfoKhr<'s> {
    raw: vks::VkExternalMemoryBufferCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalMemoryBufferCreateInfoKhr<'s> {
    pub fn builder<'b>() -> ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
        ExternalMemoryBufferCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalMemoryBufferCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExternalMemoryBufferCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
    raw: vks::VkExternalMemoryBufferCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
    pub fn new() -> ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
        ExternalMemoryBufferCreateInfoKhrBuilder {
            raw: vks::VkExternalMemoryBufferCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExportMemoryAllocateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoKhr<'s> {
    raw: vks::VkExportMemoryAllocateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportMemoryAllocateInfoKhr<'s> {
    pub fn builder<'b>() -> ExportMemoryAllocateInfoKhrBuilder<'b> {
        ExportMemoryAllocateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportMemoryAllocateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExportMemoryAllocateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportMemoryAllocateInfoKhrBuilder<'b> {
    raw: vks::VkExportMemoryAllocateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportMemoryAllocateInfoKhrBuilder<'b> {
    pub fn new() -> ExportMemoryAllocateInfoKhrBuilder<'b> {
        ExportMemoryAllocateInfoKhrBuilder {
            raw: vks::VkExportMemoryAllocateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportMemoryAllocateInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut ExportMemoryAllocateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkImportMemoryWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKhr<'s> {
    raw: vks::VkImportMemoryWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportMemoryWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        ImportMemoryWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn handle(&self) {
    }

    pub fn name(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportMemoryWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkImportMemoryWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportMemoryWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkImportMemoryWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        ImportMemoryWin32HandleInfoKhrBuilder {
            raw: vks::VkImportMemoryWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle<'m>(&'m mut self, handle: HANDLE) -> &'m mut ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn name<'m>(&'m mut self, name: LPCWSTR) -> &'m mut ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExportMemoryWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKhr<'s> {
    raw: vks::VkExportMemoryWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportMemoryWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        ExportMemoryWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn attributes(&self) {
    }

    pub fn dw_access(&self) {
    }

    pub fn name(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportMemoryWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExportMemoryWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportMemoryWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkExportMemoryWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        ExportMemoryWin32HandleInfoKhrBuilder {
            raw: vks::VkExportMemoryWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn attributes<'m, 'a>(&'m mut self, attributes: &'a SECURITY_ATTRIBUTES) -> &'m mut ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn dw_access<'m>(&'m mut self, dw_access: DWORD) -> &'m mut ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn name<'m>(&'m mut self, name: LPCWSTR) -> &'m mut ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkMemoryWin32HandlePropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKhr<'s> {
    raw: vks::VkMemoryWin32HandlePropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryWin32HandlePropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn memory_type_bits(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryWin32HandlePropertiesKHR {
        &self.raw
    }
}


/// A `VkMemoryGetWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryGetWin32HandleInfoKhr<'s> {
    raw: vks::VkMemoryGetWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryGetWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> MemoryGetWin32HandleInfoKhrBuilder<'b> {
        MemoryGetWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn memory(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryGetWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkMemoryGetWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MemoryGetWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkMemoryGetWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> MemoryGetWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> MemoryGetWin32HandleInfoKhrBuilder<'b> {
        MemoryGetWin32HandleInfoKhrBuilder {
            raw: vks::VkMemoryGetWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MemoryGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn memory<'m>(&'m mut self, memory: DeviceMemory) -> &'m mut MemoryGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut MemoryGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkImportMemoryFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportMemoryFdInfoKhr<'s> {
    raw: vks::VkImportMemoryFdInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportMemoryFdInfoKhr<'s> {
    pub fn builder<'b>() -> ImportMemoryFdInfoKhrBuilder<'b> {
        ImportMemoryFdInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn fd(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportMemoryFdInfoKHR {
        &self.raw
    }
}


/// A builder for `VkImportMemoryFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportMemoryFdInfoKhrBuilder<'b> {
    raw: vks::VkImportMemoryFdInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportMemoryFdInfoKhrBuilder<'b> {
    pub fn new() -> ImportMemoryFdInfoKhrBuilder<'b> {
        ImportMemoryFdInfoKhrBuilder {
            raw: vks::VkImportMemoryFdInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportMemoryFdInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut ImportMemoryFdInfoKhrBuilder<'b> {
        self
    }

    pub fn fd<'m>(&'m mut self, fd: i32) -> &'m mut ImportMemoryFdInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkMemoryFdPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryFdPropertiesKhr<'s> {
    raw: vks::VkMemoryFdPropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryFdPropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn memory_type_bits(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryFdPropertiesKHR {
        &self.raw
    }
}


/// A `VkMemoryGetFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryGetFdInfoKhr<'s> {
    raw: vks::VkMemoryGetFdInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryGetFdInfoKhr<'s> {
    pub fn builder<'b>() -> MemoryGetFdInfoKhrBuilder<'b> {
        MemoryGetFdInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn memory(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryGetFdInfoKHR {
        &self.raw
    }
}


/// A builder for `VkMemoryGetFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MemoryGetFdInfoKhrBuilder<'b> {
    raw: vks::VkMemoryGetFdInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> MemoryGetFdInfoKhrBuilder<'b> {
    pub fn new() -> MemoryGetFdInfoKhrBuilder<'b> {
        MemoryGetFdInfoKhrBuilder {
            raw: vks::VkMemoryGetFdInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MemoryGetFdInfoKhrBuilder<'b> {
        self
    }

    pub fn memory<'m>(&'m mut self, memory: DeviceMemory) -> &'m mut MemoryGetFdInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> &'m mut MemoryGetFdInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkWin32KeyedMutexAcquireReleaseInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKhr<'s> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> Win32KeyedMutexAcquireReleaseInfoKhr<'s> {
    pub fn builder<'b>() -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn acquire_count(&self) {
    }

    pub fn acquire_syncs(&self) {
    }

    pub fn acquire_keys(&self) {
    }

    pub fn acquire_timeouts(&self) {
    }

    pub fn release_count(&self) {
    }

    pub fn release_syncs(&self) {
    }

    pub fn release_keys(&self) {
    }

    pub fn raw(&self) -> &vks::VkWin32KeyedMutexAcquireReleaseInfoKHR {
        &self.raw
    }
}


/// A builder for `VkWin32KeyedMutexAcquireReleaseInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
    pub fn new() -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoKhrBuilder {
            raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn acquire_count<'m>(&'m mut self, acquire_count: u32) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn acquire_syncs<'m, 'a>(&'m mut self, acquire_syncs: &'a DeviceMemory) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn acquire_keys<'m, 'a>(&'m mut self, acquire_keys: &'a u64) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn acquire_timeouts<'m, 'a>(&'m mut self, acquire_timeouts: &'a u32) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn release_count<'m>(&'m mut self, release_count: u32) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn release_syncs<'m, 'a>(&'m mut self, release_syncs: &'a DeviceMemory) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

    pub fn release_keys<'m, 'a>(&'m mut self, release_keys: &'a u64) -> &'m mut Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceExternalSemaphoreInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfoKhr<'s> {
    raw: vks::VkPhysicalDeviceExternalSemaphoreInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceExternalSemaphoreInfoKhr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
        PhysicalDeviceExternalSemaphoreInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceExternalSemaphoreInfoKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceExternalSemaphoreInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceExternalSemaphoreInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
        PhysicalDeviceExternalSemaphoreInfoKhrBuilder {
            raw: vks::VkPhysicalDeviceExternalSemaphoreInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> &'m mut PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExternalSemaphorePropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalSemaphorePropertiesKhr<'s> {
    raw: vks::VkExternalSemaphorePropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalSemaphorePropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn export_from_imported_handle_types(&self) {
    }

    pub fn compatible_handle_types(&self) {
    }

    pub fn external_semaphore_features(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalSemaphorePropertiesKHR {
        &self.raw
    }
}


/// A `VkExportSemaphoreCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportSemaphoreCreateInfoKhr<'s> {
    raw: vks::VkExportSemaphoreCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportSemaphoreCreateInfoKhr<'s> {
    pub fn builder<'b>() -> ExportSemaphoreCreateInfoKhrBuilder<'b> {
        ExportSemaphoreCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportSemaphoreCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExportSemaphoreCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportSemaphoreCreateInfoKhrBuilder<'b> {
    raw: vks::VkExportSemaphoreCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportSemaphoreCreateInfoKhrBuilder<'b> {
    pub fn new() -> ExportSemaphoreCreateInfoKhrBuilder<'b> {
        ExportSemaphoreCreateInfoKhrBuilder {
            raw: vks::VkExportSemaphoreCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportSemaphoreCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalSemaphoreHandleTypeFlagsKhr) -> &'m mut ExportSemaphoreCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkImportSemaphoreWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKhr<'s> {
    raw: vks::VkImportSemaphoreWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportSemaphoreWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        ImportSemaphoreWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn semaphore(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn handle(&self) {
    }

    pub fn name(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportSemaphoreWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkImportSemaphoreWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkImportSemaphoreWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        ImportSemaphoreWin32HandleInfoKhrBuilder {
            raw: vks::VkImportSemaphoreWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn semaphore<'m>(&'m mut self, semaphore: Semaphore) -> &'m mut ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SemaphoreImportFlagsKhr) -> &'m mut ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> &'m mut ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle<'m>(&'m mut self, handle: HANDLE) -> &'m mut ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn name<'m>(&'m mut self, name: LPCWSTR) -> &'m mut ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExportSemaphoreWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKhr<'s> {
    raw: vks::VkExportSemaphoreWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportSemaphoreWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        ExportSemaphoreWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn attributes(&self) {
    }

    pub fn dw_access(&self) {
    }

    pub fn name(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportSemaphoreWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExportSemaphoreWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkExportSemaphoreWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        ExportSemaphoreWin32HandleInfoKhrBuilder {
            raw: vks::VkExportSemaphoreWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn attributes<'m, 'a>(&'m mut self, attributes: &'a SECURITY_ATTRIBUTES) -> &'m mut ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn dw_access<'m>(&'m mut self, dw_access: DWORD) -> &'m mut ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn name<'m>(&'m mut self, name: LPCWSTR) -> &'m mut ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkD3D12FenceSubmitInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct D3d12FenceSubmitInfoKHR<'s> {
    raw: vks::VkD3D12FenceSubmitInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> D3d12FenceSubmitInfoKHR<'s> {
    pub fn builder<'b>() -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        D3d12FenceSubmitInfoKHRBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn wait_semaphore_values_count(&self) {
    }

    pub fn wait_semaphore_values(&self) {
    }

    pub fn signal_semaphore_values_count(&self) {
    }

    pub fn signal_semaphore_values(&self) {
    }

    pub fn raw(&self) -> &vks::VkD3D12FenceSubmitInfoKHR {
        &self.raw
    }
}


/// A builder for `VkD3D12FenceSubmitInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct D3d12FenceSubmitInfoKHRBuilder<'b> {
    raw: vks::VkD3D12FenceSubmitInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> D3d12FenceSubmitInfoKHRBuilder<'b> {
    pub fn new() -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        D3d12FenceSubmitInfoKHRBuilder {
            raw: vks::VkD3D12FenceSubmitInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut D3d12FenceSubmitInfoKHRBuilder<'b> {
        self
    }

    pub fn wait_semaphore_values_count<'m>(&'m mut self, wait_semaphore_values_count: u32) -> &'m mut D3d12FenceSubmitInfoKHRBuilder<'b> {
        self
    }

    pub fn wait_semaphore_values<'m, 'a>(&'m mut self, wait_semaphore_values: &'a u64) -> &'m mut D3d12FenceSubmitInfoKHRBuilder<'b> {
        self
    }

    pub fn signal_semaphore_values_count<'m>(&'m mut self, signal_semaphore_values_count: u32) -> &'m mut D3d12FenceSubmitInfoKHRBuilder<'b> {
        self
    }

    pub fn signal_semaphore_values<'m, 'a>(&'m mut self, signal_semaphore_values: &'a u64) -> &'m mut D3d12FenceSubmitInfoKHRBuilder<'b> {
        self
    }

}


/// A `VkSemaphoreGetWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKhr<'s> {
    raw: vks::VkSemaphoreGetWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SemaphoreGetWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        SemaphoreGetWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn semaphore(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkSemaphoreGetWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkSemaphoreGetWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkSemaphoreGetWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        SemaphoreGetWin32HandleInfoKhrBuilder {
            raw: vks::VkSemaphoreGetWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn semaphore<'m>(&'m mut self, semaphore: Semaphore) -> &'m mut SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> &'m mut SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkImportSemaphoreFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportSemaphoreFdInfoKhr<'s> {
    raw: vks::VkImportSemaphoreFdInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportSemaphoreFdInfoKhr<'s> {
    pub fn builder<'b>() -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        ImportSemaphoreFdInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn semaphore(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn fd(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportSemaphoreFdInfoKHR {
        &self.raw
    }
}


/// A builder for `VkImportSemaphoreFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportSemaphoreFdInfoKhrBuilder<'b> {
    raw: vks::VkImportSemaphoreFdInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportSemaphoreFdInfoKhrBuilder<'b> {
    pub fn new() -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        ImportSemaphoreFdInfoKhrBuilder {
            raw: vks::VkImportSemaphoreFdInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportSemaphoreFdInfoKhrBuilder<'b> {
        self
    }

    pub fn semaphore<'m>(&'m mut self, semaphore: Semaphore) -> &'m mut ImportSemaphoreFdInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: SemaphoreImportFlagsKhr) -> &'m mut ImportSemaphoreFdInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> &'m mut ImportSemaphoreFdInfoKhrBuilder<'b> {
        self
    }

    pub fn fd<'m>(&'m mut self, fd: i32) -> &'m mut ImportSemaphoreFdInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkSemaphoreGetFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKhr<'s> {
    raw: vks::VkSemaphoreGetFdInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SemaphoreGetFdInfoKhr<'s> {
    pub fn builder<'b>() -> SemaphoreGetFdInfoKhrBuilder<'b> {
        SemaphoreGetFdInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn semaphore(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkSemaphoreGetFdInfoKHR {
        &self.raw
    }
}


/// A builder for `VkSemaphoreGetFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SemaphoreGetFdInfoKhrBuilder<'b> {
    raw: vks::VkSemaphoreGetFdInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> SemaphoreGetFdInfoKhrBuilder<'b> {
    pub fn new() -> SemaphoreGetFdInfoKhrBuilder<'b> {
        SemaphoreGetFdInfoKhrBuilder {
            raw: vks::VkSemaphoreGetFdInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SemaphoreGetFdInfoKhrBuilder<'b> {
        self
    }

    pub fn semaphore<'m>(&'m mut self, semaphore: Semaphore) -> &'m mut SemaphoreGetFdInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> &'m mut SemaphoreGetFdInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceExternalFenceInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfoKhr<'s> {
    raw: vks::VkPhysicalDeviceExternalFenceInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceExternalFenceInfoKhr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
        PhysicalDeviceExternalFenceInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceExternalFenceInfoKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceExternalFenceInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceExternalFenceInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
        PhysicalDeviceExternalFenceInfoKhrBuilder {
            raw: vks::VkPhysicalDeviceExternalFenceInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> &'m mut PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExternalFencePropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExternalFencePropertiesKhr<'s> {
    raw: vks::VkExternalFencePropertiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExternalFencePropertiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn export_from_imported_handle_types(&self) {
    }

    pub fn compatible_handle_types(&self) {
    }

    pub fn external_fence_features(&self) {
    }

    pub fn raw(&self) -> &vks::VkExternalFencePropertiesKHR {
        &self.raw
    }
}


/// A `VkExportFenceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportFenceCreateInfoKhr<'s> {
    raw: vks::VkExportFenceCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportFenceCreateInfoKhr<'s> {
    pub fn builder<'b>() -> ExportFenceCreateInfoKhrBuilder<'b> {
        ExportFenceCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn handle_types(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportFenceCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExportFenceCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportFenceCreateInfoKhrBuilder<'b> {
    raw: vks::VkExportFenceCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportFenceCreateInfoKhrBuilder<'b> {
    pub fn new() -> ExportFenceCreateInfoKhrBuilder<'b> {
        ExportFenceCreateInfoKhrBuilder {
            raw: vks::VkExportFenceCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportFenceCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_types<'m>(&'m mut self, handle_types: ExternalFenceHandleTypeFlagsKhr) -> &'m mut ExportFenceCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkImportFenceWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportFenceWin32HandleInfoKhr<'s> {
    raw: vks::VkImportFenceWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportFenceWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        ImportFenceWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn fence(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn handle(&self) {
    }

    pub fn name(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportFenceWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkImportFenceWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportFenceWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkImportFenceWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportFenceWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        ImportFenceWin32HandleInfoKhrBuilder {
            raw: vks::VkImportFenceWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn fence<'m>(&'m mut self, fence: Fence) -> &'m mut ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: FenceImportFlagsKhr) -> &'m mut ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> &'m mut ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle<'m>(&'m mut self, handle: HANDLE) -> &'m mut ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn name<'m>(&'m mut self, name: LPCWSTR) -> &'m mut ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkExportFenceWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKhr<'s> {
    raw: vks::VkExportFenceWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ExportFenceWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        ExportFenceWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn attributes(&self) {
    }

    pub fn dw_access(&self) {
    }

    pub fn name(&self) {
    }

    pub fn raw(&self) -> &vks::VkExportFenceWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkExportFenceWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ExportFenceWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkExportFenceWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ExportFenceWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        ExportFenceWin32HandleInfoKhrBuilder {
            raw: vks::VkExportFenceWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn attributes<'m, 'a>(&'m mut self, attributes: &'a SECURITY_ATTRIBUTES) -> &'m mut ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn dw_access<'m>(&'m mut self, dw_access: DWORD) -> &'m mut ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn name<'m>(&'m mut self, name: LPCWSTR) -> &'m mut ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkFenceGetWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKhr<'s> {
    raw: vks::VkFenceGetWin32HandleInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> FenceGetWin32HandleInfoKhr<'s> {
    pub fn builder<'b>() -> FenceGetWin32HandleInfoKhrBuilder<'b> {
        FenceGetWin32HandleInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn fence(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkFenceGetWin32HandleInfoKHR {
        &self.raw
    }
}


/// A builder for `VkFenceGetWin32HandleInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FenceGetWin32HandleInfoKhrBuilder<'b> {
    raw: vks::VkFenceGetWin32HandleInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> FenceGetWin32HandleInfoKhrBuilder<'b> {
    pub fn new() -> FenceGetWin32HandleInfoKhrBuilder<'b> {
        FenceGetWin32HandleInfoKhrBuilder {
            raw: vks::VkFenceGetWin32HandleInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut FenceGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn fence<'m>(&'m mut self, fence: Fence) -> &'m mut FenceGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> &'m mut FenceGetWin32HandleInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkImportFenceFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImportFenceFdInfoKhr<'s> {
    raw: vks::VkImportFenceFdInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImportFenceFdInfoKhr<'s> {
    pub fn builder<'b>() -> ImportFenceFdInfoKhrBuilder<'b> {
        ImportFenceFdInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn fence(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn fd(&self) {
    }

    pub fn raw(&self) -> &vks::VkImportFenceFdInfoKHR {
        &self.raw
    }
}


/// A builder for `VkImportFenceFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImportFenceFdInfoKhrBuilder<'b> {
    raw: vks::VkImportFenceFdInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImportFenceFdInfoKhrBuilder<'b> {
    pub fn new() -> ImportFenceFdInfoKhrBuilder<'b> {
        ImportFenceFdInfoKhrBuilder {
            raw: vks::VkImportFenceFdInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImportFenceFdInfoKhrBuilder<'b> {
        self
    }

    pub fn fence<'m>(&'m mut self, fence: Fence) -> &'m mut ImportFenceFdInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: FenceImportFlagsKhr) -> &'m mut ImportFenceFdInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> &'m mut ImportFenceFdInfoKhrBuilder<'b> {
        self
    }

    pub fn fd<'m>(&'m mut self, fd: i32) -> &'m mut ImportFenceFdInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkFenceGetFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct FenceGetFdInfoKhr<'s> {
    raw: vks::VkFenceGetFdInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> FenceGetFdInfoKhr<'s> {
    pub fn builder<'b>() -> FenceGetFdInfoKhrBuilder<'b> {
        FenceGetFdInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn fence(&self) {
    }

    pub fn handle_type(&self) {
    }

    pub fn raw(&self) -> &vks::VkFenceGetFdInfoKHR {
        &self.raw
    }
}


/// A builder for `VkFenceGetFdInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FenceGetFdInfoKhrBuilder<'b> {
    raw: vks::VkFenceGetFdInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> FenceGetFdInfoKhrBuilder<'b> {
    pub fn new() -> FenceGetFdInfoKhrBuilder<'b> {
        FenceGetFdInfoKhrBuilder {
            raw: vks::VkFenceGetFdInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut FenceGetFdInfoKhrBuilder<'b> {
        self
    }

    pub fn fence<'m>(&'m mut self, fence: Fence) -> &'m mut FenceGetFdInfoKhrBuilder<'b> {
        self
    }

    pub fn handle_type<'m>(&'m mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> &'m mut FenceGetFdInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceMultiviewFeaturesKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeaturesKhx<'s> {
    raw: vks::VkPhysicalDeviceMultiviewFeaturesKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> PhysicalDeviceMultiviewFeaturesKhx<'s> {
    pub fn builder<'b>() -> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        PhysicalDeviceMultiviewFeaturesKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn multiview(&self) {
    }

    pub fn multiview_geometry_shader(&self) {
    }

    pub fn multiview_tessellation_shader(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceMultiviewFeaturesKHX {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceMultiviewFeaturesKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
    raw: vks::VkPhysicalDeviceMultiviewFeaturesKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
    pub fn new() -> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        PhysicalDeviceMultiviewFeaturesKhxBuilder {
            raw: vks::VkPhysicalDeviceMultiviewFeaturesKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self
    }

    pub fn multiview<'m>(&'m mut self, multiview: bool) -> &'m mut PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self
    }

    pub fn multiview_geometry_shader<'m>(&'m mut self, multiview_geometry_shader: bool) -> &'m mut PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self
    }

    pub fn multiview_tessellation_shader<'m>(&'m mut self, multiview_tessellation_shader: bool) -> &'m mut PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceMultiviewPropertiesKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPropertiesKhx<'s> {
    raw: vks::VkPhysicalDeviceMultiviewPropertiesKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> PhysicalDeviceMultiviewPropertiesKhx<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn max_multiview_view_count(&self) {
    }

    pub fn max_multiview_instance_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceMultiviewPropertiesKHX {
        &self.raw
    }
}


/// A `VkRenderPassMultiviewCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfoKhx<'s> {
    raw: vks::VkRenderPassMultiviewCreateInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> RenderPassMultiviewCreateInfoKhx<'s> {
    pub fn builder<'b>() -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        RenderPassMultiviewCreateInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn subpass_count(&self) {
    }

    pub fn view_masks(&self) {
    }

    pub fn dependency_count(&self) {
    }

    pub fn view_offsets(&self) {
    }

    pub fn correlation_mask_count(&self) {
    }

    pub fn correlation_masks(&self) {
    }

    pub fn raw(&self) -> &vks::VkRenderPassMultiviewCreateInfoKHX {
        &self.raw
    }
}


/// A builder for `VkRenderPassMultiviewCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct RenderPassMultiviewCreateInfoKhxBuilder<'b> {
    raw: vks::VkRenderPassMultiviewCreateInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
    pub fn new() -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        RenderPassMultiviewCreateInfoKhxBuilder {
            raw: vks::VkRenderPassMultiviewCreateInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn subpass_count<'m>(&'m mut self, subpass_count: u32) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn view_masks<'m, 'a>(&'m mut self, view_masks: &'a u32) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn dependency_count<'m>(&'m mut self, dependency_count: u32) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn view_offsets<'m, 'a>(&'m mut self, view_offsets: &'a i32) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn correlation_mask_count<'m>(&'m mut self, correlation_mask_count: u32) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn correlation_masks<'m, 'a>(&'m mut self, correlation_masks: &'a u32) -> &'m mut RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkSurfaceCapabilities2EXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SurfaceCapabilities2Ext<'s> {
    raw: vks::VkSurfaceCapabilities2EXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> SurfaceCapabilities2Ext<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn min_image_count(&self) {
    }

    pub fn max_image_count(&self) {
    }

    pub fn current_extent(&self) {
    }

    pub fn min_image_extent(&self) {
    }

    pub fn max_image_extent(&self) {
    }

    pub fn max_image_array_layers(&self) {
    }

    pub fn supported_transforms(&self) {
    }

    pub fn current_transform(&self) {
    }

    pub fn supported_composite_alpha(&self) {
    }

    pub fn supported_usage_flags(&self) {
    }

    pub fn supported_surface_counters(&self) {
    }

    pub fn raw(&self) -> &vks::VkSurfaceCapabilities2EXT {
        &self.raw
    }
}


/// A `VkDisplayPowerInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayPowerInfoExt<'s> {
    raw: vks::VkDisplayPowerInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DisplayPowerInfoExt<'s> {
    pub fn builder<'b>() -> DisplayPowerInfoExtBuilder<'b> {
        DisplayPowerInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn power_state(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayPowerInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDisplayPowerInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplayPowerInfoExtBuilder<'b> {
    raw: vks::VkDisplayPowerInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DisplayPowerInfoExtBuilder<'b> {
    pub fn new() -> DisplayPowerInfoExtBuilder<'b> {
        DisplayPowerInfoExtBuilder {
            raw: vks::VkDisplayPowerInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DisplayPowerInfoExtBuilder<'b> {
        self
    }

    pub fn power_state<'m>(&'m mut self, power_state: DisplayPowerStateExt) -> &'m mut DisplayPowerInfoExtBuilder<'b> {
        self
    }

}


/// A `VkDeviceEventInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceEventInfoExt<'s> {
    raw: vks::VkDeviceEventInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DeviceEventInfoExt<'s> {
    pub fn builder<'b>() -> DeviceEventInfoExtBuilder<'b> {
        DeviceEventInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn device_event(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceEventInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDeviceEventInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DeviceEventInfoExtBuilder<'b> {
    raw: vks::VkDeviceEventInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DeviceEventInfoExtBuilder<'b> {
    pub fn new() -> DeviceEventInfoExtBuilder<'b> {
        DeviceEventInfoExtBuilder {
            raw: vks::VkDeviceEventInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceEventInfoExtBuilder<'b> {
        self
    }

    pub fn device_event<'m>(&'m mut self, device_event: DeviceEventTypeExt) -> &'m mut DeviceEventInfoExtBuilder<'b> {
        self
    }

}


/// A `VkDisplayEventInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DisplayEventInfoExt<'s> {
    raw: vks::VkDisplayEventInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> DisplayEventInfoExt<'s> {
    pub fn builder<'b>() -> DisplayEventInfoExtBuilder<'b> {
        DisplayEventInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn display_event(&self) {
    }

    pub fn raw(&self) -> &vks::VkDisplayEventInfoEXT {
        &self.raw
    }
}


/// A builder for `VkDisplayEventInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplayEventInfoExtBuilder<'b> {
    raw: vks::VkDisplayEventInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> DisplayEventInfoExtBuilder<'b> {
    pub fn new() -> DisplayEventInfoExtBuilder<'b> {
        DisplayEventInfoExtBuilder {
            raw: vks::VkDisplayEventInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DisplayEventInfoExtBuilder<'b> {
        self
    }

    pub fn display_event<'m>(&'m mut self, display_event: DisplayEventTypeExt) -> &'m mut DisplayEventInfoExtBuilder<'b> {
        self
    }

}


/// A `VkSwapchainCounterCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SwapchainCounterCreateInfoExt<'s> {
    raw: vks::VkSwapchainCounterCreateInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> SwapchainCounterCreateInfoExt<'s> {
    pub fn builder<'b>() -> SwapchainCounterCreateInfoExtBuilder<'b> {
        SwapchainCounterCreateInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn surface_counters(&self) {
    }

    pub fn raw(&self) -> &vks::VkSwapchainCounterCreateInfoEXT {
        &self.raw
    }
}


/// A builder for `VkSwapchainCounterCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SwapchainCounterCreateInfoExtBuilder<'b> {
    raw: vks::VkSwapchainCounterCreateInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> SwapchainCounterCreateInfoExtBuilder<'b> {
    pub fn new() -> SwapchainCounterCreateInfoExtBuilder<'b> {
        SwapchainCounterCreateInfoExtBuilder {
            raw: vks::VkSwapchainCounterCreateInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SwapchainCounterCreateInfoExtBuilder<'b> {
        self
    }

    pub fn surface_counters<'m>(&'m mut self, surface_counters: SurfaceCounterFlagsExt) -> &'m mut SwapchainCounterCreateInfoExtBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceGroupPropertiesKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceGroupPropertiesKhx<'s> {
    raw: vks::VkPhysicalDeviceGroupPropertiesKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> PhysicalDeviceGroupPropertiesKhx<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn physical_device_count(&self) {
    }

    pub fn physical_devices(&self) {
    }

    pub fn subset_allocation(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceGroupPropertiesKHX {
        &self.raw
    }
}


/// A `VkMemoryAllocateFlagsInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryAllocateFlagsInfoKhx<'s> {
    raw: vks::VkMemoryAllocateFlagsInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> MemoryAllocateFlagsInfoKhx<'s> {
    pub fn builder<'b>() -> MemoryAllocateFlagsInfoKhxBuilder<'b> {
        MemoryAllocateFlagsInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn device_mask(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryAllocateFlagsInfoKHX {
        &self.raw
    }
}


/// A builder for `VkMemoryAllocateFlagsInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct MemoryAllocateFlagsInfoKhxBuilder<'b> {
    raw: vks::VkMemoryAllocateFlagsInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> MemoryAllocateFlagsInfoKhxBuilder<'b> {
    pub fn new() -> MemoryAllocateFlagsInfoKhxBuilder<'b> {
        MemoryAllocateFlagsInfoKhxBuilder {
            raw: vks::VkMemoryAllocateFlagsInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MemoryAllocateFlagsInfoKhxBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: MemoryAllocateFlagsKhx) -> &'m mut MemoryAllocateFlagsInfoKhxBuilder<'b> {
        self
    }

    pub fn device_mask<'m>(&'m mut self, device_mask: u32) -> &'m mut MemoryAllocateFlagsInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkBindBufferMemoryDeviceGroupInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfoKhx<'s> {
    raw: vks::VkBindBufferMemoryDeviceGroupInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> BindBufferMemoryDeviceGroupInfoKhx<'s> {
    pub fn builder<'b>() -> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        BindBufferMemoryDeviceGroupInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn device_index_count(&self) {
    }

    pub fn device_indices(&self) {
    }

    pub fn raw(&self) -> &vks::VkBindBufferMemoryDeviceGroupInfoKHX {
        &self.raw
    }
}


/// A builder for `VkBindBufferMemoryDeviceGroupInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
    raw: vks::VkBindBufferMemoryDeviceGroupInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
    pub fn new() -> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        BindBufferMemoryDeviceGroupInfoKhxBuilder {
            raw: vks::VkBindBufferMemoryDeviceGroupInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

    pub fn device_index_count<'m>(&'m mut self, device_index_count: u32) -> &'m mut BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

    pub fn device_indices<'m, 'a>(&'m mut self, device_indices: &'a u32) -> &'m mut BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkBindImageMemoryDeviceGroupInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfoKhx<'s> {
    raw: vks::VkBindImageMemoryDeviceGroupInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> BindImageMemoryDeviceGroupInfoKhx<'s> {
    pub fn builder<'b>() -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        BindImageMemoryDeviceGroupInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn device_index_count(&self) {
    }

    pub fn device_indices(&self) {
    }

    pub fn s_fr_rect_count(&self) {
    }

    pub fn s_fr_rects(&self) {
    }

    pub fn raw(&self) -> &vks::VkBindImageMemoryDeviceGroupInfoKHX {
        &self.raw
    }
}


/// A builder for `VkBindImageMemoryDeviceGroupInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
    raw: vks::VkBindImageMemoryDeviceGroupInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
    pub fn new() -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        BindImageMemoryDeviceGroupInfoKhxBuilder {
            raw: vks::VkBindImageMemoryDeviceGroupInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

    pub fn device_index_count<'m>(&'m mut self, device_index_count: u32) -> &'m mut BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

    pub fn device_indices<'m, 'a>(&'m mut self, device_indices: &'a u32) -> &'m mut BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

    pub fn s_fr_rect_count<'m>(&'m mut self, s_fr_rect_count: u32) -> &'m mut BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

    pub fn s_fr_rects<'m, 'a>(&'m mut self, s_fr_rects: &'a Rect2d) -> &'m mut BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupRenderPassBeginInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfoKhx<'s> {
    raw: vks::VkDeviceGroupRenderPassBeginInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupRenderPassBeginInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        DeviceGroupRenderPassBeginInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn device_mask(&self) {
    }

    pub fn device_render_area_count(&self) {
    }

    pub fn device_render_areas(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupRenderPassBeginInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupRenderPassBeginInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupRenderPassBeginInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        DeviceGroupRenderPassBeginInfoKhxBuilder {
            raw: vks::VkDeviceGroupRenderPassBeginInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self
    }

    pub fn device_mask<'m>(&'m mut self, device_mask: u32) -> &'m mut DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self
    }

    pub fn device_render_area_count<'m>(&'m mut self, device_render_area_count: u32) -> &'m mut DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self
    }

    pub fn device_render_areas<'m, 'a>(&'m mut self, device_render_areas: &'a Rect2d) -> &'m mut DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupCommandBufferBeginInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfoKhx<'s> {
    raw: vks::VkDeviceGroupCommandBufferBeginInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupCommandBufferBeginInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
        DeviceGroupCommandBufferBeginInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn device_mask(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupCommandBufferBeginInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupCommandBufferBeginInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupCommandBufferBeginInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
        DeviceGroupCommandBufferBeginInfoKhxBuilder {
            raw: vks::VkDeviceGroupCommandBufferBeginInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
        self
    }

    pub fn device_mask<'m>(&'m mut self, device_mask: u32) -> &'m mut DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupSubmitInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupSubmitInfoKhx<'s> {
    raw: vks::VkDeviceGroupSubmitInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupSubmitInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        DeviceGroupSubmitInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn wait_semaphore_count(&self) {
    }

    pub fn wait_semaphore_device_indices(&self) {
    }

    pub fn command_buffer_count(&self) {
    }

    pub fn command_buffer_device_masks(&self) {
    }

    pub fn signal_semaphore_count(&self) {
    }

    pub fn signal_semaphore_device_indices(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupSubmitInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupSubmitInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupSubmitInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupSubmitInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupSubmitInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        DeviceGroupSubmitInfoKhxBuilder {
            raw: vks::VkDeviceGroupSubmitInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

    pub fn wait_semaphore_count<'m>(&'m mut self, wait_semaphore_count: u32) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

    pub fn wait_semaphore_device_indices<'m, 'a>(&'m mut self, wait_semaphore_device_indices: &'a u32) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

    pub fn command_buffer_count<'m>(&'m mut self, command_buffer_count: u32) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

    pub fn command_buffer_device_masks<'m, 'a>(&'m mut self, command_buffer_device_masks: &'a u32) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

    pub fn signal_semaphore_count<'m>(&'m mut self, signal_semaphore_count: u32) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

    pub fn signal_semaphore_device_indices<'m, 'a>(&'m mut self, signal_semaphore_device_indices: &'a u32) -> &'m mut DeviceGroupSubmitInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupBindSparseInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupBindSparseInfoKhx<'s> {
    raw: vks::VkDeviceGroupBindSparseInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupBindSparseInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupBindSparseInfoKhxBuilder<'b> {
        DeviceGroupBindSparseInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn resource_device_index(&self) {
    }

    pub fn memory_device_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupBindSparseInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupBindSparseInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupBindSparseInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupBindSparseInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupBindSparseInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupBindSparseInfoKhxBuilder<'b> {
        DeviceGroupBindSparseInfoKhxBuilder {
            raw: vks::VkDeviceGroupBindSparseInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupBindSparseInfoKhxBuilder<'b> {
        self
    }

    pub fn resource_device_index<'m>(&'m mut self, resource_device_index: u32) -> &'m mut DeviceGroupBindSparseInfoKhxBuilder<'b> {
        self
    }

    pub fn memory_device_index<'m>(&'m mut self, memory_device_index: u32) -> &'m mut DeviceGroupBindSparseInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupPresentCapabilitiesKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKhx<'s> {
    raw: vks::VkDeviceGroupPresentCapabilitiesKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupPresentCapabilitiesKhx<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn present_mask(&self) {
    }

    pub fn modes(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupPresentCapabilitiesKHX {
        &self.raw
    }
}


/// A `VkImageSwapchainCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKhx<'s> {
    raw: vks::VkImageSwapchainCreateInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> ImageSwapchainCreateInfoKhx<'s> {
    pub fn builder<'b>() -> ImageSwapchainCreateInfoKhxBuilder<'b> {
        ImageSwapchainCreateInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn swapchain(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageSwapchainCreateInfoKHX {
        &self.raw
    }
}


/// A builder for `VkImageSwapchainCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct ImageSwapchainCreateInfoKhxBuilder<'b> {
    raw: vks::VkImageSwapchainCreateInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> ImageSwapchainCreateInfoKhxBuilder<'b> {
    pub fn new() -> ImageSwapchainCreateInfoKhxBuilder<'b> {
        ImageSwapchainCreateInfoKhxBuilder {
            raw: vks::VkImageSwapchainCreateInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImageSwapchainCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn swapchain<'m>(&'m mut self, swapchain: SwapchainKhr) -> &'m mut ImageSwapchainCreateInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkBindImageMemorySwapchainInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKhx<'s> {
    raw: vks::VkBindImageMemorySwapchainInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> BindImageMemorySwapchainInfoKhx<'s> {
    pub fn builder<'b>() -> BindImageMemorySwapchainInfoKhxBuilder<'b> {
        BindImageMemorySwapchainInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn swapchain(&self) {
    }

    pub fn image_index(&self) {
    }

    pub fn raw(&self) -> &vks::VkBindImageMemorySwapchainInfoKHX {
        &self.raw
    }
}


/// A builder for `VkBindImageMemorySwapchainInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct BindImageMemorySwapchainInfoKhxBuilder<'b> {
    raw: vks::VkBindImageMemorySwapchainInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> BindImageMemorySwapchainInfoKhxBuilder<'b> {
    pub fn new() -> BindImageMemorySwapchainInfoKhxBuilder<'b> {
        BindImageMemorySwapchainInfoKhxBuilder {
            raw: vks::VkBindImageMemorySwapchainInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BindImageMemorySwapchainInfoKhxBuilder<'b> {
        self
    }

    pub fn swapchain<'m>(&'m mut self, swapchain: SwapchainKhr) -> &'m mut BindImageMemorySwapchainInfoKhxBuilder<'b> {
        self
    }

    pub fn image_index<'m>(&'m mut self, image_index: u32) -> &'m mut BindImageMemorySwapchainInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkAcquireNextImageInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct AcquireNextImageInfoKhx<'s> {
    raw: vks::VkAcquireNextImageInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> AcquireNextImageInfoKhx<'s> {
    pub fn builder<'b>() -> AcquireNextImageInfoKhxBuilder<'b> {
        AcquireNextImageInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn swapchain(&self) {
    }

    pub fn timeout(&self) {
    }

    pub fn semaphore(&self) {
    }

    pub fn fence(&self) {
    }

    pub fn device_mask(&self) {
    }

    pub fn raw(&self) -> &vks::VkAcquireNextImageInfoKHX {
        &self.raw
    }
}


/// A builder for `VkAcquireNextImageInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct AcquireNextImageInfoKhxBuilder<'b> {
    raw: vks::VkAcquireNextImageInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> AcquireNextImageInfoKhxBuilder<'b> {
    pub fn new() -> AcquireNextImageInfoKhxBuilder<'b> {
        AcquireNextImageInfoKhxBuilder {
            raw: vks::VkAcquireNextImageInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut AcquireNextImageInfoKhxBuilder<'b> {
        self
    }

    pub fn swapchain<'m>(&'m mut self, swapchain: SwapchainKhr) -> &'m mut AcquireNextImageInfoKhxBuilder<'b> {
        self
    }

    pub fn timeout<'m>(&'m mut self, timeout: u64) -> &'m mut AcquireNextImageInfoKhxBuilder<'b> {
        self
    }

    pub fn semaphore<'m>(&'m mut self, semaphore: Semaphore) -> &'m mut AcquireNextImageInfoKhxBuilder<'b> {
        self
    }

    pub fn fence<'m>(&'m mut self, fence: Fence) -> &'m mut AcquireNextImageInfoKhxBuilder<'b> {
        self
    }

    pub fn device_mask<'m>(&'m mut self, device_mask: u32) -> &'m mut AcquireNextImageInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupPresentInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKhx<'s> {
    raw: vks::VkDeviceGroupPresentInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupPresentInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupPresentInfoKhxBuilder<'b> {
        DeviceGroupPresentInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn swapchain_count(&self) {
    }

    pub fn device_masks(&self) {
    }

    pub fn mode(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupPresentInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupPresentInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupPresentInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupPresentInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupPresentInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupPresentInfoKhxBuilder<'b> {
        DeviceGroupPresentInfoKhxBuilder {
            raw: vks::VkDeviceGroupPresentInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupPresentInfoKhxBuilder<'b> {
        self
    }

    pub fn swapchain_count<'m>(&'m mut self, swapchain_count: u32) -> &'m mut DeviceGroupPresentInfoKhxBuilder<'b> {
        self
    }

    pub fn device_masks<'m, 'a>(&'m mut self, device_masks: &'a u32) -> &'m mut DeviceGroupPresentInfoKhxBuilder<'b> {
        self
    }

    pub fn mode<'m>(&'m mut self, mode: DeviceGroupPresentModeFlagsKhx) -> &'m mut DeviceGroupPresentInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupDeviceCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfoKhx<'s> {
    raw: vks::VkDeviceGroupDeviceCreateInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupDeviceCreateInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        DeviceGroupDeviceCreateInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn physical_device_count(&self) {
    }

    pub fn physical_devices(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupDeviceCreateInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupDeviceCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupDeviceCreateInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        DeviceGroupDeviceCreateInfoKhxBuilder {
            raw: vks::VkDeviceGroupDeviceCreateInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn physical_device_count<'m>(&'m mut self, physical_device_count: u32) -> &'m mut DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn physical_devices<'m, 'a>(&'m mut self, physical_devices: &'a PhysicalDevice) -> &'m mut DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDeviceGroupSwapchainCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKhx<'s> {
    raw: vks::VkDeviceGroupSwapchainCreateInfoKHX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupSwapchainCreateInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
        DeviceGroupSwapchainCreateInfoKhxBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn modes(&self) {
    }

    pub fn raw(&self) -> &vks::VkDeviceGroupSwapchainCreateInfoKHX {
        &self.raw
    }
}


/// A builder for `VkDeviceGroupSwapchainCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupSwapchainCreateInfoKHX,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
        DeviceGroupSwapchainCreateInfoKhxBuilder {
            raw: vks::VkDeviceGroupSwapchainCreateInfoKHX::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
        self
    }

    pub fn modes<'m>(&'m mut self, modes: DeviceGroupPresentModeFlagsKhx) -> &'m mut DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
        self
    }

}


/// A `VkDescriptorUpdateTemplateEntryKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntryKhr {
    raw: vks::VkDescriptorUpdateTemplateEntryKHR,
}

impl DescriptorUpdateTemplateEntryKhr {
    pub fn builder() -> DescriptorUpdateTemplateEntryKhrBuilder {
        DescriptorUpdateTemplateEntryKhrBuilder::new()
    }

    pub fn dst_binding(&self) {
    }

    pub fn dst_array_element(&self) {
    }

    pub fn descriptor_count(&self) {
    }

    pub fn descriptor_type(&self) {
    }

    pub fn offset(&self) {
    }

    pub fn stride(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorUpdateTemplateEntryKHR {
        &self.raw
    }
}


/// A builder for `VkDescriptorUpdateTemplateEntryKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorUpdateTemplateEntryKhrBuilder {
    raw: vks::VkDescriptorUpdateTemplateEntryKHR,
}

impl DescriptorUpdateTemplateEntryKhrBuilder {
    pub fn new() -> DescriptorUpdateTemplateEntryKhrBuilder {
        DescriptorUpdateTemplateEntryKhrBuilder {
            raw: vks::VkDescriptorUpdateTemplateEntryKHR::default(),
        }
    }

    pub fn dst_binding<'m>(&'m mut self, dst_binding: u32) -> &'m mut DescriptorUpdateTemplateEntryKhrBuilder {
        self
    }

    pub fn dst_array_element<'m>(&'m mut self, dst_array_element: u32) -> &'m mut DescriptorUpdateTemplateEntryKhrBuilder {
        self
    }

    pub fn descriptor_count<'m>(&'m mut self, descriptor_count: u32) -> &'m mut DescriptorUpdateTemplateEntryKhrBuilder {
        self
    }

    pub fn descriptor_type<'m>(&'m mut self, descriptor_type: DescriptorType) -> &'m mut DescriptorUpdateTemplateEntryKhrBuilder {
        self
    }

    pub fn offset<'m>(&'m mut self, offset: usize) -> &'m mut DescriptorUpdateTemplateEntryKhrBuilder {
        self
    }

    pub fn stride<'m>(&'m mut self, stride: usize) -> &'m mut DescriptorUpdateTemplateEntryKhrBuilder {
        self
    }

}


/// A `VkDescriptorUpdateTemplateCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfoKhr<'s> {
    raw: vks::VkDescriptorUpdateTemplateCreateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorUpdateTemplateCreateInfoKhr<'s> {
    pub fn builder<'b>() -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        DescriptorUpdateTemplateCreateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn descriptor_update_entry_count(&self) {
    }

    pub fn descriptor_update_entries(&self) {
    }

    pub fn template_type(&self) {
    }

    pub fn descriptor_set_layout(&self) {
    }

    pub fn pipeline_bind_point(&self) {
    }

    pub fn pipeline_layout(&self) {
    }

    pub fn set(&self) {
    }

    pub fn raw(&self) -> &vks::VkDescriptorUpdateTemplateCreateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkDescriptorUpdateTemplateCreateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
    raw: vks::VkDescriptorUpdateTemplateCreateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
    pub fn new() -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        DescriptorUpdateTemplateCreateInfoKhrBuilder {
            raw: vks::VkDescriptorUpdateTemplateCreateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: DescriptorUpdateTemplateCreateFlagsKhr) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn descriptor_update_entry_count<'m>(&'m mut self, descriptor_update_entry_count: u32) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn descriptor_update_entries<'m, 'a>(&'m mut self, descriptor_update_entries: &'a DescriptorUpdateTemplateEntryKhr) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn template_type<'m>(&'m mut self, template_type: DescriptorUpdateTemplateTypeKhr) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn descriptor_set_layout<'m>(&'m mut self, descriptor_set_layout: DescriptorSetLayout) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn pipeline_bind_point<'m>(&'m mut self, pipeline_bind_point: PipelineBindPoint) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn pipeline_layout<'m>(&'m mut self, pipeline_layout: PipelineLayout) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

    pub fn set<'m>(&'m mut self, set: u32) -> &'m mut DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkXYColorEXT`.
///
/// Chromaticity coordinate
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct XYColorExt {
    raw: vks::VkXYColorEXT,
}

impl XYColorExt {
    pub fn builder() -> XYColorExtBuilder {
        XYColorExtBuilder::new()
    }

    pub fn x(&self) {
    }

    pub fn y(&self) {
    }

    pub fn raw(&self) -> &vks::VkXYColorEXT {
        &self.raw
    }
}


/// A builder for `VkXYColorEXT`.
///
/// Chromaticity coordinate
#[derive(Debug, Clone, Default)]
pub struct XYColorExtBuilder {
    raw: vks::VkXYColorEXT,
}

impl XYColorExtBuilder {
    pub fn new() -> XYColorExtBuilder {
        XYColorExtBuilder {
            raw: vks::VkXYColorEXT::default(),
        }
    }

    pub fn x<'m>(&'m mut self, x: f32) -> &'m mut XYColorExtBuilder {
        self
    }

    pub fn y<'m>(&'m mut self, y: f32) -> &'m mut XYColorExtBuilder {
        self
    }

}


/// A `VkHdrMetadataEXT`.
///
///  From CTA 861.3
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct HdrMetadataExt<'s> {
    raw: vks::VkHdrMetadataEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> HdrMetadataExt<'s> {
    pub fn builder<'b>() -> HdrMetadataExtBuilder<'b> {
        HdrMetadataExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn display_primary_red(&self) {
    }

    pub fn display_primary_green(&self) {
    }

    pub fn display_primary_blue(&self) {
    }

    pub fn white_point(&self) {
    }

    pub fn max_luminance(&self) {
    }

    pub fn min_luminance(&self) {
    }

    pub fn max_content_light_level(&self) {
    }

    pub fn max_frame_average_light_level(&self) {
    }

    pub fn raw(&self) -> &vks::VkHdrMetadataEXT {
        &self.raw
    }
}


/// A builder for `VkHdrMetadataEXT`.
///
///  From CTA 861.3
#[derive(Debug, Clone, Default)]
pub struct HdrMetadataExtBuilder<'b> {
    raw: vks::VkHdrMetadataEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> HdrMetadataExtBuilder<'b> {
    pub fn new() -> HdrMetadataExtBuilder<'b> {
        HdrMetadataExtBuilder {
            raw: vks::VkHdrMetadataEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn display_primary_red<'m>(&'m mut self, display_primary_red: XYColorExt) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn display_primary_green<'m>(&'m mut self, display_primary_green: XYColorExt) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn display_primary_blue<'m>(&'m mut self, display_primary_blue: XYColorExt) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn white_point<'m>(&'m mut self, white_point: XYColorExt) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn max_luminance<'m>(&'m mut self, max_luminance: f32) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn min_luminance<'m>(&'m mut self, min_luminance: f32) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn max_content_light_level<'m>(&'m mut self, max_content_light_level: f32) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

    pub fn max_frame_average_light_level<'m>(&'m mut self, max_frame_average_light_level: f32) -> &'m mut HdrMetadataExtBuilder<'b> {
        self
    }

}


/// A `VkRefreshCycleDurationGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RefreshCycleDurationGoogle {
    raw: vks::VkRefreshCycleDurationGOOGLE,
}

impl RefreshCycleDurationGoogle {
    pub fn builder() -> RefreshCycleDurationGoogleBuilder {
        RefreshCycleDurationGoogleBuilder::new()
    }

    pub fn refresh_duration(&self) {
    }

    pub fn raw(&self) -> &vks::VkRefreshCycleDurationGOOGLE {
        &self.raw
    }
}


/// A builder for `VkRefreshCycleDurationGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct RefreshCycleDurationGoogleBuilder {
    raw: vks::VkRefreshCycleDurationGOOGLE,
}

impl RefreshCycleDurationGoogleBuilder {
    pub fn new() -> RefreshCycleDurationGoogleBuilder {
        RefreshCycleDurationGoogleBuilder {
            raw: vks::VkRefreshCycleDurationGOOGLE::default(),
        }
    }

    pub fn refresh_duration<'m>(&'m mut self, refresh_duration: u64) -> &'m mut RefreshCycleDurationGoogleBuilder {
        self
    }

}


/// A `VkPastPresentationTimingGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PastPresentationTimingGoogle {
    raw: vks::VkPastPresentationTimingGOOGLE,
}

impl PastPresentationTimingGoogle {
    pub fn builder() -> PastPresentationTimingGoogleBuilder {
        PastPresentationTimingGoogleBuilder::new()
    }

    pub fn present_id(&self) {
    }

    pub fn desired_present_time(&self) {
    }

    pub fn actual_present_time(&self) {
    }

    pub fn earliest_present_time(&self) {
    }

    pub fn present_margin(&self) {
    }

    pub fn raw(&self) -> &vks::VkPastPresentationTimingGOOGLE {
        &self.raw
    }
}


/// A builder for `VkPastPresentationTimingGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PastPresentationTimingGoogleBuilder {
    raw: vks::VkPastPresentationTimingGOOGLE,
}

impl PastPresentationTimingGoogleBuilder {
    pub fn new() -> PastPresentationTimingGoogleBuilder {
        PastPresentationTimingGoogleBuilder {
            raw: vks::VkPastPresentationTimingGOOGLE::default(),
        }
    }

    pub fn present_id<'m>(&'m mut self, present_id: u32) -> &'m mut PastPresentationTimingGoogleBuilder {
        self
    }

    pub fn desired_present_time<'m>(&'m mut self, desired_present_time: u64) -> &'m mut PastPresentationTimingGoogleBuilder {
        self
    }

    pub fn actual_present_time<'m>(&'m mut self, actual_present_time: u64) -> &'m mut PastPresentationTimingGoogleBuilder {
        self
    }

    pub fn earliest_present_time<'m>(&'m mut self, earliest_present_time: u64) -> &'m mut PastPresentationTimingGoogleBuilder {
        self
    }

    pub fn present_margin<'m>(&'m mut self, present_margin: u64) -> &'m mut PastPresentationTimingGoogleBuilder {
        self
    }

}


/// A `VkPresentTimesInfoGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PresentTimesInfoGoogle<'s> {
    raw: vks::VkPresentTimesInfoGOOGLE,
    _p: PhantomData<&'s ()>,
}

impl<'s> PresentTimesInfoGoogle<'s> {
    pub fn builder<'b>() -> PresentTimesInfoGoogleBuilder<'b> {
        PresentTimesInfoGoogleBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn swapchain_count(&self) {
    }

    pub fn times(&self) {
    }

    pub fn raw(&self) -> &vks::VkPresentTimesInfoGOOGLE {
        &self.raw
    }
}


/// A builder for `VkPresentTimesInfoGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentTimesInfoGoogleBuilder<'b> {
    raw: vks::VkPresentTimesInfoGOOGLE,
    _p: PhantomData<&'b ()>,
}

impl<'b> PresentTimesInfoGoogleBuilder<'b> {
    pub fn new() -> PresentTimesInfoGoogleBuilder<'b> {
        PresentTimesInfoGoogleBuilder {
            raw: vks::VkPresentTimesInfoGOOGLE::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PresentTimesInfoGoogleBuilder<'b> {
        self
    }

    pub fn swapchain_count<'m>(&'m mut self, swapchain_count: u32) -> &'m mut PresentTimesInfoGoogleBuilder<'b> {
        self
    }

    pub fn times<'m, 'a>(&'m mut self, times: &'a PresentTimeGoogle) -> &'m mut PresentTimesInfoGoogleBuilder<'b> {
        self
    }

}


/// A `VkPresentTimeGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PresentTimeGoogle {
    raw: vks::VkPresentTimeGOOGLE,
}

impl PresentTimeGoogle {
    pub fn builder() -> PresentTimeGoogleBuilder {
        PresentTimeGoogleBuilder::new()
    }

    pub fn present_id(&self) {
    }

    pub fn desired_present_time(&self) {
    }

    pub fn raw(&self) -> &vks::VkPresentTimeGOOGLE {
        &self.raw
    }
}


/// A builder for `VkPresentTimeGOOGLE`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentTimeGoogleBuilder {
    raw: vks::VkPresentTimeGOOGLE,
}

impl PresentTimeGoogleBuilder {
    pub fn new() -> PresentTimeGoogleBuilder {
        PresentTimeGoogleBuilder {
            raw: vks::VkPresentTimeGOOGLE::default(),
        }
    }

    pub fn present_id<'m>(&'m mut self, present_id: u32) -> &'m mut PresentTimeGoogleBuilder {
        self
    }

    pub fn desired_present_time<'m>(&'m mut self, desired_present_time: u64) -> &'m mut PresentTimeGoogleBuilder {
        self
    }

}


/// A `VkIOSSurfaceCreateInfoMVK`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct IosSurfaceCreateInfoMvk<'s> {
    raw: vks::VkIOSSurfaceCreateInfoMVK,
    _p: PhantomData<&'s ()>,
}

impl<'s> IosSurfaceCreateInfoMvk<'s> {
    pub fn builder<'b>() -> IosSurfaceCreateInfoMvkBuilder<'b> {
        IosSurfaceCreateInfoMvkBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn view(&self) {
    }

    pub fn raw(&self) -> &vks::VkIOSSurfaceCreateInfoMVK {
        &self.raw
    }
}


/// A builder for `VkIOSSurfaceCreateInfoMVK`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct IosSurfaceCreateInfoMvkBuilder<'b> {
    raw: vks::VkIOSSurfaceCreateInfoMVK,
    _p: PhantomData<&'b ()>,
}

impl<'b> IosSurfaceCreateInfoMvkBuilder<'b> {
    pub fn new() -> IosSurfaceCreateInfoMvkBuilder<'b> {
        IosSurfaceCreateInfoMvkBuilder {
            raw: vks::VkIOSSurfaceCreateInfoMVK::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut IosSurfaceCreateInfoMvkBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: IosSurfaceCreateFlagsMvk) -> &'m mut IosSurfaceCreateInfoMvkBuilder<'b> {
        self
    }

    pub fn view<'m, 'a>(&'m mut self, view: &'a ()) -> &'m mut IosSurfaceCreateInfoMvkBuilder<'b> {
        self
    }

}


/// A `VkMacOSSurfaceCreateInfoMVK`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MacOsSurfaceCreateInfoMvk<'s> {
    raw: vks::VkMacOSSurfaceCreateInfoMVK,
    _p: PhantomData<&'s ()>,
}

impl<'s> MacOsSurfaceCreateInfoMvk<'s> {
    pub fn builder<'b>() -> MacOsSurfaceCreateInfoMvkBuilder<'b> {
        MacOsSurfaceCreateInfoMvkBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn view(&self) {
    }

    pub fn raw(&self) -> &vks::VkMacOSSurfaceCreateInfoMVK {
        &self.raw
    }
}


/// A builder for `VkMacOSSurfaceCreateInfoMVK`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MacOsSurfaceCreateInfoMvkBuilder<'b> {
    raw: vks::VkMacOSSurfaceCreateInfoMVK,
    _p: PhantomData<&'b ()>,
}

impl<'b> MacOsSurfaceCreateInfoMvkBuilder<'b> {
    pub fn new() -> MacOsSurfaceCreateInfoMvkBuilder<'b> {
        MacOsSurfaceCreateInfoMvkBuilder {
            raw: vks::VkMacOSSurfaceCreateInfoMVK::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MacOsSurfaceCreateInfoMvkBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: MacOsSurfaceCreateFlagsMvk) -> &'m mut MacOsSurfaceCreateInfoMvkBuilder<'b> {
        self
    }

    pub fn view<'m, 'a>(&'m mut self, view: &'a ()) -> &'m mut MacOsSurfaceCreateInfoMvkBuilder<'b> {
        self
    }

}


/// A `VkViewportWScalingNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ViewportWScalingNv {
    raw: vks::VkViewportWScalingNV,
}

impl ViewportWScalingNv {
    pub fn builder() -> ViewportWScalingNvBuilder {
        ViewportWScalingNvBuilder::new()
    }

    pub fn xcoeff(&self) {
    }

    pub fn ycoeff(&self) {
    }

    pub fn raw(&self) -> &vks::VkViewportWScalingNV {
        &self.raw
    }
}


/// A builder for `VkViewportWScalingNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ViewportWScalingNvBuilder {
    raw: vks::VkViewportWScalingNV,
}

impl ViewportWScalingNvBuilder {
    pub fn new() -> ViewportWScalingNvBuilder {
        ViewportWScalingNvBuilder {
            raw: vks::VkViewportWScalingNV::default(),
        }
    }

    pub fn xcoeff<'m>(&'m mut self, xcoeff: f32) -> &'m mut ViewportWScalingNvBuilder {
        self
    }

    pub fn ycoeff<'m>(&'m mut self, ycoeff: f32) -> &'m mut ViewportWScalingNvBuilder {
        self
    }

}


/// A `VkPipelineViewportWScalingStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNv<'s> {
    raw: vks::VkPipelineViewportWScalingStateCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineViewportWScalingStateCreateInfoNv<'s> {
    pub fn builder<'b>() -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        PipelineViewportWScalingStateCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn viewport_wscaling_enable(&self) {
    }

    pub fn viewport_count(&self) {
    }

    pub fn viewport_wscalings(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineViewportWScalingStateCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkPipelineViewportWScalingStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
    raw: vks::VkPipelineViewportWScalingStateCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
    pub fn new() -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        PipelineViewportWScalingStateCreateInfoNvBuilder {
            raw: vks::VkPipelineViewportWScalingStateCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn viewport_wscaling_enable<'m>(&'m mut self, viewport_wscaling_enable: bool) -> &'m mut PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn viewport_count<'m>(&'m mut self, viewport_count: u32) -> &'m mut PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn viewport_wscalings<'m, 'a>(&'m mut self, viewport_wscalings: &'a ViewportWScalingNv) -> &'m mut PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkViewportSwizzleNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ViewportSwizzleNv {
    raw: vks::VkViewportSwizzleNV,
}

impl ViewportSwizzleNv {
    pub fn builder() -> ViewportSwizzleNvBuilder {
        ViewportSwizzleNvBuilder::new()
    }

    pub fn x(&self) {
    }

    pub fn y(&self) {
    }

    pub fn z(&self) {
    }

    pub fn w(&self) {
    }

    pub fn raw(&self) -> &vks::VkViewportSwizzleNV {
        &self.raw
    }
}


/// A builder for `VkViewportSwizzleNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ViewportSwizzleNvBuilder {
    raw: vks::VkViewportSwizzleNV,
}

impl ViewportSwizzleNvBuilder {
    pub fn new() -> ViewportSwizzleNvBuilder {
        ViewportSwizzleNvBuilder {
            raw: vks::VkViewportSwizzleNV::default(),
        }
    }

    pub fn x<'m>(&'m mut self, x: ViewportCoordinateSwizzleNv) -> &'m mut ViewportSwizzleNvBuilder {
        self
    }

    pub fn y<'m>(&'m mut self, y: ViewportCoordinateSwizzleNv) -> &'m mut ViewportSwizzleNvBuilder {
        self
    }

    pub fn z<'m>(&'m mut self, z: ViewportCoordinateSwizzleNv) -> &'m mut ViewportSwizzleNvBuilder {
        self
    }

    pub fn w<'m>(&'m mut self, w: ViewportCoordinateSwizzleNv) -> &'m mut ViewportSwizzleNvBuilder {
        self
    }

}


/// A `VkPipelineViewportSwizzleStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNv<'s> {
    raw: vks::VkPipelineViewportSwizzleStateCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineViewportSwizzleStateCreateInfoNv<'s> {
    pub fn builder<'b>() -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        PipelineViewportSwizzleStateCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn viewport_count(&self) {
    }

    pub fn viewport_swizzles(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineViewportSwizzleStateCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkPipelineViewportSwizzleStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
    raw: vks::VkPipelineViewportSwizzleStateCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
    pub fn new() -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        PipelineViewportSwizzleStateCreateInfoNvBuilder {
            raw: vks::VkPipelineViewportSwizzleStateCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineViewportSwizzleStateCreateFlagsNv) -> &'m mut PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn viewport_count<'m>(&'m mut self, viewport_count: u32) -> &'m mut PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn viewport_swizzles<'m, 'a>(&'m mut self, viewport_swizzles: &'a ViewportSwizzleNv) -> &'m mut PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceDiscardRectanglePropertiesEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesExt<'s> {
    raw: vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceDiscardRectanglePropertiesExt<'s> {
    pub fn builder<'b>() -> PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
        PhysicalDeviceDiscardRectanglePropertiesExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn max_discard_rectangles(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceDiscardRectanglePropertiesEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
    raw: vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
    pub fn new() -> PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
        PhysicalDeviceDiscardRectanglePropertiesExtBuilder {
            raw: vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
        self
    }

    pub fn max_discard_rectangles<'m>(&'m mut self, max_discard_rectangles: u32) -> &'m mut PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
        self
    }

}


/// A `VkPipelineDiscardRectangleStateCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoExt<'s> {
    raw: vks::VkPipelineDiscardRectangleStateCreateInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineDiscardRectangleStateCreateInfoExt<'s> {
    pub fn builder<'b>() -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        PipelineDiscardRectangleStateCreateInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn discard_rectangle_mode(&self) {
    }

    pub fn discard_rectangle_count(&self) {
    }

    pub fn discard_rectangles(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineDiscardRectangleStateCreateInfoEXT {
        &self.raw
    }
}


/// A builder for `VkPipelineDiscardRectangleStateCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
    raw: vks::VkPipelineDiscardRectangleStateCreateInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
    pub fn new() -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        PipelineDiscardRectangleStateCreateInfoExtBuilder {
            raw: vks::VkPipelineDiscardRectangleStateCreateInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineDiscardRectangleStateCreateFlagsExt) -> &'m mut PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn discard_rectangle_mode<'m>(&'m mut self, discard_rectangle_mode: DiscardRectangleModeExt) -> &'m mut PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn discard_rectangle_count<'m>(&'m mut self, discard_rectangle_count: u32) -> &'m mut PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn discard_rectangles<'m, 'a>(&'m mut self, discard_rectangles: &'a Rect2d) -> &'m mut PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx<'s> {
    raw: vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn per_view_position_all_components(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        &self.raw
    }
}


/// A `VkPhysicalDeviceSurfaceInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2Khr<'s> {
    raw: vks::VkPhysicalDeviceSurfaceInfo2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceSurfaceInfo2Khr<'s> {
    pub fn builder<'b>() -> PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
        PhysicalDeviceSurfaceInfo2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn surface(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceSurfaceInfo2KHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceSurfaceInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
    raw: vks::VkPhysicalDeviceSurfaceInfo2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
    pub fn new() -> PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
        PhysicalDeviceSurfaceInfo2KhrBuilder {
            raw: vks::VkPhysicalDeviceSurfaceInfo2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
        self
    }

    pub fn surface<'m>(&'m mut self, surface: SurfaceKhr) -> &'m mut PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
        self
    }

}


/// A `VkSurfaceCapabilities2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SurfaceCapabilities2Khr<'s> {
    raw: vks::VkSurfaceCapabilities2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SurfaceCapabilities2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn surface_capabilities(&self) {
    }

    pub fn raw(&self) -> &vks::VkSurfaceCapabilities2KHR {
        &self.raw
    }
}


/// A `VkSurfaceFormat2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SurfaceFormat2Khr<'s> {
    raw: vks::VkSurfaceFormat2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SurfaceFormat2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn surface_format(&self) {
    }

    pub fn raw(&self) -> &vks::VkSurfaceFormat2KHR {
        &self.raw
    }
}


/// A `VkSharedPresentSurfaceCapabilitiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKhr<'s> {
    raw: vks::VkSharedPresentSurfaceCapabilitiesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SharedPresentSurfaceCapabilitiesKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn shared_present_supported_usage_flags(&self) {
    }

    pub fn raw(&self) -> &vks::VkSharedPresentSurfaceCapabilitiesKHR {
        &self.raw
    }
}


/// A `VkPhysicalDevice16BitStorageFeaturesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeaturesKhr<'s> {
    raw: vks::VkPhysicalDevice16BitStorageFeaturesKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDevice16BitStorageFeaturesKhr<'s> {
    pub fn builder<'b>() -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        PhysicalDevice16BitStorageFeaturesKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn storage_buffer_16_bit_access(&self) {
    }

    pub fn uniform_and_storage_buffer_16_bit_access(&self) {
    }

    pub fn storage_push_constant_16(&self) {
    }

    pub fn storage_input_output_16(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDevice16BitStorageFeaturesKHR {
        &self.raw
    }
}


/// A builder for `VkPhysicalDevice16BitStorageFeaturesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
    raw: vks::VkPhysicalDevice16BitStorageFeaturesKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
    pub fn new() -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        PhysicalDevice16BitStorageFeaturesKhrBuilder {
            raw: vks::VkPhysicalDevice16BitStorageFeaturesKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self
    }

    pub fn storage_buffer_16_bit_access<'m>(&'m mut self, storage_buffer_16_bit_access: bool) -> &'m mut PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self
    }

    pub fn uniform_and_storage_buffer_16_bit_access<'m>(&'m mut self, uniform_and_storage_buffer_16_bit_access: bool) -> &'m mut PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self
    }

    pub fn storage_push_constant_16<'m>(&'m mut self, storage_push_constant_16: bool) -> &'m mut PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self
    }

    pub fn storage_input_output_16<'m>(&'m mut self, storage_input_output_16: bool) -> &'m mut PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self
    }

}


/// A `VkBufferMemoryRequirementsInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2Khr<'s> {
    raw: vks::VkBufferMemoryRequirementsInfo2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> BufferMemoryRequirementsInfo2Khr<'s> {
    pub fn builder<'b>() -> BufferMemoryRequirementsInfo2KhrBuilder<'b> {
        BufferMemoryRequirementsInfo2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn raw(&self) -> &vks::VkBufferMemoryRequirementsInfo2KHR {
        &self.raw
    }
}


/// A builder for `VkBufferMemoryRequirementsInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BufferMemoryRequirementsInfo2KhrBuilder<'b> {
    raw: vks::VkBufferMemoryRequirementsInfo2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> BufferMemoryRequirementsInfo2KhrBuilder<'b> {
    pub fn new() -> BufferMemoryRequirementsInfo2KhrBuilder<'b> {
        BufferMemoryRequirementsInfo2KhrBuilder {
            raw: vks::VkBufferMemoryRequirementsInfo2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut BufferMemoryRequirementsInfo2KhrBuilder<'b> {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut BufferMemoryRequirementsInfo2KhrBuilder<'b> {
        self
    }

}


/// A `VkImageMemoryRequirementsInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2Khr<'s> {
    raw: vks::VkImageMemoryRequirementsInfo2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImageMemoryRequirementsInfo2Khr<'s> {
    pub fn builder<'b>() -> ImageMemoryRequirementsInfo2KhrBuilder<'b> {
        ImageMemoryRequirementsInfo2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn image(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageMemoryRequirementsInfo2KHR {
        &self.raw
    }
}


/// A builder for `VkImageMemoryRequirementsInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageMemoryRequirementsInfo2KhrBuilder<'b> {
    raw: vks::VkImageMemoryRequirementsInfo2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageMemoryRequirementsInfo2KhrBuilder<'b> {
    pub fn new() -> ImageMemoryRequirementsInfo2KhrBuilder<'b> {
        ImageMemoryRequirementsInfo2KhrBuilder {
            raw: vks::VkImageMemoryRequirementsInfo2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImageMemoryRequirementsInfo2KhrBuilder<'b> {
        self
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut ImageMemoryRequirementsInfo2KhrBuilder<'b> {
        self
    }

}


/// A `VkImageSparseMemoryRequirementsInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2Khr<'s> {
    raw: vks::VkImageSparseMemoryRequirementsInfo2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> ImageSparseMemoryRequirementsInfo2Khr<'s> {
    pub fn builder<'b>() -> ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
        ImageSparseMemoryRequirementsInfo2KhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn image(&self) {
    }

    pub fn raw(&self) -> &vks::VkImageSparseMemoryRequirementsInfo2KHR {
        &self.raw
    }
}


/// A builder for `VkImageSparseMemoryRequirementsInfo2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
    raw: vks::VkImageSparseMemoryRequirementsInfo2KHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
    pub fn new() -> ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
        ImageSparseMemoryRequirementsInfo2KhrBuilder {
            raw: vks::VkImageSparseMemoryRequirementsInfo2KHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
        self
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
        self
    }

}


/// A `VkMemoryRequirements2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryRequirements2Khr<'s> {
    raw: vks::VkMemoryRequirements2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryRequirements2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn memory_requirements(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryRequirements2KHR {
        &self.raw
    }
}


/// A `VkSparseImageMemoryRequirements2KHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SparseImageMemoryRequirements2Khr<'s> {
    raw: vks::VkSparseImageMemoryRequirements2KHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> SparseImageMemoryRequirements2Khr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn memory_requirements(&self) {
    }

    pub fn raw(&self) -> &vks::VkSparseImageMemoryRequirements2KHR {
        &self.raw
    }
}


/// A `VkMemoryDedicatedRequirementsKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryDedicatedRequirementsKhr<'s> {
    raw: vks::VkMemoryDedicatedRequirementsKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryDedicatedRequirementsKhr<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn prefers_dedicated_allocation(&self) {
    }

    pub fn requires_dedicated_allocation(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryDedicatedRequirementsKHR {
        &self.raw
    }
}


/// A `VkMemoryDedicatedAllocateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfoKhr<'s> {
    raw: vks::VkMemoryDedicatedAllocateInfoKHR,
    _p: PhantomData<&'s ()>,
}

impl<'s> MemoryDedicatedAllocateInfoKhr<'s> {
    pub fn builder<'b>() -> MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        MemoryDedicatedAllocateInfoKhrBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn image(&self) {
    }

    pub fn buffer(&self) {
    }

    pub fn raw(&self) -> &vks::VkMemoryDedicatedAllocateInfoKHR {
        &self.raw
    }
}


/// A builder for `VkMemoryDedicatedAllocateInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct MemoryDedicatedAllocateInfoKhrBuilder<'b> {
    raw: vks::VkMemoryDedicatedAllocateInfoKHR,
    _p: PhantomData<&'b ()>,
}

impl<'b> MemoryDedicatedAllocateInfoKhrBuilder<'b> {
    pub fn new() -> MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        MemoryDedicatedAllocateInfoKhrBuilder {
            raw: vks::VkMemoryDedicatedAllocateInfoKHR::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        self
    }

    pub fn image<'m>(&'m mut self, image: Image) -> &'m mut MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        self
    }

    pub fn buffer<'m>(&'m mut self, buffer: Buffer) -> &'m mut MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        self
    }

}


/// A `VkTextureLODGatherFormatPropertiesAMD`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAmd<'s> {
    raw: vks::VkTextureLODGatherFormatPropertiesAMD,
    _p: PhantomData<&'s ()>,
}

impl<'s> TextureLODGatherFormatPropertiesAmd<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn supports_texture_gather_lo_dbias_am_d(&self) {
    }

    pub fn raw(&self) -> &vks::VkTextureLODGatherFormatPropertiesAMD {
        &self.raw
    }
}


/// A `VkPipelineCoverageToColorStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNv<'s> {
    raw: vks::VkPipelineCoverageToColorStateCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineCoverageToColorStateCreateInfoNv<'s> {
    pub fn builder<'b>() -> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        PipelineCoverageToColorStateCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn coverage_to_color_enable(&self) {
    }

    pub fn coverage_to_color_location(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineCoverageToColorStateCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkPipelineCoverageToColorStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
    raw: vks::VkPipelineCoverageToColorStateCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
    pub fn new() -> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        PipelineCoverageToColorStateCreateInfoNvBuilder {
            raw: vks::VkPipelineCoverageToColorStateCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineCoverageToColorStateCreateFlagsNv) -> &'m mut PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn coverage_to_color_enable<'m>(&'m mut self, coverage_to_color_enable: bool) -> &'m mut PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn coverage_to_color_location<'m>(&'m mut self, coverage_to_color_location: u32) -> &'m mut PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxPropertiesExt<'s> {
    raw: vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceSamplerFilterMinmaxPropertiesExt<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn filter_minmax_single_component_formats(&self) {
    }

    pub fn filter_minmax_image_component_mapping(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
        &self.raw
    }
}


/// A `VkSamplerReductionModeCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SamplerReductionModeCreateInfoExt<'s> {
    raw: vks::VkSamplerReductionModeCreateInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> SamplerReductionModeCreateInfoExt<'s> {
    pub fn builder<'b>() -> SamplerReductionModeCreateInfoExtBuilder<'b> {
        SamplerReductionModeCreateInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn reduction_mode(&self) {
    }

    pub fn raw(&self) -> &vks::VkSamplerReductionModeCreateInfoEXT {
        &self.raw
    }
}


/// A builder for `VkSamplerReductionModeCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SamplerReductionModeCreateInfoExtBuilder<'b> {
    raw: vks::VkSamplerReductionModeCreateInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> SamplerReductionModeCreateInfoExtBuilder<'b> {
    pub fn new() -> SamplerReductionModeCreateInfoExtBuilder<'b> {
        SamplerReductionModeCreateInfoExtBuilder {
            raw: vks::VkSamplerReductionModeCreateInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut SamplerReductionModeCreateInfoExtBuilder<'b> {
        self
    }

    pub fn reduction_mode<'m>(&'m mut self, reduction_mode: SamplerReductionModeExt) -> &'m mut SamplerReductionModeCreateInfoExtBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesExt<'s> {
    raw: vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceBlendOperationAdvancedFeaturesExt<'s> {
    pub fn builder<'b>() -> PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
        PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn advanced_blend_coherent_operations(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        &self.raw
    }
}


/// A builder for `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
    raw: vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
    pub fn new() -> PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
        PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder {
            raw: vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
        self
    }

    pub fn advanced_blend_coherent_operations<'m>(&'m mut self, advanced_blend_coherent_operations: bool) -> &'m mut PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
        self
    }

}


/// A `VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesExt<'s> {
    raw: vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> PhysicalDeviceBlendOperationAdvancedPropertiesExt<'s> {
    pub unsafe fn next(&self) {
    }

    pub fn advanced_blend_max_color_attachments(&self) {
    }

    pub fn advanced_blend_independent_blend(&self) {
    }

    pub fn advanced_blend_non_premultiplied_src_color(&self) {
    }

    pub fn advanced_blend_non_premultiplied_dst_color(&self) {
    }

    pub fn advanced_blend_correlated_overlap(&self) {
    }

    pub fn advanced_blend_all_operations(&self) {
    }

    pub fn raw(&self) -> &vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        &self.raw
    }
}


/// A `VkPipelineColorBlendAdvancedStateCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoExt<'s> {
    raw: vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineColorBlendAdvancedStateCreateInfoExt<'s> {
    pub fn builder<'b>() -> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        PipelineColorBlendAdvancedStateCreateInfoExtBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn src_premultiplied(&self) {
    }

    pub fn dst_premultiplied(&self) {
    }

    pub fn blend_overlap(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT {
        &self.raw
    }
}


/// A builder for `VkPipelineColorBlendAdvancedStateCreateInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
    raw: vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
    pub fn new() -> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        PipelineColorBlendAdvancedStateCreateInfoExtBuilder {
            raw: vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn src_premultiplied<'m>(&'m mut self, src_premultiplied: bool) -> &'m mut PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn dst_premultiplied<'m>(&'m mut self, dst_premultiplied: bool) -> &'m mut PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self
    }

    pub fn blend_overlap<'m>(&'m mut self, blend_overlap: BlendOverlapExt) -> &'m mut PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self
    }

}


/// A `VkPipelineCoverageModulationStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNv<'s> {
    raw: vks::VkPipelineCoverageModulationStateCreateInfoNV,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineCoverageModulationStateCreateInfoNv<'s> {
    pub fn builder<'b>() -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        PipelineCoverageModulationStateCreateInfoNvBuilder::new()
    }

    pub unsafe fn next(&self) {
    }

    pub fn flags(&self) {
    }

    pub fn coverage_modulation_mode(&self) {
    }

    pub fn coverage_modulation_table_enable(&self) {
    }

    pub fn coverage_modulation_table_count(&self) {
    }

    pub fn coverage_modulation_table(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineCoverageModulationStateCreateInfoNV {
        &self.raw
    }
}


/// A builder for `VkPipelineCoverageModulationStateCreateInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
    raw: vks::VkPipelineCoverageModulationStateCreateInfoNV,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
    pub fn new() -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        PipelineCoverageModulationStateCreateInfoNvBuilder {
            raw: vks::VkPipelineCoverageModulationStateCreateInfoNV::default(),
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m, 'a>(&'m mut self, next: &'a ()) -> &'m mut PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn flags<'m>(&'m mut self, flags: PipelineCoverageModulationStateCreateFlagsNv) -> &'m mut PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn coverage_modulation_mode<'m>(&'m mut self, coverage_modulation_mode: CoverageModulationModeNv) -> &'m mut PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn coverage_modulation_table_enable<'m>(&'m mut self, coverage_modulation_table_enable: bool) -> &'m mut PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn coverage_modulation_table_count<'m>(&'m mut self, coverage_modulation_table_count: u32) -> &'m mut PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self
    }

    pub fn coverage_modulation_table<'m, 'a>(&'m mut self, coverage_modulation_table: &'a f32) -> &'m mut PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self
    }

}


