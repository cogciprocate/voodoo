//! Structs.

#![allow(unused_mut)]

use std::ptr;
use std::ffi::{CString, CStr};
use std::marker::PhantomData;
use libc::c_void;
use num_traits::ToPrimitive;
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


impl From<Offset2d> for vks::VkOffset2D {
    fn from(f: Offset2d) -> vks::VkOffset2D {
        f.raw
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

    pub fn x<'m>(mut self, x: i32) -> Offset2dBuilder {
        self.raw.x = x.into();
        self
    }

    pub fn y<'m>(mut self, y: i32) -> Offset2dBuilder {
        self.raw.y = y.into();
        self
    }

    pub fn build(self) -> Offset2d {
        Offset2d {
            raw: self.raw,
        }
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


impl From<Offset3d> for vks::VkOffset3D {
    fn from(f: Offset3d) -> vks::VkOffset3D {
        f.raw
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

    pub fn x<'m>(mut self, x: i32) -> Offset3dBuilder {
        self.raw.x = x.into();
        self
    }

    pub fn y<'m>(mut self, y: i32) -> Offset3dBuilder {
        self.raw.y = y.into();
        self
    }

    pub fn z<'m>(mut self, z: i32) -> Offset3dBuilder {
        self.raw.z = z.into();
        self
    }

    pub fn build(self) -> Offset3d {
        Offset3d {
            raw: self.raw,
        }
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


impl From<Extent2d> for vks::VkExtent2D {
    fn from(f: Extent2d) -> vks::VkExtent2D {
        f.raw
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

    pub fn width<'m>(mut self, width: u32) -> Extent2dBuilder {
        self.raw.width = width.into();
        self
    }

    pub fn height<'m>(mut self, height: u32) -> Extent2dBuilder {
        self.raw.height = height.into();
        self
    }

    pub fn build(self) -> Extent2d {
        Extent2d {
            raw: self.raw,
        }
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


impl From<Extent3d> for vks::VkExtent3D {
    fn from(f: Extent3d) -> vks::VkExtent3D {
        f.raw
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

    pub fn width<'m>(mut self, width: u32) -> Extent3dBuilder {
        self.raw.width = width.into();
        self
    }

    pub fn height<'m>(mut self, height: u32) -> Extent3dBuilder {
        self.raw.height = height.into();
        self
    }

    pub fn depth<'m>(mut self, depth: u32) -> Extent3dBuilder {
        self.raw.depth = depth.into();
        self
    }

    pub fn build(self) -> Extent3d {
        Extent3d {
            raw: self.raw,
        }
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


impl From<Viewport> for vks::VkViewport {
    fn from(f: Viewport) -> vks::VkViewport {
        f.raw
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

    pub fn x<'m>(mut self, x: f32) -> ViewportBuilder {
        self.raw.x = x.into();
        self
    }

    pub fn y<'m>(mut self, y: f32) -> ViewportBuilder {
        self.raw.y = y.into();
        self
    }

    pub fn width<'m>(mut self, width: f32) -> ViewportBuilder {
        self.raw.width = width.into();
        self
    }

    pub fn height<'m>(mut self, height: f32) -> ViewportBuilder {
        self.raw.height = height.into();
        self
    }

    pub fn min_depth<'m>(mut self, min_depth: f32) -> ViewportBuilder {
        self.raw.minDepth = min_depth.into();
        self
    }

    pub fn max_depth<'m>(mut self, max_depth: f32) -> ViewportBuilder {
        self.raw.maxDepth = max_depth.into();
        self
    }

    pub fn build(self) -> Viewport {
        Viewport {
            raw: self.raw,
        }
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


impl From<Rect2d> for vks::VkRect2D {
    fn from(f: Rect2d) -> vks::VkRect2D {
        f.raw
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

    pub fn offset<'m>(mut self, offset: Offset2d) -> Rect2dBuilder {
        self.raw.offset = offset.raw;
        self
    }

    pub fn extent<'m>(mut self, extent: Extent2d) -> Rect2dBuilder {
        self.raw.extent = extent.raw;
        self
    }

    pub fn build(self) -> Rect2d {
        Rect2d {
            raw: self.raw,
        }
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


impl From<ClearRect> for vks::VkClearRect {
    fn from(f: ClearRect) -> vks::VkClearRect {
        f.raw
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

    pub fn rect<'m>(mut self, rect: Rect2d) -> ClearRectBuilder {
        self.raw.rect = rect.raw;
        self
    }

    pub fn base_array_layer<'m>(mut self, base_array_layer: u32) -> ClearRectBuilder {
        self.raw.baseArrayLayer = base_array_layer.into();
        self
    }

    pub fn layer_count<'m>(mut self, layer_count: u32) -> ClearRectBuilder {
        self.raw.layerCount = layer_count.into();
        self
    }

    pub fn build(self) -> ClearRect {
        ClearRect {
            raw: self.raw,
        }
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


impl From<ComponentMapping> for vks::VkComponentMapping {
    fn from(f: ComponentMapping) -> vks::VkComponentMapping {
        f.raw
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

    pub fn r<'m>(mut self, r: ComponentSwizzle) -> ComponentMappingBuilder {
        self.raw.r = r.into();
        self
    }

    pub fn g<'m>(mut self, g: ComponentSwizzle) -> ComponentMappingBuilder {
        self.raw.g = g.into();
        self
    }

    pub fn b<'m>(mut self, b: ComponentSwizzle) -> ComponentMappingBuilder {
        self.raw.b = b.into();
        self
    }

    pub fn a<'m>(mut self, a: ComponentSwizzle) -> ComponentMappingBuilder {
        self.raw.a = a.into();
        self
    }

    pub fn build(self) -> ComponentMapping {
        ComponentMapping {
            raw: self.raw,
        }
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


impl From<PhysicalDeviceProperties> for vks::VkPhysicalDeviceProperties {
    fn from(f: PhysicalDeviceProperties) -> vks::VkPhysicalDeviceProperties {
        f.raw
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


impl From<ExtensionProperties> for vks::VkExtensionProperties {
    fn from(f: ExtensionProperties) -> vks::VkExtensionProperties {
        f.raw
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


impl From<LayerProperties> for vks::VkLayerProperties {
    fn from(f: LayerProperties) -> vks::VkLayerProperties {
        f.raw
    }
}


/// A `VkApplicationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ApplicationInfo<'s> {
    raw: vks::VkApplicationInfo,
    application_name: Option<CharStr<'s>>,
    engine_name: Option<CharStr<'s>>,
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


impl<'s> From<ApplicationInfo<'s>> for vks::VkApplicationInfo {
    fn from(f: ApplicationInfo<'s>) -> vks::VkApplicationInfo {
        f.raw
    }
}


/// A builder for `VkApplicationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ApplicationInfoBuilder<'b> {
    raw: vks::VkApplicationInfo,
    application_name: Option<CharStr<'b>>,
    engine_name: Option<CharStr<'b>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> ApplicationInfoBuilder<'b> {
    pub fn new() -> ApplicationInfoBuilder<'b> {
        ApplicationInfoBuilder {
            raw: vks::VkApplicationInfo::default(),
            application_name: None,
            engine_name: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ApplicationInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn application_name<'m, 'a, T>(mut self, application_name: T) -> ApplicationInfoBuilder<'b> where 'a: 'b, T: Into<CharStr<'a>> {
        self.application_name = Some(application_name.into());
        self.raw.pApplicationName = self.application_name.as_ref().unwrap().as_ptr();
        self
    }

    pub fn application_version<'m, T>(mut self, application_version: T) -> ApplicationInfoBuilder<'b> where T: Into<Version> {
        self.raw.applicationVersion = application_version.into().into();
        self
    }

    pub fn engine_name<'m, 'a, T>(mut self, engine_name: T) -> ApplicationInfoBuilder<'b> where 'a: 'b, T: Into<CharStr<'a>> {
        self.engine_name = Some(engine_name.into());
        self.raw.pEngineName = self.engine_name.as_ref().unwrap().as_ptr();
        self
    }

    pub fn engine_version<'m, T>(mut self, engine_version: T) -> ApplicationInfoBuilder<'b> where T: Into<Version> {
        self.raw.engineVersion = engine_version.into().into();
        self
    }

    pub fn api_version<'m, T>(mut self, api_version: T) -> ApplicationInfoBuilder<'b> where T: Into<Version> {
        self.raw.apiVersion = api_version.into().into();
        self
    }

    pub fn build(self) -> ApplicationInfo<'b> {
        ApplicationInfo {
            raw: self.raw,
            application_name: self.application_name,
            engine_name: self.engine_name,
            _p: PhantomData,
        }
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

    pub unsafe fn user_data(&self) {
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


impl<'s> From<AllocationCallbacks<'s>> for vks::VkAllocationCallbacks {
    fn from(f: AllocationCallbacks<'s>) -> vks::VkAllocationCallbacks {
        f.raw
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

    pub unsafe fn user_data<'m>(mut self, user_data: *mut c_void) -> AllocationCallbacksBuilder<'b> {
        self.raw.pUserData = user_data;
        self
    }

    pub fn pfn_allocation<'m>(mut self, pfn_allocation: PFN_vkAllocationFunction) -> AllocationCallbacksBuilder<'b> {
        self.raw.pfnAllocation = pfn_allocation.into();
        self
    }

    pub fn pfn_reallocation<'m>(mut self, pfn_reallocation: PFN_vkReallocationFunction) -> AllocationCallbacksBuilder<'b> {
        self.raw.pfnReallocation = pfn_reallocation.into();
        self
    }

    pub fn pfn_free<'m>(mut self, pfn_free: PFN_vkFreeFunction) -> AllocationCallbacksBuilder<'b> {
        self.raw.pfnFree = pfn_free.into();
        self
    }

    pub fn pfn_internal_allocation<'m>(mut self, pfn_internal_allocation: PFN_vkInternalAllocationNotification) -> AllocationCallbacksBuilder<'b> {
        self.raw.pfnInternalAllocation = pfn_internal_allocation.into();
        self
    }

    pub fn pfn_internal_free<'m>(mut self, pfn_internal_free: PFN_vkInternalFreeNotification) -> AllocationCallbacksBuilder<'b> {
        self.raw.pfnInternalFree = pfn_internal_free.into();
        self
    }

    pub fn build(self) -> AllocationCallbacks<'b> {
        AllocationCallbacks {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<DeviceQueueCreateInfo<'s>> for vks::VkDeviceQueueCreateInfo {
    fn from(f: DeviceQueueCreateInfo<'s>) -> vks::VkDeviceQueueCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceQueueCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DeviceQueueCreateFlags) -> DeviceQueueCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn queue_family_index<'m>(mut self, queue_family_index: u32) -> DeviceQueueCreateInfoBuilder<'b> {
        self.raw.queueFamilyIndex = queue_family_index.into();
        self
    }

    pub fn queue_count<'m>(mut self, queue_count: u32) -> DeviceQueueCreateInfoBuilder<'b> {
        self.raw.queueCount = queue_count.into();
        self
    }

    pub fn queue_priorities<'m, 'a>(mut self, queue_priorities: &'a [f32]) -> DeviceQueueCreateInfoBuilder<'b> {
        self.raw.pQueuePriorities = queue_priorities.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> DeviceQueueCreateInfo<'b> {
        DeviceQueueCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkDeviceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DeviceCreateInfo<'s> {
    raw: vks::VkDeviceCreateInfo,
    enabled_extension_names: Option<CharStrs<'s>>,
    enabled_layer_names: Option<CharStrs<'s>>,
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


impl<'s> From<DeviceCreateInfo<'s>> for vks::VkDeviceCreateInfo {
    fn from(f: DeviceCreateInfo<'s>) -> vks::VkDeviceCreateInfo {
        f.raw
    }
}


/// A builder for `VkDeviceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DeviceCreateInfoBuilder<'b> {
    raw: vks::VkDeviceCreateInfo,
    enabled_extension_names: Option<CharStrs<'b>>,
    enabled_layer_names: Option<CharStrs<'b>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DeviceCreateInfoBuilder<'b> {
    pub fn new() -> DeviceCreateInfoBuilder<'b> {
        DeviceCreateInfoBuilder {
            raw: vks::VkDeviceCreateInfo::default(),
            enabled_extension_names: None,
            enabled_layer_names: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DeviceCreateFlags) -> DeviceCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn queue_create_info_count<'m>(mut self, queue_create_info_count: u32) -> DeviceCreateInfoBuilder<'b> {
        self.raw.queueCreateInfoCount = queue_create_info_count.into();
        self
    }

    pub fn queue_create_infos<'m, 'a>(mut self, queue_create_infos: &'a [DeviceQueueCreateInfo]) -> DeviceCreateInfoBuilder<'b> {
        self.raw.pQueueCreateInfos = queue_create_infos.as_ptr() as *const _;
        self
    }

    pub fn enabled_layer_count<'m>(mut self, enabled_layer_count: u32) -> DeviceCreateInfoBuilder<'b> {
        self.raw.enabledLayerCount = enabled_layer_count.into();
        self
    }

    pub fn enabled_layer_names<'m, 'a, T>(mut self, enabled_layer_names: T) -> DeviceCreateInfoBuilder<'b> where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        self.raw.ppEnabledLayerNames = self.enabled_layer_names.as_ref().unwrap().as_ptr();
        self
    }

    pub fn enabled_extension_count<'m>(mut self, enabled_extension_count: u32) -> DeviceCreateInfoBuilder<'b> {
        self.raw.enabledExtensionCount = enabled_extension_count.into();
        self
    }

    pub fn enabled_extension_names<'m, 'a, T>(mut self, enabled_extension_names: T) -> DeviceCreateInfoBuilder<'b> where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        self.raw.ppEnabledExtensionNames = self.enabled_extension_names.as_ref().unwrap().as_ptr();
        self
    }

    pub fn enabled_features<'m, 'a>(mut self, enabled_features: &'a [PhysicalDeviceFeatures]) -> DeviceCreateInfoBuilder<'b> {
        self.raw.pEnabledFeatures = enabled_features.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> DeviceCreateInfo<'b> {
        DeviceCreateInfo {
            raw: self.raw,
            enabled_extension_names: self.enabled_extension_names,
            enabled_layer_names: self.enabled_layer_names,
            _p: PhantomData,
        }
    }

}


/// A `VkInstanceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct InstanceCreateInfo<'s> {
    raw: vks::VkInstanceCreateInfo,
    enabled_layer_names: Option<CharStrs<'s>>,
    enabled_extension_names: Option<CharStrs<'s>>,
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


impl<'s> From<InstanceCreateInfo<'s>> for vks::VkInstanceCreateInfo {
    fn from(f: InstanceCreateInfo<'s>) -> vks::VkInstanceCreateInfo {
        f.raw
    }
}


/// A builder for `VkInstanceCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct InstanceCreateInfoBuilder<'b> {
    raw: vks::VkInstanceCreateInfo,
    enabled_layer_names: Option<CharStrs<'b>>,
    enabled_extension_names: Option<CharStrs<'b>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> InstanceCreateInfoBuilder<'b> {
    pub fn new() -> InstanceCreateInfoBuilder<'b> {
        InstanceCreateInfoBuilder {
            raw: vks::VkInstanceCreateInfo::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> InstanceCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: InstanceCreateFlags) -> InstanceCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn application_info<'m, 'a>(mut self, application_info: &'a ApplicationInfo) -> InstanceCreateInfoBuilder<'b> {
        self.raw.pApplicationInfo = application_info.raw();
        self
    }

    pub fn enabled_layer_count<'m>(mut self, enabled_layer_count: u32) -> InstanceCreateInfoBuilder<'b> {
        self.raw.enabledLayerCount = enabled_layer_count.into();
        self
    }

    pub fn enabled_layer_names<'m, 'a, T>(mut self, enabled_layer_names: T) -> InstanceCreateInfoBuilder<'b> where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        self.raw.ppEnabledLayerNames = self.enabled_layer_names.as_ref().unwrap().as_ptr();
        self
    }

    pub fn enabled_extension_count<'m>(mut self, enabled_extension_count: u32) -> InstanceCreateInfoBuilder<'b> {
        self.raw.enabledExtensionCount = enabled_extension_count.into();
        self
    }

    pub fn enabled_extension_names<'m, 'a, T>(mut self, enabled_extension_names: T) -> InstanceCreateInfoBuilder<'b> where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        self.raw.ppEnabledExtensionNames = self.enabled_extension_names.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> InstanceCreateInfo<'b> {
        InstanceCreateInfo {
            raw: self.raw,
            enabled_layer_names: self.enabled_layer_names,
            enabled_extension_names: self.enabled_extension_names,
            _p: PhantomData,
        }
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


impl From<QueueFamilyProperties> for vks::VkQueueFamilyProperties {
    fn from(f: QueueFamilyProperties) -> vks::VkQueueFamilyProperties {
        f.raw
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


impl From<PhysicalDeviceMemoryProperties> for vks::VkPhysicalDeviceMemoryProperties {
    fn from(f: PhysicalDeviceMemoryProperties) -> vks::VkPhysicalDeviceMemoryProperties {
        f.raw
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


impl<'s> From<MemoryAllocateInfo<'s>> for vks::VkMemoryAllocateInfo {
    fn from(f: MemoryAllocateInfo<'s>) -> vks::VkMemoryAllocateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MemoryAllocateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn allocation_size<'m>(mut self, allocation_size: u64) -> MemoryAllocateInfoBuilder<'b> {
        self.raw.allocationSize = allocation_size.into();
        self
    }

    pub fn memory_type_index<'m>(mut self, memory_type_index: u32) -> MemoryAllocateInfoBuilder<'b> {
        self.raw.memoryTypeIndex = memory_type_index.into();
        self
    }

    pub fn build(self) -> MemoryAllocateInfo<'b> {
        MemoryAllocateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<MemoryRequirements> for vks::VkMemoryRequirements {
    fn from(f: MemoryRequirements) -> vks::VkMemoryRequirements {
        f.raw
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


impl From<SparseImageFormatProperties> for vks::VkSparseImageFormatProperties {
    fn from(f: SparseImageFormatProperties) -> vks::VkSparseImageFormatProperties {
        f.raw
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


impl From<SparseImageMemoryRequirements> for vks::VkSparseImageMemoryRequirements {
    fn from(f: SparseImageMemoryRequirements) -> vks::VkSparseImageMemoryRequirements {
        f.raw
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


impl From<MemoryType> for vks::VkMemoryType {
    fn from(f: MemoryType) -> vks::VkMemoryType {
        f.raw
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


impl From<MemoryHeap> for vks::VkMemoryHeap {
    fn from(f: MemoryHeap) -> vks::VkMemoryHeap {
        f.raw
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


impl<'s> From<MappedMemoryRange<'s>> for vks::VkMappedMemoryRange {
    fn from(f: MappedMemoryRange<'s>) -> vks::VkMappedMemoryRange {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MappedMemoryRangeBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn memory<'m, 'a>(mut self, memory: &'a DeviceMemory) -> MappedMemoryRangeBuilder<'b> {
        self.raw.memory = memory.handle();
        self
    }

    pub fn offset<'m>(mut self, offset: u64) -> MappedMemoryRangeBuilder<'b> {
        self.raw.offset = offset.into();
        self
    }

    pub fn size<'m>(mut self, size: u64) -> MappedMemoryRangeBuilder<'b> {
        self.raw.size = size.into();
        self
    }

    pub fn build(self) -> MappedMemoryRange<'b> {
        MappedMemoryRange {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<FormatProperties> for vks::VkFormatProperties {
    fn from(f: FormatProperties) -> vks::VkFormatProperties {
        f.raw
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


impl From<ImageFormatProperties> for vks::VkImageFormatProperties {
    fn from(f: ImageFormatProperties) -> vks::VkImageFormatProperties {
        f.raw
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


impl From<DescriptorBufferInfo> for vks::VkDescriptorBufferInfo {
    fn from(f: DescriptorBufferInfo) -> vks::VkDescriptorBufferInfo {
        f.raw
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

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> DescriptorBufferInfoBuilder {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn offset<'m>(mut self, offset: u64) -> DescriptorBufferInfoBuilder {
        self.raw.offset = offset.into();
        self
    }

    pub fn range<'m>(mut self, range: u64) -> DescriptorBufferInfoBuilder {
        self.raw.range = range.into();
        self
    }

    pub fn build(self) -> DescriptorBufferInfo {
        DescriptorBufferInfo {
            raw: self.raw,
        }
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


impl From<DescriptorImageInfo> for vks::VkDescriptorImageInfo {
    fn from(f: DescriptorImageInfo) -> vks::VkDescriptorImageInfo {
        f.raw
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

    pub fn sampler<'m, 'a>(mut self, sampler: &'a Sampler) -> DescriptorImageInfoBuilder {
        self.raw.sampler = sampler.handle();
        self
    }

    pub fn image_view<'m, 'a>(mut self, image_view: &'a ImageView) -> DescriptorImageInfoBuilder {
        self.raw.imageView = image_view.handle();
        self
    }

    pub fn image_layout<'m>(mut self, image_layout: ImageLayout) -> DescriptorImageInfoBuilder {
        self.raw.imageLayout = image_layout.into();
        self
    }

    pub fn build(self) -> DescriptorImageInfo {
        DescriptorImageInfo {
            raw: self.raw,
        }
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


impl<'s> From<WriteDescriptorSet<'s>> for vks::VkWriteDescriptorSet {
    fn from(f: WriteDescriptorSet<'s>) -> vks::VkWriteDescriptorSet {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> WriteDescriptorSetBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn dst_set<'m, 'a>(mut self, dst_set: &'a DescriptorSet) -> WriteDescriptorSetBuilder<'b> {
        self.raw.dstSet = dst_set.handle();
        self
    }

    pub fn dst_binding<'m>(mut self, dst_binding: u32) -> WriteDescriptorSetBuilder<'b> {
        self.raw.dstBinding = dst_binding.into();
        self
    }

    pub fn dst_array_element<'m>(mut self, dst_array_element: u32) -> WriteDescriptorSetBuilder<'b> {
        self.raw.dstArrayElement = dst_array_element.into();
        self
    }

    pub fn descriptor_count<'m>(mut self, descriptor_count: u32) -> WriteDescriptorSetBuilder<'b> {
        self.raw.descriptorCount = descriptor_count.into();
        self
    }

    pub fn descriptor_type<'m>(mut self, descriptor_type: DescriptorType) -> WriteDescriptorSetBuilder<'b> {
        self.raw.descriptorType = descriptor_type.into();
        self
    }

    pub fn image_info<'m, 'a>(mut self, image_info: &'a DescriptorImageInfo) -> WriteDescriptorSetBuilder<'b> {
        self.raw.pImageInfo = image_info.raw();
        self
    }

    pub fn buffer_info<'m, 'a>(mut self, buffer_info: &'a DescriptorBufferInfo) -> WriteDescriptorSetBuilder<'b> {
        self.raw.pBufferInfo = buffer_info.raw();
        self
    }

    pub fn texel_buffer_view<'m, 'a>(mut self, texel_buffer_view: &'a BufferView) -> WriteDescriptorSetBuilder<'b> {
        self.raw.pTexelBufferView = &texel_buffer_view.handle();
        self
    }

    pub fn build(self) -> WriteDescriptorSet<'b> {
        WriteDescriptorSet {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<CopyDescriptorSet<'s>> for vks::VkCopyDescriptorSet {
    fn from(f: CopyDescriptorSet<'s>) -> vks::VkCopyDescriptorSet {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CopyDescriptorSetBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn src_set<'m, 'a>(mut self, src_set: &'a DescriptorSet) -> CopyDescriptorSetBuilder<'b> {
        self.raw.srcSet = src_set.handle();
        self
    }

    pub fn src_binding<'m>(mut self, src_binding: u32) -> CopyDescriptorSetBuilder<'b> {
        self.raw.srcBinding = src_binding.into();
        self
    }

    pub fn src_array_element<'m>(mut self, src_array_element: u32) -> CopyDescriptorSetBuilder<'b> {
        self.raw.srcArrayElement = src_array_element.into();
        self
    }

    pub fn dst_set<'m, 'a>(mut self, dst_set: &'a DescriptorSet) -> CopyDescriptorSetBuilder<'b> {
        self.raw.dstSet = dst_set.handle();
        self
    }

    pub fn dst_binding<'m>(mut self, dst_binding: u32) -> CopyDescriptorSetBuilder<'b> {
        self.raw.dstBinding = dst_binding.into();
        self
    }

    pub fn dst_array_element<'m>(mut self, dst_array_element: u32) -> CopyDescriptorSetBuilder<'b> {
        self.raw.dstArrayElement = dst_array_element.into();
        self
    }

    pub fn descriptor_count<'m>(mut self, descriptor_count: u32) -> CopyDescriptorSetBuilder<'b> {
        self.raw.descriptorCount = descriptor_count.into();
        self
    }

    pub fn build(self) -> CopyDescriptorSet<'b> {
        CopyDescriptorSet {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<BufferCreateInfo<'s>> for vks::VkBufferCreateInfo {
    fn from(f: BufferCreateInfo<'s>) -> vks::VkBufferCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BufferCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: BufferCreateFlags) -> BufferCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn size<'m>(mut self, size: u64) -> BufferCreateInfoBuilder<'b> {
        self.raw.size = size.into();
        self
    }

    pub fn usage<'m>(mut self, usage: BufferUsageFlags) -> BufferCreateInfoBuilder<'b> {
        self.raw.usage = usage.bits();
        self
    }

    pub fn sharing_mode<'m>(mut self, sharing_mode: SharingMode) -> BufferCreateInfoBuilder<'b> {
        self.raw.sharingMode = sharing_mode.into();
        self
    }

    pub fn queue_family_index_count<'m>(mut self, queue_family_index_count: u32) -> BufferCreateInfoBuilder<'b> {
        self.raw.queueFamilyIndexCount = queue_family_index_count.into();
        self
    }

    pub fn queue_family_indices<'m, 'a>(mut self, queue_family_indices: &'a [u32]) -> BufferCreateInfoBuilder<'b> {
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> BufferCreateInfo<'b> {
        BufferCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<BufferViewCreateInfo<'s>> for vks::VkBufferViewCreateInfo {
    fn from(f: BufferViewCreateInfo<'s>) -> vks::VkBufferViewCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BufferViewCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: BufferViewCreateFlags) -> BufferViewCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> BufferViewCreateInfoBuilder<'b> {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn format<'m>(mut self, format: Format) -> BufferViewCreateInfoBuilder<'b> {
        self.raw.format = format.into();
        self
    }

    pub fn offset<'m>(mut self, offset: u64) -> BufferViewCreateInfoBuilder<'b> {
        self.raw.offset = offset.into();
        self
    }

    pub fn range<'m>(mut self, range: u64) -> BufferViewCreateInfoBuilder<'b> {
        self.raw.range = range.into();
        self
    }

    pub fn build(self) -> BufferViewCreateInfo<'b> {
        BufferViewCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<ImageSubresource> for vks::VkImageSubresource {
    fn from(f: ImageSubresource) -> vks::VkImageSubresource {
        f.raw
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

    pub fn aspect_mask<'m>(mut self, aspect_mask: ImageAspectFlags) -> ImageSubresourceBuilder {
        self.raw.aspectMask = aspect_mask.bits();
        self
    }

    pub fn mip_level<'m>(mut self, mip_level: u32) -> ImageSubresourceBuilder {
        self.raw.mipLevel = mip_level.into();
        self
    }

    pub fn array_layer<'m>(mut self, array_layer: u32) -> ImageSubresourceBuilder {
        self.raw.arrayLayer = array_layer.into();
        self
    }

    pub fn build(self) -> ImageSubresource {
        ImageSubresource {
            raw: self.raw,
        }
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


impl From<ImageSubresourceLayers> for vks::VkImageSubresourceLayers {
    fn from(f: ImageSubresourceLayers) -> vks::VkImageSubresourceLayers {
        f.raw
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

    pub fn aspect_mask<'m>(mut self, aspect_mask: ImageAspectFlags) -> ImageSubresourceLayersBuilder {
        self.raw.aspectMask = aspect_mask.bits();
        self
    }

    pub fn mip_level<'m>(mut self, mip_level: u32) -> ImageSubresourceLayersBuilder {
        self.raw.mipLevel = mip_level.into();
        self
    }

    pub fn base_array_layer<'m>(mut self, base_array_layer: u32) -> ImageSubresourceLayersBuilder {
        self.raw.baseArrayLayer = base_array_layer.into();
        self
    }

    pub fn layer_count<'m>(mut self, layer_count: u32) -> ImageSubresourceLayersBuilder {
        self.raw.layerCount = layer_count.into();
        self
    }

    pub fn build(self) -> ImageSubresourceLayers {
        ImageSubresourceLayers {
            raw: self.raw,
        }
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


impl From<ImageSubresourceRange> for vks::VkImageSubresourceRange {
    fn from(f: ImageSubresourceRange) -> vks::VkImageSubresourceRange {
        f.raw
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

    pub fn aspect_mask<'m>(mut self, aspect_mask: ImageAspectFlags) -> ImageSubresourceRangeBuilder {
        self.raw.aspectMask = aspect_mask.bits();
        self
    }

    pub fn base_mip_level<'m>(mut self, base_mip_level: u32) -> ImageSubresourceRangeBuilder {
        self.raw.baseMipLevel = base_mip_level.into();
        self
    }

    pub fn level_count<'m>(mut self, level_count: u32) -> ImageSubresourceRangeBuilder {
        self.raw.levelCount = level_count.into();
        self
    }

    pub fn base_array_layer<'m>(mut self, base_array_layer: u32) -> ImageSubresourceRangeBuilder {
        self.raw.baseArrayLayer = base_array_layer.into();
        self
    }

    pub fn layer_count<'m>(mut self, layer_count: u32) -> ImageSubresourceRangeBuilder {
        self.raw.layerCount = layer_count.into();
        self
    }

    pub fn build(self) -> ImageSubresourceRange {
        ImageSubresourceRange {
            raw: self.raw,
        }
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


impl<'s> From<MemoryBarrier<'s>> for vks::VkMemoryBarrier {
    fn from(f: MemoryBarrier<'s>) -> vks::VkMemoryBarrier {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MemoryBarrierBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn src_access_mask<'m>(mut self, src_access_mask: AccessFlags) -> MemoryBarrierBuilder<'b> {
        self.raw.srcAccessMask = src_access_mask.bits();
        self
    }

    pub fn dst_access_mask<'m>(mut self, dst_access_mask: AccessFlags) -> MemoryBarrierBuilder<'b> {
        self.raw.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub fn build(self) -> MemoryBarrier<'b> {
        MemoryBarrier {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<BufferMemoryBarrier<'s>> for vks::VkBufferMemoryBarrier {
    fn from(f: BufferMemoryBarrier<'s>) -> vks::VkBufferMemoryBarrier {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn src_access_mask<'m>(mut self, src_access_mask: AccessFlags) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.srcAccessMask = src_access_mask.bits();
        self
    }

    pub fn dst_access_mask<'m>(mut self, dst_access_mask: AccessFlags) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub fn src_queue_family_index<'m>(mut self, src_queue_family_index: u32) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.srcQueueFamilyIndex = src_queue_family_index.into();
        self
    }

    pub fn dst_queue_family_index<'m>(mut self, dst_queue_family_index: u32) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.dstQueueFamilyIndex = dst_queue_family_index.into();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn offset<'m>(mut self, offset: u64) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.offset = offset.into();
        self
    }

    pub fn size<'m>(mut self, size: u64) -> BufferMemoryBarrierBuilder<'b> {
        self.raw.size = size.into();
        self
    }

    pub fn build(self) -> BufferMemoryBarrier<'b> {
        BufferMemoryBarrier {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImageMemoryBarrier<'s>> for vks::VkImageMemoryBarrier {
    fn from(f: ImageMemoryBarrier<'s>) -> vks::VkImageMemoryBarrier {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn src_access_mask<'m>(mut self, src_access_mask: AccessFlags) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.srcAccessMask = src_access_mask.bits();
        self
    }

    pub fn dst_access_mask<'m>(mut self, dst_access_mask: AccessFlags) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub fn old_layout<'m>(mut self, old_layout: ImageLayout) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.oldLayout = old_layout.into();
        self
    }

    pub fn new_layout<'m>(mut self, new_layout: ImageLayout) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.newLayout = new_layout.into();
        self
    }

    pub fn src_queue_family_index<'m>(mut self, src_queue_family_index: u32) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.srcQueueFamilyIndex = src_queue_family_index.into();
        self
    }

    pub fn dst_queue_family_index<'m>(mut self, dst_queue_family_index: u32) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.dstQueueFamilyIndex = dst_queue_family_index.into();
        self
    }

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn subresource_range<'m>(mut self, subresource_range: ImageSubresourceRange) -> ImageMemoryBarrierBuilder<'b> {
        self.raw.subresourceRange = subresource_range.raw;
        self
    }

    pub fn build(self) -> ImageMemoryBarrier<'b> {
        ImageMemoryBarrier {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImageCreateInfo<'s>> for vks::VkImageCreateInfo {
    fn from(f: ImageCreateInfo<'s>) -> vks::VkImageCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImageCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: ImageCreateFlags) -> ImageCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn image_type<'m>(mut self, image_type: ImageType) -> ImageCreateInfoBuilder<'b> {
        self.raw.imageType = image_type.into();
        self
    }

    pub fn format<'m>(mut self, format: Format) -> ImageCreateInfoBuilder<'b> {
        self.raw.format = format.into();
        self
    }

    pub fn extent<'m>(mut self, extent: Extent3d) -> ImageCreateInfoBuilder<'b> {
        self.raw.extent = extent.raw;
        self
    }

    pub fn mip_levels<'m>(mut self, mip_levels: u32) -> ImageCreateInfoBuilder<'b> {
        self.raw.mipLevels = mip_levels.into();
        self
    }

    pub fn array_layers<'m>(mut self, array_layers: u32) -> ImageCreateInfoBuilder<'b> {
        self.raw.arrayLayers = array_layers.into();
        self
    }

    pub fn samples<'m>(mut self, samples: SampleCountFlags) -> ImageCreateInfoBuilder<'b> {
        self.raw.samples = samples.bits();
        self
    }

    pub fn tiling<'m>(mut self, tiling: ImageTiling) -> ImageCreateInfoBuilder<'b> {
        self.raw.tiling = tiling.into();
        self
    }

    pub fn usage<'m>(mut self, usage: ImageUsageFlags) -> ImageCreateInfoBuilder<'b> {
        self.raw.usage = usage.bits();
        self
    }

    pub fn sharing_mode<'m>(mut self, sharing_mode: SharingMode) -> ImageCreateInfoBuilder<'b> {
        self.raw.sharingMode = sharing_mode.into();
        self
    }

    pub fn queue_family_index_count<'m>(mut self, queue_family_index_count: u32) -> ImageCreateInfoBuilder<'b> {
        self.raw.queueFamilyIndexCount = queue_family_index_count.into();
        self
    }

    pub fn queue_family_indices<'m, 'a>(mut self, queue_family_indices: &'a [u32]) -> ImageCreateInfoBuilder<'b> {
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const _;
        self
    }

    pub fn initial_layout<'m>(mut self, initial_layout: ImageLayout) -> ImageCreateInfoBuilder<'b> {
        self.raw.initialLayout = initial_layout.into();
        self
    }

    pub fn build(self) -> ImageCreateInfo<'b> {
        ImageCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<SubresourceLayout> for vks::VkSubresourceLayout {
    fn from(f: SubresourceLayout) -> vks::VkSubresourceLayout {
        f.raw
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


impl<'s> From<ImageViewCreateInfo<'s>> for vks::VkImageViewCreateInfo {
    fn from(f: ImageViewCreateInfo<'s>) -> vks::VkImageViewCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: ImageViewCreateFlags) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn view_type<'m>(mut self, view_type: ImageViewType) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.viewType = view_type.into();
        self
    }

    pub fn format<'m>(mut self, format: Format) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.format = format.into();
        self
    }

    pub fn components<'m>(mut self, components: ComponentMapping) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.components = components.raw;
        self
    }

    pub fn subresource_range<'m>(mut self, subresource_range: ImageSubresourceRange) -> ImageViewCreateInfoBuilder<'b> {
        self.raw.subresourceRange = subresource_range.raw;
        self
    }

    pub fn build(self) -> ImageViewCreateInfo<'b> {
        ImageViewCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<BufferCopy> for vks::VkBufferCopy {
    fn from(f: BufferCopy) -> vks::VkBufferCopy {
        f.raw
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

    pub fn src_offset<'m>(mut self, src_offset: u64) -> BufferCopyBuilder {
        self.raw.srcOffset = src_offset.into();
        self
    }

    pub fn dst_offset<'m>(mut self, dst_offset: u64) -> BufferCopyBuilder {
        self.raw.dstOffset = dst_offset.into();
        self
    }

    pub fn size<'m>(mut self, size: u64) -> BufferCopyBuilder {
        self.raw.size = size.into();
        self
    }

    pub fn build(self) -> BufferCopy {
        BufferCopy {
            raw: self.raw,
        }
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


impl From<SparseMemoryBind> for vks::VkSparseMemoryBind {
    fn from(f: SparseMemoryBind) -> vks::VkSparseMemoryBind {
        f.raw
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

    pub fn resource_offset<'m>(mut self, resource_offset: u64) -> SparseMemoryBindBuilder {
        self.raw.resourceOffset = resource_offset.into();
        self
    }

    pub fn size<'m>(mut self, size: u64) -> SparseMemoryBindBuilder {
        self.raw.size = size.into();
        self
    }

    pub fn memory<'m, 'a>(mut self, memory: &'a DeviceMemory) -> SparseMemoryBindBuilder {
        self.raw.memory = memory.handle();
        self
    }

    pub fn memory_offset<'m>(mut self, memory_offset: u64) -> SparseMemoryBindBuilder {
        self.raw.memoryOffset = memory_offset.into();
        self
    }

    pub fn flags<'m>(mut self, flags: SparseMemoryBindFlags) -> SparseMemoryBindBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> SparseMemoryBind {
        SparseMemoryBind {
            raw: self.raw,
        }
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


impl From<SparseImageMemoryBind> for vks::VkSparseImageMemoryBind {
    fn from(f: SparseImageMemoryBind) -> vks::VkSparseImageMemoryBind {
        f.raw
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

    pub fn subresource<'m>(mut self, subresource: ImageSubresource) -> SparseImageMemoryBindBuilder {
        self.raw.subresource = subresource.raw;
        self
    }

    pub fn offset<'m>(mut self, offset: Offset3d) -> SparseImageMemoryBindBuilder {
        self.raw.offset = offset.raw;
        self
    }

    pub fn extent<'m>(mut self, extent: Extent3d) -> SparseImageMemoryBindBuilder {
        self.raw.extent = extent.raw;
        self
    }

    pub fn memory<'m, 'a>(mut self, memory: &'a DeviceMemory) -> SparseImageMemoryBindBuilder {
        self.raw.memory = memory.handle();
        self
    }

    pub fn memory_offset<'m>(mut self, memory_offset: u64) -> SparseImageMemoryBindBuilder {
        self.raw.memoryOffset = memory_offset.into();
        self
    }

    pub fn flags<'m>(mut self, flags: SparseMemoryBindFlags) -> SparseImageMemoryBindBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> SparseImageMemoryBind {
        SparseImageMemoryBind {
            raw: self.raw,
        }
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


impl<'s> From<SparseBufferMemoryBindInfo<'s>> for vks::VkSparseBufferMemoryBindInfo {
    fn from(f: SparseBufferMemoryBindInfo<'s>) -> vks::VkSparseBufferMemoryBindInfo {
        f.raw
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

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> SparseBufferMemoryBindInfoBuilder<'b> {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn bind_count<'m>(mut self, bind_count: u32) -> SparseBufferMemoryBindInfoBuilder<'b> {
        self.raw.bindCount = bind_count.into();
        self
    }

    pub fn binds<'m, 'a>(mut self, binds: &'a [SparseMemoryBind]) -> SparseBufferMemoryBindInfoBuilder<'b> {
        self.raw.pBinds = binds.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> SparseBufferMemoryBindInfo<'b> {
        SparseBufferMemoryBindInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SparseImageOpaqueMemoryBindInfo<'s>> for vks::VkSparseImageOpaqueMemoryBindInfo {
    fn from(f: SparseImageOpaqueMemoryBindInfo<'s>) -> vks::VkSparseImageOpaqueMemoryBindInfo {
        f.raw
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

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn bind_count<'m>(mut self, bind_count: u32) -> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        self.raw.bindCount = bind_count.into();
        self
    }

    pub fn binds<'m, 'a>(mut self, binds: &'a [SparseMemoryBind]) -> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        self.raw.pBinds = binds.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> SparseImageOpaqueMemoryBindInfo<'b> {
        SparseImageOpaqueMemoryBindInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SparseImageMemoryBindInfo<'s>> for vks::VkSparseImageMemoryBindInfo {
    fn from(f: SparseImageMemoryBindInfo<'s>) -> vks::VkSparseImageMemoryBindInfo {
        f.raw
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

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> SparseImageMemoryBindInfoBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn bind_count<'m>(mut self, bind_count: u32) -> SparseImageMemoryBindInfoBuilder<'b> {
        self.raw.bindCount = bind_count.into();
        self
    }

    pub fn binds<'m, 'a>(mut self, binds: &'a [SparseImageMemoryBind]) -> SparseImageMemoryBindInfoBuilder<'b> {
        self.raw.pBinds = binds.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> SparseImageMemoryBindInfo<'b> {
        SparseImageMemoryBindInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkBindSparseInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BindSparseInfo<'s> {
    raw: vks::VkBindSparseInfo,
    wait_semaphores: Option<Vec<vks::VkSemaphore>>,
    signal_semaphores: Option<Vec<vks::VkSemaphore>>,
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


impl<'s> From<BindSparseInfo<'s>> for vks::VkBindSparseInfo {
    fn from(f: BindSparseInfo<'s>) -> vks::VkBindSparseInfo {
        f.raw
    }
}


/// A builder for `VkBindSparseInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BindSparseInfoBuilder<'b> {
    raw: vks::VkBindSparseInfo,
    wait_semaphores: Option<Vec<vks::VkSemaphore>>,
    signal_semaphores: Option<Vec<vks::VkSemaphore>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> BindSparseInfoBuilder<'b> {
    pub fn new() -> BindSparseInfoBuilder<'b> {
        BindSparseInfoBuilder {
            raw: vks::VkBindSparseInfo::default(),
            wait_semaphores: None,
            signal_semaphores: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BindSparseInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphore_count<'m>(mut self, wait_semaphore_count: u32) -> BindSparseInfoBuilder<'b> {
        self.raw.waitSemaphoreCount = wait_semaphore_count.into();
        self
    }

    pub fn wait_semaphores<'m, 'a>(mut self, wait_semaphores: &'a [Semaphore]) -> BindSparseInfoBuilder<'b> where 'a: 'b {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        self.raw.pWaitSemaphores = self.wait_semaphores.as_ref().unwrap().as_ptr();
        self
    }

    pub fn buffer_bind_count<'m>(mut self, buffer_bind_count: u32) -> BindSparseInfoBuilder<'b> {
        self.raw.bufferBindCount = buffer_bind_count.into();
        self
    }

    pub fn buffer_binds<'m, 'a>(mut self, buffer_binds: &'a [SparseBufferMemoryBindInfo]) -> BindSparseInfoBuilder<'b> {
        self.raw.pBufferBinds = buffer_binds.as_ptr() as *const _;
        self
    }

    pub fn image_opaque_bind_count<'m>(mut self, image_opaque_bind_count: u32) -> BindSparseInfoBuilder<'b> {
        self.raw.imageOpaqueBindCount = image_opaque_bind_count.into();
        self
    }

    pub fn image_opaque_binds<'m, 'a>(mut self, image_opaque_binds: &'a [SparseImageOpaqueMemoryBindInfo]) -> BindSparseInfoBuilder<'b> {
        self.raw.pImageOpaqueBinds = image_opaque_binds.as_ptr() as *const _;
        self
    }

    pub fn image_bind_count<'m>(mut self, image_bind_count: u32) -> BindSparseInfoBuilder<'b> {
        self.raw.imageBindCount = image_bind_count.into();
        self
    }

    pub fn image_binds<'m, 'a>(mut self, image_binds: &'a [SparseImageMemoryBindInfo]) -> BindSparseInfoBuilder<'b> {
        self.raw.pImageBinds = image_binds.as_ptr() as *const _;
        self
    }

    pub fn signal_semaphore_count<'m>(mut self, signal_semaphore_count: u32) -> BindSparseInfoBuilder<'b> {
        self.raw.signalSemaphoreCount = signal_semaphore_count.into();
        self
    }

    pub fn signal_semaphores<'m, 'a>(mut self, signal_semaphores: &'a [Semaphore]) -> BindSparseInfoBuilder<'b> where 'a: 'b {
        self.signal_semaphores = Some(signal_semaphores.iter().map(|h| h.handle()).collect());
        self.raw.pSignalSemaphores = self.signal_semaphores.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> BindSparseInfo<'b> {
        BindSparseInfo {
            raw: self.raw,
            wait_semaphores: self.wait_semaphores,
            signal_semaphores: self.signal_semaphores,
            _p: PhantomData,
        }
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


impl From<ImageCopy> for vks::VkImageCopy {
    fn from(f: ImageCopy) -> vks::VkImageCopy {
        f.raw
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

    pub fn src_subresource<'m>(mut self, src_subresource: ImageSubresourceLayers) -> ImageCopyBuilder {
        self.raw.srcSubresource = src_subresource.raw;
        self
    }

    pub fn src_offset<'m>(mut self, src_offset: Offset3d) -> ImageCopyBuilder {
        self.raw.srcOffset = src_offset.raw;
        self
    }

    pub fn dst_subresource<'m>(mut self, dst_subresource: ImageSubresourceLayers) -> ImageCopyBuilder {
        self.raw.dstSubresource = dst_subresource.raw;
        self
    }

    pub fn dst_offset<'m>(mut self, dst_offset: Offset3d) -> ImageCopyBuilder {
        self.raw.dstOffset = dst_offset.raw;
        self
    }

    pub fn extent<'m>(mut self, extent: Extent3d) -> ImageCopyBuilder {
        self.raw.extent = extent.raw;
        self
    }

    pub fn build(self) -> ImageCopy {
        ImageCopy {
            raw: self.raw,
        }
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


impl From<ImageBlit> for vks::VkImageBlit {
    fn from(f: ImageBlit) -> vks::VkImageBlit {
        f.raw
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

    pub fn src_subresource<'m>(mut self, src_subresource: ImageSubresourceLayers) -> ImageBlitBuilder {
        self.raw.srcSubresource = src_subresource.raw;
        self
    }

    pub fn src_offsets<'m>(mut self, src_offsets: [Offset3d; 2]) -> ImageBlitBuilder {
        self.raw.srcOffsets = [src_offsets[0].raw, src_offsets[1].raw, ];
        self
    }

    pub fn dst_subresource<'m>(mut self, dst_subresource: ImageSubresourceLayers) -> ImageBlitBuilder {
        self.raw.dstSubresource = dst_subresource.raw;
        self
    }

    pub fn dst_offsets<'m>(mut self, dst_offsets: [Offset3d; 2]) -> ImageBlitBuilder {
        self.raw.dstOffsets = [dst_offsets[0].raw, dst_offsets[1].raw, ];
        self
    }

    pub fn build(self) -> ImageBlit {
        ImageBlit {
            raw: self.raw,
        }
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


impl From<BufferImageCopy> for vks::VkBufferImageCopy {
    fn from(f: BufferImageCopy) -> vks::VkBufferImageCopy {
        f.raw
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

    pub fn buffer_offset<'m>(mut self, buffer_offset: u64) -> BufferImageCopyBuilder {
        self.raw.bufferOffset = buffer_offset.into();
        self
    }

    pub fn buffer_row_length<'m>(mut self, buffer_row_length: u32) -> BufferImageCopyBuilder {
        self.raw.bufferRowLength = buffer_row_length.into();
        self
    }

    pub fn buffer_image_height<'m>(mut self, buffer_image_height: u32) -> BufferImageCopyBuilder {
        self.raw.bufferImageHeight = buffer_image_height.into();
        self
    }

    pub fn image_subresource<'m>(mut self, image_subresource: ImageSubresourceLayers) -> BufferImageCopyBuilder {
        self.raw.imageSubresource = image_subresource.raw;
        self
    }

    pub fn image_offset<'m>(mut self, image_offset: Offset3d) -> BufferImageCopyBuilder {
        self.raw.imageOffset = image_offset.raw;
        self
    }

    pub fn image_extent<'m>(mut self, image_extent: Extent3d) -> BufferImageCopyBuilder {
        self.raw.imageExtent = image_extent.raw;
        self
    }

    pub fn build(self) -> BufferImageCopy {
        BufferImageCopy {
            raw: self.raw,
        }
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


impl From<ImageResolve> for vks::VkImageResolve {
    fn from(f: ImageResolve) -> vks::VkImageResolve {
        f.raw
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

    pub fn src_subresource<'m>(mut self, src_subresource: ImageSubresourceLayers) -> ImageResolveBuilder {
        self.raw.srcSubresource = src_subresource.raw;
        self
    }

    pub fn src_offset<'m>(mut self, src_offset: Offset3d) -> ImageResolveBuilder {
        self.raw.srcOffset = src_offset.raw;
        self
    }

    pub fn dst_subresource<'m>(mut self, dst_subresource: ImageSubresourceLayers) -> ImageResolveBuilder {
        self.raw.dstSubresource = dst_subresource.raw;
        self
    }

    pub fn dst_offset<'m>(mut self, dst_offset: Offset3d) -> ImageResolveBuilder {
        self.raw.dstOffset = dst_offset.raw;
        self
    }

    pub fn extent<'m>(mut self, extent: Extent3d) -> ImageResolveBuilder {
        self.raw.extent = extent.raw;
        self
    }

    pub fn build(self) -> ImageResolve {
        ImageResolve {
            raw: self.raw,
        }
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


impl<'s> From<ShaderModuleCreateInfo<'s>> for vks::VkShaderModuleCreateInfo {
    fn from(f: ShaderModuleCreateInfo<'s>) -> vks::VkShaderModuleCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ShaderModuleCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: ShaderModuleCreateFlags) -> ShaderModuleCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn code_size<'m>(mut self, code_size: usize) -> ShaderModuleCreateInfoBuilder<'b> {
        self.raw.codeSize = code_size.into();
        self
    }

    pub fn code<'m, 'a>(mut self, code: &'a [u32]) -> ShaderModuleCreateInfoBuilder<'b> {
        self.raw.pCode = code.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> ShaderModuleCreateInfo<'b> {
        ShaderModuleCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkDescriptorSetLayoutBinding`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutBinding<'s> {
    raw: vks::VkDescriptorSetLayoutBinding,
    immutable_samplers: Option<Vec<vks::VkSampler>>,
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


impl<'s> From<DescriptorSetLayoutBinding<'s>> for vks::VkDescriptorSetLayoutBinding {
    fn from(f: DescriptorSetLayoutBinding<'s>) -> vks::VkDescriptorSetLayoutBinding {
        f.raw
    }
}


/// A builder for `VkDescriptorSetLayoutBinding`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutBindingBuilder<'b> {
    raw: vks::VkDescriptorSetLayoutBinding,
    immutable_samplers: Option<Vec<vks::VkSampler>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetLayoutBindingBuilder<'b> {
    pub fn new() -> DescriptorSetLayoutBindingBuilder<'b> {
        DescriptorSetLayoutBindingBuilder {
            raw: vks::VkDescriptorSetLayoutBinding::default(),
            immutable_samplers: None,
            _p: PhantomData,
        }
    }

    pub fn binding<'m>(mut self, binding: u32) -> DescriptorSetLayoutBindingBuilder<'b> {
        self.raw.binding = binding.into();
        self
    }

    pub fn descriptor_type<'m>(mut self, descriptor_type: DescriptorType) -> DescriptorSetLayoutBindingBuilder<'b> {
        self.raw.descriptorType = descriptor_type.into();
        self
    }

    pub fn descriptor_count<'m>(mut self, descriptor_count: u32) -> DescriptorSetLayoutBindingBuilder<'b> {
        self.raw.descriptorCount = descriptor_count.into();
        self
    }

    pub fn stage_flags<'m>(mut self, stage_flags: ShaderStageFlags) -> DescriptorSetLayoutBindingBuilder<'b> {
        self.raw.stageFlags = stage_flags.bits();
        self
    }

    pub fn immutable_samplers<'m, 'a>(mut self, immutable_samplers: &'a [Sampler]) -> DescriptorSetLayoutBindingBuilder<'b> where 'a: 'b {
        self.immutable_samplers = Some(immutable_samplers.iter().map(|h| h.handle()).collect());
        self.raw.pImmutableSamplers = self.immutable_samplers.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> DescriptorSetLayoutBinding<'b> {
        DescriptorSetLayoutBinding {
            raw: self.raw,
            immutable_samplers: self.immutable_samplers,
            _p: PhantomData,
        }
    }

}


/// A `VkDescriptorSetLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutCreateInfo<'s> {
    raw: vks::VkDescriptorSetLayoutCreateInfo,
    bindings: Option<Vec<vks::VkDescriptorSetLayoutBinding>>,
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


impl<'s> From<DescriptorSetLayoutCreateInfo<'s>> for vks::VkDescriptorSetLayoutCreateInfo {
    fn from(f: DescriptorSetLayoutCreateInfo<'s>) -> vks::VkDescriptorSetLayoutCreateInfo {
        f.raw
    }
}


/// A builder for `VkDescriptorSetLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'b> {
    raw: vks::VkDescriptorSetLayoutCreateInfo,
    bindings: Option<Vec<vks::VkDescriptorSetLayoutBinding>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetLayoutCreateInfoBuilder<'b> {
    pub fn new() -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        DescriptorSetLayoutCreateInfoBuilder {
            raw: vks::VkDescriptorSetLayoutCreateInfo::default(),
            bindings: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DescriptorSetLayoutCreateFlags) -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn binding_count<'m>(mut self, binding_count: u32) -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        self.raw.bindingCount = binding_count.into();
        self
    }

    pub fn bindings<'m, 'a>(mut self, bindings: &'a [DescriptorSetLayoutBinding]) -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        self.bindings = Some(bindings.iter().map(|h| h.raw).collect());
        self.raw.pBindings = self.bindings.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> DescriptorSetLayoutCreateInfo<'b> {
        DescriptorSetLayoutCreateInfo {
            raw: self.raw,
            bindings: self.bindings,
            _p: PhantomData,
        }
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


impl From<DescriptorPoolSize> for vks::VkDescriptorPoolSize {
    fn from(f: DescriptorPoolSize) -> vks::VkDescriptorPoolSize {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: DescriptorType) -> DescriptorPoolSizeBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn descriptor_count<'m>(mut self, descriptor_count: u32) -> DescriptorPoolSizeBuilder {
        self.raw.descriptorCount = descriptor_count.into();
        self
    }

    pub fn build(self) -> DescriptorPoolSize {
        DescriptorPoolSize {
            raw: self.raw,
        }
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


impl<'s> From<DescriptorPoolCreateInfo<'s>> for vks::VkDescriptorPoolCreateInfo {
    fn from(f: DescriptorPoolCreateInfo<'s>) -> vks::VkDescriptorPoolCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DescriptorPoolCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DescriptorPoolCreateFlags) -> DescriptorPoolCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn max_sets<'m>(mut self, max_sets: u32) -> DescriptorPoolCreateInfoBuilder<'b> {
        self.raw.maxSets = max_sets.into();
        self
    }

    pub fn pool_size_count<'m>(mut self, pool_size_count: u32) -> DescriptorPoolCreateInfoBuilder<'b> {
        self.raw.poolSizeCount = pool_size_count.into();
        self
    }

    pub fn pool_sizes<'m, 'a>(mut self, pool_sizes: &'a [DescriptorPoolSize]) -> DescriptorPoolCreateInfoBuilder<'b> {
        self.raw.pPoolSizes = pool_sizes.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> DescriptorPoolCreateInfo<'b> {
        DescriptorPoolCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkDescriptorSetAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetAllocateInfo<'s> {
    raw: vks::VkDescriptorSetAllocateInfo,
    set_layouts: Option<Vec<vks::VkDescriptorSetLayout>>,
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


impl<'s> From<DescriptorSetAllocateInfo<'s>> for vks::VkDescriptorSetAllocateInfo {
    fn from(f: DescriptorSetAllocateInfo<'s>) -> vks::VkDescriptorSetAllocateInfo {
        f.raw
    }
}


/// A builder for `VkDescriptorSetAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetAllocateInfoBuilder<'b> {
    raw: vks::VkDescriptorSetAllocateInfo,
    set_layouts: Option<Vec<vks::VkDescriptorSetLayout>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DescriptorSetAllocateInfoBuilder<'b> {
    pub fn new() -> DescriptorSetAllocateInfoBuilder<'b> {
        DescriptorSetAllocateInfoBuilder {
            raw: vks::VkDescriptorSetAllocateInfo::default(),
            set_layouts: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DescriptorSetAllocateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn descriptor_pool<'m, 'a>(mut self, descriptor_pool: &'a DescriptorPool) -> DescriptorSetAllocateInfoBuilder<'b> {
        self.raw.descriptorPool = descriptor_pool.handle();
        self
    }

    pub fn descriptor_set_count<'m>(mut self, descriptor_set_count: u32) -> DescriptorSetAllocateInfoBuilder<'b> {
        self.raw.descriptorSetCount = descriptor_set_count.into();
        self
    }

    pub fn set_layouts<'m, 'a>(mut self, set_layouts: &'a [DescriptorSetLayout]) -> DescriptorSetAllocateInfoBuilder<'b> where 'a: 'b {
        self.set_layouts = Some(set_layouts.iter().map(|h| h.handle()).collect());
        self.raw.pSetLayouts = self.set_layouts.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> DescriptorSetAllocateInfo<'b> {
        DescriptorSetAllocateInfo {
            raw: self.raw,
            set_layouts: self.set_layouts,
            _p: PhantomData,
        }
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


impl From<SpecializationMapEntry> for vks::VkSpecializationMapEntry {
    fn from(f: SpecializationMapEntry) -> vks::VkSpecializationMapEntry {
        f.raw
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

    pub fn constant_id<'m>(mut self, constant_id: u32) -> SpecializationMapEntryBuilder {
        self.raw.constantID = constant_id.into();
        self
    }

    pub fn offset<'m>(mut self, offset: u32) -> SpecializationMapEntryBuilder {
        self.raw.offset = offset.into();
        self
    }

    pub fn size<'m>(mut self, size: usize) -> SpecializationMapEntryBuilder {
        self.raw.size = size.into();
        self
    }

    pub fn build(self) -> SpecializationMapEntry {
        SpecializationMapEntry {
            raw: self.raw,
        }
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

    pub unsafe fn data(&self) {
    }

    pub fn raw(&self) -> &vks::VkSpecializationInfo {
        &self.raw
    }
}


impl<'s> From<SpecializationInfo<'s>> for vks::VkSpecializationInfo {
    fn from(f: SpecializationInfo<'s>) -> vks::VkSpecializationInfo {
        f.raw
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

    pub fn map_entry_count<'m>(mut self, map_entry_count: u32) -> SpecializationInfoBuilder<'b> {
        self.raw.mapEntryCount = map_entry_count.into();
        self
    }

    pub fn map_entries<'m, 'a>(mut self, map_entries: &'a [SpecializationMapEntry]) -> SpecializationInfoBuilder<'b> {
        self.raw.pMapEntries = map_entries.as_ptr() as *const _;
        self
    }

    pub fn data_size<'m>(mut self, data_size: usize) -> SpecializationInfoBuilder<'b> {
        self.raw.dataSize = data_size.into();
        self
    }

    pub unsafe fn data<'m>(mut self, data: *const c_void) -> SpecializationInfoBuilder<'b> {
        self.raw.pData = data;
        self
    }

    pub fn build(self) -> SpecializationInfo<'b> {
        SpecializationInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkPipelineShaderStageCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineShaderStageCreateInfo<'s> {
    raw: vks::VkPipelineShaderStageCreateInfo,
    name: Option<CharStr<'s>>,
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


impl<'s> From<PipelineShaderStageCreateInfo<'s>> for vks::VkPipelineShaderStageCreateInfo {
    fn from(f: PipelineShaderStageCreateInfo<'s>) -> vks::VkPipelineShaderStageCreateInfo {
        f.raw
    }
}


/// A builder for `VkPipelineShaderStageCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineShaderStageCreateInfoBuilder<'b> {
    raw: vks::VkPipelineShaderStageCreateInfo,
    name: Option<CharStr<'b>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineShaderStageCreateInfoBuilder<'b> {
    pub fn new() -> PipelineShaderStageCreateInfoBuilder<'b> {
        PipelineShaderStageCreateInfoBuilder {
            raw: vks::VkPipelineShaderStageCreateInfo::default(),
            name: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineShaderStageCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineShaderStageCreateFlags) -> PipelineShaderStageCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn stage<'m>(mut self, stage: ShaderStageFlags) -> PipelineShaderStageCreateInfoBuilder<'b> {
        self.raw.stage = stage.bits();
        self
    }

    pub fn module<'m, 'a>(mut self, module: &'a ShaderModule) -> PipelineShaderStageCreateInfoBuilder<'b> {
        self.raw.module = module.handle();
        self
    }

    pub fn name<'m, 'a, T>(mut self, name: T) -> PipelineShaderStageCreateInfoBuilder<'b> where 'a: 'b, T: Into<CharStr<'a>> {
        self.name = Some(name.into());
        self.raw.pName = self.name.as_ref().unwrap().as_ptr();
        self
    }

    pub fn specialization_info<'m, 'a>(mut self, specialization_info: &'a SpecializationInfo) -> PipelineShaderStageCreateInfoBuilder<'b> {
        self.raw.pSpecializationInfo = specialization_info.raw();
        self
    }

    pub fn build(self) -> PipelineShaderStageCreateInfo<'b> {
        PipelineShaderStageCreateInfo {
            raw: self.raw,
            name: self.name,
            _p: PhantomData,
        }
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


impl<'s> From<ComputePipelineCreateInfo<'s>> for vks::VkComputePipelineCreateInfo {
    fn from(f: ComputePipelineCreateInfo<'s>) -> vks::VkComputePipelineCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ComputePipelineCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineCreateFlags) -> ComputePipelineCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn stage<'m>(mut self, stage: PipelineShaderStageCreateInfo) -> ComputePipelineCreateInfoBuilder<'b> {
        self.raw.stage = stage.raw;
        self
    }

    pub fn layout<'m, 'a>(mut self, layout: &'a PipelineLayout) -> ComputePipelineCreateInfoBuilder<'b> {
        self.raw.layout = layout.handle();
        self
    }

    pub fn base_pipeline_handle<'m, 'a>(mut self, base_pipeline_handle: &'a Pipeline) -> ComputePipelineCreateInfoBuilder<'b> {
        self.raw.basePipelineHandle = base_pipeline_handle.handle();
        self
    }

    pub fn base_pipeline_index<'m>(mut self, base_pipeline_index: i32) -> ComputePipelineCreateInfoBuilder<'b> {
        self.raw.basePipelineIndex = base_pipeline_index.into();
        self
    }

    pub fn build(self) -> ComputePipelineCreateInfo<'b> {
        ComputePipelineCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<VertexInputBindingDescription> for vks::VkVertexInputBindingDescription {
    fn from(f: VertexInputBindingDescription) -> vks::VkVertexInputBindingDescription {
        f.raw
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

    pub fn binding<'m>(mut self, binding: u32) -> VertexInputBindingDescriptionBuilder {
        self.raw.binding = binding.into();
        self
    }

    pub fn stride<'m>(mut self, stride: u32) -> VertexInputBindingDescriptionBuilder {
        self.raw.stride = stride.into();
        self
    }

    pub fn input_rate<'m>(mut self, input_rate: VertexInputRate) -> VertexInputBindingDescriptionBuilder {
        self.raw.inputRate = input_rate.into();
        self
    }

    pub fn build(self) -> VertexInputBindingDescription {
        VertexInputBindingDescription {
            raw: self.raw,
        }
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


impl From<VertexInputAttributeDescription> for vks::VkVertexInputAttributeDescription {
    fn from(f: VertexInputAttributeDescription) -> vks::VkVertexInputAttributeDescription {
        f.raw
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

    pub fn location<'m>(mut self, location: u32) -> VertexInputAttributeDescriptionBuilder {
        self.raw.location = location.into();
        self
    }

    pub fn binding<'m>(mut self, binding: u32) -> VertexInputAttributeDescriptionBuilder {
        self.raw.binding = binding.into();
        self
    }

    pub fn format<'m>(mut self, format: Format) -> VertexInputAttributeDescriptionBuilder {
        self.raw.format = format.into();
        self
    }

    pub fn offset<'m>(mut self, offset: u32) -> VertexInputAttributeDescriptionBuilder {
        self.raw.offset = offset.into();
        self
    }

    pub fn build(self) -> VertexInputAttributeDescription {
        VertexInputAttributeDescription {
            raw: self.raw,
        }
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


impl<'s> From<PipelineVertexInputStateCreateInfo<'s>> for vks::VkPipelineVertexInputStateCreateInfo {
    fn from(f: PipelineVertexInputStateCreateInfo<'s>) -> vks::VkPipelineVertexInputStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineVertexInputStateCreateFlags) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn vertex_binding_description_count<'m>(mut self, vertex_binding_description_count: u32) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        self.raw.vertexBindingDescriptionCount = vertex_binding_description_count.into();
        self
    }

    pub fn vertex_binding_descriptions<'m, 'a>(mut self, vertex_binding_descriptions: &'a [VertexInputBindingDescription]) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        self.raw.pVertexBindingDescriptions = vertex_binding_descriptions.as_ptr() as *const _;
        self
    }

    pub fn vertex_attribute_description_count<'m>(mut self, vertex_attribute_description_count: u32) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        self.raw.vertexAttributeDescriptionCount = vertex_attribute_description_count.into();
        self
    }

    pub fn vertex_attribute_descriptions<'m, 'a>(mut self, vertex_attribute_descriptions: &'a [VertexInputAttributeDescription]) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        self.raw.pVertexAttributeDescriptions = vertex_attribute_descriptions.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineVertexInputStateCreateInfo<'b> {
        PipelineVertexInputStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineInputAssemblyStateCreateInfo<'s>> for vks::VkPipelineInputAssemblyStateCreateInfo {
    fn from(f: PipelineInputAssemblyStateCreateInfo<'s>) -> vks::VkPipelineInputAssemblyStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineInputAssemblyStateCreateFlags) -> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn topology<'m>(mut self, topology: PrimitiveTopology) -> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self.raw.topology = topology.into();
        self
    }

    pub fn primitive_restart_enable<'m>(mut self, primitive_restart_enable: bool) -> PipelineInputAssemblyStateCreateInfoBuilder<'b> {
        self.raw.primitiveRestartEnable = primitive_restart_enable as u32;
        self
    }

    pub fn build(self) -> PipelineInputAssemblyStateCreateInfo<'b> {
        PipelineInputAssemblyStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineTessellationStateCreateInfo<'s>> for vks::VkPipelineTessellationStateCreateInfo {
    fn from(f: PipelineTessellationStateCreateInfo<'s>) -> vks::VkPipelineTessellationStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineTessellationStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineTessellationStateCreateFlags) -> PipelineTessellationStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn patch_control_points<'m>(mut self, patch_control_points: u32) -> PipelineTessellationStateCreateInfoBuilder<'b> {
        self.raw.patchControlPoints = patch_control_points.into();
        self
    }

    pub fn build(self) -> PipelineTessellationStateCreateInfo<'b> {
        PipelineTessellationStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineViewportStateCreateInfo<'s>> for vks::VkPipelineViewportStateCreateInfo {
    fn from(f: PipelineViewportStateCreateInfo<'s>) -> vks::VkPipelineViewportStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineViewportStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineViewportStateCreateFlags) -> PipelineViewportStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn viewport_count<'m>(mut self, viewport_count: u32) -> PipelineViewportStateCreateInfoBuilder<'b> {
        self.raw.viewportCount = viewport_count.into();
        self
    }

    pub fn viewports<'m, 'a>(mut self, viewports: &'a [Viewport]) -> PipelineViewportStateCreateInfoBuilder<'b> {
        self.raw.pViewports = viewports.as_ptr() as *const _;
        self
    }

    pub fn scissor_count<'m>(mut self, scissor_count: u32) -> PipelineViewportStateCreateInfoBuilder<'b> {
        self.raw.scissorCount = scissor_count.into();
        self
    }

    pub fn scissors<'m, 'a>(mut self, scissors: &'a [Rect2d]) -> PipelineViewportStateCreateInfoBuilder<'b> {
        self.raw.pScissors = scissors.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineViewportStateCreateInfo<'b> {
        PipelineViewportStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineRasterizationStateCreateInfo<'s>> for vks::VkPipelineRasterizationStateCreateInfo {
    fn from(f: PipelineRasterizationStateCreateInfo<'s>) -> vks::VkPipelineRasterizationStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineRasterizationStateCreateFlags) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn depth_clamp_enable<'m>(mut self, depth_clamp_enable: bool) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.depthClampEnable = depth_clamp_enable as u32;
        self
    }

    pub fn rasterizer_discard_enable<'m>(mut self, rasterizer_discard_enable: bool) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.rasterizerDiscardEnable = rasterizer_discard_enable as u32;
        self
    }

    pub fn polygon_mode<'m>(mut self, polygon_mode: PolygonMode) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.polygonMode = polygon_mode.into();
        self
    }

    pub fn cull_mode<'m>(mut self, cull_mode: CullModeFlags) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.cullMode = cull_mode.bits();
        self
    }

    pub fn front_face<'m>(mut self, front_face: FrontFace) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.frontFace = front_face.into();
        self
    }

    pub fn depth_bias_enable<'m>(mut self, depth_bias_enable: bool) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.depthBiasEnable = depth_bias_enable as u32;
        self
    }

    pub fn depth_bias_constant_factor<'m>(mut self, depth_bias_constant_factor: f32) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.depthBiasConstantFactor = depth_bias_constant_factor.into();
        self
    }

    pub fn depth_bias_clamp<'m>(mut self, depth_bias_clamp: f32) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.depthBiasClamp = depth_bias_clamp.into();
        self
    }

    pub fn depth_bias_slope_factor<'m>(mut self, depth_bias_slope_factor: f32) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.depthBiasSlopeFactor = depth_bias_slope_factor.into();
        self
    }

    pub fn line_width<'m>(mut self, line_width: f32) -> PipelineRasterizationStateCreateInfoBuilder<'b> {
        self.raw.lineWidth = line_width.into();
        self
    }

    pub fn build(self) -> PipelineRasterizationStateCreateInfo<'b> {
        PipelineRasterizationStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineMultisampleStateCreateInfo<'s>> for vks::VkPipelineMultisampleStateCreateInfo {
    fn from(f: PipelineMultisampleStateCreateInfo<'s>) -> vks::VkPipelineMultisampleStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineMultisampleStateCreateFlags) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn rasterization_samples<'m>(mut self, rasterization_samples: SampleCountFlags) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.rasterizationSamples = rasterization_samples.bits();
        self
    }

    pub fn sample_shading_enable<'m>(mut self, sample_shading_enable: bool) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.sampleShadingEnable = sample_shading_enable as u32;
        self
    }

    pub fn min_sample_shading<'m>(mut self, min_sample_shading: f32) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.minSampleShading = min_sample_shading.into();
        self
    }

    pub fn sample_mask<'m, 'a>(mut self, sample_mask: &'a u32) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.pSampleMask = sample_mask;
        self
    }

    pub fn alpha_to_coverage_enable<'m>(mut self, alpha_to_coverage_enable: bool) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.alphaToCoverageEnable = alpha_to_coverage_enable as u32;
        self
    }

    pub fn alpha_to_one_enable<'m>(mut self, alpha_to_one_enable: bool) -> PipelineMultisampleStateCreateInfoBuilder<'b> {
        self.raw.alphaToOneEnable = alpha_to_one_enable as u32;
        self
    }

    pub fn build(self) -> PipelineMultisampleStateCreateInfo<'b> {
        PipelineMultisampleStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<PipelineColorBlendAttachmentState> for vks::VkPipelineColorBlendAttachmentState {
    fn from(f: PipelineColorBlendAttachmentState) -> vks::VkPipelineColorBlendAttachmentState {
        f.raw
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

    pub fn blend_enable<'m>(mut self, blend_enable: bool) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.blendEnable = blend_enable as u32;
        self
    }

    pub fn src_color_blend_factor<'m>(mut self, src_color_blend_factor: BlendFactor) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.srcColorBlendFactor = src_color_blend_factor.into();
        self
    }

    pub fn dst_color_blend_factor<'m>(mut self, dst_color_blend_factor: BlendFactor) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.dstColorBlendFactor = dst_color_blend_factor.into();
        self
    }

    pub fn color_blend_op<'m>(mut self, color_blend_op: BlendOp) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.colorBlendOp = color_blend_op.into();
        self
    }

    pub fn src_alpha_blend_factor<'m>(mut self, src_alpha_blend_factor: BlendFactor) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.srcAlphaBlendFactor = src_alpha_blend_factor.into();
        self
    }

    pub fn dst_alpha_blend_factor<'m>(mut self, dst_alpha_blend_factor: BlendFactor) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.dstAlphaBlendFactor = dst_alpha_blend_factor.into();
        self
    }

    pub fn alpha_blend_op<'m>(mut self, alpha_blend_op: BlendOp) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.alphaBlendOp = alpha_blend_op.into();
        self
    }

    pub fn color_write_mask<'m>(mut self, color_write_mask: ColorComponentFlags) -> PipelineColorBlendAttachmentStateBuilder {
        self.raw.colorWriteMask = color_write_mask.bits();
        self
    }

    pub fn build(self) -> PipelineColorBlendAttachmentState {
        PipelineColorBlendAttachmentState {
            raw: self.raw,
        }
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


impl<'s> From<PipelineColorBlendStateCreateInfo<'s>> for vks::VkPipelineColorBlendStateCreateInfo {
    fn from(f: PipelineColorBlendStateCreateInfo<'s>) -> vks::VkPipelineColorBlendStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineColorBlendStateCreateFlags) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn logic_op_enable<'m>(mut self, logic_op_enable: bool) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.logicOpEnable = logic_op_enable as u32;
        self
    }

    pub fn logic_op<'m>(mut self, logic_op: LogicOp) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.logicOp = logic_op.into();
        self
    }

    pub fn attachment_count<'m>(mut self, attachment_count: u32) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.attachmentCount = attachment_count.into();
        self
    }

    pub fn attachments<'m, 'a>(mut self, attachments: &'a [PipelineColorBlendAttachmentState]) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.pAttachments = attachments.as_ptr() as *const _;
        self
    }

    pub fn blend_constants<'m>(mut self, blend_constants: [f32; 4]) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.blendConstants = blend_constants;
        self
    }

    pub fn build(self) -> PipelineColorBlendStateCreateInfo<'b> {
        PipelineColorBlendStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineDynamicStateCreateInfo<'s>> for vks::VkPipelineDynamicStateCreateInfo {
    fn from(f: PipelineDynamicStateCreateInfo<'s>) -> vks::VkPipelineDynamicStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineDynamicStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineDynamicStateCreateFlags) -> PipelineDynamicStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn dynamic_state_count<'m>(mut self, dynamic_state_count: u32) -> PipelineDynamicStateCreateInfoBuilder<'b> {
        self.raw.dynamicStateCount = dynamic_state_count.into();
        self
    }

    pub fn dynamic_states<'m, 'a>(mut self, dynamic_states: &'a [DynamicState]) -> PipelineDynamicStateCreateInfoBuilder<'b> {
        self.raw.pDynamicStates = dynamic_states.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineDynamicStateCreateInfo<'b> {
        PipelineDynamicStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<StencilOpState> for vks::VkStencilOpState {
    fn from(f: StencilOpState) -> vks::VkStencilOpState {
        f.raw
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

    pub fn fail_op<'m>(mut self, fail_op: StencilOp) -> StencilOpStateBuilder {
        self.raw.failOp = fail_op.into();
        self
    }

    pub fn pass_op<'m>(mut self, pass_op: StencilOp) -> StencilOpStateBuilder {
        self.raw.passOp = pass_op.into();
        self
    }

    pub fn depth_fail_op<'m>(mut self, depth_fail_op: StencilOp) -> StencilOpStateBuilder {
        self.raw.depthFailOp = depth_fail_op.into();
        self
    }

    pub fn compare_op<'m>(mut self, compare_op: CompareOp) -> StencilOpStateBuilder {
        self.raw.compareOp = compare_op.into();
        self
    }

    pub fn compare_mask<'m>(mut self, compare_mask: u32) -> StencilOpStateBuilder {
        self.raw.compareMask = compare_mask.into();
        self
    }

    pub fn write_mask<'m>(mut self, write_mask: u32) -> StencilOpStateBuilder {
        self.raw.writeMask = write_mask.into();
        self
    }

    pub fn reference<'m>(mut self, reference: u32) -> StencilOpStateBuilder {
        self.raw.reference = reference.into();
        self
    }

    pub fn build(self) -> StencilOpState {
        StencilOpState {
            raw: self.raw,
        }
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


impl<'s> From<PipelineDepthStencilStateCreateInfo<'s>> for vks::VkPipelineDepthStencilStateCreateInfo {
    fn from(f: PipelineDepthStencilStateCreateInfo<'s>) -> vks::VkPipelineDepthStencilStateCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineDepthStencilStateCreateFlags) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn depth_test_enable<'m>(mut self, depth_test_enable: bool) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.depthTestEnable = depth_test_enable as u32;
        self
    }

    pub fn depth_write_enable<'m>(mut self, depth_write_enable: bool) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.depthWriteEnable = depth_write_enable as u32;
        self
    }

    pub fn depth_compare_op<'m>(mut self, depth_compare_op: CompareOp) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.depthCompareOp = depth_compare_op.into();
        self
    }

    pub fn depth_bounds_test_enable<'m>(mut self, depth_bounds_test_enable: bool) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.depthBoundsTestEnable = depth_bounds_test_enable as u32;
        self
    }

    pub fn stencil_test_enable<'m>(mut self, stencil_test_enable: bool) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.stencilTestEnable = stencil_test_enable as u32;
        self
    }

    pub fn front<'m>(mut self, front: StencilOpState) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.front = front.raw;
        self
    }

    pub fn back<'m>(mut self, back: StencilOpState) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.back = back.raw;
        self
    }

    pub fn min_depth_bounds<'m>(mut self, min_depth_bounds: f32) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.minDepthBounds = min_depth_bounds.into();
        self
    }

    pub fn max_depth_bounds<'m>(mut self, max_depth_bounds: f32) -> PipelineDepthStencilStateCreateInfoBuilder<'b> {
        self.raw.maxDepthBounds = max_depth_bounds.into();
        self
    }

    pub fn build(self) -> PipelineDepthStencilStateCreateInfo<'b> {
        PipelineDepthStencilStateCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkGraphicsPipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct GraphicsPipelineCreateInfo<'s> {
    raw: vks::VkGraphicsPipelineCreateInfo,
    stages: Option<Vec<vks::VkPipelineShaderStageCreateInfo>>,
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


impl<'s> From<GraphicsPipelineCreateInfo<'s>> for vks::VkGraphicsPipelineCreateInfo {
    fn from(f: GraphicsPipelineCreateInfo<'s>) -> vks::VkGraphicsPipelineCreateInfo {
        f.raw
    }
}


/// A builder for `VkGraphicsPipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'b> {
    raw: vks::VkGraphicsPipelineCreateInfo,
    stages: Option<Vec<vks::VkPipelineShaderStageCreateInfo>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> GraphicsPipelineCreateInfoBuilder<'b> {
    pub fn new() -> GraphicsPipelineCreateInfoBuilder<'b> {
        GraphicsPipelineCreateInfoBuilder {
            raw: vks::VkGraphicsPipelineCreateInfo::default(),
            stages: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineCreateFlags) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn stage_count<'m>(mut self, stage_count: u32) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.stageCount = stage_count.into();
        self
    }

    pub fn stages<'m, 'a>(mut self, stages: &'a [PipelineShaderStageCreateInfo]) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.stages = Some(stages.iter().map(|h| h.raw).collect());
        self.raw.pStages = self.stages.as_ref().unwrap().as_ptr();
        self
    }

    pub fn vertex_input_state<'m, 'a>(mut self, vertex_input_state: &'a PipelineVertexInputStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pVertexInputState = vertex_input_state.raw();
        self
    }

    pub fn input_assembly_state<'m, 'a>(mut self, input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pInputAssemblyState = input_assembly_state.raw();
        self
    }

    pub fn tessellation_state<'m, 'a>(mut self, tessellation_state: &'a PipelineTessellationStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pTessellationState = tessellation_state.raw();
        self
    }

    pub fn viewport_state<'m, 'a>(mut self, viewport_state: &'a PipelineViewportStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pViewportState = viewport_state.raw();
        self
    }

    pub fn rasterization_state<'m, 'a>(mut self, rasterization_state: &'a PipelineRasterizationStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pRasterizationState = rasterization_state.raw();
        self
    }

    pub fn multisample_state<'m, 'a>(mut self, multisample_state: &'a PipelineMultisampleStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pMultisampleState = multisample_state.raw();
        self
    }

    pub fn depth_stencil_state<'m, 'a>(mut self, depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pDepthStencilState = depth_stencil_state.raw();
        self
    }

    pub fn color_blend_state<'m, 'a>(mut self, color_blend_state: &'a PipelineColorBlendStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pColorBlendState = color_blend_state.raw();
        self
    }

    pub fn dynamic_state<'m, 'a>(mut self, dynamic_state: &'a PipelineDynamicStateCreateInfo) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.pDynamicState = dynamic_state.raw();
        self
    }

    pub fn layout<'m, 'a>(mut self, layout: &'a PipelineLayout) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.layout = layout.handle();
        self
    }

    pub fn render_pass<'m, 'a>(mut self, render_pass: &'a RenderPass) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.renderPass = render_pass.handle();
        self
    }

    pub fn subpass<'m>(mut self, subpass: u32) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.subpass = subpass.into();
        self
    }

    pub fn base_pipeline_handle<'m, 'a>(mut self, base_pipeline_handle: &'a Pipeline) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.basePipelineHandle = base_pipeline_handle.handle();
        self
    }

    pub fn base_pipeline_index<'m>(mut self, base_pipeline_index: i32) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.raw.basePipelineIndex = base_pipeline_index.into();
        self
    }

    pub fn build(self) -> GraphicsPipelineCreateInfo<'b> {
        GraphicsPipelineCreateInfo {
            raw: self.raw,
            stages: self.stages,
            _p: PhantomData,
        }
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

    pub unsafe fn initial_data(&self) {
    }

    pub fn raw(&self) -> &vks::VkPipelineCacheCreateInfo {
        &self.raw
    }
}


impl<'s> From<PipelineCacheCreateInfo<'s>> for vks::VkPipelineCacheCreateInfo {
    fn from(f: PipelineCacheCreateInfo<'s>) -> vks::VkPipelineCacheCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineCacheCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineCacheCreateFlags) -> PipelineCacheCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn initial_data_size<'m>(mut self, initial_data_size: usize) -> PipelineCacheCreateInfoBuilder<'b> {
        self.raw.initialDataSize = initial_data_size.into();
        self
    }

    pub unsafe fn initial_data<'m>(mut self, initial_data: *const c_void) -> PipelineCacheCreateInfoBuilder<'b> {
        self.raw.pInitialData = initial_data;
        self
    }

    pub fn build(self) -> PipelineCacheCreateInfo<'b> {
        PipelineCacheCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<PushConstantRange> for vks::VkPushConstantRange {
    fn from(f: PushConstantRange) -> vks::VkPushConstantRange {
        f.raw
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

    pub fn stage_flags<'m>(mut self, stage_flags: ShaderStageFlags) -> PushConstantRangeBuilder {
        self.raw.stageFlags = stage_flags.bits();
        self
    }

    pub fn offset<'m>(mut self, offset: u32) -> PushConstantRangeBuilder {
        self.raw.offset = offset.into();
        self
    }

    pub fn size<'m>(mut self, size: u32) -> PushConstantRangeBuilder {
        self.raw.size = size.into();
        self
    }

    pub fn build(self) -> PushConstantRange {
        PushConstantRange {
            raw: self.raw,
        }
    }

}


/// A `VkPipelineLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineLayoutCreateInfo<'s> {
    raw: vks::VkPipelineLayoutCreateInfo,
    set_layouts: Option<Vec<vks::VkDescriptorSetLayout>>,
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


impl<'s> From<PipelineLayoutCreateInfo<'s>> for vks::VkPipelineLayoutCreateInfo {
    fn from(f: PipelineLayoutCreateInfo<'s>) -> vks::VkPipelineLayoutCreateInfo {
        f.raw
    }
}


/// A builder for `VkPipelineLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineLayoutCreateInfoBuilder<'b> {
    raw: vks::VkPipelineLayoutCreateInfo,
    set_layouts: Option<Vec<vks::VkDescriptorSetLayout>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> PipelineLayoutCreateInfoBuilder<'b> {
    pub fn new() -> PipelineLayoutCreateInfoBuilder<'b> {
        PipelineLayoutCreateInfoBuilder {
            raw: vks::VkPipelineLayoutCreateInfo::default(),
            set_layouts: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineLayoutCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineLayoutCreateFlags) -> PipelineLayoutCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn set_layout_count<'m>(mut self, set_layout_count: u32) -> PipelineLayoutCreateInfoBuilder<'b> {
        self.raw.setLayoutCount = set_layout_count.into();
        self
    }

    pub fn set_layouts<'m, 'a>(mut self, set_layouts: &'a [DescriptorSetLayout]) -> PipelineLayoutCreateInfoBuilder<'b> where 'a: 'b {
        self.set_layouts = Some(set_layouts.iter().map(|h| h.handle()).collect());
        self.raw.pSetLayouts = self.set_layouts.as_ref().unwrap().as_ptr();
        self
    }

    pub fn push_constant_range_count<'m>(mut self, push_constant_range_count: u32) -> PipelineLayoutCreateInfoBuilder<'b> {
        self.raw.pushConstantRangeCount = push_constant_range_count.into();
        self
    }

    pub fn push_constant_ranges<'m, 'a>(mut self, push_constant_ranges: &'a [PushConstantRange]) -> PipelineLayoutCreateInfoBuilder<'b> {
        self.raw.pPushConstantRanges = push_constant_ranges.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineLayoutCreateInfo<'b> {
        PipelineLayoutCreateInfo {
            raw: self.raw,
            set_layouts: self.set_layouts,
            _p: PhantomData,
        }
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


impl<'s> From<SamplerCreateInfo<'s>> for vks::VkSamplerCreateInfo {
    fn from(f: SamplerCreateInfo<'s>) -> vks::VkSamplerCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SamplerCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: SamplerCreateFlags) -> SamplerCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn mag_filter<'m>(mut self, mag_filter: Filter) -> SamplerCreateInfoBuilder<'b> {
        self.raw.magFilter = mag_filter.into();
        self
    }

    pub fn min_filter<'m>(mut self, min_filter: Filter) -> SamplerCreateInfoBuilder<'b> {
        self.raw.minFilter = min_filter.into();
        self
    }

    pub fn mipmap_mode<'m>(mut self, mipmap_mode: SamplerMipmapMode) -> SamplerCreateInfoBuilder<'b> {
        self.raw.mipmapMode = mipmap_mode.into();
        self
    }

    pub fn address_mode_u<'m>(mut self, address_mode_u: SamplerAddressMode) -> SamplerCreateInfoBuilder<'b> {
        self.raw.addressModeU = address_mode_u.into();
        self
    }

    pub fn address_mode_v<'m>(mut self, address_mode_v: SamplerAddressMode) -> SamplerCreateInfoBuilder<'b> {
        self.raw.addressModeV = address_mode_v.into();
        self
    }

    pub fn address_mode_w<'m>(mut self, address_mode_w: SamplerAddressMode) -> SamplerCreateInfoBuilder<'b> {
        self.raw.addressModeW = address_mode_w.into();
        self
    }

    pub fn mip_lod_bias<'m>(mut self, mip_lod_bias: f32) -> SamplerCreateInfoBuilder<'b> {
        self.raw.mipLodBias = mip_lod_bias.into();
        self
    }

    pub fn anisotropy_enable<'m>(mut self, anisotropy_enable: bool) -> SamplerCreateInfoBuilder<'b> {
        self.raw.anisotropyEnable = anisotropy_enable as u32;
        self
    }

    pub fn max_anisotropy<'m>(mut self, max_anisotropy: f32) -> SamplerCreateInfoBuilder<'b> {
        self.raw.maxAnisotropy = max_anisotropy.into();
        self
    }

    pub fn compare_enable<'m>(mut self, compare_enable: bool) -> SamplerCreateInfoBuilder<'b> {
        self.raw.compareEnable = compare_enable as u32;
        self
    }

    pub fn compare_op<'m>(mut self, compare_op: CompareOp) -> SamplerCreateInfoBuilder<'b> {
        self.raw.compareOp = compare_op.into();
        self
    }

    pub fn min_lod<'m>(mut self, min_lod: f32) -> SamplerCreateInfoBuilder<'b> {
        self.raw.minLod = min_lod.into();
        self
    }

    pub fn max_lod<'m>(mut self, max_lod: f32) -> SamplerCreateInfoBuilder<'b> {
        self.raw.maxLod = max_lod.into();
        self
    }

    pub fn border_color<'m>(mut self, border_color: BorderColor) -> SamplerCreateInfoBuilder<'b> {
        self.raw.borderColor = border_color.into();
        self
    }

    pub fn unnormalized_coordinates<'m>(mut self, unnormalized_coordinates: bool) -> SamplerCreateInfoBuilder<'b> {
        self.raw.unnormalizedCoordinates = unnormalized_coordinates as u32;
        self
    }

    pub fn build(self) -> SamplerCreateInfo<'b> {
        SamplerCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<CommandPoolCreateInfo<'s>> for vks::VkCommandPoolCreateInfo {
    fn from(f: CommandPoolCreateInfo<'s>) -> vks::VkCommandPoolCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CommandPoolCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: CommandPoolCreateFlags) -> CommandPoolCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn queue_family_index<'m>(mut self, queue_family_index: u32) -> CommandPoolCreateInfoBuilder<'b> {
        self.raw.queueFamilyIndex = queue_family_index.into();
        self
    }

    pub fn build(self) -> CommandPoolCreateInfo<'b> {
        CommandPoolCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<CommandBufferAllocateInfo<'s>> for vks::VkCommandBufferAllocateInfo {
    fn from(f: CommandBufferAllocateInfo<'s>) -> vks::VkCommandBufferAllocateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CommandBufferAllocateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn command_pool<'m, 'a>(mut self, command_pool: &'a CommandPool) -> CommandBufferAllocateInfoBuilder<'b> {
        self.raw.commandPool = command_pool.handle();
        self
    }

    pub fn level<'m>(mut self, level: CommandBufferLevel) -> CommandBufferAllocateInfoBuilder<'b> {
        self.raw.level = level.into();
        self
    }

    pub fn command_buffer_count<'m>(mut self, command_buffer_count: u32) -> CommandBufferAllocateInfoBuilder<'b> {
        self.raw.commandBufferCount = command_buffer_count.into();
        self
    }

    pub fn build(self) -> CommandBufferAllocateInfo<'b> {
        CommandBufferAllocateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<CommandBufferInheritanceInfo<'s>> for vks::VkCommandBufferInheritanceInfo {
    fn from(f: CommandBufferInheritanceInfo<'s>) -> vks::VkCommandBufferInheritanceInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn render_pass<'m, 'a>(mut self, render_pass: &'a RenderPass) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.renderPass = render_pass.handle();
        self
    }

    pub fn subpass<'m>(mut self, subpass: u32) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.subpass = subpass.into();
        self
    }

    pub fn framebuffer<'m, 'a>(mut self, framebuffer: &'a Framebuffer) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.framebuffer = framebuffer.handle();
        self
    }

    pub fn occlusion_query_enable<'m>(mut self, occlusion_query_enable: bool) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.occlusionQueryEnable = occlusion_query_enable as u32;
        self
    }

    pub fn query_flags<'m>(mut self, query_flags: QueryControlFlags) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.queryFlags = query_flags.bits();
        self
    }

    pub fn pipeline_statistics<'m>(mut self, pipeline_statistics: QueryPipelineStatisticFlags) -> CommandBufferInheritanceInfoBuilder<'b> {
        self.raw.pipelineStatistics = pipeline_statistics.bits();
        self
    }

    pub fn build(self) -> CommandBufferInheritanceInfo<'b> {
        CommandBufferInheritanceInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<CommandBufferBeginInfo<'s>> for vks::VkCommandBufferBeginInfo {
    fn from(f: CommandBufferBeginInfo<'s>) -> vks::VkCommandBufferBeginInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CommandBufferBeginInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: CommandBufferUsageFlags) -> CommandBufferBeginInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn inheritance_info<'m, 'a>(mut self, inheritance_info: &'a CommandBufferInheritanceInfo) -> CommandBufferBeginInfoBuilder<'b> {
        self.raw.pInheritanceInfo = inheritance_info.raw();
        self
    }

    pub fn build(self) -> CommandBufferBeginInfo<'b> {
        CommandBufferBeginInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<RenderPassBeginInfo<'s>> for vks::VkRenderPassBeginInfo {
    fn from(f: RenderPassBeginInfo<'s>) -> vks::VkRenderPassBeginInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> RenderPassBeginInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn render_pass<'m, 'a>(mut self, render_pass: &'a RenderPass) -> RenderPassBeginInfoBuilder<'b> {
        self.raw.renderPass = render_pass.handle();
        self
    }

    pub fn framebuffer<'m, 'a>(mut self, framebuffer: &'a Framebuffer) -> RenderPassBeginInfoBuilder<'b> {
        self.raw.framebuffer = framebuffer.handle();
        self
    }

    pub fn render_area<'m>(mut self, render_area: Rect2d) -> RenderPassBeginInfoBuilder<'b> {
        self.raw.renderArea = render_area.raw;
        self
    }

    pub fn clear_value_count<'m>(mut self, clear_value_count: u32) -> RenderPassBeginInfoBuilder<'b> {
        self.raw.clearValueCount = clear_value_count.into();
        self
    }

    pub fn clear_values<'m, 'a>(mut self, clear_values: &'a [ClearValue]) -> RenderPassBeginInfoBuilder<'b> {
        self.raw.pClearValues = clear_values.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> RenderPassBeginInfo<'b> {
        RenderPassBeginInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<ClearDepthStencilValue> for vks::VkClearDepthStencilValue {
    fn from(f: ClearDepthStencilValue) -> vks::VkClearDepthStencilValue {
        f.raw
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

    pub fn depth<'m>(mut self, depth: f32) -> ClearDepthStencilValueBuilder {
        self.raw.depth = depth.into();
        self
    }

    pub fn stencil<'m>(mut self, stencil: u32) -> ClearDepthStencilValueBuilder {
        self.raw.stencil = stencil.into();
        self
    }

    pub fn build(self) -> ClearDepthStencilValue {
        ClearDepthStencilValue {
            raw: self.raw,
        }
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


impl From<ClearAttachment> for vks::VkClearAttachment {
    fn from(f: ClearAttachment) -> vks::VkClearAttachment {
        f.raw
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

    pub fn aspect_mask<'m>(mut self, aspect_mask: ImageAspectFlags) -> ClearAttachmentBuilder {
        self.raw.aspectMask = aspect_mask.bits();
        self
    }

    pub fn color_attachment<'m>(mut self, color_attachment: u32) -> ClearAttachmentBuilder {
        self.raw.colorAttachment = color_attachment.into();
        self
    }

    pub fn clear_value<'m>(mut self, clear_value: ClearValue) -> ClearAttachmentBuilder {
        self.raw.clearValue = clear_value.into();
        self
    }

    pub fn build(self) -> ClearAttachment {
        ClearAttachment {
            raw: self.raw,
        }
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


impl From<AttachmentDescription> for vks::VkAttachmentDescription {
    fn from(f: AttachmentDescription) -> vks::VkAttachmentDescription {
        f.raw
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

    pub fn flags<'m>(mut self, flags: AttachmentDescriptionFlags) -> AttachmentDescriptionBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn format<'m>(mut self, format: Format) -> AttachmentDescriptionBuilder {
        self.raw.format = format.into();
        self
    }

    pub fn samples<'m>(mut self, samples: SampleCountFlags) -> AttachmentDescriptionBuilder {
        self.raw.samples = samples.bits();
        self
    }

    pub fn load_op<'m>(mut self, load_op: AttachmentLoadOp) -> AttachmentDescriptionBuilder {
        self.raw.loadOp = load_op.into();
        self
    }

    pub fn store_op<'m>(mut self, store_op: AttachmentStoreOp) -> AttachmentDescriptionBuilder {
        self.raw.storeOp = store_op.into();
        self
    }

    pub fn stencil_load_op<'m>(mut self, stencil_load_op: AttachmentLoadOp) -> AttachmentDescriptionBuilder {
        self.raw.stencilLoadOp = stencil_load_op.into();
        self
    }

    pub fn stencil_store_op<'m>(mut self, stencil_store_op: AttachmentStoreOp) -> AttachmentDescriptionBuilder {
        self.raw.stencilStoreOp = stencil_store_op.into();
        self
    }

    pub fn initial_layout<'m>(mut self, initial_layout: ImageLayout) -> AttachmentDescriptionBuilder {
        self.raw.initialLayout = initial_layout.into();
        self
    }

    pub fn final_layout<'m>(mut self, final_layout: ImageLayout) -> AttachmentDescriptionBuilder {
        self.raw.finalLayout = final_layout.into();
        self
    }

    pub fn build(self) -> AttachmentDescription {
        AttachmentDescription {
            raw: self.raw,
        }
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


impl From<AttachmentReference> for vks::VkAttachmentReference {
    fn from(f: AttachmentReference) -> vks::VkAttachmentReference {
        f.raw
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

    pub fn attachment<'m>(mut self, attachment: u32) -> AttachmentReferenceBuilder {
        self.raw.attachment = attachment.into();
        self
    }

    pub fn layout<'m>(mut self, layout: ImageLayout) -> AttachmentReferenceBuilder {
        self.raw.layout = layout.into();
        self
    }

    pub fn build(self) -> AttachmentReference {
        AttachmentReference {
            raw: self.raw,
        }
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


impl<'s> From<SubpassDescription<'s>> for vks::VkSubpassDescription {
    fn from(f: SubpassDescription<'s>) -> vks::VkSubpassDescription {
        f.raw
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

    pub fn flags<'m>(mut self, flags: SubpassDescriptionFlags) -> SubpassDescriptionBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn pipeline_bind_point<'m>(mut self, pipeline_bind_point: PipelineBindPoint) -> SubpassDescriptionBuilder<'b> {
        self.raw.pipelineBindPoint = pipeline_bind_point.into();
        self
    }

    pub fn input_attachment_count<'m>(mut self, input_attachment_count: u32) -> SubpassDescriptionBuilder<'b> {
        self.raw.inputAttachmentCount = input_attachment_count.into();
        self
    }

    pub fn input_attachments<'m, 'a>(mut self, input_attachments: &'a [AttachmentReference]) -> SubpassDescriptionBuilder<'b> {
        self.raw.pInputAttachments = input_attachments.as_ptr() as *const _;
        self
    }

    pub fn color_attachment_count<'m>(mut self, color_attachment_count: u32) -> SubpassDescriptionBuilder<'b> {
        self.raw.colorAttachmentCount = color_attachment_count.into();
        self
    }

    pub fn color_attachments<'m, 'a>(mut self, color_attachments: &'a [AttachmentReference]) -> SubpassDescriptionBuilder<'b> {
        self.raw.pColorAttachments = color_attachments.as_ptr() as *const _;
        self
    }

    pub fn resolve_attachments<'m, 'a>(mut self, resolve_attachments: &'a [AttachmentReference]) -> SubpassDescriptionBuilder<'b> {
        self.raw.pResolveAttachments = resolve_attachments.as_ptr() as *const _;
        self
    }

    pub fn depth_stencil_attachment<'m, 'a>(mut self, depth_stencil_attachment: &'a AttachmentReference) -> SubpassDescriptionBuilder<'b> {
        self.raw.pDepthStencilAttachment = depth_stencil_attachment.raw();
        self
    }

    pub fn preserve_attachment_count<'m>(mut self, preserve_attachment_count: u32) -> SubpassDescriptionBuilder<'b> {
        self.raw.preserveAttachmentCount = preserve_attachment_count.into();
        self
    }

    pub fn preserve_attachments<'m, 'a>(mut self, preserve_attachments: &'a [u32]) -> SubpassDescriptionBuilder<'b> {
        self.raw.pPreserveAttachments = preserve_attachments.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> SubpassDescription<'b> {
        SubpassDescription {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<SubpassDependency> for vks::VkSubpassDependency {
    fn from(f: SubpassDependency) -> vks::VkSubpassDependency {
        f.raw
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

    pub fn src_subpass<'m>(mut self, src_subpass: u32) -> SubpassDependencyBuilder {
        self.raw.srcSubpass = src_subpass.into();
        self
    }

    pub fn dst_subpass<'m>(mut self, dst_subpass: u32) -> SubpassDependencyBuilder {
        self.raw.dstSubpass = dst_subpass.into();
        self
    }

    pub fn src_stage_mask<'m>(mut self, src_stage_mask: PipelineStageFlags) -> SubpassDependencyBuilder {
        self.raw.srcStageMask = src_stage_mask.bits();
        self
    }

    pub fn dst_stage_mask<'m>(mut self, dst_stage_mask: PipelineStageFlags) -> SubpassDependencyBuilder {
        self.raw.dstStageMask = dst_stage_mask.bits();
        self
    }

    pub fn src_access_mask<'m>(mut self, src_access_mask: AccessFlags) -> SubpassDependencyBuilder {
        self.raw.srcAccessMask = src_access_mask.bits();
        self
    }

    pub fn dst_access_mask<'m>(mut self, dst_access_mask: AccessFlags) -> SubpassDependencyBuilder {
        self.raw.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub fn dependency_flags<'m>(mut self, dependency_flags: DependencyFlags) -> SubpassDependencyBuilder {
        self.raw.dependencyFlags = dependency_flags.bits();
        self
    }

    pub fn build(self) -> SubpassDependency {
        SubpassDependency {
            raw: self.raw,
        }
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


impl<'s> From<RenderPassCreateInfo<'s>> for vks::VkRenderPassCreateInfo {
    fn from(f: RenderPassCreateInfo<'s>) -> vks::VkRenderPassCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: RenderPassCreateFlags) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn attachment_count<'m>(mut self, attachment_count: u32) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.attachmentCount = attachment_count.into();
        self
    }

    pub fn attachments<'m, 'a>(mut self, attachments: &'a [AttachmentDescription]) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.pAttachments = attachments.as_ptr() as *const _;
        self
    }

    pub fn subpass_count<'m>(mut self, subpass_count: u32) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.subpassCount = subpass_count.into();
        self
    }

    pub fn subpasses<'m, 'a>(mut self, subpasses: &'a [SubpassDescription]) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.pSubpasses = subpasses.as_ptr() as *const _;
        self
    }

    pub fn dependency_count<'m>(mut self, dependency_count: u32) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.dependencyCount = dependency_count.into();
        self
    }

    pub fn dependencies<'m, 'a>(mut self, dependencies: &'a [SubpassDependency]) -> RenderPassCreateInfoBuilder<'b> {
        self.raw.pDependencies = dependencies.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> RenderPassCreateInfo<'b> {
        RenderPassCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<EventCreateInfo<'s>> for vks::VkEventCreateInfo {
    fn from(f: EventCreateInfo<'s>) -> vks::VkEventCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> EventCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: EventCreateFlags) -> EventCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> EventCreateInfo<'b> {
        EventCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<FenceCreateInfo<'s>> for vks::VkFenceCreateInfo {
    fn from(f: FenceCreateInfo<'s>) -> vks::VkFenceCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> FenceCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: FenceCreateFlags) -> FenceCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> FenceCreateInfo<'b> {
        FenceCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<PhysicalDeviceFeatures> for vks::VkPhysicalDeviceFeatures {
    fn from(f: PhysicalDeviceFeatures) -> vks::VkPhysicalDeviceFeatures {
        f.raw
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

    pub fn robust_buffer_access<'m>(mut self, robust_buffer_access: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.robustBufferAccess = robust_buffer_access as u32;
        self
    }

    pub fn full_draw_index_uint_32<'m>(mut self, full_draw_index_uint_32: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.fullDrawIndexUint32 = full_draw_index_uint_32 as u32;
        self
    }

    pub fn image_cube_array<'m>(mut self, image_cube_array: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.imageCubeArray = image_cube_array as u32;
        self
    }

    pub fn independent_blend<'m>(mut self, independent_blend: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.independentBlend = independent_blend as u32;
        self
    }

    pub fn geometry_shader<'m>(mut self, geometry_shader: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.geometryShader = geometry_shader as u32;
        self
    }

    pub fn tessellation_shader<'m>(mut self, tessellation_shader: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.tessellationShader = tessellation_shader as u32;
        self
    }

    pub fn sample_rate_shading<'m>(mut self, sample_rate_shading: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sampleRateShading = sample_rate_shading as u32;
        self
    }

    pub fn dual_src_blend<'m>(mut self, dual_src_blend: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.dualSrcBlend = dual_src_blend as u32;
        self
    }

    pub fn logic_op<'m>(mut self, logic_op: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.logicOp = logic_op as u32;
        self
    }

    pub fn multi_draw_indirect<'m>(mut self, multi_draw_indirect: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.multiDrawIndirect = multi_draw_indirect as u32;
        self
    }

    pub fn draw_indirect_first_instance<'m>(mut self, draw_indirect_first_instance: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.drawIndirectFirstInstance = draw_indirect_first_instance as u32;
        self
    }

    pub fn depth_clamp<'m>(mut self, depth_clamp: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.depthClamp = depth_clamp as u32;
        self
    }

    pub fn depth_bias_clamp<'m>(mut self, depth_bias_clamp: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.depthBiasClamp = depth_bias_clamp as u32;
        self
    }

    pub fn fill_mode_non_solid<'m>(mut self, fill_mode_non_solid: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.fillModeNonSolid = fill_mode_non_solid as u32;
        self
    }

    pub fn depth_bounds<'m>(mut self, depth_bounds: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.depthBounds = depth_bounds as u32;
        self
    }

    pub fn wide_lines<'m>(mut self, wide_lines: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.wideLines = wide_lines as u32;
        self
    }

    pub fn large_points<'m>(mut self, large_points: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.largePoints = large_points as u32;
        self
    }

    pub fn alpha_to_one<'m>(mut self, alpha_to_one: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.alphaToOne = alpha_to_one as u32;
        self
    }

    pub fn multi_viewport<'m>(mut self, multi_viewport: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.multiViewport = multi_viewport as u32;
        self
    }

    pub fn sampler_anisotropy<'m>(mut self, sampler_anisotropy: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.samplerAnisotropy = sampler_anisotropy as u32;
        self
    }

    pub fn texture_compression_et_c2<'m>(mut self, texture_compression_et_c2: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.textureCompressionETC2 = texture_compression_et_c2 as u32;
        self
    }

    pub fn texture_compression_as_tc_ld_r<'m>(mut self, texture_compression_as_tc_ld_r: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.textureCompressionASTC_LDR = texture_compression_as_tc_ld_r as u32;
        self
    }

    pub fn texture_compression_bc<'m>(mut self, texture_compression_bc: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.textureCompressionBC = texture_compression_bc as u32;
        self
    }

    pub fn occlusion_query_precise<'m>(mut self, occlusion_query_precise: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.occlusionQueryPrecise = occlusion_query_precise as u32;
        self
    }

    pub fn pipeline_statistics_query<'m>(mut self, pipeline_statistics_query: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.pipelineStatisticsQuery = pipeline_statistics_query as u32;
        self
    }

    pub fn vertex_pipeline_stores_and_atomics<'m>(mut self, vertex_pipeline_stores_and_atomics: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.vertexPipelineStoresAndAtomics = vertex_pipeline_stores_and_atomics as u32;
        self
    }

    pub fn fragment_stores_and_atomics<'m>(mut self, fragment_stores_and_atomics: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.fragmentStoresAndAtomics = fragment_stores_and_atomics as u32;
        self
    }

    pub fn shader_tessellation_and_geometry_point_size<'m>(mut self, shader_tessellation_and_geometry_point_size: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderTessellationAndGeometryPointSize = shader_tessellation_and_geometry_point_size as u32;
        self
    }

    pub fn shader_image_gather_extended<'m>(mut self, shader_image_gather_extended: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderImageGatherExtended = shader_image_gather_extended as u32;
        self
    }

    pub fn shader_storage_image_extended_formats<'m>(mut self, shader_storage_image_extended_formats: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderStorageImageExtendedFormats = shader_storage_image_extended_formats as u32;
        self
    }

    pub fn shader_storage_image_multisample<'m>(mut self, shader_storage_image_multisample: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderStorageImageMultisample = shader_storage_image_multisample as u32;
        self
    }

    pub fn shader_storage_image_read_without_format<'m>(mut self, shader_storage_image_read_without_format: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderStorageImageReadWithoutFormat = shader_storage_image_read_without_format as u32;
        self
    }

    pub fn shader_storage_image_write_without_format<'m>(mut self, shader_storage_image_write_without_format: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderStorageImageWriteWithoutFormat = shader_storage_image_write_without_format as u32;
        self
    }

    pub fn shader_uniform_buffer_array_dynamic_indexing<'m>(mut self, shader_uniform_buffer_array_dynamic_indexing: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderUniformBufferArrayDynamicIndexing = shader_uniform_buffer_array_dynamic_indexing as u32;
        self
    }

    pub fn shader_sampled_image_array_dynamic_indexing<'m>(mut self, shader_sampled_image_array_dynamic_indexing: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderSampledImageArrayDynamicIndexing = shader_sampled_image_array_dynamic_indexing as u32;
        self
    }

    pub fn shader_storage_buffer_array_dynamic_indexing<'m>(mut self, shader_storage_buffer_array_dynamic_indexing: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderStorageBufferArrayDynamicIndexing = shader_storage_buffer_array_dynamic_indexing as u32;
        self
    }

    pub fn shader_storage_image_array_dynamic_indexing<'m>(mut self, shader_storage_image_array_dynamic_indexing: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderStorageImageArrayDynamicIndexing = shader_storage_image_array_dynamic_indexing as u32;
        self
    }

    pub fn shader_clip_distance<'m>(mut self, shader_clip_distance: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderClipDistance = shader_clip_distance as u32;
        self
    }

    pub fn shader_cull_distance<'m>(mut self, shader_cull_distance: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderCullDistance = shader_cull_distance as u32;
        self
    }

    pub fn shader_float_64<'m>(mut self, shader_float_64: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderFloat64 = shader_float_64 as u32;
        self
    }

    pub fn shader_int_64<'m>(mut self, shader_int_64: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderInt64 = shader_int_64 as u32;
        self
    }

    pub fn shader_int_16<'m>(mut self, shader_int_16: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderInt16 = shader_int_16 as u32;
        self
    }

    pub fn shader_resource_residency<'m>(mut self, shader_resource_residency: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderResourceResidency = shader_resource_residency as u32;
        self
    }

    pub fn shader_resource_min_lod<'m>(mut self, shader_resource_min_lod: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.shaderResourceMinLod = shader_resource_min_lod as u32;
        self
    }

    pub fn sparse_binding<'m>(mut self, sparse_binding: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseBinding = sparse_binding as u32;
        self
    }

    pub fn sparse_residency_buffer<'m>(mut self, sparse_residency_buffer: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidencyBuffer = sparse_residency_buffer as u32;
        self
    }

    pub fn sparse_residency_image_2d<'m>(mut self, sparse_residency_image_2d: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidencyImage2D = sparse_residency_image_2d as u32;
        self
    }

    pub fn sparse_residency_image_3d<'m>(mut self, sparse_residency_image_3d: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidencyImage3D = sparse_residency_image_3d as u32;
        self
    }

    pub fn sparse_residency_2samples<'m>(mut self, sparse_residency_2samples: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidency2Samples = sparse_residency_2samples as u32;
        self
    }

    pub fn sparse_residency_4samples<'m>(mut self, sparse_residency_4samples: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidency4Samples = sparse_residency_4samples as u32;
        self
    }

    pub fn sparse_residency_8samples<'m>(mut self, sparse_residency_8samples: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidency8Samples = sparse_residency_8samples as u32;
        self
    }

    pub fn sparse_residency_16_samples<'m>(mut self, sparse_residency_16_samples: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidency16Samples = sparse_residency_16_samples as u32;
        self
    }

    pub fn sparse_residency_aliased<'m>(mut self, sparse_residency_aliased: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.sparseResidencyAliased = sparse_residency_aliased as u32;
        self
    }

    pub fn variable_multisample_rate<'m>(mut self, variable_multisample_rate: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.variableMultisampleRate = variable_multisample_rate as u32;
        self
    }

    pub fn inherited_queries<'m>(mut self, inherited_queries: bool) -> PhysicalDeviceFeaturesBuilder {
        self.raw.inheritedQueries = inherited_queries as u32;
        self
    }

    pub fn build(self) -> PhysicalDeviceFeatures {
        PhysicalDeviceFeatures {
            raw: self.raw,
        }
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


impl From<PhysicalDeviceSparseProperties> for vks::VkPhysicalDeviceSparseProperties {
    fn from(f: PhysicalDeviceSparseProperties) -> vks::VkPhysicalDeviceSparseProperties {
        f.raw
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


impl From<PhysicalDeviceLimits> for vks::VkPhysicalDeviceLimits {
    fn from(f: PhysicalDeviceLimits) -> vks::VkPhysicalDeviceLimits {
        f.raw
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


impl<'s> From<SemaphoreCreateInfo<'s>> for vks::VkSemaphoreCreateInfo {
    fn from(f: SemaphoreCreateInfo<'s>) -> vks::VkSemaphoreCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SemaphoreCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: SemaphoreCreateFlags) -> SemaphoreCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> SemaphoreCreateInfo<'b> {
        SemaphoreCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<QueryPoolCreateInfo<'s>> for vks::VkQueryPoolCreateInfo {
    fn from(f: QueryPoolCreateInfo<'s>) -> vks::VkQueryPoolCreateInfo {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> QueryPoolCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: QueryPoolCreateFlags) -> QueryPoolCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn query_type<'m>(mut self, query_type: QueryType) -> QueryPoolCreateInfoBuilder<'b> {
        self.raw.queryType = query_type.into();
        self
    }

    pub fn query_count<'m>(mut self, query_count: u32) -> QueryPoolCreateInfoBuilder<'b> {
        self.raw.queryCount = query_count.into();
        self
    }

    pub fn pipeline_statistics<'m>(mut self, pipeline_statistics: QueryPipelineStatisticFlags) -> QueryPoolCreateInfoBuilder<'b> {
        self.raw.pipelineStatistics = pipeline_statistics.bits();
        self
    }

    pub fn build(self) -> QueryPoolCreateInfo<'b> {
        QueryPoolCreateInfo {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkFramebufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FramebufferCreateInfo<'s> {
    raw: vks::VkFramebufferCreateInfo,
    attachments: Option<Vec<vks::VkImageView>>,
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


impl<'s> From<FramebufferCreateInfo<'s>> for vks::VkFramebufferCreateInfo {
    fn from(f: FramebufferCreateInfo<'s>) -> vks::VkFramebufferCreateInfo {
        f.raw
    }
}


/// A builder for `VkFramebufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FramebufferCreateInfoBuilder<'b> {
    raw: vks::VkFramebufferCreateInfo,
    attachments: Option<Vec<vks::VkImageView>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> FramebufferCreateInfoBuilder<'b> {
    pub fn new() -> FramebufferCreateInfoBuilder<'b> {
        FramebufferCreateInfoBuilder {
            raw: vks::VkFramebufferCreateInfo::default(),
            attachments: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: FramebufferCreateFlags) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn render_pass<'m, 'a>(mut self, render_pass: &'a RenderPass) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.renderPass = render_pass.handle();
        self
    }

    pub fn attachment_count<'m>(mut self, attachment_count: u32) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.attachmentCount = attachment_count.into();
        self
    }

    pub fn attachments<'m, 'a>(mut self, attachments: &'a [ImageView]) -> FramebufferCreateInfoBuilder<'b> where 'a: 'b {
        self.attachments = Some(attachments.iter().map(|h| h.handle()).collect());
        self.raw.pAttachments = self.attachments.as_ref().unwrap().as_ptr();
        self
    }

    pub fn width<'m>(mut self, width: u32) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.width = width.into();
        self
    }

    pub fn height<'m>(mut self, height: u32) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.height = height.into();
        self
    }

    pub fn layers<'m>(mut self, layers: u32) -> FramebufferCreateInfoBuilder<'b> {
        self.raw.layers = layers.into();
        self
    }

    pub fn build(self) -> FramebufferCreateInfo<'b> {
        FramebufferCreateInfo {
            raw: self.raw,
            attachments: self.attachments,
            _p: PhantomData,
        }
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


impl From<DrawIndirectCommand> for vks::VkDrawIndirectCommand {
    fn from(f: DrawIndirectCommand) -> vks::VkDrawIndirectCommand {
        f.raw
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

    pub fn vertex_count<'m>(mut self, vertex_count: u32) -> DrawIndirectCommandBuilder {
        self.raw.vertexCount = vertex_count.into();
        self
    }

    pub fn instance_count<'m>(mut self, instance_count: u32) -> DrawIndirectCommandBuilder {
        self.raw.instanceCount = instance_count.into();
        self
    }

    pub fn first_vertex<'m>(mut self, first_vertex: u32) -> DrawIndirectCommandBuilder {
        self.raw.firstVertex = first_vertex.into();
        self
    }

    pub fn first_instance<'m>(mut self, first_instance: u32) -> DrawIndirectCommandBuilder {
        self.raw.firstInstance = first_instance.into();
        self
    }

    pub fn build(self) -> DrawIndirectCommand {
        DrawIndirectCommand {
            raw: self.raw,
        }
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


impl From<DrawIndexedIndirectCommand> for vks::VkDrawIndexedIndirectCommand {
    fn from(f: DrawIndexedIndirectCommand) -> vks::VkDrawIndexedIndirectCommand {
        f.raw
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

    pub fn index_count<'m>(mut self, index_count: u32) -> DrawIndexedIndirectCommandBuilder {
        self.raw.indexCount = index_count.into();
        self
    }

    pub fn instance_count<'m>(mut self, instance_count: u32) -> DrawIndexedIndirectCommandBuilder {
        self.raw.instanceCount = instance_count.into();
        self
    }

    pub fn first_index<'m>(mut self, first_index: u32) -> DrawIndexedIndirectCommandBuilder {
        self.raw.firstIndex = first_index.into();
        self
    }

    pub fn vertex_offset<'m>(mut self, vertex_offset: i32) -> DrawIndexedIndirectCommandBuilder {
        self.raw.vertexOffset = vertex_offset.into();
        self
    }

    pub fn first_instance<'m>(mut self, first_instance: u32) -> DrawIndexedIndirectCommandBuilder {
        self.raw.firstInstance = first_instance.into();
        self
    }

    pub fn build(self) -> DrawIndexedIndirectCommand {
        DrawIndexedIndirectCommand {
            raw: self.raw,
        }
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


impl From<DispatchIndirectCommand> for vks::VkDispatchIndirectCommand {
    fn from(f: DispatchIndirectCommand) -> vks::VkDispatchIndirectCommand {
        f.raw
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

    pub fn x<'m>(mut self, x: u32) -> DispatchIndirectCommandBuilder {
        self.raw.x = x.into();
        self
    }

    pub fn y<'m>(mut self, y: u32) -> DispatchIndirectCommandBuilder {
        self.raw.y = y.into();
        self
    }

    pub fn z<'m>(mut self, z: u32) -> DispatchIndirectCommandBuilder {
        self.raw.z = z.into();
        self
    }

    pub fn build(self) -> DispatchIndirectCommand {
        DispatchIndirectCommand {
            raw: self.raw,
        }
    }

}


/// A `VkSubmitInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SubmitInfo<'s> {
    raw: vks::VkSubmitInfo,
    command_buffers: Option<Vec<vks::VkCommandBuffer>>,
    signal_semaphores: Option<Vec<vks::VkSemaphore>>,
    wait_semaphores: Option<Vec<vks::VkSemaphore>>,
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


impl<'s> From<SubmitInfo<'s>> for vks::VkSubmitInfo {
    fn from(f: SubmitInfo<'s>) -> vks::VkSubmitInfo {
        f.raw
    }
}


/// A builder for `VkSubmitInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SubmitInfoBuilder<'b> {
    raw: vks::VkSubmitInfo,
    command_buffers: Option<Vec<vks::VkCommandBuffer>>,
    signal_semaphores: Option<Vec<vks::VkSemaphore>>,
    wait_semaphores: Option<Vec<vks::VkSemaphore>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> SubmitInfoBuilder<'b> {
    pub fn new() -> SubmitInfoBuilder<'b> {
        SubmitInfoBuilder {
            raw: vks::VkSubmitInfo::default(),
            command_buffers: None,
            signal_semaphores: None,
            wait_semaphores: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SubmitInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphore_count<'m>(mut self, wait_semaphore_count: u32) -> SubmitInfoBuilder<'b> {
        self.raw.waitSemaphoreCount = wait_semaphore_count.into();
        self
    }

    pub fn wait_semaphores<'m, 'a>(mut self, wait_semaphores: &'a [Semaphore]) -> SubmitInfoBuilder<'b> where 'a: 'b {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        self.raw.pWaitSemaphores = self.wait_semaphores.as_ref().unwrap().as_ptr();
        self
    }

    pub fn wait_dst_stage_mask<'m, 'a>(mut self, wait_dst_stage_mask: &'a PipelineStageFlags) -> SubmitInfoBuilder<'b> {
        self.raw.pWaitDstStageMask = wait_dst_stage_mask as *const PipelineStageFlags as *const _;
        self
    }

    pub fn command_buffer_count<'m>(mut self, command_buffer_count: u32) -> SubmitInfoBuilder<'b> {
        self.raw.commandBufferCount = command_buffer_count.into();
        self
    }

    pub fn command_buffers<'m, 'a>(mut self, command_buffers: &'a [CommandBuffer]) -> SubmitInfoBuilder<'b> where 'a: 'b {
        self.command_buffers = Some(command_buffers.iter().map(|h| h.handle()).collect());
        self.raw.pCommandBuffers = self.command_buffers.as_ref().unwrap().as_ptr();
        self
    }

    pub fn signal_semaphore_count<'m>(mut self, signal_semaphore_count: u32) -> SubmitInfoBuilder<'b> {
        self.raw.signalSemaphoreCount = signal_semaphore_count.into();
        self
    }

    pub fn signal_semaphores<'m, 'a>(mut self, signal_semaphores: &'a [Semaphore]) -> SubmitInfoBuilder<'b> where 'a: 'b {
        self.signal_semaphores = Some(signal_semaphores.iter().map(|h| h.handle()).collect());
        self.raw.pSignalSemaphores = self.signal_semaphores.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> SubmitInfo<'b> {
        SubmitInfo {
            raw: self.raw,
            command_buffers: self.command_buffers,
            signal_semaphores: self.signal_semaphores,
            wait_semaphores: self.wait_semaphores,
            _p: PhantomData,
        }
    }

}


/// A `VkDisplayPropertiesKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DisplayPropertiesKhr<'s> {
    raw: vks::VkDisplayPropertiesKHR,
    display_name: Option<CharStr<'s>>,
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


impl<'s> From<DisplayPropertiesKhr<'s>> for vks::VkDisplayPropertiesKHR {
    fn from(f: DisplayPropertiesKhr<'s>) -> vks::VkDisplayPropertiesKHR {
        f.raw
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


impl From<DisplayPlanePropertiesKhr> for vks::VkDisplayPlanePropertiesKHR {
    fn from(f: DisplayPlanePropertiesKhr) -> vks::VkDisplayPlanePropertiesKHR {
        f.raw
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


impl From<DisplayModeParametersKhr> for vks::VkDisplayModeParametersKHR {
    fn from(f: DisplayModeParametersKhr) -> vks::VkDisplayModeParametersKHR {
        f.raw
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

    pub fn visible_region<'m>(mut self, visible_region: Extent2d) -> DisplayModeParametersKhrBuilder {
        self.raw.visibleRegion = visible_region.raw;
        self
    }

    pub fn refresh_rate<'m>(mut self, refresh_rate: u32) -> DisplayModeParametersKhrBuilder {
        self.raw.refreshRate = refresh_rate.into();
        self
    }

    pub fn build(self) -> DisplayModeParametersKhr {
        DisplayModeParametersKhr {
            raw: self.raw,
        }
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


impl From<DisplayModePropertiesKhr> for vks::VkDisplayModePropertiesKHR {
    fn from(f: DisplayModePropertiesKhr) -> vks::VkDisplayModePropertiesKHR {
        f.raw
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


impl<'s> From<DisplayModeCreateInfoKhr<'s>> for vks::VkDisplayModeCreateInfoKHR {
    fn from(f: DisplayModeCreateInfoKhr<'s>) -> vks::VkDisplayModeCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DisplayModeCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DisplayModeCreateFlagsKhr) -> DisplayModeCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn parameters<'m>(mut self, parameters: DisplayModeParametersKhr) -> DisplayModeCreateInfoKhrBuilder<'b> {
        self.raw.parameters = parameters.raw;
        self
    }

    pub fn build(self) -> DisplayModeCreateInfoKhr<'b> {
        DisplayModeCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<DisplayPlaneCapabilitiesKhr> for vks::VkDisplayPlaneCapabilitiesKHR {
    fn from(f: DisplayPlaneCapabilitiesKhr) -> vks::VkDisplayPlaneCapabilitiesKHR {
        f.raw
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


impl<'s> From<DisplaySurfaceCreateInfoKhr<'s>> for vks::VkDisplaySurfaceCreateInfoKHR {
    fn from(f: DisplaySurfaceCreateInfoKhr<'s>) -> vks::VkDisplaySurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DisplaySurfaceCreateFlagsKhr) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn display_mode<'m, 'a>(mut self, display_mode: &'a DisplayModeKhr) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.displayMode = display_mode.handle();
        self
    }

    pub fn plane_index<'m>(mut self, plane_index: u32) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.planeIndex = plane_index.into();
        self
    }

    pub fn plane_stack_index<'m>(mut self, plane_stack_index: u32) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.planeStackIndex = plane_stack_index.into();
        self
    }

    pub fn transform<'m>(mut self, transform: SurfaceTransformFlagsKhr) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.transform = transform.bits();
        self
    }

    pub fn global_alpha<'m>(mut self, global_alpha: f32) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.globalAlpha = global_alpha.into();
        self
    }

    pub fn alpha_mode<'m>(mut self, alpha_mode: DisplayPlaneAlphaFlagsKhr) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.alphaMode = alpha_mode.bits();
        self
    }

    pub fn image_extent<'m>(mut self, image_extent: Extent2d) -> DisplaySurfaceCreateInfoKhrBuilder<'b> {
        self.raw.imageExtent = image_extent.raw;
        self
    }

    pub fn build(self) -> DisplaySurfaceCreateInfoKhr<'b> {
        DisplaySurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<DisplayPresentInfoKhr<'s>> for vks::VkDisplayPresentInfoKHR {
    fn from(f: DisplayPresentInfoKhr<'s>) -> vks::VkDisplayPresentInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DisplayPresentInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn src_rect<'m>(mut self, src_rect: Rect2d) -> DisplayPresentInfoKhrBuilder<'b> {
        self.raw.srcRect = src_rect.raw;
        self
    }

    pub fn dst_rect<'m>(mut self, dst_rect: Rect2d) -> DisplayPresentInfoKhrBuilder<'b> {
        self.raw.dstRect = dst_rect.raw;
        self
    }

    pub fn persistent<'m>(mut self, persistent: bool) -> DisplayPresentInfoKhrBuilder<'b> {
        self.raw.persistent = persistent as u32;
        self
    }

    pub fn build(self) -> DisplayPresentInfoKhr<'b> {
        DisplayPresentInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<SurfaceCapabilitiesKhr> for vks::VkSurfaceCapabilitiesKHR {
    fn from(f: SurfaceCapabilitiesKhr) -> vks::VkSurfaceCapabilitiesKHR {
        f.raw
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

    pub unsafe fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkAndroidSurfaceCreateInfoKHR {
        &self.raw
    }
}


impl<'s> From<AndroidSurfaceCreateInfoKhr<'s>> for vks::VkAndroidSurfaceCreateInfoKHR {
    fn from(f: AndroidSurfaceCreateInfoKhr<'s>) -> vks::VkAndroidSurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> AndroidSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: AndroidSurfaceCreateFlagsKhr) -> AndroidSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn window<'m>(mut self, window: *mut ANativeWindow) -> AndroidSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.window = window;
        self
    }

    pub fn build(self) -> AndroidSurfaceCreateInfoKhr<'b> {
        AndroidSurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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

    pub unsafe fn connection(&self) {
    }

    pub unsafe fn mir_surface(&self) {
    }

    pub fn raw(&self) -> &vks::VkMirSurfaceCreateInfoKHR {
        &self.raw
    }
}


impl<'s> From<MirSurfaceCreateInfoKhr<'s>> for vks::VkMirSurfaceCreateInfoKHR {
    fn from(f: MirSurfaceCreateInfoKhr<'s>) -> vks::VkMirSurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MirSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: MirSurfaceCreateFlagsKhr) -> MirSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn connection<'m>(mut self, connection: *mut MirConnection) -> MirSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.connection = connection;
        self
    }

    pub unsafe fn mir_surface<'m>(mut self, mir_surface: *mut MirSurface) -> MirSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.mirSurface = mir_surface;
        self
    }

    pub fn build(self) -> MirSurfaceCreateInfoKhr<'b> {
        MirSurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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

    pub unsafe fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkViSurfaceCreateInfoNN {
        &self.raw
    }
}


impl<'s> From<ViSurfaceCreateInfoNn<'s>> for vks::VkViSurfaceCreateInfoNN {
    fn from(f: ViSurfaceCreateInfoNn<'s>) -> vks::VkViSurfaceCreateInfoNN {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ViSurfaceCreateInfoNnBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: ViSurfaceCreateFlagsNn) -> ViSurfaceCreateInfoNnBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn window<'m>(mut self, window: *mut c_void) -> ViSurfaceCreateInfoNnBuilder<'b> {
        self.raw.window = window;
        self
    }

    pub fn build(self) -> ViSurfaceCreateInfoNn<'b> {
        ViSurfaceCreateInfoNn {
            raw: self.raw,
            _p: PhantomData,
        }
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

    pub unsafe fn display(&self) {
    }

    pub unsafe fn surface(&self) {
    }

    pub fn raw(&self) -> &vks::VkWaylandSurfaceCreateInfoKHR {
        &self.raw
    }
}


impl<'s> From<WaylandSurfaceCreateInfoKhr<'s>> for vks::VkWaylandSurfaceCreateInfoKHR {
    fn from(f: WaylandSurfaceCreateInfoKhr<'s>) -> vks::VkWaylandSurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: WaylandSurfaceCreateFlagsKhr) -> WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn display<'m>(mut self, display: *mut wl_display) -> WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.display = display;
        self
    }

    pub unsafe fn surface<'m>(mut self, surface: *mut wl_surface) -> WaylandSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.surface = surface;
        self
    }

    pub fn build(self) -> WaylandSurfaceCreateInfoKhr<'b> {
        WaylandSurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<Win32SurfaceCreateInfoKhr<'s>> for vks::VkWin32SurfaceCreateInfoKHR {
    fn from(f: Win32SurfaceCreateInfoKhr<'s>) -> vks::VkWin32SurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> Win32SurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: Win32SurfaceCreateFlagsKhr) -> Win32SurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn hinstance<'m>(mut self, hinstance: HINSTANCE) -> Win32SurfaceCreateInfoKhrBuilder<'b> {
        self.raw.hinstance = hinstance.into();
        self
    }

    pub fn hwnd<'m>(mut self, hwnd: HWND) -> Win32SurfaceCreateInfoKhrBuilder<'b> {
        self.raw.hwnd = hwnd.into();
        self
    }

    pub fn build(self) -> Win32SurfaceCreateInfoKhr<'b> {
        Win32SurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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

    pub unsafe fn dpy(&self) {
    }

    pub unsafe fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkXlibSurfaceCreateInfoKHR {
        &self.raw
    }
}


impl<'s> From<XlibSurfaceCreateInfoKhr<'s>> for vks::VkXlibSurfaceCreateInfoKHR {
    fn from(f: XlibSurfaceCreateInfoKhr<'s>) -> vks::VkXlibSurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> XlibSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: XlibSurfaceCreateFlagsKhr) -> XlibSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn dpy<'m>(mut self, dpy: *mut Display) -> XlibSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.dpy = dpy;
        self
    }

    pub unsafe fn window<'m>(mut self, window: u32) -> XlibSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.window = window.into();
        self
    }

    pub fn build(self) -> XlibSurfaceCreateInfoKhr<'b> {
        XlibSurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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

    pub unsafe fn connection(&self) {
    }

    pub unsafe fn window(&self) {
    }

    pub fn raw(&self) -> &vks::VkXcbSurfaceCreateInfoKHR {
        &self.raw
    }
}


impl<'s> From<XcbSurfaceCreateInfoKhr<'s>> for vks::VkXcbSurfaceCreateInfoKHR {
    fn from(f: XcbSurfaceCreateInfoKhr<'s>) -> vks::VkXcbSurfaceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> XcbSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: XcbSurfaceCreateFlagsKhr) -> XcbSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn connection<'m>(mut self, connection: *mut xcb_connection_t) -> XcbSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.connection = connection;
        self
    }

    pub unsafe fn window<'m>(mut self, window: xcb_window_t) -> XcbSurfaceCreateInfoKhrBuilder<'b> {
        self.raw.window = window.into();
        self
    }

    pub fn build(self) -> XcbSurfaceCreateInfoKhr<'b> {
        XcbSurfaceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<SurfaceFormatKhr> for vks::VkSurfaceFormatKHR {
    fn from(f: SurfaceFormatKhr) -> vks::VkSurfaceFormatKHR {
        f.raw
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


impl<'s> From<SwapchainCreateInfoKhr<'s>> for vks::VkSwapchainCreateInfoKHR {
    fn from(f: SwapchainCreateInfoKhr<'s>) -> vks::VkSwapchainCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: SwapchainCreateFlagsKhr) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn surface<'m, 'a>(mut self, surface: &'a Surface) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.surface = surface.handle();
        self
    }

    pub fn min_image_count<'m>(mut self, min_image_count: u32) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.minImageCount = min_image_count.into();
        self
    }

    pub fn image_format<'m>(mut self, image_format: Format) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.imageFormat = image_format.into();
        self
    }

    pub fn image_color_space<'m>(mut self, image_color_space: ColorSpaceKhr) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.imageColorSpace = image_color_space.into();
        self
    }

    pub fn image_extent<'m>(mut self, image_extent: Extent2d) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.imageExtent = image_extent.raw;
        self
    }

    pub fn image_array_layers<'m>(mut self, image_array_layers: u32) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.imageArrayLayers = image_array_layers.into();
        self
    }

    pub fn image_usage<'m>(mut self, image_usage: ImageUsageFlags) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.imageUsage = image_usage.bits();
        self
    }

    pub fn image_sharing_mode<'m>(mut self, image_sharing_mode: SharingMode) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.imageSharingMode = image_sharing_mode.into();
        self
    }

    pub fn queue_family_index_count<'m>(mut self, queue_family_index_count: u32) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.queueFamilyIndexCount = queue_family_index_count.into();
        self
    }

    pub fn queue_family_indices<'m, 'a>(mut self, queue_family_indices: &'a [u32]) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const _;
        self
    }

    pub fn pre_transform<'m>(mut self, pre_transform: SurfaceTransformFlagsKhr) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.preTransform = pre_transform.bits();
        self
    }

    pub fn composite_alpha<'m>(mut self, composite_alpha: CompositeAlphaFlagsKhr) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.compositeAlpha = composite_alpha.bits();
        self
    }

    pub fn present_mode<'m>(mut self, present_mode: PresentModeKhr) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.presentMode = present_mode.into();
        self
    }

    pub fn clipped<'m>(mut self, clipped: bool) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.clipped = clipped as u32;
        self
    }

    pub fn old_swapchain<'m, 'a>(mut self, old_swapchain: &'a Swapchain) -> SwapchainCreateInfoKhrBuilder<'b> {
        self.raw.oldSwapchain = old_swapchain.handle();
        self
    }

    pub fn build(self) -> SwapchainCreateInfoKhr<'b> {
        SwapchainCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentInfoKhr<'s> {
    raw: vks::VkPresentInfoKHR,
    swapchains: Option<Vec<vks::VkSwapchainKHR>>,
    wait_semaphores: Option<Vec<vks::VkSemaphore>>,
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


impl<'s> From<PresentInfoKhr<'s>> for vks::VkPresentInfoKHR {
    fn from(f: PresentInfoKhr<'s>) -> vks::VkPresentInfoKHR {
        f.raw
    }
}


/// A builder for `VkPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentInfoKhrBuilder<'b> {
    raw: vks::VkPresentInfoKHR,
    swapchains: Option<Vec<vks::VkSwapchainKHR>>,
    wait_semaphores: Option<Vec<vks::VkSemaphore>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> PresentInfoKhrBuilder<'b> {
    pub fn new() -> PresentInfoKhrBuilder<'b> {
        PresentInfoKhrBuilder {
            raw: vks::VkPresentInfoKHR::default(),
            swapchains: None,
            wait_semaphores: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PresentInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphore_count<'m>(mut self, wait_semaphore_count: u32) -> PresentInfoKhrBuilder<'b> {
        self.raw.waitSemaphoreCount = wait_semaphore_count.into();
        self
    }

    pub fn wait_semaphores<'m, 'a>(mut self, wait_semaphores: &'a [Semaphore]) -> PresentInfoKhrBuilder<'b> where 'a: 'b {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        self.raw.pWaitSemaphores = self.wait_semaphores.as_ref().unwrap().as_ptr();
        self
    }

    pub fn swapchain_count<'m>(mut self, swapchain_count: u32) -> PresentInfoKhrBuilder<'b> {
        self.raw.swapchainCount = swapchain_count.into();
        self
    }

    pub fn swapchains<'m, 'a>(mut self, swapchains: &'a [Swapchain]) -> PresentInfoKhrBuilder<'b> where 'a: 'b {
        self.swapchains = Some(swapchains.iter().map(|h| h.handle()).collect());
        self.raw.pSwapchains = self.swapchains.as_ref().unwrap().as_ptr();
        self
    }

    pub fn image_indices<'m, 'a>(mut self, image_indices: &'a [u32]) -> PresentInfoKhrBuilder<'b> {
        self.raw.pImageIndices = image_indices.as_ptr() as *const _;
        self
    }

    pub fn results<'m, 'a>(mut self, results: &'a mut [ResultEnum]) -> PresentInfoKhrBuilder<'b> {
        self.raw.pResults = results.as_mut_ptr() as *mut _;
        self
    }

    pub fn build(self) -> PresentInfoKhr<'b> {
        PresentInfoKhr {
            raw: self.raw,
            swapchains: self.swapchains,
            wait_semaphores: self.wait_semaphores,
            _p: PhantomData,
        }
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

    pub unsafe fn user_data(&self) {
    }

    pub fn raw(&self) -> &vks::VkDebugReportCallbackCreateInfoEXT {
        &self.raw
    }
}


impl<'s> From<DebugReportCallbackCreateInfoExt<'s>> for vks::VkDebugReportCallbackCreateInfoEXT {
    fn from(f: DebugReportCallbackCreateInfoExt<'s>) -> vks::VkDebugReportCallbackCreateInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DebugReportCallbackCreateInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DebugReportFlagsExt) -> DebugReportCallbackCreateInfoExtBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn pfn_callback<'m>(mut self, pfn_callback: PFN_vkDebugReportCallbackEXT) -> DebugReportCallbackCreateInfoExtBuilder<'b> {
        self.raw.pfnCallback = pfn_callback.into();
        self
    }

    pub unsafe fn user_data<'m>(mut self, user_data: *mut c_void) -> DebugReportCallbackCreateInfoExtBuilder<'b> {
        self.raw.pUserData = user_data;
        self
    }

    pub fn build(self) -> DebugReportCallbackCreateInfoExt<'b> {
        DebugReportCallbackCreateInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ValidationFlagsExt<'s>> for vks::VkValidationFlagsEXT {
    fn from(f: ValidationFlagsExt<'s>) -> vks::VkValidationFlagsEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ValidationFlagsExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn disabled_validation_check_count<'m>(mut self, disabled_validation_check_count: u32) -> ValidationFlagsExtBuilder<'b> {
        self.raw.disabledValidationCheckCount = disabled_validation_check_count.into();
        self
    }

    pub fn disabled_validation_checks<'m, 'a>(mut self, disabled_validation_checks: &'a mut [ValidationCheckExt]) -> ValidationFlagsExtBuilder<'b> {
        self.raw.pDisabledValidationChecks = disabled_validation_checks.as_mut_ptr() as *mut _;
        self
    }

    pub fn build(self) -> ValidationFlagsExt<'b> {
        ValidationFlagsExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineRasterizationStateRasterizationOrderAmd<'s>> for vks::VkPipelineRasterizationStateRasterizationOrderAMD {
    fn from(f: PipelineRasterizationStateRasterizationOrderAmd<'s>) -> vks::VkPipelineRasterizationStateRasterizationOrderAMD {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn rasterization_order<'m>(mut self, rasterization_order: RasterizationOrderAmd) -> PipelineRasterizationStateRasterizationOrderAmdBuilder<'b> {
        self.raw.rasterizationOrder = rasterization_order.into();
        self
    }

    pub fn build(self) -> PipelineRasterizationStateRasterizationOrderAmd<'b> {
        PipelineRasterizationStateRasterizationOrderAmd {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkDebugMarkerObjectNameInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerObjectNameInfoExt<'s> {
    raw: vks::VkDebugMarkerObjectNameInfoEXT,
    object_name: Option<CharStr<'s>>,
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


impl<'s> From<DebugMarkerObjectNameInfoExt<'s>> for vks::VkDebugMarkerObjectNameInfoEXT {
    fn from(f: DebugMarkerObjectNameInfoExt<'s>) -> vks::VkDebugMarkerObjectNameInfoEXT {
        f.raw
    }
}


/// A builder for `VkDebugMarkerObjectNameInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerObjectNameInfoExtBuilder<'b> {
    raw: vks::VkDebugMarkerObjectNameInfoEXT,
    object_name: Option<CharStr<'b>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DebugMarkerObjectNameInfoExtBuilder<'b> {
    pub fn new() -> DebugMarkerObjectNameInfoExtBuilder<'b> {
        DebugMarkerObjectNameInfoExtBuilder {
            raw: vks::VkDebugMarkerObjectNameInfoEXT::default(),
            object_name: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DebugMarkerObjectNameInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn object_type<'m>(mut self, object_type: DebugReportObjectTypeExt) -> DebugMarkerObjectNameInfoExtBuilder<'b> {
        self.raw.objectType = object_type.into();
        self
    }

    pub fn object<'m>(mut self, object: u64) -> DebugMarkerObjectNameInfoExtBuilder<'b> {
        self.raw.object = object.into();
        self
    }

    pub fn object_name<'m, 'a, T>(mut self, object_name: T) -> DebugMarkerObjectNameInfoExtBuilder<'b> where 'a: 'b, T: Into<CharStr<'a>> {
        self.object_name = Some(object_name.into());
        self.raw.pObjectName = self.object_name.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> DebugMarkerObjectNameInfoExt<'b> {
        DebugMarkerObjectNameInfoExt {
            raw: self.raw,
            object_name: self.object_name,
            _p: PhantomData,
        }
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

    pub unsafe fn tag(&self) {
    }

    pub fn raw(&self) -> &vks::VkDebugMarkerObjectTagInfoEXT {
        &self.raw
    }
}


impl<'s> From<DebugMarkerObjectTagInfoExt<'s>> for vks::VkDebugMarkerObjectTagInfoEXT {
    fn from(f: DebugMarkerObjectTagInfoExt<'s>) -> vks::VkDebugMarkerObjectTagInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn object_type<'m>(mut self, object_type: DebugReportObjectTypeExt) -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        self.raw.objectType = object_type.into();
        self
    }

    pub fn object<'m>(mut self, object: u64) -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        self.raw.object = object.into();
        self
    }

    pub fn tag_name<'m>(mut self, tag_name: u64) -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        self.raw.tagName = tag_name.into();
        self
    }

    pub fn tag_size<'m>(mut self, tag_size: usize) -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        self.raw.tagSize = tag_size.into();
        self
    }

    pub unsafe fn tag<'m>(mut self, tag: *const c_void) -> DebugMarkerObjectTagInfoExtBuilder<'b> {
        self.raw.pTag = tag;
        self
    }

    pub fn build(self) -> DebugMarkerObjectTagInfoExt<'b> {
        DebugMarkerObjectTagInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkDebugMarkerMarkerInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerMarkerInfoExt<'s> {
    raw: vks::VkDebugMarkerMarkerInfoEXT,
    marker_name: Option<CharStr<'s>>,
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


impl<'s> From<DebugMarkerMarkerInfoExt<'s>> for vks::VkDebugMarkerMarkerInfoEXT {
    fn from(f: DebugMarkerMarkerInfoExt<'s>) -> vks::VkDebugMarkerMarkerInfoEXT {
        f.raw
    }
}


/// A builder for `VkDebugMarkerMarkerInfoEXT`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DebugMarkerMarkerInfoExtBuilder<'b> {
    raw: vks::VkDebugMarkerMarkerInfoEXT,
    marker_name: Option<CharStr<'b>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> DebugMarkerMarkerInfoExtBuilder<'b> {
    pub fn new() -> DebugMarkerMarkerInfoExtBuilder<'b> {
        DebugMarkerMarkerInfoExtBuilder {
            raw: vks::VkDebugMarkerMarkerInfoEXT::default(),
            marker_name: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DebugMarkerMarkerInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn marker_name<'m, 'a, T>(mut self, marker_name: T) -> DebugMarkerMarkerInfoExtBuilder<'b> where 'a: 'b, T: Into<CharStr<'a>> {
        self.marker_name = Some(marker_name.into());
        self.raw.pMarkerName = self.marker_name.as_ref().unwrap().as_ptr();
        self
    }

    pub fn color<'m>(mut self, color: [f32; 4]) -> DebugMarkerMarkerInfoExtBuilder<'b> {
        self.raw.color = color;
        self
    }

    pub fn build(self) -> DebugMarkerMarkerInfoExt<'b> {
        DebugMarkerMarkerInfoExt {
            raw: self.raw,
            marker_name: self.marker_name,
            _p: PhantomData,
        }
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


impl<'s> From<DedicatedAllocationImageCreateInfoNv<'s>> for vks::VkDedicatedAllocationImageCreateInfoNV {
    fn from(f: DedicatedAllocationImageCreateInfoNv<'s>) -> vks::VkDedicatedAllocationImageCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DedicatedAllocationImageCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn dedicated_allocation<'m>(mut self, dedicated_allocation: bool) -> DedicatedAllocationImageCreateInfoNvBuilder<'b> {
        self.raw.dedicatedAllocation = dedicated_allocation as u32;
        self
    }

    pub fn build(self) -> DedicatedAllocationImageCreateInfoNv<'b> {
        DedicatedAllocationImageCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<DedicatedAllocationBufferCreateInfoNv<'s>> for vks::VkDedicatedAllocationBufferCreateInfoNV {
    fn from(f: DedicatedAllocationBufferCreateInfoNv<'s>) -> vks::VkDedicatedAllocationBufferCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn dedicated_allocation<'m>(mut self, dedicated_allocation: bool) -> DedicatedAllocationBufferCreateInfoNvBuilder<'b> {
        self.raw.dedicatedAllocation = dedicated_allocation as u32;
        self
    }

    pub fn build(self) -> DedicatedAllocationBufferCreateInfoNv<'b> {
        DedicatedAllocationBufferCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<DedicatedAllocationMemoryAllocateInfoNv<'s>> for vks::VkDedicatedAllocationMemoryAllocateInfoNV {
    fn from(f: DedicatedAllocationMemoryAllocateInfoNv<'s>) -> vks::VkDedicatedAllocationMemoryAllocateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> DedicatedAllocationMemoryAllocateInfoNvBuilder<'b> {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn build(self) -> DedicatedAllocationMemoryAllocateInfoNv<'b> {
        DedicatedAllocationMemoryAllocateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<ExternalImageFormatPropertiesNv> for vks::VkExternalImageFormatPropertiesNV {
    fn from(f: ExternalImageFormatPropertiesNv) -> vks::VkExternalImageFormatPropertiesNV {
        f.raw
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


impl<'s> From<ExternalMemoryImageCreateInfoNv<'s>> for vks::VkExternalMemoryImageCreateInfoNV {
    fn from(f: ExternalMemoryImageCreateInfoNv<'s>) -> vks::VkExternalMemoryImageCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExternalMemoryImageCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalMemoryHandleTypeFlagsNv) -> ExternalMemoryImageCreateInfoNvBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExternalMemoryImageCreateInfoNv<'b> {
        ExternalMemoryImageCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExportMemoryAllocateInfoNv<'s>> for vks::VkExportMemoryAllocateInfoNV {
    fn from(f: ExportMemoryAllocateInfoNv<'s>) -> vks::VkExportMemoryAllocateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportMemoryAllocateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalMemoryHandleTypeFlagsNv) -> ExportMemoryAllocateInfoNvBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExportMemoryAllocateInfoNv<'b> {
        ExportMemoryAllocateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportMemoryWin32HandleInfoNv<'s>> for vks::VkImportMemoryWin32HandleInfoNV {
    fn from(f: ImportMemoryWin32HandleInfoNv<'s>) -> vks::VkImportMemoryWin32HandleInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsNv) -> ImportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn handle<'m>(mut self, handle: HANDLE) -> ImportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.handle = handle.into();
        self
    }

    pub fn build(self) -> ImportMemoryWin32HandleInfoNv<'b> {
        ImportMemoryWin32HandleInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExportMemoryWin32HandleInfoNv<'s>> for vks::VkExportMemoryWin32HandleInfoNV {
    fn from(f: ExportMemoryWin32HandleInfoNv<'s>) -> vks::VkExportMemoryWin32HandleInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn attributes<'m, 'a>(mut self, attributes: &'a [SECURITY_ATTRIBUTES]) -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.pAttributes = attributes.as_ptr() as *const _;
        self
    }

    pub fn dw_access<'m>(mut self, dw_access: DWORD) -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.dwAccess = dw_access.into();
        self
    }

    pub fn build(self) -> ExportMemoryWin32HandleInfoNv<'b> {
        ExportMemoryWin32HandleInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkWin32KeyedMutexAcquireReleaseInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNv<'s> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV,
    acquire_syncs: Option<Vec<vks::VkDeviceMemory>>,
    release_syncs: Option<Vec<vks::VkDeviceMemory>>,
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


impl<'s> From<Win32KeyedMutexAcquireReleaseInfoNv<'s>> for vks::VkWin32KeyedMutexAcquireReleaseInfoNV {
    fn from(f: Win32KeyedMutexAcquireReleaseInfoNv<'s>) -> vks::VkWin32KeyedMutexAcquireReleaseInfoNV {
        f.raw
    }
}


/// A builder for `VkWin32KeyedMutexAcquireReleaseInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV,
    acquire_syncs: Option<Vec<vks::VkDeviceMemory>>,
    release_syncs: Option<Vec<vks::VkDeviceMemory>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
    pub fn new() -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoNvBuilder {
            raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV::default(),
            acquire_syncs: None,
            release_syncs: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn acquire_count<'m>(mut self, acquire_count: u32) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self.raw.acquireCount = acquire_count.into();
        self
    }

    pub fn acquire_syncs<'m, 'a>(mut self, acquire_syncs: &'a [DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> where 'a: 'b {
        self.acquire_syncs = Some(acquire_syncs.iter().map(|h| h.handle()).collect());
        self.raw.pAcquireSyncs = self.acquire_syncs.as_ref().unwrap().as_ptr();
        self
    }

    pub fn acquire_keys<'m, 'a>(mut self, acquire_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self.raw.pAcquireKeys = acquire_keys.as_ptr() as *const _;
        self
    }

    pub fn acquire_timeout_milliseconds<'m, 'a>(mut self, acquire_timeout_milliseconds: &'a [u32]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self.raw.pAcquireTimeoutMilliseconds = acquire_timeout_milliseconds.as_ptr() as *const _;
        self
    }

    pub fn release_count<'m>(mut self, release_count: u32) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self.raw.releaseCount = release_count.into();
        self
    }

    pub fn release_syncs<'m, 'a>(mut self, release_syncs: &'a [DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> where 'a: 'b {
        self.release_syncs = Some(release_syncs.iter().map(|h| h.handle()).collect());
        self.raw.pReleaseSyncs = self.release_syncs.as_ref().unwrap().as_ptr();
        self
    }

    pub fn release_keys<'m, 'a>(mut self, release_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        self.raw.pReleaseKeys = release_keys.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> Win32KeyedMutexAcquireReleaseInfoNv<'b> {
        Win32KeyedMutexAcquireReleaseInfoNv {
            raw: self.raw,
            acquire_syncs: self.acquire_syncs,
            release_syncs: self.release_syncs,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGeneratedCommandsFeaturesNvx<'s>> for vks::VkDeviceGeneratedCommandsFeaturesNVX {
    fn from(f: DeviceGeneratedCommandsFeaturesNvx<'s>) -> vks::VkDeviceGeneratedCommandsFeaturesNVX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn compute_binding_point_support<'m>(mut self, compute_binding_point_support: bool) -> DeviceGeneratedCommandsFeaturesNvxBuilder<'b> {
        self.raw.computeBindingPointSupport = compute_binding_point_support as u32;
        self
    }

    pub fn build(self) -> DeviceGeneratedCommandsFeaturesNvx<'b> {
        DeviceGeneratedCommandsFeaturesNvx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGeneratedCommandsLimitsNvx<'s>> for vks::VkDeviceGeneratedCommandsLimitsNVX {
    fn from(f: DeviceGeneratedCommandsLimitsNvx<'s>) -> vks::VkDeviceGeneratedCommandsLimitsNVX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn max_indirect_commands_layout_token_count<'m>(mut self, max_indirect_commands_layout_token_count: u32) -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self.raw.maxIndirectCommandsLayoutTokenCount = max_indirect_commands_layout_token_count.into();
        self
    }

    pub fn max_object_entry_counts<'m>(mut self, max_object_entry_counts: u32) -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self.raw.maxObjectEntryCounts = max_object_entry_counts.into();
        self
    }

    pub fn min_sequence_count_buffer_offset_alignment<'m>(mut self, min_sequence_count_buffer_offset_alignment: u32) -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self.raw.minSequenceCountBufferOffsetAlignment = min_sequence_count_buffer_offset_alignment.into();
        self
    }

    pub fn min_sequence_index_buffer_offset_alignment<'m>(mut self, min_sequence_index_buffer_offset_alignment: u32) -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self.raw.minSequenceIndexBufferOffsetAlignment = min_sequence_index_buffer_offset_alignment.into();
        self
    }

    pub fn min_commands_token_buffer_offset_alignment<'m>(mut self, min_commands_token_buffer_offset_alignment: u32) -> DeviceGeneratedCommandsLimitsNvxBuilder<'b> {
        self.raw.minCommandsTokenBufferOffsetAlignment = min_commands_token_buffer_offset_alignment.into();
        self
    }

    pub fn build(self) -> DeviceGeneratedCommandsLimitsNvx<'b> {
        DeviceGeneratedCommandsLimitsNvx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl From<IndirectCommandsTokenNvx> for vks::VkIndirectCommandsTokenNVX {
    fn from(f: IndirectCommandsTokenNvx) -> vks::VkIndirectCommandsTokenNVX {
        f.raw
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

    pub fn token_type<'m>(mut self, token_type: IndirectCommandsTokenTypeNvx) -> IndirectCommandsTokenNvxBuilder {
        self.raw.tokenType = token_type.into();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> IndirectCommandsTokenNvxBuilder {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn offset<'m>(mut self, offset: u64) -> IndirectCommandsTokenNvxBuilder {
        self.raw.offset = offset.into();
        self
    }

    pub fn build(self) -> IndirectCommandsTokenNvx {
        IndirectCommandsTokenNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl From<IndirectCommandsLayoutTokenNvx> for vks::VkIndirectCommandsLayoutTokenNVX {
    fn from(f: IndirectCommandsLayoutTokenNvx) -> vks::VkIndirectCommandsLayoutTokenNVX {
        f.raw
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

    pub fn token_type<'m>(mut self, token_type: IndirectCommandsTokenTypeNvx) -> IndirectCommandsLayoutTokenNvxBuilder {
        self.raw.tokenType = token_type.into();
        self
    }

    pub fn binding_unit<'m>(mut self, binding_unit: u32) -> IndirectCommandsLayoutTokenNvxBuilder {
        self.raw.bindingUnit = binding_unit.into();
        self
    }

    pub fn dynamic_count<'m>(mut self, dynamic_count: u32) -> IndirectCommandsLayoutTokenNvxBuilder {
        self.raw.dynamicCount = dynamic_count.into();
        self
    }

    pub fn divisor<'m>(mut self, divisor: u32) -> IndirectCommandsLayoutTokenNvxBuilder {
        self.raw.divisor = divisor.into();
        self
    }

    pub fn build(self) -> IndirectCommandsLayoutTokenNvx {
        IndirectCommandsLayoutTokenNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<IndirectCommandsLayoutCreateInfoNvx<'s>> for vks::VkIndirectCommandsLayoutCreateInfoNVX {
    fn from(f: IndirectCommandsLayoutCreateInfoNvx<'s>) -> vks::VkIndirectCommandsLayoutCreateInfoNVX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn pipeline_bind_point<'m>(mut self, pipeline_bind_point: PipelineBindPoint) -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self.raw.pipelineBindPoint = pipeline_bind_point.into();
        self
    }

    pub fn flags<'m>(mut self, flags: IndirectCommandsLayoutUsageFlagsNvx) -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn token_count<'m>(mut self, token_count: u32) -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self.raw.tokenCount = token_count.into();
        self
    }

    pub fn tokens<'m, 'a>(mut self, tokens: &'a [IndirectCommandsLayoutTokenNvx]) -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        self.raw.pTokens = tokens.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> IndirectCommandsLayoutCreateInfoNvx<'b> {
        IndirectCommandsLayoutCreateInfoNvx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<CmdProcessCommandsInfoNvx<'s>> for vks::VkCmdProcessCommandsInfoNVX {
    fn from(f: CmdProcessCommandsInfoNvx<'s>) -> vks::VkCmdProcessCommandsInfoNVX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn object_table<'m, 'a>(mut self, object_table: &'a ObjectTableNvx) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.objectTable = object_table.handle();
        self
    }

    pub fn indirect_commands_layout<'m, 'a>(mut self, indirect_commands_layout: &'a IndirectCommandsLayoutNvx) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.indirectCommandsLayout = indirect_commands_layout.handle();
        self
    }

    pub fn indirect_commands_token_count<'m>(mut self, indirect_commands_token_count: u32) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.indirectCommandsTokenCount = indirect_commands_token_count.into();
        self
    }

    pub fn indirect_commands_tokens<'m, 'a>(mut self, indirect_commands_tokens: &'a [IndirectCommandsTokenNvx]) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.pIndirectCommandsTokens = indirect_commands_tokens.as_ptr() as *const _;
        self
    }

    pub fn max_sequences_count<'m>(mut self, max_sequences_count: u32) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.maxSequencesCount = max_sequences_count.into();
        self
    }

    pub fn target_command_buffer<'m, 'a>(mut self, target_command_buffer: &'a CommandBuffer) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.targetCommandBuffer = target_command_buffer.handle();
        self
    }

    pub fn sequences_count_buffer<'m, 'a>(mut self, sequences_count_buffer: &'a Buffer) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.sequencesCountBuffer = sequences_count_buffer.handle();
        self
    }

    pub fn sequences_count_offset<'m>(mut self, sequences_count_offset: u64) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.sequencesCountOffset = sequences_count_offset.into();
        self
    }

    pub fn sequences_index_buffer<'m, 'a>(mut self, sequences_index_buffer: &'a Buffer) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.sequencesIndexBuffer = sequences_index_buffer.handle();
        self
    }

    pub fn sequences_index_offset<'m>(mut self, sequences_index_offset: u64) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        self.raw.sequencesIndexOffset = sequences_index_offset.into();
        self
    }

    pub fn build(self) -> CmdProcessCommandsInfoNvx<'b> {
        CmdProcessCommandsInfoNvx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<CmdReserveSpaceForCommandsInfoNvx<'s>> for vks::VkCmdReserveSpaceForCommandsInfoNVX {
    fn from(f: CmdReserveSpaceForCommandsInfoNvx<'s>) -> vks::VkCmdReserveSpaceForCommandsInfoNVX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn object_table<'m, 'a>(mut self, object_table: &'a ObjectTableNvx) -> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self.raw.objectTable = object_table.handle();
        self
    }

    pub fn indirect_commands_layout<'m, 'a>(mut self, indirect_commands_layout: &'a IndirectCommandsLayoutNvx) -> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self.raw.indirectCommandsLayout = indirect_commands_layout.handle();
        self
    }

    pub fn max_sequences_count<'m>(mut self, max_sequences_count: u32) -> CmdReserveSpaceForCommandsInfoNvxBuilder<'b> {
        self.raw.maxSequencesCount = max_sequences_count.into();
        self
    }

    pub fn build(self) -> CmdReserveSpaceForCommandsInfoNvx<'b> {
        CmdReserveSpaceForCommandsInfoNvx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<ObjectTableCreateInfoNvx<'s>> for vks::VkObjectTableCreateInfoNVX {
    fn from(f: ObjectTableCreateInfoNvx<'s>) -> vks::VkObjectTableCreateInfoNVX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn object_count<'m>(mut self, object_count: u32) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.objectCount = object_count.into();
        self
    }

    pub fn object_entry_types<'m, 'a>(mut self, object_entry_types: &'a [ObjectEntryTypeNvx]) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.pObjectEntryTypes = object_entry_types.as_ptr() as *const _;
        self
    }

    pub fn object_entry_counts<'m, 'a>(mut self, object_entry_counts: &'a [u32]) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.pObjectEntryCounts = object_entry_counts.as_ptr() as *const _;
        self
    }

    pub fn object_entry_usage_flags<'m>(mut self, object_entry_usage_flags: ObjectEntryUsageFlagsNvx) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.pObjectEntryUsageFlags = object_entry_usage_flags.bits();
        self
    }

    pub fn max_uniform_buffers_per_descriptor<'m>(mut self, max_uniform_buffers_per_descriptor: u32) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.maxUniformBuffersPerDescriptor = max_uniform_buffers_per_descriptor.into();
        self
    }

    pub fn max_storage_buffers_per_descriptor<'m>(mut self, max_storage_buffers_per_descriptor: u32) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.maxStorageBuffersPerDescriptor = max_storage_buffers_per_descriptor.into();
        self
    }

    pub fn max_storage_images_per_descriptor<'m>(mut self, max_storage_images_per_descriptor: u32) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.maxStorageImagesPerDescriptor = max_storage_images_per_descriptor.into();
        self
    }

    pub fn max_sampled_images_per_descriptor<'m>(mut self, max_sampled_images_per_descriptor: u32) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.maxSampledImagesPerDescriptor = max_sampled_images_per_descriptor.into();
        self
    }

    pub fn max_pipeline_layouts<'m>(mut self, max_pipeline_layouts: u32) -> ObjectTableCreateInfoNvxBuilder<'b> {
        self.raw.maxPipelineLayouts = max_pipeline_layouts.into();
        self
    }

    pub fn build(self) -> ObjectTableCreateInfoNvx<'b> {
        ObjectTableCreateInfoNvx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl From<ObjectTableEntryNvx> for vks::VkObjectTableEntryNVX {
    fn from(f: ObjectTableEntryNvx) -> vks::VkObjectTableEntryNVX {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: ObjectEntryTypeNvx) -> ObjectTableEntryNvxBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn flags<'m>(mut self, flags: ObjectEntryUsageFlagsNvx) -> ObjectTableEntryNvxBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> ObjectTableEntryNvx {
        ObjectTableEntryNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl From<ObjectTablePipelineEntryNvx> for vks::VkObjectTablePipelineEntryNVX {
    fn from(f: ObjectTablePipelineEntryNvx) -> vks::VkObjectTablePipelineEntryNVX {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: ObjectEntryTypeNvx) -> ObjectTablePipelineEntryNvxBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn flags<'m>(mut self, flags: ObjectEntryUsageFlagsNvx) -> ObjectTablePipelineEntryNvxBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn pipeline<'m, 'a>(mut self, pipeline: &'a Pipeline) -> ObjectTablePipelineEntryNvxBuilder {
        self.raw.pipeline = pipeline.handle();
        self
    }

    pub fn build(self) -> ObjectTablePipelineEntryNvx {
        ObjectTablePipelineEntryNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl From<ObjectTableDescriptorSetEntryNvx> for vks::VkObjectTableDescriptorSetEntryNVX {
    fn from(f: ObjectTableDescriptorSetEntryNvx) -> vks::VkObjectTableDescriptorSetEntryNVX {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: ObjectEntryTypeNvx) -> ObjectTableDescriptorSetEntryNvxBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn flags<'m>(mut self, flags: ObjectEntryUsageFlagsNvx) -> ObjectTableDescriptorSetEntryNvxBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn pipeline_layout<'m, 'a>(mut self, pipeline_layout: &'a PipelineLayout) -> ObjectTableDescriptorSetEntryNvxBuilder {
        self.raw.pipelineLayout = pipeline_layout.handle();
        self
    }

    pub fn descriptor_set<'m, 'a>(mut self, descriptor_set: &'a DescriptorSet) -> ObjectTableDescriptorSetEntryNvxBuilder {
        self.raw.descriptorSet = descriptor_set.handle();
        self
    }

    pub fn build(self) -> ObjectTableDescriptorSetEntryNvx {
        ObjectTableDescriptorSetEntryNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl From<ObjectTableVertexBufferEntryNvx> for vks::VkObjectTableVertexBufferEntryNVX {
    fn from(f: ObjectTableVertexBufferEntryNvx) -> vks::VkObjectTableVertexBufferEntryNVX {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: ObjectEntryTypeNvx) -> ObjectTableVertexBufferEntryNvxBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn flags<'m>(mut self, flags: ObjectEntryUsageFlagsNvx) -> ObjectTableVertexBufferEntryNvxBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> ObjectTableVertexBufferEntryNvxBuilder {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn build(self) -> ObjectTableVertexBufferEntryNvx {
        ObjectTableVertexBufferEntryNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl From<ObjectTableIndexBufferEntryNvx> for vks::VkObjectTableIndexBufferEntryNVX {
    fn from(f: ObjectTableIndexBufferEntryNvx) -> vks::VkObjectTableIndexBufferEntryNVX {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: ObjectEntryTypeNvx) -> ObjectTableIndexBufferEntryNvxBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn flags<'m>(mut self, flags: ObjectEntryUsageFlagsNvx) -> ObjectTableIndexBufferEntryNvxBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> ObjectTableIndexBufferEntryNvxBuilder {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn index_type<'m>(mut self, index_type: IndexType) -> ObjectTableIndexBufferEntryNvxBuilder {
        self.raw.indexType = index_type.into();
        self
    }

    pub fn build(self) -> ObjectTableIndexBufferEntryNvx {
        ObjectTableIndexBufferEntryNvx {
            raw: self.raw,
        }
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


#[cfg(feature = "experimental")]
impl From<ObjectTablePushConstantEntryNvx> for vks::VkObjectTablePushConstantEntryNVX {
    fn from(f: ObjectTablePushConstantEntryNvx) -> vks::VkObjectTablePushConstantEntryNVX {
        f.raw
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

    pub fn type_of<'m>(mut self, type_of: ObjectEntryTypeNvx) -> ObjectTablePushConstantEntryNvxBuilder {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn flags<'m>(mut self, flags: ObjectEntryUsageFlagsNvx) -> ObjectTablePushConstantEntryNvxBuilder {
        self.raw.flags = flags.bits();
        self
    }

    pub fn pipeline_layout<'m, 'a>(mut self, pipeline_layout: &'a PipelineLayout) -> ObjectTablePushConstantEntryNvxBuilder {
        self.raw.pipelineLayout = pipeline_layout.handle();
        self
    }

    pub fn stage_flags<'m>(mut self, stage_flags: ShaderStageFlags) -> ObjectTablePushConstantEntryNvxBuilder {
        self.raw.stageFlags = stage_flags.bits();
        self
    }

    pub fn build(self) -> ObjectTablePushConstantEntryNvx {
        ObjectTablePushConstantEntryNvx {
            raw: self.raw,
        }
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


impl<'s> From<PhysicalDeviceFeatures2Khr<'s>> for vks::VkPhysicalDeviceFeatures2KHR {
    fn from(f: PhysicalDeviceFeatures2Khr<'s>) -> vks::VkPhysicalDeviceFeatures2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDeviceFeatures2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn features<'m>(mut self, features: PhysicalDeviceFeatures) -> PhysicalDeviceFeatures2KhrBuilder<'b> {
        self.raw.features = features.raw;
        self
    }

    pub fn build(self) -> PhysicalDeviceFeatures2Khr<'b> {
        PhysicalDeviceFeatures2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceProperties2Khr<'s>> for vks::VkPhysicalDeviceProperties2KHR {
    fn from(f: PhysicalDeviceProperties2Khr<'s>) -> vks::VkPhysicalDeviceProperties2KHR {
        f.raw
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


impl<'s> From<FormatProperties2Khr<'s>> for vks::VkFormatProperties2KHR {
    fn from(f: FormatProperties2Khr<'s>) -> vks::VkFormatProperties2KHR {
        f.raw
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


impl<'s> From<ImageFormatProperties2Khr<'s>> for vks::VkImageFormatProperties2KHR {
    fn from(f: ImageFormatProperties2Khr<'s>) -> vks::VkImageFormatProperties2KHR {
        f.raw
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


impl<'s> From<PhysicalDeviceImageFormatInfo2Khr<'s>> for vks::VkPhysicalDeviceImageFormatInfo2KHR {
    fn from(f: PhysicalDeviceImageFormatInfo2Khr<'s>) -> vks::VkPhysicalDeviceImageFormatInfo2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn format<'m>(mut self, format: Format) -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self.raw.format = format.into();
        self
    }

    pub fn type_of<'m>(mut self, type_of: ImageType) -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn tiling<'m>(mut self, tiling: ImageTiling) -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self.raw.tiling = tiling.into();
        self
    }

    pub fn usage<'m>(mut self, usage: ImageUsageFlags) -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self.raw.usage = usage.bits();
        self
    }

    pub fn flags<'m>(mut self, flags: ImageCreateFlags) -> PhysicalDeviceImageFormatInfo2KhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn build(self) -> PhysicalDeviceImageFormatInfo2Khr<'b> {
        PhysicalDeviceImageFormatInfo2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<QueueFamilyProperties2Khr<'s>> for vks::VkQueueFamilyProperties2KHR {
    fn from(f: QueueFamilyProperties2Khr<'s>) -> vks::VkQueueFamilyProperties2KHR {
        f.raw
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


impl<'s> From<PhysicalDeviceMemoryProperties2Khr<'s>> for vks::VkPhysicalDeviceMemoryProperties2KHR {
    fn from(f: PhysicalDeviceMemoryProperties2Khr<'s>) -> vks::VkPhysicalDeviceMemoryProperties2KHR {
        f.raw
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


impl<'s> From<SparseImageFormatProperties2Khr<'s>> for vks::VkSparseImageFormatProperties2KHR {
    fn from(f: SparseImageFormatProperties2Khr<'s>) -> vks::VkSparseImageFormatProperties2KHR {
        f.raw
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


impl<'s> From<PhysicalDeviceSparseImageFormatInfo2Khr<'s>> for vks::VkPhysicalDeviceSparseImageFormatInfo2KHR {
    fn from(f: PhysicalDeviceSparseImageFormatInfo2Khr<'s>) -> vks::VkPhysicalDeviceSparseImageFormatInfo2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn format<'m>(mut self, format: Format) -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self.raw.format = format.into();
        self
    }

    pub fn type_of<'m>(mut self, type_of: ImageType) -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self.raw.type_ = type_of.into();
        self
    }

    pub fn samples<'m>(mut self, samples: SampleCountFlags) -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self.raw.samples = samples.bits();
        self
    }

    pub fn usage<'m>(mut self, usage: ImageUsageFlags) -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self.raw.usage = usage.bits();
        self
    }

    pub fn tiling<'m>(mut self, tiling: ImageTiling) -> PhysicalDeviceSparseImageFormatInfo2KhrBuilder<'b> {
        self.raw.tiling = tiling.into();
        self
    }

    pub fn build(self) -> PhysicalDeviceSparseImageFormatInfo2Khr<'b> {
        PhysicalDeviceSparseImageFormatInfo2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDevicePushDescriptorPropertiesKhr<'s>> for vks::VkPhysicalDevicePushDescriptorPropertiesKHR {
    fn from(f: PhysicalDevicePushDescriptorPropertiesKhr<'s>) -> vks::VkPhysicalDevicePushDescriptorPropertiesKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn max_push_descriptors<'m>(mut self, max_push_descriptors: u32) -> PhysicalDevicePushDescriptorPropertiesKhrBuilder<'b> {
        self.raw.maxPushDescriptors = max_push_descriptors.into();
        self
    }

    pub fn build(self) -> PhysicalDevicePushDescriptorPropertiesKhr<'b> {
        PhysicalDevicePushDescriptorPropertiesKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PresentRegionsKhr<'s>> for vks::VkPresentRegionsKHR {
    fn from(f: PresentRegionsKhr<'s>) -> vks::VkPresentRegionsKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PresentRegionsKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn swapchain_count<'m>(mut self, swapchain_count: u32) -> PresentRegionsKhrBuilder<'b> {
        self.raw.swapchainCount = swapchain_count.into();
        self
    }

    pub fn regions<'m, 'a>(mut self, regions: &'a [PresentRegionKhr]) -> PresentRegionsKhrBuilder<'b> {
        self.raw.pRegions = regions.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PresentRegionsKhr<'b> {
        PresentRegionsKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PresentRegionKhr<'s>> for vks::VkPresentRegionKHR {
    fn from(f: PresentRegionKhr<'s>) -> vks::VkPresentRegionKHR {
        f.raw
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

    pub fn rectangle_count<'m>(mut self, rectangle_count: u32) -> PresentRegionKhrBuilder<'b> {
        self.raw.rectangleCount = rectangle_count.into();
        self
    }

    pub fn rectangles<'m, 'a>(mut self, rectangles: &'a [RectLayerKhr]) -> PresentRegionKhrBuilder<'b> {
        self.raw.pRectangles = rectangles.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PresentRegionKhr<'b> {
        PresentRegionKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<RectLayerKhr> for vks::VkRectLayerKHR {
    fn from(f: RectLayerKhr) -> vks::VkRectLayerKHR {
        f.raw
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

    pub fn offset<'m>(mut self, offset: Offset2d) -> RectLayerKhrBuilder {
        self.raw.offset = offset.raw;
        self
    }

    pub fn extent<'m>(mut self, extent: Extent2d) -> RectLayerKhrBuilder {
        self.raw.extent = extent.raw;
        self
    }

    pub fn layer<'m>(mut self, layer: u32) -> RectLayerKhrBuilder {
        self.raw.layer = layer.into();
        self
    }

    pub fn build(self) -> RectLayerKhr {
        RectLayerKhr {
            raw: self.raw,
        }
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


impl<'s> From<PhysicalDeviceVariablePointerFeaturesKhr<'s>> for vks::VkPhysicalDeviceVariablePointerFeaturesKHR {
    fn from(f: PhysicalDeviceVariablePointerFeaturesKhr<'s>) -> vks::VkPhysicalDeviceVariablePointerFeaturesKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn variable_pointers_storage_buffer<'m>(mut self, variable_pointers_storage_buffer: bool) -> PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        self.raw.variablePointersStorageBuffer = variable_pointers_storage_buffer as u32;
        self
    }

    pub fn variable_pointers<'m>(mut self, variable_pointers: bool) -> PhysicalDeviceVariablePointerFeaturesKhrBuilder<'b> {
        self.raw.variablePointers = variable_pointers as u32;
        self
    }

    pub fn build(self) -> PhysicalDeviceVariablePointerFeaturesKhr<'b> {
        PhysicalDeviceVariablePointerFeaturesKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<ExternalMemoryPropertiesKhr> for vks::VkExternalMemoryPropertiesKHR {
    fn from(f: ExternalMemoryPropertiesKhr) -> vks::VkExternalMemoryPropertiesKHR {
        f.raw
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


impl<'s> From<PhysicalDeviceExternalImageFormatInfoKhr<'s>> for vks::VkPhysicalDeviceExternalImageFormatInfoKHR {
    fn from(f: PhysicalDeviceExternalImageFormatInfoKhr<'s>) -> vks::VkPhysicalDeviceExternalImageFormatInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> PhysicalDeviceExternalImageFormatInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> PhysicalDeviceExternalImageFormatInfoKhr<'b> {
        PhysicalDeviceExternalImageFormatInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExternalImageFormatPropertiesKhr<'s>> for vks::VkExternalImageFormatPropertiesKHR {
    fn from(f: ExternalImageFormatPropertiesKhr<'s>) -> vks::VkExternalImageFormatPropertiesKHR {
        f.raw
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


impl<'s> From<PhysicalDeviceExternalBufferInfoKhr<'s>> for vks::VkPhysicalDeviceExternalBufferInfoKHR {
    fn from(f: PhysicalDeviceExternalBufferInfoKhr<'s>) -> vks::VkPhysicalDeviceExternalBufferInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: BufferCreateFlags) -> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn usage<'m>(mut self, usage: BufferUsageFlags) -> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self.raw.usage = usage.bits();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> PhysicalDeviceExternalBufferInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> PhysicalDeviceExternalBufferInfoKhr<'b> {
        PhysicalDeviceExternalBufferInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExternalBufferPropertiesKhr<'s>> for vks::VkExternalBufferPropertiesKHR {
    fn from(f: ExternalBufferPropertiesKhr<'s>) -> vks::VkExternalBufferPropertiesKHR {
        f.raw
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


impl<'s> From<PhysicalDeviceIDPropertiesKhr<'s>> for vks::VkPhysicalDeviceIDPropertiesKHR {
    fn from(f: PhysicalDeviceIDPropertiesKhr<'s>) -> vks::VkPhysicalDeviceIDPropertiesKHR {
        f.raw
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


impl<'s> From<ExternalMemoryImageCreateInfoKhr<'s>> for vks::VkExternalMemoryImageCreateInfoKHR {
    fn from(f: ExternalMemoryImageCreateInfoKhr<'s>) -> vks::VkExternalMemoryImageCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExternalMemoryImageCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) -> ExternalMemoryImageCreateInfoKhrBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExternalMemoryImageCreateInfoKhr<'b> {
        ExternalMemoryImageCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExternalMemoryBufferCreateInfoKhr<'s>> for vks::VkExternalMemoryBufferCreateInfoKHR {
    fn from(f: ExternalMemoryBufferCreateInfoKhr<'s>) -> vks::VkExternalMemoryBufferCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) -> ExternalMemoryBufferCreateInfoKhrBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExternalMemoryBufferCreateInfoKhr<'b> {
        ExternalMemoryBufferCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExportMemoryAllocateInfoKhr<'s>> for vks::VkExportMemoryAllocateInfoKHR {
    fn from(f: ExportMemoryAllocateInfoKhr<'s>) -> vks::VkExportMemoryAllocateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportMemoryAllocateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) -> ExportMemoryAllocateInfoKhrBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExportMemoryAllocateInfoKhr<'b> {
        ExportMemoryAllocateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportMemoryWin32HandleInfoKhr<'s>> for vks::VkImportMemoryWin32HandleInfoKHR {
    fn from(f: ImportMemoryWin32HandleInfoKhr<'s>) -> vks::VkImportMemoryWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn handle<'m>(mut self, handle: HANDLE) -> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.handle = handle.into();
        self
    }

    pub fn name<'m>(mut self, name: LPCWSTR) -> ImportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.name = name.into();
        self
    }

    pub fn build(self) -> ImportMemoryWin32HandleInfoKhr<'b> {
        ImportMemoryWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExportMemoryWin32HandleInfoKhr<'s>> for vks::VkExportMemoryWin32HandleInfoKHR {
    fn from(f: ExportMemoryWin32HandleInfoKhr<'s>) -> vks::VkExportMemoryWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn attributes<'m, 'a>(mut self, attributes: &'a [SECURITY_ATTRIBUTES]) -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.pAttributes = attributes.as_ptr() as *const _;
        self
    }

    pub fn dw_access<'m>(mut self, dw_access: DWORD) -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.dwAccess = dw_access.into();
        self
    }

    pub fn name<'m>(mut self, name: LPCWSTR) -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.name = name.into();
        self
    }

    pub fn build(self) -> ExportMemoryWin32HandleInfoKhr<'b> {
        ExportMemoryWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<MemoryWin32HandlePropertiesKhr<'s>> for vks::VkMemoryWin32HandlePropertiesKHR {
    fn from(f: MemoryWin32HandlePropertiesKhr<'s>) -> vks::VkMemoryWin32HandlePropertiesKHR {
        f.raw
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


impl<'s> From<MemoryGetWin32HandleInfoKhr<'s>> for vks::VkMemoryGetWin32HandleInfoKHR {
    fn from(f: MemoryGetWin32HandleInfoKhr<'s>) -> vks::VkMemoryGetWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MemoryGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn memory<'m, 'a>(mut self, memory: &'a DeviceMemory) -> MemoryGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.memory = memory.handle();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> MemoryGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> MemoryGetWin32HandleInfoKhr<'b> {
        MemoryGetWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportMemoryFdInfoKhr<'s>> for vks::VkImportMemoryFdInfoKHR {
    fn from(f: ImportMemoryFdInfoKhr<'s>) -> vks::VkImportMemoryFdInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportMemoryFdInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> ImportMemoryFdInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn fd<'m>(mut self, fd: i32) -> ImportMemoryFdInfoKhrBuilder<'b> {
        self.raw.fd = fd.into();
        self
    }

    pub fn build(self) -> ImportMemoryFdInfoKhr<'b> {
        ImportMemoryFdInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<MemoryFdPropertiesKhr<'s>> for vks::VkMemoryFdPropertiesKHR {
    fn from(f: MemoryFdPropertiesKhr<'s>) -> vks::VkMemoryFdPropertiesKHR {
        f.raw
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


impl<'s> From<MemoryGetFdInfoKhr<'s>> for vks::VkMemoryGetFdInfoKHR {
    fn from(f: MemoryGetFdInfoKhr<'s>) -> vks::VkMemoryGetFdInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MemoryGetFdInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn memory<'m, 'a>(mut self, memory: &'a DeviceMemory) -> MemoryGetFdInfoKhrBuilder<'b> {
        self.raw.memory = memory.handle();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) -> MemoryGetFdInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> MemoryGetFdInfoKhr<'b> {
        MemoryGetFdInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkWin32KeyedMutexAcquireReleaseInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKhr<'s> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    acquire_syncs: Option<Vec<vks::VkDeviceMemory>>,
    release_syncs: Option<Vec<vks::VkDeviceMemory>>,
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


impl<'s> From<Win32KeyedMutexAcquireReleaseInfoKhr<'s>> for vks::VkWin32KeyedMutexAcquireReleaseInfoKHR {
    fn from(f: Win32KeyedMutexAcquireReleaseInfoKhr<'s>) -> vks::VkWin32KeyedMutexAcquireReleaseInfoKHR {
        f.raw
    }
}


/// A builder for `VkWin32KeyedMutexAcquireReleaseInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    acquire_syncs: Option<Vec<vks::VkDeviceMemory>>,
    release_syncs: Option<Vec<vks::VkDeviceMemory>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
    pub fn new() -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoKhrBuilder {
            raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR::default(),
            acquire_syncs: None,
            release_syncs: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn acquire_count<'m>(mut self, acquire_count: u32) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self.raw.acquireCount = acquire_count.into();
        self
    }

    pub fn acquire_syncs<'m, 'a>(mut self, acquire_syncs: &'a [DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> where 'a: 'b {
        self.acquire_syncs = Some(acquire_syncs.iter().map(|h| h.handle()).collect());
        self.raw.pAcquireSyncs = self.acquire_syncs.as_ref().unwrap().as_ptr();
        self
    }

    pub fn acquire_keys<'m, 'a>(mut self, acquire_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self.raw.pAcquireKeys = acquire_keys.as_ptr() as *const _;
        self
    }

    pub fn acquire_timeouts<'m, 'a>(mut self, acquire_timeouts: &'a [u32]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self.raw.pAcquireTimeouts = acquire_timeouts.as_ptr() as *const _;
        self
    }

    pub fn release_count<'m>(mut self, release_count: u32) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self.raw.releaseCount = release_count.into();
        self
    }

    pub fn release_syncs<'m, 'a>(mut self, release_syncs: &'a [DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> where 'a: 'b {
        self.release_syncs = Some(release_syncs.iter().map(|h| h.handle()).collect());
        self.raw.pReleaseSyncs = self.release_syncs.as_ref().unwrap().as_ptr();
        self
    }

    pub fn release_keys<'m, 'a>(mut self, release_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        self.raw.pReleaseKeys = release_keys.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> Win32KeyedMutexAcquireReleaseInfoKhr<'b> {
        Win32KeyedMutexAcquireReleaseInfoKhr {
            raw: self.raw,
            acquire_syncs: self.acquire_syncs,
            release_syncs: self.release_syncs,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceExternalSemaphoreInfoKhr<'s>> for vks::VkPhysicalDeviceExternalSemaphoreInfoKHR {
    fn from(f: PhysicalDeviceExternalSemaphoreInfoKhr<'s>) -> vks::VkPhysicalDeviceExternalSemaphoreInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> PhysicalDeviceExternalSemaphoreInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> PhysicalDeviceExternalSemaphoreInfoKhr<'b> {
        PhysicalDeviceExternalSemaphoreInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExternalSemaphorePropertiesKhr<'s>> for vks::VkExternalSemaphorePropertiesKHR {
    fn from(f: ExternalSemaphorePropertiesKhr<'s>) -> vks::VkExternalSemaphorePropertiesKHR {
        f.raw
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


impl<'s> From<ExportSemaphoreCreateInfoKhr<'s>> for vks::VkExportSemaphoreCreateInfoKHR {
    fn from(f: ExportSemaphoreCreateInfoKhr<'s>) -> vks::VkExportSemaphoreCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportSemaphoreCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalSemaphoreHandleTypeFlagsKhr) -> ExportSemaphoreCreateInfoKhrBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExportSemaphoreCreateInfoKhr<'b> {
        ExportSemaphoreCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportSemaphoreWin32HandleInfoKhr<'s>> for vks::VkImportSemaphoreWin32HandleInfoKHR {
    fn from(f: ImportSemaphoreWin32HandleInfoKhr<'s>) -> vks::VkImportSemaphoreWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn semaphore<'m, 'a>(mut self, semaphore: &'a Semaphore) -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.semaphore = semaphore.handle();
        self
    }

    pub fn flags<'m>(mut self, flags: SemaphoreImportFlagsKhr) -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn handle<'m>(mut self, handle: HANDLE) -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.handle = handle.into();
        self
    }

    pub fn name<'m>(mut self, name: LPCWSTR) -> ImportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.name = name.into();
        self
    }

    pub fn build(self) -> ImportSemaphoreWin32HandleInfoKhr<'b> {
        ImportSemaphoreWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExportSemaphoreWin32HandleInfoKhr<'s>> for vks::VkExportSemaphoreWin32HandleInfoKHR {
    fn from(f: ExportSemaphoreWin32HandleInfoKhr<'s>) -> vks::VkExportSemaphoreWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn attributes<'m, 'a>(mut self, attributes: &'a [SECURITY_ATTRIBUTES]) -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.pAttributes = attributes.as_ptr() as *const _;
        self
    }

    pub fn dw_access<'m>(mut self, dw_access: DWORD) -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.dwAccess = dw_access.into();
        self
    }

    pub fn name<'m>(mut self, name: LPCWSTR) -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.name = name.into();
        self
    }

    pub fn build(self) -> ExportSemaphoreWin32HandleInfoKhr<'b> {
        ExportSemaphoreWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<D3d12FenceSubmitInfoKHR<'s>> for vks::VkD3D12FenceSubmitInfoKHR {
    fn from(f: D3d12FenceSubmitInfoKHR<'s>) -> vks::VkD3D12FenceSubmitInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphore_values_count<'m>(mut self, wait_semaphore_values_count: u32) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        self.raw.waitSemaphoreValuesCount = wait_semaphore_values_count.into();
        self
    }

    pub fn wait_semaphore_values<'m, 'a>(mut self, wait_semaphore_values: &'a [u64]) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        self.raw.pWaitSemaphoreValues = wait_semaphore_values.as_ptr() as *const _;
        self
    }

    pub fn signal_semaphore_values_count<'m>(mut self, signal_semaphore_values_count: u32) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        self.raw.signalSemaphoreValuesCount = signal_semaphore_values_count.into();
        self
    }

    pub fn signal_semaphore_values<'m, 'a>(mut self, signal_semaphore_values: &'a [u64]) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        self.raw.pSignalSemaphoreValues = signal_semaphore_values.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> D3d12FenceSubmitInfoKHR<'b> {
        D3d12FenceSubmitInfoKHR {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SemaphoreGetWin32HandleInfoKhr<'s>> for vks::VkSemaphoreGetWin32HandleInfoKHR {
    fn from(f: SemaphoreGetWin32HandleInfoKhr<'s>) -> vks::VkSemaphoreGetWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn semaphore<'m, 'a>(mut self, semaphore: &'a Semaphore) -> SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.semaphore = semaphore.handle();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> SemaphoreGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> SemaphoreGetWin32HandleInfoKhr<'b> {
        SemaphoreGetWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportSemaphoreFdInfoKhr<'s>> for vks::VkImportSemaphoreFdInfoKHR {
    fn from(f: ImportSemaphoreFdInfoKhr<'s>) -> vks::VkImportSemaphoreFdInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn semaphore<'m, 'a>(mut self, semaphore: &'a Semaphore) -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        self.raw.semaphore = semaphore.handle();
        self
    }

    pub fn flags<'m>(mut self, flags: SemaphoreImportFlagsKhr) -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn fd<'m>(mut self, fd: i32) -> ImportSemaphoreFdInfoKhrBuilder<'b> {
        self.raw.fd = fd.into();
        self
    }

    pub fn build(self) -> ImportSemaphoreFdInfoKhr<'b> {
        ImportSemaphoreFdInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SemaphoreGetFdInfoKhr<'s>> for vks::VkSemaphoreGetFdInfoKHR {
    fn from(f: SemaphoreGetFdInfoKhr<'s>) -> vks::VkSemaphoreGetFdInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SemaphoreGetFdInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn semaphore<'m, 'a>(mut self, semaphore: &'a Semaphore) -> SemaphoreGetFdInfoKhrBuilder<'b> {
        self.raw.semaphore = semaphore.handle();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) -> SemaphoreGetFdInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> SemaphoreGetFdInfoKhr<'b> {
        SemaphoreGetFdInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceExternalFenceInfoKhr<'s>> for vks::VkPhysicalDeviceExternalFenceInfoKHR {
    fn from(f: PhysicalDeviceExternalFenceInfoKhr<'s>) -> vks::VkPhysicalDeviceExternalFenceInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> PhysicalDeviceExternalFenceInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> PhysicalDeviceExternalFenceInfoKhr<'b> {
        PhysicalDeviceExternalFenceInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExternalFencePropertiesKhr<'s>> for vks::VkExternalFencePropertiesKHR {
    fn from(f: ExternalFencePropertiesKhr<'s>) -> vks::VkExternalFencePropertiesKHR {
        f.raw
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


impl<'s> From<ExportFenceCreateInfoKhr<'s>> for vks::VkExportFenceCreateInfoKHR {
    fn from(f: ExportFenceCreateInfoKhr<'s>) -> vks::VkExportFenceCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportFenceCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn handle_types<'m>(mut self, handle_types: ExternalFenceHandleTypeFlagsKhr) -> ExportFenceCreateInfoKhrBuilder<'b> {
        self.raw.handleTypes = handle_types.bits();
        self
    }

    pub fn build(self) -> ExportFenceCreateInfoKhr<'b> {
        ExportFenceCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportFenceWin32HandleInfoKhr<'s>> for vks::VkImportFenceWin32HandleInfoKHR {
    fn from(f: ImportFenceWin32HandleInfoKhr<'s>) -> vks::VkImportFenceWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn fence<'m, 'a>(mut self, fence: &'a Fence) -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.fence = fence.handle();
        self
    }

    pub fn flags<'m>(mut self, flags: FenceImportFlagsKhr) -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn handle<'m>(mut self, handle: HANDLE) -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.handle = handle.into();
        self
    }

    pub fn name<'m>(mut self, name: LPCWSTR) -> ImportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.name = name.into();
        self
    }

    pub fn build(self) -> ImportFenceWin32HandleInfoKhr<'b> {
        ImportFenceWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ExportFenceWin32HandleInfoKhr<'s>> for vks::VkExportFenceWin32HandleInfoKHR {
    fn from(f: ExportFenceWin32HandleInfoKhr<'s>) -> vks::VkExportFenceWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn attributes<'m, 'a>(mut self, attributes: &'a [SECURITY_ATTRIBUTES]) -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.pAttributes = attributes.as_ptr() as *const _;
        self
    }

    pub fn dw_access<'m>(mut self, dw_access: DWORD) -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.dwAccess = dw_access.into();
        self
    }

    pub fn name<'m>(mut self, name: LPCWSTR) -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.name = name.into();
        self
    }

    pub fn build(self) -> ExportFenceWin32HandleInfoKhr<'b> {
        ExportFenceWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<FenceGetWin32HandleInfoKhr<'s>> for vks::VkFenceGetWin32HandleInfoKHR {
    fn from(f: FenceGetWin32HandleInfoKhr<'s>) -> vks::VkFenceGetWin32HandleInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> FenceGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn fence<'m, 'a>(mut self, fence: &'a Fence) -> FenceGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.fence = fence.handle();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> FenceGetWin32HandleInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> FenceGetWin32HandleInfoKhr<'b> {
        FenceGetWin32HandleInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImportFenceFdInfoKhr<'s>> for vks::VkImportFenceFdInfoKHR {
    fn from(f: ImportFenceFdInfoKhr<'s>) -> vks::VkImportFenceFdInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImportFenceFdInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn fence<'m, 'a>(mut self, fence: &'a Fence) -> ImportFenceFdInfoKhrBuilder<'b> {
        self.raw.fence = fence.handle();
        self
    }

    pub fn flags<'m>(mut self, flags: FenceImportFlagsKhr) -> ImportFenceFdInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> ImportFenceFdInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn fd<'m>(mut self, fd: i32) -> ImportFenceFdInfoKhrBuilder<'b> {
        self.raw.fd = fd.into();
        self
    }

    pub fn build(self) -> ImportFenceFdInfoKhr<'b> {
        ImportFenceFdInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<FenceGetFdInfoKhr<'s>> for vks::VkFenceGetFdInfoKHR {
    fn from(f: FenceGetFdInfoKhr<'s>) -> vks::VkFenceGetFdInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> FenceGetFdInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn fence<'m, 'a>(mut self, fence: &'a Fence) -> FenceGetFdInfoKhrBuilder<'b> {
        self.raw.fence = fence.handle();
        self
    }

    pub fn handle_type<'m>(mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) -> FenceGetFdInfoKhrBuilder<'b> {
        self.raw.handleType = handle_type.bits();
        self
    }

    pub fn build(self) -> FenceGetFdInfoKhr<'b> {
        FenceGetFdInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<PhysicalDeviceMultiviewFeaturesKhx<'s>> for vks::VkPhysicalDeviceMultiviewFeaturesKHX {
    fn from(f: PhysicalDeviceMultiviewFeaturesKhx<'s>) -> vks::VkPhysicalDeviceMultiviewFeaturesKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn multiview<'m>(mut self, multiview: bool) -> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self.raw.multiview = multiview as u32;
        self
    }

    pub fn multiview_geometry_shader<'m>(mut self, multiview_geometry_shader: bool) -> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self.raw.multiviewGeometryShader = multiview_geometry_shader as u32;
        self
    }

    pub fn multiview_tessellation_shader<'m>(mut self, multiview_tessellation_shader: bool) -> PhysicalDeviceMultiviewFeaturesKhxBuilder<'b> {
        self.raw.multiviewTessellationShader = multiview_tessellation_shader as u32;
        self
    }

    pub fn build(self) -> PhysicalDeviceMultiviewFeaturesKhx<'b> {
        PhysicalDeviceMultiviewFeaturesKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<PhysicalDeviceMultiviewPropertiesKhx<'s>> for vks::VkPhysicalDeviceMultiviewPropertiesKHX {
    fn from(f: PhysicalDeviceMultiviewPropertiesKhx<'s>) -> vks::VkPhysicalDeviceMultiviewPropertiesKHX {
        f.raw
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


#[cfg(feature = "experimental")]
impl<'s> From<RenderPassMultiviewCreateInfoKhx<'s>> for vks::VkRenderPassMultiviewCreateInfoKHX {
    fn from(f: RenderPassMultiviewCreateInfoKhx<'s>) -> vks::VkRenderPassMultiviewCreateInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn subpass_count<'m>(mut self, subpass_count: u32) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.subpassCount = subpass_count.into();
        self
    }

    pub fn view_masks<'m, 'a>(mut self, view_masks: &'a [u32]) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.pViewMasks = view_masks.as_ptr() as *const _;
        self
    }

    pub fn dependency_count<'m>(mut self, dependency_count: u32) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.dependencyCount = dependency_count.into();
        self
    }

    pub fn view_offsets<'m, 'a>(mut self, view_offsets: &'a [i32]) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.pViewOffsets = view_offsets.as_ptr() as *const _;
        self
    }

    pub fn correlation_mask_count<'m>(mut self, correlation_mask_count: u32) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.correlationMaskCount = correlation_mask_count.into();
        self
    }

    pub fn correlation_masks<'m, 'a>(mut self, correlation_masks: &'a [u32]) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.pCorrelationMasks = correlation_masks.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> RenderPassMultiviewCreateInfoKhx<'b> {
        RenderPassMultiviewCreateInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SurfaceCapabilities2Ext<'s>> for vks::VkSurfaceCapabilities2EXT {
    fn from(f: SurfaceCapabilities2Ext<'s>) -> vks::VkSurfaceCapabilities2EXT {
        f.raw
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


impl<'s> From<DisplayPowerInfoExt<'s>> for vks::VkDisplayPowerInfoEXT {
    fn from(f: DisplayPowerInfoExt<'s>) -> vks::VkDisplayPowerInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DisplayPowerInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn power_state<'m>(mut self, power_state: DisplayPowerStateExt) -> DisplayPowerInfoExtBuilder<'b> {
        self.raw.powerState = power_state.into();
        self
    }

    pub fn build(self) -> DisplayPowerInfoExt<'b> {
        DisplayPowerInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<DeviceEventInfoExt<'s>> for vks::VkDeviceEventInfoEXT {
    fn from(f: DeviceEventInfoExt<'s>) -> vks::VkDeviceEventInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceEventInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn device_event<'m>(mut self, device_event: DeviceEventTypeExt) -> DeviceEventInfoExtBuilder<'b> {
        self.raw.deviceEvent = device_event.into();
        self
    }

    pub fn build(self) -> DeviceEventInfoExt<'b> {
        DeviceEventInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<DisplayEventInfoExt<'s>> for vks::VkDisplayEventInfoEXT {
    fn from(f: DisplayEventInfoExt<'s>) -> vks::VkDisplayEventInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DisplayEventInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn display_event<'m>(mut self, display_event: DisplayEventTypeExt) -> DisplayEventInfoExtBuilder<'b> {
        self.raw.displayEvent = display_event.into();
        self
    }

    pub fn build(self) -> DisplayEventInfoExt<'b> {
        DisplayEventInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SwapchainCounterCreateInfoExt<'s>> for vks::VkSwapchainCounterCreateInfoEXT {
    fn from(f: SwapchainCounterCreateInfoExt<'s>) -> vks::VkSwapchainCounterCreateInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SwapchainCounterCreateInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn surface_counters<'m>(mut self, surface_counters: SurfaceCounterFlagsExt) -> SwapchainCounterCreateInfoExtBuilder<'b> {
        self.raw.surfaceCounters = surface_counters.bits();
        self
    }

    pub fn build(self) -> SwapchainCounterCreateInfoExt<'b> {
        SwapchainCounterCreateInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<PhysicalDeviceGroupPropertiesKhx<'s>> for vks::VkPhysicalDeviceGroupPropertiesKHX {
    fn from(f: PhysicalDeviceGroupPropertiesKhx<'s>) -> vks::VkPhysicalDeviceGroupPropertiesKHX {
        f.raw
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


#[cfg(feature = "experimental")]
impl<'s> From<MemoryAllocateFlagsInfoKhx<'s>> for vks::VkMemoryAllocateFlagsInfoKHX {
    fn from(f: MemoryAllocateFlagsInfoKhx<'s>) -> vks::VkMemoryAllocateFlagsInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MemoryAllocateFlagsInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: MemoryAllocateFlagsKhx) -> MemoryAllocateFlagsInfoKhxBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn device_mask<'m>(mut self, device_mask: u32) -> MemoryAllocateFlagsInfoKhxBuilder<'b> {
        self.raw.deviceMask = device_mask.into();
        self
    }

    pub fn build(self) -> MemoryAllocateFlagsInfoKhx<'b> {
        MemoryAllocateFlagsInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<BindBufferMemoryDeviceGroupInfoKhx<'s>> for vks::VkBindBufferMemoryDeviceGroupInfoKHX {
    fn from(f: BindBufferMemoryDeviceGroupInfoKhx<'s>) -> vks::VkBindBufferMemoryDeviceGroupInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn device_index_count<'m>(mut self, device_index_count: u32) -> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.deviceIndexCount = device_index_count.into();
        self
    }

    pub fn device_indices<'m, 'a>(mut self, device_indices: &'a [u32]) -> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.pDeviceIndices = device_indices.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> BindBufferMemoryDeviceGroupInfoKhx<'b> {
        BindBufferMemoryDeviceGroupInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<BindImageMemoryDeviceGroupInfoKhx<'s>> for vks::VkBindImageMemoryDeviceGroupInfoKHX {
    fn from(f: BindImageMemoryDeviceGroupInfoKhx<'s>) -> vks::VkBindImageMemoryDeviceGroupInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn device_index_count<'m>(mut self, device_index_count: u32) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.deviceIndexCount = device_index_count.into();
        self
    }

    pub fn device_indices<'m, 'a>(mut self, device_indices: &'a [u32]) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.pDeviceIndices = device_indices.as_ptr() as *const _;
        self
    }

    pub fn s_fr_rect_count<'m>(mut self, s_fr_rect_count: u32) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.SFRRectCount = s_fr_rect_count.into();
        self
    }

    pub fn s_fr_rects<'m, 'a>(mut self, s_fr_rects: &'a [Rect2d]) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        self.raw.pSFRRects = s_fr_rects.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> BindImageMemoryDeviceGroupInfoKhx<'b> {
        BindImageMemoryDeviceGroupInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupRenderPassBeginInfoKhx<'s>> for vks::VkDeviceGroupRenderPassBeginInfoKHX {
    fn from(f: DeviceGroupRenderPassBeginInfoKhx<'s>) -> vks::VkDeviceGroupRenderPassBeginInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn device_mask<'m>(mut self, device_mask: u32) -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self.raw.deviceMask = device_mask.into();
        self
    }

    pub fn device_render_area_count<'m>(mut self, device_render_area_count: u32) -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self.raw.deviceRenderAreaCount = device_render_area_count.into();
        self
    }

    pub fn device_render_areas<'m, 'a>(mut self, device_render_areas: &'a [Rect2d]) -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        self.raw.pDeviceRenderAreas = device_render_areas.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> DeviceGroupRenderPassBeginInfoKhx<'b> {
        DeviceGroupRenderPassBeginInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupCommandBufferBeginInfoKhx<'s>> for vks::VkDeviceGroupCommandBufferBeginInfoKHX {
    fn from(f: DeviceGroupCommandBufferBeginInfoKhx<'s>) -> vks::VkDeviceGroupCommandBufferBeginInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn device_mask<'m>(mut self, device_mask: u32) -> DeviceGroupCommandBufferBeginInfoKhxBuilder<'b> {
        self.raw.deviceMask = device_mask.into();
        self
    }

    pub fn build(self) -> DeviceGroupCommandBufferBeginInfoKhx<'b> {
        DeviceGroupCommandBufferBeginInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupSubmitInfoKhx<'s>> for vks::VkDeviceGroupSubmitInfoKHX {
    fn from(f: DeviceGroupSubmitInfoKhx<'s>) -> vks::VkDeviceGroupSubmitInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphore_count<'m>(mut self, wait_semaphore_count: u32) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.waitSemaphoreCount = wait_semaphore_count.into();
        self
    }

    pub fn wait_semaphore_device_indices<'m, 'a>(mut self, wait_semaphore_device_indices: &'a [u32]) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.pWaitSemaphoreDeviceIndices = wait_semaphore_device_indices.as_ptr() as *const _;
        self
    }

    pub fn command_buffer_count<'m>(mut self, command_buffer_count: u32) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.commandBufferCount = command_buffer_count.into();
        self
    }

    pub fn command_buffer_device_masks<'m, 'a>(mut self, command_buffer_device_masks: &'a [u32]) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.pCommandBufferDeviceMasks = command_buffer_device_masks.as_ptr() as *const _;
        self
    }

    pub fn signal_semaphore_count<'m>(mut self, signal_semaphore_count: u32) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.signalSemaphoreCount = signal_semaphore_count.into();
        self
    }

    pub fn signal_semaphore_device_indices<'m, 'a>(mut self, signal_semaphore_device_indices: &'a [u32]) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        self.raw.pSignalSemaphoreDeviceIndices = signal_semaphore_device_indices.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> DeviceGroupSubmitInfoKhx<'b> {
        DeviceGroupSubmitInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupBindSparseInfoKhx<'s>> for vks::VkDeviceGroupBindSparseInfoKHX {
    fn from(f: DeviceGroupBindSparseInfoKhx<'s>) -> vks::VkDeviceGroupBindSparseInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupBindSparseInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn resource_device_index<'m>(mut self, resource_device_index: u32) -> DeviceGroupBindSparseInfoKhxBuilder<'b> {
        self.raw.resourceDeviceIndex = resource_device_index.into();
        self
    }

    pub fn memory_device_index<'m>(mut self, memory_device_index: u32) -> DeviceGroupBindSparseInfoKhxBuilder<'b> {
        self.raw.memoryDeviceIndex = memory_device_index.into();
        self
    }

    pub fn build(self) -> DeviceGroupBindSparseInfoKhx<'b> {
        DeviceGroupBindSparseInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupPresentCapabilitiesKhx<'s>> for vks::VkDeviceGroupPresentCapabilitiesKHX {
    fn from(f: DeviceGroupPresentCapabilitiesKhx<'s>) -> vks::VkDeviceGroupPresentCapabilitiesKHX {
        f.raw
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


#[cfg(feature = "experimental")]
impl<'s> From<ImageSwapchainCreateInfoKhx<'s>> for vks::VkImageSwapchainCreateInfoKHX {
    fn from(f: ImageSwapchainCreateInfoKhx<'s>) -> vks::VkImageSwapchainCreateInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImageSwapchainCreateInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn swapchain<'m, 'a>(mut self, swapchain: &'a Swapchain) -> ImageSwapchainCreateInfoKhxBuilder<'b> {
        self.raw.swapchain = swapchain.handle();
        self
    }

    pub fn build(self) -> ImageSwapchainCreateInfoKhx<'b> {
        ImageSwapchainCreateInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<BindImageMemorySwapchainInfoKhx<'s>> for vks::VkBindImageMemorySwapchainInfoKHX {
    fn from(f: BindImageMemorySwapchainInfoKhx<'s>) -> vks::VkBindImageMemorySwapchainInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BindImageMemorySwapchainInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn swapchain<'m, 'a>(mut self, swapchain: &'a Swapchain) -> BindImageMemorySwapchainInfoKhxBuilder<'b> {
        self.raw.swapchain = swapchain.handle();
        self
    }

    pub fn image_index<'m>(mut self, image_index: u32) -> BindImageMemorySwapchainInfoKhxBuilder<'b> {
        self.raw.imageIndex = image_index.into();
        self
    }

    pub fn build(self) -> BindImageMemorySwapchainInfoKhx<'b> {
        BindImageMemorySwapchainInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<AcquireNextImageInfoKhx<'s>> for vks::VkAcquireNextImageInfoKHX {
    fn from(f: AcquireNextImageInfoKhx<'s>) -> vks::VkAcquireNextImageInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> AcquireNextImageInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn swapchain<'m, 'a>(mut self, swapchain: &'a Swapchain) -> AcquireNextImageInfoKhxBuilder<'b> {
        self.raw.swapchain = swapchain.handle();
        self
    }

    pub fn timeout<'m>(mut self, timeout: u64) -> AcquireNextImageInfoKhxBuilder<'b> {
        self.raw.timeout = timeout.into();
        self
    }

    pub fn semaphore<'m, 'a>(mut self, semaphore: &'a Semaphore) -> AcquireNextImageInfoKhxBuilder<'b> {
        self.raw.semaphore = semaphore.handle();
        self
    }

    pub fn fence<'m, 'a>(mut self, fence: &'a Fence) -> AcquireNextImageInfoKhxBuilder<'b> {
        self.raw.fence = fence.handle();
        self
    }

    pub fn device_mask<'m>(mut self, device_mask: u32) -> AcquireNextImageInfoKhxBuilder<'b> {
        self.raw.deviceMask = device_mask.into();
        self
    }

    pub fn build(self) -> AcquireNextImageInfoKhx<'b> {
        AcquireNextImageInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupPresentInfoKhx<'s>> for vks::VkDeviceGroupPresentInfoKHX {
    fn from(f: DeviceGroupPresentInfoKhx<'s>) -> vks::VkDeviceGroupPresentInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupPresentInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn swapchain_count<'m>(mut self, swapchain_count: u32) -> DeviceGroupPresentInfoKhxBuilder<'b> {
        self.raw.swapchainCount = swapchain_count.into();
        self
    }

    pub fn device_masks<'m, 'a>(mut self, device_masks: &'a [u32]) -> DeviceGroupPresentInfoKhxBuilder<'b> {
        self.raw.pDeviceMasks = device_masks.as_ptr() as *const _;
        self
    }

    pub fn mode<'m>(mut self, mode: DeviceGroupPresentModeFlagsKhx) -> DeviceGroupPresentInfoKhxBuilder<'b> {
        self.raw.mode = mode.bits();
        self
    }

    pub fn build(self) -> DeviceGroupPresentInfoKhx<'b> {
        DeviceGroupPresentInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


/// A `VkDeviceGroupDeviceCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupDeviceCreateInfoKhx<'s> {
    raw: vks::VkDeviceGroupDeviceCreateInfoKHX,
    physical_devices: Option<Vec<vks::VkPhysicalDevice>>,
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupDeviceCreateInfoKhx<'s>> for vks::VkDeviceGroupDeviceCreateInfoKHX {
    fn from(f: DeviceGroupDeviceCreateInfoKhx<'s>) -> vks::VkDeviceGroupDeviceCreateInfoKHX {
        f.raw
    }
}


/// A builder for `VkDeviceGroupDeviceCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupDeviceCreateInfoKHX,
    physical_devices: Option<Vec<vks::VkPhysicalDevice>>,
    _p: PhantomData<&'b ()>,
}

#[cfg(feature = "experimental")]
impl<'b> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
    pub fn new() -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        DeviceGroupDeviceCreateInfoKhxBuilder {
            raw: vks::VkDeviceGroupDeviceCreateInfoKHX::default(),
            physical_devices: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn physical_device_count<'m>(mut self, physical_device_count: u32) -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        self.raw.physicalDeviceCount = physical_device_count.into();
        self
    }

    pub fn physical_devices<'m, 'a>(mut self, physical_devices: &'a [PhysicalDevice]) -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> where 'a: 'b {
        self.physical_devices = Some(physical_devices.iter().map(|h| h.handle()).collect());
        self.raw.pPhysicalDevices = self.physical_devices.as_ref().unwrap().as_ptr();
        self
    }

    pub fn build(self) -> DeviceGroupDeviceCreateInfoKhx<'b> {
        DeviceGroupDeviceCreateInfoKhx {
            raw: self.raw,
            physical_devices: self.physical_devices,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<DeviceGroupSwapchainCreateInfoKhx<'s>> for vks::VkDeviceGroupSwapchainCreateInfoKHX {
    fn from(f: DeviceGroupSwapchainCreateInfoKhx<'s>) -> vks::VkDeviceGroupSwapchainCreateInfoKHX {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn modes<'m>(mut self, modes: DeviceGroupPresentModeFlagsKhx) -> DeviceGroupSwapchainCreateInfoKhxBuilder<'b> {
        self.raw.modes = modes.bits();
        self
    }

    pub fn build(self) -> DeviceGroupSwapchainCreateInfoKhx<'b> {
        DeviceGroupSwapchainCreateInfoKhx {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<DescriptorUpdateTemplateEntryKhr> for vks::VkDescriptorUpdateTemplateEntryKHR {
    fn from(f: DescriptorUpdateTemplateEntryKhr) -> vks::VkDescriptorUpdateTemplateEntryKHR {
        f.raw
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

    pub fn dst_binding<'m>(mut self, dst_binding: u32) -> DescriptorUpdateTemplateEntryKhrBuilder {
        self.raw.dstBinding = dst_binding.into();
        self
    }

    pub fn dst_array_element<'m>(mut self, dst_array_element: u32) -> DescriptorUpdateTemplateEntryKhrBuilder {
        self.raw.dstArrayElement = dst_array_element.into();
        self
    }

    pub fn descriptor_count<'m>(mut self, descriptor_count: u32) -> DescriptorUpdateTemplateEntryKhrBuilder {
        self.raw.descriptorCount = descriptor_count.into();
        self
    }

    pub fn descriptor_type<'m>(mut self, descriptor_type: DescriptorType) -> DescriptorUpdateTemplateEntryKhrBuilder {
        self.raw.descriptorType = descriptor_type.into();
        self
    }

    pub fn offset<'m>(mut self, offset: usize) -> DescriptorUpdateTemplateEntryKhrBuilder {
        self.raw.offset = offset.into();
        self
    }

    pub fn stride<'m>(mut self, stride: usize) -> DescriptorUpdateTemplateEntryKhrBuilder {
        self.raw.stride = stride.into();
        self
    }

    pub fn build(self) -> DescriptorUpdateTemplateEntryKhr {
        DescriptorUpdateTemplateEntryKhr {
            raw: self.raw,
        }
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


impl<'s> From<DescriptorUpdateTemplateCreateInfoKhr<'s>> for vks::VkDescriptorUpdateTemplateCreateInfoKHR {
    fn from(f: DescriptorUpdateTemplateCreateInfoKhr<'s>) -> vks::VkDescriptorUpdateTemplateCreateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: DescriptorUpdateTemplateCreateFlagsKhr) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn descriptor_update_entry_count<'m>(mut self, descriptor_update_entry_count: u32) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.descriptorUpdateEntryCount = descriptor_update_entry_count.into();
        self
    }

    pub fn descriptor_update_entries<'m, 'a>(mut self, descriptor_update_entries: &'a [DescriptorUpdateTemplateEntryKhr]) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.pDescriptorUpdateEntries = descriptor_update_entries.as_ptr() as *const _;
        self
    }

    pub fn template_type<'m>(mut self, template_type: DescriptorUpdateTemplateTypeKhr) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.templateType = template_type.into();
        self
    }

    pub fn descriptor_set_layout<'m, 'a>(mut self, descriptor_set_layout: &'a DescriptorSetLayout) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.descriptorSetLayout = descriptor_set_layout.handle();
        self
    }

    pub fn pipeline_bind_point<'m>(mut self, pipeline_bind_point: PipelineBindPoint) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.pipelineBindPoint = pipeline_bind_point.into();
        self
    }

    pub fn pipeline_layout<'m, 'a>(mut self, pipeline_layout: &'a PipelineLayout) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.pipelineLayout = pipeline_layout.handle();
        self
    }

    pub fn set<'m>(mut self, set: u32) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        self.raw.set = set.into();
        self
    }

    pub fn build(self) -> DescriptorUpdateTemplateCreateInfoKhr<'b> {
        DescriptorUpdateTemplateCreateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<XYColorExt> for vks::VkXYColorEXT {
    fn from(f: XYColorExt) -> vks::VkXYColorEXT {
        f.raw
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

    pub fn x<'m>(mut self, x: f32) -> XYColorExtBuilder {
        self.raw.x = x.into();
        self
    }

    pub fn y<'m>(mut self, y: f32) -> XYColorExtBuilder {
        self.raw.y = y.into();
        self
    }

    pub fn build(self) -> XYColorExt {
        XYColorExt {
            raw: self.raw,
        }
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


impl<'s> From<HdrMetadataExt<'s>> for vks::VkHdrMetadataEXT {
    fn from(f: HdrMetadataExt<'s>) -> vks::VkHdrMetadataEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> HdrMetadataExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn display_primary_red<'m>(mut self, display_primary_red: XYColorExt) -> HdrMetadataExtBuilder<'b> {
        self.raw.displayPrimaryRed = display_primary_red.raw;
        self
    }

    pub fn display_primary_green<'m>(mut self, display_primary_green: XYColorExt) -> HdrMetadataExtBuilder<'b> {
        self.raw.displayPrimaryGreen = display_primary_green.raw;
        self
    }

    pub fn display_primary_blue<'m>(mut self, display_primary_blue: XYColorExt) -> HdrMetadataExtBuilder<'b> {
        self.raw.displayPrimaryBlue = display_primary_blue.raw;
        self
    }

    pub fn white_point<'m>(mut self, white_point: XYColorExt) -> HdrMetadataExtBuilder<'b> {
        self.raw.whitePoint = white_point.raw;
        self
    }

    pub fn max_luminance<'m>(mut self, max_luminance: f32) -> HdrMetadataExtBuilder<'b> {
        self.raw.maxLuminance = max_luminance.into();
        self
    }

    pub fn min_luminance<'m>(mut self, min_luminance: f32) -> HdrMetadataExtBuilder<'b> {
        self.raw.minLuminance = min_luminance.into();
        self
    }

    pub fn max_content_light_level<'m>(mut self, max_content_light_level: f32) -> HdrMetadataExtBuilder<'b> {
        self.raw.maxContentLightLevel = max_content_light_level.into();
        self
    }

    pub fn max_frame_average_light_level<'m>(mut self, max_frame_average_light_level: f32) -> HdrMetadataExtBuilder<'b> {
        self.raw.maxFrameAverageLightLevel = max_frame_average_light_level.into();
        self
    }

    pub fn build(self) -> HdrMetadataExt<'b> {
        HdrMetadataExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<RefreshCycleDurationGoogle> for vks::VkRefreshCycleDurationGOOGLE {
    fn from(f: RefreshCycleDurationGoogle) -> vks::VkRefreshCycleDurationGOOGLE {
        f.raw
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

    pub fn refresh_duration<'m>(mut self, refresh_duration: u64) -> RefreshCycleDurationGoogleBuilder {
        self.raw.refreshDuration = refresh_duration.into();
        self
    }

    pub fn build(self) -> RefreshCycleDurationGoogle {
        RefreshCycleDurationGoogle {
            raw: self.raw,
        }
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


impl From<PastPresentationTimingGoogle> for vks::VkPastPresentationTimingGOOGLE {
    fn from(f: PastPresentationTimingGoogle) -> vks::VkPastPresentationTimingGOOGLE {
        f.raw
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

    pub fn present_id<'m>(mut self, present_id: u32) -> PastPresentationTimingGoogleBuilder {
        self.raw.presentID = present_id.into();
        self
    }

    pub fn desired_present_time<'m>(mut self, desired_present_time: u64) -> PastPresentationTimingGoogleBuilder {
        self.raw.desiredPresentTime = desired_present_time.into();
        self
    }

    pub fn actual_present_time<'m>(mut self, actual_present_time: u64) -> PastPresentationTimingGoogleBuilder {
        self.raw.actualPresentTime = actual_present_time.into();
        self
    }

    pub fn earliest_present_time<'m>(mut self, earliest_present_time: u64) -> PastPresentationTimingGoogleBuilder {
        self.raw.earliestPresentTime = earliest_present_time.into();
        self
    }

    pub fn present_margin<'m>(mut self, present_margin: u64) -> PastPresentationTimingGoogleBuilder {
        self.raw.presentMargin = present_margin.into();
        self
    }

    pub fn build(self) -> PastPresentationTimingGoogle {
        PastPresentationTimingGoogle {
            raw: self.raw,
        }
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


impl<'s> From<PresentTimesInfoGoogle<'s>> for vks::VkPresentTimesInfoGOOGLE {
    fn from(f: PresentTimesInfoGoogle<'s>) -> vks::VkPresentTimesInfoGOOGLE {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PresentTimesInfoGoogleBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn swapchain_count<'m>(mut self, swapchain_count: u32) -> PresentTimesInfoGoogleBuilder<'b> {
        self.raw.swapchainCount = swapchain_count.into();
        self
    }

    pub fn times<'m, 'a>(mut self, times: &'a [PresentTimeGoogle]) -> PresentTimesInfoGoogleBuilder<'b> {
        self.raw.pTimes = times.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PresentTimesInfoGoogle<'b> {
        PresentTimesInfoGoogle {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<PresentTimeGoogle> for vks::VkPresentTimeGOOGLE {
    fn from(f: PresentTimeGoogle) -> vks::VkPresentTimeGOOGLE {
        f.raw
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

    pub fn present_id<'m>(mut self, present_id: u32) -> PresentTimeGoogleBuilder {
        self.raw.presentID = present_id.into();
        self
    }

    pub fn desired_present_time<'m>(mut self, desired_present_time: u64) -> PresentTimeGoogleBuilder {
        self.raw.desiredPresentTime = desired_present_time.into();
        self
    }

    pub fn build(self) -> PresentTimeGoogle {
        PresentTimeGoogle {
            raw: self.raw,
        }
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

    pub unsafe fn view(&self) {
    }

    pub fn raw(&self) -> &vks::VkIOSSurfaceCreateInfoMVK {
        &self.raw
    }
}


impl<'s> From<IosSurfaceCreateInfoMvk<'s>> for vks::VkIOSSurfaceCreateInfoMVK {
    fn from(f: IosSurfaceCreateInfoMvk<'s>) -> vks::VkIOSSurfaceCreateInfoMVK {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> IosSurfaceCreateInfoMvkBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: IosSurfaceCreateFlagsMvk) -> IosSurfaceCreateInfoMvkBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn view<'m>(mut self, view: *const c_void) -> IosSurfaceCreateInfoMvkBuilder<'b> {
        self.raw.pView = view;
        self
    }

    pub fn build(self) -> IosSurfaceCreateInfoMvk<'b> {
        IosSurfaceCreateInfoMvk {
            raw: self.raw,
            _p: PhantomData,
        }
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

    pub unsafe fn view(&self) {
    }

    pub fn raw(&self) -> &vks::VkMacOSSurfaceCreateInfoMVK {
        &self.raw
    }
}


impl<'s> From<MacOsSurfaceCreateInfoMvk<'s>> for vks::VkMacOSSurfaceCreateInfoMVK {
    fn from(f: MacOsSurfaceCreateInfoMvk<'s>) -> vks::VkMacOSSurfaceCreateInfoMVK {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MacOsSurfaceCreateInfoMvkBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: MacOsSurfaceCreateFlagsMvk) -> MacOsSurfaceCreateInfoMvkBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub unsafe fn view<'m>(mut self, view: *const c_void) -> MacOsSurfaceCreateInfoMvkBuilder<'b> {
        self.raw.pView = view;
        self
    }

    pub fn build(self) -> MacOsSurfaceCreateInfoMvk<'b> {
        MacOsSurfaceCreateInfoMvk {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<ViewportWScalingNv> for vks::VkViewportWScalingNV {
    fn from(f: ViewportWScalingNv) -> vks::VkViewportWScalingNV {
        f.raw
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

    pub fn xcoeff<'m>(mut self, xcoeff: f32) -> ViewportWScalingNvBuilder {
        self.raw.xcoeff = xcoeff.into();
        self
    }

    pub fn ycoeff<'m>(mut self, ycoeff: f32) -> ViewportWScalingNvBuilder {
        self.raw.ycoeff = ycoeff.into();
        self
    }

    pub fn build(self) -> ViewportWScalingNv {
        ViewportWScalingNv {
            raw: self.raw,
        }
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


impl<'s> From<PipelineViewportWScalingStateCreateInfoNv<'s>> for vks::VkPipelineViewportWScalingStateCreateInfoNV {
    fn from(f: PipelineViewportWScalingStateCreateInfoNv<'s>) -> vks::VkPipelineViewportWScalingStateCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn viewport_wscaling_enable<'m>(mut self, viewport_wscaling_enable: bool) -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self.raw.viewportWScalingEnable = viewport_wscaling_enable as u32;
        self
    }

    pub fn viewport_count<'m>(mut self, viewport_count: u32) -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self.raw.viewportCount = viewport_count.into();
        self
    }

    pub fn viewport_wscalings<'m, 'a>(mut self, viewport_wscalings: &'a [ViewportWScalingNv]) -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        self.raw.pViewportWScalings = viewport_wscalings.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineViewportWScalingStateCreateInfoNv<'b> {
        PipelineViewportWScalingStateCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl From<ViewportSwizzleNv> for vks::VkViewportSwizzleNV {
    fn from(f: ViewportSwizzleNv) -> vks::VkViewportSwizzleNV {
        f.raw
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

    pub fn x<'m>(mut self, x: ViewportCoordinateSwizzleNv) -> ViewportSwizzleNvBuilder {
        self.raw.x = x.into();
        self
    }

    pub fn y<'m>(mut self, y: ViewportCoordinateSwizzleNv) -> ViewportSwizzleNvBuilder {
        self.raw.y = y.into();
        self
    }

    pub fn z<'m>(mut self, z: ViewportCoordinateSwizzleNv) -> ViewportSwizzleNvBuilder {
        self.raw.z = z.into();
        self
    }

    pub fn w<'m>(mut self, w: ViewportCoordinateSwizzleNv) -> ViewportSwizzleNvBuilder {
        self.raw.w = w.into();
        self
    }

    pub fn build(self) -> ViewportSwizzleNv {
        ViewportSwizzleNv {
            raw: self.raw,
        }
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


impl<'s> From<PipelineViewportSwizzleStateCreateInfoNv<'s>> for vks::VkPipelineViewportSwizzleStateCreateInfoNV {
    fn from(f: PipelineViewportSwizzleStateCreateInfoNv<'s>) -> vks::VkPipelineViewportSwizzleStateCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineViewportSwizzleStateCreateFlagsNv) -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn viewport_count<'m>(mut self, viewport_count: u32) -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self.raw.viewportCount = viewport_count.into();
        self
    }

    pub fn viewport_swizzles<'m, 'a>(mut self, viewport_swizzles: &'a [ViewportSwizzleNv]) -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        self.raw.pViewportSwizzles = viewport_swizzles.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineViewportSwizzleStateCreateInfoNv<'b> {
        PipelineViewportSwizzleStateCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceDiscardRectanglePropertiesExt<'s>> for vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    fn from(f: PhysicalDeviceDiscardRectanglePropertiesExt<'s>) -> vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn max_discard_rectangles<'m>(mut self, max_discard_rectangles: u32) -> PhysicalDeviceDiscardRectanglePropertiesExtBuilder<'b> {
        self.raw.maxDiscardRectangles = max_discard_rectangles.into();
        self
    }

    pub fn build(self) -> PhysicalDeviceDiscardRectanglePropertiesExt<'b> {
        PhysicalDeviceDiscardRectanglePropertiesExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineDiscardRectangleStateCreateInfoExt<'s>> for vks::VkPipelineDiscardRectangleStateCreateInfoEXT {
    fn from(f: PipelineDiscardRectangleStateCreateInfoExt<'s>) -> vks::VkPipelineDiscardRectangleStateCreateInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineDiscardRectangleStateCreateFlagsExt) -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn discard_rectangle_mode<'m>(mut self, discard_rectangle_mode: DiscardRectangleModeExt) -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self.raw.discardRectangleMode = discard_rectangle_mode.into();
        self
    }

    pub fn discard_rectangle_count<'m>(mut self, discard_rectangle_count: u32) -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self.raw.discardRectangleCount = discard_rectangle_count.into();
        self
    }

    pub fn discard_rectangles<'m, 'a>(mut self, discard_rectangles: &'a [Rect2d]) -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        self.raw.pDiscardRectangles = discard_rectangles.as_ptr() as *const _;
        self
    }

    pub fn build(self) -> PipelineDiscardRectangleStateCreateInfoExt<'b> {
        PipelineDiscardRectangleStateCreateInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx<'s>> for vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn from(f: PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx<'s>) -> vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        f.raw
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


impl<'s> From<PhysicalDeviceSurfaceInfo2Khr<'s>> for vks::VkPhysicalDeviceSurfaceInfo2KHR {
    fn from(f: PhysicalDeviceSurfaceInfo2Khr<'s>) -> vks::VkPhysicalDeviceSurfaceInfo2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn surface<'m, 'a>(mut self, surface: &'a Surface) -> PhysicalDeviceSurfaceInfo2KhrBuilder<'b> {
        self.raw.surface = surface.handle();
        self
    }

    pub fn build(self) -> PhysicalDeviceSurfaceInfo2Khr<'b> {
        PhysicalDeviceSurfaceInfo2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<SurfaceCapabilities2Khr<'s>> for vks::VkSurfaceCapabilities2KHR {
    fn from(f: SurfaceCapabilities2Khr<'s>) -> vks::VkSurfaceCapabilities2KHR {
        f.raw
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


impl<'s> From<SurfaceFormat2Khr<'s>> for vks::VkSurfaceFormat2KHR {
    fn from(f: SurfaceFormat2Khr<'s>) -> vks::VkSurfaceFormat2KHR {
        f.raw
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


impl<'s> From<SharedPresentSurfaceCapabilitiesKhr<'s>> for vks::VkSharedPresentSurfaceCapabilitiesKHR {
    fn from(f: SharedPresentSurfaceCapabilitiesKhr<'s>) -> vks::VkSharedPresentSurfaceCapabilitiesKHR {
        f.raw
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


impl<'s> From<PhysicalDevice16BitStorageFeaturesKhr<'s>> for vks::VkPhysicalDevice16BitStorageFeaturesKHR {
    fn from(f: PhysicalDevice16BitStorageFeaturesKhr<'s>) -> vks::VkPhysicalDevice16BitStorageFeaturesKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn storage_buffer_16_bit_access<'m>(mut self, storage_buffer_16_bit_access: bool) -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self.raw.storageBuffer16BitAccess = storage_buffer_16_bit_access as u32;
        self
    }

    pub fn uniform_and_storage_buffer_16_bit_access<'m>(mut self, uniform_and_storage_buffer_16_bit_access: bool) -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self.raw.uniformAndStorageBuffer16BitAccess = uniform_and_storage_buffer_16_bit_access as u32;
        self
    }

    pub fn storage_push_constant_16<'m>(mut self, storage_push_constant_16: bool) -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self.raw.storagePushConstant16 = storage_push_constant_16 as u32;
        self
    }

    pub fn storage_input_output_16<'m>(mut self, storage_input_output_16: bool) -> PhysicalDevice16BitStorageFeaturesKhrBuilder<'b> {
        self.raw.storageInputOutput16 = storage_input_output_16 as u32;
        self
    }

    pub fn build(self) -> PhysicalDevice16BitStorageFeaturesKhr<'b> {
        PhysicalDevice16BitStorageFeaturesKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<BufferMemoryRequirementsInfo2Khr<'s>> for vks::VkBufferMemoryRequirementsInfo2KHR {
    fn from(f: BufferMemoryRequirementsInfo2Khr<'s>) -> vks::VkBufferMemoryRequirementsInfo2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BufferMemoryRequirementsInfo2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> BufferMemoryRequirementsInfo2KhrBuilder<'b> {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn build(self) -> BufferMemoryRequirementsInfo2Khr<'b> {
        BufferMemoryRequirementsInfo2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImageMemoryRequirementsInfo2Khr<'s>> for vks::VkImageMemoryRequirementsInfo2KHR {
    fn from(f: ImageMemoryRequirementsInfo2Khr<'s>) -> vks::VkImageMemoryRequirementsInfo2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImageMemoryRequirementsInfo2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> ImageMemoryRequirementsInfo2KhrBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn build(self) -> ImageMemoryRequirementsInfo2Khr<'b> {
        ImageMemoryRequirementsInfo2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<ImageSparseMemoryRequirementsInfo2Khr<'s>> for vks::VkImageSparseMemoryRequirementsInfo2KHR {
    fn from(f: ImageSparseMemoryRequirementsInfo2Khr<'s>) -> vks::VkImageSparseMemoryRequirementsInfo2KHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> ImageSparseMemoryRequirementsInfo2KhrBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn build(self) -> ImageSparseMemoryRequirementsInfo2Khr<'b> {
        ImageSparseMemoryRequirementsInfo2Khr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<MemoryRequirements2Khr<'s>> for vks::VkMemoryRequirements2KHR {
    fn from(f: MemoryRequirements2Khr<'s>) -> vks::VkMemoryRequirements2KHR {
        f.raw
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


impl<'s> From<SparseImageMemoryRequirements2Khr<'s>> for vks::VkSparseImageMemoryRequirements2KHR {
    fn from(f: SparseImageMemoryRequirements2Khr<'s>) -> vks::VkSparseImageMemoryRequirements2KHR {
        f.raw
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


impl<'s> From<MemoryDedicatedRequirementsKhr<'s>> for vks::VkMemoryDedicatedRequirementsKHR {
    fn from(f: MemoryDedicatedRequirementsKhr<'s>) -> vks::VkMemoryDedicatedRequirementsKHR {
        f.raw
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


impl<'s> From<MemoryDedicatedAllocateInfoKhr<'s>> for vks::VkMemoryDedicatedAllocateInfoKHR {
    fn from(f: MemoryDedicatedAllocateInfoKhr<'s>) -> vks::VkMemoryDedicatedAllocateInfoKHR {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn image<'m, 'a>(mut self, image: &'a Image) -> MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        self.raw.image = image.handle();
        self
    }

    pub fn buffer<'m, 'a>(mut self, buffer: &'a Buffer) -> MemoryDedicatedAllocateInfoKhrBuilder<'b> {
        self.raw.buffer = buffer.handle();
        self
    }

    pub fn build(self) -> MemoryDedicatedAllocateInfoKhr<'b> {
        MemoryDedicatedAllocateInfoKhr {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<TextureLODGatherFormatPropertiesAmd<'s>> for vks::VkTextureLODGatherFormatPropertiesAMD {
    fn from(f: TextureLODGatherFormatPropertiesAmd<'s>) -> vks::VkTextureLODGatherFormatPropertiesAMD {
        f.raw
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


impl<'s> From<PipelineCoverageToColorStateCreateInfoNv<'s>> for vks::VkPipelineCoverageToColorStateCreateInfoNV {
    fn from(f: PipelineCoverageToColorStateCreateInfoNv<'s>) -> vks::VkPipelineCoverageToColorStateCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineCoverageToColorStateCreateFlagsNv) -> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn coverage_to_color_enable<'m>(mut self, coverage_to_color_enable: bool) -> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self.raw.coverageToColorEnable = coverage_to_color_enable as u32;
        self
    }

    pub fn coverage_to_color_location<'m>(mut self, coverage_to_color_location: u32) -> PipelineCoverageToColorStateCreateInfoNvBuilder<'b> {
        self.raw.coverageToColorLocation = coverage_to_color_location.into();
        self
    }

    pub fn build(self) -> PipelineCoverageToColorStateCreateInfoNv<'b> {
        PipelineCoverageToColorStateCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceSamplerFilterMinmaxPropertiesExt<'s>> for vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    fn from(f: PhysicalDeviceSamplerFilterMinmaxPropertiesExt<'s>) -> vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
        f.raw
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


impl<'s> From<SamplerReductionModeCreateInfoExt<'s>> for vks::VkSamplerReductionModeCreateInfoEXT {
    fn from(f: SamplerReductionModeCreateInfoExt<'s>) -> vks::VkSamplerReductionModeCreateInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SamplerReductionModeCreateInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn reduction_mode<'m>(mut self, reduction_mode: SamplerReductionModeExt) -> SamplerReductionModeCreateInfoExtBuilder<'b> {
        self.raw.reductionMode = reduction_mode.into();
        self
    }

    pub fn build(self) -> SamplerReductionModeCreateInfoExt<'b> {
        SamplerReductionModeCreateInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceBlendOperationAdvancedFeaturesExt<'s>> for vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn from(f: PhysicalDeviceBlendOperationAdvancedFeaturesExt<'s>) -> vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *mut c_void) -> PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn advanced_blend_coherent_operations<'m>(mut self, advanced_blend_coherent_operations: bool) -> PhysicalDeviceBlendOperationAdvancedFeaturesExtBuilder<'b> {
        self.raw.advancedBlendCoherentOperations = advanced_blend_coherent_operations as u32;
        self
    }

    pub fn build(self) -> PhysicalDeviceBlendOperationAdvancedFeaturesExt<'b> {
        PhysicalDeviceBlendOperationAdvancedFeaturesExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PhysicalDeviceBlendOperationAdvancedPropertiesExt<'s>> for vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn from(f: PhysicalDeviceBlendOperationAdvancedPropertiesExt<'s>) -> vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        f.raw
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


impl<'s> From<PipelineColorBlendAdvancedStateCreateInfoExt<'s>> for vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    fn from(f: PipelineColorBlendAdvancedStateCreateInfoExt<'s>) -> vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn src_premultiplied<'m>(mut self, src_premultiplied: bool) -> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self.raw.srcPremultiplied = src_premultiplied as u32;
        self
    }

    pub fn dst_premultiplied<'m>(mut self, dst_premultiplied: bool) -> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self.raw.dstPremultiplied = dst_premultiplied as u32;
        self
    }

    pub fn blend_overlap<'m>(mut self, blend_overlap: BlendOverlapExt) -> PipelineColorBlendAdvancedStateCreateInfoExtBuilder<'b> {
        self.raw.blendOverlap = blend_overlap.into();
        self
    }

    pub fn build(self) -> PipelineColorBlendAdvancedStateCreateInfoExt<'b> {
        PipelineColorBlendAdvancedStateCreateInfoExt {
            raw: self.raw,
            _p: PhantomData,
        }
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


impl<'s> From<PipelineCoverageModulationStateCreateInfoNv<'s>> for vks::VkPipelineCoverageModulationStateCreateInfoNV {
    fn from(f: PipelineCoverageModulationStateCreateInfoNv<'s>) -> vks::VkPipelineCoverageModulationStateCreateInfoNV {
        f.raw
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

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn flags<'m>(mut self, flags: PipelineCoverageModulationStateCreateFlagsNv) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self.raw.flags = flags.bits();
        self
    }

    pub fn coverage_modulation_mode<'m>(mut self, coverage_modulation_mode: CoverageModulationModeNv) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self.raw.coverageModulationMode = coverage_modulation_mode.into();
        self
    }

    pub fn coverage_modulation_table_enable<'m>(mut self, coverage_modulation_table_enable: bool) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self.raw.coverageModulationTableEnable = coverage_modulation_table_enable as u32;
        self
    }

    pub fn coverage_modulation_table_count<'m>(mut self, coverage_modulation_table_count: u32) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self.raw.coverageModulationTableCount = coverage_modulation_table_count.into();
        self
    }

    pub fn coverage_modulation_table<'m, 'a>(mut self, coverage_modulation_table: &'a f32) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        self.raw.pCoverageModulationTable = coverage_modulation_table;
        self
    }

    pub fn build(self) -> PipelineCoverageModulationStateCreateInfoNv<'b> {
        PipelineCoverageModulationStateCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


