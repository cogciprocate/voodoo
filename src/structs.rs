//! Structs.

#![allow(unused_mut)]

use std::ptr;
use std::ffi::{CString, CStr};
use std::marker::PhantomData;
use std::slice;
use libc::{c_void, c_char};
use num_traits::ToPrimitive;
use smallvec::SmallVec;
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

    pub fn x<'a>(&'a self) -> i32 {
        self.raw.x.into()
    }

    pub fn y<'a>(&'a self) -> i32 {
        self.raw.y.into()
    }

    pub fn set_x<'m>(& mut self, x: i32) {
        self.raw.x = x.into();
    }

    pub fn set_y<'m>(& mut self, y: i32) {
        self.raw.y = y.into();
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


impl From<vks::VkOffset2D> for Offset2d {
    fn from(f: vks::VkOffset2D) -> Offset2d {
        Offset2d { raw: f, }
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

    pub fn get_x<'a>(&'a self) -> i32 {
        self.raw.x.into()
    }

    pub fn get_y<'a>(&'a self) -> i32 {
        self.raw.y.into()
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

    pub fn x<'a>(&'a self) -> i32 {
        self.raw.x.into()
    }

    pub fn y<'a>(&'a self) -> i32 {
        self.raw.y.into()
    }

    pub fn z<'a>(&'a self) -> i32 {
        self.raw.z.into()
    }

    pub fn set_x<'m>(& mut self, x: i32) {
        self.raw.x = x.into();
    }

    pub fn set_y<'m>(& mut self, y: i32) {
        self.raw.y = y.into();
    }

    pub fn set_z<'m>(& mut self, z: i32) {
        self.raw.z = z.into();
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


impl From<vks::VkOffset3D> for Offset3d {
    fn from(f: vks::VkOffset3D) -> Offset3d {
        Offset3d { raw: f, }
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

    pub fn get_x<'a>(&'a self) -> i32 {
        self.raw.x.into()
    }

    pub fn get_y<'a>(&'a self) -> i32 {
        self.raw.y.into()
    }

    pub fn get_z<'a>(&'a self) -> i32 {
        self.raw.z.into()
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

    pub fn width<'a>(&'a self) -> u32 {
        self.raw.width.into()
    }

    pub fn height<'a>(&'a self) -> u32 {
        self.raw.height.into()
    }

    pub fn set_width<'m>(& mut self, width: u32) {
        self.raw.width = width.into();
    }

    pub fn set_height<'m>(& mut self, height: u32) {
        self.raw.height = height.into();
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


impl From<vks::VkExtent2D> for Extent2d {
    fn from(f: vks::VkExtent2D) -> Extent2d {
        Extent2d { raw: f, }
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

    pub fn get_width<'a>(&'a self) -> u32 {
        self.raw.width.into()
    }

    pub fn get_height<'a>(&'a self) -> u32 {
        self.raw.height.into()
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

    pub fn width<'a>(&'a self) -> u32 {
        self.raw.width.into()
    }

    pub fn height<'a>(&'a self) -> u32 {
        self.raw.height.into()
    }

    pub fn depth<'a>(&'a self) -> u32 {
        self.raw.depth.into()
    }

    pub fn set_width<'m>(& mut self, width: u32) {
        self.raw.width = width.into();
    }

    pub fn set_height<'m>(& mut self, height: u32) {
        self.raw.height = height.into();
    }

    pub fn set_depth<'m>(& mut self, depth: u32) {
        self.raw.depth = depth.into();
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


impl From<vks::VkExtent3D> for Extent3d {
    fn from(f: vks::VkExtent3D) -> Extent3d {
        Extent3d { raw: f, }
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

    pub fn get_width<'a>(&'a self) -> u32 {
        self.raw.width.into()
    }

    pub fn get_height<'a>(&'a self) -> u32 {
        self.raw.height.into()
    }

    pub fn get_depth<'a>(&'a self) -> u32 {
        self.raw.depth.into()
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

    pub fn x<'a>(&'a self) -> f32 {
        self.raw.x.into()
    }

    pub fn y<'a>(&'a self) -> f32 {
        self.raw.y.into()
    }

    pub fn width<'a>(&'a self) -> f32 {
        self.raw.width.into()
    }

    pub fn height<'a>(&'a self) -> f32 {
        self.raw.height.into()
    }

    pub fn min_depth<'a>(&'a self) -> f32 {
        self.raw.minDepth.into()
    }

    pub fn max_depth<'a>(&'a self) -> f32 {
        self.raw.maxDepth.into()
    }

    pub fn set_x<'m>(& mut self, x: f32) {
        self.raw.x = x.into();
    }

    pub fn set_y<'m>(& mut self, y: f32) {
        self.raw.y = y.into();
    }

    pub fn set_width<'m>(& mut self, width: f32) {
        self.raw.width = width.into();
    }

    pub fn set_height<'m>(& mut self, height: f32) {
        self.raw.height = height.into();
    }

    pub fn set_min_depth<'m>(& mut self, min_depth: f32) {
        self.raw.minDepth = min_depth.into();
    }

    pub fn set_max_depth<'m>(& mut self, max_depth: f32) {
        self.raw.maxDepth = max_depth.into();
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


impl From<vks::VkViewport> for Viewport {
    fn from(f: vks::VkViewport) -> Viewport {
        Viewport { raw: f, }
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

    pub fn get_x<'a>(&'a self) -> f32 {
        self.raw.x.into()
    }

    pub fn get_y<'a>(&'a self) -> f32 {
        self.raw.y.into()
    }

    pub fn get_width<'a>(&'a self) -> f32 {
        self.raw.width.into()
    }

    pub fn get_height<'a>(&'a self) -> f32 {
        self.raw.height.into()
    }

    pub fn get_min_depth<'a>(&'a self) -> f32 {
        self.raw.minDepth.into()
    }

    pub fn get_max_depth<'a>(&'a self) -> f32 {
        self.raw.maxDepth.into()
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

    pub fn offset<'a>(&'a self) -> Offset2d {
        self.raw.offset.into()
    }

    pub fn extent<'a>(&'a self) -> Extent2d {
        self.raw.extent.into()
    }

    pub fn set_offset<'m>(& mut self, offset: Offset2d) {
        self.raw.offset = offset.raw;
    }

    pub fn set_extent<'m>(& mut self, extent: Extent2d) {
        self.raw.extent = extent.raw;
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


impl From<vks::VkRect2D> for Rect2d {
    fn from(f: vks::VkRect2D) -> Rect2d {
        Rect2d { raw: f, }
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

    pub fn get_offset<'a>(&'a self) -> Offset2d {
        self.raw.offset.into()
    }

    pub fn get_extent<'a>(&'a self) -> Extent2d {
        self.raw.extent.into()
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

    pub fn rect<'a>(&'a self) -> Rect2d {
        self.raw.rect.into()
    }

    pub fn base_array_layer<'a>(&'a self) -> u32 {
        self.raw.baseArrayLayer.into()
    }

    pub fn layer_count<'a>(&'a self) -> u32 {
        self.raw.layerCount.into()
    }

    pub fn set_rect<'m>(& mut self, rect: Rect2d) {
        self.raw.rect = rect.raw;
    }

    pub fn set_base_array_layer<'m>(& mut self, base_array_layer: u32) {
        self.raw.baseArrayLayer = base_array_layer.into();
    }

    pub fn set_layer_count<'m>(& mut self, layer_count: u32) {
        self.raw.layerCount = layer_count.into();
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


impl From<vks::VkClearRect> for ClearRect {
    fn from(f: vks::VkClearRect) -> ClearRect {
        ClearRect { raw: f, }
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

    pub fn get_rect<'a>(&'a self) -> Rect2d {
        self.raw.rect.into()
    }

    pub fn get_base_array_layer<'a>(&'a self) -> u32 {
        self.raw.baseArrayLayer.into()
    }

    pub fn get_layer_count<'a>(&'a self) -> u32 {
        self.raw.layerCount.into()
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

    pub fn r<'a>(&'a self) -> ComponentSwizzle {
        self.raw.r.into()
    }

    pub fn g<'a>(&'a self) -> ComponentSwizzle {
        self.raw.g.into()
    }

    pub fn b<'a>(&'a self) -> ComponentSwizzle {
        self.raw.b.into()
    }

    pub fn a<'a>(&'a self) -> ComponentSwizzle {
        self.raw.a.into()
    }

    pub fn set_r<'m>(& mut self, r: ComponentSwizzle) {
        self.raw.r = r.into();
    }

    pub fn set_g<'m>(& mut self, g: ComponentSwizzle) {
        self.raw.g = g.into();
    }

    pub fn set_b<'m>(& mut self, b: ComponentSwizzle) {
        self.raw.b = b.into();
    }

    pub fn set_a<'m>(& mut self, a: ComponentSwizzle) {
        self.raw.a = a.into();
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


impl From<vks::VkComponentMapping> for ComponentMapping {
    fn from(f: vks::VkComponentMapping) -> ComponentMapping {
        ComponentMapping { raw: f, }
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

    pub fn get_r<'a>(&'a self) -> ComponentSwizzle {
        self.raw.r.into()
    }

    pub fn get_g<'a>(&'a self) -> ComponentSwizzle {
        self.raw.g.into()
    }

    pub fn get_b<'a>(&'a self) -> ComponentSwizzle {
        self.raw.b.into()
    }

    pub fn get_a<'a>(&'a self) -> ComponentSwizzle {
        self.raw.a.into()
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
    pub fn api_version<'a>(&'a self) -> Version {
        self.raw.apiVersion.into()
    }

    pub fn driver_version<'a>(&'a self) -> Version {
        self.raw.driverVersion.into()
    }

    pub fn vendor_id<'a>(&'a self) -> u32 {
        self.raw.vendorID.into()
    }

    pub fn device_id<'a>(&'a self) -> u32 {
        self.raw.deviceID.into()
    }

    pub fn device_type<'a>(&'a self) -> PhysicalDeviceType {
        self.raw.deviceType.into()
    }

    pub fn device_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(&self.raw.deviceName as *const _) }
    }

    pub fn pipeline_cache_uu_id<'a>(&'a self) -> &[u8] {
        unsafe { slice::from_raw_parts(&self.raw.pipelineCacheUUID as *const _, vks::VK_UUID_SIZE as usize) }
    }

    pub fn limits<'a>(&'a self) -> PhysicalDeviceLimits {
        self.raw.limits.into()
    }

    pub fn sparse_properties<'a>(&'a self) -> PhysicalDeviceSparseProperties {
        self.raw.sparseProperties.into()
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


impl From<vks::VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
    fn from(f: vks::VkPhysicalDeviceProperties) -> PhysicalDeviceProperties {
        PhysicalDeviceProperties { raw: f, }
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
    pub fn extension_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(&self.raw.extensionName as *const _) }
    }

    pub fn spec_version<'a>(&'a self) -> Version {
        self.raw.specVersion.into()
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


impl From<vks::VkExtensionProperties> for ExtensionProperties {
    fn from(f: vks::VkExtensionProperties) -> ExtensionProperties {
        ExtensionProperties { raw: f, }
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
    pub fn layer_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(&self.raw.layerName as *const _) }
    }

    pub fn spec_version<'a>(&'a self) -> Version {
        self.raw.specVersion.into()
    }

    pub fn implementation_version<'a>(&'a self) -> Version {
        self.raw.implementationVersion.into()
    }

    pub fn description<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(&self.raw.description as *const _) }
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


impl From<vks::VkLayerProperties> for LayerProperties {
    fn from(f: vks::VkLayerProperties) -> LayerProperties {
        LayerProperties { raw: f, }
    }
}


/// A `VkApplicationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ApplicationInfo<'s> {
    raw: vks::VkApplicationInfo,
    engine_name: Option<CharStr<'s>>,
    application_name: Option<CharStr<'s>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> ApplicationInfo<'s> {
    pub fn builder<'b>() -> ApplicationInfoBuilder<'b> {
        ApplicationInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn application_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    pub fn application_version<'a>(&'a self) -> Version {
        self.raw.applicationVersion.into()
    }

    pub fn engine_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pEngineName) }
    }

    pub fn engine_version<'a>(&'a self) -> Version {
        self.raw.engineVersion.into()
    }

    pub fn api_version<'a>(&'a self) -> Version {
        self.raw.apiVersion.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_application_name<'m, 'a, T>(&'s mut self, application_name: T)
            where 'a: 's, T: Into<CharStr<'a>> {
        self.application_name = Some(application_name.into());
        {
            let application_name = self.application_name.as_ref().unwrap();
            self.raw.pApplicationName = application_name.as_ptr();
        }
    }

    pub fn set_application_version<'m, T>(&'s mut self, application_version: T)
            where T: Into<Version> {
        self.raw.applicationVersion = application_version.into().into();
    }

    pub fn set_engine_name<'m, 'a, T>(&'s mut self, engine_name: T)
            where 'a: 's, T: Into<CharStr<'a>> {
        self.engine_name = Some(engine_name.into());
        {
            let engine_name = self.engine_name.as_ref().unwrap();
            self.raw.pEngineName = engine_name.as_ptr();
        }
    }

    pub fn set_engine_version<'m, T>(&'s mut self, engine_version: T)
            where T: Into<Version> {
        self.raw.engineVersion = engine_version.into().into();
    }

    pub fn set_api_version<'m, T>(&'s mut self, api_version: T)
            where T: Into<Version> {
        self.raw.apiVersion = api_version.into().into();
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


impl<'s> From<vks::VkApplicationInfo> for ApplicationInfo<'s> {
    fn from(f: vks::VkApplicationInfo) -> ApplicationInfo<'s> {
        ApplicationInfo { raw: f, engine_name: None, application_name: None, _p: PhantomData }
    }
}


/// A builder for `VkApplicationInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct ApplicationInfoBuilder<'b> {
    raw: vks::VkApplicationInfo,
    engine_name: Option<CharStr<'b>>,
    application_name: Option<CharStr<'b>>,
    _p: PhantomData<&'b ()>, 
}

impl<'b> ApplicationInfoBuilder<'b> {
    pub fn new() -> ApplicationInfoBuilder<'b> {
        ApplicationInfoBuilder {
            raw: vks::VkApplicationInfo::default(),
            engine_name: None,
            application_name: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> ApplicationInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn application_name<'m, 'a, T>(mut self, application_name: T) -> ApplicationInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStr<'a>> {
        self.application_name = Some(application_name.into());
        {
            let application_name = self.application_name.as_ref().unwrap();
            self.raw.pApplicationName = application_name.as_ptr();
        }
        self
    }

    pub fn application_version<'m, T>(mut self, application_version: T) -> ApplicationInfoBuilder<'b>
            where T: Into<Version> {
        self.raw.applicationVersion = application_version.into().into();
        self
    }

    pub fn engine_name<'m, 'a, T>(mut self, engine_name: T) -> ApplicationInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStr<'a>> {
        self.engine_name = Some(engine_name.into());
        {
            let engine_name = self.engine_name.as_ref().unwrap();
            self.raw.pEngineName = engine_name.as_ptr();
        }
        self
    }

    pub fn engine_version<'m, T>(mut self, engine_version: T) -> ApplicationInfoBuilder<'b>
            where T: Into<Version> {
        self.raw.engineVersion = engine_version.into().into();
        self
    }

    pub fn api_version<'m, T>(mut self, api_version: T) -> ApplicationInfoBuilder<'b>
            where T: Into<Version> {
        self.raw.apiVersion = api_version.into().into();
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_application_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    pub fn get_application_version<'a>(&'a self) -> Version {
        self.raw.applicationVersion.into()
    }

    pub fn get_engine_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pEngineName) }
    }

    pub fn get_engine_version<'a>(&'a self) -> Version {
        self.raw.engineVersion.into()
    }

    pub fn get_api_version<'a>(&'a self) -> Version {
        self.raw.apiVersion.into()
    }

    pub fn build(self) -> ApplicationInfo<'b> {
        ApplicationInfo {
            raw: self.raw,
            engine_name: self.engine_name,
            application_name: self.application_name,
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

    pub fn user_data<'a>(&'a self) -> *mut c_void {
        self.raw.pUserData
    }

    pub fn pfn_allocation<'a>(&'a self) -> PFN_vkAllocationFunction {
        self.raw.pfnAllocation.into()
    }

    pub fn pfn_reallocation<'a>(&'a self) -> PFN_vkReallocationFunction {
        self.raw.pfnReallocation.into()
    }

    pub fn pfn_free<'a>(&'a self) -> PFN_vkFreeFunction {
        self.raw.pfnFree.into()
    }

    pub fn pfn_internal_allocation<'a>(&'a self) -> PFN_vkInternalAllocationNotification {
        self.raw.pfnInternalAllocation.into()
    }

    pub fn pfn_internal_free<'a>(&'a self) -> PFN_vkInternalFreeNotification {
        self.raw.pfnInternalFree.into()
    }

    pub unsafe fn set_user_data<'m>(&'s mut self, user_data: *mut c_void) {
        self.raw.pUserData = user_data;
    }

    pub fn set_pfn_allocation<'m>(&'s mut self, pfn_allocation: PFN_vkAllocationFunction) {
        self.raw.pfnAllocation = pfn_allocation.into();
    }

    pub fn set_pfn_reallocation<'m>(&'s mut self, pfn_reallocation: PFN_vkReallocationFunction) {
        self.raw.pfnReallocation = pfn_reallocation.into();
    }

    pub fn set_pfn_free<'m>(&'s mut self, pfn_free: PFN_vkFreeFunction) {
        self.raw.pfnFree = pfn_free.into();
    }

    pub fn set_pfn_internal_allocation<'m>(&'s mut self, pfn_internal_allocation: PFN_vkInternalAllocationNotification) {
        self.raw.pfnInternalAllocation = pfn_internal_allocation.into();
    }

    pub fn set_pfn_internal_free<'m>(&'s mut self, pfn_internal_free: PFN_vkInternalFreeNotification) {
        self.raw.pfnInternalFree = pfn_internal_free.into();
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


impl<'s> From<vks::VkAllocationCallbacks> for AllocationCallbacks<'s> {
    fn from(f: vks::VkAllocationCallbacks) -> AllocationCallbacks<'s> {
        AllocationCallbacks { raw: f, _p: PhantomData }
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

    pub fn get_user_data<'a>(&'a self) -> *mut c_void {
        self.raw.pUserData
    }

    pub fn get_pfn_allocation<'a>(&'a self) -> PFN_vkAllocationFunction {
        self.raw.pfnAllocation.into()
    }

    pub fn get_pfn_reallocation<'a>(&'a self) -> PFN_vkReallocationFunction {
        self.raw.pfnReallocation.into()
    }

    pub fn get_pfn_free<'a>(&'a self) -> PFN_vkFreeFunction {
        self.raw.pfnFree.into()
    }

    pub fn get_pfn_internal_allocation<'a>(&'a self) -> PFN_vkInternalAllocationNotification {
        self.raw.pfnInternalAllocation.into()
    }

    pub fn get_pfn_internal_free<'a>(&'a self) -> PFN_vkInternalFreeNotification {
        self.raw.pfnInternalFree.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DeviceQueueCreateFlags {
        DeviceQueueCreateFlags::from_bits(self.raw.flags)
            .expect("DeviceQueueCreateInfo::flags: error converting flags")
    }

    pub fn queue_family_index<'a>(&'a self) -> u32 {
        self.raw.queueFamilyIndex.into()
    }

    pub fn queue_priorities<'a>(&'a self) -> &'a [f32] {
        unsafe { slice::from_raw_parts(self.raw.pQueuePriorities as *const _, self.raw.queueCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DeviceQueueCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_queue_family_index<'m>(&'s mut self, queue_family_index: u32) {
        self.raw.queueFamilyIndex = queue_family_index.into();
    }

    pub fn set_queue_priorities<'m, 'a>(&'s mut self, queue_priorities: &'a [f32]) {
        assert!(self.raw.queueCount == 0 || self.raw.queueCount == queue_priorities.len() as _, 
            "count inconsistency found when specifying `DeviceQueueCreateInfo::queue_priorities`.");
        self.raw.queueCount = queue_priorities.len() as _;
        self.raw.pQueuePriorities = queue_priorities.as_ptr() as *const f32 as *const _;
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


impl<'s> From<vks::VkDeviceQueueCreateInfo> for DeviceQueueCreateInfo<'s> {
    fn from(f: vks::VkDeviceQueueCreateInfo) -> DeviceQueueCreateInfo<'s> {
        DeviceQueueCreateInfo { raw: f, _p: PhantomData }
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

    pub fn queue_priorities<'m, 'a>(mut self, queue_priorities: &'a [f32]) -> DeviceQueueCreateInfoBuilder<'b> {
        assert!(self.raw.queueCount == 0 || self.raw.queueCount == queue_priorities.len() as _, 
            "count inconsistency found when specifying `DeviceQueueCreateInfo::queue_priorities`.");
        self.raw.queueCount = queue_priorities.len() as _;
        self.raw.pQueuePriorities = queue_priorities.as_ptr() as *const f32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DeviceQueueCreateFlags {
        DeviceQueueCreateFlags::from_bits(self.raw.flags)
            .expect("DeviceQueueCreateInfo::flags: error converting flags")
    }

    pub fn get_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.queueFamilyIndex.into()
    }

    pub fn get_queue_priorities<'a>(&'a self) -> &'a [f32] {
        unsafe { slice::from_raw_parts(self.raw.pQueuePriorities as *const _, self.raw.queueCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DeviceCreateFlags {
        DeviceCreateFlags::from_bits(self.raw.flags)
            .expect("DeviceCreateInfo::flags: error converting flags")
    }

    pub fn queue_create_infos<'a>(&'a self) -> &'a [DeviceQueueCreateInfo] {
        unsafe { slice::from_raw_parts(self.raw.pQueueCreateInfos as *const _, self.raw.queueCreateInfoCount as usize) }
    }

    pub fn enabled_layer_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledLayerNames as *const _, self.raw.enabledLayerCount as usize) }
    }

    pub fn enabled_extension_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledExtensionNames as *const _, self.raw.enabledExtensionCount as usize) }
    }

    pub fn enabled_features<'a>(&'a self) -> &'a PhysicalDeviceFeatures {
        unsafe { &*(self.raw.pEnabledFeatures as *const vks::VkPhysicalDeviceFeatures as *const _) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DeviceCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_queue_create_infos<'m, 'a>(&'s mut self, queue_create_infos: &'a [DeviceQueueCreateInfo]) {
        assert!(self.raw.queueCreateInfoCount == 0 || self.raw.queueCreateInfoCount == queue_create_infos.len() as _, 
            "count inconsistency found when specifying `DeviceCreateInfo::queue_create_infos`.");
        self.raw.queueCreateInfoCount = queue_create_infos.len() as _;
        self.raw.pQueueCreateInfos = queue_create_infos.as_ptr() as *const vks::VkDeviceQueueCreateInfo as *const _;
    }

    pub fn set_enabled_layer_names<'m, 'a, T>(&'s mut self, enabled_layer_names: T)
            where 'a: 's, T: Into<CharStrs<'a>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        {
            let enabled_layer_names = self.enabled_layer_names.as_ref().unwrap();
            self.raw.ppEnabledLayerNames = enabled_layer_names.as_ptr();
            assert!(self.raw.enabledLayerCount == 0 || self.raw.enabledLayerCount == enabled_layer_names.len() as _, 
                "count inconsistency found when specifying `DeviceCreateInfo::enabled_layer_names`.");
            self.raw.enabledLayerCount = enabled_layer_names.len() as _;
        }
    }

    pub fn set_enabled_extension_names<'m, 'a, T>(&'s mut self, enabled_extension_names: T)
            where 'a: 's, T: Into<CharStrs<'a>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        {
            let enabled_extension_names = self.enabled_extension_names.as_ref().unwrap();
            self.raw.ppEnabledExtensionNames = enabled_extension_names.as_ptr();
            assert!(self.raw.enabledExtensionCount == 0 || self.raw.enabledExtensionCount == enabled_extension_names.len() as _, 
                "count inconsistency found when specifying `DeviceCreateInfo::enabled_extension_names`.");
            self.raw.enabledExtensionCount = enabled_extension_names.len() as _;
        }
    }

    pub fn set_enabled_features<'m, 'a>(&'s mut self, enabled_features: &'a PhysicalDeviceFeatures) {
        self.raw.pEnabledFeatures = enabled_features.raw();
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


impl<'s> From<vks::VkDeviceCreateInfo> for DeviceCreateInfo<'s> {
    fn from(f: vks::VkDeviceCreateInfo) -> DeviceCreateInfo<'s> {
        DeviceCreateInfo { raw: f, enabled_extension_names: None, enabled_layer_names: None, _p: PhantomData }
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

    pub fn queue_create_infos<'m, 'a>(mut self, queue_create_infos: &'a [DeviceQueueCreateInfo]) -> DeviceCreateInfoBuilder<'b> {
        assert!(self.raw.queueCreateInfoCount == 0 || self.raw.queueCreateInfoCount == queue_create_infos.len() as _, 
            "count inconsistency found when specifying `DeviceCreateInfo::queue_create_infos`.");
        self.raw.queueCreateInfoCount = queue_create_infos.len() as _;
        self.raw.pQueueCreateInfos = queue_create_infos.as_ptr() as *const vks::VkDeviceQueueCreateInfo as *const _;
        self
    }

    pub fn enabled_layer_names<'m, 'a, T>(mut self, enabled_layer_names: T) -> DeviceCreateInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        {
            let enabled_layer_names = self.enabled_layer_names.as_ref().unwrap();
            self.raw.ppEnabledLayerNames = enabled_layer_names.as_ptr();
            assert!(self.raw.enabledLayerCount == 0 || self.raw.enabledLayerCount == enabled_layer_names.len() as _, 
                "count inconsistency found when specifying `DeviceCreateInfo::enabled_layer_names`.");
            self.raw.enabledLayerCount = enabled_layer_names.len() as _;
        }
        self
    }

    pub fn enabled_extension_names<'m, 'a, T>(mut self, enabled_extension_names: T) -> DeviceCreateInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        {
            let enabled_extension_names = self.enabled_extension_names.as_ref().unwrap();
            self.raw.ppEnabledExtensionNames = enabled_extension_names.as_ptr();
            assert!(self.raw.enabledExtensionCount == 0 || self.raw.enabledExtensionCount == enabled_extension_names.len() as _, 
                "count inconsistency found when specifying `DeviceCreateInfo::enabled_extension_names`.");
            self.raw.enabledExtensionCount = enabled_extension_names.len() as _;
        }
        self
    }

    pub fn enabled_features<'m, 'a>(mut self, enabled_features: &'a PhysicalDeviceFeatures) -> DeviceCreateInfoBuilder<'b> {
        self.raw.pEnabledFeatures = enabled_features.raw();
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DeviceCreateFlags {
        DeviceCreateFlags::from_bits(self.raw.flags)
            .expect("DeviceCreateInfo::flags: error converting flags")
    }

    pub fn get_queue_create_infos<'a>(&'a self) -> &'a [DeviceQueueCreateInfo] {
        unsafe { slice::from_raw_parts(self.raw.pQueueCreateInfos as *const _, self.raw.queueCreateInfoCount as usize) }
    }

    pub fn get_enabled_layer_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledLayerNames as *const _, self.raw.enabledLayerCount as usize) }
    }

    pub fn get_enabled_extension_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledExtensionNames as *const _, self.raw.enabledExtensionCount as usize) }
    }

    pub fn get_enabled_features<'a>(&'a self) -> &'a PhysicalDeviceFeatures {
        unsafe { &*(self.raw.pEnabledFeatures as *const vks::VkPhysicalDeviceFeatures as *const _) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> InstanceCreateFlags {
        InstanceCreateFlags::from_bits(self.raw.flags)
            .expect("InstanceCreateInfo::flags: error converting flags")
    }

    pub fn application_info<'a>(&'a self) -> &'a ApplicationInfo {
        unsafe { &*(self.raw.pApplicationInfo as *const vks::VkApplicationInfo as *const _) }
    }

    pub fn enabled_layer_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledLayerNames as *const _, self.raw.enabledLayerCount as usize) }
    }

    pub fn enabled_extension_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledExtensionNames as *const _, self.raw.enabledExtensionCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: InstanceCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_application_info<'m, 'a>(&'s mut self, application_info: &'a ApplicationInfo) {
        self.raw.pApplicationInfo = application_info.raw();
    }

    pub fn set_enabled_layer_names<'m, 'a, T>(&'s mut self, enabled_layer_names: T)
            where 'a: 's, T: Into<CharStrs<'a>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        {
            let enabled_layer_names = self.enabled_layer_names.as_ref().unwrap();
            self.raw.ppEnabledLayerNames = enabled_layer_names.as_ptr();
            assert!(self.raw.enabledLayerCount == 0 || self.raw.enabledLayerCount == enabled_layer_names.len() as _, 
                "count inconsistency found when specifying `InstanceCreateInfo::enabled_layer_names`.");
            self.raw.enabledLayerCount = enabled_layer_names.len() as _;
        }
    }

    pub fn set_enabled_extension_names<'m, 'a, T>(&'s mut self, enabled_extension_names: T)
            where 'a: 's, T: Into<CharStrs<'a>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        {
            let enabled_extension_names = self.enabled_extension_names.as_ref().unwrap();
            self.raw.ppEnabledExtensionNames = enabled_extension_names.as_ptr();
            assert!(self.raw.enabledExtensionCount == 0 || self.raw.enabledExtensionCount == enabled_extension_names.len() as _, 
                "count inconsistency found when specifying `InstanceCreateInfo::enabled_extension_names`.");
            self.raw.enabledExtensionCount = enabled_extension_names.len() as _;
        }
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


impl<'s> From<vks::VkInstanceCreateInfo> for InstanceCreateInfo<'s> {
    fn from(f: vks::VkInstanceCreateInfo) -> InstanceCreateInfo<'s> {
        InstanceCreateInfo { raw: f, enabled_layer_names: None, enabled_extension_names: None, _p: PhantomData }
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

    pub fn enabled_layer_names<'m, 'a, T>(mut self, enabled_layer_names: T) -> InstanceCreateInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_layer_names = Some(enabled_layer_names.into());
        {
            let enabled_layer_names = self.enabled_layer_names.as_ref().unwrap();
            self.raw.ppEnabledLayerNames = enabled_layer_names.as_ptr();
            assert!(self.raw.enabledLayerCount == 0 || self.raw.enabledLayerCount == enabled_layer_names.len() as _, 
                "count inconsistency found when specifying `InstanceCreateInfo::enabled_layer_names`.");
            self.raw.enabledLayerCount = enabled_layer_names.len() as _;
        }
        self
    }

    pub fn enabled_extension_names<'m, 'a, T>(mut self, enabled_extension_names: T) -> InstanceCreateInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStrs<'a>> {
        self.enabled_extension_names = Some(enabled_extension_names.into());
        {
            let enabled_extension_names = self.enabled_extension_names.as_ref().unwrap();
            self.raw.ppEnabledExtensionNames = enabled_extension_names.as_ptr();
            assert!(self.raw.enabledExtensionCount == 0 || self.raw.enabledExtensionCount == enabled_extension_names.len() as _, 
                "count inconsistency found when specifying `InstanceCreateInfo::enabled_extension_names`.");
            self.raw.enabledExtensionCount = enabled_extension_names.len() as _;
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> InstanceCreateFlags {
        InstanceCreateFlags::from_bits(self.raw.flags)
            .expect("InstanceCreateInfo::flags: error converting flags")
    }

    pub fn get_application_info<'a>(&'a self) -> &'a ApplicationInfo {
        unsafe { &*(self.raw.pApplicationInfo as *const vks::VkApplicationInfo as *const _) }
    }

    pub fn get_enabled_layer_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledLayerNames as *const _, self.raw.enabledLayerCount as usize) }
    }

    pub fn get_enabled_extension_names<'a>(&'a self) -> &'a [*const c_char] {
        unsafe { slice::from_raw_parts(self.raw.ppEnabledExtensionNames as *const _, self.raw.enabledExtensionCount as usize) }
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
    pub fn queue_flags<'a>(&'a self) -> QueueFlags {
        QueueFlags::from_bits(self.raw.queueFlags)
            .expect("QueueFamilyProperties::queue_flags: error converting flags")
    }

    pub fn queue_count<'a>(&'a self) -> u32 {
        self.raw.queueCount.into()
    }

    pub fn timestamp_valid_bits<'a>(&'a self) -> u32 {
        self.raw.timestampValidBits.into()
    }

    pub fn min_image_transfer_granularity<'a>(&'a self) -> Extent3d {
        self.raw.minImageTransferGranularity.into()
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


impl From<vks::VkQueueFamilyProperties> for QueueFamilyProperties {
    fn from(f: vks::VkQueueFamilyProperties) -> QueueFamilyProperties {
        QueueFamilyProperties { raw: f, }
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
    pub fn memory_type_count<'a>(&'a self) -> u32 {
        self.raw.memoryTypeCount.into()
    }

    pub fn memory_types<'a>(&'a self) -> &[MemoryType] {
        unsafe { slice::from_raw_parts(&self.raw.memoryTypes as *const vks::VkMemoryType as *const _, vks::VK_MAX_MEMORY_TYPES as usize) }
    }

    pub fn memory_heap_count<'a>(&'a self) -> u32 {
        self.raw.memoryHeapCount.into()
    }

    pub fn memory_heaps<'a>(&'a self) -> &[MemoryHeap] {
        unsafe { slice::from_raw_parts(&self.raw.memoryHeaps as *const vks::VkMemoryHeap as *const _, vks::VK_MAX_MEMORY_HEAPS as usize) }
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


impl From<vks::VkPhysicalDeviceMemoryProperties> for PhysicalDeviceMemoryProperties {
    fn from(f: vks::VkPhysicalDeviceMemoryProperties) -> PhysicalDeviceMemoryProperties {
        PhysicalDeviceMemoryProperties { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn allocation_size<'a>(&'a self) -> u64 {
        self.raw.allocationSize.into()
    }

    pub fn memory_type_index<'a>(&'a self) -> u32 {
        self.raw.memoryTypeIndex.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_allocation_size<'m>(&'s mut self, allocation_size: u64) {
        self.raw.allocationSize = allocation_size.into();
    }

    pub fn set_memory_type_index<'m>(&'s mut self, memory_type_index: u32) {
        self.raw.memoryTypeIndex = memory_type_index.into();
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


impl<'s> From<vks::VkMemoryAllocateInfo> for MemoryAllocateInfo<'s> {
    fn from(f: vks::VkMemoryAllocateInfo) -> MemoryAllocateInfo<'s> {
        MemoryAllocateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_allocation_size<'a>(&'a self) -> u64 {
        self.raw.allocationSize.into()
    }

    pub fn get_memory_type_index<'a>(&'a self) -> u32 {
        self.raw.memoryTypeIndex.into()
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
    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn alignment<'a>(&'a self) -> u64 {
        self.raw.alignment.into()
    }

    pub fn memory_type_bits<'a>(&'a self) -> u32 {
        self.raw.memoryTypeBits.into()
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


impl From<vks::VkMemoryRequirements> for MemoryRequirements {
    fn from(f: vks::VkMemoryRequirements) -> MemoryRequirements {
        MemoryRequirements { raw: f, }
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
    pub fn aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("SparseImageFormatProperties::aspect_mask: error converting flags")
    }

    pub fn image_granularity<'a>(&'a self) -> Extent3d {
        self.raw.imageGranularity.into()
    }

    pub fn flags<'a>(&'a self) -> SparseImageFormatFlags {
        SparseImageFormatFlags::from_bits(self.raw.flags)
            .expect("SparseImageFormatProperties::flags: error converting flags")
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


impl From<vks::VkSparseImageFormatProperties> for SparseImageFormatProperties {
    fn from(f: vks::VkSparseImageFormatProperties) -> SparseImageFormatProperties {
        SparseImageFormatProperties { raw: f, }
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
    pub fn format_properties<'a>(&'a self) -> SparseImageFormatProperties {
        self.raw.formatProperties.into()
    }

    pub fn image_mip_tail_first_lod<'a>(&'a self) -> u32 {
        self.raw.imageMipTailFirstLod.into()
    }

    pub fn image_mip_tail_size<'a>(&'a self) -> u64 {
        self.raw.imageMipTailSize.into()
    }

    pub fn image_mip_tail_offset<'a>(&'a self) -> u64 {
        self.raw.imageMipTailOffset.into()
    }

    pub fn image_mip_tail_stride<'a>(&'a self) -> u64 {
        self.raw.imageMipTailStride.into()
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


impl From<vks::VkSparseImageMemoryRequirements> for SparseImageMemoryRequirements {
    fn from(f: vks::VkSparseImageMemoryRequirements) -> SparseImageMemoryRequirements {
        SparseImageMemoryRequirements { raw: f, }
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
    pub fn property_flags<'a>(&'a self) -> MemoryPropertyFlags {
        MemoryPropertyFlags::from_bits(self.raw.propertyFlags)
            .expect("MemoryType::property_flags: error converting flags")
    }

    pub fn heap_index<'a>(&'a self) -> u32 {
        self.raw.heapIndex.into()
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


impl From<vks::VkMemoryType> for MemoryType {
    fn from(f: vks::VkMemoryType) -> MemoryType {
        MemoryType { raw: f, }
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
    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn flags<'a>(&'a self) -> MemoryHeapFlags {
        MemoryHeapFlags::from_bits(self.raw.flags)
            .expect("MemoryHeap::flags: error converting flags")
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


impl From<vks::VkMemoryHeap> for MemoryHeap {
    fn from(f: vks::VkMemoryHeap) -> MemoryHeap {
        MemoryHeap { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_memory<'m, 'a>(&'s mut self, memory: &'a DeviceMemory) {
        self.raw.memory = memory.handle();
    }

    pub fn set_offset<'m>(&'s mut self, offset: u64) {
        self.raw.offset = offset.into();
    }

    pub fn set_size<'m>(&'s mut self, size: u64) {
        self.raw.size = size.into();
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


impl<'s> From<vks::VkMappedMemoryRange> for MappedMemoryRange<'s> {
    fn from(f: vks::VkMappedMemoryRange) -> MappedMemoryRange<'s> {
        MappedMemoryRange { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn get_offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn get_size<'a>(&'a self) -> u64 {
        self.raw.size.into()
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
    pub fn linear_tiling_features<'a>(&'a self) -> FormatFeatureFlags {
        FormatFeatureFlags::from_bits(self.raw.linearTilingFeatures)
            .expect("FormatProperties::linear_tiling_features: error converting flags")
    }

    pub fn optimal_tiling_features<'a>(&'a self) -> FormatFeatureFlags {
        FormatFeatureFlags::from_bits(self.raw.optimalTilingFeatures)
            .expect("FormatProperties::optimal_tiling_features: error converting flags")
    }

    pub fn buffer_features<'a>(&'a self) -> FormatFeatureFlags {
        FormatFeatureFlags::from_bits(self.raw.bufferFeatures)
            .expect("FormatProperties::buffer_features: error converting flags")
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


impl From<vks::VkFormatProperties> for FormatProperties {
    fn from(f: vks::VkFormatProperties) -> FormatProperties {
        FormatProperties { raw: f, }
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
    pub fn max_extent<'a>(&'a self) -> Extent3d {
        self.raw.maxExtent.into()
    }

    pub fn max_mip_levels<'a>(&'a self) -> u32 {
        self.raw.maxMipLevels.into()
    }

    pub fn max_array_layers<'a>(&'a self) -> u32 {
        self.raw.maxArrayLayers.into()
    }

    pub fn sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.sampleCounts)
            .expect("ImageFormatProperties::sample_counts: error converting flags")
    }

    pub fn max_resource_size<'a>(&'a self) -> u64 {
        self.raw.maxResourceSize.into()
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


impl From<vks::VkImageFormatProperties> for ImageFormatProperties {
    fn from(f: vks::VkImageFormatProperties) -> ImageFormatProperties {
        ImageFormatProperties { raw: f, }
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

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn range<'a>(&'a self) -> u64 {
        self.raw.range.into()
    }

    pub fn set_buffer<'m, 'a>(& mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
    }

    pub fn set_offset<'m>(& mut self, offset: u64) {
        self.raw.offset = offset.into();
    }

    pub fn set_range<'m>(& mut self, range: u64) {
        self.raw.range = range.into();
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


impl From<vks::VkDescriptorBufferInfo> for DescriptorBufferInfo {
    fn from(f: vks::VkDescriptorBufferInfo) -> DescriptorBufferInfo {
        DescriptorBufferInfo { raw: f, }
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

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn get_offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn get_range<'a>(&'a self) -> u64 {
        self.raw.range.into()
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

    pub fn sampler_handle<'a>(&'a self) -> vks::VkSampler {
        self.raw.sampler
    }

    pub fn image_view_handle<'a>(&'a self) -> vks::VkImageView {
        self.raw.imageView
    }

    pub fn image_layout<'a>(&'a self) -> ImageLayout {
        self.raw.imageLayout.into()
    }

    pub fn set_sampler<'m, 'a>(& mut self, sampler: &'a Sampler) {
        self.raw.sampler = sampler.handle();
    }

    pub fn set_image_view<'m, 'a>(& mut self, image_view: &'a ImageView) {
        self.raw.imageView = image_view.handle();
    }

    pub fn set_image_layout<'m>(& mut self, image_layout: ImageLayout) {
        self.raw.imageLayout = image_layout.into();
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


impl From<vks::VkDescriptorImageInfo> for DescriptorImageInfo {
    fn from(f: vks::VkDescriptorImageInfo) -> DescriptorImageInfo {
        DescriptorImageInfo { raw: f, }
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

    pub fn get_sampler_handle<'a>(&'a self) -> vks::VkSampler {
        self.raw.sampler
    }

    pub fn get_image_view_handle<'a>(&'a self) -> vks::VkImageView {
        self.raw.imageView
    }

    pub fn get_image_layout<'a>(&'a self) -> ImageLayout {
        self.raw.imageLayout.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn dst_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.dstSet
    }

    pub fn dst_binding<'a>(&'a self) -> u32 {
        self.raw.dstBinding.into()
    }

    pub fn dst_array_element<'a>(&'a self) -> u32 {
        self.raw.dstArrayElement.into()
    }

    pub fn descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn descriptor_type<'a>(&'a self) -> DescriptorType {
        self.raw.descriptorType.into()
    }

    pub fn image_info<'a>(&'a self) -> &'a DescriptorImageInfo {
        unsafe { &*(self.raw.pImageInfo as *const vks::VkDescriptorImageInfo as *const _) }
    }

    pub fn buffer_info<'a>(&'a self) -> &'a DescriptorBufferInfo {
        unsafe { &*(self.raw.pBufferInfo as *const vks::VkDescriptorBufferInfo as *const _) }
    }

    pub fn texel_buffer_view_handle<'a>(&'a self) -> &'a vks::VkBufferView {
        unsafe { &*(self.raw.pTexelBufferView as *const _) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_dst_set<'m, 'a>(&'s mut self, dst_set: &'a DescriptorSet) {
        self.raw.dstSet = dst_set.handle();
    }

    pub fn set_dst_binding<'m>(&'s mut self, dst_binding: u32) {
        self.raw.dstBinding = dst_binding.into();
    }

    pub fn set_dst_array_element<'m>(&'s mut self, dst_array_element: u32) {
        self.raw.dstArrayElement = dst_array_element.into();
    }

    pub fn set_descriptor_count<'m>(&'s mut self, descriptor_count: u32) {
        self.raw.descriptorCount = descriptor_count.into();
    }

    pub fn set_descriptor_type<'m>(&'s mut self, descriptor_type: DescriptorType) {
        self.raw.descriptorType = descriptor_type.into();
    }

    pub fn set_image_info<'m, 'a>(&'s mut self, image_info: &'a DescriptorImageInfo) {
        self.raw.pImageInfo = image_info.raw();
    }

    pub fn set_buffer_info<'m, 'a>(&'s mut self, buffer_info: &'a DescriptorBufferInfo) {
        self.raw.pBufferInfo = buffer_info.raw();
    }

    pub fn set_texel_buffer_view<'m, 'a>(&'s mut self, texel_buffer_view: &'a BufferView) {
        self.raw.pTexelBufferView = &texel_buffer_view.handle();
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


impl<'s> From<vks::VkWriteDescriptorSet> for WriteDescriptorSet<'s> {
    fn from(f: vks::VkWriteDescriptorSet) -> WriteDescriptorSet<'s> {
        WriteDescriptorSet { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_dst_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.dstSet
    }

    pub fn get_dst_binding<'a>(&'a self) -> u32 {
        self.raw.dstBinding.into()
    }

    pub fn get_dst_array_element<'a>(&'a self) -> u32 {
        self.raw.dstArrayElement.into()
    }

    pub fn get_descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn get_descriptor_type<'a>(&'a self) -> DescriptorType {
        self.raw.descriptorType.into()
    }

    pub fn get_image_info<'a>(&'a self) -> &'a DescriptorImageInfo {
        unsafe { &*(self.raw.pImageInfo as *const vks::VkDescriptorImageInfo as *const _) }
    }

    pub fn get_buffer_info<'a>(&'a self) -> &'a DescriptorBufferInfo {
        unsafe { &*(self.raw.pBufferInfo as *const vks::VkDescriptorBufferInfo as *const _) }
    }

    pub fn get_texel_buffer_view_handle<'a>(&'a self) -> &'a vks::VkBufferView {
        unsafe { &*(self.raw.pTexelBufferView as *const _) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn src_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.srcSet
    }

    pub fn src_binding<'a>(&'a self) -> u32 {
        self.raw.srcBinding.into()
    }

    pub fn src_array_element<'a>(&'a self) -> u32 {
        self.raw.srcArrayElement.into()
    }

    pub fn dst_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.dstSet
    }

    pub fn dst_binding<'a>(&'a self) -> u32 {
        self.raw.dstBinding.into()
    }

    pub fn dst_array_element<'a>(&'a self) -> u32 {
        self.raw.dstArrayElement.into()
    }

    pub fn descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_src_set<'m, 'a>(&'s mut self, src_set: &'a DescriptorSet) {
        self.raw.srcSet = src_set.handle();
    }

    pub fn set_src_binding<'m>(&'s mut self, src_binding: u32) {
        self.raw.srcBinding = src_binding.into();
    }

    pub fn set_src_array_element<'m>(&'s mut self, src_array_element: u32) {
        self.raw.srcArrayElement = src_array_element.into();
    }

    pub fn set_dst_set<'m, 'a>(&'s mut self, dst_set: &'a DescriptorSet) {
        self.raw.dstSet = dst_set.handle();
    }

    pub fn set_dst_binding<'m>(&'s mut self, dst_binding: u32) {
        self.raw.dstBinding = dst_binding.into();
    }

    pub fn set_dst_array_element<'m>(&'s mut self, dst_array_element: u32) {
        self.raw.dstArrayElement = dst_array_element.into();
    }

    pub fn set_descriptor_count<'m>(&'s mut self, descriptor_count: u32) {
        self.raw.descriptorCount = descriptor_count.into();
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


impl<'s> From<vks::VkCopyDescriptorSet> for CopyDescriptorSet<'s> {
    fn from(f: vks::VkCopyDescriptorSet) -> CopyDescriptorSet<'s> {
        CopyDescriptorSet { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_src_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.srcSet
    }

    pub fn get_src_binding<'a>(&'a self) -> u32 {
        self.raw.srcBinding.into()
    }

    pub fn get_src_array_element<'a>(&'a self) -> u32 {
        self.raw.srcArrayElement.into()
    }

    pub fn get_dst_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.dstSet
    }

    pub fn get_dst_binding<'a>(&'a self) -> u32 {
        self.raw.dstBinding.into()
    }

    pub fn get_dst_array_element<'a>(&'a self) -> u32 {
        self.raw.dstArrayElement.into()
    }

    pub fn get_descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> BufferCreateFlags {
        BufferCreateFlags::from_bits(self.raw.flags)
            .expect("BufferCreateInfo::flags: error converting flags")
    }

    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn usage<'a>(&'a self) -> BufferUsageFlags {
        BufferUsageFlags::from_bits(self.raw.usage)
            .expect("BufferCreateInfo::usage: error converting flags")
    }

    pub fn sharing_mode<'a>(&'a self) -> SharingMode {
        self.raw.sharingMode.into()
    }

    pub fn queue_family_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pQueueFamilyIndices as *const _, self.raw.queueFamilyIndexCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: BufferCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_size<'m>(&'s mut self, size: u64) {
        self.raw.size = size.into();
    }

    pub fn set_usage<'m>(&'s mut self, usage: BufferUsageFlags) {
        self.raw.usage = usage.bits();
    }

    pub fn set_sharing_mode<'m>(&'s mut self, sharing_mode: SharingMode) {
        self.raw.sharingMode = sharing_mode.into();
    }

    pub fn set_queue_family_indices<'m, 'a>(&'s mut self, queue_family_indices: &'a [u32]) {
        assert!(self.raw.queueFamilyIndexCount == 0 || self.raw.queueFamilyIndexCount == queue_family_indices.len() as _, 
            "count inconsistency found when specifying `BufferCreateInfo::queue_family_indices`.");
        self.raw.queueFamilyIndexCount = queue_family_indices.len() as _;
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const u32 as *const _;
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


impl<'s> From<vks::VkBufferCreateInfo> for BufferCreateInfo<'s> {
    fn from(f: vks::VkBufferCreateInfo) -> BufferCreateInfo<'s> {
        BufferCreateInfo { raw: f, _p: PhantomData }
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

    pub fn queue_family_indices<'m, 'a>(mut self, queue_family_indices: &'a [u32]) -> BufferCreateInfoBuilder<'b> {
        assert!(self.raw.queueFamilyIndexCount == 0 || self.raw.queueFamilyIndexCount == queue_family_indices.len() as _, 
            "count inconsistency found when specifying `BufferCreateInfo::queue_family_indices`.");
        self.raw.queueFamilyIndexCount = queue_family_indices.len() as _;
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> BufferCreateFlags {
        BufferCreateFlags::from_bits(self.raw.flags)
            .expect("BufferCreateInfo::flags: error converting flags")
    }

    pub fn get_size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn get_usage<'a>(&'a self) -> BufferUsageFlags {
        BufferUsageFlags::from_bits(self.raw.usage)
            .expect("BufferCreateInfo::usage: error converting flags")
    }

    pub fn get_sharing_mode<'a>(&'a self) -> SharingMode {
        self.raw.sharingMode.into()
    }

    pub fn get_queue_family_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pQueueFamilyIndices as *const _, self.raw.queueFamilyIndexCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> BufferViewCreateFlags {
        BufferViewCreateFlags::from_bits(self.raw.flags)
            .expect("BufferViewCreateInfo::flags: error converting flags")
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn range<'a>(&'a self) -> u64 {
        self.raw.range.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: BufferViewCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_buffer<'m, 'a>(&'s mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
    }

    pub fn set_format<'m>(&'s mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_offset<'m>(&'s mut self, offset: u64) {
        self.raw.offset = offset.into();
    }

    pub fn set_range<'m>(&'s mut self, range: u64) {
        self.raw.range = range.into();
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


impl<'s> From<vks::VkBufferViewCreateInfo> for BufferViewCreateInfo<'s> {
    fn from(f: vks::VkBufferViewCreateInfo) -> BufferViewCreateInfo<'s> {
        BufferViewCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> BufferViewCreateFlags {
        BufferViewCreateFlags::from_bits(self.raw.flags)
            .expect("BufferViewCreateInfo::flags: error converting flags")
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn get_range<'a>(&'a self) -> u64 {
        self.raw.range.into()
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

    pub fn aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ImageSubresource::aspect_mask: error converting flags")
    }

    pub fn mip_level<'a>(&'a self) -> u32 {
        self.raw.mipLevel.into()
    }

    pub fn array_layer<'a>(&'a self) -> u32 {
        self.raw.arrayLayer.into()
    }

    pub fn set_aspect_mask<'m>(& mut self, aspect_mask: ImageAspectFlags) {
        self.raw.aspectMask = aspect_mask.bits();
    }

    pub fn set_mip_level<'m>(& mut self, mip_level: u32) {
        self.raw.mipLevel = mip_level.into();
    }

    pub fn set_array_layer<'m>(& mut self, array_layer: u32) {
        self.raw.arrayLayer = array_layer.into();
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


impl From<vks::VkImageSubresource> for ImageSubresource {
    fn from(f: vks::VkImageSubresource) -> ImageSubresource {
        ImageSubresource { raw: f, }
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

    pub fn get_aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ImageSubresource::aspect_mask: error converting flags")
    }

    pub fn get_mip_level<'a>(&'a self) -> u32 {
        self.raw.mipLevel.into()
    }

    pub fn get_array_layer<'a>(&'a self) -> u32 {
        self.raw.arrayLayer.into()
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

    pub fn aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ImageSubresourceLayers::aspect_mask: error converting flags")
    }

    pub fn mip_level<'a>(&'a self) -> u32 {
        self.raw.mipLevel.into()
    }

    pub fn base_array_layer<'a>(&'a self) -> u32 {
        self.raw.baseArrayLayer.into()
    }

    pub fn layer_count<'a>(&'a self) -> u32 {
        self.raw.layerCount.into()
    }

    pub fn set_aspect_mask<'m>(& mut self, aspect_mask: ImageAspectFlags) {
        self.raw.aspectMask = aspect_mask.bits();
    }

    pub fn set_mip_level<'m>(& mut self, mip_level: u32) {
        self.raw.mipLevel = mip_level.into();
    }

    pub fn set_base_array_layer<'m>(& mut self, base_array_layer: u32) {
        self.raw.baseArrayLayer = base_array_layer.into();
    }

    pub fn set_layer_count<'m>(& mut self, layer_count: u32) {
        self.raw.layerCount = layer_count.into();
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


impl From<vks::VkImageSubresourceLayers> for ImageSubresourceLayers {
    fn from(f: vks::VkImageSubresourceLayers) -> ImageSubresourceLayers {
        ImageSubresourceLayers { raw: f, }
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

    pub fn get_aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ImageSubresourceLayers::aspect_mask: error converting flags")
    }

    pub fn get_mip_level<'a>(&'a self) -> u32 {
        self.raw.mipLevel.into()
    }

    pub fn get_base_array_layer<'a>(&'a self) -> u32 {
        self.raw.baseArrayLayer.into()
    }

    pub fn get_layer_count<'a>(&'a self) -> u32 {
        self.raw.layerCount.into()
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

    pub fn aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ImageSubresourceRange::aspect_mask: error converting flags")
    }

    pub fn base_mip_level<'a>(&'a self) -> u32 {
        self.raw.baseMipLevel.into()
    }

    pub fn level_count<'a>(&'a self) -> u32 {
        self.raw.levelCount.into()
    }

    pub fn base_array_layer<'a>(&'a self) -> u32 {
        self.raw.baseArrayLayer.into()
    }

    pub fn layer_count<'a>(&'a self) -> u32 {
        self.raw.layerCount.into()
    }

    pub fn set_aspect_mask<'m>(& mut self, aspect_mask: ImageAspectFlags) {
        self.raw.aspectMask = aspect_mask.bits();
    }

    pub fn set_base_mip_level<'m>(& mut self, base_mip_level: u32) {
        self.raw.baseMipLevel = base_mip_level.into();
    }

    pub fn set_level_count<'m>(& mut self, level_count: u32) {
        self.raw.levelCount = level_count.into();
    }

    pub fn set_base_array_layer<'m>(& mut self, base_array_layer: u32) {
        self.raw.baseArrayLayer = base_array_layer.into();
    }

    pub fn set_layer_count<'m>(& mut self, layer_count: u32) {
        self.raw.layerCount = layer_count.into();
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


impl From<vks::VkImageSubresourceRange> for ImageSubresourceRange {
    fn from(f: vks::VkImageSubresourceRange) -> ImageSubresourceRange {
        ImageSubresourceRange { raw: f, }
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

    pub fn get_aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ImageSubresourceRange::aspect_mask: error converting flags")
    }

    pub fn get_base_mip_level<'a>(&'a self) -> u32 {
        self.raw.baseMipLevel.into()
    }

    pub fn get_level_count<'a>(&'a self) -> u32 {
        self.raw.levelCount.into()
    }

    pub fn get_base_array_layer<'a>(&'a self) -> u32 {
        self.raw.baseArrayLayer.into()
    }

    pub fn get_layer_count<'a>(&'a self) -> u32 {
        self.raw.layerCount.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("MemoryBarrier::src_access_mask: error converting flags")
    }

    pub fn dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("MemoryBarrier::dst_access_mask: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_src_access_mask<'m>(&'s mut self, src_access_mask: AccessFlags) {
        self.raw.srcAccessMask = src_access_mask.bits();
    }

    pub fn set_dst_access_mask<'m>(&'s mut self, dst_access_mask: AccessFlags) {
        self.raw.dstAccessMask = dst_access_mask.bits();
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


impl<'s> From<vks::VkMemoryBarrier> for MemoryBarrier<'s> {
    fn from(f: vks::VkMemoryBarrier) -> MemoryBarrier<'s> {
        MemoryBarrier { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("MemoryBarrier::src_access_mask: error converting flags")
    }

    pub fn get_dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("MemoryBarrier::dst_access_mask: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("BufferMemoryBarrier::src_access_mask: error converting flags")
    }

    pub fn dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("BufferMemoryBarrier::dst_access_mask: error converting flags")
    }

    pub fn src_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.srcQueueFamilyIndex.into()
    }

    pub fn dst_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.dstQueueFamilyIndex.into()
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_src_access_mask<'m>(&'s mut self, src_access_mask: AccessFlags) {
        self.raw.srcAccessMask = src_access_mask.bits();
    }

    pub fn set_dst_access_mask<'m>(&'s mut self, dst_access_mask: AccessFlags) {
        self.raw.dstAccessMask = dst_access_mask.bits();
    }

    pub fn set_src_queue_family_index<'m>(&'s mut self, src_queue_family_index: u32) {
        self.raw.srcQueueFamilyIndex = src_queue_family_index.into();
    }

    pub fn set_dst_queue_family_index<'m>(&'s mut self, dst_queue_family_index: u32) {
        self.raw.dstQueueFamilyIndex = dst_queue_family_index.into();
    }

    pub fn set_buffer<'m, 'a>(&'s mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
    }

    pub fn set_offset<'m>(&'s mut self, offset: u64) {
        self.raw.offset = offset.into();
    }

    pub fn set_size<'m>(&'s mut self, size: u64) {
        self.raw.size = size.into();
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


impl<'s> From<vks::VkBufferMemoryBarrier> for BufferMemoryBarrier<'s> {
    fn from(f: vks::VkBufferMemoryBarrier) -> BufferMemoryBarrier<'s> {
        BufferMemoryBarrier { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("BufferMemoryBarrier::src_access_mask: error converting flags")
    }

    pub fn get_dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("BufferMemoryBarrier::dst_access_mask: error converting flags")
    }

    pub fn get_src_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.srcQueueFamilyIndex.into()
    }

    pub fn get_dst_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.dstQueueFamilyIndex.into()
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn get_offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn get_size<'a>(&'a self) -> u64 {
        self.raw.size.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("ImageMemoryBarrier::src_access_mask: error converting flags")
    }

    pub fn dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("ImageMemoryBarrier::dst_access_mask: error converting flags")
    }

    pub fn old_layout<'a>(&'a self) -> ImageLayout {
        self.raw.oldLayout.into()
    }

    pub fn new_layout<'a>(&'a self) -> ImageLayout {
        self.raw.newLayout.into()
    }

    pub fn src_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.srcQueueFamilyIndex.into()
    }

    pub fn dst_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.dstQueueFamilyIndex.into()
    }

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn subresource_range<'a>(&'a self) -> ImageSubresourceRange {
        self.raw.subresourceRange.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_src_access_mask<'m>(&'s mut self, src_access_mask: AccessFlags) {
        self.raw.srcAccessMask = src_access_mask.bits();
    }

    pub fn set_dst_access_mask<'m>(&'s mut self, dst_access_mask: AccessFlags) {
        self.raw.dstAccessMask = dst_access_mask.bits();
    }

    pub fn set_old_layout<'m>(&'s mut self, old_layout: ImageLayout) {
        self.raw.oldLayout = old_layout.into();
    }

    pub fn set_new_layout<'m>(&'s mut self, new_layout: ImageLayout) {
        self.raw.newLayout = new_layout.into();
    }

    pub fn set_src_queue_family_index<'m>(&'s mut self, src_queue_family_index: u32) {
        self.raw.srcQueueFamilyIndex = src_queue_family_index.into();
    }

    pub fn set_dst_queue_family_index<'m>(&'s mut self, dst_queue_family_index: u32) {
        self.raw.dstQueueFamilyIndex = dst_queue_family_index.into();
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
    }

    pub fn set_subresource_range<'m>(&'s mut self, subresource_range: ImageSubresourceRange) {
        self.raw.subresourceRange = subresource_range.raw;
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


impl<'s> From<vks::VkImageMemoryBarrier> for ImageMemoryBarrier<'s> {
    fn from(f: vks::VkImageMemoryBarrier) -> ImageMemoryBarrier<'s> {
        ImageMemoryBarrier { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("ImageMemoryBarrier::src_access_mask: error converting flags")
    }

    pub fn get_dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("ImageMemoryBarrier::dst_access_mask: error converting flags")
    }

    pub fn get_old_layout<'a>(&'a self) -> ImageLayout {
        self.raw.oldLayout.into()
    }

    pub fn get_new_layout<'a>(&'a self) -> ImageLayout {
        self.raw.newLayout.into()
    }

    pub fn get_src_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.srcQueueFamilyIndex.into()
    }

    pub fn get_dst_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.dstQueueFamilyIndex.into()
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn get_subresource_range<'a>(&'a self) -> ImageSubresourceRange {
        self.raw.subresourceRange.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> ImageCreateFlags {
        ImageCreateFlags::from_bits(self.raw.flags)
            .expect("ImageCreateInfo::flags: error converting flags")
    }

    pub fn image_type<'a>(&'a self) -> ImageType {
        self.raw.imageType.into()
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
    }

    pub fn mip_levels<'a>(&'a self) -> u32 {
        self.raw.mipLevels.into()
    }

    pub fn array_layers<'a>(&'a self) -> u32 {
        self.raw.arrayLayers.into()
    }

    pub fn samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.samples)
            .expect("ImageCreateInfo::samples: error converting flags")
    }

    pub fn tiling<'a>(&'a self) -> ImageTiling {
        self.raw.tiling.into()
    }

    pub fn usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.usage)
            .expect("ImageCreateInfo::usage: error converting flags")
    }

    pub fn sharing_mode<'a>(&'a self) -> SharingMode {
        self.raw.sharingMode.into()
    }

    pub fn queue_family_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pQueueFamilyIndices as *const _, self.raw.queueFamilyIndexCount as usize) }
    }

    pub fn initial_layout<'a>(&'a self) -> ImageLayout {
        self.raw.initialLayout.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: ImageCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_image_type<'m>(&'s mut self, image_type: ImageType) {
        self.raw.imageType = image_type.into();
    }

    pub fn set_format<'m>(&'s mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_extent<'m>(&'s mut self, extent: Extent3d) {
        self.raw.extent = extent.raw;
    }

    pub fn set_mip_levels<'m>(&'s mut self, mip_levels: u32) {
        self.raw.mipLevels = mip_levels.into();
    }

    pub fn set_array_layers<'m>(&'s mut self, array_layers: u32) {
        self.raw.arrayLayers = array_layers.into();
    }

    pub fn set_samples<'m>(&'s mut self, samples: SampleCountFlags) {
        self.raw.samples = samples.bits();
    }

    pub fn set_tiling<'m>(&'s mut self, tiling: ImageTiling) {
        self.raw.tiling = tiling.into();
    }

    pub fn set_usage<'m>(&'s mut self, usage: ImageUsageFlags) {
        self.raw.usage = usage.bits();
    }

    pub fn set_sharing_mode<'m>(&'s mut self, sharing_mode: SharingMode) {
        self.raw.sharingMode = sharing_mode.into();
    }

    pub fn set_queue_family_indices<'m, 'a>(&'s mut self, queue_family_indices: &'a [u32]) {
        assert!(self.raw.queueFamilyIndexCount == 0 || self.raw.queueFamilyIndexCount == queue_family_indices.len() as _, 
            "count inconsistency found when specifying `ImageCreateInfo::queue_family_indices`.");
        self.raw.queueFamilyIndexCount = queue_family_indices.len() as _;
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const u32 as *const _;
    }

    pub fn set_initial_layout<'m>(&'s mut self, initial_layout: ImageLayout) {
        self.raw.initialLayout = initial_layout.into();
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


impl<'s> From<vks::VkImageCreateInfo> for ImageCreateInfo<'s> {
    fn from(f: vks::VkImageCreateInfo) -> ImageCreateInfo<'s> {
        ImageCreateInfo { raw: f, _p: PhantomData }
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

    pub fn queue_family_indices<'m, 'a>(mut self, queue_family_indices: &'a [u32]) -> ImageCreateInfoBuilder<'b> {
        assert!(self.raw.queueFamilyIndexCount == 0 || self.raw.queueFamilyIndexCount == queue_family_indices.len() as _, 
            "count inconsistency found when specifying `ImageCreateInfo::queue_family_indices`.");
        self.raw.queueFamilyIndexCount = queue_family_indices.len() as _;
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn initial_layout<'m>(mut self, initial_layout: ImageLayout) -> ImageCreateInfoBuilder<'b> {
        self.raw.initialLayout = initial_layout.into();
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> ImageCreateFlags {
        ImageCreateFlags::from_bits(self.raw.flags)
            .expect("ImageCreateInfo::flags: error converting flags")
    }

    pub fn get_image_type<'a>(&'a self) -> ImageType {
        self.raw.imageType.into()
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
    }

    pub fn get_mip_levels<'a>(&'a self) -> u32 {
        self.raw.mipLevels.into()
    }

    pub fn get_array_layers<'a>(&'a self) -> u32 {
        self.raw.arrayLayers.into()
    }

    pub fn get_samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.samples)
            .expect("ImageCreateInfo::samples: error converting flags")
    }

    pub fn get_tiling<'a>(&'a self) -> ImageTiling {
        self.raw.tiling.into()
    }

    pub fn get_usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.usage)
            .expect("ImageCreateInfo::usage: error converting flags")
    }

    pub fn get_sharing_mode<'a>(&'a self) -> SharingMode {
        self.raw.sharingMode.into()
    }

    pub fn get_queue_family_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pQueueFamilyIndices as *const _, self.raw.queueFamilyIndexCount as usize) }
    }

    pub fn get_initial_layout<'a>(&'a self) -> ImageLayout {
        self.raw.initialLayout.into()
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
    pub fn offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn row_pitch<'a>(&'a self) -> u64 {
        self.raw.rowPitch.into()
    }

    pub fn array_pitch<'a>(&'a self) -> u64 {
        self.raw.arrayPitch.into()
    }

    pub fn depth_pitch<'a>(&'a self) -> u64 {
        self.raw.depthPitch.into()
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


impl From<vks::VkSubresourceLayout> for SubresourceLayout {
    fn from(f: vks::VkSubresourceLayout) -> SubresourceLayout {
        SubresourceLayout { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> ImageViewCreateFlags {
        ImageViewCreateFlags::from_bits(self.raw.flags)
            .expect("ImageViewCreateInfo::flags: error converting flags")
    }

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn view_type<'a>(&'a self) -> ImageViewType {
        self.raw.viewType.into()
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn components<'a>(&'a self) -> ComponentMapping {
        self.raw.components.into()
    }

    pub fn subresource_range<'a>(&'a self) -> ImageSubresourceRange {
        self.raw.subresourceRange.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: ImageViewCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
    }

    pub fn set_view_type<'m>(&'s mut self, view_type: ImageViewType) {
        self.raw.viewType = view_type.into();
    }

    pub fn set_format<'m>(&'s mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_components<'m>(&'s mut self, components: ComponentMapping) {
        self.raw.components = components.raw;
    }

    pub fn set_subresource_range<'m>(&'s mut self, subresource_range: ImageSubresourceRange) {
        self.raw.subresourceRange = subresource_range.raw;
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


impl<'s> From<vks::VkImageViewCreateInfo> for ImageViewCreateInfo<'s> {
    fn from(f: vks::VkImageViewCreateInfo) -> ImageViewCreateInfo<'s> {
        ImageViewCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> ImageViewCreateFlags {
        ImageViewCreateFlags::from_bits(self.raw.flags)
            .expect("ImageViewCreateInfo::flags: error converting flags")
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn get_view_type<'a>(&'a self) -> ImageViewType {
        self.raw.viewType.into()
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_components<'a>(&'a self) -> ComponentMapping {
        self.raw.components.into()
    }

    pub fn get_subresource_range<'a>(&'a self) -> ImageSubresourceRange {
        self.raw.subresourceRange.into()
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

    pub fn src_offset<'a>(&'a self) -> u64 {
        self.raw.srcOffset.into()
    }

    pub fn dst_offset<'a>(&'a self) -> u64 {
        self.raw.dstOffset.into()
    }

    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn set_src_offset<'m>(& mut self, src_offset: u64) {
        self.raw.srcOffset = src_offset.into();
    }

    pub fn set_dst_offset<'m>(& mut self, dst_offset: u64) {
        self.raw.dstOffset = dst_offset.into();
    }

    pub fn set_size<'m>(& mut self, size: u64) {
        self.raw.size = size.into();
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


impl From<vks::VkBufferCopy> for BufferCopy {
    fn from(f: vks::VkBufferCopy) -> BufferCopy {
        BufferCopy { raw: f, }
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

    pub fn get_src_offset<'a>(&'a self) -> u64 {
        self.raw.srcOffset.into()
    }

    pub fn get_dst_offset<'a>(&'a self) -> u64 {
        self.raw.dstOffset.into()
    }

    pub fn get_size<'a>(&'a self) -> u64 {
        self.raw.size.into()
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

    pub fn resource_offset<'a>(&'a self) -> u64 {
        self.raw.resourceOffset.into()
    }

    pub fn size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn memory_offset<'a>(&'a self) -> u64 {
        self.raw.memoryOffset.into()
    }

    pub fn flags<'a>(&'a self) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags::from_bits(self.raw.flags)
            .expect("SparseMemoryBind::flags: error converting flags")
    }

    pub fn set_resource_offset<'m>(& mut self, resource_offset: u64) {
        self.raw.resourceOffset = resource_offset.into();
    }

    pub fn set_size<'m>(& mut self, size: u64) {
        self.raw.size = size.into();
    }

    pub fn set_memory<'m, 'a>(& mut self, memory: &'a DeviceMemory) {
        self.raw.memory = memory.handle();
    }

    pub fn set_memory_offset<'m>(& mut self, memory_offset: u64) {
        self.raw.memoryOffset = memory_offset.into();
    }

    pub fn set_flags<'m>(& mut self, flags: SparseMemoryBindFlags) {
        self.raw.flags = flags.bits();
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


impl From<vks::VkSparseMemoryBind> for SparseMemoryBind {
    fn from(f: vks::VkSparseMemoryBind) -> SparseMemoryBind {
        SparseMemoryBind { raw: f, }
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

    pub fn get_resource_offset<'a>(&'a self) -> u64 {
        self.raw.resourceOffset.into()
    }

    pub fn get_size<'a>(&'a self) -> u64 {
        self.raw.size.into()
    }

    pub fn get_memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn get_memory_offset<'a>(&'a self) -> u64 {
        self.raw.memoryOffset.into()
    }

    pub fn get_flags<'a>(&'a self) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags::from_bits(self.raw.flags)
            .expect("SparseMemoryBind::flags: error converting flags")
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

    pub fn subresource<'a>(&'a self) -> ImageSubresource {
        self.raw.subresource.into()
    }

    pub fn offset<'a>(&'a self) -> Offset3d {
        self.raw.offset.into()
    }

    pub fn extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
    }

    pub fn memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn memory_offset<'a>(&'a self) -> u64 {
        self.raw.memoryOffset.into()
    }

    pub fn flags<'a>(&'a self) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags::from_bits(self.raw.flags)
            .expect("SparseImageMemoryBind::flags: error converting flags")
    }

    pub fn set_subresource<'m>(& mut self, subresource: ImageSubresource) {
        self.raw.subresource = subresource.raw;
    }

    pub fn set_offset<'m>(& mut self, offset: Offset3d) {
        self.raw.offset = offset.raw;
    }

    pub fn set_extent<'m>(& mut self, extent: Extent3d) {
        self.raw.extent = extent.raw;
    }

    pub fn set_memory<'m, 'a>(& mut self, memory: &'a DeviceMemory) {
        self.raw.memory = memory.handle();
    }

    pub fn set_memory_offset<'m>(& mut self, memory_offset: u64) {
        self.raw.memoryOffset = memory_offset.into();
    }

    pub fn set_flags<'m>(& mut self, flags: SparseMemoryBindFlags) {
        self.raw.flags = flags.bits();
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


impl From<vks::VkSparseImageMemoryBind> for SparseImageMemoryBind {
    fn from(f: vks::VkSparseImageMemoryBind) -> SparseImageMemoryBind {
        SparseImageMemoryBind { raw: f, }
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

    pub fn get_subresource<'a>(&'a self) -> ImageSubresource {
        self.raw.subresource.into()
    }

    pub fn get_offset<'a>(&'a self) -> Offset3d {
        self.raw.offset.into()
    }

    pub fn get_extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
    }

    pub fn get_memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn get_memory_offset<'a>(&'a self) -> u64 {
        self.raw.memoryOffset.into()
    }

    pub fn get_flags<'a>(&'a self) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags::from_bits(self.raw.flags)
            .expect("SparseImageMemoryBind::flags: error converting flags")
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

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn binds<'a>(&'a self) -> &'a [SparseMemoryBind] {
        unsafe { slice::from_raw_parts(self.raw.pBinds as *const _, self.raw.bindCount as usize) }
    }

    pub fn set_buffer<'m, 'a>(&'s mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
    }

    pub fn set_binds<'m, 'a>(&'s mut self, binds: &'a [SparseMemoryBind]) {
        assert!(self.raw.bindCount == 0 || self.raw.bindCount == binds.len() as _, 
            "count inconsistency found when specifying `SparseBufferMemoryBindInfo::binds`.");
        self.raw.bindCount = binds.len() as _;
        self.raw.pBinds = binds.as_ptr() as *const vks::VkSparseMemoryBind as *const _;
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


impl<'s> From<vks::VkSparseBufferMemoryBindInfo> for SparseBufferMemoryBindInfo<'s> {
    fn from(f: vks::VkSparseBufferMemoryBindInfo) -> SparseBufferMemoryBindInfo<'s> {
        SparseBufferMemoryBindInfo { raw: f, _p: PhantomData }
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

    pub fn binds<'m, 'a>(mut self, binds: &'a [SparseMemoryBind]) -> SparseBufferMemoryBindInfoBuilder<'b> {
        assert!(self.raw.bindCount == 0 || self.raw.bindCount == binds.len() as _, 
            "count inconsistency found when specifying `SparseBufferMemoryBindInfo::binds`.");
        self.raw.bindCount = binds.len() as _;
        self.raw.pBinds = binds.as_ptr() as *const vks::VkSparseMemoryBind as *const _;
        self
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn get_binds<'a>(&'a self) -> &'a [SparseMemoryBind] {
        unsafe { slice::from_raw_parts(self.raw.pBinds as *const _, self.raw.bindCount as usize) }
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

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn binds<'a>(&'a self) -> &'a [SparseMemoryBind] {
        unsafe { slice::from_raw_parts(self.raw.pBinds as *const _, self.raw.bindCount as usize) }
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
    }

    pub fn set_binds<'m, 'a>(&'s mut self, binds: &'a [SparseMemoryBind]) {
        assert!(self.raw.bindCount == 0 || self.raw.bindCount == binds.len() as _, 
            "count inconsistency found when specifying `SparseImageOpaqueMemoryBindInfo::binds`.");
        self.raw.bindCount = binds.len() as _;
        self.raw.pBinds = binds.as_ptr() as *const vks::VkSparseMemoryBind as *const _;
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


impl<'s> From<vks::VkSparseImageOpaqueMemoryBindInfo> for SparseImageOpaqueMemoryBindInfo<'s> {
    fn from(f: vks::VkSparseImageOpaqueMemoryBindInfo) -> SparseImageOpaqueMemoryBindInfo<'s> {
        SparseImageOpaqueMemoryBindInfo { raw: f, _p: PhantomData }
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

    pub fn binds<'m, 'a>(mut self, binds: &'a [SparseMemoryBind]) -> SparseImageOpaqueMemoryBindInfoBuilder<'b> {
        assert!(self.raw.bindCount == 0 || self.raw.bindCount == binds.len() as _, 
            "count inconsistency found when specifying `SparseImageOpaqueMemoryBindInfo::binds`.");
        self.raw.bindCount = binds.len() as _;
        self.raw.pBinds = binds.as_ptr() as *const vks::VkSparseMemoryBind as *const _;
        self
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn get_binds<'a>(&'a self) -> &'a [SparseMemoryBind] {
        unsafe { slice::from_raw_parts(self.raw.pBinds as *const _, self.raw.bindCount as usize) }
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

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn binds<'a>(&'a self) -> &'a [SparseImageMemoryBind] {
        unsafe { slice::from_raw_parts(self.raw.pBinds as *const _, self.raw.bindCount as usize) }
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
    }

    pub fn set_binds<'m, 'a>(&'s mut self, binds: &'a [SparseImageMemoryBind]) {
        assert!(self.raw.bindCount == 0 || self.raw.bindCount == binds.len() as _, 
            "count inconsistency found when specifying `SparseImageMemoryBindInfo::binds`.");
        self.raw.bindCount = binds.len() as _;
        self.raw.pBinds = binds.as_ptr() as *const vks::VkSparseImageMemoryBind as *const _;
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


impl<'s> From<vks::VkSparseImageMemoryBindInfo> for SparseImageMemoryBindInfo<'s> {
    fn from(f: vks::VkSparseImageMemoryBindInfo) -> SparseImageMemoryBindInfo<'s> {
        SparseImageMemoryBindInfo { raw: f, _p: PhantomData }
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

    pub fn binds<'m, 'a>(mut self, binds: &'a [SparseImageMemoryBind]) -> SparseImageMemoryBindInfoBuilder<'b> {
        assert!(self.raw.bindCount == 0 || self.raw.bindCount == binds.len() as _, 
            "count inconsistency found when specifying `SparseImageMemoryBindInfo::binds`.");
        self.raw.bindCount = binds.len() as _;
        self.raw.pBinds = binds.as_ptr() as *const vks::VkSparseImageMemoryBind as *const _;
        self
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn get_binds<'a>(&'a self) -> &'a [SparseImageMemoryBind] {
        unsafe { slice::from_raw_parts(self.raw.pBinds as *const _, self.raw.bindCount as usize) }
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
    signal_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    wait_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> BindSparseInfo<'s> {
    pub fn builder<'b>() -> BindSparseInfoBuilder<'b> {
        BindSparseInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn wait_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphores as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn buffer_binds<'a>(&'a self) -> &'a [SparseBufferMemoryBindInfo] {
        unsafe { slice::from_raw_parts(self.raw.pBufferBinds as *const _, self.raw.bufferBindCount as usize) }
    }

    pub fn image_opaque_binds<'a>(&'a self) -> &'a [SparseImageOpaqueMemoryBindInfo] {
        unsafe { slice::from_raw_parts(self.raw.pImageOpaqueBinds as *const _, self.raw.imageOpaqueBindCount as usize) }
    }

    pub fn image_binds<'a>(&'a self) -> &'a [SparseImageMemoryBindInfo] {
        unsafe { slice::from_raw_parts(self.raw.pImageBinds as *const _, self.raw.imageBindCount as usize) }
    }

    pub fn signal_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphores as *const _, self.raw.signalSemaphoreCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_wait_semaphores<'m, 'a>(&'s mut self, wait_semaphores: &'a [&'a Semaphore])
            where 'a: 's {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        {
            let wait_semaphores = self.wait_semaphores.as_ref().unwrap();
            self.raw.pWaitSemaphores = wait_semaphores.as_ptr();
            assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphores.len() as _, 
                "count inconsistency found when specifying `BindSparseInfo::wait_semaphores`.");
            self.raw.waitSemaphoreCount = wait_semaphores.len() as _;
        }
    }

    pub fn set_buffer_binds<'m, 'a>(&'s mut self, buffer_binds: &'a [SparseBufferMemoryBindInfo]) {
        assert!(self.raw.bufferBindCount == 0 || self.raw.bufferBindCount == buffer_binds.len() as _, 
            "count inconsistency found when specifying `BindSparseInfo::buffer_binds`.");
        self.raw.bufferBindCount = buffer_binds.len() as _;
        self.raw.pBufferBinds = buffer_binds.as_ptr() as *const vks::VkSparseBufferMemoryBindInfo as *const _;
    }

    pub fn set_image_opaque_binds<'m, 'a>(&'s mut self, image_opaque_binds: &'a [SparseImageOpaqueMemoryBindInfo]) {
        assert!(self.raw.imageOpaqueBindCount == 0 || self.raw.imageOpaqueBindCount == image_opaque_binds.len() as _, 
            "count inconsistency found when specifying `BindSparseInfo::image_opaque_binds`.");
        self.raw.imageOpaqueBindCount = image_opaque_binds.len() as _;
        self.raw.pImageOpaqueBinds = image_opaque_binds.as_ptr() as *const vks::VkSparseImageOpaqueMemoryBindInfo as *const _;
    }

    pub fn set_image_binds<'m, 'a>(&'s mut self, image_binds: &'a [SparseImageMemoryBindInfo]) {
        assert!(self.raw.imageBindCount == 0 || self.raw.imageBindCount == image_binds.len() as _, 
            "count inconsistency found when specifying `BindSparseInfo::image_binds`.");
        self.raw.imageBindCount = image_binds.len() as _;
        self.raw.pImageBinds = image_binds.as_ptr() as *const vks::VkSparseImageMemoryBindInfo as *const _;
    }

    pub fn set_signal_semaphores<'m, 'a>(&'s mut self, signal_semaphores: &'a [&'a Semaphore])
            where 'a: 's {
        self.signal_semaphores = Some(signal_semaphores.iter().map(|h| h.handle()).collect());
        {
            let signal_semaphores = self.signal_semaphores.as_ref().unwrap();
            self.raw.pSignalSemaphores = signal_semaphores.as_ptr();
            assert!(self.raw.signalSemaphoreCount == 0 || self.raw.signalSemaphoreCount == signal_semaphores.len() as _, 
                "count inconsistency found when specifying `BindSparseInfo::signal_semaphores`.");
            self.raw.signalSemaphoreCount = signal_semaphores.len() as _;
        }
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


impl<'s> From<vks::VkBindSparseInfo> for BindSparseInfo<'s> {
    fn from(f: vks::VkBindSparseInfo) -> BindSparseInfo<'s> {
        BindSparseInfo { raw: f, signal_semaphores: None, wait_semaphores: None, _p: PhantomData }
    }
}


/// A builder for `VkBindSparseInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct BindSparseInfoBuilder<'b> {
    raw: vks::VkBindSparseInfo,
    signal_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    wait_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    _p: PhantomData<&'b ()>, 
}

impl<'b> BindSparseInfoBuilder<'b> {
    pub fn new() -> BindSparseInfoBuilder<'b> {
        BindSparseInfoBuilder {
            raw: vks::VkBindSparseInfo::default(),
            signal_semaphores: None,
            wait_semaphores: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> BindSparseInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphores<'m, 'a>(mut self, wait_semaphores: &'a [&'a Semaphore]) -> BindSparseInfoBuilder<'b>
            where 'a: 'b {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        {
            let wait_semaphores = self.wait_semaphores.as_ref().unwrap();
            self.raw.pWaitSemaphores = wait_semaphores.as_ptr();
            assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphores.len() as _, 
                "count inconsistency found when specifying `BindSparseInfo::wait_semaphores`.");
            self.raw.waitSemaphoreCount = wait_semaphores.len() as _;
        }
        self
    }

    pub fn buffer_binds<'m, 'a>(mut self, buffer_binds: &'a [SparseBufferMemoryBindInfo]) -> BindSparseInfoBuilder<'b> {
        assert!(self.raw.bufferBindCount == 0 || self.raw.bufferBindCount == buffer_binds.len() as _, 
            "count inconsistency found when specifying `BindSparseInfo::buffer_binds`.");
        self.raw.bufferBindCount = buffer_binds.len() as _;
        self.raw.pBufferBinds = buffer_binds.as_ptr() as *const vks::VkSparseBufferMemoryBindInfo as *const _;
        self
    }

    pub fn image_opaque_binds<'m, 'a>(mut self, image_opaque_binds: &'a [SparseImageOpaqueMemoryBindInfo]) -> BindSparseInfoBuilder<'b> {
        assert!(self.raw.imageOpaqueBindCount == 0 || self.raw.imageOpaqueBindCount == image_opaque_binds.len() as _, 
            "count inconsistency found when specifying `BindSparseInfo::image_opaque_binds`.");
        self.raw.imageOpaqueBindCount = image_opaque_binds.len() as _;
        self.raw.pImageOpaqueBinds = image_opaque_binds.as_ptr() as *const vks::VkSparseImageOpaqueMemoryBindInfo as *const _;
        self
    }

    pub fn image_binds<'m, 'a>(mut self, image_binds: &'a [SparseImageMemoryBindInfo]) -> BindSparseInfoBuilder<'b> {
        assert!(self.raw.imageBindCount == 0 || self.raw.imageBindCount == image_binds.len() as _, 
            "count inconsistency found when specifying `BindSparseInfo::image_binds`.");
        self.raw.imageBindCount = image_binds.len() as _;
        self.raw.pImageBinds = image_binds.as_ptr() as *const vks::VkSparseImageMemoryBindInfo as *const _;
        self
    }

    pub fn signal_semaphores<'m, 'a>(mut self, signal_semaphores: &'a [&'a Semaphore]) -> BindSparseInfoBuilder<'b>
            where 'a: 'b {
        self.signal_semaphores = Some(signal_semaphores.iter().map(|h| h.handle()).collect());
        {
            let signal_semaphores = self.signal_semaphores.as_ref().unwrap();
            self.raw.pSignalSemaphores = signal_semaphores.as_ptr();
            assert!(self.raw.signalSemaphoreCount == 0 || self.raw.signalSemaphoreCount == signal_semaphores.len() as _, 
                "count inconsistency found when specifying `BindSparseInfo::signal_semaphores`.");
            self.raw.signalSemaphoreCount = signal_semaphores.len() as _;
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_wait_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphores as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn get_buffer_binds<'a>(&'a self) -> &'a [SparseBufferMemoryBindInfo] {
        unsafe { slice::from_raw_parts(self.raw.pBufferBinds as *const _, self.raw.bufferBindCount as usize) }
    }

    pub fn get_image_opaque_binds<'a>(&'a self) -> &'a [SparseImageOpaqueMemoryBindInfo] {
        unsafe { slice::from_raw_parts(self.raw.pImageOpaqueBinds as *const _, self.raw.imageOpaqueBindCount as usize) }
    }

    pub fn get_image_binds<'a>(&'a self) -> &'a [SparseImageMemoryBindInfo] {
        unsafe { slice::from_raw_parts(self.raw.pImageBinds as *const _, self.raw.imageBindCount as usize) }
    }

    pub fn get_signal_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphores as *const _, self.raw.signalSemaphoreCount as usize) }
    }

    pub fn build(self) -> BindSparseInfo<'b> {
        BindSparseInfo {
            raw: self.raw,
            signal_semaphores: self.signal_semaphores,
            wait_semaphores: self.wait_semaphores,
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

    pub fn src_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.srcSubresource.into()
    }

    pub fn src_offset<'a>(&'a self) -> Offset3d {
        self.raw.srcOffset.into()
    }

    pub fn dst_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.dstSubresource.into()
    }

    pub fn dst_offset<'a>(&'a self) -> Offset3d {
        self.raw.dstOffset.into()
    }

    pub fn extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
    }

    pub fn set_src_subresource<'m>(& mut self, src_subresource: ImageSubresourceLayers) {
        self.raw.srcSubresource = src_subresource.raw;
    }

    pub fn set_src_offset<'m>(& mut self, src_offset: Offset3d) {
        self.raw.srcOffset = src_offset.raw;
    }

    pub fn set_dst_subresource<'m>(& mut self, dst_subresource: ImageSubresourceLayers) {
        self.raw.dstSubresource = dst_subresource.raw;
    }

    pub fn set_dst_offset<'m>(& mut self, dst_offset: Offset3d) {
        self.raw.dstOffset = dst_offset.raw;
    }

    pub fn set_extent<'m>(& mut self, extent: Extent3d) {
        self.raw.extent = extent.raw;
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


impl From<vks::VkImageCopy> for ImageCopy {
    fn from(f: vks::VkImageCopy) -> ImageCopy {
        ImageCopy { raw: f, }
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

    pub fn get_src_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.srcSubresource.into()
    }

    pub fn get_src_offset<'a>(&'a self) -> Offset3d {
        self.raw.srcOffset.into()
    }

    pub fn get_dst_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.dstSubresource.into()
    }

    pub fn get_dst_offset<'a>(&'a self) -> Offset3d {
        self.raw.dstOffset.into()
    }

    pub fn get_extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
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

    pub fn src_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.srcSubresource.into()
    }

    pub fn src_offsets<'a>(&'a self) -> &[Offset3d] {
        unsafe { slice::from_raw_parts(&self.raw.srcOffsets as *const vks::VkOffset3D as *const _, 2 as usize) }
    }

    pub fn dst_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.dstSubresource.into()
    }

    pub fn dst_offsets<'a>(&'a self) -> &[Offset3d] {
        unsafe { slice::from_raw_parts(&self.raw.dstOffsets as *const vks::VkOffset3D as *const _, 2 as usize) }
    }

    pub fn set_src_subresource<'m>(& mut self, src_subresource: ImageSubresourceLayers) {
        self.raw.srcSubresource = src_subresource.raw;
    }

    pub fn set_src_offsets<'m>(& mut self, src_offsets: [Offset3d; 2]) {
        self.raw.srcOffsets = [src_offsets[0].raw, src_offsets[1].raw, ];
    }

    pub fn set_dst_subresource<'m>(& mut self, dst_subresource: ImageSubresourceLayers) {
        self.raw.dstSubresource = dst_subresource.raw;
    }

    pub fn set_dst_offsets<'m>(& mut self, dst_offsets: [Offset3d; 2]) {
        self.raw.dstOffsets = [dst_offsets[0].raw, dst_offsets[1].raw, ];
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


impl From<vks::VkImageBlit> for ImageBlit {
    fn from(f: vks::VkImageBlit) -> ImageBlit {
        ImageBlit { raw: f, }
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

    pub fn get_src_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.srcSubresource.into()
    }

    pub fn get_src_offsets<'a>(&'a self) -> &[Offset3d] {
        unsafe { slice::from_raw_parts(&self.raw.srcOffsets as *const vks::VkOffset3D as *const _, 2 as usize) }
    }

    pub fn get_dst_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.dstSubresource.into()
    }

    pub fn get_dst_offsets<'a>(&'a self) -> &[Offset3d] {
        unsafe { slice::from_raw_parts(&self.raw.dstOffsets as *const vks::VkOffset3D as *const _, 2 as usize) }
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

    pub fn buffer_offset<'a>(&'a self) -> u64 {
        self.raw.bufferOffset.into()
    }

    pub fn buffer_row_length<'a>(&'a self) -> u32 {
        self.raw.bufferRowLength.into()
    }

    pub fn buffer_image_height<'a>(&'a self) -> u32 {
        self.raw.bufferImageHeight.into()
    }

    pub fn image_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.imageSubresource.into()
    }

    pub fn image_offset<'a>(&'a self) -> Offset3d {
        self.raw.imageOffset.into()
    }

    pub fn image_extent<'a>(&'a self) -> Extent3d {
        self.raw.imageExtent.into()
    }

    pub fn set_buffer_offset<'m>(& mut self, buffer_offset: u64) {
        self.raw.bufferOffset = buffer_offset.into();
    }

    pub fn set_buffer_row_length<'m>(& mut self, buffer_row_length: u32) {
        self.raw.bufferRowLength = buffer_row_length.into();
    }

    pub fn set_buffer_image_height<'m>(& mut self, buffer_image_height: u32) {
        self.raw.bufferImageHeight = buffer_image_height.into();
    }

    pub fn set_image_subresource<'m>(& mut self, image_subresource: ImageSubresourceLayers) {
        self.raw.imageSubresource = image_subresource.raw;
    }

    pub fn set_image_offset<'m>(& mut self, image_offset: Offset3d) {
        self.raw.imageOffset = image_offset.raw;
    }

    pub fn set_image_extent<'m>(& mut self, image_extent: Extent3d) {
        self.raw.imageExtent = image_extent.raw;
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


impl From<vks::VkBufferImageCopy> for BufferImageCopy {
    fn from(f: vks::VkBufferImageCopy) -> BufferImageCopy {
        BufferImageCopy { raw: f, }
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

    pub fn get_buffer_offset<'a>(&'a self) -> u64 {
        self.raw.bufferOffset.into()
    }

    pub fn get_buffer_row_length<'a>(&'a self) -> u32 {
        self.raw.bufferRowLength.into()
    }

    pub fn get_buffer_image_height<'a>(&'a self) -> u32 {
        self.raw.bufferImageHeight.into()
    }

    pub fn get_image_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.imageSubresource.into()
    }

    pub fn get_image_offset<'a>(&'a self) -> Offset3d {
        self.raw.imageOffset.into()
    }

    pub fn get_image_extent<'a>(&'a self) -> Extent3d {
        self.raw.imageExtent.into()
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

    pub fn src_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.srcSubresource.into()
    }

    pub fn src_offset<'a>(&'a self) -> Offset3d {
        self.raw.srcOffset.into()
    }

    pub fn dst_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.dstSubresource.into()
    }

    pub fn dst_offset<'a>(&'a self) -> Offset3d {
        self.raw.dstOffset.into()
    }

    pub fn extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
    }

    pub fn set_src_subresource<'m>(& mut self, src_subresource: ImageSubresourceLayers) {
        self.raw.srcSubresource = src_subresource.raw;
    }

    pub fn set_src_offset<'m>(& mut self, src_offset: Offset3d) {
        self.raw.srcOffset = src_offset.raw;
    }

    pub fn set_dst_subresource<'m>(& mut self, dst_subresource: ImageSubresourceLayers) {
        self.raw.dstSubresource = dst_subresource.raw;
    }

    pub fn set_dst_offset<'m>(& mut self, dst_offset: Offset3d) {
        self.raw.dstOffset = dst_offset.raw;
    }

    pub fn set_extent<'m>(& mut self, extent: Extent3d) {
        self.raw.extent = extent.raw;
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


impl From<vks::VkImageResolve> for ImageResolve {
    fn from(f: vks::VkImageResolve) -> ImageResolve {
        ImageResolve { raw: f, }
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

    pub fn get_src_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.srcSubresource.into()
    }

    pub fn get_src_offset<'a>(&'a self) -> Offset3d {
        self.raw.srcOffset.into()
    }

    pub fn get_dst_subresource<'a>(&'a self) -> ImageSubresourceLayers {
        self.raw.dstSubresource.into()
    }

    pub fn get_dst_offset<'a>(&'a self) -> Offset3d {
        self.raw.dstOffset.into()
    }

    pub fn get_extent<'a>(&'a self) -> Extent3d {
        self.raw.extent.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> ShaderModuleCreateFlags {
        ShaderModuleCreateFlags::from_bits(self.raw.flags)
            .expect("ShaderModuleCreateInfo::flags: error converting flags")
    }

    pub fn code<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pCode as *const _, self.raw.codeSize as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: ShaderModuleCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_code<'m, 'a>(&'s mut self, code: &'a [u32]) {
        assert!(self.raw.codeSize == 0 || self.raw.codeSize == code.len() as _, 
            "count inconsistency found when specifying `ShaderModuleCreateInfo::code`.");
        self.raw.codeSize = code.len() as _;
        self.raw.pCode = code.as_ptr() as *const u32 as *const _;
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


impl<'s> From<vks::VkShaderModuleCreateInfo> for ShaderModuleCreateInfo<'s> {
    fn from(f: vks::VkShaderModuleCreateInfo) -> ShaderModuleCreateInfo<'s> {
        ShaderModuleCreateInfo { raw: f, _p: PhantomData }
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

    pub fn code<'m, 'a>(mut self, code: &'a [u32]) -> ShaderModuleCreateInfoBuilder<'b> {
        assert!(self.raw.codeSize == 0 || self.raw.codeSize == code.len() as _, 
            "count inconsistency found when specifying `ShaderModuleCreateInfo::code`.");
        self.raw.codeSize = code.len() as _;
        self.raw.pCode = code.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> ShaderModuleCreateFlags {
        ShaderModuleCreateFlags::from_bits(self.raw.flags)
            .expect("ShaderModuleCreateInfo::flags: error converting flags")
    }

    pub fn get_code<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pCode as *const _, self.raw.codeSize as usize) }
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
    immutable_samplers: Option<SmallVec<[vks::VkSampler; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorSetLayoutBinding<'s> {
    pub fn builder<'b>() -> DescriptorSetLayoutBindingBuilder<'b> {
        DescriptorSetLayoutBindingBuilder::new()
    }

    pub fn binding<'a>(&'a self) -> u32 {
        self.raw.binding.into()
    }

    pub fn descriptor_type<'a>(&'a self) -> DescriptorType {
        self.raw.descriptorType.into()
    }

    pub fn descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn stage_flags<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stageFlags)
            .expect("DescriptorSetLayoutBinding::stage_flags: error converting flags")
    }

    pub fn immutable_samplers_handle<'a>(&'a self) -> &'a vks::VkSampler {
        unsafe { &*(self.raw.pImmutableSamplers as *const _) }
    }

    pub fn set_binding<'m>(&'s mut self, binding: u32) {
        self.raw.binding = binding.into();
    }

    pub fn set_descriptor_type<'m>(&'s mut self, descriptor_type: DescriptorType) {
        self.raw.descriptorType = descriptor_type.into();
    }

    pub fn set_descriptor_count<'m>(&'s mut self, descriptor_count: u32) {
        self.raw.descriptorCount = descriptor_count.into();
    }

    pub fn set_stage_flags<'m>(&'s mut self, stage_flags: ShaderStageFlags) {
        self.raw.stageFlags = stage_flags.bits();
    }

    pub fn set_immutable_samplers<'m, 'a>(&'s mut self, immutable_samplers: &'a [&'a Sampler])
            where 'a: 's {
        self.immutable_samplers = Some(immutable_samplers.iter().map(|h| h.handle()).collect());
        {
            let immutable_samplers = self.immutable_samplers.as_ref().unwrap();
            self.raw.pImmutableSamplers = immutable_samplers.as_ptr();
        }
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


impl<'s> From<vks::VkDescriptorSetLayoutBinding> for DescriptorSetLayoutBinding<'s> {
    fn from(f: vks::VkDescriptorSetLayoutBinding) -> DescriptorSetLayoutBinding<'s> {
        DescriptorSetLayoutBinding { raw: f, immutable_samplers: None, _p: PhantomData }
    }
}


/// A builder for `VkDescriptorSetLayoutBinding`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutBindingBuilder<'b> {
    raw: vks::VkDescriptorSetLayoutBinding,
    immutable_samplers: Option<SmallVec<[vks::VkSampler; 8]>>,
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

    pub fn immutable_samplers<'m, 'a>(mut self, immutable_samplers: &'a [&'a Sampler]) -> DescriptorSetLayoutBindingBuilder<'b>
            where 'a: 'b {
        self.immutable_samplers = Some(immutable_samplers.iter().map(|h| h.handle()).collect());
        {
            let immutable_samplers = self.immutable_samplers.as_ref().unwrap();
            self.raw.pImmutableSamplers = immutable_samplers.as_ptr();
        }
        self
    }

    pub fn get_binding<'a>(&'a self) -> u32 {
        self.raw.binding.into()
    }

    pub fn get_descriptor_type<'a>(&'a self) -> DescriptorType {
        self.raw.descriptorType.into()
    }

    pub fn get_descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn get_stage_flags<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stageFlags)
            .expect("DescriptorSetLayoutBinding::stage_flags: error converting flags")
    }

    pub fn get_immutable_samplers_handle<'a>(&'a self) -> &'a vks::VkSampler {
        unsafe { &*(self.raw.pImmutableSamplers as *const _) }
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
    bindings: Option<SmallVec<[vks::VkDescriptorSetLayoutBinding; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorSetLayoutCreateInfo<'s> {
    pub fn builder<'b>() -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        DescriptorSetLayoutCreateInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DescriptorSetLayoutCreateFlags {
        DescriptorSetLayoutCreateFlags::from_bits(self.raw.flags)
            .expect("DescriptorSetLayoutCreateInfo::flags: error converting flags")
    }

    pub fn bindings<'a>(&'a self) -> &'a [vks::VkDescriptorSetLayoutBinding] {
        unsafe { slice::from_raw_parts(self.raw.pBindings as *const _, self.raw.bindingCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DescriptorSetLayoutCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_bindings<'m, 'a>(&'s mut self, bindings: &'a [DescriptorSetLayoutBinding]) {
        self.bindings = Some(bindings.iter().map(|h| h.raw).collect());
        {
            let bindings = self.bindings.as_ref().unwrap();
            self.raw.pBindings = bindings.as_ptr();
            assert!(self.raw.bindingCount == 0 || self.raw.bindingCount == bindings.len() as _, 
                "count inconsistency found when specifying `DescriptorSetLayoutCreateInfo::bindings`.");
            self.raw.bindingCount = bindings.len() as _;
        }
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


impl<'s> From<vks::VkDescriptorSetLayoutCreateInfo> for DescriptorSetLayoutCreateInfo<'s> {
    fn from(f: vks::VkDescriptorSetLayoutCreateInfo) -> DescriptorSetLayoutCreateInfo<'s> {
        DescriptorSetLayoutCreateInfo { raw: f, bindings: None, _p: PhantomData }
    }
}


/// A builder for `VkDescriptorSetLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'b> {
    raw: vks::VkDescriptorSetLayoutCreateInfo,
    bindings: Option<SmallVec<[vks::VkDescriptorSetLayoutBinding; 8]>>,
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

    pub fn bindings<'m, 'a>(mut self, bindings: &'a [DescriptorSetLayoutBinding]) -> DescriptorSetLayoutCreateInfoBuilder<'b> {
        self.bindings = Some(bindings.iter().map(|h| h.raw).collect());
        {
            let bindings = self.bindings.as_ref().unwrap();
            self.raw.pBindings = bindings.as_ptr();
            assert!(self.raw.bindingCount == 0 || self.raw.bindingCount == bindings.len() as _, 
                "count inconsistency found when specifying `DescriptorSetLayoutCreateInfo::bindings`.");
            self.raw.bindingCount = bindings.len() as _;
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DescriptorSetLayoutCreateFlags {
        DescriptorSetLayoutCreateFlags::from_bits(self.raw.flags)
            .expect("DescriptorSetLayoutCreateInfo::flags: error converting flags")
    }

    pub fn get_bindings<'a>(&'a self) -> &'a [vks::VkDescriptorSetLayoutBinding] {
        unsafe { slice::from_raw_parts(self.raw.pBindings as *const _, self.raw.bindingCount as usize) }
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

    pub fn type_of<'a>(&'a self) -> DescriptorType {
        self.raw.type_.into()
    }

    pub fn descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn set_type_of<'m>(& mut self, type_of: DescriptorType) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_descriptor_count<'m>(& mut self, descriptor_count: u32) {
        self.raw.descriptorCount = descriptor_count.into();
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


impl From<vks::VkDescriptorPoolSize> for DescriptorPoolSize {
    fn from(f: vks::VkDescriptorPoolSize) -> DescriptorPoolSize {
        DescriptorPoolSize { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> DescriptorType {
        self.raw.type_.into()
    }

    pub fn get_descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DescriptorPoolCreateFlags {
        DescriptorPoolCreateFlags::from_bits(self.raw.flags)
            .expect("DescriptorPoolCreateInfo::flags: error converting flags")
    }

    pub fn max_sets<'a>(&'a self) -> u32 {
        self.raw.maxSets.into()
    }

    pub fn pool_sizes<'a>(&'a self) -> &'a [DescriptorPoolSize] {
        unsafe { slice::from_raw_parts(self.raw.pPoolSizes as *const _, self.raw.poolSizeCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DescriptorPoolCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_max_sets<'m>(&'s mut self, max_sets: u32) {
        self.raw.maxSets = max_sets.into();
    }

    pub fn set_pool_sizes<'m, 'a>(&'s mut self, pool_sizes: &'a [DescriptorPoolSize]) {
        assert!(self.raw.poolSizeCount == 0 || self.raw.poolSizeCount == pool_sizes.len() as _, 
            "count inconsistency found when specifying `DescriptorPoolCreateInfo::pool_sizes`.");
        self.raw.poolSizeCount = pool_sizes.len() as _;
        self.raw.pPoolSizes = pool_sizes.as_ptr() as *const vks::VkDescriptorPoolSize as *const _;
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


impl<'s> From<vks::VkDescriptorPoolCreateInfo> for DescriptorPoolCreateInfo<'s> {
    fn from(f: vks::VkDescriptorPoolCreateInfo) -> DescriptorPoolCreateInfo<'s> {
        DescriptorPoolCreateInfo { raw: f, _p: PhantomData }
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

    pub fn pool_sizes<'m, 'a>(mut self, pool_sizes: &'a [DescriptorPoolSize]) -> DescriptorPoolCreateInfoBuilder<'b> {
        assert!(self.raw.poolSizeCount == 0 || self.raw.poolSizeCount == pool_sizes.len() as _, 
            "count inconsistency found when specifying `DescriptorPoolCreateInfo::pool_sizes`.");
        self.raw.poolSizeCount = pool_sizes.len() as _;
        self.raw.pPoolSizes = pool_sizes.as_ptr() as *const vks::VkDescriptorPoolSize as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DescriptorPoolCreateFlags {
        DescriptorPoolCreateFlags::from_bits(self.raw.flags)
            .expect("DescriptorPoolCreateInfo::flags: error converting flags")
    }

    pub fn get_max_sets<'a>(&'a self) -> u32 {
        self.raw.maxSets.into()
    }

    pub fn get_pool_sizes<'a>(&'a self) -> &'a [DescriptorPoolSize] {
        unsafe { slice::from_raw_parts(self.raw.pPoolSizes as *const _, self.raw.poolSizeCount as usize) }
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
    set_layouts: Option<SmallVec<[vks::VkDescriptorSetLayout; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> DescriptorSetAllocateInfo<'s> {
    pub fn builder<'b>() -> DescriptorSetAllocateInfoBuilder<'b> {
        DescriptorSetAllocateInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn descriptor_pool_handle<'a>(&'a self) -> vks::VkDescriptorPool {
        self.raw.descriptorPool
    }

    pub fn set_layouts_handle<'a>(&'a self) -> &'a [vks::VkDescriptorSetLayout] {
        unsafe { slice::from_raw_parts(self.raw.pSetLayouts as *const _, self.raw.descriptorSetCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_descriptor_pool<'m, 'a>(&'s mut self, descriptor_pool: &'a DescriptorPool) {
        self.raw.descriptorPool = descriptor_pool.handle();
    }

    pub fn set_set_layouts<'m, 'a>(&'s mut self, set_layouts: &'a [&'a DescriptorSetLayout])
            where 'a: 's {
        self.set_layouts = Some(set_layouts.iter().map(|h| h.handle()).collect());
        {
            let set_layouts = self.set_layouts.as_ref().unwrap();
            self.raw.pSetLayouts = set_layouts.as_ptr();
            assert!(self.raw.descriptorSetCount == 0 || self.raw.descriptorSetCount == set_layouts.len() as _, 
                "count inconsistency found when specifying `DescriptorSetAllocateInfo::set_layouts`.");
            self.raw.descriptorSetCount = set_layouts.len() as _;
        }
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


impl<'s> From<vks::VkDescriptorSetAllocateInfo> for DescriptorSetAllocateInfo<'s> {
    fn from(f: vks::VkDescriptorSetAllocateInfo) -> DescriptorSetAllocateInfo<'s> {
        DescriptorSetAllocateInfo { raw: f, set_layouts: None, _p: PhantomData }
    }
}


/// A builder for `VkDescriptorSetAllocateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct DescriptorSetAllocateInfoBuilder<'b> {
    raw: vks::VkDescriptorSetAllocateInfo,
    set_layouts: Option<SmallVec<[vks::VkDescriptorSetLayout; 8]>>,
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

    pub fn set_layouts<'m, 'a>(mut self, set_layouts: &'a [&'a DescriptorSetLayout]) -> DescriptorSetAllocateInfoBuilder<'b>
            where 'a: 'b {
        self.set_layouts = Some(set_layouts.iter().map(|h| h.handle()).collect());
        {
            let set_layouts = self.set_layouts.as_ref().unwrap();
            self.raw.pSetLayouts = set_layouts.as_ptr();
            assert!(self.raw.descriptorSetCount == 0 || self.raw.descriptorSetCount == set_layouts.len() as _, 
                "count inconsistency found when specifying `DescriptorSetAllocateInfo::set_layouts`.");
            self.raw.descriptorSetCount = set_layouts.len() as _;
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_descriptor_pool_handle<'a>(&'a self) -> vks::VkDescriptorPool {
        self.raw.descriptorPool
    }

    pub fn get_set_layouts_handle<'a>(&'a self) -> &'a [vks::VkDescriptorSetLayout] {
        unsafe { slice::from_raw_parts(self.raw.pSetLayouts as *const _, self.raw.descriptorSetCount as usize) }
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

    pub fn constant_id<'a>(&'a self) -> u32 {
        self.raw.constantID.into()
    }

    pub fn offset<'a>(&'a self) -> u32 {
        self.raw.offset.into()
    }

    pub fn size<'a>(&'a self) -> usize {
        self.raw.size.into()
    }

    pub fn set_constant_id<'m>(& mut self, constant_id: u32) {
        self.raw.constantID = constant_id.into();
    }

    pub fn set_offset<'m>(& mut self, offset: u32) {
        self.raw.offset = offset.into();
    }

    pub fn set_size<'m>(& mut self, size: usize) {
        self.raw.size = size.into();
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


impl From<vks::VkSpecializationMapEntry> for SpecializationMapEntry {
    fn from(f: vks::VkSpecializationMapEntry) -> SpecializationMapEntry {
        SpecializationMapEntry { raw: f, }
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

    pub fn get_constant_id<'a>(&'a self) -> u32 {
        self.raw.constantID.into()
    }

    pub fn get_offset<'a>(&'a self) -> u32 {
        self.raw.offset.into()
    }

    pub fn get_size<'a>(&'a self) -> usize {
        self.raw.size.into()
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

    pub fn map_entries<'a>(&'a self) -> &'a [SpecializationMapEntry] {
        unsafe { slice::from_raw_parts(self.raw.pMapEntries as *const _, self.raw.mapEntryCount as usize) }
    }

    pub fn data_size<'a>(&'a self) -> usize {
        self.raw.dataSize.into()
    }

    pub fn data<'a>(&'a self) -> *const c_void {
        self.raw.pData
    }

    pub fn set_map_entries<'m, 'a>(&'s mut self, map_entries: &'a [SpecializationMapEntry]) {
        assert!(self.raw.mapEntryCount == 0 || self.raw.mapEntryCount == map_entries.len() as _, 
            "count inconsistency found when specifying `SpecializationInfo::map_entries`.");
        self.raw.mapEntryCount = map_entries.len() as _;
        self.raw.pMapEntries = map_entries.as_ptr() as *const vks::VkSpecializationMapEntry as *const _;
    }

    pub fn set_data_size<'m>(&'s mut self, data_size: usize) {
        self.raw.dataSize = data_size.into();
    }

    pub unsafe fn set_data<'m>(&'s mut self, data: *const c_void) {
        self.raw.pData = data;
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


impl<'s> From<vks::VkSpecializationInfo> for SpecializationInfo<'s> {
    fn from(f: vks::VkSpecializationInfo) -> SpecializationInfo<'s> {
        SpecializationInfo { raw: f, _p: PhantomData }
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

    pub fn map_entries<'m, 'a>(mut self, map_entries: &'a [SpecializationMapEntry]) -> SpecializationInfoBuilder<'b> {
        assert!(self.raw.mapEntryCount == 0 || self.raw.mapEntryCount == map_entries.len() as _, 
            "count inconsistency found when specifying `SpecializationInfo::map_entries`.");
        self.raw.mapEntryCount = map_entries.len() as _;
        self.raw.pMapEntries = map_entries.as_ptr() as *const vks::VkSpecializationMapEntry as *const _;
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

    pub fn get_map_entries<'a>(&'a self) -> &'a [SpecializationMapEntry] {
        unsafe { slice::from_raw_parts(self.raw.pMapEntries as *const _, self.raw.mapEntryCount as usize) }
    }

    pub fn get_data_size<'a>(&'a self) -> usize {
        self.raw.dataSize.into()
    }

    pub fn get_data<'a>(&'a self) -> *const c_void {
        self.raw.pData
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineShaderStageCreateFlags {
        PipelineShaderStageCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineShaderStageCreateInfo::flags: error converting flags")
    }

    pub fn stage<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stage)
            .expect("PipelineShaderStageCreateInfo::stage: error converting flags")
    }

    pub fn module_handle<'a>(&'a self) -> vks::VkShaderModule {
        self.raw.module
    }

    pub fn name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pName) }
    }

    pub fn specialization_info<'a>(&'a self) -> &'a SpecializationInfo {
        unsafe { &*(self.raw.pSpecializationInfo as *const vks::VkSpecializationInfo as *const _) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineShaderStageCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_stage<'m>(&'s mut self, stage: ShaderStageFlags) {
        self.raw.stage = stage.bits();
    }

    pub fn set_module<'m, 'a>(&'s mut self, module: &'a ShaderModule) {
        self.raw.module = module.handle();
    }

    pub fn set_name<'m, 'a, T>(&'s mut self, name: T)
            where 'a: 's, T: Into<CharStr<'a>> {
        self.name = Some(name.into());
        {
            let name = self.name.as_ref().unwrap();
            self.raw.pName = name.as_ptr();
        }
    }

    pub fn set_specialization_info<'m, 'a>(&'s mut self, specialization_info: &'a SpecializationInfo) {
        self.raw.pSpecializationInfo = specialization_info.raw();
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


impl<'s> From<vks::VkPipelineShaderStageCreateInfo> for PipelineShaderStageCreateInfo<'s> {
    fn from(f: vks::VkPipelineShaderStageCreateInfo) -> PipelineShaderStageCreateInfo<'s> {
        PipelineShaderStageCreateInfo { raw: f, name: None, _p: PhantomData }
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

    pub fn name<'m, 'a, T>(mut self, name: T) -> PipelineShaderStageCreateInfoBuilder<'b>
            where 'a: 'b, T: Into<CharStr<'a>> {
        self.name = Some(name.into());
        {
            let name = self.name.as_ref().unwrap();
            self.raw.pName = name.as_ptr();
        }
        self
    }

    pub fn specialization_info<'m, 'a>(mut self, specialization_info: &'a SpecializationInfo) -> PipelineShaderStageCreateInfoBuilder<'b> {
        self.raw.pSpecializationInfo = specialization_info.raw();
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineShaderStageCreateFlags {
        PipelineShaderStageCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineShaderStageCreateInfo::flags: error converting flags")
    }

    pub fn get_stage<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stage)
            .expect("PipelineShaderStageCreateInfo::stage: error converting flags")
    }

    pub fn get_module_handle<'a>(&'a self) -> vks::VkShaderModule {
        self.raw.module
    }

    pub fn get_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pName) }
    }

    pub fn get_specialization_info<'a>(&'a self) -> &'a SpecializationInfo {
        unsafe { &*(self.raw.pSpecializationInfo as *const vks::VkSpecializationInfo as *const _) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineCreateFlags {
        PipelineCreateFlags::from_bits(self.raw.flags)
            .expect("ComputePipelineCreateInfo::flags: error converting flags")
    }

    pub fn stage<'a>(&'a self) -> PipelineShaderStageCreateInfo {
        self.raw.stage.into()
    }

    pub fn layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.layout
    }

    pub fn base_pipeline_handle_handle<'a>(&'a self) -> vks::VkPipeline {
        self.raw.basePipelineHandle
    }

    pub fn base_pipeline_index<'a>(&'a self) -> i32 {
        self.raw.basePipelineIndex.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_stage<'m>(&'s mut self, stage: PipelineShaderStageCreateInfo) {
        self.raw.stage = stage.raw;
    }

    pub fn set_layout<'m, 'a>(&'s mut self, layout: &'a PipelineLayout) {
        self.raw.layout = layout.handle();
    }

    pub fn set_base_pipeline_handle<'m, 'a>(&'s mut self, base_pipeline_handle: &'a Pipeline) {
        self.raw.basePipelineHandle = base_pipeline_handle.handle();
    }

    pub fn set_base_pipeline_index<'m>(&'s mut self, base_pipeline_index: i32) {
        self.raw.basePipelineIndex = base_pipeline_index.into();
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


impl<'s> From<vks::VkComputePipelineCreateInfo> for ComputePipelineCreateInfo<'s> {
    fn from(f: vks::VkComputePipelineCreateInfo) -> ComputePipelineCreateInfo<'s> {
        ComputePipelineCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineCreateFlags {
        PipelineCreateFlags::from_bits(self.raw.flags)
            .expect("ComputePipelineCreateInfo::flags: error converting flags")
    }

    pub fn get_stage<'a>(&'a self) -> PipelineShaderStageCreateInfo {
        self.raw.stage.into()
    }

    pub fn get_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.layout
    }

    pub fn get_base_pipeline_handle_handle<'a>(&'a self) -> vks::VkPipeline {
        self.raw.basePipelineHandle
    }

    pub fn get_base_pipeline_index<'a>(&'a self) -> i32 {
        self.raw.basePipelineIndex.into()
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

    pub fn binding<'a>(&'a self) -> u32 {
        self.raw.binding.into()
    }

    pub fn stride<'a>(&'a self) -> u32 {
        self.raw.stride.into()
    }

    pub fn input_rate<'a>(&'a self) -> VertexInputRate {
        self.raw.inputRate.into()
    }

    pub fn set_binding<'m>(& mut self, binding: u32) {
        self.raw.binding = binding.into();
    }

    pub fn set_stride<'m>(& mut self, stride: u32) {
        self.raw.stride = stride.into();
    }

    pub fn set_input_rate<'m>(& mut self, input_rate: VertexInputRate) {
        self.raw.inputRate = input_rate.into();
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


impl From<vks::VkVertexInputBindingDescription> for VertexInputBindingDescription {
    fn from(f: vks::VkVertexInputBindingDescription) -> VertexInputBindingDescription {
        VertexInputBindingDescription { raw: f, }
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

    pub fn get_binding<'a>(&'a self) -> u32 {
        self.raw.binding.into()
    }

    pub fn get_stride<'a>(&'a self) -> u32 {
        self.raw.stride.into()
    }

    pub fn get_input_rate<'a>(&'a self) -> VertexInputRate {
        self.raw.inputRate.into()
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

    pub fn location<'a>(&'a self) -> u32 {
        self.raw.location.into()
    }

    pub fn binding<'a>(&'a self) -> u32 {
        self.raw.binding.into()
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn offset<'a>(&'a self) -> u32 {
        self.raw.offset.into()
    }

    pub fn set_location<'m>(& mut self, location: u32) {
        self.raw.location = location.into();
    }

    pub fn set_binding<'m>(& mut self, binding: u32) {
        self.raw.binding = binding.into();
    }

    pub fn set_format<'m>(& mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_offset<'m>(& mut self, offset: u32) {
        self.raw.offset = offset.into();
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


impl From<vks::VkVertexInputAttributeDescription> for VertexInputAttributeDescription {
    fn from(f: vks::VkVertexInputAttributeDescription) -> VertexInputAttributeDescription {
        VertexInputAttributeDescription { raw: f, }
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

    pub fn get_location<'a>(&'a self) -> u32 {
        self.raw.location.into()
    }

    pub fn get_binding<'a>(&'a self) -> u32 {
        self.raw.binding.into()
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_offset<'a>(&'a self) -> u32 {
        self.raw.offset.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineVertexInputStateCreateFlags {
        PipelineVertexInputStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineVertexInputStateCreateInfo::flags: error converting flags")
    }

    pub fn vertex_binding_descriptions<'a>(&'a self) -> &'a [VertexInputBindingDescription] {
        unsafe { slice::from_raw_parts(self.raw.pVertexBindingDescriptions as *const _, self.raw.vertexBindingDescriptionCount as usize) }
    }

    pub fn vertex_attribute_descriptions<'a>(&'a self) -> &'a [VertexInputAttributeDescription] {
        unsafe { slice::from_raw_parts(self.raw.pVertexAttributeDescriptions as *const _, self.raw.vertexAttributeDescriptionCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineVertexInputStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_vertex_binding_descriptions<'m, 'a>(&'s mut self, vertex_binding_descriptions: &'a [VertexInputBindingDescription]) {
        assert!(self.raw.vertexBindingDescriptionCount == 0 || self.raw.vertexBindingDescriptionCount == vertex_binding_descriptions.len() as _, 
            "count inconsistency found when specifying `PipelineVertexInputStateCreateInfo::vertex_binding_descriptions`.");
        self.raw.vertexBindingDescriptionCount = vertex_binding_descriptions.len() as _;
        self.raw.pVertexBindingDescriptions = vertex_binding_descriptions.as_ptr() as *const vks::VkVertexInputBindingDescription as *const _;
    }

    pub fn set_vertex_attribute_descriptions<'m, 'a>(&'s mut self, vertex_attribute_descriptions: &'a [VertexInputAttributeDescription]) {
        assert!(self.raw.vertexAttributeDescriptionCount == 0 || self.raw.vertexAttributeDescriptionCount == vertex_attribute_descriptions.len() as _, 
            "count inconsistency found when specifying `PipelineVertexInputStateCreateInfo::vertex_attribute_descriptions`.");
        self.raw.vertexAttributeDescriptionCount = vertex_attribute_descriptions.len() as _;
        self.raw.pVertexAttributeDescriptions = vertex_attribute_descriptions.as_ptr() as *const vks::VkVertexInputAttributeDescription as *const _;
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


impl<'s> From<vks::VkPipelineVertexInputStateCreateInfo> for PipelineVertexInputStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineVertexInputStateCreateInfo) -> PipelineVertexInputStateCreateInfo<'s> {
        PipelineVertexInputStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn vertex_binding_descriptions<'m, 'a>(mut self, vertex_binding_descriptions: &'a [VertexInputBindingDescription]) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        assert!(self.raw.vertexBindingDescriptionCount == 0 || self.raw.vertexBindingDescriptionCount == vertex_binding_descriptions.len() as _, 
            "count inconsistency found when specifying `PipelineVertexInputStateCreateInfo::vertex_binding_descriptions`.");
        self.raw.vertexBindingDescriptionCount = vertex_binding_descriptions.len() as _;
        self.raw.pVertexBindingDescriptions = vertex_binding_descriptions.as_ptr() as *const vks::VkVertexInputBindingDescription as *const _;
        self
    }

    pub fn vertex_attribute_descriptions<'m, 'a>(mut self, vertex_attribute_descriptions: &'a [VertexInputAttributeDescription]) -> PipelineVertexInputStateCreateInfoBuilder<'b> {
        assert!(self.raw.vertexAttributeDescriptionCount == 0 || self.raw.vertexAttributeDescriptionCount == vertex_attribute_descriptions.len() as _, 
            "count inconsistency found when specifying `PipelineVertexInputStateCreateInfo::vertex_attribute_descriptions`.");
        self.raw.vertexAttributeDescriptionCount = vertex_attribute_descriptions.len() as _;
        self.raw.pVertexAttributeDescriptions = vertex_attribute_descriptions.as_ptr() as *const vks::VkVertexInputAttributeDescription as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineVertexInputStateCreateFlags {
        PipelineVertexInputStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineVertexInputStateCreateInfo::flags: error converting flags")
    }

    pub fn get_vertex_binding_descriptions<'a>(&'a self) -> &'a [VertexInputBindingDescription] {
        unsafe { slice::from_raw_parts(self.raw.pVertexBindingDescriptions as *const _, self.raw.vertexBindingDescriptionCount as usize) }
    }

    pub fn get_vertex_attribute_descriptions<'a>(&'a self) -> &'a [VertexInputAttributeDescription] {
        unsafe { slice::from_raw_parts(self.raw.pVertexAttributeDescriptions as *const _, self.raw.vertexAttributeDescriptionCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineInputAssemblyStateCreateFlags {
        PipelineInputAssemblyStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineInputAssemblyStateCreateInfo::flags: error converting flags")
    }

    pub fn topology<'a>(&'a self) -> PrimitiveTopology {
        self.raw.topology.into()
    }

    pub fn primitive_restart_enable<'a>(&'a self) -> bool {
        self.raw.primitiveRestartEnable != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineInputAssemblyStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_topology<'m>(&'s mut self, topology: PrimitiveTopology) {
        self.raw.topology = topology.into();
    }

    pub fn set_primitive_restart_enable<'m>(&'s mut self, primitive_restart_enable: bool) {
        self.raw.primitiveRestartEnable = primitive_restart_enable as u32;
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


impl<'s> From<vks::VkPipelineInputAssemblyStateCreateInfo> for PipelineInputAssemblyStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineInputAssemblyStateCreateInfo) -> PipelineInputAssemblyStateCreateInfo<'s> {
        PipelineInputAssemblyStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineInputAssemblyStateCreateFlags {
        PipelineInputAssemblyStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineInputAssemblyStateCreateInfo::flags: error converting flags")
    }

    pub fn get_topology<'a>(&'a self) -> PrimitiveTopology {
        self.raw.topology.into()
    }

    pub fn get_primitive_restart_enable<'a>(&'a self) -> bool {
        self.raw.primitiveRestartEnable != 0
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineTessellationStateCreateFlags {
        PipelineTessellationStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineTessellationStateCreateInfo::flags: error converting flags")
    }

    pub fn patch_control_points<'a>(&'a self) -> u32 {
        self.raw.patchControlPoints.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineTessellationStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_patch_control_points<'m>(&'s mut self, patch_control_points: u32) {
        self.raw.patchControlPoints = patch_control_points.into();
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


impl<'s> From<vks::VkPipelineTessellationStateCreateInfo> for PipelineTessellationStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineTessellationStateCreateInfo) -> PipelineTessellationStateCreateInfo<'s> {
        PipelineTessellationStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineTessellationStateCreateFlags {
        PipelineTessellationStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineTessellationStateCreateInfo::flags: error converting flags")
    }

    pub fn get_patch_control_points<'a>(&'a self) -> u32 {
        self.raw.patchControlPoints.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineViewportStateCreateFlags {
        PipelineViewportStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineViewportStateCreateInfo::flags: error converting flags")
    }

    pub fn viewports<'a>(&'a self) -> &'a [Viewport] {
        unsafe { slice::from_raw_parts(self.raw.pViewports as *const _, self.raw.viewportCount as usize) }
    }

    pub fn scissors<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pScissors as *const _, self.raw.scissorCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineViewportStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_viewports<'m, 'a>(&'s mut self, viewports: &'a [Viewport]) {
        assert!(self.raw.viewportCount == 0 || self.raw.viewportCount == viewports.len() as _, 
            "count inconsistency found when specifying `PipelineViewportStateCreateInfo::viewports`.");
        self.raw.viewportCount = viewports.len() as _;
        self.raw.pViewports = viewports.as_ptr() as *const vks::VkViewport as *const _;
    }

    pub fn set_scissors<'m, 'a>(&'s mut self, scissors: &'a [Rect2d]) {
        assert!(self.raw.scissorCount == 0 || self.raw.scissorCount == scissors.len() as _, 
            "count inconsistency found when specifying `PipelineViewportStateCreateInfo::scissors`.");
        self.raw.scissorCount = scissors.len() as _;
        self.raw.pScissors = scissors.as_ptr() as *const vks::VkRect2D as *const _;
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


impl<'s> From<vks::VkPipelineViewportStateCreateInfo> for PipelineViewportStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineViewportStateCreateInfo) -> PipelineViewportStateCreateInfo<'s> {
        PipelineViewportStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn viewports<'m, 'a>(mut self, viewports: &'a [Viewport]) -> PipelineViewportStateCreateInfoBuilder<'b> {
        assert!(self.raw.viewportCount == 0 || self.raw.viewportCount == viewports.len() as _, 
            "count inconsistency found when specifying `PipelineViewportStateCreateInfo::viewports`.");
        self.raw.viewportCount = viewports.len() as _;
        self.raw.pViewports = viewports.as_ptr() as *const vks::VkViewport as *const _;
        self
    }

    pub fn scissors<'m, 'a>(mut self, scissors: &'a [Rect2d]) -> PipelineViewportStateCreateInfoBuilder<'b> {
        assert!(self.raw.scissorCount == 0 || self.raw.scissorCount == scissors.len() as _, 
            "count inconsistency found when specifying `PipelineViewportStateCreateInfo::scissors`.");
        self.raw.scissorCount = scissors.len() as _;
        self.raw.pScissors = scissors.as_ptr() as *const vks::VkRect2D as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineViewportStateCreateFlags {
        PipelineViewportStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineViewportStateCreateInfo::flags: error converting flags")
    }

    pub fn get_viewports<'a>(&'a self) -> &'a [Viewport] {
        unsafe { slice::from_raw_parts(self.raw.pViewports as *const _, self.raw.viewportCount as usize) }
    }

    pub fn get_scissors<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pScissors as *const _, self.raw.scissorCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineRasterizationStateCreateFlags {
        PipelineRasterizationStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineRasterizationStateCreateInfo::flags: error converting flags")
    }

    pub fn depth_clamp_enable<'a>(&'a self) -> bool {
        self.raw.depthClampEnable != 0
    }

    pub fn rasterizer_discard_enable<'a>(&'a self) -> bool {
        self.raw.rasterizerDiscardEnable != 0
    }

    pub fn polygon_mode<'a>(&'a self) -> PolygonMode {
        self.raw.polygonMode.into()
    }

    pub fn cull_mode<'a>(&'a self) -> CullModeFlags {
        CullModeFlags::from_bits(self.raw.cullMode)
            .expect("PipelineRasterizationStateCreateInfo::cull_mode: error converting flags")
    }

    pub fn front_face<'a>(&'a self) -> FrontFace {
        self.raw.frontFace.into()
    }

    pub fn depth_bias_enable<'a>(&'a self) -> bool {
        self.raw.depthBiasEnable != 0
    }

    pub fn depth_bias_constant_factor<'a>(&'a self) -> f32 {
        self.raw.depthBiasConstantFactor.into()
    }

    pub fn depth_bias_clamp<'a>(&'a self) -> f32 {
        self.raw.depthBiasClamp.into()
    }

    pub fn depth_bias_slope_factor<'a>(&'a self) -> f32 {
        self.raw.depthBiasSlopeFactor.into()
    }

    pub fn line_width<'a>(&'a self) -> f32 {
        self.raw.lineWidth.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineRasterizationStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_depth_clamp_enable<'m>(&'s mut self, depth_clamp_enable: bool) {
        self.raw.depthClampEnable = depth_clamp_enable as u32;
    }

    pub fn set_rasterizer_discard_enable<'m>(&'s mut self, rasterizer_discard_enable: bool) {
        self.raw.rasterizerDiscardEnable = rasterizer_discard_enable as u32;
    }

    pub fn set_polygon_mode<'m>(&'s mut self, polygon_mode: PolygonMode) {
        self.raw.polygonMode = polygon_mode.into();
    }

    pub fn set_cull_mode<'m>(&'s mut self, cull_mode: CullModeFlags) {
        self.raw.cullMode = cull_mode.bits();
    }

    pub fn set_front_face<'m>(&'s mut self, front_face: FrontFace) {
        self.raw.frontFace = front_face.into();
    }

    pub fn set_depth_bias_enable<'m>(&'s mut self, depth_bias_enable: bool) {
        self.raw.depthBiasEnable = depth_bias_enable as u32;
    }

    pub fn set_depth_bias_constant_factor<'m>(&'s mut self, depth_bias_constant_factor: f32) {
        self.raw.depthBiasConstantFactor = depth_bias_constant_factor.into();
    }

    pub fn set_depth_bias_clamp<'m>(&'s mut self, depth_bias_clamp: f32) {
        self.raw.depthBiasClamp = depth_bias_clamp.into();
    }

    pub fn set_depth_bias_slope_factor<'m>(&'s mut self, depth_bias_slope_factor: f32) {
        self.raw.depthBiasSlopeFactor = depth_bias_slope_factor.into();
    }

    pub fn set_line_width<'m>(&'s mut self, line_width: f32) {
        self.raw.lineWidth = line_width.into();
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


impl<'s> From<vks::VkPipelineRasterizationStateCreateInfo> for PipelineRasterizationStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineRasterizationStateCreateInfo) -> PipelineRasterizationStateCreateInfo<'s> {
        PipelineRasterizationStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineRasterizationStateCreateFlags {
        PipelineRasterizationStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineRasterizationStateCreateInfo::flags: error converting flags")
    }

    pub fn get_depth_clamp_enable<'a>(&'a self) -> bool {
        self.raw.depthClampEnable != 0
    }

    pub fn get_rasterizer_discard_enable<'a>(&'a self) -> bool {
        self.raw.rasterizerDiscardEnable != 0
    }

    pub fn get_polygon_mode<'a>(&'a self) -> PolygonMode {
        self.raw.polygonMode.into()
    }

    pub fn get_cull_mode<'a>(&'a self) -> CullModeFlags {
        CullModeFlags::from_bits(self.raw.cullMode)
            .expect("PipelineRasterizationStateCreateInfo::cull_mode: error converting flags")
    }

    pub fn get_front_face<'a>(&'a self) -> FrontFace {
        self.raw.frontFace.into()
    }

    pub fn get_depth_bias_enable<'a>(&'a self) -> bool {
        self.raw.depthBiasEnable != 0
    }

    pub fn get_depth_bias_constant_factor<'a>(&'a self) -> f32 {
        self.raw.depthBiasConstantFactor.into()
    }

    pub fn get_depth_bias_clamp<'a>(&'a self) -> f32 {
        self.raw.depthBiasClamp.into()
    }

    pub fn get_depth_bias_slope_factor<'a>(&'a self) -> f32 {
        self.raw.depthBiasSlopeFactor.into()
    }

    pub fn get_line_width<'a>(&'a self) -> f32 {
        self.raw.lineWidth.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineMultisampleStateCreateFlags {
        PipelineMultisampleStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineMultisampleStateCreateInfo::flags: error converting flags")
    }

    pub fn rasterization_samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.rasterizationSamples)
            .expect("PipelineMultisampleStateCreateInfo::rasterization_samples: error converting flags")
    }

    pub fn sample_shading_enable<'a>(&'a self) -> bool {
        self.raw.sampleShadingEnable != 0
    }

    pub fn min_sample_shading<'a>(&'a self) -> f32 {
        self.raw.minSampleShading.into()
    }

    pub fn sample_mask<'a>(&'a self) -> &'a u32 {
        unsafe { &*(self.raw.pSampleMask as *const _) }
    }

    pub fn alpha_to_coverage_enable<'a>(&'a self) -> bool {
        self.raw.alphaToCoverageEnable != 0
    }

    pub fn alpha_to_one_enable<'a>(&'a self) -> bool {
        self.raw.alphaToOneEnable != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineMultisampleStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_rasterization_samples<'m>(&'s mut self, rasterization_samples: SampleCountFlags) {
        self.raw.rasterizationSamples = rasterization_samples.bits();
    }

    pub fn set_sample_shading_enable<'m>(&'s mut self, sample_shading_enable: bool) {
        self.raw.sampleShadingEnable = sample_shading_enable as u32;
    }

    pub fn set_min_sample_shading<'m>(&'s mut self, min_sample_shading: f32) {
        self.raw.minSampleShading = min_sample_shading.into();
    }

    pub fn set_sample_mask<'m, 'a>(&'s mut self, sample_mask: &'a u32) {
        self.raw.pSampleMask = sample_mask;
    }

    pub fn set_alpha_to_coverage_enable<'m>(&'s mut self, alpha_to_coverage_enable: bool) {
        self.raw.alphaToCoverageEnable = alpha_to_coverage_enable as u32;
    }

    pub fn set_alpha_to_one_enable<'m>(&'s mut self, alpha_to_one_enable: bool) {
        self.raw.alphaToOneEnable = alpha_to_one_enable as u32;
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


impl<'s> From<vks::VkPipelineMultisampleStateCreateInfo> for PipelineMultisampleStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineMultisampleStateCreateInfo) -> PipelineMultisampleStateCreateInfo<'s> {
        PipelineMultisampleStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineMultisampleStateCreateFlags {
        PipelineMultisampleStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineMultisampleStateCreateInfo::flags: error converting flags")
    }

    pub fn get_rasterization_samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.rasterizationSamples)
            .expect("PipelineMultisampleStateCreateInfo::rasterization_samples: error converting flags")
    }

    pub fn get_sample_shading_enable<'a>(&'a self) -> bool {
        self.raw.sampleShadingEnable != 0
    }

    pub fn get_min_sample_shading<'a>(&'a self) -> f32 {
        self.raw.minSampleShading.into()
    }

    pub fn get_sample_mask<'a>(&'a self) -> &'a u32 {
        unsafe { &*(self.raw.pSampleMask as *const _) }
    }

    pub fn get_alpha_to_coverage_enable<'a>(&'a self) -> bool {
        self.raw.alphaToCoverageEnable != 0
    }

    pub fn get_alpha_to_one_enable<'a>(&'a self) -> bool {
        self.raw.alphaToOneEnable != 0
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

    pub fn blend_enable<'a>(&'a self) -> bool {
        self.raw.blendEnable != 0
    }

    pub fn src_color_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.srcColorBlendFactor.into()
    }

    pub fn dst_color_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.dstColorBlendFactor.into()
    }

    pub fn color_blend_op<'a>(&'a self) -> BlendOp {
        self.raw.colorBlendOp.into()
    }

    pub fn src_alpha_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.srcAlphaBlendFactor.into()
    }

    pub fn dst_alpha_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.dstAlphaBlendFactor.into()
    }

    pub fn alpha_blend_op<'a>(&'a self) -> BlendOp {
        self.raw.alphaBlendOp.into()
    }

    pub fn color_write_mask<'a>(&'a self) -> ColorComponentFlags {
        ColorComponentFlags::from_bits(self.raw.colorWriteMask)
            .expect("PipelineColorBlendAttachmentState::color_write_mask: error converting flags")
    }

    pub fn set_blend_enable<'m>(& mut self, blend_enable: bool) {
        self.raw.blendEnable = blend_enable as u32;
    }

    pub fn set_src_color_blend_factor<'m>(& mut self, src_color_blend_factor: BlendFactor) {
        self.raw.srcColorBlendFactor = src_color_blend_factor.into();
    }

    pub fn set_dst_color_blend_factor<'m>(& mut self, dst_color_blend_factor: BlendFactor) {
        self.raw.dstColorBlendFactor = dst_color_blend_factor.into();
    }

    pub fn set_color_blend_op<'m>(& mut self, color_blend_op: BlendOp) {
        self.raw.colorBlendOp = color_blend_op.into();
    }

    pub fn set_src_alpha_blend_factor<'m>(& mut self, src_alpha_blend_factor: BlendFactor) {
        self.raw.srcAlphaBlendFactor = src_alpha_blend_factor.into();
    }

    pub fn set_dst_alpha_blend_factor<'m>(& mut self, dst_alpha_blend_factor: BlendFactor) {
        self.raw.dstAlphaBlendFactor = dst_alpha_blend_factor.into();
    }

    pub fn set_alpha_blend_op<'m>(& mut self, alpha_blend_op: BlendOp) {
        self.raw.alphaBlendOp = alpha_blend_op.into();
    }

    pub fn set_color_write_mask<'m>(& mut self, color_write_mask: ColorComponentFlags) {
        self.raw.colorWriteMask = color_write_mask.bits();
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


impl From<vks::VkPipelineColorBlendAttachmentState> for PipelineColorBlendAttachmentState {
    fn from(f: vks::VkPipelineColorBlendAttachmentState) -> PipelineColorBlendAttachmentState {
        PipelineColorBlendAttachmentState { raw: f, }
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

    pub fn get_blend_enable<'a>(&'a self) -> bool {
        self.raw.blendEnable != 0
    }

    pub fn get_src_color_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.srcColorBlendFactor.into()
    }

    pub fn get_dst_color_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.dstColorBlendFactor.into()
    }

    pub fn get_color_blend_op<'a>(&'a self) -> BlendOp {
        self.raw.colorBlendOp.into()
    }

    pub fn get_src_alpha_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.srcAlphaBlendFactor.into()
    }

    pub fn get_dst_alpha_blend_factor<'a>(&'a self) -> BlendFactor {
        self.raw.dstAlphaBlendFactor.into()
    }

    pub fn get_alpha_blend_op<'a>(&'a self) -> BlendOp {
        self.raw.alphaBlendOp.into()
    }

    pub fn get_color_write_mask<'a>(&'a self) -> ColorComponentFlags {
        ColorComponentFlags::from_bits(self.raw.colorWriteMask)
            .expect("PipelineColorBlendAttachmentState::color_write_mask: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineColorBlendStateCreateFlags {
        PipelineColorBlendStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineColorBlendStateCreateInfo::flags: error converting flags")
    }

    pub fn logic_op_enable<'a>(&'a self) -> bool {
        self.raw.logicOpEnable != 0
    }

    pub fn logic_op<'a>(&'a self) -> LogicOp {
        self.raw.logicOp.into()
    }

    pub fn attachments<'a>(&'a self) -> &'a [PipelineColorBlendAttachmentState] {
        unsafe { slice::from_raw_parts(self.raw.pAttachments as *const _, self.raw.attachmentCount as usize) }
    }

    pub fn blend_constants<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.blendConstants as *const _, 4 as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineColorBlendStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_logic_op_enable<'m>(&'s mut self, logic_op_enable: bool) {
        self.raw.logicOpEnable = logic_op_enable as u32;
    }

    pub fn set_logic_op<'m>(&'s mut self, logic_op: LogicOp) {
        self.raw.logicOp = logic_op.into();
    }

    pub fn set_attachments<'m, 'a>(&'s mut self, attachments: &'a [PipelineColorBlendAttachmentState]) {
        assert!(self.raw.attachmentCount == 0 || self.raw.attachmentCount == attachments.len() as _, 
            "count inconsistency found when specifying `PipelineColorBlendStateCreateInfo::attachments`.");
        self.raw.attachmentCount = attachments.len() as _;
        self.raw.pAttachments = attachments.as_ptr() as *const vks::VkPipelineColorBlendAttachmentState as *const _;
    }

    pub fn set_blend_constants<'m>(&'s mut self, blend_constants: [f32; 4]) {
        self.raw.blendConstants = blend_constants;
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


impl<'s> From<vks::VkPipelineColorBlendStateCreateInfo> for PipelineColorBlendStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineColorBlendStateCreateInfo) -> PipelineColorBlendStateCreateInfo<'s> {
        PipelineColorBlendStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn attachments<'m, 'a>(mut self, attachments: &'a [PipelineColorBlendAttachmentState]) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        assert!(self.raw.attachmentCount == 0 || self.raw.attachmentCount == attachments.len() as _, 
            "count inconsistency found when specifying `PipelineColorBlendStateCreateInfo::attachments`.");
        self.raw.attachmentCount = attachments.len() as _;
        self.raw.pAttachments = attachments.as_ptr() as *const vks::VkPipelineColorBlendAttachmentState as *const _;
        self
    }

    pub fn blend_constants<'m>(mut self, blend_constants: [f32; 4]) -> PipelineColorBlendStateCreateInfoBuilder<'b> {
        self.raw.blendConstants = blend_constants;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineColorBlendStateCreateFlags {
        PipelineColorBlendStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineColorBlendStateCreateInfo::flags: error converting flags")
    }

    pub fn get_logic_op_enable<'a>(&'a self) -> bool {
        self.raw.logicOpEnable != 0
    }

    pub fn get_logic_op<'a>(&'a self) -> LogicOp {
        self.raw.logicOp.into()
    }

    pub fn get_attachments<'a>(&'a self) -> &'a [PipelineColorBlendAttachmentState] {
        unsafe { slice::from_raw_parts(self.raw.pAttachments as *const _, self.raw.attachmentCount as usize) }
    }

    pub fn get_blend_constants<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.blendConstants as *const _, 4 as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineDynamicStateCreateFlags {
        PipelineDynamicStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineDynamicStateCreateInfo::flags: error converting flags")
    }

    pub fn dynamic_states<'a>(&'a self) -> &'a [DynamicState] {
        unsafe { slice::from_raw_parts(self.raw.pDynamicStates as *const _, self.raw.dynamicStateCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineDynamicStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_dynamic_states<'m, 'a>(&'s mut self, dynamic_states: &'a [DynamicState]) {
        assert!(self.raw.dynamicStateCount == 0 || self.raw.dynamicStateCount == dynamic_states.len() as _, 
            "count inconsistency found when specifying `PipelineDynamicStateCreateInfo::dynamic_states`.");
        self.raw.dynamicStateCount = dynamic_states.len() as _;
        self.raw.pDynamicStates = dynamic_states.as_ptr() as *const DynamicState as *const _;
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


impl<'s> From<vks::VkPipelineDynamicStateCreateInfo> for PipelineDynamicStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineDynamicStateCreateInfo) -> PipelineDynamicStateCreateInfo<'s> {
        PipelineDynamicStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn dynamic_states<'m, 'a>(mut self, dynamic_states: &'a [DynamicState]) -> PipelineDynamicStateCreateInfoBuilder<'b> {
        assert!(self.raw.dynamicStateCount == 0 || self.raw.dynamicStateCount == dynamic_states.len() as _, 
            "count inconsistency found when specifying `PipelineDynamicStateCreateInfo::dynamic_states`.");
        self.raw.dynamicStateCount = dynamic_states.len() as _;
        self.raw.pDynamicStates = dynamic_states.as_ptr() as *const DynamicState as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineDynamicStateCreateFlags {
        PipelineDynamicStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineDynamicStateCreateInfo::flags: error converting flags")
    }

    pub fn get_dynamic_states<'a>(&'a self) -> &'a [DynamicState] {
        unsafe { slice::from_raw_parts(self.raw.pDynamicStates as *const _, self.raw.dynamicStateCount as usize) }
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

    pub fn fail_op<'a>(&'a self) -> StencilOp {
        self.raw.failOp.into()
    }

    pub fn pass_op<'a>(&'a self) -> StencilOp {
        self.raw.passOp.into()
    }

    pub fn depth_fail_op<'a>(&'a self) -> StencilOp {
        self.raw.depthFailOp.into()
    }

    pub fn compare_op<'a>(&'a self) -> CompareOp {
        self.raw.compareOp.into()
    }

    pub fn compare_mask<'a>(&'a self) -> u32 {
        self.raw.compareMask.into()
    }

    pub fn write_mask<'a>(&'a self) -> u32 {
        self.raw.writeMask.into()
    }

    pub fn reference<'a>(&'a self) -> u32 {
        self.raw.reference.into()
    }

    pub fn set_fail_op<'m>(& mut self, fail_op: StencilOp) {
        self.raw.failOp = fail_op.into();
    }

    pub fn set_pass_op<'m>(& mut self, pass_op: StencilOp) {
        self.raw.passOp = pass_op.into();
    }

    pub fn set_depth_fail_op<'m>(& mut self, depth_fail_op: StencilOp) {
        self.raw.depthFailOp = depth_fail_op.into();
    }

    pub fn set_compare_op<'m>(& mut self, compare_op: CompareOp) {
        self.raw.compareOp = compare_op.into();
    }

    pub fn set_compare_mask<'m>(& mut self, compare_mask: u32) {
        self.raw.compareMask = compare_mask.into();
    }

    pub fn set_write_mask<'m>(& mut self, write_mask: u32) {
        self.raw.writeMask = write_mask.into();
    }

    pub fn set_reference<'m>(& mut self, reference: u32) {
        self.raw.reference = reference.into();
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


impl From<vks::VkStencilOpState> for StencilOpState {
    fn from(f: vks::VkStencilOpState) -> StencilOpState {
        StencilOpState { raw: f, }
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

    pub fn get_fail_op<'a>(&'a self) -> StencilOp {
        self.raw.failOp.into()
    }

    pub fn get_pass_op<'a>(&'a self) -> StencilOp {
        self.raw.passOp.into()
    }

    pub fn get_depth_fail_op<'a>(&'a self) -> StencilOp {
        self.raw.depthFailOp.into()
    }

    pub fn get_compare_op<'a>(&'a self) -> CompareOp {
        self.raw.compareOp.into()
    }

    pub fn get_compare_mask<'a>(&'a self) -> u32 {
        self.raw.compareMask.into()
    }

    pub fn get_write_mask<'a>(&'a self) -> u32 {
        self.raw.writeMask.into()
    }

    pub fn get_reference<'a>(&'a self) -> u32 {
        self.raw.reference.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineDepthStencilStateCreateFlags {
        PipelineDepthStencilStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineDepthStencilStateCreateInfo::flags: error converting flags")
    }

    pub fn depth_test_enable<'a>(&'a self) -> bool {
        self.raw.depthTestEnable != 0
    }

    pub fn depth_write_enable<'a>(&'a self) -> bool {
        self.raw.depthWriteEnable != 0
    }

    pub fn depth_compare_op<'a>(&'a self) -> CompareOp {
        self.raw.depthCompareOp.into()
    }

    pub fn depth_bounds_test_enable<'a>(&'a self) -> bool {
        self.raw.depthBoundsTestEnable != 0
    }

    pub fn stencil_test_enable<'a>(&'a self) -> bool {
        self.raw.stencilTestEnable != 0
    }

    pub fn front<'a>(&'a self) -> StencilOpState {
        self.raw.front.into()
    }

    pub fn back<'a>(&'a self) -> StencilOpState {
        self.raw.back.into()
    }

    pub fn min_depth_bounds<'a>(&'a self) -> f32 {
        self.raw.minDepthBounds.into()
    }

    pub fn max_depth_bounds<'a>(&'a self) -> f32 {
        self.raw.maxDepthBounds.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineDepthStencilStateCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_depth_test_enable<'m>(&'s mut self, depth_test_enable: bool) {
        self.raw.depthTestEnable = depth_test_enable as u32;
    }

    pub fn set_depth_write_enable<'m>(&'s mut self, depth_write_enable: bool) {
        self.raw.depthWriteEnable = depth_write_enable as u32;
    }

    pub fn set_depth_compare_op<'m>(&'s mut self, depth_compare_op: CompareOp) {
        self.raw.depthCompareOp = depth_compare_op.into();
    }

    pub fn set_depth_bounds_test_enable<'m>(&'s mut self, depth_bounds_test_enable: bool) {
        self.raw.depthBoundsTestEnable = depth_bounds_test_enable as u32;
    }

    pub fn set_stencil_test_enable<'m>(&'s mut self, stencil_test_enable: bool) {
        self.raw.stencilTestEnable = stencil_test_enable as u32;
    }

    pub fn set_front<'m>(&'s mut self, front: StencilOpState) {
        self.raw.front = front.raw;
    }

    pub fn set_back<'m>(&'s mut self, back: StencilOpState) {
        self.raw.back = back.raw;
    }

    pub fn set_min_depth_bounds<'m>(&'s mut self, min_depth_bounds: f32) {
        self.raw.minDepthBounds = min_depth_bounds.into();
    }

    pub fn set_max_depth_bounds<'m>(&'s mut self, max_depth_bounds: f32) {
        self.raw.maxDepthBounds = max_depth_bounds.into();
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


impl<'s> From<vks::VkPipelineDepthStencilStateCreateInfo> for PipelineDepthStencilStateCreateInfo<'s> {
    fn from(f: vks::VkPipelineDepthStencilStateCreateInfo) -> PipelineDepthStencilStateCreateInfo<'s> {
        PipelineDepthStencilStateCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineDepthStencilStateCreateFlags {
        PipelineDepthStencilStateCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineDepthStencilStateCreateInfo::flags: error converting flags")
    }

    pub fn get_depth_test_enable<'a>(&'a self) -> bool {
        self.raw.depthTestEnable != 0
    }

    pub fn get_depth_write_enable<'a>(&'a self) -> bool {
        self.raw.depthWriteEnable != 0
    }

    pub fn get_depth_compare_op<'a>(&'a self) -> CompareOp {
        self.raw.depthCompareOp.into()
    }

    pub fn get_depth_bounds_test_enable<'a>(&'a self) -> bool {
        self.raw.depthBoundsTestEnable != 0
    }

    pub fn get_stencil_test_enable<'a>(&'a self) -> bool {
        self.raw.stencilTestEnable != 0
    }

    pub fn get_front<'a>(&'a self) -> StencilOpState {
        self.raw.front.into()
    }

    pub fn get_back<'a>(&'a self) -> StencilOpState {
        self.raw.back.into()
    }

    pub fn get_min_depth_bounds<'a>(&'a self) -> f32 {
        self.raw.minDepthBounds.into()
    }

    pub fn get_max_depth_bounds<'a>(&'a self) -> f32 {
        self.raw.maxDepthBounds.into()
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
    stages: Option<SmallVec<[vks::VkPipelineShaderStageCreateInfo; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> GraphicsPipelineCreateInfo<'s> {
    pub fn builder<'b>() -> GraphicsPipelineCreateInfoBuilder<'b> {
        GraphicsPipelineCreateInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineCreateFlags {
        PipelineCreateFlags::from_bits(self.raw.flags)
            .expect("GraphicsPipelineCreateInfo::flags: error converting flags")
    }

    pub fn stages<'a>(&'a self) -> &'a [vks::VkPipelineShaderStageCreateInfo] {
        unsafe { slice::from_raw_parts(self.raw.pStages as *const _, self.raw.stageCount as usize) }
    }

    pub fn vertex_input_state<'a>(&'a self) -> &'a PipelineVertexInputStateCreateInfo {
        unsafe { &*(self.raw.pVertexInputState as *const vks::VkPipelineVertexInputStateCreateInfo as *const _) }
    }

    pub fn input_assembly_state<'a>(&'a self) -> &'a PipelineInputAssemblyStateCreateInfo {
        unsafe { &*(self.raw.pInputAssemblyState as *const vks::VkPipelineInputAssemblyStateCreateInfo as *const _) }
    }

    pub fn tessellation_state<'a>(&'a self) -> &'a PipelineTessellationStateCreateInfo {
        unsafe { &*(self.raw.pTessellationState as *const vks::VkPipelineTessellationStateCreateInfo as *const _) }
    }

    pub fn viewport_state<'a>(&'a self) -> &'a PipelineViewportStateCreateInfo {
        unsafe { &*(self.raw.pViewportState as *const vks::VkPipelineViewportStateCreateInfo as *const _) }
    }

    pub fn rasterization_state<'a>(&'a self) -> &'a PipelineRasterizationStateCreateInfo {
        unsafe { &*(self.raw.pRasterizationState as *const vks::VkPipelineRasterizationStateCreateInfo as *const _) }
    }

    pub fn multisample_state<'a>(&'a self) -> &'a PipelineMultisampleStateCreateInfo {
        unsafe { &*(self.raw.pMultisampleState as *const vks::VkPipelineMultisampleStateCreateInfo as *const _) }
    }

    pub fn depth_stencil_state<'a>(&'a self) -> &'a PipelineDepthStencilStateCreateInfo {
        unsafe { &*(self.raw.pDepthStencilState as *const vks::VkPipelineDepthStencilStateCreateInfo as *const _) }
    }

    pub fn color_blend_state<'a>(&'a self) -> &'a PipelineColorBlendStateCreateInfo {
        unsafe { &*(self.raw.pColorBlendState as *const vks::VkPipelineColorBlendStateCreateInfo as *const _) }
    }

    pub fn dynamic_state<'a>(&'a self) -> &'a PipelineDynamicStateCreateInfo {
        unsafe { &*(self.raw.pDynamicState as *const vks::VkPipelineDynamicStateCreateInfo as *const _) }
    }

    pub fn layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.layout
    }

    pub fn render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn subpass<'a>(&'a self) -> u32 {
        self.raw.subpass.into()
    }

    pub fn base_pipeline_handle_handle<'a>(&'a self) -> vks::VkPipeline {
        self.raw.basePipelineHandle
    }

    pub fn base_pipeline_index<'a>(&'a self) -> i32 {
        self.raw.basePipelineIndex.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_stages<'m, 'a>(&'s mut self, stages: &'a [PipelineShaderStageCreateInfo]) {
        self.stages = Some(stages.iter().map(|h| h.raw).collect());
        {
            let stages = self.stages.as_ref().unwrap();
            self.raw.pStages = stages.as_ptr();
            assert!(self.raw.stageCount == 0 || self.raw.stageCount == stages.len() as _, 
                "count inconsistency found when specifying `GraphicsPipelineCreateInfo::stages`.");
            self.raw.stageCount = stages.len() as _;
        }
    }

    pub fn set_vertex_input_state<'m, 'a>(&'s mut self, vertex_input_state: &'a PipelineVertexInputStateCreateInfo) {
        self.raw.pVertexInputState = vertex_input_state.raw();
    }

    pub fn set_input_assembly_state<'m, 'a>(&'s mut self, input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo) {
        self.raw.pInputAssemblyState = input_assembly_state.raw();
    }

    pub fn set_tessellation_state<'m, 'a>(&'s mut self, tessellation_state: &'a PipelineTessellationStateCreateInfo) {
        self.raw.pTessellationState = tessellation_state.raw();
    }

    pub fn set_viewport_state<'m, 'a>(&'s mut self, viewport_state: &'a PipelineViewportStateCreateInfo) {
        self.raw.pViewportState = viewport_state.raw();
    }

    pub fn set_rasterization_state<'m, 'a>(&'s mut self, rasterization_state: &'a PipelineRasterizationStateCreateInfo) {
        self.raw.pRasterizationState = rasterization_state.raw();
    }

    pub fn set_multisample_state<'m, 'a>(&'s mut self, multisample_state: &'a PipelineMultisampleStateCreateInfo) {
        self.raw.pMultisampleState = multisample_state.raw();
    }

    pub fn set_depth_stencil_state<'m, 'a>(&'s mut self, depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo) {
        self.raw.pDepthStencilState = depth_stencil_state.raw();
    }

    pub fn set_color_blend_state<'m, 'a>(&'s mut self, color_blend_state: &'a PipelineColorBlendStateCreateInfo) {
        self.raw.pColorBlendState = color_blend_state.raw();
    }

    pub fn set_dynamic_state<'m, 'a>(&'s mut self, dynamic_state: &'a PipelineDynamicStateCreateInfo) {
        self.raw.pDynamicState = dynamic_state.raw();
    }

    pub fn set_layout<'m, 'a>(&'s mut self, layout: &'a PipelineLayout) {
        self.raw.layout = layout.handle();
    }

    pub fn set_render_pass<'m, 'a>(&'s mut self, render_pass: &'a RenderPass) {
        self.raw.renderPass = render_pass.handle();
    }

    pub fn set_subpass<'m>(&'s mut self, subpass: u32) {
        self.raw.subpass = subpass.into();
    }

    pub fn set_base_pipeline_handle<'m, 'a>(&'s mut self, base_pipeline_handle: &'a Pipeline) {
        self.raw.basePipelineHandle = base_pipeline_handle.handle();
    }

    pub fn set_base_pipeline_index<'m>(&'s mut self, base_pipeline_index: i32) {
        self.raw.basePipelineIndex = base_pipeline_index.into();
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


impl<'s> From<vks::VkGraphicsPipelineCreateInfo> for GraphicsPipelineCreateInfo<'s> {
    fn from(f: vks::VkGraphicsPipelineCreateInfo) -> GraphicsPipelineCreateInfo<'s> {
        GraphicsPipelineCreateInfo { raw: f, stages: None, _p: PhantomData }
    }
}


/// A builder for `VkGraphicsPipelineCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'b> {
    raw: vks::VkGraphicsPipelineCreateInfo,
    stages: Option<SmallVec<[vks::VkPipelineShaderStageCreateInfo; 8]>>,
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

    pub fn stages<'m, 'a>(mut self, stages: &'a [PipelineShaderStageCreateInfo]) -> GraphicsPipelineCreateInfoBuilder<'b> {
        self.stages = Some(stages.iter().map(|h| h.raw).collect());
        {
            let stages = self.stages.as_ref().unwrap();
            self.raw.pStages = stages.as_ptr();
            assert!(self.raw.stageCount == 0 || self.raw.stageCount == stages.len() as _, 
                "count inconsistency found when specifying `GraphicsPipelineCreateInfo::stages`.");
            self.raw.stageCount = stages.len() as _;
        }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineCreateFlags {
        PipelineCreateFlags::from_bits(self.raw.flags)
            .expect("GraphicsPipelineCreateInfo::flags: error converting flags")
    }

    pub fn get_stages<'a>(&'a self) -> &'a [vks::VkPipelineShaderStageCreateInfo] {
        unsafe { slice::from_raw_parts(self.raw.pStages as *const _, self.raw.stageCount as usize) }
    }

    pub fn get_vertex_input_state<'a>(&'a self) -> &'a PipelineVertexInputStateCreateInfo {
        unsafe { &*(self.raw.pVertexInputState as *const vks::VkPipelineVertexInputStateCreateInfo as *const _) }
    }

    pub fn get_input_assembly_state<'a>(&'a self) -> &'a PipelineInputAssemblyStateCreateInfo {
        unsafe { &*(self.raw.pInputAssemblyState as *const vks::VkPipelineInputAssemblyStateCreateInfo as *const _) }
    }

    pub fn get_tessellation_state<'a>(&'a self) -> &'a PipelineTessellationStateCreateInfo {
        unsafe { &*(self.raw.pTessellationState as *const vks::VkPipelineTessellationStateCreateInfo as *const _) }
    }

    pub fn get_viewport_state<'a>(&'a self) -> &'a PipelineViewportStateCreateInfo {
        unsafe { &*(self.raw.pViewportState as *const vks::VkPipelineViewportStateCreateInfo as *const _) }
    }

    pub fn get_rasterization_state<'a>(&'a self) -> &'a PipelineRasterizationStateCreateInfo {
        unsafe { &*(self.raw.pRasterizationState as *const vks::VkPipelineRasterizationStateCreateInfo as *const _) }
    }

    pub fn get_multisample_state<'a>(&'a self) -> &'a PipelineMultisampleStateCreateInfo {
        unsafe { &*(self.raw.pMultisampleState as *const vks::VkPipelineMultisampleStateCreateInfo as *const _) }
    }

    pub fn get_depth_stencil_state<'a>(&'a self) -> &'a PipelineDepthStencilStateCreateInfo {
        unsafe { &*(self.raw.pDepthStencilState as *const vks::VkPipelineDepthStencilStateCreateInfo as *const _) }
    }

    pub fn get_color_blend_state<'a>(&'a self) -> &'a PipelineColorBlendStateCreateInfo {
        unsafe { &*(self.raw.pColorBlendState as *const vks::VkPipelineColorBlendStateCreateInfo as *const _) }
    }

    pub fn get_dynamic_state<'a>(&'a self) -> &'a PipelineDynamicStateCreateInfo {
        unsafe { &*(self.raw.pDynamicState as *const vks::VkPipelineDynamicStateCreateInfo as *const _) }
    }

    pub fn get_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.layout
    }

    pub fn get_render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn get_subpass<'a>(&'a self) -> u32 {
        self.raw.subpass.into()
    }

    pub fn get_base_pipeline_handle_handle<'a>(&'a self) -> vks::VkPipeline {
        self.raw.basePipelineHandle
    }

    pub fn get_base_pipeline_index<'a>(&'a self) -> i32 {
        self.raw.basePipelineIndex.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineCacheCreateFlags {
        PipelineCacheCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineCacheCreateInfo::flags: error converting flags")
    }

    pub fn initial_data_size<'a>(&'a self) -> usize {
        self.raw.initialDataSize.into()
    }

    pub fn initial_data<'a>(&'a self) -> *const c_void {
        self.raw.pInitialData
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineCacheCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_initial_data_size<'m>(&'s mut self, initial_data_size: usize) {
        self.raw.initialDataSize = initial_data_size.into();
    }

    pub unsafe fn set_initial_data<'m>(&'s mut self, initial_data: *const c_void) {
        self.raw.pInitialData = initial_data;
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


impl<'s> From<vks::VkPipelineCacheCreateInfo> for PipelineCacheCreateInfo<'s> {
    fn from(f: vks::VkPipelineCacheCreateInfo) -> PipelineCacheCreateInfo<'s> {
        PipelineCacheCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineCacheCreateFlags {
        PipelineCacheCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineCacheCreateInfo::flags: error converting flags")
    }

    pub fn get_initial_data_size<'a>(&'a self) -> usize {
        self.raw.initialDataSize.into()
    }

    pub fn get_initial_data<'a>(&'a self) -> *const c_void {
        self.raw.pInitialData
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

    pub fn stage_flags<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stageFlags)
            .expect("PushConstantRange::stage_flags: error converting flags")
    }

    pub fn offset<'a>(&'a self) -> u32 {
        self.raw.offset.into()
    }

    pub fn size<'a>(&'a self) -> u32 {
        self.raw.size.into()
    }

    pub fn set_stage_flags<'m>(& mut self, stage_flags: ShaderStageFlags) {
        self.raw.stageFlags = stage_flags.bits();
    }

    pub fn set_offset<'m>(& mut self, offset: u32) {
        self.raw.offset = offset.into();
    }

    pub fn set_size<'m>(& mut self, size: u32) {
        self.raw.size = size.into();
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


impl From<vks::VkPushConstantRange> for PushConstantRange {
    fn from(f: vks::VkPushConstantRange) -> PushConstantRange {
        PushConstantRange { raw: f, }
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

    pub fn get_stage_flags<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stageFlags)
            .expect("PushConstantRange::stage_flags: error converting flags")
    }

    pub fn get_offset<'a>(&'a self) -> u32 {
        self.raw.offset.into()
    }

    pub fn get_size<'a>(&'a self) -> u32 {
        self.raw.size.into()
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
    set_layouts: Option<SmallVec<[vks::VkDescriptorSetLayout; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> PipelineLayoutCreateInfo<'s> {
    pub fn builder<'b>() -> PipelineLayoutCreateInfoBuilder<'b> {
        PipelineLayoutCreateInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineLayoutCreateFlags {
        PipelineLayoutCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineLayoutCreateInfo::flags: error converting flags")
    }

    pub fn set_layouts_handle<'a>(&'a self) -> &'a [vks::VkDescriptorSetLayout] {
        unsafe { slice::from_raw_parts(self.raw.pSetLayouts as *const _, self.raw.setLayoutCount as usize) }
    }

    pub fn push_constant_ranges<'a>(&'a self) -> &'a [PushConstantRange] {
        unsafe { slice::from_raw_parts(self.raw.pPushConstantRanges as *const _, self.raw.pushConstantRangeCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineLayoutCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_set_layouts<'m, 'a>(&'s mut self, set_layouts: &'a [&'a DescriptorSetLayout])
            where 'a: 's {
        self.set_layouts = Some(set_layouts.iter().map(|h| h.handle()).collect());
        {
            let set_layouts = self.set_layouts.as_ref().unwrap();
            self.raw.pSetLayouts = set_layouts.as_ptr();
            assert!(self.raw.setLayoutCount == 0 || self.raw.setLayoutCount == set_layouts.len() as _, 
                "count inconsistency found when specifying `PipelineLayoutCreateInfo::set_layouts`.");
            self.raw.setLayoutCount = set_layouts.len() as _;
        }
    }

    pub fn set_push_constant_ranges<'m, 'a>(&'s mut self, push_constant_ranges: &'a [PushConstantRange]) {
        assert!(self.raw.pushConstantRangeCount == 0 || self.raw.pushConstantRangeCount == push_constant_ranges.len() as _, 
            "count inconsistency found when specifying `PipelineLayoutCreateInfo::push_constant_ranges`.");
        self.raw.pushConstantRangeCount = push_constant_ranges.len() as _;
        self.raw.pPushConstantRanges = push_constant_ranges.as_ptr() as *const vks::VkPushConstantRange as *const _;
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


impl<'s> From<vks::VkPipelineLayoutCreateInfo> for PipelineLayoutCreateInfo<'s> {
    fn from(f: vks::VkPipelineLayoutCreateInfo) -> PipelineLayoutCreateInfo<'s> {
        PipelineLayoutCreateInfo { raw: f, set_layouts: None, _p: PhantomData }
    }
}


/// A builder for `VkPipelineLayoutCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PipelineLayoutCreateInfoBuilder<'b> {
    raw: vks::VkPipelineLayoutCreateInfo,
    set_layouts: Option<SmallVec<[vks::VkDescriptorSetLayout; 8]>>,
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

    pub fn set_layouts<'m, 'a>(mut self, set_layouts: &'a [&'a DescriptorSetLayout]) -> PipelineLayoutCreateInfoBuilder<'b>
            where 'a: 'b {
        self.set_layouts = Some(set_layouts.iter().map(|h| h.handle()).collect());
        {
            let set_layouts = self.set_layouts.as_ref().unwrap();
            self.raw.pSetLayouts = set_layouts.as_ptr();
            assert!(self.raw.setLayoutCount == 0 || self.raw.setLayoutCount == set_layouts.len() as _, 
                "count inconsistency found when specifying `PipelineLayoutCreateInfo::set_layouts`.");
            self.raw.setLayoutCount = set_layouts.len() as _;
        }
        self
    }

    pub fn push_constant_ranges<'m, 'a>(mut self, push_constant_ranges: &'a [PushConstantRange]) -> PipelineLayoutCreateInfoBuilder<'b> {
        assert!(self.raw.pushConstantRangeCount == 0 || self.raw.pushConstantRangeCount == push_constant_ranges.len() as _, 
            "count inconsistency found when specifying `PipelineLayoutCreateInfo::push_constant_ranges`.");
        self.raw.pushConstantRangeCount = push_constant_ranges.len() as _;
        self.raw.pPushConstantRanges = push_constant_ranges.as_ptr() as *const vks::VkPushConstantRange as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineLayoutCreateFlags {
        PipelineLayoutCreateFlags::from_bits(self.raw.flags)
            .expect("PipelineLayoutCreateInfo::flags: error converting flags")
    }

    pub fn get_set_layouts_handle<'a>(&'a self) -> &'a [vks::VkDescriptorSetLayout] {
        unsafe { slice::from_raw_parts(self.raw.pSetLayouts as *const _, self.raw.setLayoutCount as usize) }
    }

    pub fn get_push_constant_ranges<'a>(&'a self) -> &'a [PushConstantRange] {
        unsafe { slice::from_raw_parts(self.raw.pPushConstantRanges as *const _, self.raw.pushConstantRangeCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> SamplerCreateFlags {
        SamplerCreateFlags::from_bits(self.raw.flags)
            .expect("SamplerCreateInfo::flags: error converting flags")
    }

    pub fn mag_filter<'a>(&'a self) -> Filter {
        self.raw.magFilter.into()
    }

    pub fn min_filter<'a>(&'a self) -> Filter {
        self.raw.minFilter.into()
    }

    pub fn mipmap_mode<'a>(&'a self) -> SamplerMipmapMode {
        self.raw.mipmapMode.into()
    }

    pub fn address_mode_u<'a>(&'a self) -> SamplerAddressMode {
        self.raw.addressModeU.into()
    }

    pub fn address_mode_v<'a>(&'a self) -> SamplerAddressMode {
        self.raw.addressModeV.into()
    }

    pub fn address_mode_w<'a>(&'a self) -> SamplerAddressMode {
        self.raw.addressModeW.into()
    }

    pub fn mip_lod_bias<'a>(&'a self) -> f32 {
        self.raw.mipLodBias.into()
    }

    pub fn anisotropy_enable<'a>(&'a self) -> bool {
        self.raw.anisotropyEnable != 0
    }

    pub fn max_anisotropy<'a>(&'a self) -> f32 {
        self.raw.maxAnisotropy.into()
    }

    pub fn compare_enable<'a>(&'a self) -> bool {
        self.raw.compareEnable != 0
    }

    pub fn compare_op<'a>(&'a self) -> CompareOp {
        self.raw.compareOp.into()
    }

    pub fn min_lod<'a>(&'a self) -> f32 {
        self.raw.minLod.into()
    }

    pub fn max_lod<'a>(&'a self) -> f32 {
        self.raw.maxLod.into()
    }

    pub fn border_color<'a>(&'a self) -> BorderColor {
        self.raw.borderColor.into()
    }

    pub fn unnormalized_coordinates<'a>(&'a self) -> bool {
        self.raw.unnormalizedCoordinates != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: SamplerCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_mag_filter<'m>(&'s mut self, mag_filter: Filter) {
        self.raw.magFilter = mag_filter.into();
    }

    pub fn set_min_filter<'m>(&'s mut self, min_filter: Filter) {
        self.raw.minFilter = min_filter.into();
    }

    pub fn set_mipmap_mode<'m>(&'s mut self, mipmap_mode: SamplerMipmapMode) {
        self.raw.mipmapMode = mipmap_mode.into();
    }

    pub fn set_address_mode_u<'m>(&'s mut self, address_mode_u: SamplerAddressMode) {
        self.raw.addressModeU = address_mode_u.into();
    }

    pub fn set_address_mode_v<'m>(&'s mut self, address_mode_v: SamplerAddressMode) {
        self.raw.addressModeV = address_mode_v.into();
    }

    pub fn set_address_mode_w<'m>(&'s mut self, address_mode_w: SamplerAddressMode) {
        self.raw.addressModeW = address_mode_w.into();
    }

    pub fn set_mip_lod_bias<'m>(&'s mut self, mip_lod_bias: f32) {
        self.raw.mipLodBias = mip_lod_bias.into();
    }

    pub fn set_anisotropy_enable<'m>(&'s mut self, anisotropy_enable: bool) {
        self.raw.anisotropyEnable = anisotropy_enable as u32;
    }

    pub fn set_max_anisotropy<'m>(&'s mut self, max_anisotropy: f32) {
        self.raw.maxAnisotropy = max_anisotropy.into();
    }

    pub fn set_compare_enable<'m>(&'s mut self, compare_enable: bool) {
        self.raw.compareEnable = compare_enable as u32;
    }

    pub fn set_compare_op<'m>(&'s mut self, compare_op: CompareOp) {
        self.raw.compareOp = compare_op.into();
    }

    pub fn set_min_lod<'m>(&'s mut self, min_lod: f32) {
        self.raw.minLod = min_lod.into();
    }

    pub fn set_max_lod<'m>(&'s mut self, max_lod: f32) {
        self.raw.maxLod = max_lod.into();
    }

    pub fn set_border_color<'m>(&'s mut self, border_color: BorderColor) {
        self.raw.borderColor = border_color.into();
    }

    pub fn set_unnormalized_coordinates<'m>(&'s mut self, unnormalized_coordinates: bool) {
        self.raw.unnormalizedCoordinates = unnormalized_coordinates as u32;
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


impl<'s> From<vks::VkSamplerCreateInfo> for SamplerCreateInfo<'s> {
    fn from(f: vks::VkSamplerCreateInfo) -> SamplerCreateInfo<'s> {
        SamplerCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> SamplerCreateFlags {
        SamplerCreateFlags::from_bits(self.raw.flags)
            .expect("SamplerCreateInfo::flags: error converting flags")
    }

    pub fn get_mag_filter<'a>(&'a self) -> Filter {
        self.raw.magFilter.into()
    }

    pub fn get_min_filter<'a>(&'a self) -> Filter {
        self.raw.minFilter.into()
    }

    pub fn get_mipmap_mode<'a>(&'a self) -> SamplerMipmapMode {
        self.raw.mipmapMode.into()
    }

    pub fn get_address_mode_u<'a>(&'a self) -> SamplerAddressMode {
        self.raw.addressModeU.into()
    }

    pub fn get_address_mode_v<'a>(&'a self) -> SamplerAddressMode {
        self.raw.addressModeV.into()
    }

    pub fn get_address_mode_w<'a>(&'a self) -> SamplerAddressMode {
        self.raw.addressModeW.into()
    }

    pub fn get_mip_lod_bias<'a>(&'a self) -> f32 {
        self.raw.mipLodBias.into()
    }

    pub fn get_anisotropy_enable<'a>(&'a self) -> bool {
        self.raw.anisotropyEnable != 0
    }

    pub fn get_max_anisotropy<'a>(&'a self) -> f32 {
        self.raw.maxAnisotropy.into()
    }

    pub fn get_compare_enable<'a>(&'a self) -> bool {
        self.raw.compareEnable != 0
    }

    pub fn get_compare_op<'a>(&'a self) -> CompareOp {
        self.raw.compareOp.into()
    }

    pub fn get_min_lod<'a>(&'a self) -> f32 {
        self.raw.minLod.into()
    }

    pub fn get_max_lod<'a>(&'a self) -> f32 {
        self.raw.maxLod.into()
    }

    pub fn get_border_color<'a>(&'a self) -> BorderColor {
        self.raw.borderColor.into()
    }

    pub fn get_unnormalized_coordinates<'a>(&'a self) -> bool {
        self.raw.unnormalizedCoordinates != 0
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> CommandPoolCreateFlags {
        CommandPoolCreateFlags::from_bits(self.raw.flags)
            .expect("CommandPoolCreateInfo::flags: error converting flags")
    }

    pub fn queue_family_index<'a>(&'a self) -> u32 {
        self.raw.queueFamilyIndex.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: CommandPoolCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_queue_family_index<'m>(&'s mut self, queue_family_index: u32) {
        self.raw.queueFamilyIndex = queue_family_index.into();
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


impl<'s> From<vks::VkCommandPoolCreateInfo> for CommandPoolCreateInfo<'s> {
    fn from(f: vks::VkCommandPoolCreateInfo) -> CommandPoolCreateInfo<'s> {
        CommandPoolCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> CommandPoolCreateFlags {
        CommandPoolCreateFlags::from_bits(self.raw.flags)
            .expect("CommandPoolCreateInfo::flags: error converting flags")
    }

    pub fn get_queue_family_index<'a>(&'a self) -> u32 {
        self.raw.queueFamilyIndex.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn command_pool_handle<'a>(&'a self) -> vks::VkCommandPool {
        self.raw.commandPool
    }

    pub fn level<'a>(&'a self) -> CommandBufferLevel {
        self.raw.level.into()
    }

    pub fn command_buffer_count<'a>(&'a self) -> u32 {
        self.raw.commandBufferCount.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_command_pool<'m, 'a>(&'s mut self, command_pool: &'a CommandPool) {
        self.raw.commandPool = command_pool.handle();
    }

    pub fn set_level<'m>(&'s mut self, level: CommandBufferLevel) {
        self.raw.level = level.into();
    }

    pub fn set_command_buffer_count<'m>(&'s mut self, command_buffer_count: u32) {
        self.raw.commandBufferCount = command_buffer_count.into();
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


impl<'s> From<vks::VkCommandBufferAllocateInfo> for CommandBufferAllocateInfo<'s> {
    fn from(f: vks::VkCommandBufferAllocateInfo) -> CommandBufferAllocateInfo<'s> {
        CommandBufferAllocateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_command_pool_handle<'a>(&'a self) -> vks::VkCommandPool {
        self.raw.commandPool
    }

    pub fn get_level<'a>(&'a self) -> CommandBufferLevel {
        self.raw.level.into()
    }

    pub fn get_command_buffer_count<'a>(&'a self) -> u32 {
        self.raw.commandBufferCount.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn subpass<'a>(&'a self) -> u32 {
        self.raw.subpass.into()
    }

    pub fn framebuffer_handle<'a>(&'a self) -> vks::VkFramebuffer {
        self.raw.framebuffer
    }

    pub fn occlusion_query_enable<'a>(&'a self) -> bool {
        self.raw.occlusionQueryEnable != 0
    }

    pub fn query_flags<'a>(&'a self) -> QueryControlFlags {
        QueryControlFlags::from_bits(self.raw.queryFlags)
            .expect("CommandBufferInheritanceInfo::query_flags: error converting flags")
    }

    pub fn pipeline_statistics<'a>(&'a self) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags::from_bits(self.raw.pipelineStatistics)
            .expect("CommandBufferInheritanceInfo::pipeline_statistics: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_render_pass<'m, 'a>(&'s mut self, render_pass: &'a RenderPass) {
        self.raw.renderPass = render_pass.handle();
    }

    pub fn set_subpass<'m>(&'s mut self, subpass: u32) {
        self.raw.subpass = subpass.into();
    }

    pub fn set_framebuffer<'m, 'a>(&'s mut self, framebuffer: &'a Framebuffer) {
        self.raw.framebuffer = framebuffer.handle();
    }

    pub fn set_occlusion_query_enable<'m>(&'s mut self, occlusion_query_enable: bool) {
        self.raw.occlusionQueryEnable = occlusion_query_enable as u32;
    }

    pub fn set_query_flags<'m>(&'s mut self, query_flags: QueryControlFlags) {
        self.raw.queryFlags = query_flags.bits();
    }

    pub fn set_pipeline_statistics<'m>(&'s mut self, pipeline_statistics: QueryPipelineStatisticFlags) {
        self.raw.pipelineStatistics = pipeline_statistics.bits();
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


impl<'s> From<vks::VkCommandBufferInheritanceInfo> for CommandBufferInheritanceInfo<'s> {
    fn from(f: vks::VkCommandBufferInheritanceInfo) -> CommandBufferInheritanceInfo<'s> {
        CommandBufferInheritanceInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn get_subpass<'a>(&'a self) -> u32 {
        self.raw.subpass.into()
    }

    pub fn get_framebuffer_handle<'a>(&'a self) -> vks::VkFramebuffer {
        self.raw.framebuffer
    }

    pub fn get_occlusion_query_enable<'a>(&'a self) -> bool {
        self.raw.occlusionQueryEnable != 0
    }

    pub fn get_query_flags<'a>(&'a self) -> QueryControlFlags {
        QueryControlFlags::from_bits(self.raw.queryFlags)
            .expect("CommandBufferInheritanceInfo::query_flags: error converting flags")
    }

    pub fn get_pipeline_statistics<'a>(&'a self) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags::from_bits(self.raw.pipelineStatistics)
            .expect("CommandBufferInheritanceInfo::pipeline_statistics: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> CommandBufferUsageFlags {
        CommandBufferUsageFlags::from_bits(self.raw.flags)
            .expect("CommandBufferBeginInfo::flags: error converting flags")
    }

    pub fn inheritance_info<'a>(&'a self) -> &'a CommandBufferInheritanceInfo {
        unsafe { &*(self.raw.pInheritanceInfo as *const vks::VkCommandBufferInheritanceInfo as *const _) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: CommandBufferUsageFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_inheritance_info<'m, 'a>(&'s mut self, inheritance_info: &'a CommandBufferInheritanceInfo) {
        self.raw.pInheritanceInfo = inheritance_info.raw();
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


impl<'s> From<vks::VkCommandBufferBeginInfo> for CommandBufferBeginInfo<'s> {
    fn from(f: vks::VkCommandBufferBeginInfo) -> CommandBufferBeginInfo<'s> {
        CommandBufferBeginInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> CommandBufferUsageFlags {
        CommandBufferUsageFlags::from_bits(self.raw.flags)
            .expect("CommandBufferBeginInfo::flags: error converting flags")
    }

    pub fn get_inheritance_info<'a>(&'a self) -> &'a CommandBufferInheritanceInfo {
        unsafe { &*(self.raw.pInheritanceInfo as *const vks::VkCommandBufferInheritanceInfo as *const _) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn framebuffer_handle<'a>(&'a self) -> vks::VkFramebuffer {
        self.raw.framebuffer
    }

    pub fn render_area<'a>(&'a self) -> Rect2d {
        self.raw.renderArea.into()
    }

    pub fn clear_values<'a>(&'a self) -> &'a [ClearValue] {
        unsafe { slice::from_raw_parts(self.raw.pClearValues as *const _, self.raw.clearValueCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_render_pass<'m, 'a>(&'s mut self, render_pass: &'a RenderPass) {
        self.raw.renderPass = render_pass.handle();
    }

    pub fn set_framebuffer<'m, 'a>(&'s mut self, framebuffer: &'a Framebuffer) {
        self.raw.framebuffer = framebuffer.handle();
    }

    pub fn set_render_area<'m>(&'s mut self, render_area: Rect2d) {
        self.raw.renderArea = render_area.raw;
    }

    pub fn set_clear_values<'m, 'a>(&'s mut self, clear_values: &'a [ClearValue]) {
        assert!(self.raw.clearValueCount == 0 || self.raw.clearValueCount == clear_values.len() as _, 
            "count inconsistency found when specifying `RenderPassBeginInfo::clear_values`.");
        self.raw.clearValueCount = clear_values.len() as _;
        self.raw.pClearValues = clear_values.as_ptr() as *const ClearValue as *const _;
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


impl<'s> From<vks::VkRenderPassBeginInfo> for RenderPassBeginInfo<'s> {
    fn from(f: vks::VkRenderPassBeginInfo) -> RenderPassBeginInfo<'s> {
        RenderPassBeginInfo { raw: f, _p: PhantomData }
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

    pub fn clear_values<'m, 'a>(mut self, clear_values: &'a [ClearValue]) -> RenderPassBeginInfoBuilder<'b> {
        assert!(self.raw.clearValueCount == 0 || self.raw.clearValueCount == clear_values.len() as _, 
            "count inconsistency found when specifying `RenderPassBeginInfo::clear_values`.");
        self.raw.clearValueCount = clear_values.len() as _;
        self.raw.pClearValues = clear_values.as_ptr() as *const ClearValue as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn get_framebuffer_handle<'a>(&'a self) -> vks::VkFramebuffer {
        self.raw.framebuffer
    }

    pub fn get_render_area<'a>(&'a self) -> Rect2d {
        self.raw.renderArea.into()
    }

    pub fn get_clear_values<'a>(&'a self) -> &'a [ClearValue] {
        unsafe { slice::from_raw_parts(self.raw.pClearValues as *const _, self.raw.clearValueCount as usize) }
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

    pub fn depth<'a>(&'a self) -> f32 {
        self.raw.depth.into()
    }

    pub fn stencil<'a>(&'a self) -> u32 {
        self.raw.stencil.into()
    }

    pub fn set_depth<'m>(& mut self, depth: f32) {
        self.raw.depth = depth.into();
    }

    pub fn set_stencil<'m>(& mut self, stencil: u32) {
        self.raw.stencil = stencil.into();
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


impl From<vks::VkClearDepthStencilValue> for ClearDepthStencilValue {
    fn from(f: vks::VkClearDepthStencilValue) -> ClearDepthStencilValue {
        ClearDepthStencilValue { raw: f, }
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

    pub fn get_depth<'a>(&'a self) -> f32 {
        self.raw.depth.into()
    }

    pub fn get_stencil<'a>(&'a self) -> u32 {
        self.raw.stencil.into()
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

    pub fn aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ClearAttachment::aspect_mask: error converting flags")
    }

    pub fn color_attachment<'a>(&'a self) -> u32 {
        self.raw.colorAttachment.into()
    }

    pub fn clear_value<'a>(&'a self) -> ClearValue {
        self.raw.clearValue.into()
    }

    pub fn set_aspect_mask<'m>(& mut self, aspect_mask: ImageAspectFlags) {
        self.raw.aspectMask = aspect_mask.bits();
    }

    pub fn set_color_attachment<'m>(& mut self, color_attachment: u32) {
        self.raw.colorAttachment = color_attachment.into();
    }

    pub fn set_clear_value<'m>(& mut self, clear_value: ClearValue) {
        self.raw.clearValue = clear_value.into();
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


impl From<vks::VkClearAttachment> for ClearAttachment {
    fn from(f: vks::VkClearAttachment) -> ClearAttachment {
        ClearAttachment { raw: f, }
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

    pub fn get_aspect_mask<'a>(&'a self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits(self.raw.aspectMask)
            .expect("ClearAttachment::aspect_mask: error converting flags")
    }

    pub fn get_color_attachment<'a>(&'a self) -> u32 {
        self.raw.colorAttachment.into()
    }

    pub fn get_clear_value<'a>(&'a self) -> ClearValue {
        self.raw.clearValue.into()
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

    pub fn flags<'a>(&'a self) -> AttachmentDescriptionFlags {
        AttachmentDescriptionFlags::from_bits(self.raw.flags)
            .expect("AttachmentDescription::flags: error converting flags")
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.samples)
            .expect("AttachmentDescription::samples: error converting flags")
    }

    pub fn load_op<'a>(&'a self) -> AttachmentLoadOp {
        self.raw.loadOp.into()
    }

    pub fn store_op<'a>(&'a self) -> AttachmentStoreOp {
        self.raw.storeOp.into()
    }

    pub fn stencil_load_op<'a>(&'a self) -> AttachmentLoadOp {
        self.raw.stencilLoadOp.into()
    }

    pub fn stencil_store_op<'a>(&'a self) -> AttachmentStoreOp {
        self.raw.stencilStoreOp.into()
    }

    pub fn initial_layout<'a>(&'a self) -> ImageLayout {
        self.raw.initialLayout.into()
    }

    pub fn final_layout<'a>(&'a self) -> ImageLayout {
        self.raw.finalLayout.into()
    }

    pub fn set_flags<'m>(& mut self, flags: AttachmentDescriptionFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_format<'m>(& mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_samples<'m>(& mut self, samples: SampleCountFlags) {
        self.raw.samples = samples.bits();
    }

    pub fn set_load_op<'m>(& mut self, load_op: AttachmentLoadOp) {
        self.raw.loadOp = load_op.into();
    }

    pub fn set_store_op<'m>(& mut self, store_op: AttachmentStoreOp) {
        self.raw.storeOp = store_op.into();
    }

    pub fn set_stencil_load_op<'m>(& mut self, stencil_load_op: AttachmentLoadOp) {
        self.raw.stencilLoadOp = stencil_load_op.into();
    }

    pub fn set_stencil_store_op<'m>(& mut self, stencil_store_op: AttachmentStoreOp) {
        self.raw.stencilStoreOp = stencil_store_op.into();
    }

    pub fn set_initial_layout<'m>(& mut self, initial_layout: ImageLayout) {
        self.raw.initialLayout = initial_layout.into();
    }

    pub fn set_final_layout<'m>(& mut self, final_layout: ImageLayout) {
        self.raw.finalLayout = final_layout.into();
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


impl From<vks::VkAttachmentDescription> for AttachmentDescription {
    fn from(f: vks::VkAttachmentDescription) -> AttachmentDescription {
        AttachmentDescription { raw: f, }
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

    pub fn get_flags<'a>(&'a self) -> AttachmentDescriptionFlags {
        AttachmentDescriptionFlags::from_bits(self.raw.flags)
            .expect("AttachmentDescription::flags: error converting flags")
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.samples)
            .expect("AttachmentDescription::samples: error converting flags")
    }

    pub fn get_load_op<'a>(&'a self) -> AttachmentLoadOp {
        self.raw.loadOp.into()
    }

    pub fn get_store_op<'a>(&'a self) -> AttachmentStoreOp {
        self.raw.storeOp.into()
    }

    pub fn get_stencil_load_op<'a>(&'a self) -> AttachmentLoadOp {
        self.raw.stencilLoadOp.into()
    }

    pub fn get_stencil_store_op<'a>(&'a self) -> AttachmentStoreOp {
        self.raw.stencilStoreOp.into()
    }

    pub fn get_initial_layout<'a>(&'a self) -> ImageLayout {
        self.raw.initialLayout.into()
    }

    pub fn get_final_layout<'a>(&'a self) -> ImageLayout {
        self.raw.finalLayout.into()
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

    pub fn attachment<'a>(&'a self) -> u32 {
        self.raw.attachment.into()
    }

    pub fn layout<'a>(&'a self) -> ImageLayout {
        self.raw.layout.into()
    }

    pub fn set_attachment<'m>(& mut self, attachment: u32) {
        self.raw.attachment = attachment.into();
    }

    pub fn set_layout<'m>(& mut self, layout: ImageLayout) {
        self.raw.layout = layout.into();
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


impl From<vks::VkAttachmentReference> for AttachmentReference {
    fn from(f: vks::VkAttachmentReference) -> AttachmentReference {
        AttachmentReference { raw: f, }
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

    pub fn get_attachment<'a>(&'a self) -> u32 {
        self.raw.attachment.into()
    }

    pub fn get_layout<'a>(&'a self) -> ImageLayout {
        self.raw.layout.into()
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

    pub fn flags<'a>(&'a self) -> SubpassDescriptionFlags {
        SubpassDescriptionFlags::from_bits(self.raw.flags)
            .expect("SubpassDescription::flags: error converting flags")
    }

    pub fn pipeline_bind_point<'a>(&'a self) -> PipelineBindPoint {
        self.raw.pipelineBindPoint.into()
    }

    pub fn input_attachments<'a>(&'a self) -> &'a [AttachmentReference] {
        unsafe { slice::from_raw_parts(self.raw.pInputAttachments as *const _, self.raw.inputAttachmentCount as usize) }
    }

    pub fn color_attachments<'a>(&'a self) -> &'a [AttachmentReference] {
        unsafe { slice::from_raw_parts(self.raw.pColorAttachments as *const _, self.raw.colorAttachmentCount as usize) }
    }

    pub fn resolve_attachments<'a>(&'a self) -> &'a [AttachmentReference] {
        unsafe { slice::from_raw_parts(self.raw.pResolveAttachments as *const _, self.raw.colorAttachmentCount as usize) }
    }

    pub fn depth_stencil_attachment<'a>(&'a self) -> &'a AttachmentReference {
        unsafe { &*(self.raw.pDepthStencilAttachment as *const vks::VkAttachmentReference as *const _) }
    }

    pub fn preserve_attachments<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pPreserveAttachments as *const _, self.raw.preserveAttachmentCount as usize) }
    }

    pub fn set_flags<'m>(&'s mut self, flags: SubpassDescriptionFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_pipeline_bind_point<'m>(&'s mut self, pipeline_bind_point: PipelineBindPoint) {
        self.raw.pipelineBindPoint = pipeline_bind_point.into();
    }

    pub fn set_input_attachments<'m, 'a>(&'s mut self, input_attachments: &'a [AttachmentReference]) {
        assert!(self.raw.inputAttachmentCount == 0 || self.raw.inputAttachmentCount == input_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::input_attachments`.");
        self.raw.inputAttachmentCount = input_attachments.len() as _;
        self.raw.pInputAttachments = input_attachments.as_ptr() as *const vks::VkAttachmentReference as *const _;
    }

    pub fn set_color_attachments<'m, 'a>(&'s mut self, color_attachments: &'a [AttachmentReference]) {
        assert!(self.raw.colorAttachmentCount == 0 || self.raw.colorAttachmentCount == color_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::color_attachments`.");
        self.raw.colorAttachmentCount = color_attachments.len() as _;
        self.raw.pColorAttachments = color_attachments.as_ptr() as *const vks::VkAttachmentReference as *const _;
    }

    pub fn set_resolve_attachments<'m, 'a>(&'s mut self, resolve_attachments: &'a [AttachmentReference]) {
        assert!(self.raw.colorAttachmentCount == 0 || self.raw.colorAttachmentCount == resolve_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::resolve_attachments`.");
        self.raw.colorAttachmentCount = resolve_attachments.len() as _;
        self.raw.pResolveAttachments = resolve_attachments.as_ptr() as *const vks::VkAttachmentReference as *const _;
    }

    pub fn set_depth_stencil_attachment<'m, 'a>(&'s mut self, depth_stencil_attachment: &'a AttachmentReference) {
        self.raw.pDepthStencilAttachment = depth_stencil_attachment.raw();
    }

    pub fn set_preserve_attachments<'m, 'a>(&'s mut self, preserve_attachments: &'a [u32]) {
        assert!(self.raw.preserveAttachmentCount == 0 || self.raw.preserveAttachmentCount == preserve_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::preserve_attachments`.");
        self.raw.preserveAttachmentCount = preserve_attachments.len() as _;
        self.raw.pPreserveAttachments = preserve_attachments.as_ptr() as *const u32 as *const _;
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


impl<'s> From<vks::VkSubpassDescription> for SubpassDescription<'s> {
    fn from(f: vks::VkSubpassDescription) -> SubpassDescription<'s> {
        SubpassDescription { raw: f, _p: PhantomData }
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

    pub fn input_attachments<'m, 'a>(mut self, input_attachments: &'a [AttachmentReference]) -> SubpassDescriptionBuilder<'b> {
        assert!(self.raw.inputAttachmentCount == 0 || self.raw.inputAttachmentCount == input_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::input_attachments`.");
        self.raw.inputAttachmentCount = input_attachments.len() as _;
        self.raw.pInputAttachments = input_attachments.as_ptr() as *const vks::VkAttachmentReference as *const _;
        self
    }

    pub fn color_attachments<'m, 'a>(mut self, color_attachments: &'a [AttachmentReference]) -> SubpassDescriptionBuilder<'b> {
        assert!(self.raw.colorAttachmentCount == 0 || self.raw.colorAttachmentCount == color_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::color_attachments`.");
        self.raw.colorAttachmentCount = color_attachments.len() as _;
        self.raw.pColorAttachments = color_attachments.as_ptr() as *const vks::VkAttachmentReference as *const _;
        self
    }

    pub fn resolve_attachments<'m, 'a>(mut self, resolve_attachments: &'a [AttachmentReference]) -> SubpassDescriptionBuilder<'b> {
        assert!(self.raw.colorAttachmentCount == 0 || self.raw.colorAttachmentCount == resolve_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::resolve_attachments`.");
        self.raw.colorAttachmentCount = resolve_attachments.len() as _;
        self.raw.pResolveAttachments = resolve_attachments.as_ptr() as *const vks::VkAttachmentReference as *const _;
        self
    }

    pub fn depth_stencil_attachment<'m, 'a>(mut self, depth_stencil_attachment: &'a AttachmentReference) -> SubpassDescriptionBuilder<'b> {
        self.raw.pDepthStencilAttachment = depth_stencil_attachment.raw();
        self
    }

    pub fn preserve_attachments<'m, 'a>(mut self, preserve_attachments: &'a [u32]) -> SubpassDescriptionBuilder<'b> {
        assert!(self.raw.preserveAttachmentCount == 0 || self.raw.preserveAttachmentCount == preserve_attachments.len() as _, 
            "count inconsistency found when specifying `SubpassDescription::preserve_attachments`.");
        self.raw.preserveAttachmentCount = preserve_attachments.len() as _;
        self.raw.pPreserveAttachments = preserve_attachments.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn get_flags<'a>(&'a self) -> SubpassDescriptionFlags {
        SubpassDescriptionFlags::from_bits(self.raw.flags)
            .expect("SubpassDescription::flags: error converting flags")
    }

    pub fn get_pipeline_bind_point<'a>(&'a self) -> PipelineBindPoint {
        self.raw.pipelineBindPoint.into()
    }

    pub fn get_input_attachments<'a>(&'a self) -> &'a [AttachmentReference] {
        unsafe { slice::from_raw_parts(self.raw.pInputAttachments as *const _, self.raw.inputAttachmentCount as usize) }
    }

    pub fn get_color_attachments<'a>(&'a self) -> &'a [AttachmentReference] {
        unsafe { slice::from_raw_parts(self.raw.pColorAttachments as *const _, self.raw.colorAttachmentCount as usize) }
    }

    pub fn get_resolve_attachments<'a>(&'a self) -> &'a [AttachmentReference] {
        unsafe { slice::from_raw_parts(self.raw.pResolveAttachments as *const _, self.raw.colorAttachmentCount as usize) }
    }

    pub fn get_depth_stencil_attachment<'a>(&'a self) -> &'a AttachmentReference {
        unsafe { &*(self.raw.pDepthStencilAttachment as *const vks::VkAttachmentReference as *const _) }
    }

    pub fn get_preserve_attachments<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pPreserveAttachments as *const _, self.raw.preserveAttachmentCount as usize) }
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

    pub fn src_subpass<'a>(&'a self) -> u32 {
        self.raw.srcSubpass.into()
    }

    pub fn dst_subpass<'a>(&'a self) -> u32 {
        self.raw.dstSubpass.into()
    }

    pub fn src_stage_mask<'a>(&'a self) -> PipelineStageFlags {
        PipelineStageFlags::from_bits(self.raw.srcStageMask)
            .expect("SubpassDependency::src_stage_mask: error converting flags")
    }

    pub fn dst_stage_mask<'a>(&'a self) -> PipelineStageFlags {
        PipelineStageFlags::from_bits(self.raw.dstStageMask)
            .expect("SubpassDependency::dst_stage_mask: error converting flags")
    }

    pub fn src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("SubpassDependency::src_access_mask: error converting flags")
    }

    pub fn dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("SubpassDependency::dst_access_mask: error converting flags")
    }

    pub fn dependency_flags<'a>(&'a self) -> DependencyFlags {
        DependencyFlags::from_bits(self.raw.dependencyFlags)
            .expect("SubpassDependency::dependency_flags: error converting flags")
    }

    pub fn set_src_subpass<'m>(& mut self, src_subpass: u32) {
        self.raw.srcSubpass = src_subpass.into();
    }

    pub fn set_dst_subpass<'m>(& mut self, dst_subpass: u32) {
        self.raw.dstSubpass = dst_subpass.into();
    }

    pub fn set_src_stage_mask<'m>(& mut self, src_stage_mask: PipelineStageFlags) {
        self.raw.srcStageMask = src_stage_mask.bits();
    }

    pub fn set_dst_stage_mask<'m>(& mut self, dst_stage_mask: PipelineStageFlags) {
        self.raw.dstStageMask = dst_stage_mask.bits();
    }

    pub fn set_src_access_mask<'m>(& mut self, src_access_mask: AccessFlags) {
        self.raw.srcAccessMask = src_access_mask.bits();
    }

    pub fn set_dst_access_mask<'m>(& mut self, dst_access_mask: AccessFlags) {
        self.raw.dstAccessMask = dst_access_mask.bits();
    }

    pub fn set_dependency_flags<'m>(& mut self, dependency_flags: DependencyFlags) {
        self.raw.dependencyFlags = dependency_flags.bits();
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


impl From<vks::VkSubpassDependency> for SubpassDependency {
    fn from(f: vks::VkSubpassDependency) -> SubpassDependency {
        SubpassDependency { raw: f, }
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

    pub fn get_src_subpass<'a>(&'a self) -> u32 {
        self.raw.srcSubpass.into()
    }

    pub fn get_dst_subpass<'a>(&'a self) -> u32 {
        self.raw.dstSubpass.into()
    }

    pub fn get_src_stage_mask<'a>(&'a self) -> PipelineStageFlags {
        PipelineStageFlags::from_bits(self.raw.srcStageMask)
            .expect("SubpassDependency::src_stage_mask: error converting flags")
    }

    pub fn get_dst_stage_mask<'a>(&'a self) -> PipelineStageFlags {
        PipelineStageFlags::from_bits(self.raw.dstStageMask)
            .expect("SubpassDependency::dst_stage_mask: error converting flags")
    }

    pub fn get_src_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.srcAccessMask)
            .expect("SubpassDependency::src_access_mask: error converting flags")
    }

    pub fn get_dst_access_mask<'a>(&'a self) -> AccessFlags {
        AccessFlags::from_bits(self.raw.dstAccessMask)
            .expect("SubpassDependency::dst_access_mask: error converting flags")
    }

    pub fn get_dependency_flags<'a>(&'a self) -> DependencyFlags {
        DependencyFlags::from_bits(self.raw.dependencyFlags)
            .expect("SubpassDependency::dependency_flags: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> RenderPassCreateFlags {
        RenderPassCreateFlags::from_bits(self.raw.flags)
            .expect("RenderPassCreateInfo::flags: error converting flags")
    }

    pub fn attachments<'a>(&'a self) -> &'a [AttachmentDescription] {
        unsafe { slice::from_raw_parts(self.raw.pAttachments as *const _, self.raw.attachmentCount as usize) }
    }

    pub fn subpasses<'a>(&'a self) -> &'a [SubpassDescription] {
        unsafe { slice::from_raw_parts(self.raw.pSubpasses as *const _, self.raw.subpassCount as usize) }
    }

    pub fn dependencies<'a>(&'a self) -> &'a [SubpassDependency] {
        unsafe { slice::from_raw_parts(self.raw.pDependencies as *const _, self.raw.dependencyCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: RenderPassCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_attachments<'m, 'a>(&'s mut self, attachments: &'a [AttachmentDescription]) {
        assert!(self.raw.attachmentCount == 0 || self.raw.attachmentCount == attachments.len() as _, 
            "count inconsistency found when specifying `RenderPassCreateInfo::attachments`.");
        self.raw.attachmentCount = attachments.len() as _;
        self.raw.pAttachments = attachments.as_ptr() as *const vks::VkAttachmentDescription as *const _;
    }

    pub fn set_subpasses<'m, 'a>(&'s mut self, subpasses: &'a [SubpassDescription]) {
        assert!(self.raw.subpassCount == 0 || self.raw.subpassCount == subpasses.len() as _, 
            "count inconsistency found when specifying `RenderPassCreateInfo::subpasses`.");
        self.raw.subpassCount = subpasses.len() as _;
        self.raw.pSubpasses = subpasses.as_ptr() as *const vks::VkSubpassDescription as *const _;
    }

    pub fn set_dependencies<'m, 'a>(&'s mut self, dependencies: &'a [SubpassDependency]) {
        assert!(self.raw.dependencyCount == 0 || self.raw.dependencyCount == dependencies.len() as _, 
            "count inconsistency found when specifying `RenderPassCreateInfo::dependencies`.");
        self.raw.dependencyCount = dependencies.len() as _;
        self.raw.pDependencies = dependencies.as_ptr() as *const vks::VkSubpassDependency as *const _;
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


impl<'s> From<vks::VkRenderPassCreateInfo> for RenderPassCreateInfo<'s> {
    fn from(f: vks::VkRenderPassCreateInfo) -> RenderPassCreateInfo<'s> {
        RenderPassCreateInfo { raw: f, _p: PhantomData }
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

    pub fn attachments<'m, 'a>(mut self, attachments: &'a [AttachmentDescription]) -> RenderPassCreateInfoBuilder<'b> {
        assert!(self.raw.attachmentCount == 0 || self.raw.attachmentCount == attachments.len() as _, 
            "count inconsistency found when specifying `RenderPassCreateInfo::attachments`.");
        self.raw.attachmentCount = attachments.len() as _;
        self.raw.pAttachments = attachments.as_ptr() as *const vks::VkAttachmentDescription as *const _;
        self
    }

    pub fn subpasses<'m, 'a>(mut self, subpasses: &'a [SubpassDescription]) -> RenderPassCreateInfoBuilder<'b> {
        assert!(self.raw.subpassCount == 0 || self.raw.subpassCount == subpasses.len() as _, 
            "count inconsistency found when specifying `RenderPassCreateInfo::subpasses`.");
        self.raw.subpassCount = subpasses.len() as _;
        self.raw.pSubpasses = subpasses.as_ptr() as *const vks::VkSubpassDescription as *const _;
        self
    }

    pub fn dependencies<'m, 'a>(mut self, dependencies: &'a [SubpassDependency]) -> RenderPassCreateInfoBuilder<'b> {
        assert!(self.raw.dependencyCount == 0 || self.raw.dependencyCount == dependencies.len() as _, 
            "count inconsistency found when specifying `RenderPassCreateInfo::dependencies`.");
        self.raw.dependencyCount = dependencies.len() as _;
        self.raw.pDependencies = dependencies.as_ptr() as *const vks::VkSubpassDependency as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> RenderPassCreateFlags {
        RenderPassCreateFlags::from_bits(self.raw.flags)
            .expect("RenderPassCreateInfo::flags: error converting flags")
    }

    pub fn get_attachments<'a>(&'a self) -> &'a [AttachmentDescription] {
        unsafe { slice::from_raw_parts(self.raw.pAttachments as *const _, self.raw.attachmentCount as usize) }
    }

    pub fn get_subpasses<'a>(&'a self) -> &'a [SubpassDescription] {
        unsafe { slice::from_raw_parts(self.raw.pSubpasses as *const _, self.raw.subpassCount as usize) }
    }

    pub fn get_dependencies<'a>(&'a self) -> &'a [SubpassDependency] {
        unsafe { slice::from_raw_parts(self.raw.pDependencies as *const _, self.raw.dependencyCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> EventCreateFlags {
        EventCreateFlags::from_bits(self.raw.flags)
            .expect("EventCreateInfo::flags: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: EventCreateFlags) {
        self.raw.flags = flags.bits();
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


impl<'s> From<vks::VkEventCreateInfo> for EventCreateInfo<'s> {
    fn from(f: vks::VkEventCreateInfo) -> EventCreateInfo<'s> {
        EventCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> EventCreateFlags {
        EventCreateFlags::from_bits(self.raw.flags)
            .expect("EventCreateInfo::flags: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> FenceCreateFlags {
        FenceCreateFlags::from_bits(self.raw.flags)
            .expect("FenceCreateInfo::flags: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: FenceCreateFlags) {
        self.raw.flags = flags.bits();
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


impl<'s> From<vks::VkFenceCreateInfo> for FenceCreateInfo<'s> {
    fn from(f: vks::VkFenceCreateInfo) -> FenceCreateInfo<'s> {
        FenceCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> FenceCreateFlags {
        FenceCreateFlags::from_bits(self.raw.flags)
            .expect("FenceCreateInfo::flags: error converting flags")
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

    pub fn robust_buffer_access<'a>(&'a self) -> bool {
        self.raw.robustBufferAccess != 0
    }

    pub fn full_draw_index_uint_32<'a>(&'a self) -> bool {
        self.raw.fullDrawIndexUint32 != 0
    }

    pub fn image_cube_array<'a>(&'a self) -> bool {
        self.raw.imageCubeArray != 0
    }

    pub fn independent_blend<'a>(&'a self) -> bool {
        self.raw.independentBlend != 0
    }

    pub fn geometry_shader<'a>(&'a self) -> bool {
        self.raw.geometryShader != 0
    }

    pub fn tessellation_shader<'a>(&'a self) -> bool {
        self.raw.tessellationShader != 0
    }

    pub fn sample_rate_shading<'a>(&'a self) -> bool {
        self.raw.sampleRateShading != 0
    }

    pub fn dual_src_blend<'a>(&'a self) -> bool {
        self.raw.dualSrcBlend != 0
    }

    pub fn logic_op<'a>(&'a self) -> bool {
        self.raw.logicOp != 0
    }

    pub fn multi_draw_indirect<'a>(&'a self) -> bool {
        self.raw.multiDrawIndirect != 0
    }

    pub fn draw_indirect_first_instance<'a>(&'a self) -> bool {
        self.raw.drawIndirectFirstInstance != 0
    }

    pub fn depth_clamp<'a>(&'a self) -> bool {
        self.raw.depthClamp != 0
    }

    pub fn depth_bias_clamp<'a>(&'a self) -> bool {
        self.raw.depthBiasClamp != 0
    }

    pub fn fill_mode_non_solid<'a>(&'a self) -> bool {
        self.raw.fillModeNonSolid != 0
    }

    pub fn depth_bounds<'a>(&'a self) -> bool {
        self.raw.depthBounds != 0
    }

    pub fn wide_lines<'a>(&'a self) -> bool {
        self.raw.wideLines != 0
    }

    pub fn large_points<'a>(&'a self) -> bool {
        self.raw.largePoints != 0
    }

    pub fn alpha_to_one<'a>(&'a self) -> bool {
        self.raw.alphaToOne != 0
    }

    pub fn multi_viewport<'a>(&'a self) -> bool {
        self.raw.multiViewport != 0
    }

    pub fn sampler_anisotropy<'a>(&'a self) -> bool {
        self.raw.samplerAnisotropy != 0
    }

    pub fn texture_compression_et_c2<'a>(&'a self) -> bool {
        self.raw.textureCompressionETC2 != 0
    }

    pub fn texture_compression_as_tc_ld_r<'a>(&'a self) -> bool {
        self.raw.textureCompressionASTC_LDR != 0
    }

    pub fn texture_compression_bc<'a>(&'a self) -> bool {
        self.raw.textureCompressionBC != 0
    }

    pub fn occlusion_query_precise<'a>(&'a self) -> bool {
        self.raw.occlusionQueryPrecise != 0
    }

    pub fn pipeline_statistics_query<'a>(&'a self) -> bool {
        self.raw.pipelineStatisticsQuery != 0
    }

    pub fn vertex_pipeline_stores_and_atomics<'a>(&'a self) -> bool {
        self.raw.vertexPipelineStoresAndAtomics != 0
    }

    pub fn fragment_stores_and_atomics<'a>(&'a self) -> bool {
        self.raw.fragmentStoresAndAtomics != 0
    }

    pub fn shader_tessellation_and_geometry_point_size<'a>(&'a self) -> bool {
        self.raw.shaderTessellationAndGeometryPointSize != 0
    }

    pub fn shader_image_gather_extended<'a>(&'a self) -> bool {
        self.raw.shaderImageGatherExtended != 0
    }

    pub fn shader_storage_image_extended_formats<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageExtendedFormats != 0
    }

    pub fn shader_storage_image_multisample<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageMultisample != 0
    }

    pub fn shader_storage_image_read_without_format<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageReadWithoutFormat != 0
    }

    pub fn shader_storage_image_write_without_format<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageWriteWithoutFormat != 0
    }

    pub fn shader_uniform_buffer_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderUniformBufferArrayDynamicIndexing != 0
    }

    pub fn shader_sampled_image_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderSampledImageArrayDynamicIndexing != 0
    }

    pub fn shader_storage_buffer_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderStorageBufferArrayDynamicIndexing != 0
    }

    pub fn shader_storage_image_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageArrayDynamicIndexing != 0
    }

    pub fn shader_clip_distance<'a>(&'a self) -> bool {
        self.raw.shaderClipDistance != 0
    }

    pub fn shader_cull_distance<'a>(&'a self) -> bool {
        self.raw.shaderCullDistance != 0
    }

    pub fn shader_float_64<'a>(&'a self) -> bool {
        self.raw.shaderFloat64 != 0
    }

    pub fn shader_int_64<'a>(&'a self) -> bool {
        self.raw.shaderInt64 != 0
    }

    pub fn shader_int_16<'a>(&'a self) -> bool {
        self.raw.shaderInt16 != 0
    }

    pub fn shader_resource_residency<'a>(&'a self) -> bool {
        self.raw.shaderResourceResidency != 0
    }

    pub fn shader_resource_min_lod<'a>(&'a self) -> bool {
        self.raw.shaderResourceMinLod != 0
    }

    pub fn sparse_binding<'a>(&'a self) -> bool {
        self.raw.sparseBinding != 0
    }

    pub fn sparse_residency_buffer<'a>(&'a self) -> bool {
        self.raw.sparseResidencyBuffer != 0
    }

    pub fn sparse_residency_image_2d<'a>(&'a self) -> bool {
        self.raw.sparseResidencyImage2D != 0
    }

    pub fn sparse_residency_image_3d<'a>(&'a self) -> bool {
        self.raw.sparseResidencyImage3D != 0
    }

    pub fn sparse_residency_2samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency2Samples != 0
    }

    pub fn sparse_residency_4samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency4Samples != 0
    }

    pub fn sparse_residency_8samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency8Samples != 0
    }

    pub fn sparse_residency_16_samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency16Samples != 0
    }

    pub fn sparse_residency_aliased<'a>(&'a self) -> bool {
        self.raw.sparseResidencyAliased != 0
    }

    pub fn variable_multisample_rate<'a>(&'a self) -> bool {
        self.raw.variableMultisampleRate != 0
    }

    pub fn inherited_queries<'a>(&'a self) -> bool {
        self.raw.inheritedQueries != 0
    }

    pub fn set_robust_buffer_access<'m>(& mut self, robust_buffer_access: bool) {
        self.raw.robustBufferAccess = robust_buffer_access as u32;
    }

    pub fn set_full_draw_index_uint_32<'m>(& mut self, full_draw_index_uint_32: bool) {
        self.raw.fullDrawIndexUint32 = full_draw_index_uint_32 as u32;
    }

    pub fn set_image_cube_array<'m>(& mut self, image_cube_array: bool) {
        self.raw.imageCubeArray = image_cube_array as u32;
    }

    pub fn set_independent_blend<'m>(& mut self, independent_blend: bool) {
        self.raw.independentBlend = independent_blend as u32;
    }

    pub fn set_geometry_shader<'m>(& mut self, geometry_shader: bool) {
        self.raw.geometryShader = geometry_shader as u32;
    }

    pub fn set_tessellation_shader<'m>(& mut self, tessellation_shader: bool) {
        self.raw.tessellationShader = tessellation_shader as u32;
    }

    pub fn set_sample_rate_shading<'m>(& mut self, sample_rate_shading: bool) {
        self.raw.sampleRateShading = sample_rate_shading as u32;
    }

    pub fn set_dual_src_blend<'m>(& mut self, dual_src_blend: bool) {
        self.raw.dualSrcBlend = dual_src_blend as u32;
    }

    pub fn set_logic_op<'m>(& mut self, logic_op: bool) {
        self.raw.logicOp = logic_op as u32;
    }

    pub fn set_multi_draw_indirect<'m>(& mut self, multi_draw_indirect: bool) {
        self.raw.multiDrawIndirect = multi_draw_indirect as u32;
    }

    pub fn set_draw_indirect_first_instance<'m>(& mut self, draw_indirect_first_instance: bool) {
        self.raw.drawIndirectFirstInstance = draw_indirect_first_instance as u32;
    }

    pub fn set_depth_clamp<'m>(& mut self, depth_clamp: bool) {
        self.raw.depthClamp = depth_clamp as u32;
    }

    pub fn set_depth_bias_clamp<'m>(& mut self, depth_bias_clamp: bool) {
        self.raw.depthBiasClamp = depth_bias_clamp as u32;
    }

    pub fn set_fill_mode_non_solid<'m>(& mut self, fill_mode_non_solid: bool) {
        self.raw.fillModeNonSolid = fill_mode_non_solid as u32;
    }

    pub fn set_depth_bounds<'m>(& mut self, depth_bounds: bool) {
        self.raw.depthBounds = depth_bounds as u32;
    }

    pub fn set_wide_lines<'m>(& mut self, wide_lines: bool) {
        self.raw.wideLines = wide_lines as u32;
    }

    pub fn set_large_points<'m>(& mut self, large_points: bool) {
        self.raw.largePoints = large_points as u32;
    }

    pub fn set_alpha_to_one<'m>(& mut self, alpha_to_one: bool) {
        self.raw.alphaToOne = alpha_to_one as u32;
    }

    pub fn set_multi_viewport<'m>(& mut self, multi_viewport: bool) {
        self.raw.multiViewport = multi_viewport as u32;
    }

    pub fn set_sampler_anisotropy<'m>(& mut self, sampler_anisotropy: bool) {
        self.raw.samplerAnisotropy = sampler_anisotropy as u32;
    }

    pub fn set_texture_compression_et_c2<'m>(& mut self, texture_compression_et_c2: bool) {
        self.raw.textureCompressionETC2 = texture_compression_et_c2 as u32;
    }

    pub fn set_texture_compression_as_tc_ld_r<'m>(& mut self, texture_compression_as_tc_ld_r: bool) {
        self.raw.textureCompressionASTC_LDR = texture_compression_as_tc_ld_r as u32;
    }

    pub fn set_texture_compression_bc<'m>(& mut self, texture_compression_bc: bool) {
        self.raw.textureCompressionBC = texture_compression_bc as u32;
    }

    pub fn set_occlusion_query_precise<'m>(& mut self, occlusion_query_precise: bool) {
        self.raw.occlusionQueryPrecise = occlusion_query_precise as u32;
    }

    pub fn set_pipeline_statistics_query<'m>(& mut self, pipeline_statistics_query: bool) {
        self.raw.pipelineStatisticsQuery = pipeline_statistics_query as u32;
    }

    pub fn set_vertex_pipeline_stores_and_atomics<'m>(& mut self, vertex_pipeline_stores_and_atomics: bool) {
        self.raw.vertexPipelineStoresAndAtomics = vertex_pipeline_stores_and_atomics as u32;
    }

    pub fn set_fragment_stores_and_atomics<'m>(& mut self, fragment_stores_and_atomics: bool) {
        self.raw.fragmentStoresAndAtomics = fragment_stores_and_atomics as u32;
    }

    pub fn set_shader_tessellation_and_geometry_point_size<'m>(& mut self, shader_tessellation_and_geometry_point_size: bool) {
        self.raw.shaderTessellationAndGeometryPointSize = shader_tessellation_and_geometry_point_size as u32;
    }

    pub fn set_shader_image_gather_extended<'m>(& mut self, shader_image_gather_extended: bool) {
        self.raw.shaderImageGatherExtended = shader_image_gather_extended as u32;
    }

    pub fn set_shader_storage_image_extended_formats<'m>(& mut self, shader_storage_image_extended_formats: bool) {
        self.raw.shaderStorageImageExtendedFormats = shader_storage_image_extended_formats as u32;
    }

    pub fn set_shader_storage_image_multisample<'m>(& mut self, shader_storage_image_multisample: bool) {
        self.raw.shaderStorageImageMultisample = shader_storage_image_multisample as u32;
    }

    pub fn set_shader_storage_image_read_without_format<'m>(& mut self, shader_storage_image_read_without_format: bool) {
        self.raw.shaderStorageImageReadWithoutFormat = shader_storage_image_read_without_format as u32;
    }

    pub fn set_shader_storage_image_write_without_format<'m>(& mut self, shader_storage_image_write_without_format: bool) {
        self.raw.shaderStorageImageWriteWithoutFormat = shader_storage_image_write_without_format as u32;
    }

    pub fn set_shader_uniform_buffer_array_dynamic_indexing<'m>(& mut self, shader_uniform_buffer_array_dynamic_indexing: bool) {
        self.raw.shaderUniformBufferArrayDynamicIndexing = shader_uniform_buffer_array_dynamic_indexing as u32;
    }

    pub fn set_shader_sampled_image_array_dynamic_indexing<'m>(& mut self, shader_sampled_image_array_dynamic_indexing: bool) {
        self.raw.shaderSampledImageArrayDynamicIndexing = shader_sampled_image_array_dynamic_indexing as u32;
    }

    pub fn set_shader_storage_buffer_array_dynamic_indexing<'m>(& mut self, shader_storage_buffer_array_dynamic_indexing: bool) {
        self.raw.shaderStorageBufferArrayDynamicIndexing = shader_storage_buffer_array_dynamic_indexing as u32;
    }

    pub fn set_shader_storage_image_array_dynamic_indexing<'m>(& mut self, shader_storage_image_array_dynamic_indexing: bool) {
        self.raw.shaderStorageImageArrayDynamicIndexing = shader_storage_image_array_dynamic_indexing as u32;
    }

    pub fn set_shader_clip_distance<'m>(& mut self, shader_clip_distance: bool) {
        self.raw.shaderClipDistance = shader_clip_distance as u32;
    }

    pub fn set_shader_cull_distance<'m>(& mut self, shader_cull_distance: bool) {
        self.raw.shaderCullDistance = shader_cull_distance as u32;
    }

    pub fn set_shader_float_64<'m>(& mut self, shader_float_64: bool) {
        self.raw.shaderFloat64 = shader_float_64 as u32;
    }

    pub fn set_shader_int_64<'m>(& mut self, shader_int_64: bool) {
        self.raw.shaderInt64 = shader_int_64 as u32;
    }

    pub fn set_shader_int_16<'m>(& mut self, shader_int_16: bool) {
        self.raw.shaderInt16 = shader_int_16 as u32;
    }

    pub fn set_shader_resource_residency<'m>(& mut self, shader_resource_residency: bool) {
        self.raw.shaderResourceResidency = shader_resource_residency as u32;
    }

    pub fn set_shader_resource_min_lod<'m>(& mut self, shader_resource_min_lod: bool) {
        self.raw.shaderResourceMinLod = shader_resource_min_lod as u32;
    }

    pub fn set_sparse_binding<'m>(& mut self, sparse_binding: bool) {
        self.raw.sparseBinding = sparse_binding as u32;
    }

    pub fn set_sparse_residency_buffer<'m>(& mut self, sparse_residency_buffer: bool) {
        self.raw.sparseResidencyBuffer = sparse_residency_buffer as u32;
    }

    pub fn set_sparse_residency_image_2d<'m>(& mut self, sparse_residency_image_2d: bool) {
        self.raw.sparseResidencyImage2D = sparse_residency_image_2d as u32;
    }

    pub fn set_sparse_residency_image_3d<'m>(& mut self, sparse_residency_image_3d: bool) {
        self.raw.sparseResidencyImage3D = sparse_residency_image_3d as u32;
    }

    pub fn set_sparse_residency_2samples<'m>(& mut self, sparse_residency_2samples: bool) {
        self.raw.sparseResidency2Samples = sparse_residency_2samples as u32;
    }

    pub fn set_sparse_residency_4samples<'m>(& mut self, sparse_residency_4samples: bool) {
        self.raw.sparseResidency4Samples = sparse_residency_4samples as u32;
    }

    pub fn set_sparse_residency_8samples<'m>(& mut self, sparse_residency_8samples: bool) {
        self.raw.sparseResidency8Samples = sparse_residency_8samples as u32;
    }

    pub fn set_sparse_residency_16_samples<'m>(& mut self, sparse_residency_16_samples: bool) {
        self.raw.sparseResidency16Samples = sparse_residency_16_samples as u32;
    }

    pub fn set_sparse_residency_aliased<'m>(& mut self, sparse_residency_aliased: bool) {
        self.raw.sparseResidencyAliased = sparse_residency_aliased as u32;
    }

    pub fn set_variable_multisample_rate<'m>(& mut self, variable_multisample_rate: bool) {
        self.raw.variableMultisampleRate = variable_multisample_rate as u32;
    }

    pub fn set_inherited_queries<'m>(& mut self, inherited_queries: bool) {
        self.raw.inheritedQueries = inherited_queries as u32;
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


impl From<vks::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn from(f: vks::VkPhysicalDeviceFeatures) -> PhysicalDeviceFeatures {
        PhysicalDeviceFeatures { raw: f, }
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

    pub fn get_robust_buffer_access<'a>(&'a self) -> bool {
        self.raw.robustBufferAccess != 0
    }

    pub fn get_full_draw_index_uint_32<'a>(&'a self) -> bool {
        self.raw.fullDrawIndexUint32 != 0
    }

    pub fn get_image_cube_array<'a>(&'a self) -> bool {
        self.raw.imageCubeArray != 0
    }

    pub fn get_independent_blend<'a>(&'a self) -> bool {
        self.raw.independentBlend != 0
    }

    pub fn get_geometry_shader<'a>(&'a self) -> bool {
        self.raw.geometryShader != 0
    }

    pub fn get_tessellation_shader<'a>(&'a self) -> bool {
        self.raw.tessellationShader != 0
    }

    pub fn get_sample_rate_shading<'a>(&'a self) -> bool {
        self.raw.sampleRateShading != 0
    }

    pub fn get_dual_src_blend<'a>(&'a self) -> bool {
        self.raw.dualSrcBlend != 0
    }

    pub fn get_logic_op<'a>(&'a self) -> bool {
        self.raw.logicOp != 0
    }

    pub fn get_multi_draw_indirect<'a>(&'a self) -> bool {
        self.raw.multiDrawIndirect != 0
    }

    pub fn get_draw_indirect_first_instance<'a>(&'a self) -> bool {
        self.raw.drawIndirectFirstInstance != 0
    }

    pub fn get_depth_clamp<'a>(&'a self) -> bool {
        self.raw.depthClamp != 0
    }

    pub fn get_depth_bias_clamp<'a>(&'a self) -> bool {
        self.raw.depthBiasClamp != 0
    }

    pub fn get_fill_mode_non_solid<'a>(&'a self) -> bool {
        self.raw.fillModeNonSolid != 0
    }

    pub fn get_depth_bounds<'a>(&'a self) -> bool {
        self.raw.depthBounds != 0
    }

    pub fn get_wide_lines<'a>(&'a self) -> bool {
        self.raw.wideLines != 0
    }

    pub fn get_large_points<'a>(&'a self) -> bool {
        self.raw.largePoints != 0
    }

    pub fn get_alpha_to_one<'a>(&'a self) -> bool {
        self.raw.alphaToOne != 0
    }

    pub fn get_multi_viewport<'a>(&'a self) -> bool {
        self.raw.multiViewport != 0
    }

    pub fn get_sampler_anisotropy<'a>(&'a self) -> bool {
        self.raw.samplerAnisotropy != 0
    }

    pub fn get_texture_compression_et_c2<'a>(&'a self) -> bool {
        self.raw.textureCompressionETC2 != 0
    }

    pub fn get_texture_compression_as_tc_ld_r<'a>(&'a self) -> bool {
        self.raw.textureCompressionASTC_LDR != 0
    }

    pub fn get_texture_compression_bc<'a>(&'a self) -> bool {
        self.raw.textureCompressionBC != 0
    }

    pub fn get_occlusion_query_precise<'a>(&'a self) -> bool {
        self.raw.occlusionQueryPrecise != 0
    }

    pub fn get_pipeline_statistics_query<'a>(&'a self) -> bool {
        self.raw.pipelineStatisticsQuery != 0
    }

    pub fn get_vertex_pipeline_stores_and_atomics<'a>(&'a self) -> bool {
        self.raw.vertexPipelineStoresAndAtomics != 0
    }

    pub fn get_fragment_stores_and_atomics<'a>(&'a self) -> bool {
        self.raw.fragmentStoresAndAtomics != 0
    }

    pub fn get_shader_tessellation_and_geometry_point_size<'a>(&'a self) -> bool {
        self.raw.shaderTessellationAndGeometryPointSize != 0
    }

    pub fn get_shader_image_gather_extended<'a>(&'a self) -> bool {
        self.raw.shaderImageGatherExtended != 0
    }

    pub fn get_shader_storage_image_extended_formats<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageExtendedFormats != 0
    }

    pub fn get_shader_storage_image_multisample<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageMultisample != 0
    }

    pub fn get_shader_storage_image_read_without_format<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageReadWithoutFormat != 0
    }

    pub fn get_shader_storage_image_write_without_format<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageWriteWithoutFormat != 0
    }

    pub fn get_shader_uniform_buffer_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderUniformBufferArrayDynamicIndexing != 0
    }

    pub fn get_shader_sampled_image_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderSampledImageArrayDynamicIndexing != 0
    }

    pub fn get_shader_storage_buffer_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderStorageBufferArrayDynamicIndexing != 0
    }

    pub fn get_shader_storage_image_array_dynamic_indexing<'a>(&'a self) -> bool {
        self.raw.shaderStorageImageArrayDynamicIndexing != 0
    }

    pub fn get_shader_clip_distance<'a>(&'a self) -> bool {
        self.raw.shaderClipDistance != 0
    }

    pub fn get_shader_cull_distance<'a>(&'a self) -> bool {
        self.raw.shaderCullDistance != 0
    }

    pub fn get_shader_float_64<'a>(&'a self) -> bool {
        self.raw.shaderFloat64 != 0
    }

    pub fn get_shader_int_64<'a>(&'a self) -> bool {
        self.raw.shaderInt64 != 0
    }

    pub fn get_shader_int_16<'a>(&'a self) -> bool {
        self.raw.shaderInt16 != 0
    }

    pub fn get_shader_resource_residency<'a>(&'a self) -> bool {
        self.raw.shaderResourceResidency != 0
    }

    pub fn get_shader_resource_min_lod<'a>(&'a self) -> bool {
        self.raw.shaderResourceMinLod != 0
    }

    pub fn get_sparse_binding<'a>(&'a self) -> bool {
        self.raw.sparseBinding != 0
    }

    pub fn get_sparse_residency_buffer<'a>(&'a self) -> bool {
        self.raw.sparseResidencyBuffer != 0
    }

    pub fn get_sparse_residency_image_2d<'a>(&'a self) -> bool {
        self.raw.sparseResidencyImage2D != 0
    }

    pub fn get_sparse_residency_image_3d<'a>(&'a self) -> bool {
        self.raw.sparseResidencyImage3D != 0
    }

    pub fn get_sparse_residency_2samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency2Samples != 0
    }

    pub fn get_sparse_residency_4samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency4Samples != 0
    }

    pub fn get_sparse_residency_8samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency8Samples != 0
    }

    pub fn get_sparse_residency_16_samples<'a>(&'a self) -> bool {
        self.raw.sparseResidency16Samples != 0
    }

    pub fn get_sparse_residency_aliased<'a>(&'a self) -> bool {
        self.raw.sparseResidencyAliased != 0
    }

    pub fn get_variable_multisample_rate<'a>(&'a self) -> bool {
        self.raw.variableMultisampleRate != 0
    }

    pub fn get_inherited_queries<'a>(&'a self) -> bool {
        self.raw.inheritedQueries != 0
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
    pub fn residency_standard_2d_block_shape<'a>(&'a self) -> bool {
        self.raw.residencyStandard2DBlockShape != 0
    }

    pub fn residency_standard_2d_multisample_block_shape<'a>(&'a self) -> bool {
        self.raw.residencyStandard2DMultisampleBlockShape != 0
    }

    pub fn residency_standard_3d_block_shape<'a>(&'a self) -> bool {
        self.raw.residencyStandard3DBlockShape != 0
    }

    pub fn residency_aligned_mip_size<'a>(&'a self) -> bool {
        self.raw.residencyAlignedMipSize != 0
    }

    pub fn residency_non_resident_strict<'a>(&'a self) -> bool {
        self.raw.residencyNonResidentStrict != 0
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


impl From<vks::VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
    fn from(f: vks::VkPhysicalDeviceSparseProperties) -> PhysicalDeviceSparseProperties {
        PhysicalDeviceSparseProperties { raw: f, }
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
    pub fn max_image_dimension_1d<'a>(&'a self) -> u32 {
        self.raw.maxImageDimension1D.into()
    }

    pub fn max_image_dimension_2d<'a>(&'a self) -> u32 {
        self.raw.maxImageDimension2D.into()
    }

    pub fn max_image_dimension_3d<'a>(&'a self) -> u32 {
        self.raw.maxImageDimension3D.into()
    }

    pub fn max_image_dimension_cube<'a>(&'a self) -> u32 {
        self.raw.maxImageDimensionCube.into()
    }

    pub fn max_image_array_layers<'a>(&'a self) -> u32 {
        self.raw.maxImageArrayLayers.into()
    }

    pub fn max_texel_buffer_elements<'a>(&'a self) -> u32 {
        self.raw.maxTexelBufferElements.into()
    }

    pub fn max_uniform_buffer_range<'a>(&'a self) -> u32 {
        self.raw.maxUniformBufferRange.into()
    }

    pub fn max_storage_buffer_range<'a>(&'a self) -> u32 {
        self.raw.maxStorageBufferRange.into()
    }

    pub fn max_push_constants_size<'a>(&'a self) -> u32 {
        self.raw.maxPushConstantsSize.into()
    }

    pub fn max_memory_allocation_count<'a>(&'a self) -> u32 {
        self.raw.maxMemoryAllocationCount.into()
    }

    pub fn max_sampler_allocation_count<'a>(&'a self) -> u32 {
        self.raw.maxSamplerAllocationCount.into()
    }

    pub fn buffer_image_granularity<'a>(&'a self) -> u64 {
        self.raw.bufferImageGranularity.into()
    }

    pub fn sparse_address_space_size<'a>(&'a self) -> u64 {
        self.raw.sparseAddressSpaceSize.into()
    }

    pub fn max_bound_descriptor_sets<'a>(&'a self) -> u32 {
        self.raw.maxBoundDescriptorSets.into()
    }

    pub fn max_per_stage_descriptor_samplers<'a>(&'a self) -> u32 {
        self.raw.maxPerStageDescriptorSamplers.into()
    }

    pub fn max_per_stage_descriptor_uniform_buffers<'a>(&'a self) -> u32 {
        self.raw.maxPerStageDescriptorUniformBuffers.into()
    }

    pub fn max_per_stage_descriptor_storage_buffers<'a>(&'a self) -> u32 {
        self.raw.maxPerStageDescriptorStorageBuffers.into()
    }

    pub fn max_per_stage_descriptor_sampled_images<'a>(&'a self) -> u32 {
        self.raw.maxPerStageDescriptorSampledImages.into()
    }

    pub fn max_per_stage_descriptor_storage_images<'a>(&'a self) -> u32 {
        self.raw.maxPerStageDescriptorStorageImages.into()
    }

    pub fn max_per_stage_descriptor_input_attachments<'a>(&'a self) -> u32 {
        self.raw.maxPerStageDescriptorInputAttachments.into()
    }

    pub fn max_per_stage_resources<'a>(&'a self) -> u32 {
        self.raw.maxPerStageResources.into()
    }

    pub fn max_descriptor_set_samplers<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetSamplers.into()
    }

    pub fn max_descriptor_set_uniform_buffers<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetUniformBuffers.into()
    }

    pub fn max_descriptor_set_uniform_buffers_dynamic<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetUniformBuffersDynamic.into()
    }

    pub fn max_descriptor_set_storage_buffers<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetStorageBuffers.into()
    }

    pub fn max_descriptor_set_storage_buffers_dynamic<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetStorageBuffersDynamic.into()
    }

    pub fn max_descriptor_set_sampled_images<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetSampledImages.into()
    }

    pub fn max_descriptor_set_storage_images<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetStorageImages.into()
    }

    pub fn max_descriptor_set_input_attachments<'a>(&'a self) -> u32 {
        self.raw.maxDescriptorSetInputAttachments.into()
    }

    pub fn max_vertex_input_attributes<'a>(&'a self) -> u32 {
        self.raw.maxVertexInputAttributes.into()
    }

    pub fn max_vertex_input_bindings<'a>(&'a self) -> u32 {
        self.raw.maxVertexInputBindings.into()
    }

    pub fn max_vertex_input_attribute_offset<'a>(&'a self) -> u32 {
        self.raw.maxVertexInputAttributeOffset.into()
    }

    pub fn max_vertex_input_binding_stride<'a>(&'a self) -> u32 {
        self.raw.maxVertexInputBindingStride.into()
    }

    pub fn max_vertex_output_components<'a>(&'a self) -> u32 {
        self.raw.maxVertexOutputComponents.into()
    }

    pub fn max_tessellation_generation_level<'a>(&'a self) -> u32 {
        self.raw.maxTessellationGenerationLevel.into()
    }

    pub fn max_tessellation_patch_size<'a>(&'a self) -> u32 {
        self.raw.maxTessellationPatchSize.into()
    }

    pub fn max_tessellation_control_per_vertex_input_components<'a>(&'a self) -> u32 {
        self.raw.maxTessellationControlPerVertexInputComponents.into()
    }

    pub fn max_tessellation_control_per_vertex_output_components<'a>(&'a self) -> u32 {
        self.raw.maxTessellationControlPerVertexOutputComponents.into()
    }

    pub fn max_tessellation_control_per_patch_output_components<'a>(&'a self) -> u32 {
        self.raw.maxTessellationControlPerPatchOutputComponents.into()
    }

    pub fn max_tessellation_control_total_output_components<'a>(&'a self) -> u32 {
        self.raw.maxTessellationControlTotalOutputComponents.into()
    }

    pub fn max_tessellation_evaluation_input_components<'a>(&'a self) -> u32 {
        self.raw.maxTessellationEvaluationInputComponents.into()
    }

    pub fn max_tessellation_evaluation_output_components<'a>(&'a self) -> u32 {
        self.raw.maxTessellationEvaluationOutputComponents.into()
    }

    pub fn max_geometry_shader_invocations<'a>(&'a self) -> u32 {
        self.raw.maxGeometryShaderInvocations.into()
    }

    pub fn max_geometry_input_components<'a>(&'a self) -> u32 {
        self.raw.maxGeometryInputComponents.into()
    }

    pub fn max_geometry_output_components<'a>(&'a self) -> u32 {
        self.raw.maxGeometryOutputComponents.into()
    }

    pub fn max_geometry_output_vertices<'a>(&'a self) -> u32 {
        self.raw.maxGeometryOutputVertices.into()
    }

    pub fn max_geometry_total_output_components<'a>(&'a self) -> u32 {
        self.raw.maxGeometryTotalOutputComponents.into()
    }

    pub fn max_fragment_input_components<'a>(&'a self) -> u32 {
        self.raw.maxFragmentInputComponents.into()
    }

    pub fn max_fragment_output_attachments<'a>(&'a self) -> u32 {
        self.raw.maxFragmentOutputAttachments.into()
    }

    pub fn max_fragment_dual_src_attachments<'a>(&'a self) -> u32 {
        self.raw.maxFragmentDualSrcAttachments.into()
    }

    pub fn max_fragment_combined_output_resources<'a>(&'a self) -> u32 {
        self.raw.maxFragmentCombinedOutputResources.into()
    }

    pub fn max_compute_shared_memory_size<'a>(&'a self) -> u32 {
        self.raw.maxComputeSharedMemorySize.into()
    }

    pub fn max_compute_work_group_count<'a>(&'a self) -> &[u32] {
        unsafe { slice::from_raw_parts(&self.raw.maxComputeWorkGroupCount as *const _, 3 as usize) }
    }

    pub fn max_compute_work_group_invocations<'a>(&'a self) -> u32 {
        self.raw.maxComputeWorkGroupInvocations.into()
    }

    pub fn max_compute_work_group_size<'a>(&'a self) -> &[u32] {
        unsafe { slice::from_raw_parts(&self.raw.maxComputeWorkGroupSize as *const _, 3 as usize) }
    }

    pub fn sub_pixel_precision_bits<'a>(&'a self) -> u32 {
        self.raw.subPixelPrecisionBits.into()
    }

    pub fn sub_texel_precision_bits<'a>(&'a self) -> u32 {
        self.raw.subTexelPrecisionBits.into()
    }

    pub fn mipmap_precision_bits<'a>(&'a self) -> u32 {
        self.raw.mipmapPrecisionBits.into()
    }

    pub fn max_draw_indexed_index_value<'a>(&'a self) -> u32 {
        self.raw.maxDrawIndexedIndexValue.into()
    }

    pub fn max_draw_indirect_count<'a>(&'a self) -> u32 {
        self.raw.maxDrawIndirectCount.into()
    }

    pub fn max_sampler_lod_bias<'a>(&'a self) -> f32 {
        self.raw.maxSamplerLodBias.into()
    }

    pub fn max_sampler_anisotropy<'a>(&'a self) -> f32 {
        self.raw.maxSamplerAnisotropy.into()
    }

    pub fn max_viewports<'a>(&'a self) -> u32 {
        self.raw.maxViewports.into()
    }

    pub fn max_viewport_dimensions<'a>(&'a self) -> &[u32] {
        unsafe { slice::from_raw_parts(&self.raw.maxViewportDimensions as *const _, 2 as usize) }
    }

    pub fn viewport_bounds_range<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.viewportBoundsRange as *const _, 2 as usize) }
    }

    pub fn viewport_sub_pixel_bits<'a>(&'a self) -> u32 {
        self.raw.viewportSubPixelBits.into()
    }

    pub fn min_memory_map_alignment<'a>(&'a self) -> usize {
        self.raw.minMemoryMapAlignment.into()
    }

    pub fn min_texel_buffer_offset_alignment<'a>(&'a self) -> u64 {
        self.raw.minTexelBufferOffsetAlignment.into()
    }

    pub fn min_uniform_buffer_offset_alignment<'a>(&'a self) -> u64 {
        self.raw.minUniformBufferOffsetAlignment.into()
    }

    pub fn min_storage_buffer_offset_alignment<'a>(&'a self) -> u64 {
        self.raw.minStorageBufferOffsetAlignment.into()
    }

    pub fn min_texel_offset<'a>(&'a self) -> i32 {
        self.raw.minTexelOffset.into()
    }

    pub fn max_texel_offset<'a>(&'a self) -> u32 {
        self.raw.maxTexelOffset.into()
    }

    pub fn min_texel_gather_offset<'a>(&'a self) -> i32 {
        self.raw.minTexelGatherOffset.into()
    }

    pub fn max_texel_gather_offset<'a>(&'a self) -> u32 {
        self.raw.maxTexelGatherOffset.into()
    }

    pub fn min_interpolation_offset<'a>(&'a self) -> f32 {
        self.raw.minInterpolationOffset.into()
    }

    pub fn max_interpolation_offset<'a>(&'a self) -> f32 {
        self.raw.maxInterpolationOffset.into()
    }

    pub fn sub_pixel_interpolation_offset_bits<'a>(&'a self) -> u32 {
        self.raw.subPixelInterpolationOffsetBits.into()
    }

    pub fn max_framebuffer_width<'a>(&'a self) -> u32 {
        self.raw.maxFramebufferWidth.into()
    }

    pub fn max_framebuffer_height<'a>(&'a self) -> u32 {
        self.raw.maxFramebufferHeight.into()
    }

    pub fn max_framebuffer_layers<'a>(&'a self) -> u32 {
        self.raw.maxFramebufferLayers.into()
    }

    pub fn framebuffer_color_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.framebufferColorSampleCounts)
            .expect("PhysicalDeviceLimits::framebuffer_color_sample_counts: error converting flags")
    }

    pub fn framebuffer_depth_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.framebufferDepthSampleCounts)
            .expect("PhysicalDeviceLimits::framebuffer_depth_sample_counts: error converting flags")
    }

    pub fn framebuffer_stencil_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.framebufferStencilSampleCounts)
            .expect("PhysicalDeviceLimits::framebuffer_stencil_sample_counts: error converting flags")
    }

    pub fn framebuffer_no_attachments_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.framebufferNoAttachmentsSampleCounts)
            .expect("PhysicalDeviceLimits::framebuffer_no_attachments_sample_counts: error converting flags")
    }

    pub fn max_color_attachments<'a>(&'a self) -> u32 {
        self.raw.maxColorAttachments.into()
    }

    pub fn sampled_image_color_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.sampledImageColorSampleCounts)
            .expect("PhysicalDeviceLimits::sampled_image_color_sample_counts: error converting flags")
    }

    pub fn sampled_image_integer_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.sampledImageIntegerSampleCounts)
            .expect("PhysicalDeviceLimits::sampled_image_integer_sample_counts: error converting flags")
    }

    pub fn sampled_image_depth_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.sampledImageDepthSampleCounts)
            .expect("PhysicalDeviceLimits::sampled_image_depth_sample_counts: error converting flags")
    }

    pub fn sampled_image_stencil_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.sampledImageStencilSampleCounts)
            .expect("PhysicalDeviceLimits::sampled_image_stencil_sample_counts: error converting flags")
    }

    pub fn storage_image_sample_counts<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.storageImageSampleCounts)
            .expect("PhysicalDeviceLimits::storage_image_sample_counts: error converting flags")
    }

    pub fn max_sample_mask_words<'a>(&'a self) -> u32 {
        self.raw.maxSampleMaskWords.into()
    }

    pub fn timestamp_compute_and_graphics<'a>(&'a self) -> bool {
        self.raw.timestampComputeAndGraphics != 0
    }

    pub fn timestamp_period<'a>(&'a self) -> f32 {
        self.raw.timestampPeriod.into()
    }

    pub fn max_clip_distances<'a>(&'a self) -> u32 {
        self.raw.maxClipDistances.into()
    }

    pub fn max_cull_distances<'a>(&'a self) -> u32 {
        self.raw.maxCullDistances.into()
    }

    pub fn max_combined_clip_and_cull_distances<'a>(&'a self) -> u32 {
        self.raw.maxCombinedClipAndCullDistances.into()
    }

    pub fn discrete_queue_priorities<'a>(&'a self) -> u32 {
        self.raw.discreteQueuePriorities.into()
    }

    pub fn point_size_range<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.pointSizeRange as *const _, 2 as usize) }
    }

    pub fn line_width_range<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.lineWidthRange as *const _, 2 as usize) }
    }

    pub fn point_size_granularity<'a>(&'a self) -> f32 {
        self.raw.pointSizeGranularity.into()
    }

    pub fn line_width_granularity<'a>(&'a self) -> f32 {
        self.raw.lineWidthGranularity.into()
    }

    pub fn strict_lines<'a>(&'a self) -> bool {
        self.raw.strictLines != 0
    }

    pub fn standard_sample_locations<'a>(&'a self) -> bool {
        self.raw.standardSampleLocations != 0
    }

    pub fn optimal_buffer_copy_offset_alignment<'a>(&'a self) -> u64 {
        self.raw.optimalBufferCopyOffsetAlignment.into()
    }

    pub fn optimal_buffer_copy_row_pitch_alignment<'a>(&'a self) -> u64 {
        self.raw.optimalBufferCopyRowPitchAlignment.into()
    }

    pub fn non_coherent_atom_size<'a>(&'a self) -> u64 {
        self.raw.nonCoherentAtomSize.into()
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


impl From<vks::VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
    fn from(f: vks::VkPhysicalDeviceLimits) -> PhysicalDeviceLimits {
        PhysicalDeviceLimits { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> SemaphoreCreateFlags {
        SemaphoreCreateFlags::from_bits(self.raw.flags)
            .expect("SemaphoreCreateInfo::flags: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: SemaphoreCreateFlags) {
        self.raw.flags = flags.bits();
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


impl<'s> From<vks::VkSemaphoreCreateInfo> for SemaphoreCreateInfo<'s> {
    fn from(f: vks::VkSemaphoreCreateInfo) -> SemaphoreCreateInfo<'s> {
        SemaphoreCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> SemaphoreCreateFlags {
        SemaphoreCreateFlags::from_bits(self.raw.flags)
            .expect("SemaphoreCreateInfo::flags: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> QueryPoolCreateFlags {
        QueryPoolCreateFlags::from_bits(self.raw.flags)
            .expect("QueryPoolCreateInfo::flags: error converting flags")
    }

    pub fn query_type<'a>(&'a self) -> QueryType {
        self.raw.queryType.into()
    }

    pub fn query_count<'a>(&'a self) -> u32 {
        self.raw.queryCount.into()
    }

    pub fn pipeline_statistics<'a>(&'a self) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags::from_bits(self.raw.pipelineStatistics)
            .expect("QueryPoolCreateInfo::pipeline_statistics: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: QueryPoolCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_query_type<'m>(&'s mut self, query_type: QueryType) {
        self.raw.queryType = query_type.into();
    }

    pub fn set_query_count<'m>(&'s mut self, query_count: u32) {
        self.raw.queryCount = query_count.into();
    }

    pub fn set_pipeline_statistics<'m>(&'s mut self, pipeline_statistics: QueryPipelineStatisticFlags) {
        self.raw.pipelineStatistics = pipeline_statistics.bits();
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


impl<'s> From<vks::VkQueryPoolCreateInfo> for QueryPoolCreateInfo<'s> {
    fn from(f: vks::VkQueryPoolCreateInfo) -> QueryPoolCreateInfo<'s> {
        QueryPoolCreateInfo { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> QueryPoolCreateFlags {
        QueryPoolCreateFlags::from_bits(self.raw.flags)
            .expect("QueryPoolCreateInfo::flags: error converting flags")
    }

    pub fn get_query_type<'a>(&'a self) -> QueryType {
        self.raw.queryType.into()
    }

    pub fn get_query_count<'a>(&'a self) -> u32 {
        self.raw.queryCount.into()
    }

    pub fn get_pipeline_statistics<'a>(&'a self) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags::from_bits(self.raw.pipelineStatistics)
            .expect("QueryPoolCreateInfo::pipeline_statistics: error converting flags")
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
    attachments: Option<SmallVec<[vks::VkImageView; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> FramebufferCreateInfo<'s> {
    pub fn builder<'b>() -> FramebufferCreateInfoBuilder<'b> {
        FramebufferCreateInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> FramebufferCreateFlags {
        FramebufferCreateFlags::from_bits(self.raw.flags)
            .expect("FramebufferCreateInfo::flags: error converting flags")
    }

    pub fn render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn attachments_handle<'a>(&'a self) -> &'a [vks::VkImageView] {
        unsafe { slice::from_raw_parts(self.raw.pAttachments as *const _, self.raw.attachmentCount as usize) }
    }

    pub fn width<'a>(&'a self) -> u32 {
        self.raw.width.into()
    }

    pub fn height<'a>(&'a self) -> u32 {
        self.raw.height.into()
    }

    pub fn layers<'a>(&'a self) -> u32 {
        self.raw.layers.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: FramebufferCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_render_pass<'m, 'a>(&'s mut self, render_pass: &'a RenderPass) {
        self.raw.renderPass = render_pass.handle();
    }

    pub fn set_attachments<'m, 'a>(&'s mut self, attachments: &'a [&'a ImageView])
            where 'a: 's {
        self.attachments = Some(attachments.iter().map(|h| h.handle()).collect());
        {
            let attachments = self.attachments.as_ref().unwrap();
            self.raw.pAttachments = attachments.as_ptr();
            assert!(self.raw.attachmentCount == 0 || self.raw.attachmentCount == attachments.len() as _, 
                "count inconsistency found when specifying `FramebufferCreateInfo::attachments`.");
            self.raw.attachmentCount = attachments.len() as _;
        }
    }

    pub fn set_width<'m>(&'s mut self, width: u32) {
        self.raw.width = width.into();
    }

    pub fn set_height<'m>(&'s mut self, height: u32) {
        self.raw.height = height.into();
    }

    pub fn set_layers<'m>(&'s mut self, layers: u32) {
        self.raw.layers = layers.into();
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


impl<'s> From<vks::VkFramebufferCreateInfo> for FramebufferCreateInfo<'s> {
    fn from(f: vks::VkFramebufferCreateInfo) -> FramebufferCreateInfo<'s> {
        FramebufferCreateInfo { raw: f, attachments: None, _p: PhantomData }
    }
}


/// A builder for `VkFramebufferCreateInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct FramebufferCreateInfoBuilder<'b> {
    raw: vks::VkFramebufferCreateInfo,
    attachments: Option<SmallVec<[vks::VkImageView; 8]>>,
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

    pub fn attachments<'m, 'a>(mut self, attachments: &'a [&'a ImageView]) -> FramebufferCreateInfoBuilder<'b>
            where 'a: 'b {
        self.attachments = Some(attachments.iter().map(|h| h.handle()).collect());
        {
            let attachments = self.attachments.as_ref().unwrap();
            self.raw.pAttachments = attachments.as_ptr();
            assert!(self.raw.attachmentCount == 0 || self.raw.attachmentCount == attachments.len() as _, 
                "count inconsistency found when specifying `FramebufferCreateInfo::attachments`.");
            self.raw.attachmentCount = attachments.len() as _;
        }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> FramebufferCreateFlags {
        FramebufferCreateFlags::from_bits(self.raw.flags)
            .expect("FramebufferCreateInfo::flags: error converting flags")
    }

    pub fn get_render_pass_handle<'a>(&'a self) -> vks::VkRenderPass {
        self.raw.renderPass
    }

    pub fn get_attachments_handle<'a>(&'a self) -> &'a [vks::VkImageView] {
        unsafe { slice::from_raw_parts(self.raw.pAttachments as *const _, self.raw.attachmentCount as usize) }
    }

    pub fn get_width<'a>(&'a self) -> u32 {
        self.raw.width.into()
    }

    pub fn get_height<'a>(&'a self) -> u32 {
        self.raw.height.into()
    }

    pub fn get_layers<'a>(&'a self) -> u32 {
        self.raw.layers.into()
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

    pub fn vertex_count<'a>(&'a self) -> u32 {
        self.raw.vertexCount.into()
    }

    pub fn instance_count<'a>(&'a self) -> u32 {
        self.raw.instanceCount.into()
    }

    pub fn first_vertex<'a>(&'a self) -> u32 {
        self.raw.firstVertex.into()
    }

    pub fn first_instance<'a>(&'a self) -> u32 {
        self.raw.firstInstance.into()
    }

    pub fn set_vertex_count<'m>(& mut self, vertex_count: u32) {
        self.raw.vertexCount = vertex_count.into();
    }

    pub fn set_instance_count<'m>(& mut self, instance_count: u32) {
        self.raw.instanceCount = instance_count.into();
    }

    pub fn set_first_vertex<'m>(& mut self, first_vertex: u32) {
        self.raw.firstVertex = first_vertex.into();
    }

    pub fn set_first_instance<'m>(& mut self, first_instance: u32) {
        self.raw.firstInstance = first_instance.into();
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


impl From<vks::VkDrawIndirectCommand> for DrawIndirectCommand {
    fn from(f: vks::VkDrawIndirectCommand) -> DrawIndirectCommand {
        DrawIndirectCommand { raw: f, }
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

    pub fn get_vertex_count<'a>(&'a self) -> u32 {
        self.raw.vertexCount.into()
    }

    pub fn get_instance_count<'a>(&'a self) -> u32 {
        self.raw.instanceCount.into()
    }

    pub fn get_first_vertex<'a>(&'a self) -> u32 {
        self.raw.firstVertex.into()
    }

    pub fn get_first_instance<'a>(&'a self) -> u32 {
        self.raw.firstInstance.into()
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

    pub fn index_count<'a>(&'a self) -> u32 {
        self.raw.indexCount.into()
    }

    pub fn instance_count<'a>(&'a self) -> u32 {
        self.raw.instanceCount.into()
    }

    pub fn first_index<'a>(&'a self) -> u32 {
        self.raw.firstIndex.into()
    }

    pub fn vertex_offset<'a>(&'a self) -> i32 {
        self.raw.vertexOffset.into()
    }

    pub fn first_instance<'a>(&'a self) -> u32 {
        self.raw.firstInstance.into()
    }

    pub fn set_index_count<'m>(& mut self, index_count: u32) {
        self.raw.indexCount = index_count.into();
    }

    pub fn set_instance_count<'m>(& mut self, instance_count: u32) {
        self.raw.instanceCount = instance_count.into();
    }

    pub fn set_first_index<'m>(& mut self, first_index: u32) {
        self.raw.firstIndex = first_index.into();
    }

    pub fn set_vertex_offset<'m>(& mut self, vertex_offset: i32) {
        self.raw.vertexOffset = vertex_offset.into();
    }

    pub fn set_first_instance<'m>(& mut self, first_instance: u32) {
        self.raw.firstInstance = first_instance.into();
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


impl From<vks::VkDrawIndexedIndirectCommand> for DrawIndexedIndirectCommand {
    fn from(f: vks::VkDrawIndexedIndirectCommand) -> DrawIndexedIndirectCommand {
        DrawIndexedIndirectCommand { raw: f, }
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

    pub fn get_index_count<'a>(&'a self) -> u32 {
        self.raw.indexCount.into()
    }

    pub fn get_instance_count<'a>(&'a self) -> u32 {
        self.raw.instanceCount.into()
    }

    pub fn get_first_index<'a>(&'a self) -> u32 {
        self.raw.firstIndex.into()
    }

    pub fn get_vertex_offset<'a>(&'a self) -> i32 {
        self.raw.vertexOffset.into()
    }

    pub fn get_first_instance<'a>(&'a self) -> u32 {
        self.raw.firstInstance.into()
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

    pub fn x<'a>(&'a self) -> u32 {
        self.raw.x.into()
    }

    pub fn y<'a>(&'a self) -> u32 {
        self.raw.y.into()
    }

    pub fn z<'a>(&'a self) -> u32 {
        self.raw.z.into()
    }

    pub fn set_x<'m>(& mut self, x: u32) {
        self.raw.x = x.into();
    }

    pub fn set_y<'m>(& mut self, y: u32) {
        self.raw.y = y.into();
    }

    pub fn set_z<'m>(& mut self, z: u32) {
        self.raw.z = z.into();
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


impl From<vks::VkDispatchIndirectCommand> for DispatchIndirectCommand {
    fn from(f: vks::VkDispatchIndirectCommand) -> DispatchIndirectCommand {
        DispatchIndirectCommand { raw: f, }
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

    pub fn get_x<'a>(&'a self) -> u32 {
        self.raw.x.into()
    }

    pub fn get_y<'a>(&'a self) -> u32 {
        self.raw.y.into()
    }

    pub fn get_z<'a>(&'a self) -> u32 {
        self.raw.z.into()
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
    signal_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    command_buffers: Option<SmallVec<[vks::VkCommandBuffer; 8]>>,
    wait_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> SubmitInfo<'s> {
    pub fn builder<'b>() -> SubmitInfoBuilder<'b> {
        SubmitInfoBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn wait_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphores as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn wait_dst_stage_mask<'a>(&'a self) -> &'a PipelineStageFlags {
        unsafe { &*(self.raw.pWaitDstStageMask as *const _) }
    }

    pub fn command_buffers_handle<'a>(&'a self) -> &'a [vks::VkCommandBuffer] {
        unsafe { slice::from_raw_parts(self.raw.pCommandBuffers as *const _, self.raw.commandBufferCount as usize) }
    }

    pub fn signal_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphores as *const _, self.raw.signalSemaphoreCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_wait_semaphores<'m, 'a>(&'s mut self, wait_semaphores: &'a [&'a Semaphore])
            where 'a: 's {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        {
            let wait_semaphores = self.wait_semaphores.as_ref().unwrap();
            self.raw.pWaitSemaphores = wait_semaphores.as_ptr();
            assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphores.len() as _, 
                "count inconsistency found when specifying `SubmitInfo::wait_semaphores`.");
            self.raw.waitSemaphoreCount = wait_semaphores.len() as _;
        }
    }

    pub fn set_wait_dst_stage_mask<'m, 'a>(&'s mut self, wait_dst_stage_mask: &'a PipelineStageFlags) {
        self.raw.pWaitDstStageMask = wait_dst_stage_mask as *const PipelineStageFlags as *const _;
    }

    pub fn set_command_buffers<'m, 'a>(&'s mut self, command_buffers: &'a [&'a CommandBuffer])
            where 'a: 's {
        self.command_buffers = Some(command_buffers.iter().map(|h| h.handle()).collect());
        {
            let command_buffers = self.command_buffers.as_ref().unwrap();
            self.raw.pCommandBuffers = command_buffers.as_ptr();
            assert!(self.raw.commandBufferCount == 0 || self.raw.commandBufferCount == command_buffers.len() as _, 
                "count inconsistency found when specifying `SubmitInfo::command_buffers`.");
            self.raw.commandBufferCount = command_buffers.len() as _;
        }
    }

    pub fn set_signal_semaphores<'m, 'a>(&'s mut self, signal_semaphores: &'a [&'a Semaphore])
            where 'a: 's {
        self.signal_semaphores = Some(signal_semaphores.iter().map(|h| h.handle()).collect());
        {
            let signal_semaphores = self.signal_semaphores.as_ref().unwrap();
            self.raw.pSignalSemaphores = signal_semaphores.as_ptr();
            assert!(self.raw.signalSemaphoreCount == 0 || self.raw.signalSemaphoreCount == signal_semaphores.len() as _, 
                "count inconsistency found when specifying `SubmitInfo::signal_semaphores`.");
            self.raw.signalSemaphoreCount = signal_semaphores.len() as _;
        }
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


impl<'s> From<vks::VkSubmitInfo> for SubmitInfo<'s> {
    fn from(f: vks::VkSubmitInfo) -> SubmitInfo<'s> {
        SubmitInfo { raw: f, signal_semaphores: None, command_buffers: None, wait_semaphores: None, _p: PhantomData }
    }
}


/// A builder for `VkSubmitInfo`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct SubmitInfoBuilder<'b> {
    raw: vks::VkSubmitInfo,
    signal_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    command_buffers: Option<SmallVec<[vks::VkCommandBuffer; 8]>>,
    wait_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    _p: PhantomData<&'b ()>, 
}

impl<'b> SubmitInfoBuilder<'b> {
    pub fn new() -> SubmitInfoBuilder<'b> {
        SubmitInfoBuilder {
            raw: vks::VkSubmitInfo::default(),
            signal_semaphores: None,
            command_buffers: None,
            wait_semaphores: None,
            _p: PhantomData,
        }
    }

    pub unsafe fn next<'m>(mut self, next: *const c_void) -> SubmitInfoBuilder<'b> {
        self.raw.pNext = next;
        self
    }

    pub fn wait_semaphores<'m, 'a>(mut self, wait_semaphores: &'a [&'a Semaphore]) -> SubmitInfoBuilder<'b>
            where 'a: 'b {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        {
            let wait_semaphores = self.wait_semaphores.as_ref().unwrap();
            self.raw.pWaitSemaphores = wait_semaphores.as_ptr();
            assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphores.len() as _, 
                "count inconsistency found when specifying `SubmitInfo::wait_semaphores`.");
            self.raw.waitSemaphoreCount = wait_semaphores.len() as _;
        }
        self
    }

    pub fn wait_dst_stage_mask<'m, 'a>(mut self, wait_dst_stage_mask: &'a PipelineStageFlags) -> SubmitInfoBuilder<'b> {
        self.raw.pWaitDstStageMask = wait_dst_stage_mask as *const PipelineStageFlags as *const _;
        self
    }

    pub fn command_buffers<'m, 'a>(mut self, command_buffers: &'a [&'a CommandBuffer]) -> SubmitInfoBuilder<'b>
            where 'a: 'b {
        self.command_buffers = Some(command_buffers.iter().map(|h| h.handle()).collect());
        {
            let command_buffers = self.command_buffers.as_ref().unwrap();
            self.raw.pCommandBuffers = command_buffers.as_ptr();
            assert!(self.raw.commandBufferCount == 0 || self.raw.commandBufferCount == command_buffers.len() as _, 
                "count inconsistency found when specifying `SubmitInfo::command_buffers`.");
            self.raw.commandBufferCount = command_buffers.len() as _;
        }
        self
    }

    pub fn signal_semaphores<'m, 'a>(mut self, signal_semaphores: &'a [&'a Semaphore]) -> SubmitInfoBuilder<'b>
            where 'a: 'b {
        self.signal_semaphores = Some(signal_semaphores.iter().map(|h| h.handle()).collect());
        {
            let signal_semaphores = self.signal_semaphores.as_ref().unwrap();
            self.raw.pSignalSemaphores = signal_semaphores.as_ptr();
            assert!(self.raw.signalSemaphoreCount == 0 || self.raw.signalSemaphoreCount == signal_semaphores.len() as _, 
                "count inconsistency found when specifying `SubmitInfo::signal_semaphores`.");
            self.raw.signalSemaphoreCount = signal_semaphores.len() as _;
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_wait_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphores as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn get_wait_dst_stage_mask<'a>(&'a self) -> &'a PipelineStageFlags {
        unsafe { &*(self.raw.pWaitDstStageMask as *const _) }
    }

    pub fn get_command_buffers_handle<'a>(&'a self) -> &'a [vks::VkCommandBuffer] {
        unsafe { slice::from_raw_parts(self.raw.pCommandBuffers as *const _, self.raw.commandBufferCount as usize) }
    }

    pub fn get_signal_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphores as *const _, self.raw.signalSemaphoreCount as usize) }
    }

    pub fn build(self) -> SubmitInfo<'b> {
        SubmitInfo {
            raw: self.raw,
            signal_semaphores: self.signal_semaphores,
            command_buffers: self.command_buffers,
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
    pub fn display_handle<'a>(&'a self) -> vks::VkDisplayKHR {
        self.raw.display
    }

    pub fn display_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.displayName) }
    }

    pub fn physical_dimensions<'a>(&'a self) -> Extent2d {
        self.raw.physicalDimensions.into()
    }

    pub fn physical_resolution<'a>(&'a self) -> Extent2d {
        self.raw.physicalResolution.into()
    }

    pub fn supported_transforms<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.supportedTransforms)
            .expect("DisplayPropertiesKhr::supported_transforms: error converting flags")
    }

    pub fn plane_reorder_possible<'a>(&'a self) -> bool {
        self.raw.planeReorderPossible != 0
    }

    pub fn persistent_content<'a>(&'a self) -> bool {
        self.raw.persistentContent != 0
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


impl<'s> From<vks::VkDisplayPropertiesKHR> for DisplayPropertiesKhr<'s> {
    fn from(f: vks::VkDisplayPropertiesKHR) -> DisplayPropertiesKhr<'s> {
        DisplayPropertiesKhr { raw: f, display_name: None, _p: PhantomData }
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
    pub fn current_display_handle<'a>(&'a self) -> vks::VkDisplayKHR {
        self.raw.currentDisplay
    }

    pub fn current_stack_index<'a>(&'a self) -> u32 {
        self.raw.currentStackIndex.into()
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


impl From<vks::VkDisplayPlanePropertiesKHR> for DisplayPlanePropertiesKhr {
    fn from(f: vks::VkDisplayPlanePropertiesKHR) -> DisplayPlanePropertiesKhr {
        DisplayPlanePropertiesKhr { raw: f, }
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

    pub fn visible_region<'a>(&'a self) -> Extent2d {
        self.raw.visibleRegion.into()
    }

    pub fn refresh_rate<'a>(&'a self) -> u32 {
        self.raw.refreshRate.into()
    }

    pub fn set_visible_region<'m>(& mut self, visible_region: Extent2d) {
        self.raw.visibleRegion = visible_region.raw;
    }

    pub fn set_refresh_rate<'m>(& mut self, refresh_rate: u32) {
        self.raw.refreshRate = refresh_rate.into();
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


impl From<vks::VkDisplayModeParametersKHR> for DisplayModeParametersKhr {
    fn from(f: vks::VkDisplayModeParametersKHR) -> DisplayModeParametersKhr {
        DisplayModeParametersKhr { raw: f, }
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

    pub fn get_visible_region<'a>(&'a self) -> Extent2d {
        self.raw.visibleRegion.into()
    }

    pub fn get_refresh_rate<'a>(&'a self) -> u32 {
        self.raw.refreshRate.into()
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
    pub fn display_mode_handle<'a>(&'a self) -> vks::VkDisplayModeKHR {
        self.raw.displayMode
    }

    pub fn parameters<'a>(&'a self) -> DisplayModeParametersKhr {
        self.raw.parameters.into()
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


impl From<vks::VkDisplayModePropertiesKHR> for DisplayModePropertiesKhr {
    fn from(f: vks::VkDisplayModePropertiesKHR) -> DisplayModePropertiesKhr {
        DisplayModePropertiesKhr { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DisplayModeCreateFlagsKhr {
        DisplayModeCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("DisplayModeCreateInfoKhr::flags: error converting flags")
    }

    pub fn parameters<'a>(&'a self) -> DisplayModeParametersKhr {
        self.raw.parameters.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DisplayModeCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_parameters<'m>(&'s mut self, parameters: DisplayModeParametersKhr) {
        self.raw.parameters = parameters.raw;
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


impl<'s> From<vks::VkDisplayModeCreateInfoKHR> for DisplayModeCreateInfoKhr<'s> {
    fn from(f: vks::VkDisplayModeCreateInfoKHR) -> DisplayModeCreateInfoKhr<'s> {
        DisplayModeCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DisplayModeCreateFlagsKhr {
        DisplayModeCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("DisplayModeCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_parameters<'a>(&'a self) -> DisplayModeParametersKhr {
        self.raw.parameters.into()
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
    pub fn supported_alpha<'a>(&'a self) -> DisplayPlaneAlphaFlagsKhr {
        DisplayPlaneAlphaFlagsKhr::from_bits(self.raw.supportedAlpha)
            .expect("DisplayPlaneCapabilitiesKhr::supported_alpha: error converting flags")
    }

    pub fn min_src_position<'a>(&'a self) -> Offset2d {
        self.raw.minSrcPosition.into()
    }

    pub fn max_src_position<'a>(&'a self) -> Offset2d {
        self.raw.maxSrcPosition.into()
    }

    pub fn min_src_extent<'a>(&'a self) -> Extent2d {
        self.raw.minSrcExtent.into()
    }

    pub fn max_src_extent<'a>(&'a self) -> Extent2d {
        self.raw.maxSrcExtent.into()
    }

    pub fn min_dst_position<'a>(&'a self) -> Offset2d {
        self.raw.minDstPosition.into()
    }

    pub fn max_dst_position<'a>(&'a self) -> Offset2d {
        self.raw.maxDstPosition.into()
    }

    pub fn min_dst_extent<'a>(&'a self) -> Extent2d {
        self.raw.minDstExtent.into()
    }

    pub fn max_dst_extent<'a>(&'a self) -> Extent2d {
        self.raw.maxDstExtent.into()
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


impl From<vks::VkDisplayPlaneCapabilitiesKHR> for DisplayPlaneCapabilitiesKhr {
    fn from(f: vks::VkDisplayPlaneCapabilitiesKHR) -> DisplayPlaneCapabilitiesKhr {
        DisplayPlaneCapabilitiesKhr { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DisplaySurfaceCreateFlagsKhr {
        DisplaySurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("DisplaySurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn display_mode_handle<'a>(&'a self) -> vks::VkDisplayModeKHR {
        self.raw.displayMode
    }

    pub fn plane_index<'a>(&'a self) -> u32 {
        self.raw.planeIndex.into()
    }

    pub fn plane_stack_index<'a>(&'a self) -> u32 {
        self.raw.planeStackIndex.into()
    }

    pub fn transform<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.transform)
            .expect("DisplaySurfaceCreateInfoKhr::transform: error converting flags")
    }

    pub fn global_alpha<'a>(&'a self) -> f32 {
        self.raw.globalAlpha.into()
    }

    pub fn alpha_mode<'a>(&'a self) -> DisplayPlaneAlphaFlagsKhr {
        DisplayPlaneAlphaFlagsKhr::from_bits(self.raw.alphaMode)
            .expect("DisplaySurfaceCreateInfoKhr::alpha_mode: error converting flags")
    }

    pub fn image_extent<'a>(&'a self) -> Extent2d {
        self.raw.imageExtent.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DisplaySurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_display_mode<'m, 'a>(&'s mut self, display_mode: &'a DisplayModeKhr) {
        self.raw.displayMode = display_mode.handle();
    }

    pub fn set_plane_index<'m>(&'s mut self, plane_index: u32) {
        self.raw.planeIndex = plane_index.into();
    }

    pub fn set_plane_stack_index<'m>(&'s mut self, plane_stack_index: u32) {
        self.raw.planeStackIndex = plane_stack_index.into();
    }

    pub fn set_transform<'m>(&'s mut self, transform: SurfaceTransformFlagsKhr) {
        self.raw.transform = transform.bits();
    }

    pub fn set_global_alpha<'m>(&'s mut self, global_alpha: f32) {
        self.raw.globalAlpha = global_alpha.into();
    }

    pub fn set_alpha_mode<'m>(&'s mut self, alpha_mode: DisplayPlaneAlphaFlagsKhr) {
        self.raw.alphaMode = alpha_mode.bits();
    }

    pub fn set_image_extent<'m>(&'s mut self, image_extent: Extent2d) {
        self.raw.imageExtent = image_extent.raw;
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


impl<'s> From<vks::VkDisplaySurfaceCreateInfoKHR> for DisplaySurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkDisplaySurfaceCreateInfoKHR) -> DisplaySurfaceCreateInfoKhr<'s> {
        DisplaySurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DisplaySurfaceCreateFlagsKhr {
        DisplaySurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("DisplaySurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_display_mode_handle<'a>(&'a self) -> vks::VkDisplayModeKHR {
        self.raw.displayMode
    }

    pub fn get_plane_index<'a>(&'a self) -> u32 {
        self.raw.planeIndex.into()
    }

    pub fn get_plane_stack_index<'a>(&'a self) -> u32 {
        self.raw.planeStackIndex.into()
    }

    pub fn get_transform<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.transform)
            .expect("DisplaySurfaceCreateInfoKhr::transform: error converting flags")
    }

    pub fn get_global_alpha<'a>(&'a self) -> f32 {
        self.raw.globalAlpha.into()
    }

    pub fn get_alpha_mode<'a>(&'a self) -> DisplayPlaneAlphaFlagsKhr {
        DisplayPlaneAlphaFlagsKhr::from_bits(self.raw.alphaMode)
            .expect("DisplaySurfaceCreateInfoKhr::alpha_mode: error converting flags")
    }

    pub fn get_image_extent<'a>(&'a self) -> Extent2d {
        self.raw.imageExtent.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn src_rect<'a>(&'a self) -> Rect2d {
        self.raw.srcRect.into()
    }

    pub fn dst_rect<'a>(&'a self) -> Rect2d {
        self.raw.dstRect.into()
    }

    pub fn persistent<'a>(&'a self) -> bool {
        self.raw.persistent != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_src_rect<'m>(&'s mut self, src_rect: Rect2d) {
        self.raw.srcRect = src_rect.raw;
    }

    pub fn set_dst_rect<'m>(&'s mut self, dst_rect: Rect2d) {
        self.raw.dstRect = dst_rect.raw;
    }

    pub fn set_persistent<'m>(&'s mut self, persistent: bool) {
        self.raw.persistent = persistent as u32;
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


impl<'s> From<vks::VkDisplayPresentInfoKHR> for DisplayPresentInfoKhr<'s> {
    fn from(f: vks::VkDisplayPresentInfoKHR) -> DisplayPresentInfoKhr<'s> {
        DisplayPresentInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_src_rect<'a>(&'a self) -> Rect2d {
        self.raw.srcRect.into()
    }

    pub fn get_dst_rect<'a>(&'a self) -> Rect2d {
        self.raw.dstRect.into()
    }

    pub fn get_persistent<'a>(&'a self) -> bool {
        self.raw.persistent != 0
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
    pub fn min_image_count<'a>(&'a self) -> u32 {
        self.raw.minImageCount.into()
    }

    pub fn max_image_count<'a>(&'a self) -> u32 {
        self.raw.maxImageCount.into()
    }

    pub fn current_extent<'a>(&'a self) -> Extent2d {
        self.raw.currentExtent.into()
    }

    pub fn min_image_extent<'a>(&'a self) -> Extent2d {
        self.raw.minImageExtent.into()
    }

    pub fn max_image_extent<'a>(&'a self) -> Extent2d {
        self.raw.maxImageExtent.into()
    }

    pub fn max_image_array_layers<'a>(&'a self) -> u32 {
        self.raw.maxImageArrayLayers.into()
    }

    pub fn supported_transforms<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.supportedTransforms)
            .expect("SurfaceCapabilitiesKhr::supported_transforms: error converting flags")
    }

    pub fn current_transform<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.currentTransform)
            .expect("SurfaceCapabilitiesKhr::current_transform: error converting flags")
    }

    pub fn supported_composite_alpha<'a>(&'a self) -> CompositeAlphaFlagsKhr {
        CompositeAlphaFlagsKhr::from_bits(self.raw.supportedCompositeAlpha)
            .expect("SurfaceCapabilitiesKhr::supported_composite_alpha: error converting flags")
    }

    pub fn supported_usage_flags<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.supportedUsageFlags)
            .expect("SurfaceCapabilitiesKhr::supported_usage_flags: error converting flags")
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


impl From<vks::VkSurfaceCapabilitiesKHR> for SurfaceCapabilitiesKhr {
    fn from(f: vks::VkSurfaceCapabilitiesKHR) -> SurfaceCapabilitiesKhr {
        SurfaceCapabilitiesKhr { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> AndroidSurfaceCreateFlagsKhr {
        AndroidSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("AndroidSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn window<'a>(&'a self) -> *mut ANativeWindow {
        self.raw.window
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: AndroidSurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_window<'m>(&'s mut self, window: *mut ANativeWindow) {
        self.raw.window = window;
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


impl<'s> From<vks::VkAndroidSurfaceCreateInfoKHR> for AndroidSurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkAndroidSurfaceCreateInfoKHR) -> AndroidSurfaceCreateInfoKhr<'s> {
        AndroidSurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> AndroidSurfaceCreateFlagsKhr {
        AndroidSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("AndroidSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_window<'a>(&'a self) -> *mut ANativeWindow {
        self.raw.window
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> MirSurfaceCreateFlagsKhr {
        MirSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("MirSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn connection<'a>(&'a self) -> *mut MirConnection {
        self.raw.connection
    }

    pub fn mir_surface<'a>(&'a self) -> *mut MirSurface {
        self.raw.mirSurface
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: MirSurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_connection<'m>(&'s mut self, connection: *mut MirConnection) {
        self.raw.connection = connection;
    }

    pub unsafe fn set_mir_surface<'m>(&'s mut self, mir_surface: *mut MirSurface) {
        self.raw.mirSurface = mir_surface;
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


impl<'s> From<vks::VkMirSurfaceCreateInfoKHR> for MirSurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkMirSurfaceCreateInfoKHR) -> MirSurfaceCreateInfoKhr<'s> {
        MirSurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> MirSurfaceCreateFlagsKhr {
        MirSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("MirSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_connection<'a>(&'a self) -> *mut MirConnection {
        self.raw.connection
    }

    pub fn get_mir_surface<'a>(&'a self) -> *mut MirSurface {
        self.raw.mirSurface
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> ViSurfaceCreateFlagsNn {
        ViSurfaceCreateFlagsNn::from_bits(self.raw.flags)
            .expect("ViSurfaceCreateInfoNn::flags: error converting flags")
    }

    pub fn window<'a>(&'a self) -> *mut c_void {
        self.raw.window
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: ViSurfaceCreateFlagsNn) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_window<'m>(&'s mut self, window: *mut c_void) {
        self.raw.window = window;
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


impl<'s> From<vks::VkViSurfaceCreateInfoNN> for ViSurfaceCreateInfoNn<'s> {
    fn from(f: vks::VkViSurfaceCreateInfoNN) -> ViSurfaceCreateInfoNn<'s> {
        ViSurfaceCreateInfoNn { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> ViSurfaceCreateFlagsNn {
        ViSurfaceCreateFlagsNn::from_bits(self.raw.flags)
            .expect("ViSurfaceCreateInfoNn::flags: error converting flags")
    }

    pub fn get_window<'a>(&'a self) -> *mut c_void {
        self.raw.window
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> WaylandSurfaceCreateFlagsKhr {
        WaylandSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("WaylandSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn display<'a>(&'a self) -> *mut wl_display {
        self.raw.display
    }

    pub fn surface<'a>(&'a self) -> *mut wl_surface {
        self.raw.surface
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: WaylandSurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_display<'m>(&'s mut self, display: *mut wl_display) {
        self.raw.display = display;
    }

    pub unsafe fn set_surface<'m>(&'s mut self, surface: *mut wl_surface) {
        self.raw.surface = surface;
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


impl<'s> From<vks::VkWaylandSurfaceCreateInfoKHR> for WaylandSurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkWaylandSurfaceCreateInfoKHR) -> WaylandSurfaceCreateInfoKhr<'s> {
        WaylandSurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> WaylandSurfaceCreateFlagsKhr {
        WaylandSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("WaylandSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_display<'a>(&'a self) -> *mut wl_display {
        self.raw.display
    }

    pub fn get_surface<'a>(&'a self) -> *mut wl_surface {
        self.raw.surface
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> Win32SurfaceCreateFlagsKhr {
        Win32SurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("Win32SurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn hinstance<'a>(&'a self) -> HINSTANCE {
        self.raw.hinstance.into()
    }

    pub fn hwnd<'a>(&'a self) -> HWND {
        self.raw.hwnd.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: Win32SurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_hinstance<'m>(&'s mut self, hinstance: HINSTANCE) {
        self.raw.hinstance = hinstance.into();
    }

    pub fn set_hwnd<'m>(&'s mut self, hwnd: HWND) {
        self.raw.hwnd = hwnd.into();
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


impl<'s> From<vks::VkWin32SurfaceCreateInfoKHR> for Win32SurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkWin32SurfaceCreateInfoKHR) -> Win32SurfaceCreateInfoKhr<'s> {
        Win32SurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> Win32SurfaceCreateFlagsKhr {
        Win32SurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("Win32SurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_hinstance<'a>(&'a self) -> HINSTANCE {
        self.raw.hinstance.into()
    }

    pub fn get_hwnd<'a>(&'a self) -> HWND {
        self.raw.hwnd.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> XlibSurfaceCreateFlagsKhr {
        XlibSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("XlibSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn dpy<'a>(&'a self) -> *mut Display {
        self.raw.dpy
    }

    pub fn window<'a>(&'a self) -> u32 {
        self.raw.window.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: XlibSurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_dpy<'m>(&'s mut self, dpy: *mut Display) {
        self.raw.dpy = dpy;
    }

    pub unsafe fn set_window<'m>(&'s mut self, window: u32) {
        self.raw.window = window.into();
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


impl<'s> From<vks::VkXlibSurfaceCreateInfoKHR> for XlibSurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkXlibSurfaceCreateInfoKHR) -> XlibSurfaceCreateInfoKhr<'s> {
        XlibSurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> XlibSurfaceCreateFlagsKhr {
        XlibSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("XlibSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_dpy<'a>(&'a self) -> *mut Display {
        self.raw.dpy
    }

    pub fn get_window<'a>(&'a self) -> u32 {
        self.raw.window.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> XcbSurfaceCreateFlagsKhr {
        XcbSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("XcbSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn connection<'a>(&'a self) -> *mut xcb_connection_t {
        self.raw.connection
    }

    pub fn window<'a>(&'a self) -> xcb_window_t {
        self.raw.window.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: XcbSurfaceCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_connection<'m>(&'s mut self, connection: *mut xcb_connection_t) {
        self.raw.connection = connection;
    }

    pub unsafe fn set_window<'m>(&'s mut self, window: xcb_window_t) {
        self.raw.window = window.into();
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


impl<'s> From<vks::VkXcbSurfaceCreateInfoKHR> for XcbSurfaceCreateInfoKhr<'s> {
    fn from(f: vks::VkXcbSurfaceCreateInfoKHR) -> XcbSurfaceCreateInfoKhr<'s> {
        XcbSurfaceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> XcbSurfaceCreateFlagsKhr {
        XcbSurfaceCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("XcbSurfaceCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_connection<'a>(&'a self) -> *mut xcb_connection_t {
        self.raw.connection
    }

    pub fn get_window<'a>(&'a self) -> xcb_window_t {
        self.raw.window.into()
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
    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn color_space<'a>(&'a self) -> ColorSpaceKhr {
        self.raw.colorSpace.into()
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


impl From<vks::VkSurfaceFormatKHR> for SurfaceFormatKhr {
    fn from(f: vks::VkSurfaceFormatKHR) -> SurfaceFormatKhr {
        SurfaceFormatKhr { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> SwapchainCreateFlagsKhr {
        SwapchainCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("SwapchainCreateInfoKhr::flags: error converting flags")
    }

    pub fn surface_handle<'a>(&'a self) -> vks::VkSurfaceKHR {
        self.raw.surface
    }

    pub fn min_image_count<'a>(&'a self) -> u32 {
        self.raw.minImageCount.into()
    }

    pub fn image_format<'a>(&'a self) -> Format {
        self.raw.imageFormat.into()
    }

    pub fn image_color_space<'a>(&'a self) -> ColorSpaceKhr {
        self.raw.imageColorSpace.into()
    }

    pub fn image_extent<'a>(&'a self) -> Extent2d {
        self.raw.imageExtent.into()
    }

    pub fn image_array_layers<'a>(&'a self) -> u32 {
        self.raw.imageArrayLayers.into()
    }

    pub fn image_usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.imageUsage)
            .expect("SwapchainCreateInfoKhr::image_usage: error converting flags")
    }

    pub fn image_sharing_mode<'a>(&'a self) -> SharingMode {
        self.raw.imageSharingMode.into()
    }

    pub fn queue_family_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pQueueFamilyIndices as *const _, self.raw.queueFamilyIndexCount as usize) }
    }

    pub fn pre_transform<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.preTransform)
            .expect("SwapchainCreateInfoKhr::pre_transform: error converting flags")
    }

    pub fn composite_alpha<'a>(&'a self) -> CompositeAlphaFlagsKhr {
        CompositeAlphaFlagsKhr::from_bits(self.raw.compositeAlpha)
            .expect("SwapchainCreateInfoKhr::composite_alpha: error converting flags")
    }

    pub fn present_mode<'a>(&'a self) -> PresentModeKhr {
        self.raw.presentMode.into()
    }

    pub fn clipped<'a>(&'a self) -> bool {
        self.raw.clipped != 0
    }

    pub fn old_swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.oldSwapchain
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: SwapchainCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_surface<'m, 'a>(&'s mut self, surface: &'a Surface) {
        self.raw.surface = surface.handle();
    }

    pub fn set_min_image_count<'m>(&'s mut self, min_image_count: u32) {
        self.raw.minImageCount = min_image_count.into();
    }

    pub fn set_image_format<'m>(&'s mut self, image_format: Format) {
        self.raw.imageFormat = image_format.into();
    }

    pub fn set_image_color_space<'m>(&'s mut self, image_color_space: ColorSpaceKhr) {
        self.raw.imageColorSpace = image_color_space.into();
    }

    pub fn set_image_extent<'m>(&'s mut self, image_extent: Extent2d) {
        self.raw.imageExtent = image_extent.raw;
    }

    pub fn set_image_array_layers<'m>(&'s mut self, image_array_layers: u32) {
        self.raw.imageArrayLayers = image_array_layers.into();
    }

    pub fn set_image_usage<'m>(&'s mut self, image_usage: ImageUsageFlags) {
        self.raw.imageUsage = image_usage.bits();
    }

    pub fn set_image_sharing_mode<'m>(&'s mut self, image_sharing_mode: SharingMode) {
        self.raw.imageSharingMode = image_sharing_mode.into();
    }

    pub fn set_queue_family_indices<'m, 'a>(&'s mut self, queue_family_indices: &'a [u32]) {
        assert!(self.raw.queueFamilyIndexCount == 0 || self.raw.queueFamilyIndexCount == queue_family_indices.len() as _, 
            "count inconsistency found when specifying `SwapchainCreateInfoKhr::queue_family_indices`.");
        self.raw.queueFamilyIndexCount = queue_family_indices.len() as _;
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const u32 as *const _;
    }

    pub fn set_pre_transform<'m>(&'s mut self, pre_transform: SurfaceTransformFlagsKhr) {
        self.raw.preTransform = pre_transform.bits();
    }

    pub fn set_composite_alpha<'m>(&'s mut self, composite_alpha: CompositeAlphaFlagsKhr) {
        self.raw.compositeAlpha = composite_alpha.bits();
    }

    pub fn set_present_mode<'m>(&'s mut self, present_mode: PresentModeKhr) {
        self.raw.presentMode = present_mode.into();
    }

    pub fn set_clipped<'m>(&'s mut self, clipped: bool) {
        self.raw.clipped = clipped as u32;
    }

    pub fn set_old_swapchain<'m, 'a>(&'s mut self, old_swapchain: &'a Swapchain) {
        self.raw.oldSwapchain = old_swapchain.handle();
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


impl<'s> From<vks::VkSwapchainCreateInfoKHR> for SwapchainCreateInfoKhr<'s> {
    fn from(f: vks::VkSwapchainCreateInfoKHR) -> SwapchainCreateInfoKhr<'s> {
        SwapchainCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn queue_family_indices<'m, 'a>(mut self, queue_family_indices: &'a [u32]) -> SwapchainCreateInfoKhrBuilder<'b> {
        assert!(self.raw.queueFamilyIndexCount == 0 || self.raw.queueFamilyIndexCount == queue_family_indices.len() as _, 
            "count inconsistency found when specifying `SwapchainCreateInfoKhr::queue_family_indices`.");
        self.raw.queueFamilyIndexCount = queue_family_indices.len() as _;
        self.raw.pQueueFamilyIndices = queue_family_indices.as_ptr() as *const u32 as *const _;
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> SwapchainCreateFlagsKhr {
        SwapchainCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("SwapchainCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_surface_handle<'a>(&'a self) -> vks::VkSurfaceKHR {
        self.raw.surface
    }

    pub fn get_min_image_count<'a>(&'a self) -> u32 {
        self.raw.minImageCount.into()
    }

    pub fn get_image_format<'a>(&'a self) -> Format {
        self.raw.imageFormat.into()
    }

    pub fn get_image_color_space<'a>(&'a self) -> ColorSpaceKhr {
        self.raw.imageColorSpace.into()
    }

    pub fn get_image_extent<'a>(&'a self) -> Extent2d {
        self.raw.imageExtent.into()
    }

    pub fn get_image_array_layers<'a>(&'a self) -> u32 {
        self.raw.imageArrayLayers.into()
    }

    pub fn get_image_usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.imageUsage)
            .expect("SwapchainCreateInfoKhr::image_usage: error converting flags")
    }

    pub fn get_image_sharing_mode<'a>(&'a self) -> SharingMode {
        self.raw.imageSharingMode.into()
    }

    pub fn get_queue_family_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pQueueFamilyIndices as *const _, self.raw.queueFamilyIndexCount as usize) }
    }

    pub fn get_pre_transform<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.preTransform)
            .expect("SwapchainCreateInfoKhr::pre_transform: error converting flags")
    }

    pub fn get_composite_alpha<'a>(&'a self) -> CompositeAlphaFlagsKhr {
        CompositeAlphaFlagsKhr::from_bits(self.raw.compositeAlpha)
            .expect("SwapchainCreateInfoKhr::composite_alpha: error converting flags")
    }

    pub fn get_present_mode<'a>(&'a self) -> PresentModeKhr {
        self.raw.presentMode.into()
    }

    pub fn get_clipped<'a>(&'a self) -> bool {
        self.raw.clipped != 0
    }

    pub fn get_old_swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.oldSwapchain
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
    swapchains: Option<SmallVec<[vks::VkSwapchainKHR; 8]>>,
    wait_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> PresentInfoKhr<'s> {
    pub fn builder<'b>() -> PresentInfoKhrBuilder<'b> {
        PresentInfoKhrBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn wait_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphores as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn swapchains_handle<'a>(&'a self) -> &'a [vks::VkSwapchainKHR] {
        unsafe { slice::from_raw_parts(self.raw.pSwapchains as *const _, self.raw.swapchainCount as usize) }
    }

    pub fn image_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pImageIndices as *const _, self.raw.swapchainCount as usize) }
    }

    pub fn results<'a>(&'a self) -> &'a [ResultEnum] {
        unsafe { slice::from_raw_parts(self.raw.pResults as *const _, self.raw.swapchainCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_wait_semaphores<'m, 'a>(&'s mut self, wait_semaphores: &'a [&'a Semaphore])
            where 'a: 's {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        {
            let wait_semaphores = self.wait_semaphores.as_ref().unwrap();
            self.raw.pWaitSemaphores = wait_semaphores.as_ptr();
            assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphores.len() as _, 
                "count inconsistency found when specifying `PresentInfoKhr::wait_semaphores`.");
            self.raw.waitSemaphoreCount = wait_semaphores.len() as _;
        }
    }

    pub fn set_swapchains<'m, 'a>(&'s mut self, swapchains: &'a [&'a Swapchain])
            where 'a: 's {
        self.swapchains = Some(swapchains.iter().map(|h| h.handle()).collect());
        {
            let swapchains = self.swapchains.as_ref().unwrap();
            self.raw.pSwapchains = swapchains.as_ptr();
            assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == swapchains.len() as _, 
                "count inconsistency found when specifying `PresentInfoKhr::swapchains`.");
            self.raw.swapchainCount = swapchains.len() as _;
        }
    }

    pub fn set_image_indices<'m, 'a>(&'s mut self, image_indices: &'a [u32]) {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == image_indices.len() as _, 
            "count inconsistency found when specifying `PresentInfoKhr::image_indices`.");
        self.raw.swapchainCount = image_indices.len() as _;
        self.raw.pImageIndices = image_indices.as_ptr() as *const u32 as *const _;
    }

    pub fn set_results<'m, 'a>(&'s mut self, results: &'a mut [ResultEnum]) {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == results.len() as _, 
            "count inconsistency found when specifying `PresentInfoKhr::results`.");
        self.raw.swapchainCount = results.len() as _;
        self.raw.pResults = results.as_mut_ptr() as *mut ResultEnum as *mut _;
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


impl<'s> From<vks::VkPresentInfoKHR> for PresentInfoKhr<'s> {
    fn from(f: vks::VkPresentInfoKHR) -> PresentInfoKhr<'s> {
        PresentInfoKhr { raw: f, swapchains: None, wait_semaphores: None, _p: PhantomData }
    }
}


/// A builder for `VkPresentInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct PresentInfoKhrBuilder<'b> {
    raw: vks::VkPresentInfoKHR,
    swapchains: Option<SmallVec<[vks::VkSwapchainKHR; 8]>>,
    wait_semaphores: Option<SmallVec<[vks::VkSemaphore; 8]>>,
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

    pub fn wait_semaphores<'m, 'a>(mut self, wait_semaphores: &'a [&'a Semaphore]) -> PresentInfoKhrBuilder<'b>
            where 'a: 'b {
        self.wait_semaphores = Some(wait_semaphores.iter().map(|h| h.handle()).collect());
        {
            let wait_semaphores = self.wait_semaphores.as_ref().unwrap();
            self.raw.pWaitSemaphores = wait_semaphores.as_ptr();
            assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphores.len() as _, 
                "count inconsistency found when specifying `PresentInfoKhr::wait_semaphores`.");
            self.raw.waitSemaphoreCount = wait_semaphores.len() as _;
        }
        self
    }

    pub fn swapchains<'m, 'a>(mut self, swapchains: &'a [&'a Swapchain]) -> PresentInfoKhrBuilder<'b>
            where 'a: 'b {
        self.swapchains = Some(swapchains.iter().map(|h| h.handle()).collect());
        {
            let swapchains = self.swapchains.as_ref().unwrap();
            self.raw.pSwapchains = swapchains.as_ptr();
            assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == swapchains.len() as _, 
                "count inconsistency found when specifying `PresentInfoKhr::swapchains`.");
            self.raw.swapchainCount = swapchains.len() as _;
        }
        self
    }

    pub fn image_indices<'m, 'a>(mut self, image_indices: &'a [u32]) -> PresentInfoKhrBuilder<'b> {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == image_indices.len() as _, 
            "count inconsistency found when specifying `PresentInfoKhr::image_indices`.");
        self.raw.swapchainCount = image_indices.len() as _;
        self.raw.pImageIndices = image_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn results<'m, 'a>(mut self, results: &'a mut [ResultEnum]) -> PresentInfoKhrBuilder<'b> {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == results.len() as _, 
            "count inconsistency found when specifying `PresentInfoKhr::results`.");
        self.raw.swapchainCount = results.len() as _;
        self.raw.pResults = results.as_mut_ptr() as *mut ResultEnum as *mut _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_wait_semaphores_handle<'a>(&'a self) -> &'a [vks::VkSemaphore] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphores as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn get_swapchains_handle<'a>(&'a self) -> &'a [vks::VkSwapchainKHR] {
        unsafe { slice::from_raw_parts(self.raw.pSwapchains as *const _, self.raw.swapchainCount as usize) }
    }

    pub fn get_image_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pImageIndices as *const _, self.raw.swapchainCount as usize) }
    }

    pub fn get_results<'a>(&'a self) -> &'a [ResultEnum] {
        unsafe { slice::from_raw_parts(self.raw.pResults as *const _, self.raw.swapchainCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DebugReportFlagsExt {
        DebugReportFlagsExt::from_bits(self.raw.flags)
            .expect("DebugReportCallbackCreateInfoExt::flags: error converting flags")
    }

    pub fn pfn_callback<'a>(&'a self) -> PFN_vkDebugReportCallbackEXT {
        self.raw.pfnCallback.into()
    }

    pub fn user_data<'a>(&'a self) -> *mut c_void {
        self.raw.pUserData
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DebugReportFlagsExt) {
        self.raw.flags = flags.bits();
    }

    pub fn set_pfn_callback<'m>(&'s mut self, pfn_callback: PFN_vkDebugReportCallbackEXT) {
        self.raw.pfnCallback = pfn_callback.into();
    }

    pub unsafe fn set_user_data<'m>(&'s mut self, user_data: *mut c_void) {
        self.raw.pUserData = user_data;
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


impl<'s> From<vks::VkDebugReportCallbackCreateInfoEXT> for DebugReportCallbackCreateInfoExt<'s> {
    fn from(f: vks::VkDebugReportCallbackCreateInfoEXT) -> DebugReportCallbackCreateInfoExt<'s> {
        DebugReportCallbackCreateInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DebugReportFlagsExt {
        DebugReportFlagsExt::from_bits(self.raw.flags)
            .expect("DebugReportCallbackCreateInfoExt::flags: error converting flags")
    }

    pub fn get_pfn_callback<'a>(&'a self) -> PFN_vkDebugReportCallbackEXT {
        self.raw.pfnCallback.into()
    }

    pub fn get_user_data<'a>(&'a self) -> *mut c_void {
        self.raw.pUserData
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn disabled_validation_checks<'a>(&'a self) -> &'a [ValidationCheckExt] {
        unsafe { slice::from_raw_parts(self.raw.pDisabledValidationChecks as *const _, self.raw.disabledValidationCheckCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_disabled_validation_checks<'m, 'a>(&'s mut self, disabled_validation_checks: &'a mut [ValidationCheckExt]) {
        assert!(self.raw.disabledValidationCheckCount == 0 || self.raw.disabledValidationCheckCount == disabled_validation_checks.len() as _, 
            "count inconsistency found when specifying `ValidationFlagsExt::disabled_validation_checks`.");
        self.raw.disabledValidationCheckCount = disabled_validation_checks.len() as _;
        self.raw.pDisabledValidationChecks = disabled_validation_checks.as_mut_ptr() as *mut ValidationCheckExt as *mut _;
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


impl<'s> From<vks::VkValidationFlagsEXT> for ValidationFlagsExt<'s> {
    fn from(f: vks::VkValidationFlagsEXT) -> ValidationFlagsExt<'s> {
        ValidationFlagsExt { raw: f, _p: PhantomData }
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

    pub fn disabled_validation_checks<'m, 'a>(mut self, disabled_validation_checks: &'a mut [ValidationCheckExt]) -> ValidationFlagsExtBuilder<'b> {
        assert!(self.raw.disabledValidationCheckCount == 0 || self.raw.disabledValidationCheckCount == disabled_validation_checks.len() as _, 
            "count inconsistency found when specifying `ValidationFlagsExt::disabled_validation_checks`.");
        self.raw.disabledValidationCheckCount = disabled_validation_checks.len() as _;
        self.raw.pDisabledValidationChecks = disabled_validation_checks.as_mut_ptr() as *mut ValidationCheckExt as *mut _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_disabled_validation_checks<'a>(&'a self) -> &'a [ValidationCheckExt] {
        unsafe { slice::from_raw_parts(self.raw.pDisabledValidationChecks as *const _, self.raw.disabledValidationCheckCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn rasterization_order<'a>(&'a self) -> RasterizationOrderAmd {
        self.raw.rasterizationOrder.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_rasterization_order<'m>(&'s mut self, rasterization_order: RasterizationOrderAmd) {
        self.raw.rasterizationOrder = rasterization_order.into();
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


impl<'s> From<vks::VkPipelineRasterizationStateRasterizationOrderAMD> for PipelineRasterizationStateRasterizationOrderAmd<'s> {
    fn from(f: vks::VkPipelineRasterizationStateRasterizationOrderAMD) -> PipelineRasterizationStateRasterizationOrderAmd<'s> {
        PipelineRasterizationStateRasterizationOrderAmd { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_rasterization_order<'a>(&'a self) -> RasterizationOrderAmd {
        self.raw.rasterizationOrder.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn object_type<'a>(&'a self) -> DebugReportObjectTypeExt {
        self.raw.objectType.into()
    }

    pub fn object<'a>(&'a self) -> u64 {
        self.raw.object.into()
    }

    pub fn object_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pObjectName) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_object_type<'m>(&'s mut self, object_type: DebugReportObjectTypeExt) {
        self.raw.objectType = object_type.into();
    }

    pub fn set_object<'m>(&'s mut self, object: u64) {
        self.raw.object = object.into();
    }

    pub fn set_object_name<'m, 'a, T>(&'s mut self, object_name: T)
            where 'a: 's, T: Into<CharStr<'a>> {
        self.object_name = Some(object_name.into());
        {
            let object_name = self.object_name.as_ref().unwrap();
            self.raw.pObjectName = object_name.as_ptr();
        }
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


impl<'s> From<vks::VkDebugMarkerObjectNameInfoEXT> for DebugMarkerObjectNameInfoExt<'s> {
    fn from(f: vks::VkDebugMarkerObjectNameInfoEXT) -> DebugMarkerObjectNameInfoExt<'s> {
        DebugMarkerObjectNameInfoExt { raw: f, object_name: None, _p: PhantomData }
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

    pub fn object_name<'m, 'a, T>(mut self, object_name: T) -> DebugMarkerObjectNameInfoExtBuilder<'b>
            where 'a: 'b, T: Into<CharStr<'a>> {
        self.object_name = Some(object_name.into());
        {
            let object_name = self.object_name.as_ref().unwrap();
            self.raw.pObjectName = object_name.as_ptr();
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_object_type<'a>(&'a self) -> DebugReportObjectTypeExt {
        self.raw.objectType.into()
    }

    pub fn get_object<'a>(&'a self) -> u64 {
        self.raw.object.into()
    }

    pub fn get_object_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pObjectName) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn object_type<'a>(&'a self) -> DebugReportObjectTypeExt {
        self.raw.objectType.into()
    }

    pub fn object<'a>(&'a self) -> u64 {
        self.raw.object.into()
    }

    pub fn tag_name<'a>(&'a self) -> u64 {
        self.raw.tagName.into()
    }

    pub fn tag_size<'a>(&'a self) -> usize {
        self.raw.tagSize.into()
    }

    pub fn tag<'a>(&'a self) -> *const c_void {
        self.raw.pTag
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_object_type<'m>(&'s mut self, object_type: DebugReportObjectTypeExt) {
        self.raw.objectType = object_type.into();
    }

    pub fn set_object<'m>(&'s mut self, object: u64) {
        self.raw.object = object.into();
    }

    pub fn set_tag_name<'m>(&'s mut self, tag_name: u64) {
        self.raw.tagName = tag_name.into();
    }

    pub fn set_tag_size<'m>(&'s mut self, tag_size: usize) {
        self.raw.tagSize = tag_size.into();
    }

    pub unsafe fn set_tag<'m>(&'s mut self, tag: *const c_void) {
        self.raw.pTag = tag;
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


impl<'s> From<vks::VkDebugMarkerObjectTagInfoEXT> for DebugMarkerObjectTagInfoExt<'s> {
    fn from(f: vks::VkDebugMarkerObjectTagInfoEXT) -> DebugMarkerObjectTagInfoExt<'s> {
        DebugMarkerObjectTagInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_object_type<'a>(&'a self) -> DebugReportObjectTypeExt {
        self.raw.objectType.into()
    }

    pub fn get_object<'a>(&'a self) -> u64 {
        self.raw.object.into()
    }

    pub fn get_tag_name<'a>(&'a self) -> u64 {
        self.raw.tagName.into()
    }

    pub fn get_tag_size<'a>(&'a self) -> usize {
        self.raw.tagSize.into()
    }

    pub fn get_tag<'a>(&'a self) -> *const c_void {
        self.raw.pTag
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn marker_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pMarkerName) }
    }

    pub fn color<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.color as *const _, 4 as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_marker_name<'m, 'a, T>(&'s mut self, marker_name: T)
            where 'a: 's, T: Into<CharStr<'a>> {
        self.marker_name = Some(marker_name.into());
        {
            let marker_name = self.marker_name.as_ref().unwrap();
            self.raw.pMarkerName = marker_name.as_ptr();
        }
    }

    pub fn set_color<'m>(&'s mut self, color: [f32; 4]) {
        self.raw.color = color;
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


impl<'s> From<vks::VkDebugMarkerMarkerInfoEXT> for DebugMarkerMarkerInfoExt<'s> {
    fn from(f: vks::VkDebugMarkerMarkerInfoEXT) -> DebugMarkerMarkerInfoExt<'s> {
        DebugMarkerMarkerInfoExt { raw: f, marker_name: None, _p: PhantomData }
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

    pub fn marker_name<'m, 'a, T>(mut self, marker_name: T) -> DebugMarkerMarkerInfoExtBuilder<'b>
            where 'a: 'b, T: Into<CharStr<'a>> {
        self.marker_name = Some(marker_name.into());
        {
            let marker_name = self.marker_name.as_ref().unwrap();
            self.raw.pMarkerName = marker_name.as_ptr();
        }
        self
    }

    pub fn color<'m>(mut self, color: [f32; 4]) -> DebugMarkerMarkerInfoExtBuilder<'b> {
        self.raw.color = color;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_marker_name<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pMarkerName) }
    }

    pub fn get_color<'a>(&'a self) -> &[f32] {
        unsafe { slice::from_raw_parts(&self.raw.color as *const _, 4 as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn dedicated_allocation<'a>(&'a self) -> bool {
        self.raw.dedicatedAllocation != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_dedicated_allocation<'m>(&'s mut self, dedicated_allocation: bool) {
        self.raw.dedicatedAllocation = dedicated_allocation as u32;
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


impl<'s> From<vks::VkDedicatedAllocationImageCreateInfoNV> for DedicatedAllocationImageCreateInfoNv<'s> {
    fn from(f: vks::VkDedicatedAllocationImageCreateInfoNV) -> DedicatedAllocationImageCreateInfoNv<'s> {
        DedicatedAllocationImageCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_dedicated_allocation<'a>(&'a self) -> bool {
        self.raw.dedicatedAllocation != 0
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn dedicated_allocation<'a>(&'a self) -> bool {
        self.raw.dedicatedAllocation != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_dedicated_allocation<'m>(&'s mut self, dedicated_allocation: bool) {
        self.raw.dedicatedAllocation = dedicated_allocation as u32;
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


impl<'s> From<vks::VkDedicatedAllocationBufferCreateInfoNV> for DedicatedAllocationBufferCreateInfoNv<'s> {
    fn from(f: vks::VkDedicatedAllocationBufferCreateInfoNV) -> DedicatedAllocationBufferCreateInfoNv<'s> {
        DedicatedAllocationBufferCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_dedicated_allocation<'a>(&'a self) -> bool {
        self.raw.dedicatedAllocation != 0
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
    }

    pub fn set_buffer<'m, 'a>(&'s mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
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


impl<'s> From<vks::VkDedicatedAllocationMemoryAllocateInfoNV> for DedicatedAllocationMemoryAllocateInfoNv<'s> {
    fn from(f: vks::VkDedicatedAllocationMemoryAllocateInfoNV) -> DedicatedAllocationMemoryAllocateInfoNv<'s> {
        DedicatedAllocationMemoryAllocateInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
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
    pub fn image_format_properties<'a>(&'a self) -> ImageFormatProperties {
        self.raw.imageFormatProperties.into()
    }

    pub fn external_memory_features<'a>(&'a self) -> ExternalMemoryFeatureFlagsNv {
        ExternalMemoryFeatureFlagsNv::from_bits(self.raw.externalMemoryFeatures)
            .expect("ExternalImageFormatPropertiesNv::external_memory_features: error converting flags")
    }

    pub fn export_from_imported_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.exportFromImportedHandleTypes)
            .expect("ExternalImageFormatPropertiesNv::export_from_imported_handle_types: error converting flags")
    }

    pub fn compatible_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.compatibleHandleTypes)
            .expect("ExternalImageFormatPropertiesNv::compatible_handle_types: error converting flags")
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


impl From<vks::VkExternalImageFormatPropertiesNV> for ExternalImageFormatPropertiesNv {
    fn from(f: vks::VkExternalImageFormatPropertiesNV) -> ExternalImageFormatPropertiesNv {
        ExternalImageFormatPropertiesNv { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.handleTypes)
            .expect("ExternalMemoryImageCreateInfoNv::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalMemoryHandleTypeFlagsNv) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExternalMemoryImageCreateInfoNV> for ExternalMemoryImageCreateInfoNv<'s> {
    fn from(f: vks::VkExternalMemoryImageCreateInfoNV) -> ExternalMemoryImageCreateInfoNv<'s> {
        ExternalMemoryImageCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.handleTypes)
            .expect("ExternalMemoryImageCreateInfoNv::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.handleTypes)
            .expect("ExportMemoryAllocateInfoNv::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalMemoryHandleTypeFlagsNv) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExportMemoryAllocateInfoNV> for ExportMemoryAllocateInfoNv<'s> {
    fn from(f: vks::VkExportMemoryAllocateInfoNV) -> ExportMemoryAllocateInfoNv<'s> {
        ExportMemoryAllocateInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.handleTypes)
            .expect("ExportMemoryAllocateInfoNv::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.handleType)
            .expect("ImportMemoryWin32HandleInfoNv::handle_type: error converting flags")
    }

    pub fn handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsNv) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_handle<'m>(&'s mut self, handle: HANDLE) {
        self.raw.handle = handle.into();
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


impl<'s> From<vks::VkImportMemoryWin32HandleInfoNV> for ImportMemoryWin32HandleInfoNv<'s> {
    fn from(f: vks::VkImportMemoryWin32HandleInfoNV) -> ImportMemoryWin32HandleInfoNv<'s> {
        ImportMemoryWin32HandleInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsNv {
        ExternalMemoryHandleTypeFlagsNv::from_bits(self.raw.handleType)
            .expect("ImportMemoryWin32HandleInfoNv::handle_type: error converting flags")
    }

    pub fn get_handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_attributes<'m, 'a>(&'s mut self, attributes: &'a SECURITY_ATTRIBUTES) {
        self.raw.pAttributes = attributes;
    }

    pub fn set_dw_access<'m>(&'s mut self, dw_access: DWORD) {
        self.raw.dwAccess = dw_access.into();
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


impl<'s> From<vks::VkExportMemoryWin32HandleInfoNV> for ExportMemoryWin32HandleInfoNv<'s> {
    fn from(f: vks::VkExportMemoryWin32HandleInfoNV) -> ExportMemoryWin32HandleInfoNv<'s> {
        ExportMemoryWin32HandleInfoNv { raw: f, _p: PhantomData }
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

    pub fn attributes<'m, 'a>(mut self, attributes: &'a SECURITY_ATTRIBUTES) -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.pAttributes = attributes;
        self
    }

    pub fn dw_access<'m>(mut self, dw_access: DWORD) -> ExportMemoryWin32HandleInfoNvBuilder<'b> {
        self.raw.dwAccess = dw_access.into();
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn get_dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
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
    acquire_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
    release_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> Win32KeyedMutexAcquireReleaseInfoNv<'s> {
    pub fn builder<'b>() -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoNvBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn acquire_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireSyncs as *const _, self.raw.acquireCount as usize) }
    }

    pub fn acquire_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireKeys as *const _, self.raw.acquireCount as usize) }
    }

    pub fn acquire_timeout_milliseconds<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireTimeoutMilliseconds as *const _, self.raw.acquireCount as usize) }
    }

    pub fn release_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseSyncs as *const _, self.raw.releaseCount as usize) }
    }

    pub fn release_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseKeys as *const _, self.raw.releaseCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_acquire_syncs<'m, 'a>(&'s mut self, acquire_syncs: &'a [&'a DeviceMemory])
            where 'a: 's {
        self.acquire_syncs = Some(acquire_syncs.iter().map(|h| h.handle()).collect());
        {
            let acquire_syncs = self.acquire_syncs.as_ref().unwrap();
            self.raw.pAcquireSyncs = acquire_syncs.as_ptr();
            assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::acquire_syncs`.");
            self.raw.acquireCount = acquire_syncs.len() as _;
        }
    }

    pub fn set_acquire_keys<'m, 'a>(&'s mut self, acquire_keys: &'a [u64]) {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::acquire_keys`.");
        self.raw.acquireCount = acquire_keys.len() as _;
        self.raw.pAcquireKeys = acquire_keys.as_ptr() as *const u64 as *const _;
    }

    pub fn set_acquire_timeout_milliseconds<'m, 'a>(&'s mut self, acquire_timeout_milliseconds: &'a [u32]) {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_timeout_milliseconds.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::acquire_timeout_milliseconds`.");
        self.raw.acquireCount = acquire_timeout_milliseconds.len() as _;
        self.raw.pAcquireTimeoutMilliseconds = acquire_timeout_milliseconds.as_ptr() as *const u32 as *const _;
    }

    pub fn set_release_syncs<'m, 'a>(&'s mut self, release_syncs: &'a [&'a DeviceMemory])
            where 'a: 's {
        self.release_syncs = Some(release_syncs.iter().map(|h| h.handle()).collect());
        {
            let release_syncs = self.release_syncs.as_ref().unwrap();
            self.raw.pReleaseSyncs = release_syncs.as_ptr();
            assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::release_syncs`.");
            self.raw.releaseCount = release_syncs.len() as _;
        }
    }

    pub fn set_release_keys<'m, 'a>(&'s mut self, release_keys: &'a [u64]) {
        assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::release_keys`.");
        self.raw.releaseCount = release_keys.len() as _;
        self.raw.pReleaseKeys = release_keys.as_ptr() as *const u64 as *const _;
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


impl<'s> From<vks::VkWin32KeyedMutexAcquireReleaseInfoNV> for Win32KeyedMutexAcquireReleaseInfoNv<'s> {
    fn from(f: vks::VkWin32KeyedMutexAcquireReleaseInfoNV) -> Win32KeyedMutexAcquireReleaseInfoNv<'s> {
        Win32KeyedMutexAcquireReleaseInfoNv { raw: f, acquire_syncs: None, release_syncs: None, _p: PhantomData }
    }
}


/// A builder for `VkWin32KeyedMutexAcquireReleaseInfoNV`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoNV,
    acquire_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
    release_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
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

    pub fn acquire_syncs<'m, 'a>(mut self, acquire_syncs: &'a [&'a DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b>
            where 'a: 'b {
        self.acquire_syncs = Some(acquire_syncs.iter().map(|h| h.handle()).collect());
        {
            let acquire_syncs = self.acquire_syncs.as_ref().unwrap();
            self.raw.pAcquireSyncs = acquire_syncs.as_ptr();
            assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::acquire_syncs`.");
            self.raw.acquireCount = acquire_syncs.len() as _;
        }
        self
    }

    pub fn acquire_keys<'m, 'a>(mut self, acquire_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::acquire_keys`.");
        self.raw.acquireCount = acquire_keys.len() as _;
        self.raw.pAcquireKeys = acquire_keys.as_ptr() as *const u64 as *const _;
        self
    }

    pub fn acquire_timeout_milliseconds<'m, 'a>(mut self, acquire_timeout_milliseconds: &'a [u32]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_timeout_milliseconds.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::acquire_timeout_milliseconds`.");
        self.raw.acquireCount = acquire_timeout_milliseconds.len() as _;
        self.raw.pAcquireTimeoutMilliseconds = acquire_timeout_milliseconds.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn release_syncs<'m, 'a>(mut self, release_syncs: &'a [&'a DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b>
            where 'a: 'b {
        self.release_syncs = Some(release_syncs.iter().map(|h| h.handle()).collect());
        {
            let release_syncs = self.release_syncs.as_ref().unwrap();
            self.raw.pReleaseSyncs = release_syncs.as_ptr();
            assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::release_syncs`.");
            self.raw.releaseCount = release_syncs.len() as _;
        }
        self
    }

    pub fn release_keys<'m, 'a>(mut self, release_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoNvBuilder<'b> {
        assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoNv::release_keys`.");
        self.raw.releaseCount = release_keys.len() as _;
        self.raw.pReleaseKeys = release_keys.as_ptr() as *const u64 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_acquire_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireSyncs as *const _, self.raw.acquireCount as usize) }
    }

    pub fn get_acquire_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireKeys as *const _, self.raw.acquireCount as usize) }
    }

    pub fn get_acquire_timeout_milliseconds<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireTimeoutMilliseconds as *const _, self.raw.acquireCount as usize) }
    }

    pub fn get_release_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseSyncs as *const _, self.raw.releaseCount as usize) }
    }

    pub fn get_release_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseKeys as *const _, self.raw.releaseCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn compute_binding_point_support<'a>(&'a self) -> bool {
        self.raw.computeBindingPointSupport != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_compute_binding_point_support<'m>(&'s mut self, compute_binding_point_support: bool) {
        self.raw.computeBindingPointSupport = compute_binding_point_support as u32;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGeneratedCommandsFeaturesNVX> for DeviceGeneratedCommandsFeaturesNvx<'s> {
    fn from(f: vks::VkDeviceGeneratedCommandsFeaturesNVX) -> DeviceGeneratedCommandsFeaturesNvx<'s> {
        DeviceGeneratedCommandsFeaturesNvx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_compute_binding_point_support<'a>(&'a self) -> bool {
        self.raw.computeBindingPointSupport != 0
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn max_indirect_commands_layout_token_count<'a>(&'a self) -> u32 {
        self.raw.maxIndirectCommandsLayoutTokenCount.into()
    }

    pub fn max_object_entry_counts<'a>(&'a self) -> u32 {
        self.raw.maxObjectEntryCounts.into()
    }

    pub fn min_sequence_count_buffer_offset_alignment<'a>(&'a self) -> u32 {
        self.raw.minSequenceCountBufferOffsetAlignment.into()
    }

    pub fn min_sequence_index_buffer_offset_alignment<'a>(&'a self) -> u32 {
        self.raw.minSequenceIndexBufferOffsetAlignment.into()
    }

    pub fn min_commands_token_buffer_offset_alignment<'a>(&'a self) -> u32 {
        self.raw.minCommandsTokenBufferOffsetAlignment.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_max_indirect_commands_layout_token_count<'m>(&'s mut self, max_indirect_commands_layout_token_count: u32) {
        self.raw.maxIndirectCommandsLayoutTokenCount = max_indirect_commands_layout_token_count.into();
    }

    pub fn set_max_object_entry_counts<'m>(&'s mut self, max_object_entry_counts: u32) {
        self.raw.maxObjectEntryCounts = max_object_entry_counts.into();
    }

    pub fn set_min_sequence_count_buffer_offset_alignment<'m>(&'s mut self, min_sequence_count_buffer_offset_alignment: u32) {
        self.raw.minSequenceCountBufferOffsetAlignment = min_sequence_count_buffer_offset_alignment.into();
    }

    pub fn set_min_sequence_index_buffer_offset_alignment<'m>(&'s mut self, min_sequence_index_buffer_offset_alignment: u32) {
        self.raw.minSequenceIndexBufferOffsetAlignment = min_sequence_index_buffer_offset_alignment.into();
    }

    pub fn set_min_commands_token_buffer_offset_alignment<'m>(&'s mut self, min_commands_token_buffer_offset_alignment: u32) {
        self.raw.minCommandsTokenBufferOffsetAlignment = min_commands_token_buffer_offset_alignment.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGeneratedCommandsLimitsNVX> for DeviceGeneratedCommandsLimitsNvx<'s> {
    fn from(f: vks::VkDeviceGeneratedCommandsLimitsNVX) -> DeviceGeneratedCommandsLimitsNvx<'s> {
        DeviceGeneratedCommandsLimitsNvx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_max_indirect_commands_layout_token_count<'a>(&'a self) -> u32 {
        self.raw.maxIndirectCommandsLayoutTokenCount.into()
    }

    pub fn get_max_object_entry_counts<'a>(&'a self) -> u32 {
        self.raw.maxObjectEntryCounts.into()
    }

    pub fn get_min_sequence_count_buffer_offset_alignment<'a>(&'a self) -> u32 {
        self.raw.minSequenceCountBufferOffsetAlignment.into()
    }

    pub fn get_min_sequence_index_buffer_offset_alignment<'a>(&'a self) -> u32 {
        self.raw.minSequenceIndexBufferOffsetAlignment.into()
    }

    pub fn get_min_commands_token_buffer_offset_alignment<'a>(&'a self) -> u32 {
        self.raw.minCommandsTokenBufferOffsetAlignment.into()
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

    pub fn token_type<'a>(&'a self) -> IndirectCommandsTokenTypeNvx {
        self.raw.tokenType.into()
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
    }

    pub fn set_token_type<'m>(& mut self, token_type: IndirectCommandsTokenTypeNvx) {
        self.raw.tokenType = token_type.into();
    }

    pub fn set_buffer<'m, 'a>(& mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
    }

    pub fn set_offset<'m>(& mut self, offset: u64) {
        self.raw.offset = offset.into();
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


#[cfg(feature = "experimental")]
impl From<vks::VkIndirectCommandsTokenNVX> for IndirectCommandsTokenNvx {
    fn from(f: vks::VkIndirectCommandsTokenNVX) -> IndirectCommandsTokenNvx {
        IndirectCommandsTokenNvx { raw: f, }
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

    pub fn get_token_type<'a>(&'a self) -> IndirectCommandsTokenTypeNvx {
        self.raw.tokenType.into()
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn get_offset<'a>(&'a self) -> u64 {
        self.raw.offset.into()
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

    pub fn token_type<'a>(&'a self) -> IndirectCommandsTokenTypeNvx {
        self.raw.tokenType.into()
    }

    pub fn binding_unit<'a>(&'a self) -> u32 {
        self.raw.bindingUnit.into()
    }

    pub fn dynamic_count<'a>(&'a self) -> u32 {
        self.raw.dynamicCount.into()
    }

    pub fn divisor<'a>(&'a self) -> u32 {
        self.raw.divisor.into()
    }

    pub fn set_token_type<'m>(& mut self, token_type: IndirectCommandsTokenTypeNvx) {
        self.raw.tokenType = token_type.into();
    }

    pub fn set_binding_unit<'m>(& mut self, binding_unit: u32) {
        self.raw.bindingUnit = binding_unit.into();
    }

    pub fn set_dynamic_count<'m>(& mut self, dynamic_count: u32) {
        self.raw.dynamicCount = dynamic_count.into();
    }

    pub fn set_divisor<'m>(& mut self, divisor: u32) {
        self.raw.divisor = divisor.into();
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


#[cfg(feature = "experimental")]
impl From<vks::VkIndirectCommandsLayoutTokenNVX> for IndirectCommandsLayoutTokenNvx {
    fn from(f: vks::VkIndirectCommandsLayoutTokenNVX) -> IndirectCommandsLayoutTokenNvx {
        IndirectCommandsLayoutTokenNvx { raw: f, }
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

    pub fn get_token_type<'a>(&'a self) -> IndirectCommandsTokenTypeNvx {
        self.raw.tokenType.into()
    }

    pub fn get_binding_unit<'a>(&'a self) -> u32 {
        self.raw.bindingUnit.into()
    }

    pub fn get_dynamic_count<'a>(&'a self) -> u32 {
        self.raw.dynamicCount.into()
    }

    pub fn get_divisor<'a>(&'a self) -> u32 {
        self.raw.divisor.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn pipeline_bind_point<'a>(&'a self) -> PipelineBindPoint {
        self.raw.pipelineBindPoint.into()
    }

    pub fn flags<'a>(&'a self) -> IndirectCommandsLayoutUsageFlagsNvx {
        IndirectCommandsLayoutUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("IndirectCommandsLayoutCreateInfoNvx::flags: error converting flags")
    }

    pub fn tokens<'a>(&'a self) -> &'a [IndirectCommandsLayoutTokenNvx] {
        unsafe { slice::from_raw_parts(self.raw.pTokens as *const _, self.raw.tokenCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_pipeline_bind_point<'m>(&'s mut self, pipeline_bind_point: PipelineBindPoint) {
        self.raw.pipelineBindPoint = pipeline_bind_point.into();
    }

    pub fn set_flags<'m>(&'s mut self, flags: IndirectCommandsLayoutUsageFlagsNvx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_tokens<'m, 'a>(&'s mut self, tokens: &'a [IndirectCommandsLayoutTokenNvx]) {
        assert!(self.raw.tokenCount == 0 || self.raw.tokenCount == tokens.len() as _, 
            "count inconsistency found when specifying `IndirectCommandsLayoutCreateInfoNvx::tokens`.");
        self.raw.tokenCount = tokens.len() as _;
        self.raw.pTokens = tokens.as_ptr() as *const vks::VkIndirectCommandsLayoutTokenNVX as *const _;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkIndirectCommandsLayoutCreateInfoNVX> for IndirectCommandsLayoutCreateInfoNvx<'s> {
    fn from(f: vks::VkIndirectCommandsLayoutCreateInfoNVX) -> IndirectCommandsLayoutCreateInfoNvx<'s> {
        IndirectCommandsLayoutCreateInfoNvx { raw: f, _p: PhantomData }
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

    pub fn tokens<'m, 'a>(mut self, tokens: &'a [IndirectCommandsLayoutTokenNvx]) -> IndirectCommandsLayoutCreateInfoNvxBuilder<'b> {
        assert!(self.raw.tokenCount == 0 || self.raw.tokenCount == tokens.len() as _, 
            "count inconsistency found when specifying `IndirectCommandsLayoutCreateInfoNvx::tokens`.");
        self.raw.tokenCount = tokens.len() as _;
        self.raw.pTokens = tokens.as_ptr() as *const vks::VkIndirectCommandsLayoutTokenNVX as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_pipeline_bind_point<'a>(&'a self) -> PipelineBindPoint {
        self.raw.pipelineBindPoint.into()
    }

    pub fn get_flags<'a>(&'a self) -> IndirectCommandsLayoutUsageFlagsNvx {
        IndirectCommandsLayoutUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("IndirectCommandsLayoutCreateInfoNvx::flags: error converting flags")
    }

    pub fn get_tokens<'a>(&'a self) -> &'a [IndirectCommandsLayoutTokenNvx] {
        unsafe { slice::from_raw_parts(self.raw.pTokens as *const _, self.raw.tokenCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn object_table_handle<'a>(&'a self) -> vks::VkObjectTableNVX {
        self.raw.objectTable
    }

    pub fn indirect_commands_layout_handle<'a>(&'a self) -> vks::VkIndirectCommandsLayoutNVX {
        self.raw.indirectCommandsLayout
    }

    pub fn indirect_commands_tokens<'a>(&'a self) -> &'a [IndirectCommandsTokenNvx] {
        unsafe { slice::from_raw_parts(self.raw.pIndirectCommandsTokens as *const _, self.raw.indirectCommandsTokenCount as usize) }
    }

    pub fn max_sequences_count<'a>(&'a self) -> u32 {
        self.raw.maxSequencesCount.into()
    }

    pub fn target_command_buffer_handle<'a>(&'a self) -> vks::VkCommandBuffer {
        self.raw.targetCommandBuffer
    }

    pub fn sequences_count_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.sequencesCountBuffer
    }

    pub fn sequences_count_offset<'a>(&'a self) -> u64 {
        self.raw.sequencesCountOffset.into()
    }

    pub fn sequences_index_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.sequencesIndexBuffer
    }

    pub fn sequences_index_offset<'a>(&'a self) -> u64 {
        self.raw.sequencesIndexOffset.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_object_table<'m, 'a>(&'s mut self, object_table: &'a ObjectTableNvx) {
        self.raw.objectTable = object_table.handle();
    }

    pub fn set_indirect_commands_layout<'m, 'a>(&'s mut self, indirect_commands_layout: &'a IndirectCommandsLayoutNvx) {
        self.raw.indirectCommandsLayout = indirect_commands_layout.handle();
    }

    pub fn set_indirect_commands_tokens<'m, 'a>(&'s mut self, indirect_commands_tokens: &'a [IndirectCommandsTokenNvx]) {
        assert!(self.raw.indirectCommandsTokenCount == 0 || self.raw.indirectCommandsTokenCount == indirect_commands_tokens.len() as _, 
            "count inconsistency found when specifying `CmdProcessCommandsInfoNvx::indirect_commands_tokens`.");
        self.raw.indirectCommandsTokenCount = indirect_commands_tokens.len() as _;
        self.raw.pIndirectCommandsTokens = indirect_commands_tokens.as_ptr() as *const vks::VkIndirectCommandsTokenNVX as *const _;
    }

    pub fn set_max_sequences_count<'m>(&'s mut self, max_sequences_count: u32) {
        self.raw.maxSequencesCount = max_sequences_count.into();
    }

    pub fn set_target_command_buffer<'m, 'a>(&'s mut self, target_command_buffer: &'a CommandBuffer) {
        self.raw.targetCommandBuffer = target_command_buffer.handle();
    }

    pub fn set_sequences_count_buffer<'m, 'a>(&'s mut self, sequences_count_buffer: &'a Buffer) {
        self.raw.sequencesCountBuffer = sequences_count_buffer.handle();
    }

    pub fn set_sequences_count_offset<'m>(&'s mut self, sequences_count_offset: u64) {
        self.raw.sequencesCountOffset = sequences_count_offset.into();
    }

    pub fn set_sequences_index_buffer<'m, 'a>(&'s mut self, sequences_index_buffer: &'a Buffer) {
        self.raw.sequencesIndexBuffer = sequences_index_buffer.handle();
    }

    pub fn set_sequences_index_offset<'m>(&'s mut self, sequences_index_offset: u64) {
        self.raw.sequencesIndexOffset = sequences_index_offset.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkCmdProcessCommandsInfoNVX> for CmdProcessCommandsInfoNvx<'s> {
    fn from(f: vks::VkCmdProcessCommandsInfoNVX) -> CmdProcessCommandsInfoNvx<'s> {
        CmdProcessCommandsInfoNvx { raw: f, _p: PhantomData }
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

    pub fn indirect_commands_tokens<'m, 'a>(mut self, indirect_commands_tokens: &'a [IndirectCommandsTokenNvx]) -> CmdProcessCommandsInfoNvxBuilder<'b> {
        assert!(self.raw.indirectCommandsTokenCount == 0 || self.raw.indirectCommandsTokenCount == indirect_commands_tokens.len() as _, 
            "count inconsistency found when specifying `CmdProcessCommandsInfoNvx::indirect_commands_tokens`.");
        self.raw.indirectCommandsTokenCount = indirect_commands_tokens.len() as _;
        self.raw.pIndirectCommandsTokens = indirect_commands_tokens.as_ptr() as *const vks::VkIndirectCommandsTokenNVX as *const _;
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_object_table_handle<'a>(&'a self) -> vks::VkObjectTableNVX {
        self.raw.objectTable
    }

    pub fn get_indirect_commands_layout_handle<'a>(&'a self) -> vks::VkIndirectCommandsLayoutNVX {
        self.raw.indirectCommandsLayout
    }

    pub fn get_indirect_commands_tokens<'a>(&'a self) -> &'a [IndirectCommandsTokenNvx] {
        unsafe { slice::from_raw_parts(self.raw.pIndirectCommandsTokens as *const _, self.raw.indirectCommandsTokenCount as usize) }
    }

    pub fn get_max_sequences_count<'a>(&'a self) -> u32 {
        self.raw.maxSequencesCount.into()
    }

    pub fn get_target_command_buffer_handle<'a>(&'a self) -> vks::VkCommandBuffer {
        self.raw.targetCommandBuffer
    }

    pub fn get_sequences_count_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.sequencesCountBuffer
    }

    pub fn get_sequences_count_offset<'a>(&'a self) -> u64 {
        self.raw.sequencesCountOffset.into()
    }

    pub fn get_sequences_index_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.sequencesIndexBuffer
    }

    pub fn get_sequences_index_offset<'a>(&'a self) -> u64 {
        self.raw.sequencesIndexOffset.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn object_table_handle<'a>(&'a self) -> vks::VkObjectTableNVX {
        self.raw.objectTable
    }

    pub fn indirect_commands_layout_handle<'a>(&'a self) -> vks::VkIndirectCommandsLayoutNVX {
        self.raw.indirectCommandsLayout
    }

    pub fn max_sequences_count<'a>(&'a self) -> u32 {
        self.raw.maxSequencesCount.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_object_table<'m, 'a>(&'s mut self, object_table: &'a ObjectTableNvx) {
        self.raw.objectTable = object_table.handle();
    }

    pub fn set_indirect_commands_layout<'m, 'a>(&'s mut self, indirect_commands_layout: &'a IndirectCommandsLayoutNvx) {
        self.raw.indirectCommandsLayout = indirect_commands_layout.handle();
    }

    pub fn set_max_sequences_count<'m>(&'s mut self, max_sequences_count: u32) {
        self.raw.maxSequencesCount = max_sequences_count.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkCmdReserveSpaceForCommandsInfoNVX> for CmdReserveSpaceForCommandsInfoNvx<'s> {
    fn from(f: vks::VkCmdReserveSpaceForCommandsInfoNVX) -> CmdReserveSpaceForCommandsInfoNvx<'s> {
        CmdReserveSpaceForCommandsInfoNvx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_object_table_handle<'a>(&'a self) -> vks::VkObjectTableNVX {
        self.raw.objectTable
    }

    pub fn get_indirect_commands_layout_handle<'a>(&'a self) -> vks::VkIndirectCommandsLayoutNVX {
        self.raw.indirectCommandsLayout
    }

    pub fn get_max_sequences_count<'a>(&'a self) -> u32 {
        self.raw.maxSequencesCount.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn object_entry_types<'a>(&'a self) -> &'a [ObjectEntryTypeNvx] {
        unsafe { slice::from_raw_parts(self.raw.pObjectEntryTypes as *const _, self.raw.objectCount as usize) }
    }

    pub fn object_entry_counts<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pObjectEntryCounts as *const _, self.raw.objectCount as usize) }
    }

    pub fn object_entry_usage_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        unsafe { slice::from_raw_parts(self.raw.pObjectEntryUsageFlags as *const _, self.raw.objectCount as usize) }
    }

    pub fn max_uniform_buffers_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxUniformBuffersPerDescriptor.into()
    }

    pub fn max_storage_buffers_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxStorageBuffersPerDescriptor.into()
    }

    pub fn max_storage_images_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxStorageImagesPerDescriptor.into()
    }

    pub fn max_sampled_images_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxSampledImagesPerDescriptor.into()
    }

    pub fn max_pipeline_layouts<'a>(&'a self) -> u32 {
        self.raw.maxPipelineLayouts.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_object_entry_types<'m, 'a>(&'s mut self, object_entry_types: &'a [ObjectEntryTypeNvx]) {
        assert!(self.raw.objectCount == 0 || self.raw.objectCount == object_entry_types.len() as _, 
            "count inconsistency found when specifying `ObjectTableCreateInfoNvx::object_entry_types`.");
        self.raw.objectCount = object_entry_types.len() as _;
        self.raw.pObjectEntryTypes = object_entry_types.as_ptr() as *const ObjectEntryTypeNvx as *const _;
    }

    pub fn set_object_entry_counts<'m, 'a>(&'s mut self, object_entry_counts: &'a [u32]) {
        assert!(self.raw.objectCount == 0 || self.raw.objectCount == object_entry_counts.len() as _, 
            "count inconsistency found when specifying `ObjectTableCreateInfoNvx::object_entry_counts`.");
        self.raw.objectCount = object_entry_counts.len() as _;
        self.raw.pObjectEntryCounts = object_entry_counts.as_ptr() as *const u32 as *const _;
    }

    pub fn set_object_entry_usage_flags<'m>(&'s mut self, object_entry_usage_flags: ObjectEntryUsageFlagsNvx) {
        assert!(self.raw.objectCount == 0 || self.raw.objectCount == object_entry_usage_flags.len() as _, 
            "count inconsistency found when specifying `ObjectTableCreateInfoNvx::object_entry_usage_flags`.");
        self.raw.objectCount = object_entry_usage_flags.len() as _;
        self.raw.pObjectEntryUsageFlags = object_entry_usage_flags.bits();
    }

    pub fn set_max_uniform_buffers_per_descriptor<'m>(&'s mut self, max_uniform_buffers_per_descriptor: u32) {
        self.raw.maxUniformBuffersPerDescriptor = max_uniform_buffers_per_descriptor.into();
    }

    pub fn set_max_storage_buffers_per_descriptor<'m>(&'s mut self, max_storage_buffers_per_descriptor: u32) {
        self.raw.maxStorageBuffersPerDescriptor = max_storage_buffers_per_descriptor.into();
    }

    pub fn set_max_storage_images_per_descriptor<'m>(&'s mut self, max_storage_images_per_descriptor: u32) {
        self.raw.maxStorageImagesPerDescriptor = max_storage_images_per_descriptor.into();
    }

    pub fn set_max_sampled_images_per_descriptor<'m>(&'s mut self, max_sampled_images_per_descriptor: u32) {
        self.raw.maxSampledImagesPerDescriptor = max_sampled_images_per_descriptor.into();
    }

    pub fn set_max_pipeline_layouts<'m>(&'s mut self, max_pipeline_layouts: u32) {
        self.raw.maxPipelineLayouts = max_pipeline_layouts.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkObjectTableCreateInfoNVX> for ObjectTableCreateInfoNvx<'s> {
    fn from(f: vks::VkObjectTableCreateInfoNVX) -> ObjectTableCreateInfoNvx<'s> {
        ObjectTableCreateInfoNvx { raw: f, _p: PhantomData }
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

    pub fn object_entry_types<'m, 'a>(mut self, object_entry_types: &'a [ObjectEntryTypeNvx]) -> ObjectTableCreateInfoNvxBuilder<'b> {
        assert!(self.raw.objectCount == 0 || self.raw.objectCount == object_entry_types.len() as _, 
            "count inconsistency found when specifying `ObjectTableCreateInfoNvx::object_entry_types`.");
        self.raw.objectCount = object_entry_types.len() as _;
        self.raw.pObjectEntryTypes = object_entry_types.as_ptr() as *const ObjectEntryTypeNvx as *const _;
        self
    }

    pub fn object_entry_counts<'m, 'a>(mut self, object_entry_counts: &'a [u32]) -> ObjectTableCreateInfoNvxBuilder<'b> {
        assert!(self.raw.objectCount == 0 || self.raw.objectCount == object_entry_counts.len() as _, 
            "count inconsistency found when specifying `ObjectTableCreateInfoNvx::object_entry_counts`.");
        self.raw.objectCount = object_entry_counts.len() as _;
        self.raw.pObjectEntryCounts = object_entry_counts.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn object_entry_usage_flags<'m>(mut self, object_entry_usage_flags: ObjectEntryUsageFlagsNvx) -> ObjectTableCreateInfoNvxBuilder<'b> {
        assert!(self.raw.objectCount == 0 || self.raw.objectCount == object_entry_usage_flags.len() as _, 
            "count inconsistency found when specifying `ObjectTableCreateInfoNvx::object_entry_usage_flags`.");
        self.raw.objectCount = object_entry_usage_flags.len() as _;
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_object_entry_types<'a>(&'a self) -> &'a [ObjectEntryTypeNvx] {
        unsafe { slice::from_raw_parts(self.raw.pObjectEntryTypes as *const _, self.raw.objectCount as usize) }
    }

    pub fn get_object_entry_counts<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pObjectEntryCounts as *const _, self.raw.objectCount as usize) }
    }

    pub fn get_object_entry_usage_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        unsafe { slice::from_raw_parts(self.raw.pObjectEntryUsageFlags as *const _, self.raw.objectCount as usize) }
    }

    pub fn get_max_uniform_buffers_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxUniformBuffersPerDescriptor.into()
    }

    pub fn get_max_storage_buffers_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxStorageBuffersPerDescriptor.into()
    }

    pub fn get_max_storage_images_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxStorageImagesPerDescriptor.into()
    }

    pub fn get_max_sampled_images_per_descriptor<'a>(&'a self) -> u32 {
        self.raw.maxSampledImagesPerDescriptor.into()
    }

    pub fn get_max_pipeline_layouts<'a>(&'a self) -> u32 {
        self.raw.maxPipelineLayouts.into()
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

    pub fn type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableEntryNvx::flags: error converting flags")
    }

    pub fn set_type_of<'m>(& mut self, type_of: ObjectEntryTypeNvx) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_flags<'m>(& mut self, flags: ObjectEntryUsageFlagsNvx) {
        self.raw.flags = flags.bits();
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


#[cfg(feature = "experimental")]
impl From<vks::VkObjectTableEntryNVX> for ObjectTableEntryNvx {
    fn from(f: vks::VkObjectTableEntryNVX) -> ObjectTableEntryNvx {
        ObjectTableEntryNvx { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn get_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableEntryNvx::flags: error converting flags")
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

    pub fn type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTablePipelineEntryNvx::flags: error converting flags")
    }

    pub fn pipeline_handle<'a>(&'a self) -> vks::VkPipeline {
        self.raw.pipeline
    }

    pub fn set_type_of<'m>(& mut self, type_of: ObjectEntryTypeNvx) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_flags<'m>(& mut self, flags: ObjectEntryUsageFlagsNvx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_pipeline<'m, 'a>(& mut self, pipeline: &'a Pipeline) {
        self.raw.pipeline = pipeline.handle();
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


#[cfg(feature = "experimental")]
impl From<vks::VkObjectTablePipelineEntryNVX> for ObjectTablePipelineEntryNvx {
    fn from(f: vks::VkObjectTablePipelineEntryNVX) -> ObjectTablePipelineEntryNvx {
        ObjectTablePipelineEntryNvx { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn get_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTablePipelineEntryNvx::flags: error converting flags")
    }

    pub fn get_pipeline_handle<'a>(&'a self) -> vks::VkPipeline {
        self.raw.pipeline
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

    pub fn type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableDescriptorSetEntryNvx::flags: error converting flags")
    }

    pub fn pipeline_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.pipelineLayout
    }

    pub fn descriptor_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.descriptorSet
    }

    pub fn set_type_of<'m>(& mut self, type_of: ObjectEntryTypeNvx) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_flags<'m>(& mut self, flags: ObjectEntryUsageFlagsNvx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_pipeline_layout<'m, 'a>(& mut self, pipeline_layout: &'a PipelineLayout) {
        self.raw.pipelineLayout = pipeline_layout.handle();
    }

    pub fn set_descriptor_set<'m, 'a>(& mut self, descriptor_set: &'a DescriptorSet) {
        self.raw.descriptorSet = descriptor_set.handle();
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


#[cfg(feature = "experimental")]
impl From<vks::VkObjectTableDescriptorSetEntryNVX> for ObjectTableDescriptorSetEntryNvx {
    fn from(f: vks::VkObjectTableDescriptorSetEntryNVX) -> ObjectTableDescriptorSetEntryNvx {
        ObjectTableDescriptorSetEntryNvx { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn get_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableDescriptorSetEntryNvx::flags: error converting flags")
    }

    pub fn get_pipeline_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.pipelineLayout
    }

    pub fn get_descriptor_set_handle<'a>(&'a self) -> vks::VkDescriptorSet {
        self.raw.descriptorSet
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

    pub fn type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableVertexBufferEntryNvx::flags: error converting flags")
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn set_type_of<'m>(& mut self, type_of: ObjectEntryTypeNvx) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_flags<'m>(& mut self, flags: ObjectEntryUsageFlagsNvx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_buffer<'m, 'a>(& mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
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


#[cfg(feature = "experimental")]
impl From<vks::VkObjectTableVertexBufferEntryNVX> for ObjectTableVertexBufferEntryNvx {
    fn from(f: vks::VkObjectTableVertexBufferEntryNVX) -> ObjectTableVertexBufferEntryNvx {
        ObjectTableVertexBufferEntryNvx { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn get_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableVertexBufferEntryNvx::flags: error converting flags")
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
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

    pub fn type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableIndexBufferEntryNvx::flags: error converting flags")
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn index_type<'a>(&'a self) -> IndexType {
        self.raw.indexType.into()
    }

    pub fn set_type_of<'m>(& mut self, type_of: ObjectEntryTypeNvx) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_flags<'m>(& mut self, flags: ObjectEntryUsageFlagsNvx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_buffer<'m, 'a>(& mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
    }

    pub fn set_index_type<'m>(& mut self, index_type: IndexType) {
        self.raw.indexType = index_type.into();
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


#[cfg(feature = "experimental")]
impl From<vks::VkObjectTableIndexBufferEntryNVX> for ObjectTableIndexBufferEntryNvx {
    fn from(f: vks::VkObjectTableIndexBufferEntryNVX) -> ObjectTableIndexBufferEntryNvx {
        ObjectTableIndexBufferEntryNvx { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn get_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTableIndexBufferEntryNvx::flags: error converting flags")
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub fn get_index_type<'a>(&'a self) -> IndexType {
        self.raw.indexType.into()
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

    pub fn type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTablePushConstantEntryNvx::flags: error converting flags")
    }

    pub fn pipeline_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.pipelineLayout
    }

    pub fn stage_flags<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stageFlags)
            .expect("ObjectTablePushConstantEntryNvx::stage_flags: error converting flags")
    }

    pub fn set_type_of<'m>(& mut self, type_of: ObjectEntryTypeNvx) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_flags<'m>(& mut self, flags: ObjectEntryUsageFlagsNvx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_pipeline_layout<'m, 'a>(& mut self, pipeline_layout: &'a PipelineLayout) {
        self.raw.pipelineLayout = pipeline_layout.handle();
    }

    pub fn set_stage_flags<'m>(& mut self, stage_flags: ShaderStageFlags) {
        self.raw.stageFlags = stage_flags.bits();
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


#[cfg(feature = "experimental")]
impl From<vks::VkObjectTablePushConstantEntryNVX> for ObjectTablePushConstantEntryNvx {
    fn from(f: vks::VkObjectTablePushConstantEntryNVX) -> ObjectTablePushConstantEntryNvx {
        ObjectTablePushConstantEntryNvx { raw: f, }
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

    pub fn get_type_of<'a>(&'a self) -> ObjectEntryTypeNvx {
        self.raw.type_.into()
    }

    pub fn get_flags<'a>(&'a self) -> ObjectEntryUsageFlagsNvx {
        ObjectEntryUsageFlagsNvx::from_bits(self.raw.flags)
            .expect("ObjectTablePushConstantEntryNvx::flags: error converting flags")
    }

    pub fn get_pipeline_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.pipelineLayout
    }

    pub fn get_stage_flags<'a>(&'a self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits(self.raw.stageFlags)
            .expect("ObjectTablePushConstantEntryNvx::stage_flags: error converting flags")
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn features<'a>(&'a self) -> PhysicalDeviceFeatures {
        self.raw.features.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_features<'m>(&'s mut self, features: PhysicalDeviceFeatures) {
        self.raw.features = features.raw;
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


impl<'s> From<vks::VkPhysicalDeviceFeatures2KHR> for PhysicalDeviceFeatures2Khr<'s> {
    fn from(f: vks::VkPhysicalDeviceFeatures2KHR) -> PhysicalDeviceFeatures2Khr<'s> {
        PhysicalDeviceFeatures2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_features<'a>(&'a self) -> PhysicalDeviceFeatures {
        self.raw.features.into()
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn properties<'a>(&'a self) -> PhysicalDeviceProperties {
        self.raw.properties.into()
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


impl<'s> From<vks::VkPhysicalDeviceProperties2KHR> for PhysicalDeviceProperties2Khr<'s> {
    fn from(f: vks::VkPhysicalDeviceProperties2KHR) -> PhysicalDeviceProperties2Khr<'s> {
        PhysicalDeviceProperties2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn format_properties<'a>(&'a self) -> FormatProperties {
        self.raw.formatProperties.into()
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


impl<'s> From<vks::VkFormatProperties2KHR> for FormatProperties2Khr<'s> {
    fn from(f: vks::VkFormatProperties2KHR) -> FormatProperties2Khr<'s> {
        FormatProperties2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn image_format_properties<'a>(&'a self) -> ImageFormatProperties {
        self.raw.imageFormatProperties.into()
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


impl<'s> From<vks::VkImageFormatProperties2KHR> for ImageFormatProperties2Khr<'s> {
    fn from(f: vks::VkImageFormatProperties2KHR) -> ImageFormatProperties2Khr<'s> {
        ImageFormatProperties2Khr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn type_of<'a>(&'a self) -> ImageType {
        self.raw.type_.into()
    }

    pub fn tiling<'a>(&'a self) -> ImageTiling {
        self.raw.tiling.into()
    }

    pub fn usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.usage)
            .expect("PhysicalDeviceImageFormatInfo2Khr::usage: error converting flags")
    }

    pub fn flags<'a>(&'a self) -> ImageCreateFlags {
        ImageCreateFlags::from_bits(self.raw.flags)
            .expect("PhysicalDeviceImageFormatInfo2Khr::flags: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_format<'m>(&'s mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_type_of<'m>(&'s mut self, type_of: ImageType) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_tiling<'m>(&'s mut self, tiling: ImageTiling) {
        self.raw.tiling = tiling.into();
    }

    pub fn set_usage<'m>(&'s mut self, usage: ImageUsageFlags) {
        self.raw.usage = usage.bits();
    }

    pub fn set_flags<'m>(&'s mut self, flags: ImageCreateFlags) {
        self.raw.flags = flags.bits();
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


impl<'s> From<vks::VkPhysicalDeviceImageFormatInfo2KHR> for PhysicalDeviceImageFormatInfo2Khr<'s> {
    fn from(f: vks::VkPhysicalDeviceImageFormatInfo2KHR) -> PhysicalDeviceImageFormatInfo2Khr<'s> {
        PhysicalDeviceImageFormatInfo2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_type_of<'a>(&'a self) -> ImageType {
        self.raw.type_.into()
    }

    pub fn get_tiling<'a>(&'a self) -> ImageTiling {
        self.raw.tiling.into()
    }

    pub fn get_usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.usage)
            .expect("PhysicalDeviceImageFormatInfo2Khr::usage: error converting flags")
    }

    pub fn get_flags<'a>(&'a self) -> ImageCreateFlags {
        ImageCreateFlags::from_bits(self.raw.flags)
            .expect("PhysicalDeviceImageFormatInfo2Khr::flags: error converting flags")
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn queue_family_properties<'a>(&'a self) -> QueueFamilyProperties {
        self.raw.queueFamilyProperties.into()
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


impl<'s> From<vks::VkQueueFamilyProperties2KHR> for QueueFamilyProperties2Khr<'s> {
    fn from(f: vks::VkQueueFamilyProperties2KHR) -> QueueFamilyProperties2Khr<'s> {
        QueueFamilyProperties2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn memory_properties<'a>(&'a self) -> PhysicalDeviceMemoryProperties {
        self.raw.memoryProperties.into()
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


impl<'s> From<vks::VkPhysicalDeviceMemoryProperties2KHR> for PhysicalDeviceMemoryProperties2Khr<'s> {
    fn from(f: vks::VkPhysicalDeviceMemoryProperties2KHR) -> PhysicalDeviceMemoryProperties2Khr<'s> {
        PhysicalDeviceMemoryProperties2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn properties<'a>(&'a self) -> SparseImageFormatProperties {
        self.raw.properties.into()
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


impl<'s> From<vks::VkSparseImageFormatProperties2KHR> for SparseImageFormatProperties2Khr<'s> {
    fn from(f: vks::VkSparseImageFormatProperties2KHR) -> SparseImageFormatProperties2Khr<'s> {
        SparseImageFormatProperties2Khr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn type_of<'a>(&'a self) -> ImageType {
        self.raw.type_.into()
    }

    pub fn samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.samples)
            .expect("PhysicalDeviceSparseImageFormatInfo2Khr::samples: error converting flags")
    }

    pub fn usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.usage)
            .expect("PhysicalDeviceSparseImageFormatInfo2Khr::usage: error converting flags")
    }

    pub fn tiling<'a>(&'a self) -> ImageTiling {
        self.raw.tiling.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_format<'m>(&'s mut self, format: Format) {
        self.raw.format = format.into();
    }

    pub fn set_type_of<'m>(&'s mut self, type_of: ImageType) {
        self.raw.type_ = type_of.into();
    }

    pub fn set_samples<'m>(&'s mut self, samples: SampleCountFlags) {
        self.raw.samples = samples.bits();
    }

    pub fn set_usage<'m>(&'s mut self, usage: ImageUsageFlags) {
        self.raw.usage = usage.bits();
    }

    pub fn set_tiling<'m>(&'s mut self, tiling: ImageTiling) {
        self.raw.tiling = tiling.into();
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


impl<'s> From<vks::VkPhysicalDeviceSparseImageFormatInfo2KHR> for PhysicalDeviceSparseImageFormatInfo2Khr<'s> {
    fn from(f: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR) -> PhysicalDeviceSparseImageFormatInfo2Khr<'s> {
        PhysicalDeviceSparseImageFormatInfo2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_format<'a>(&'a self) -> Format {
        self.raw.format.into()
    }

    pub fn get_type_of<'a>(&'a self) -> ImageType {
        self.raw.type_.into()
    }

    pub fn get_samples<'a>(&'a self) -> SampleCountFlags {
        SampleCountFlags::from_bits(self.raw.samples)
            .expect("PhysicalDeviceSparseImageFormatInfo2Khr::samples: error converting flags")
    }

    pub fn get_usage<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.usage)
            .expect("PhysicalDeviceSparseImageFormatInfo2Khr::usage: error converting flags")
    }

    pub fn get_tiling<'a>(&'a self) -> ImageTiling {
        self.raw.tiling.into()
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn max_push_descriptors<'a>(&'a self) -> u32 {
        self.raw.maxPushDescriptors.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_max_push_descriptors<'m>(&'s mut self, max_push_descriptors: u32) {
        self.raw.maxPushDescriptors = max_push_descriptors.into();
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


impl<'s> From<vks::VkPhysicalDevicePushDescriptorPropertiesKHR> for PhysicalDevicePushDescriptorPropertiesKhr<'s> {
    fn from(f: vks::VkPhysicalDevicePushDescriptorPropertiesKHR) -> PhysicalDevicePushDescriptorPropertiesKhr<'s> {
        PhysicalDevicePushDescriptorPropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_max_push_descriptors<'a>(&'a self) -> u32 {
        self.raw.maxPushDescriptors.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn regions<'a>(&'a self) -> &'a [PresentRegionKhr] {
        unsafe { slice::from_raw_parts(self.raw.pRegions as *const _, self.raw.swapchainCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_regions<'m, 'a>(&'s mut self, regions: &'a [PresentRegionKhr]) {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == regions.len() as _, 
            "count inconsistency found when specifying `PresentRegionsKhr::regions`.");
        self.raw.swapchainCount = regions.len() as _;
        self.raw.pRegions = regions.as_ptr() as *const vks::VkPresentRegionKHR as *const _;
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


impl<'s> From<vks::VkPresentRegionsKHR> for PresentRegionsKhr<'s> {
    fn from(f: vks::VkPresentRegionsKHR) -> PresentRegionsKhr<'s> {
        PresentRegionsKhr { raw: f, _p: PhantomData }
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

    pub fn regions<'m, 'a>(mut self, regions: &'a [PresentRegionKhr]) -> PresentRegionsKhrBuilder<'b> {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == regions.len() as _, 
            "count inconsistency found when specifying `PresentRegionsKhr::regions`.");
        self.raw.swapchainCount = regions.len() as _;
        self.raw.pRegions = regions.as_ptr() as *const vks::VkPresentRegionKHR as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_regions<'a>(&'a self) -> &'a [PresentRegionKhr] {
        unsafe { slice::from_raw_parts(self.raw.pRegions as *const _, self.raw.swapchainCount as usize) }
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

    pub fn rectangles<'a>(&'a self) -> &'a [RectLayerKhr] {
        unsafe { slice::from_raw_parts(self.raw.pRectangles as *const _, self.raw.rectangleCount as usize) }
    }

    pub fn set_rectangles<'m, 'a>(&'s mut self, rectangles: &'a [RectLayerKhr]) {
        assert!(self.raw.rectangleCount == 0 || self.raw.rectangleCount == rectangles.len() as _, 
            "count inconsistency found when specifying `PresentRegionKhr::rectangles`.");
        self.raw.rectangleCount = rectangles.len() as _;
        self.raw.pRectangles = rectangles.as_ptr() as *const vks::VkRectLayerKHR as *const _;
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


impl<'s> From<vks::VkPresentRegionKHR> for PresentRegionKhr<'s> {
    fn from(f: vks::VkPresentRegionKHR) -> PresentRegionKhr<'s> {
        PresentRegionKhr { raw: f, _p: PhantomData }
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

    pub fn rectangles<'m, 'a>(mut self, rectangles: &'a [RectLayerKhr]) -> PresentRegionKhrBuilder<'b> {
        assert!(self.raw.rectangleCount == 0 || self.raw.rectangleCount == rectangles.len() as _, 
            "count inconsistency found when specifying `PresentRegionKhr::rectangles`.");
        self.raw.rectangleCount = rectangles.len() as _;
        self.raw.pRectangles = rectangles.as_ptr() as *const vks::VkRectLayerKHR as *const _;
        self
    }

    pub fn get_rectangles<'a>(&'a self) -> &'a [RectLayerKhr] {
        unsafe { slice::from_raw_parts(self.raw.pRectangles as *const _, self.raw.rectangleCount as usize) }
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

    pub fn offset<'a>(&'a self) -> Offset2d {
        self.raw.offset.into()
    }

    pub fn extent<'a>(&'a self) -> Extent2d {
        self.raw.extent.into()
    }

    pub fn layer<'a>(&'a self) -> u32 {
        self.raw.layer.into()
    }

    pub fn set_offset<'m>(& mut self, offset: Offset2d) {
        self.raw.offset = offset.raw;
    }

    pub fn set_extent<'m>(& mut self, extent: Extent2d) {
        self.raw.extent = extent.raw;
    }

    pub fn set_layer<'m>(& mut self, layer: u32) {
        self.raw.layer = layer.into();
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


impl From<vks::VkRectLayerKHR> for RectLayerKhr {
    fn from(f: vks::VkRectLayerKHR) -> RectLayerKhr {
        RectLayerKhr { raw: f, }
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

    pub fn get_offset<'a>(&'a self) -> Offset2d {
        self.raw.offset.into()
    }

    pub fn get_extent<'a>(&'a self) -> Extent2d {
        self.raw.extent.into()
    }

    pub fn get_layer<'a>(&'a self) -> u32 {
        self.raw.layer.into()
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn variable_pointers_storage_buffer<'a>(&'a self) -> bool {
        self.raw.variablePointersStorageBuffer != 0
    }

    pub fn variable_pointers<'a>(&'a self) -> bool {
        self.raw.variablePointers != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_variable_pointers_storage_buffer<'m>(&'s mut self, variable_pointers_storage_buffer: bool) {
        self.raw.variablePointersStorageBuffer = variable_pointers_storage_buffer as u32;
    }

    pub fn set_variable_pointers<'m>(&'s mut self, variable_pointers: bool) {
        self.raw.variablePointers = variable_pointers as u32;
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


impl<'s> From<vks::VkPhysicalDeviceVariablePointerFeaturesKHR> for PhysicalDeviceVariablePointerFeaturesKhr<'s> {
    fn from(f: vks::VkPhysicalDeviceVariablePointerFeaturesKHR) -> PhysicalDeviceVariablePointerFeaturesKhr<'s> {
        PhysicalDeviceVariablePointerFeaturesKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_variable_pointers_storage_buffer<'a>(&'a self) -> bool {
        self.raw.variablePointersStorageBuffer != 0
    }

    pub fn get_variable_pointers<'a>(&'a self) -> bool {
        self.raw.variablePointers != 0
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
    pub fn external_memory_features<'a>(&'a self) -> ExternalMemoryFeatureFlagsKhr {
        ExternalMemoryFeatureFlagsKhr::from_bits(self.raw.externalMemoryFeatures)
            .expect("ExternalMemoryPropertiesKhr::external_memory_features: error converting flags")
    }

    pub fn export_from_imported_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.exportFromImportedHandleTypes)
            .expect("ExternalMemoryPropertiesKhr::export_from_imported_handle_types: error converting flags")
    }

    pub fn compatible_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.compatibleHandleTypes)
            .expect("ExternalMemoryPropertiesKhr::compatible_handle_types: error converting flags")
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


impl From<vks::VkExternalMemoryPropertiesKHR> for ExternalMemoryPropertiesKhr {
    fn from(f: vks::VkExternalMemoryPropertiesKHR) -> ExternalMemoryPropertiesKhr {
        ExternalMemoryPropertiesKhr { raw: f, }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalImageFormatInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkPhysicalDeviceExternalImageFormatInfoKHR> for PhysicalDeviceExternalImageFormatInfoKhr<'s> {
    fn from(f: vks::VkPhysicalDeviceExternalImageFormatInfoKHR) -> PhysicalDeviceExternalImageFormatInfoKhr<'s> {
        PhysicalDeviceExternalImageFormatInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalImageFormatInfoKhr::handle_type: error converting flags")
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn external_memory_properties<'a>(&'a self) -> ExternalMemoryPropertiesKhr {
        self.raw.externalMemoryProperties.into()
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


impl<'s> From<vks::VkExternalImageFormatPropertiesKHR> for ExternalImageFormatPropertiesKhr<'s> {
    fn from(f: vks::VkExternalImageFormatPropertiesKHR) -> ExternalImageFormatPropertiesKhr<'s> {
        ExternalImageFormatPropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> BufferCreateFlags {
        BufferCreateFlags::from_bits(self.raw.flags)
            .expect("PhysicalDeviceExternalBufferInfoKhr::flags: error converting flags")
    }

    pub fn usage<'a>(&'a self) -> BufferUsageFlags {
        BufferUsageFlags::from_bits(self.raw.usage)
            .expect("PhysicalDeviceExternalBufferInfoKhr::usage: error converting flags")
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalBufferInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: BufferCreateFlags) {
        self.raw.flags = flags.bits();
    }

    pub fn set_usage<'m>(&'s mut self, usage: BufferUsageFlags) {
        self.raw.usage = usage.bits();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkPhysicalDeviceExternalBufferInfoKHR> for PhysicalDeviceExternalBufferInfoKhr<'s> {
    fn from(f: vks::VkPhysicalDeviceExternalBufferInfoKHR) -> PhysicalDeviceExternalBufferInfoKhr<'s> {
        PhysicalDeviceExternalBufferInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> BufferCreateFlags {
        BufferCreateFlags::from_bits(self.raw.flags)
            .expect("PhysicalDeviceExternalBufferInfoKhr::flags: error converting flags")
    }

    pub fn get_usage<'a>(&'a self) -> BufferUsageFlags {
        BufferUsageFlags::from_bits(self.raw.usage)
            .expect("PhysicalDeviceExternalBufferInfoKhr::usage: error converting flags")
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalBufferInfoKhr::handle_type: error converting flags")
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn external_memory_properties<'a>(&'a self) -> ExternalMemoryPropertiesKhr {
        self.raw.externalMemoryProperties.into()
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


impl<'s> From<vks::VkExternalBufferPropertiesKHR> for ExternalBufferPropertiesKhr<'s> {
    fn from(f: vks::VkExternalBufferPropertiesKHR) -> ExternalBufferPropertiesKhr<'s> {
        ExternalBufferPropertiesKhr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn device_uu_id<'a>(&'a self) -> &[u8] {
        unsafe { slice::from_raw_parts(&self.raw.deviceUUID as *const _, vks::VK_UUID_SIZE as usize) }
    }

    pub fn driver_uu_id<'a>(&'a self) -> &[u8] {
        unsafe { slice::from_raw_parts(&self.raw.driverUUID as *const _, vks::VK_UUID_SIZE as usize) }
    }

    pub fn device_lu_id<'a>(&'a self) -> &[u8] {
        unsafe { slice::from_raw_parts(&self.raw.deviceLUID as *const _, vks::VK_LUID_SIZE_KHR as usize) }
    }

    pub fn device_node_mask<'a>(&'a self) -> u32 {
        self.raw.deviceNodeMask.into()
    }

    pub fn device_lu_id_valid<'a>(&'a self) -> bool {
        self.raw.deviceLUIDValid != 0
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


impl<'s> From<vks::VkPhysicalDeviceIDPropertiesKHR> for PhysicalDeviceIDPropertiesKhr<'s> {
    fn from(f: vks::VkPhysicalDeviceIDPropertiesKHR) -> PhysicalDeviceIDPropertiesKhr<'s> {
        PhysicalDeviceIDPropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExternalMemoryImageCreateInfoKhr::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExternalMemoryImageCreateInfoKHR> for ExternalMemoryImageCreateInfoKhr<'s> {
    fn from(f: vks::VkExternalMemoryImageCreateInfoKHR) -> ExternalMemoryImageCreateInfoKhr<'s> {
        ExternalMemoryImageCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExternalMemoryImageCreateInfoKhr::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExternalMemoryBufferCreateInfoKhr::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExternalMemoryBufferCreateInfoKHR> for ExternalMemoryBufferCreateInfoKhr<'s> {
    fn from(f: vks::VkExternalMemoryBufferCreateInfoKHR) -> ExternalMemoryBufferCreateInfoKhr<'s> {
        ExternalMemoryBufferCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExternalMemoryBufferCreateInfoKhr::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExportMemoryAllocateInfoKhr::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExportMemoryAllocateInfoKHR> for ExportMemoryAllocateInfoKhr<'s> {
    fn from(f: vks::VkExportMemoryAllocateInfoKHR) -> ExportMemoryAllocateInfoKhr<'s> {
        ExportMemoryAllocateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExportMemoryAllocateInfoKhr::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportMemoryWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub fn handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub fn name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_handle<'m>(&'s mut self, handle: HANDLE) {
        self.raw.handle = handle.into();
    }

    pub fn set_name<'m>(&'s mut self, name: LPCWSTR) {
        self.raw.name = name.into();
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


impl<'s> From<vks::VkImportMemoryWin32HandleInfoKHR> for ImportMemoryWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkImportMemoryWin32HandleInfoKHR) -> ImportMemoryWin32HandleInfoKhr<'s> {
        ImportMemoryWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportMemoryWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub fn get_handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub fn get_name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub fn name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_attributes<'m, 'a>(&'s mut self, attributes: &'a SECURITY_ATTRIBUTES) {
        self.raw.pAttributes = attributes;
    }

    pub fn set_dw_access<'m>(&'s mut self, dw_access: DWORD) {
        self.raw.dwAccess = dw_access.into();
    }

    pub fn set_name<'m>(&'s mut self, name: LPCWSTR) {
        self.raw.name = name.into();
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


impl<'s> From<vks::VkExportMemoryWin32HandleInfoKHR> for ExportMemoryWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkExportMemoryWin32HandleInfoKHR) -> ExportMemoryWin32HandleInfoKhr<'s> {
        ExportMemoryWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn attributes<'m, 'a>(mut self, attributes: &'a SECURITY_ATTRIBUTES) -> ExportMemoryWin32HandleInfoKhrBuilder<'b> {
        self.raw.pAttributes = attributes;
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn get_dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub fn get_name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn memory_type_bits<'a>(&'a self) -> u32 {
        self.raw.memoryTypeBits.into()
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


impl<'s> From<vks::VkMemoryWin32HandlePropertiesKHR> for MemoryWin32HandlePropertiesKhr<'s> {
    fn from(f: vks::VkMemoryWin32HandlePropertiesKHR) -> MemoryWin32HandlePropertiesKhr<'s> {
        MemoryWin32HandlePropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("MemoryGetWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_memory<'m, 'a>(&'s mut self, memory: &'a DeviceMemory) {
        self.raw.memory = memory.handle();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkMemoryGetWin32HandleInfoKHR> for MemoryGetWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkMemoryGetWin32HandleInfoKHR) -> MemoryGetWin32HandleInfoKhr<'s> {
        MemoryGetWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("MemoryGetWin32HandleInfoKhr::handle_type: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportMemoryFdInfoKhr::handle_type: error converting flags")
    }

    pub fn fd<'a>(&'a self) -> i32 {
        self.raw.fd.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_fd<'m>(&'s mut self, fd: i32) {
        self.raw.fd = fd.into();
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


impl<'s> From<vks::VkImportMemoryFdInfoKHR> for ImportMemoryFdInfoKhr<'s> {
    fn from(f: vks::VkImportMemoryFdInfoKHR) -> ImportMemoryFdInfoKhr<'s> {
        ImportMemoryFdInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportMemoryFdInfoKhr::handle_type: error converting flags")
    }

    pub fn get_fd<'a>(&'a self) -> i32 {
        self.raw.fd.into()
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn memory_type_bits<'a>(&'a self) -> u32 {
        self.raw.memoryTypeBits.into()
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


impl<'s> From<vks::VkMemoryFdPropertiesKHR> for MemoryFdPropertiesKhr<'s> {
    fn from(f: vks::VkMemoryFdPropertiesKHR) -> MemoryFdPropertiesKhr<'s> {
        MemoryFdPropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("MemoryGetFdInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_memory<'m, 'a>(&'s mut self, memory: &'a DeviceMemory) {
        self.raw.memory = memory.handle();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalMemoryHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkMemoryGetFdInfoKHR> for MemoryGetFdInfoKhr<'s> {
    fn from(f: vks::VkMemoryGetFdInfoKHR) -> MemoryGetFdInfoKhr<'s> {
        MemoryGetFdInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_memory_handle<'a>(&'a self) -> vks::VkDeviceMemory {
        self.raw.memory
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalMemoryHandleTypeFlagsKhr {
        ExternalMemoryHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("MemoryGetFdInfoKhr::handle_type: error converting flags")
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
    acquire_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
    release_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
    _p: PhantomData<&'s ()>,
}

impl<'s> Win32KeyedMutexAcquireReleaseInfoKhr<'s> {
    pub fn builder<'b>() -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        Win32KeyedMutexAcquireReleaseInfoKhrBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn acquire_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireSyncs as *const _, self.raw.acquireCount as usize) }
    }

    pub fn acquire_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireKeys as *const _, self.raw.acquireCount as usize) }
    }

    pub fn acquire_timeouts<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireTimeouts as *const _, self.raw.acquireCount as usize) }
    }

    pub fn release_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseSyncs as *const _, self.raw.releaseCount as usize) }
    }

    pub fn release_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseKeys as *const _, self.raw.releaseCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_acquire_syncs<'m, 'a>(&'s mut self, acquire_syncs: &'a [&'a DeviceMemory])
            where 'a: 's {
        self.acquire_syncs = Some(acquire_syncs.iter().map(|h| h.handle()).collect());
        {
            let acquire_syncs = self.acquire_syncs.as_ref().unwrap();
            self.raw.pAcquireSyncs = acquire_syncs.as_ptr();
            assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::acquire_syncs`.");
            self.raw.acquireCount = acquire_syncs.len() as _;
        }
    }

    pub fn set_acquire_keys<'m, 'a>(&'s mut self, acquire_keys: &'a [u64]) {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::acquire_keys`.");
        self.raw.acquireCount = acquire_keys.len() as _;
        self.raw.pAcquireKeys = acquire_keys.as_ptr() as *const u64 as *const _;
    }

    pub fn set_acquire_timeouts<'m, 'a>(&'s mut self, acquire_timeouts: &'a [u32]) {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_timeouts.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::acquire_timeouts`.");
        self.raw.acquireCount = acquire_timeouts.len() as _;
        self.raw.pAcquireTimeouts = acquire_timeouts.as_ptr() as *const u32 as *const _;
    }

    pub fn set_release_syncs<'m, 'a>(&'s mut self, release_syncs: &'a [&'a DeviceMemory])
            where 'a: 's {
        self.release_syncs = Some(release_syncs.iter().map(|h| h.handle()).collect());
        {
            let release_syncs = self.release_syncs.as_ref().unwrap();
            self.raw.pReleaseSyncs = release_syncs.as_ptr();
            assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::release_syncs`.");
            self.raw.releaseCount = release_syncs.len() as _;
        }
    }

    pub fn set_release_keys<'m, 'a>(&'s mut self, release_keys: &'a [u64]) {
        assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::release_keys`.");
        self.raw.releaseCount = release_keys.len() as _;
        self.raw.pReleaseKeys = release_keys.as_ptr() as *const u64 as *const _;
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


impl<'s> From<vks::VkWin32KeyedMutexAcquireReleaseInfoKHR> for Win32KeyedMutexAcquireReleaseInfoKhr<'s> {
    fn from(f: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR) -> Win32KeyedMutexAcquireReleaseInfoKhr<'s> {
        Win32KeyedMutexAcquireReleaseInfoKhr { raw: f, acquire_syncs: None, release_syncs: None, _p: PhantomData }
    }
}


/// A builder for `VkWin32KeyedMutexAcquireReleaseInfoKHR`.
///
/// 
#[derive(Debug, Clone, Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
    raw: vks::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    acquire_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
    release_syncs: Option<SmallVec<[vks::VkDeviceMemory; 8]>>,
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

    pub fn acquire_syncs<'m, 'a>(mut self, acquire_syncs: &'a [&'a DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b>
            where 'a: 'b {
        self.acquire_syncs = Some(acquire_syncs.iter().map(|h| h.handle()).collect());
        {
            let acquire_syncs = self.acquire_syncs.as_ref().unwrap();
            self.raw.pAcquireSyncs = acquire_syncs.as_ptr();
            assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::acquire_syncs`.");
            self.raw.acquireCount = acquire_syncs.len() as _;
        }
        self
    }

    pub fn acquire_keys<'m, 'a>(mut self, acquire_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::acquire_keys`.");
        self.raw.acquireCount = acquire_keys.len() as _;
        self.raw.pAcquireKeys = acquire_keys.as_ptr() as *const u64 as *const _;
        self
    }

    pub fn acquire_timeouts<'m, 'a>(mut self, acquire_timeouts: &'a [u32]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        assert!(self.raw.acquireCount == 0 || self.raw.acquireCount == acquire_timeouts.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::acquire_timeouts`.");
        self.raw.acquireCount = acquire_timeouts.len() as _;
        self.raw.pAcquireTimeouts = acquire_timeouts.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn release_syncs<'m, 'a>(mut self, release_syncs: &'a [&'a DeviceMemory]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b>
            where 'a: 'b {
        self.release_syncs = Some(release_syncs.iter().map(|h| h.handle()).collect());
        {
            let release_syncs = self.release_syncs.as_ref().unwrap();
            self.raw.pReleaseSyncs = release_syncs.as_ptr();
            assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_syncs.len() as _, 
                "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::release_syncs`.");
            self.raw.releaseCount = release_syncs.len() as _;
        }
        self
    }

    pub fn release_keys<'m, 'a>(mut self, release_keys: &'a [u64]) -> Win32KeyedMutexAcquireReleaseInfoKhrBuilder<'b> {
        assert!(self.raw.releaseCount == 0 || self.raw.releaseCount == release_keys.len() as _, 
            "count inconsistency found when specifying `Win32KeyedMutexAcquireReleaseInfoKhr::release_keys`.");
        self.raw.releaseCount = release_keys.len() as _;
        self.raw.pReleaseKeys = release_keys.as_ptr() as *const u64 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_acquire_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireSyncs as *const _, self.raw.acquireCount as usize) }
    }

    pub fn get_acquire_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireKeys as *const _, self.raw.acquireCount as usize) }
    }

    pub fn get_acquire_timeouts<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pAcquireTimeouts as *const _, self.raw.acquireCount as usize) }
    }

    pub fn get_release_syncs_handle<'a>(&'a self) -> &'a [vks::VkDeviceMemory] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseSyncs as *const _, self.raw.releaseCount as usize) }
    }

    pub fn get_release_keys<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pReleaseKeys as *const _, self.raw.releaseCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalSemaphoreInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkPhysicalDeviceExternalSemaphoreInfoKHR> for PhysicalDeviceExternalSemaphoreInfoKhr<'s> {
    fn from(f: vks::VkPhysicalDeviceExternalSemaphoreInfoKHR) -> PhysicalDeviceExternalSemaphoreInfoKhr<'s> {
        PhysicalDeviceExternalSemaphoreInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalSemaphoreInfoKhr::handle_type: error converting flags")
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn export_from_imported_handle_types<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.exportFromImportedHandleTypes)
            .expect("ExternalSemaphorePropertiesKhr::export_from_imported_handle_types: error converting flags")
    }

    pub fn compatible_handle_types<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.compatibleHandleTypes)
            .expect("ExternalSemaphorePropertiesKhr::compatible_handle_types: error converting flags")
    }

    pub fn external_semaphore_features<'a>(&'a self) -> ExternalSemaphoreFeatureFlagsKhr {
        ExternalSemaphoreFeatureFlagsKhr::from_bits(self.raw.externalSemaphoreFeatures)
            .expect("ExternalSemaphorePropertiesKhr::external_semaphore_features: error converting flags")
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


impl<'s> From<vks::VkExternalSemaphorePropertiesKHR> for ExternalSemaphorePropertiesKhr<'s> {
    fn from(f: vks::VkExternalSemaphorePropertiesKHR) -> ExternalSemaphorePropertiesKhr<'s> {
        ExternalSemaphorePropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExportSemaphoreCreateInfoKhr::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalSemaphoreHandleTypeFlagsKhr) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExportSemaphoreCreateInfoKHR> for ExportSemaphoreCreateInfoKhr<'s> {
    fn from(f: vks::VkExportSemaphoreCreateInfoKHR) -> ExportSemaphoreCreateInfoKhr<'s> {
        ExportSemaphoreCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExportSemaphoreCreateInfoKhr::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn flags<'a>(&'a self) -> SemaphoreImportFlagsKhr {
        SemaphoreImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportSemaphoreWin32HandleInfoKhr::flags: error converting flags")
    }

    pub fn handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportSemaphoreWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub fn handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub fn name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_semaphore<'m, 'a>(&'s mut self, semaphore: &'a Semaphore) {
        self.raw.semaphore = semaphore.handle();
    }

    pub fn set_flags<'m>(&'s mut self, flags: SemaphoreImportFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_handle<'m>(&'s mut self, handle: HANDLE) {
        self.raw.handle = handle.into();
    }

    pub fn set_name<'m>(&'s mut self, name: LPCWSTR) {
        self.raw.name = name.into();
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


impl<'s> From<vks::VkImportSemaphoreWin32HandleInfoKHR> for ImportSemaphoreWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkImportSemaphoreWin32HandleInfoKHR) -> ImportSemaphoreWin32HandleInfoKhr<'s> {
        ImportSemaphoreWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn get_flags<'a>(&'a self) -> SemaphoreImportFlagsKhr {
        SemaphoreImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportSemaphoreWin32HandleInfoKhr::flags: error converting flags")
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportSemaphoreWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub fn get_handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub fn get_name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub fn name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_attributes<'m, 'a>(&'s mut self, attributes: &'a SECURITY_ATTRIBUTES) {
        self.raw.pAttributes = attributes;
    }

    pub fn set_dw_access<'m>(&'s mut self, dw_access: DWORD) {
        self.raw.dwAccess = dw_access.into();
    }

    pub fn set_name<'m>(&'s mut self, name: LPCWSTR) {
        self.raw.name = name.into();
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


impl<'s> From<vks::VkExportSemaphoreWin32HandleInfoKHR> for ExportSemaphoreWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkExportSemaphoreWin32HandleInfoKHR) -> ExportSemaphoreWin32HandleInfoKhr<'s> {
        ExportSemaphoreWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn attributes<'m, 'a>(mut self, attributes: &'a SECURITY_ATTRIBUTES) -> ExportSemaphoreWin32HandleInfoKhrBuilder<'b> {
        self.raw.pAttributes = attributes;
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn get_dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub fn get_name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn wait_semaphore_values<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphoreValues as *const _, self.raw.waitSemaphoreValuesCount as usize) }
    }

    pub fn signal_semaphore_values<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphoreValues as *const _, self.raw.signalSemaphoreValuesCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_wait_semaphore_values<'m, 'a>(&'s mut self, wait_semaphore_values: &'a [u64]) {
        assert!(self.raw.waitSemaphoreValuesCount == 0 || self.raw.waitSemaphoreValuesCount == wait_semaphore_values.len() as _, 
            "count inconsistency found when specifying `D3d12FenceSubmitInfoKHR::wait_semaphore_values`.");
        self.raw.waitSemaphoreValuesCount = wait_semaphore_values.len() as _;
        self.raw.pWaitSemaphoreValues = wait_semaphore_values.as_ptr() as *const u64 as *const _;
    }

    pub fn set_signal_semaphore_values<'m, 'a>(&'s mut self, signal_semaphore_values: &'a [u64]) {
        assert!(self.raw.signalSemaphoreValuesCount == 0 || self.raw.signalSemaphoreValuesCount == signal_semaphore_values.len() as _, 
            "count inconsistency found when specifying `D3d12FenceSubmitInfoKHR::signal_semaphore_values`.");
        self.raw.signalSemaphoreValuesCount = signal_semaphore_values.len() as _;
        self.raw.pSignalSemaphoreValues = signal_semaphore_values.as_ptr() as *const u64 as *const _;
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


impl<'s> From<vks::VkD3D12FenceSubmitInfoKHR> for D3d12FenceSubmitInfoKHR<'s> {
    fn from(f: vks::VkD3D12FenceSubmitInfoKHR) -> D3d12FenceSubmitInfoKHR<'s> {
        D3d12FenceSubmitInfoKHR { raw: f, _p: PhantomData }
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

    pub fn wait_semaphore_values<'m, 'a>(mut self, wait_semaphore_values: &'a [u64]) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        assert!(self.raw.waitSemaphoreValuesCount == 0 || self.raw.waitSemaphoreValuesCount == wait_semaphore_values.len() as _, 
            "count inconsistency found when specifying `D3d12FenceSubmitInfoKHR::wait_semaphore_values`.");
        self.raw.waitSemaphoreValuesCount = wait_semaphore_values.len() as _;
        self.raw.pWaitSemaphoreValues = wait_semaphore_values.as_ptr() as *const u64 as *const _;
        self
    }

    pub fn signal_semaphore_values<'m, 'a>(mut self, signal_semaphore_values: &'a [u64]) -> D3d12FenceSubmitInfoKHRBuilder<'b> {
        assert!(self.raw.signalSemaphoreValuesCount == 0 || self.raw.signalSemaphoreValuesCount == signal_semaphore_values.len() as _, 
            "count inconsistency found when specifying `D3d12FenceSubmitInfoKHR::signal_semaphore_values`.");
        self.raw.signalSemaphoreValuesCount = signal_semaphore_values.len() as _;
        self.raw.pSignalSemaphoreValues = signal_semaphore_values.as_ptr() as *const u64 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_wait_semaphore_values<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphoreValues as *const _, self.raw.waitSemaphoreValuesCount as usize) }
    }

    pub fn get_signal_semaphore_values<'a>(&'a self) -> &'a [u64] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphoreValues as *const _, self.raw.signalSemaphoreValuesCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("SemaphoreGetWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_semaphore<'m, 'a>(&'s mut self, semaphore: &'a Semaphore) {
        self.raw.semaphore = semaphore.handle();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkSemaphoreGetWin32HandleInfoKHR> for SemaphoreGetWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkSemaphoreGetWin32HandleInfoKHR) -> SemaphoreGetWin32HandleInfoKhr<'s> {
        SemaphoreGetWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("SemaphoreGetWin32HandleInfoKhr::handle_type: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn flags<'a>(&'a self) -> SemaphoreImportFlagsKhr {
        SemaphoreImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportSemaphoreFdInfoKhr::flags: error converting flags")
    }

    pub fn handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportSemaphoreFdInfoKhr::handle_type: error converting flags")
    }

    pub fn fd<'a>(&'a self) -> i32 {
        self.raw.fd.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_semaphore<'m, 'a>(&'s mut self, semaphore: &'a Semaphore) {
        self.raw.semaphore = semaphore.handle();
    }

    pub fn set_flags<'m>(&'s mut self, flags: SemaphoreImportFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_fd<'m>(&'s mut self, fd: i32) {
        self.raw.fd = fd.into();
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


impl<'s> From<vks::VkImportSemaphoreFdInfoKHR> for ImportSemaphoreFdInfoKhr<'s> {
    fn from(f: vks::VkImportSemaphoreFdInfoKHR) -> ImportSemaphoreFdInfoKhr<'s> {
        ImportSemaphoreFdInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn get_flags<'a>(&'a self) -> SemaphoreImportFlagsKhr {
        SemaphoreImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportSemaphoreFdInfoKhr::flags: error converting flags")
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportSemaphoreFdInfoKhr::handle_type: error converting flags")
    }

    pub fn get_fd<'a>(&'a self) -> i32 {
        self.raw.fd.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("SemaphoreGetFdInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_semaphore<'m, 'a>(&'s mut self, semaphore: &'a Semaphore) {
        self.raw.semaphore = semaphore.handle();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalSemaphoreHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkSemaphoreGetFdInfoKHR> for SemaphoreGetFdInfoKhr<'s> {
    fn from(f: vks::VkSemaphoreGetFdInfoKHR) -> SemaphoreGetFdInfoKhr<'s> {
        SemaphoreGetFdInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalSemaphoreHandleTypeFlagsKhr {
        ExternalSemaphoreHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("SemaphoreGetFdInfoKhr::handle_type: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalFenceInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkPhysicalDeviceExternalFenceInfoKHR> for PhysicalDeviceExternalFenceInfoKhr<'s> {
    fn from(f: vks::VkPhysicalDeviceExternalFenceInfoKHR) -> PhysicalDeviceExternalFenceInfoKhr<'s> {
        PhysicalDeviceExternalFenceInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("PhysicalDeviceExternalFenceInfoKhr::handle_type: error converting flags")
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn export_from_imported_handle_types<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.exportFromImportedHandleTypes)
            .expect("ExternalFencePropertiesKhr::export_from_imported_handle_types: error converting flags")
    }

    pub fn compatible_handle_types<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.compatibleHandleTypes)
            .expect("ExternalFencePropertiesKhr::compatible_handle_types: error converting flags")
    }

    pub fn external_fence_features<'a>(&'a self) -> ExternalFenceFeatureFlagsKhr {
        ExternalFenceFeatureFlagsKhr::from_bits(self.raw.externalFenceFeatures)
            .expect("ExternalFencePropertiesKhr::external_fence_features: error converting flags")
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


impl<'s> From<vks::VkExternalFencePropertiesKHR> for ExternalFencePropertiesKhr<'s> {
    fn from(f: vks::VkExternalFencePropertiesKHR) -> ExternalFencePropertiesKhr<'s> {
        ExternalFencePropertiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn handle_types<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExportFenceCreateInfoKhr::handle_types: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_handle_types<'m>(&'s mut self, handle_types: ExternalFenceHandleTypeFlagsKhr) {
        self.raw.handleTypes = handle_types.bits();
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


impl<'s> From<vks::VkExportFenceCreateInfoKHR> for ExportFenceCreateInfoKhr<'s> {
    fn from(f: vks::VkExportFenceCreateInfoKHR) -> ExportFenceCreateInfoKhr<'s> {
        ExportFenceCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_handle_types<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleTypes)
            .expect("ExportFenceCreateInfoKhr::handle_types: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn flags<'a>(&'a self) -> FenceImportFlagsKhr {
        FenceImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportFenceWin32HandleInfoKhr::flags: error converting flags")
    }

    pub fn handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportFenceWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub fn handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub fn name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_fence<'m, 'a>(&'s mut self, fence: &'a Fence) {
        self.raw.fence = fence.handle();
    }

    pub fn set_flags<'m>(&'s mut self, flags: FenceImportFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_handle<'m>(&'s mut self, handle: HANDLE) {
        self.raw.handle = handle.into();
    }

    pub fn set_name<'m>(&'s mut self, name: LPCWSTR) {
        self.raw.name = name.into();
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


impl<'s> From<vks::VkImportFenceWin32HandleInfoKHR> for ImportFenceWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkImportFenceWin32HandleInfoKHR) -> ImportFenceWin32HandleInfoKhr<'s> {
        ImportFenceWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn get_flags<'a>(&'a self) -> FenceImportFlagsKhr {
        FenceImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportFenceWin32HandleInfoKhr::flags: error converting flags")
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportFenceWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub fn get_handle<'a>(&'a self) -> HANDLE {
        self.raw.handle.into()
    }

    pub fn get_name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub fn name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_attributes<'m, 'a>(&'s mut self, attributes: &'a SECURITY_ATTRIBUTES) {
        self.raw.pAttributes = attributes;
    }

    pub fn set_dw_access<'m>(&'s mut self, dw_access: DWORD) {
        self.raw.dwAccess = dw_access.into();
    }

    pub fn set_name<'m>(&'s mut self, name: LPCWSTR) {
        self.raw.name = name.into();
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


impl<'s> From<vks::VkExportFenceWin32HandleInfoKHR> for ExportFenceWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkExportFenceWin32HandleInfoKHR) -> ExportFenceWin32HandleInfoKhr<'s> {
        ExportFenceWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn attributes<'m, 'a>(mut self, attributes: &'a SECURITY_ATTRIBUTES) -> ExportFenceWin32HandleInfoKhrBuilder<'b> {
        self.raw.pAttributes = attributes;
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_attributes<'a>(&'a self) -> &'a SECURITY_ATTRIBUTES {
        unsafe { &*(self.raw.pAttributes as *const _) }
    }

    pub fn get_dw_access<'a>(&'a self) -> DWORD {
        self.raw.dwAccess.into()
    }

    pub fn get_name<'a>(&'a self) -> LPCWSTR {
        self.raw.name.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("FenceGetWin32HandleInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_fence<'m, 'a>(&'s mut self, fence: &'a Fence) {
        self.raw.fence = fence.handle();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkFenceGetWin32HandleInfoKHR> for FenceGetWin32HandleInfoKhr<'s> {
    fn from(f: vks::VkFenceGetWin32HandleInfoKHR) -> FenceGetWin32HandleInfoKhr<'s> {
        FenceGetWin32HandleInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("FenceGetWin32HandleInfoKhr::handle_type: error converting flags")
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn flags<'a>(&'a self) -> FenceImportFlagsKhr {
        FenceImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportFenceFdInfoKhr::flags: error converting flags")
    }

    pub fn handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportFenceFdInfoKhr::handle_type: error converting flags")
    }

    pub fn fd<'a>(&'a self) -> i32 {
        self.raw.fd.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_fence<'m, 'a>(&'s mut self, fence: &'a Fence) {
        self.raw.fence = fence.handle();
    }

    pub fn set_flags<'m>(&'s mut self, flags: FenceImportFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
    }

    pub fn set_fd<'m>(&'s mut self, fd: i32) {
        self.raw.fd = fd.into();
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


impl<'s> From<vks::VkImportFenceFdInfoKHR> for ImportFenceFdInfoKhr<'s> {
    fn from(f: vks::VkImportFenceFdInfoKHR) -> ImportFenceFdInfoKhr<'s> {
        ImportFenceFdInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn get_flags<'a>(&'a self) -> FenceImportFlagsKhr {
        FenceImportFlagsKhr::from_bits(self.raw.flags)
            .expect("ImportFenceFdInfoKhr::flags: error converting flags")
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("ImportFenceFdInfoKhr::handle_type: error converting flags")
    }

    pub fn get_fd<'a>(&'a self) -> i32 {
        self.raw.fd.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("FenceGetFdInfoKhr::handle_type: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_fence<'m, 'a>(&'s mut self, fence: &'a Fence) {
        self.raw.fence = fence.handle();
    }

    pub fn set_handle_type<'m>(&'s mut self, handle_type: ExternalFenceHandleTypeFlagsKhr) {
        self.raw.handleType = handle_type.bits();
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


impl<'s> From<vks::VkFenceGetFdInfoKHR> for FenceGetFdInfoKhr<'s> {
    fn from(f: vks::VkFenceGetFdInfoKHR) -> FenceGetFdInfoKhr<'s> {
        FenceGetFdInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn get_handle_type<'a>(&'a self) -> ExternalFenceHandleTypeFlagsKhr {
        ExternalFenceHandleTypeFlagsKhr::from_bits(self.raw.handleType)
            .expect("FenceGetFdInfoKhr::handle_type: error converting flags")
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn multiview<'a>(&'a self) -> bool {
        self.raw.multiview != 0
    }

    pub fn multiview_geometry_shader<'a>(&'a self) -> bool {
        self.raw.multiviewGeometryShader != 0
    }

    pub fn multiview_tessellation_shader<'a>(&'a self) -> bool {
        self.raw.multiviewTessellationShader != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_multiview<'m>(&'s mut self, multiview: bool) {
        self.raw.multiview = multiview as u32;
    }

    pub fn set_multiview_geometry_shader<'m>(&'s mut self, multiview_geometry_shader: bool) {
        self.raw.multiviewGeometryShader = multiview_geometry_shader as u32;
    }

    pub fn set_multiview_tessellation_shader<'m>(&'s mut self, multiview_tessellation_shader: bool) {
        self.raw.multiviewTessellationShader = multiview_tessellation_shader as u32;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkPhysicalDeviceMultiviewFeaturesKHX> for PhysicalDeviceMultiviewFeaturesKhx<'s> {
    fn from(f: vks::VkPhysicalDeviceMultiviewFeaturesKHX) -> PhysicalDeviceMultiviewFeaturesKhx<'s> {
        PhysicalDeviceMultiviewFeaturesKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_multiview<'a>(&'a self) -> bool {
        self.raw.multiview != 0
    }

    pub fn get_multiview_geometry_shader<'a>(&'a self) -> bool {
        self.raw.multiviewGeometryShader != 0
    }

    pub fn get_multiview_tessellation_shader<'a>(&'a self) -> bool {
        self.raw.multiviewTessellationShader != 0
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn max_multiview_view_count<'a>(&'a self) -> u32 {
        self.raw.maxMultiviewViewCount.into()
    }

    pub fn max_multiview_instance_index<'a>(&'a self) -> u32 {
        self.raw.maxMultiviewInstanceIndex.into()
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkPhysicalDeviceMultiviewPropertiesKHX> for PhysicalDeviceMultiviewPropertiesKhx<'s> {
    fn from(f: vks::VkPhysicalDeviceMultiviewPropertiesKHX) -> PhysicalDeviceMultiviewPropertiesKhx<'s> {
        PhysicalDeviceMultiviewPropertiesKhx { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn subpass_count<'a>(&'a self) -> u32 {
        self.raw.subpassCount.into()
    }

    pub fn view_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { &*(self.raw.pViewMasks as *const _) }
    }

    pub fn dependency_count<'a>(&'a self) -> u32 {
        self.raw.dependencyCount.into()
    }

    pub fn view_offsets<'a>(&'a self) -> &'a [i32] {
        unsafe { &*(self.raw.pViewOffsets as *const _) }
    }

    pub fn correlation_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pCorrelationMasks as *const _, self.raw.correlationMaskCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_subpass_count<'m>(&'s mut self, subpass_count: u32) {
        self.raw.subpassCount = subpass_count.into();
    }

    pub fn set_view_masks<'m, 'a>(&'s mut self, view_masks: &'a [u32]) {
        self.raw.pViewMasks = view_masks.as_ptr() as *const u32 as *const _;
    }

    pub fn set_dependency_count<'m>(&'s mut self, dependency_count: u32) {
        self.raw.dependencyCount = dependency_count.into();
    }

    pub fn set_view_offsets<'m, 'a>(&'s mut self, view_offsets: &'a [i32]) {
        self.raw.pViewOffsets = view_offsets.as_ptr() as *const i32 as *const _;
    }

    pub fn set_correlation_masks<'m, 'a>(&'s mut self, correlation_masks: &'a [u32]) {
        assert!(self.raw.correlationMaskCount == 0 || self.raw.correlationMaskCount == correlation_masks.len() as _, 
            "count inconsistency found when specifying `RenderPassMultiviewCreateInfoKhx::correlation_masks`.");
        self.raw.correlationMaskCount = correlation_masks.len() as _;
        self.raw.pCorrelationMasks = correlation_masks.as_ptr() as *const u32 as *const _;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkRenderPassMultiviewCreateInfoKHX> for RenderPassMultiviewCreateInfoKhx<'s> {
    fn from(f: vks::VkRenderPassMultiviewCreateInfoKHX) -> RenderPassMultiviewCreateInfoKhx<'s> {
        RenderPassMultiviewCreateInfoKhx { raw: f, _p: PhantomData }
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
        self.raw.pViewMasks = view_masks.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn dependency_count<'m>(mut self, dependency_count: u32) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.dependencyCount = dependency_count.into();
        self
    }

    pub fn view_offsets<'m, 'a>(mut self, view_offsets: &'a [i32]) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        self.raw.pViewOffsets = view_offsets.as_ptr() as *const i32 as *const _;
        self
    }

    pub fn correlation_masks<'m, 'a>(mut self, correlation_masks: &'a [u32]) -> RenderPassMultiviewCreateInfoKhxBuilder<'b> {
        assert!(self.raw.correlationMaskCount == 0 || self.raw.correlationMaskCount == correlation_masks.len() as _, 
            "count inconsistency found when specifying `RenderPassMultiviewCreateInfoKhx::correlation_masks`.");
        self.raw.correlationMaskCount = correlation_masks.len() as _;
        self.raw.pCorrelationMasks = correlation_masks.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_subpass_count<'a>(&'a self) -> u32 {
        self.raw.subpassCount.into()
    }

    pub fn get_view_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { &*(self.raw.pViewMasks as *const _) }
    }

    pub fn get_dependency_count<'a>(&'a self) -> u32 {
        self.raw.dependencyCount.into()
    }

    pub fn get_view_offsets<'a>(&'a self) -> &'a [i32] {
        unsafe { &*(self.raw.pViewOffsets as *const _) }
    }

    pub fn get_correlation_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pCorrelationMasks as *const _, self.raw.correlationMaskCount as usize) }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn min_image_count<'a>(&'a self) -> u32 {
        self.raw.minImageCount.into()
    }

    pub fn max_image_count<'a>(&'a self) -> u32 {
        self.raw.maxImageCount.into()
    }

    pub fn current_extent<'a>(&'a self) -> Extent2d {
        self.raw.currentExtent.into()
    }

    pub fn min_image_extent<'a>(&'a self) -> Extent2d {
        self.raw.minImageExtent.into()
    }

    pub fn max_image_extent<'a>(&'a self) -> Extent2d {
        self.raw.maxImageExtent.into()
    }

    pub fn max_image_array_layers<'a>(&'a self) -> u32 {
        self.raw.maxImageArrayLayers.into()
    }

    pub fn supported_transforms<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.supportedTransforms)
            .expect("SurfaceCapabilities2Ext::supported_transforms: error converting flags")
    }

    pub fn current_transform<'a>(&'a self) -> SurfaceTransformFlagsKhr {
        SurfaceTransformFlagsKhr::from_bits(self.raw.currentTransform)
            .expect("SurfaceCapabilities2Ext::current_transform: error converting flags")
    }

    pub fn supported_composite_alpha<'a>(&'a self) -> CompositeAlphaFlagsKhr {
        CompositeAlphaFlagsKhr::from_bits(self.raw.supportedCompositeAlpha)
            .expect("SurfaceCapabilities2Ext::supported_composite_alpha: error converting flags")
    }

    pub fn supported_usage_flags<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.supportedUsageFlags)
            .expect("SurfaceCapabilities2Ext::supported_usage_flags: error converting flags")
    }

    pub fn supported_surface_counters<'a>(&'a self) -> SurfaceCounterFlagsExt {
        SurfaceCounterFlagsExt::from_bits(self.raw.supportedSurfaceCounters)
            .expect("SurfaceCapabilities2Ext::supported_surface_counters: error converting flags")
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


impl<'s> From<vks::VkSurfaceCapabilities2EXT> for SurfaceCapabilities2Ext<'s> {
    fn from(f: vks::VkSurfaceCapabilities2EXT) -> SurfaceCapabilities2Ext<'s> {
        SurfaceCapabilities2Ext { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn power_state<'a>(&'a self) -> DisplayPowerStateExt {
        self.raw.powerState.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_power_state<'m>(&'s mut self, power_state: DisplayPowerStateExt) {
        self.raw.powerState = power_state.into();
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


impl<'s> From<vks::VkDisplayPowerInfoEXT> for DisplayPowerInfoExt<'s> {
    fn from(f: vks::VkDisplayPowerInfoEXT) -> DisplayPowerInfoExt<'s> {
        DisplayPowerInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_power_state<'a>(&'a self) -> DisplayPowerStateExt {
        self.raw.powerState.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn device_event<'a>(&'a self) -> DeviceEventTypeExt {
        self.raw.deviceEvent.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_device_event<'m>(&'s mut self, device_event: DeviceEventTypeExt) {
        self.raw.deviceEvent = device_event.into();
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


impl<'s> From<vks::VkDeviceEventInfoEXT> for DeviceEventInfoExt<'s> {
    fn from(f: vks::VkDeviceEventInfoEXT) -> DeviceEventInfoExt<'s> {
        DeviceEventInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_device_event<'a>(&'a self) -> DeviceEventTypeExt {
        self.raw.deviceEvent.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn display_event<'a>(&'a self) -> DisplayEventTypeExt {
        self.raw.displayEvent.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_display_event<'m>(&'s mut self, display_event: DisplayEventTypeExt) {
        self.raw.displayEvent = display_event.into();
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


impl<'s> From<vks::VkDisplayEventInfoEXT> for DisplayEventInfoExt<'s> {
    fn from(f: vks::VkDisplayEventInfoEXT) -> DisplayEventInfoExt<'s> {
        DisplayEventInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_display_event<'a>(&'a self) -> DisplayEventTypeExt {
        self.raw.displayEvent.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn surface_counters<'a>(&'a self) -> SurfaceCounterFlagsExt {
        SurfaceCounterFlagsExt::from_bits(self.raw.surfaceCounters)
            .expect("SwapchainCounterCreateInfoExt::surface_counters: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_surface_counters<'m>(&'s mut self, surface_counters: SurfaceCounterFlagsExt) {
        self.raw.surfaceCounters = surface_counters.bits();
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


impl<'s> From<vks::VkSwapchainCounterCreateInfoEXT> for SwapchainCounterCreateInfoExt<'s> {
    fn from(f: vks::VkSwapchainCounterCreateInfoEXT) -> SwapchainCounterCreateInfoExt<'s> {
        SwapchainCounterCreateInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_surface_counters<'a>(&'a self) -> SurfaceCounterFlagsExt {
        SurfaceCounterFlagsExt::from_bits(self.raw.surfaceCounters)
            .expect("SwapchainCounterCreateInfoExt::surface_counters: error converting flags")
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn physical_device_count<'a>(&'a self) -> u32 {
        self.raw.physicalDeviceCount.into()
    }

    pub fn physical_devices_handle<'a>(&'a self) -> vks::VkPhysicalDevice {
        unsafe { slice::from_raw_parts(&self.raw.physicalDevices as *const _, vks::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize) }
    }

    pub fn subset_allocation<'a>(&'a self) -> bool {
        self.raw.subsetAllocation != 0
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkPhysicalDeviceGroupPropertiesKHX> for PhysicalDeviceGroupPropertiesKhx<'s> {
    fn from(f: vks::VkPhysicalDeviceGroupPropertiesKHX) -> PhysicalDeviceGroupPropertiesKhx<'s> {
        PhysicalDeviceGroupPropertiesKhx { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> MemoryAllocateFlagsKhx {
        MemoryAllocateFlagsKhx::from_bits(self.raw.flags)
            .expect("MemoryAllocateFlagsInfoKhx::flags: error converting flags")
    }

    pub fn device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: MemoryAllocateFlagsKhx) {
        self.raw.flags = flags.bits();
    }

    pub fn set_device_mask<'m>(&'s mut self, device_mask: u32) {
        self.raw.deviceMask = device_mask.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkMemoryAllocateFlagsInfoKHX> for MemoryAllocateFlagsInfoKhx<'s> {
    fn from(f: vks::VkMemoryAllocateFlagsInfoKHX) -> MemoryAllocateFlagsInfoKhx<'s> {
        MemoryAllocateFlagsInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> MemoryAllocateFlagsKhx {
        MemoryAllocateFlagsKhx::from_bits(self.raw.flags)
            .expect("MemoryAllocateFlagsInfoKhx::flags: error converting flags")
    }

    pub fn get_device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pDeviceIndices as *const _, self.raw.deviceIndexCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_device_indices<'m, 'a>(&'s mut self, device_indices: &'a [u32]) {
        assert!(self.raw.deviceIndexCount == 0 || self.raw.deviceIndexCount == device_indices.len() as _, 
            "count inconsistency found when specifying `BindBufferMemoryDeviceGroupInfoKhx::device_indices`.");
        self.raw.deviceIndexCount = device_indices.len() as _;
        self.raw.pDeviceIndices = device_indices.as_ptr() as *const u32 as *const _;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkBindBufferMemoryDeviceGroupInfoKHX> for BindBufferMemoryDeviceGroupInfoKhx<'s> {
    fn from(f: vks::VkBindBufferMemoryDeviceGroupInfoKHX) -> BindBufferMemoryDeviceGroupInfoKhx<'s> {
        BindBufferMemoryDeviceGroupInfoKhx { raw: f, _p: PhantomData }
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

    pub fn device_indices<'m, 'a>(mut self, device_indices: &'a [u32]) -> BindBufferMemoryDeviceGroupInfoKhxBuilder<'b> {
        assert!(self.raw.deviceIndexCount == 0 || self.raw.deviceIndexCount == device_indices.len() as _, 
            "count inconsistency found when specifying `BindBufferMemoryDeviceGroupInfoKhx::device_indices`.");
        self.raw.deviceIndexCount = device_indices.len() as _;
        self.raw.pDeviceIndices = device_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pDeviceIndices as *const _, self.raw.deviceIndexCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pDeviceIndices as *const _, self.raw.deviceIndexCount as usize) }
    }

    pub fn s_fr_rects<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pSFRRects as *const _, self.raw.SFRRectCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_device_indices<'m, 'a>(&'s mut self, device_indices: &'a [u32]) {
        assert!(self.raw.deviceIndexCount == 0 || self.raw.deviceIndexCount == device_indices.len() as _, 
            "count inconsistency found when specifying `BindImageMemoryDeviceGroupInfoKhx::device_indices`.");
        self.raw.deviceIndexCount = device_indices.len() as _;
        self.raw.pDeviceIndices = device_indices.as_ptr() as *const u32 as *const _;
    }

    pub fn set_s_fr_rects<'m, 'a>(&'s mut self, s_fr_rects: &'a [Rect2d]) {
        assert!(self.raw.SFRRectCount == 0 || self.raw.SFRRectCount == s_fr_rects.len() as _, 
            "count inconsistency found when specifying `BindImageMemoryDeviceGroupInfoKhx::s_fr_rects`.");
        self.raw.SFRRectCount = s_fr_rects.len() as _;
        self.raw.pSFRRects = s_fr_rects.as_ptr() as *const vks::VkRect2D as *const _;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkBindImageMemoryDeviceGroupInfoKHX> for BindImageMemoryDeviceGroupInfoKhx<'s> {
    fn from(f: vks::VkBindImageMemoryDeviceGroupInfoKHX) -> BindImageMemoryDeviceGroupInfoKhx<'s> {
        BindImageMemoryDeviceGroupInfoKhx { raw: f, _p: PhantomData }
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

    pub fn device_indices<'m, 'a>(mut self, device_indices: &'a [u32]) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        assert!(self.raw.deviceIndexCount == 0 || self.raw.deviceIndexCount == device_indices.len() as _, 
            "count inconsistency found when specifying `BindImageMemoryDeviceGroupInfoKhx::device_indices`.");
        self.raw.deviceIndexCount = device_indices.len() as _;
        self.raw.pDeviceIndices = device_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn s_fr_rects<'m, 'a>(mut self, s_fr_rects: &'a [Rect2d]) -> BindImageMemoryDeviceGroupInfoKhxBuilder<'b> {
        assert!(self.raw.SFRRectCount == 0 || self.raw.SFRRectCount == s_fr_rects.len() as _, 
            "count inconsistency found when specifying `BindImageMemoryDeviceGroupInfoKhx::s_fr_rects`.");
        self.raw.SFRRectCount = s_fr_rects.len() as _;
        self.raw.pSFRRects = s_fr_rects.as_ptr() as *const vks::VkRect2D as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pDeviceIndices as *const _, self.raw.deviceIndexCount as usize) }
    }

    pub fn get_s_fr_rects<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pSFRRects as *const _, self.raw.SFRRectCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
    }

    pub fn device_render_areas<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pDeviceRenderAreas as *const _, self.raw.deviceRenderAreaCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_device_mask<'m>(&'s mut self, device_mask: u32) {
        self.raw.deviceMask = device_mask.into();
    }

    pub fn set_device_render_areas<'m, 'a>(&'s mut self, device_render_areas: &'a [Rect2d]) {
        assert!(self.raw.deviceRenderAreaCount == 0 || self.raw.deviceRenderAreaCount == device_render_areas.len() as _, 
            "count inconsistency found when specifying `DeviceGroupRenderPassBeginInfoKhx::device_render_areas`.");
        self.raw.deviceRenderAreaCount = device_render_areas.len() as _;
        self.raw.pDeviceRenderAreas = device_render_areas.as_ptr() as *const vks::VkRect2D as *const _;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupRenderPassBeginInfoKHX> for DeviceGroupRenderPassBeginInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupRenderPassBeginInfoKHX) -> DeviceGroupRenderPassBeginInfoKhx<'s> {
        DeviceGroupRenderPassBeginInfoKhx { raw: f, _p: PhantomData }
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

    pub fn device_render_areas<'m, 'a>(mut self, device_render_areas: &'a [Rect2d]) -> DeviceGroupRenderPassBeginInfoKhxBuilder<'b> {
        assert!(self.raw.deviceRenderAreaCount == 0 || self.raw.deviceRenderAreaCount == device_render_areas.len() as _, 
            "count inconsistency found when specifying `DeviceGroupRenderPassBeginInfoKhx::device_render_areas`.");
        self.raw.deviceRenderAreaCount = device_render_areas.len() as _;
        self.raw.pDeviceRenderAreas = device_render_areas.as_ptr() as *const vks::VkRect2D as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
    }

    pub fn get_device_render_areas<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pDeviceRenderAreas as *const _, self.raw.deviceRenderAreaCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_device_mask<'m>(&'s mut self, device_mask: u32) {
        self.raw.deviceMask = device_mask.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupCommandBufferBeginInfoKHX> for DeviceGroupCommandBufferBeginInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupCommandBufferBeginInfoKHX) -> DeviceGroupCommandBufferBeginInfoKhx<'s> {
        DeviceGroupCommandBufferBeginInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn wait_semaphore_device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphoreDeviceIndices as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn command_buffer_device_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pCommandBufferDeviceMasks as *const _, self.raw.commandBufferCount as usize) }
    }

    pub fn signal_semaphore_device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphoreDeviceIndices as *const _, self.raw.signalSemaphoreCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_wait_semaphore_device_indices<'m, 'a>(&'s mut self, wait_semaphore_device_indices: &'a [u32]) {
        assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphore_device_indices.len() as _, 
            "count inconsistency found when specifying `DeviceGroupSubmitInfoKhx::wait_semaphore_device_indices`.");
        self.raw.waitSemaphoreCount = wait_semaphore_device_indices.len() as _;
        self.raw.pWaitSemaphoreDeviceIndices = wait_semaphore_device_indices.as_ptr() as *const u32 as *const _;
    }

    pub fn set_command_buffer_device_masks<'m, 'a>(&'s mut self, command_buffer_device_masks: &'a [u32]) {
        assert!(self.raw.commandBufferCount == 0 || self.raw.commandBufferCount == command_buffer_device_masks.len() as _, 
            "count inconsistency found when specifying `DeviceGroupSubmitInfoKhx::command_buffer_device_masks`.");
        self.raw.commandBufferCount = command_buffer_device_masks.len() as _;
        self.raw.pCommandBufferDeviceMasks = command_buffer_device_masks.as_ptr() as *const u32 as *const _;
    }

    pub fn set_signal_semaphore_device_indices<'m, 'a>(&'s mut self, signal_semaphore_device_indices: &'a [u32]) {
        assert!(self.raw.signalSemaphoreCount == 0 || self.raw.signalSemaphoreCount == signal_semaphore_device_indices.len() as _, 
            "count inconsistency found when specifying `DeviceGroupSubmitInfoKhx::signal_semaphore_device_indices`.");
        self.raw.signalSemaphoreCount = signal_semaphore_device_indices.len() as _;
        self.raw.pSignalSemaphoreDeviceIndices = signal_semaphore_device_indices.as_ptr() as *const u32 as *const _;
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupSubmitInfoKHX> for DeviceGroupSubmitInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupSubmitInfoKHX) -> DeviceGroupSubmitInfoKhx<'s> {
        DeviceGroupSubmitInfoKhx { raw: f, _p: PhantomData }
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

    pub fn wait_semaphore_device_indices<'m, 'a>(mut self, wait_semaphore_device_indices: &'a [u32]) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        assert!(self.raw.waitSemaphoreCount == 0 || self.raw.waitSemaphoreCount == wait_semaphore_device_indices.len() as _, 
            "count inconsistency found when specifying `DeviceGroupSubmitInfoKhx::wait_semaphore_device_indices`.");
        self.raw.waitSemaphoreCount = wait_semaphore_device_indices.len() as _;
        self.raw.pWaitSemaphoreDeviceIndices = wait_semaphore_device_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn command_buffer_device_masks<'m, 'a>(mut self, command_buffer_device_masks: &'a [u32]) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        assert!(self.raw.commandBufferCount == 0 || self.raw.commandBufferCount == command_buffer_device_masks.len() as _, 
            "count inconsistency found when specifying `DeviceGroupSubmitInfoKhx::command_buffer_device_masks`.");
        self.raw.commandBufferCount = command_buffer_device_masks.len() as _;
        self.raw.pCommandBufferDeviceMasks = command_buffer_device_masks.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn signal_semaphore_device_indices<'m, 'a>(mut self, signal_semaphore_device_indices: &'a [u32]) -> DeviceGroupSubmitInfoKhxBuilder<'b> {
        assert!(self.raw.signalSemaphoreCount == 0 || self.raw.signalSemaphoreCount == signal_semaphore_device_indices.len() as _, 
            "count inconsistency found when specifying `DeviceGroupSubmitInfoKhx::signal_semaphore_device_indices`.");
        self.raw.signalSemaphoreCount = signal_semaphore_device_indices.len() as _;
        self.raw.pSignalSemaphoreDeviceIndices = signal_semaphore_device_indices.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_wait_semaphore_device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pWaitSemaphoreDeviceIndices as *const _, self.raw.waitSemaphoreCount as usize) }
    }

    pub fn get_command_buffer_device_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pCommandBufferDeviceMasks as *const _, self.raw.commandBufferCount as usize) }
    }

    pub fn get_signal_semaphore_device_indices<'a>(&'a self) -> &'a [u32] {
        unsafe { slice::from_raw_parts(self.raw.pSignalSemaphoreDeviceIndices as *const _, self.raw.signalSemaphoreCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn resource_device_index<'a>(&'a self) -> u32 {
        self.raw.resourceDeviceIndex.into()
    }

    pub fn memory_device_index<'a>(&'a self) -> u32 {
        self.raw.memoryDeviceIndex.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_resource_device_index<'m>(&'s mut self, resource_device_index: u32) {
        self.raw.resourceDeviceIndex = resource_device_index.into();
    }

    pub fn set_memory_device_index<'m>(&'s mut self, memory_device_index: u32) {
        self.raw.memoryDeviceIndex = memory_device_index.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupBindSparseInfoKHX> for DeviceGroupBindSparseInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupBindSparseInfoKHX) -> DeviceGroupBindSparseInfoKhx<'s> {
        DeviceGroupBindSparseInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_resource_device_index<'a>(&'a self) -> u32 {
        self.raw.resourceDeviceIndex.into()
    }

    pub fn get_memory_device_index<'a>(&'a self) -> u32 {
        self.raw.memoryDeviceIndex.into()
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
    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn present_mask<'a>(&'a self) -> &[u32] {
        unsafe { slice::from_raw_parts(&self.raw.presentMask as *const _, vks::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize) }
    }

    pub fn modes<'a>(&'a self) -> DeviceGroupPresentModeFlagsKhx {
        DeviceGroupPresentModeFlagsKhx::from_bits(self.raw.modes)
            .expect("DeviceGroupPresentCapabilitiesKhx::modes: error converting flags")
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupPresentCapabilitiesKHX> for DeviceGroupPresentCapabilitiesKhx<'s> {
    fn from(f: vks::VkDeviceGroupPresentCapabilitiesKHX) -> DeviceGroupPresentCapabilitiesKhx<'s> {
        DeviceGroupPresentCapabilitiesKhx { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.swapchain
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_swapchain<'m, 'a>(&'s mut self, swapchain: &'a Swapchain) {
        self.raw.swapchain = swapchain.handle();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkImageSwapchainCreateInfoKHX> for ImageSwapchainCreateInfoKhx<'s> {
    fn from(f: vks::VkImageSwapchainCreateInfoKHX) -> ImageSwapchainCreateInfoKhx<'s> {
        ImageSwapchainCreateInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.swapchain
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.swapchain
    }

    pub fn image_index<'a>(&'a self) -> u32 {
        self.raw.imageIndex.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_swapchain<'m, 'a>(&'s mut self, swapchain: &'a Swapchain) {
        self.raw.swapchain = swapchain.handle();
    }

    pub fn set_image_index<'m>(&'s mut self, image_index: u32) {
        self.raw.imageIndex = image_index.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkBindImageMemorySwapchainInfoKHX> for BindImageMemorySwapchainInfoKhx<'s> {
    fn from(f: vks::VkBindImageMemorySwapchainInfoKHX) -> BindImageMemorySwapchainInfoKhx<'s> {
        BindImageMemorySwapchainInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.swapchain
    }

    pub fn get_image_index<'a>(&'a self) -> u32 {
        self.raw.imageIndex.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.swapchain
    }

    pub fn timeout<'a>(&'a self) -> u64 {
        self.raw.timeout.into()
    }

    pub fn semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_swapchain<'m, 'a>(&'s mut self, swapchain: &'a Swapchain) {
        self.raw.swapchain = swapchain.handle();
    }

    pub fn set_timeout<'m>(&'s mut self, timeout: u64) {
        self.raw.timeout = timeout.into();
    }

    pub fn set_semaphore<'m, 'a>(&'s mut self, semaphore: &'a Semaphore) {
        self.raw.semaphore = semaphore.handle();
    }

    pub fn set_fence<'m, 'a>(&'s mut self, fence: &'a Fence) {
        self.raw.fence = fence.handle();
    }

    pub fn set_device_mask<'m>(&'s mut self, device_mask: u32) {
        self.raw.deviceMask = device_mask.into();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkAcquireNextImageInfoKHX> for AcquireNextImageInfoKhx<'s> {
    fn from(f: vks::VkAcquireNextImageInfoKHX) -> AcquireNextImageInfoKhx<'s> {
        AcquireNextImageInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_swapchain_handle<'a>(&'a self) -> vks::VkSwapchainKHR {
        self.raw.swapchain
    }

    pub fn get_timeout<'a>(&'a self) -> u64 {
        self.raw.timeout.into()
    }

    pub fn get_semaphore_handle<'a>(&'a self) -> vks::VkSemaphore {
        self.raw.semaphore
    }

    pub fn get_fence_handle<'a>(&'a self) -> vks::VkFence {
        self.raw.fence
    }

    pub fn get_device_mask<'a>(&'a self) -> u32 {
        self.raw.deviceMask.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn swapchain_count<'a>(&'a self) -> u32 {
        self.raw.swapchainCount.into()
    }

    pub fn device_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { &*(self.raw.pDeviceMasks as *const _) }
    }

    pub fn mode<'a>(&'a self) -> DeviceGroupPresentModeFlagsKhx {
        DeviceGroupPresentModeFlagsKhx::from_bits(self.raw.mode)
            .expect("DeviceGroupPresentInfoKhx::mode: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_swapchain_count<'m>(&'s mut self, swapchain_count: u32) {
        self.raw.swapchainCount = swapchain_count.into();
    }

    pub fn set_device_masks<'m, 'a>(&'s mut self, device_masks: &'a [u32]) {
        self.raw.pDeviceMasks = device_masks.as_ptr() as *const u32 as *const _;
    }

    pub fn set_mode<'m>(&'s mut self, mode: DeviceGroupPresentModeFlagsKhx) {
        self.raw.mode = mode.bits();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupPresentInfoKHX> for DeviceGroupPresentInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupPresentInfoKHX) -> DeviceGroupPresentInfoKhx<'s> {
        DeviceGroupPresentInfoKhx { raw: f, _p: PhantomData }
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
        self.raw.pDeviceMasks = device_masks.as_ptr() as *const u32 as *const _;
        self
    }

    pub fn mode<'m>(mut self, mode: DeviceGroupPresentModeFlagsKhx) -> DeviceGroupPresentInfoKhxBuilder<'b> {
        self.raw.mode = mode.bits();
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_swapchain_count<'a>(&'a self) -> u32 {
        self.raw.swapchainCount.into()
    }

    pub fn get_device_masks<'a>(&'a self) -> &'a [u32] {
        unsafe { &*(self.raw.pDeviceMasks as *const _) }
    }

    pub fn get_mode<'a>(&'a self) -> DeviceGroupPresentModeFlagsKhx {
        DeviceGroupPresentModeFlagsKhx::from_bits(self.raw.mode)
            .expect("DeviceGroupPresentInfoKhx::mode: error converting flags")
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
    physical_devices: Option<SmallVec<[vks::VkPhysicalDevice; 8]>>,
    _p: PhantomData<&'s ()>,
}

#[cfg(feature = "experimental")]
impl<'s> DeviceGroupDeviceCreateInfoKhx<'s> {
    pub fn builder<'b>() -> DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
        DeviceGroupDeviceCreateInfoKhxBuilder::new()
    }

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn physical_devices_handle<'a>(&'a self) -> &'a [vks::VkPhysicalDevice] {
        unsafe { slice::from_raw_parts(self.raw.pPhysicalDevices as *const _, self.raw.physicalDeviceCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_physical_devices<'m, 'a>(&'s mut self, physical_devices: &'a [&'a PhysicalDevice])
            where 'a: 's {
        self.physical_devices = Some(physical_devices.iter().map(|h| h.handle()).collect());
        {
            let physical_devices = self.physical_devices.as_ref().unwrap();
            self.raw.pPhysicalDevices = physical_devices.as_ptr();
            assert!(self.raw.physicalDeviceCount == 0 || self.raw.physicalDeviceCount == physical_devices.len() as _, 
                "count inconsistency found when specifying `DeviceGroupDeviceCreateInfoKhx::physical_devices`.");
            self.raw.physicalDeviceCount = physical_devices.len() as _;
        }
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupDeviceCreateInfoKHX> for DeviceGroupDeviceCreateInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupDeviceCreateInfoKHX) -> DeviceGroupDeviceCreateInfoKhx<'s> {
        DeviceGroupDeviceCreateInfoKhx { raw: f, physical_devices: None, _p: PhantomData }
    }
}


/// A builder for `VkDeviceGroupDeviceCreateInfoKHX`.
///
/// 
#[cfg(feature = "experimental")]
#[derive(Debug, Clone, Default)]
pub struct DeviceGroupDeviceCreateInfoKhxBuilder<'b> {
    raw: vks::VkDeviceGroupDeviceCreateInfoKHX,
    physical_devices: Option<SmallVec<[vks::VkPhysicalDevice; 8]>>,
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

    pub fn physical_devices<'m, 'a>(mut self, physical_devices: &'a [&'a PhysicalDevice]) -> DeviceGroupDeviceCreateInfoKhxBuilder<'b>
            where 'a: 'b {
        self.physical_devices = Some(physical_devices.iter().map(|h| h.handle()).collect());
        {
            let physical_devices = self.physical_devices.as_ref().unwrap();
            self.raw.pPhysicalDevices = physical_devices.as_ptr();
            assert!(self.raw.physicalDeviceCount == 0 || self.raw.physicalDeviceCount == physical_devices.len() as _, 
                "count inconsistency found when specifying `DeviceGroupDeviceCreateInfoKhx::physical_devices`.");
            self.raw.physicalDeviceCount = physical_devices.len() as _;
        }
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_physical_devices_handle<'a>(&'a self) -> &'a [vks::VkPhysicalDevice] {
        unsafe { slice::from_raw_parts(self.raw.pPhysicalDevices as *const _, self.raw.physicalDeviceCount as usize) }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn modes<'a>(&'a self) -> DeviceGroupPresentModeFlagsKhx {
        DeviceGroupPresentModeFlagsKhx::from_bits(self.raw.modes)
            .expect("DeviceGroupSwapchainCreateInfoKhx::modes: error converting flags")
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_modes<'m>(&'s mut self, modes: DeviceGroupPresentModeFlagsKhx) {
        self.raw.modes = modes.bits();
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkDeviceGroupSwapchainCreateInfoKHX> for DeviceGroupSwapchainCreateInfoKhx<'s> {
    fn from(f: vks::VkDeviceGroupSwapchainCreateInfoKHX) -> DeviceGroupSwapchainCreateInfoKhx<'s> {
        DeviceGroupSwapchainCreateInfoKhx { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_modes<'a>(&'a self) -> DeviceGroupPresentModeFlagsKhx {
        DeviceGroupPresentModeFlagsKhx::from_bits(self.raw.modes)
            .expect("DeviceGroupSwapchainCreateInfoKhx::modes: error converting flags")
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

    pub fn dst_binding<'a>(&'a self) -> u32 {
        self.raw.dstBinding.into()
    }

    pub fn dst_array_element<'a>(&'a self) -> u32 {
        self.raw.dstArrayElement.into()
    }

    pub fn descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn descriptor_type<'a>(&'a self) -> DescriptorType {
        self.raw.descriptorType.into()
    }

    pub fn offset<'a>(&'a self) -> usize {
        self.raw.offset.into()
    }

    pub fn stride<'a>(&'a self) -> usize {
        self.raw.stride.into()
    }

    pub fn set_dst_binding<'m>(& mut self, dst_binding: u32) {
        self.raw.dstBinding = dst_binding.into();
    }

    pub fn set_dst_array_element<'m>(& mut self, dst_array_element: u32) {
        self.raw.dstArrayElement = dst_array_element.into();
    }

    pub fn set_descriptor_count<'m>(& mut self, descriptor_count: u32) {
        self.raw.descriptorCount = descriptor_count.into();
    }

    pub fn set_descriptor_type<'m>(& mut self, descriptor_type: DescriptorType) {
        self.raw.descriptorType = descriptor_type.into();
    }

    pub fn set_offset<'m>(& mut self, offset: usize) {
        self.raw.offset = offset.into();
    }

    pub fn set_stride<'m>(& mut self, stride: usize) {
        self.raw.stride = stride.into();
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


impl From<vks::VkDescriptorUpdateTemplateEntryKHR> for DescriptorUpdateTemplateEntryKhr {
    fn from(f: vks::VkDescriptorUpdateTemplateEntryKHR) -> DescriptorUpdateTemplateEntryKhr {
        DescriptorUpdateTemplateEntryKhr { raw: f, }
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

    pub fn get_dst_binding<'a>(&'a self) -> u32 {
        self.raw.dstBinding.into()
    }

    pub fn get_dst_array_element<'a>(&'a self) -> u32 {
        self.raw.dstArrayElement.into()
    }

    pub fn get_descriptor_count<'a>(&'a self) -> u32 {
        self.raw.descriptorCount.into()
    }

    pub fn get_descriptor_type<'a>(&'a self) -> DescriptorType {
        self.raw.descriptorType.into()
    }

    pub fn get_offset<'a>(&'a self) -> usize {
        self.raw.offset.into()
    }

    pub fn get_stride<'a>(&'a self) -> usize {
        self.raw.stride.into()
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> DescriptorUpdateTemplateCreateFlagsKhr {
        DescriptorUpdateTemplateCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("DescriptorUpdateTemplateCreateInfoKhr::flags: error converting flags")
    }

    pub fn descriptor_update_entries<'a>(&'a self) -> &'a [DescriptorUpdateTemplateEntryKhr] {
        unsafe { slice::from_raw_parts(self.raw.pDescriptorUpdateEntries as *const _, self.raw.descriptorUpdateEntryCount as usize) }
    }

    pub fn template_type<'a>(&'a self) -> DescriptorUpdateTemplateTypeKhr {
        self.raw.templateType.into()
    }

    pub fn descriptor_set_layout_handle<'a>(&'a self) -> vks::VkDescriptorSetLayout {
        self.raw.descriptorSetLayout
    }

    pub fn pipeline_bind_point<'a>(&'a self) -> PipelineBindPoint {
        self.raw.pipelineBindPoint.into()
    }

    pub fn pipeline_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.pipelineLayout
    }

    pub fn set<'a>(&'a self) -> u32 {
        self.raw.set.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: DescriptorUpdateTemplateCreateFlagsKhr) {
        self.raw.flags = flags.bits();
    }

    pub fn set_descriptor_update_entries<'m, 'a>(&'s mut self, descriptor_update_entries: &'a [DescriptorUpdateTemplateEntryKhr]) {
        assert!(self.raw.descriptorUpdateEntryCount == 0 || self.raw.descriptorUpdateEntryCount == descriptor_update_entries.len() as _, 
            "count inconsistency found when specifying `DescriptorUpdateTemplateCreateInfoKhr::descriptor_update_entries`.");
        self.raw.descriptorUpdateEntryCount = descriptor_update_entries.len() as _;
        self.raw.pDescriptorUpdateEntries = descriptor_update_entries.as_ptr() as *const vks::VkDescriptorUpdateTemplateEntryKHR as *const _;
    }

    pub fn set_template_type<'m>(&'s mut self, template_type: DescriptorUpdateTemplateTypeKhr) {
        self.raw.templateType = template_type.into();
    }

    pub fn set_descriptor_set_layout<'m, 'a>(&'s mut self, descriptor_set_layout: &'a DescriptorSetLayout) {
        self.raw.descriptorSetLayout = descriptor_set_layout.handle();
    }

    pub fn set_pipeline_bind_point<'m>(&'s mut self, pipeline_bind_point: PipelineBindPoint) {
        self.raw.pipelineBindPoint = pipeline_bind_point.into();
    }

    pub fn set_pipeline_layout<'m, 'a>(&'s mut self, pipeline_layout: &'a PipelineLayout) {
        self.raw.pipelineLayout = pipeline_layout.handle();
    }

    pub fn set_set<'m>(&'s mut self, set: u32) {
        self.raw.set = set.into();
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


impl<'s> From<vks::VkDescriptorUpdateTemplateCreateInfoKHR> for DescriptorUpdateTemplateCreateInfoKhr<'s> {
    fn from(f: vks::VkDescriptorUpdateTemplateCreateInfoKHR) -> DescriptorUpdateTemplateCreateInfoKhr<'s> {
        DescriptorUpdateTemplateCreateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn descriptor_update_entries<'m, 'a>(mut self, descriptor_update_entries: &'a [DescriptorUpdateTemplateEntryKhr]) -> DescriptorUpdateTemplateCreateInfoKhrBuilder<'b> {
        assert!(self.raw.descriptorUpdateEntryCount == 0 || self.raw.descriptorUpdateEntryCount == descriptor_update_entries.len() as _, 
            "count inconsistency found when specifying `DescriptorUpdateTemplateCreateInfoKhr::descriptor_update_entries`.");
        self.raw.descriptorUpdateEntryCount = descriptor_update_entries.len() as _;
        self.raw.pDescriptorUpdateEntries = descriptor_update_entries.as_ptr() as *const vks::VkDescriptorUpdateTemplateEntryKHR as *const _;
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> DescriptorUpdateTemplateCreateFlagsKhr {
        DescriptorUpdateTemplateCreateFlagsKhr::from_bits(self.raw.flags)
            .expect("DescriptorUpdateTemplateCreateInfoKhr::flags: error converting flags")
    }

    pub fn get_descriptor_update_entries<'a>(&'a self) -> &'a [DescriptorUpdateTemplateEntryKhr] {
        unsafe { slice::from_raw_parts(self.raw.pDescriptorUpdateEntries as *const _, self.raw.descriptorUpdateEntryCount as usize) }
    }

    pub fn get_template_type<'a>(&'a self) -> DescriptorUpdateTemplateTypeKhr {
        self.raw.templateType.into()
    }

    pub fn get_descriptor_set_layout_handle<'a>(&'a self) -> vks::VkDescriptorSetLayout {
        self.raw.descriptorSetLayout
    }

    pub fn get_pipeline_bind_point<'a>(&'a self) -> PipelineBindPoint {
        self.raw.pipelineBindPoint.into()
    }

    pub fn get_pipeline_layout_handle<'a>(&'a self) -> vks::VkPipelineLayout {
        self.raw.pipelineLayout
    }

    pub fn get_set<'a>(&'a self) -> u32 {
        self.raw.set.into()
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

    pub fn x<'a>(&'a self) -> f32 {
        self.raw.x.into()
    }

    pub fn y<'a>(&'a self) -> f32 {
        self.raw.y.into()
    }

    pub fn set_x<'m>(& mut self, x: f32) {
        self.raw.x = x.into();
    }

    pub fn set_y<'m>(& mut self, y: f32) {
        self.raw.y = y.into();
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


impl From<vks::VkXYColorEXT> for XYColorExt {
    fn from(f: vks::VkXYColorEXT) -> XYColorExt {
        XYColorExt { raw: f, }
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

    pub fn get_x<'a>(&'a self) -> f32 {
        self.raw.x.into()
    }

    pub fn get_y<'a>(&'a self) -> f32 {
        self.raw.y.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn display_primary_red<'a>(&'a self) -> XYColorExt {
        self.raw.displayPrimaryRed.into()
    }

    pub fn display_primary_green<'a>(&'a self) -> XYColorExt {
        self.raw.displayPrimaryGreen.into()
    }

    pub fn display_primary_blue<'a>(&'a self) -> XYColorExt {
        self.raw.displayPrimaryBlue.into()
    }

    pub fn white_point<'a>(&'a self) -> XYColorExt {
        self.raw.whitePoint.into()
    }

    pub fn max_luminance<'a>(&'a self) -> f32 {
        self.raw.maxLuminance.into()
    }

    pub fn min_luminance<'a>(&'a self) -> f32 {
        self.raw.minLuminance.into()
    }

    pub fn max_content_light_level<'a>(&'a self) -> f32 {
        self.raw.maxContentLightLevel.into()
    }

    pub fn max_frame_average_light_level<'a>(&'a self) -> f32 {
        self.raw.maxFrameAverageLightLevel.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_display_primary_red<'m>(&'s mut self, display_primary_red: XYColorExt) {
        self.raw.displayPrimaryRed = display_primary_red.raw;
    }

    pub fn set_display_primary_green<'m>(&'s mut self, display_primary_green: XYColorExt) {
        self.raw.displayPrimaryGreen = display_primary_green.raw;
    }

    pub fn set_display_primary_blue<'m>(&'s mut self, display_primary_blue: XYColorExt) {
        self.raw.displayPrimaryBlue = display_primary_blue.raw;
    }

    pub fn set_white_point<'m>(&'s mut self, white_point: XYColorExt) {
        self.raw.whitePoint = white_point.raw;
    }

    pub fn set_max_luminance<'m>(&'s mut self, max_luminance: f32) {
        self.raw.maxLuminance = max_luminance.into();
    }

    pub fn set_min_luminance<'m>(&'s mut self, min_luminance: f32) {
        self.raw.minLuminance = min_luminance.into();
    }

    pub fn set_max_content_light_level<'m>(&'s mut self, max_content_light_level: f32) {
        self.raw.maxContentLightLevel = max_content_light_level.into();
    }

    pub fn set_max_frame_average_light_level<'m>(&'s mut self, max_frame_average_light_level: f32) {
        self.raw.maxFrameAverageLightLevel = max_frame_average_light_level.into();
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


impl<'s> From<vks::VkHdrMetadataEXT> for HdrMetadataExt<'s> {
    fn from(f: vks::VkHdrMetadataEXT) -> HdrMetadataExt<'s> {
        HdrMetadataExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_display_primary_red<'a>(&'a self) -> XYColorExt {
        self.raw.displayPrimaryRed.into()
    }

    pub fn get_display_primary_green<'a>(&'a self) -> XYColorExt {
        self.raw.displayPrimaryGreen.into()
    }

    pub fn get_display_primary_blue<'a>(&'a self) -> XYColorExt {
        self.raw.displayPrimaryBlue.into()
    }

    pub fn get_white_point<'a>(&'a self) -> XYColorExt {
        self.raw.whitePoint.into()
    }

    pub fn get_max_luminance<'a>(&'a self) -> f32 {
        self.raw.maxLuminance.into()
    }

    pub fn get_min_luminance<'a>(&'a self) -> f32 {
        self.raw.minLuminance.into()
    }

    pub fn get_max_content_light_level<'a>(&'a self) -> f32 {
        self.raw.maxContentLightLevel.into()
    }

    pub fn get_max_frame_average_light_level<'a>(&'a self) -> f32 {
        self.raw.maxFrameAverageLightLevel.into()
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

    pub fn refresh_duration<'a>(&'a self) -> u64 {
        self.raw.refreshDuration.into()
    }

    pub fn set_refresh_duration<'m>(& mut self, refresh_duration: u64) {
        self.raw.refreshDuration = refresh_duration.into();
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


impl From<vks::VkRefreshCycleDurationGOOGLE> for RefreshCycleDurationGoogle {
    fn from(f: vks::VkRefreshCycleDurationGOOGLE) -> RefreshCycleDurationGoogle {
        RefreshCycleDurationGoogle { raw: f, }
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

    pub fn get_refresh_duration<'a>(&'a self) -> u64 {
        self.raw.refreshDuration.into()
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

    pub fn present_id<'a>(&'a self) -> u32 {
        self.raw.presentID.into()
    }

    pub fn desired_present_time<'a>(&'a self) -> u64 {
        self.raw.desiredPresentTime.into()
    }

    pub fn actual_present_time<'a>(&'a self) -> u64 {
        self.raw.actualPresentTime.into()
    }

    pub fn earliest_present_time<'a>(&'a self) -> u64 {
        self.raw.earliestPresentTime.into()
    }

    pub fn present_margin<'a>(&'a self) -> u64 {
        self.raw.presentMargin.into()
    }

    pub fn set_present_id<'m>(& mut self, present_id: u32) {
        self.raw.presentID = present_id.into();
    }

    pub fn set_desired_present_time<'m>(& mut self, desired_present_time: u64) {
        self.raw.desiredPresentTime = desired_present_time.into();
    }

    pub fn set_actual_present_time<'m>(& mut self, actual_present_time: u64) {
        self.raw.actualPresentTime = actual_present_time.into();
    }

    pub fn set_earliest_present_time<'m>(& mut self, earliest_present_time: u64) {
        self.raw.earliestPresentTime = earliest_present_time.into();
    }

    pub fn set_present_margin<'m>(& mut self, present_margin: u64) {
        self.raw.presentMargin = present_margin.into();
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


impl From<vks::VkPastPresentationTimingGOOGLE> for PastPresentationTimingGoogle {
    fn from(f: vks::VkPastPresentationTimingGOOGLE) -> PastPresentationTimingGoogle {
        PastPresentationTimingGoogle { raw: f, }
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

    pub fn get_present_id<'a>(&'a self) -> u32 {
        self.raw.presentID.into()
    }

    pub fn get_desired_present_time<'a>(&'a self) -> u64 {
        self.raw.desiredPresentTime.into()
    }

    pub fn get_actual_present_time<'a>(&'a self) -> u64 {
        self.raw.actualPresentTime.into()
    }

    pub fn get_earliest_present_time<'a>(&'a self) -> u64 {
        self.raw.earliestPresentTime.into()
    }

    pub fn get_present_margin<'a>(&'a self) -> u64 {
        self.raw.presentMargin.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn times<'a>(&'a self) -> &'a [PresentTimeGoogle] {
        unsafe { slice::from_raw_parts(self.raw.pTimes as *const _, self.raw.swapchainCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_times<'m, 'a>(&'s mut self, times: &'a [PresentTimeGoogle]) {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == times.len() as _, 
            "count inconsistency found when specifying `PresentTimesInfoGoogle::times`.");
        self.raw.swapchainCount = times.len() as _;
        self.raw.pTimes = times.as_ptr() as *const vks::VkPresentTimeGOOGLE as *const _;
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


impl<'s> From<vks::VkPresentTimesInfoGOOGLE> for PresentTimesInfoGoogle<'s> {
    fn from(f: vks::VkPresentTimesInfoGOOGLE) -> PresentTimesInfoGoogle<'s> {
        PresentTimesInfoGoogle { raw: f, _p: PhantomData }
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

    pub fn times<'m, 'a>(mut self, times: &'a [PresentTimeGoogle]) -> PresentTimesInfoGoogleBuilder<'b> {
        assert!(self.raw.swapchainCount == 0 || self.raw.swapchainCount == times.len() as _, 
            "count inconsistency found when specifying `PresentTimesInfoGoogle::times`.");
        self.raw.swapchainCount = times.len() as _;
        self.raw.pTimes = times.as_ptr() as *const vks::VkPresentTimeGOOGLE as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_times<'a>(&'a self) -> &'a [PresentTimeGoogle] {
        unsafe { slice::from_raw_parts(self.raw.pTimes as *const _, self.raw.swapchainCount as usize) }
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

    pub fn present_id<'a>(&'a self) -> u32 {
        self.raw.presentID.into()
    }

    pub fn desired_present_time<'a>(&'a self) -> u64 {
        self.raw.desiredPresentTime.into()
    }

    pub fn set_present_id<'m>(& mut self, present_id: u32) {
        self.raw.presentID = present_id.into();
    }

    pub fn set_desired_present_time<'m>(& mut self, desired_present_time: u64) {
        self.raw.desiredPresentTime = desired_present_time.into();
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


impl From<vks::VkPresentTimeGOOGLE> for PresentTimeGoogle {
    fn from(f: vks::VkPresentTimeGOOGLE) -> PresentTimeGoogle {
        PresentTimeGoogle { raw: f, }
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

    pub fn get_present_id<'a>(&'a self) -> u32 {
        self.raw.presentID.into()
    }

    pub fn get_desired_present_time<'a>(&'a self) -> u64 {
        self.raw.desiredPresentTime.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> IosSurfaceCreateFlagsMvk {
        IosSurfaceCreateFlagsMvk::from_bits(self.raw.flags)
            .expect("IosSurfaceCreateInfoMvk::flags: error converting flags")
    }

    pub fn view<'a>(&'a self) -> *const c_void {
        self.raw.pView
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: IosSurfaceCreateFlagsMvk) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_view<'m>(&'s mut self, view: *const c_void) {
        self.raw.pView = view;
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


impl<'s> From<vks::VkIOSSurfaceCreateInfoMVK> for IosSurfaceCreateInfoMvk<'s> {
    fn from(f: vks::VkIOSSurfaceCreateInfoMVK) -> IosSurfaceCreateInfoMvk<'s> {
        IosSurfaceCreateInfoMvk { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> IosSurfaceCreateFlagsMvk {
        IosSurfaceCreateFlagsMvk::from_bits(self.raw.flags)
            .expect("IosSurfaceCreateInfoMvk::flags: error converting flags")
    }

    pub fn get_view<'a>(&'a self) -> *const c_void {
        self.raw.pView
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> MacOsSurfaceCreateFlagsMvk {
        MacOsSurfaceCreateFlagsMvk::from_bits(self.raw.flags)
            .expect("MacOsSurfaceCreateInfoMvk::flags: error converting flags")
    }

    pub fn view<'a>(&'a self) -> *const c_void {
        self.raw.pView
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: MacOsSurfaceCreateFlagsMvk) {
        self.raw.flags = flags.bits();
    }

    pub unsafe fn set_view<'m>(&'s mut self, view: *const c_void) {
        self.raw.pView = view;
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


impl<'s> From<vks::VkMacOSSurfaceCreateInfoMVK> for MacOsSurfaceCreateInfoMvk<'s> {
    fn from(f: vks::VkMacOSSurfaceCreateInfoMVK) -> MacOsSurfaceCreateInfoMvk<'s> {
        MacOsSurfaceCreateInfoMvk { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> MacOsSurfaceCreateFlagsMvk {
        MacOsSurfaceCreateFlagsMvk::from_bits(self.raw.flags)
            .expect("MacOsSurfaceCreateInfoMvk::flags: error converting flags")
    }

    pub fn get_view<'a>(&'a self) -> *const c_void {
        self.raw.pView
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

    pub fn xcoeff<'a>(&'a self) -> f32 {
        self.raw.xcoeff.into()
    }

    pub fn ycoeff<'a>(&'a self) -> f32 {
        self.raw.ycoeff.into()
    }

    pub fn set_xcoeff<'m>(& mut self, xcoeff: f32) {
        self.raw.xcoeff = xcoeff.into();
    }

    pub fn set_ycoeff<'m>(& mut self, ycoeff: f32) {
        self.raw.ycoeff = ycoeff.into();
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


impl From<vks::VkViewportWScalingNV> for ViewportWScalingNv {
    fn from(f: vks::VkViewportWScalingNV) -> ViewportWScalingNv {
        ViewportWScalingNv { raw: f, }
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

    pub fn get_xcoeff<'a>(&'a self) -> f32 {
        self.raw.xcoeff.into()
    }

    pub fn get_ycoeff<'a>(&'a self) -> f32 {
        self.raw.ycoeff.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn viewport_wscaling_enable<'a>(&'a self) -> bool {
        self.raw.viewportWScalingEnable != 0
    }

    pub fn viewport_wscalings<'a>(&'a self) -> &'a [ViewportWScalingNv] {
        unsafe { slice::from_raw_parts(self.raw.pViewportWScalings as *const _, self.raw.viewportCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_viewport_wscaling_enable<'m>(&'s mut self, viewport_wscaling_enable: bool) {
        self.raw.viewportWScalingEnable = viewport_wscaling_enable as u32;
    }

    pub fn set_viewport_wscalings<'m, 'a>(&'s mut self, viewport_wscalings: &'a [ViewportWScalingNv]) {
        assert!(self.raw.viewportCount == 0 || self.raw.viewportCount == viewport_wscalings.len() as _, 
            "count inconsistency found when specifying `PipelineViewportWScalingStateCreateInfoNv::viewport_wscalings`.");
        self.raw.viewportCount = viewport_wscalings.len() as _;
        self.raw.pViewportWScalings = viewport_wscalings.as_ptr() as *const vks::VkViewportWScalingNV as *const _;
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


impl<'s> From<vks::VkPipelineViewportWScalingStateCreateInfoNV> for PipelineViewportWScalingStateCreateInfoNv<'s> {
    fn from(f: vks::VkPipelineViewportWScalingStateCreateInfoNV) -> PipelineViewportWScalingStateCreateInfoNv<'s> {
        PipelineViewportWScalingStateCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn viewport_wscalings<'m, 'a>(mut self, viewport_wscalings: &'a [ViewportWScalingNv]) -> PipelineViewportWScalingStateCreateInfoNvBuilder<'b> {
        assert!(self.raw.viewportCount == 0 || self.raw.viewportCount == viewport_wscalings.len() as _, 
            "count inconsistency found when specifying `PipelineViewportWScalingStateCreateInfoNv::viewport_wscalings`.");
        self.raw.viewportCount = viewport_wscalings.len() as _;
        self.raw.pViewportWScalings = viewport_wscalings.as_ptr() as *const vks::VkViewportWScalingNV as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_viewport_wscaling_enable<'a>(&'a self) -> bool {
        self.raw.viewportWScalingEnable != 0
    }

    pub fn get_viewport_wscalings<'a>(&'a self) -> &'a [ViewportWScalingNv] {
        unsafe { slice::from_raw_parts(self.raw.pViewportWScalings as *const _, self.raw.viewportCount as usize) }
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

    pub fn x<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.x.into()
    }

    pub fn y<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.y.into()
    }

    pub fn z<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.z.into()
    }

    pub fn w<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.w.into()
    }

    pub fn set_x<'m>(& mut self, x: ViewportCoordinateSwizzleNv) {
        self.raw.x = x.into();
    }

    pub fn set_y<'m>(& mut self, y: ViewportCoordinateSwizzleNv) {
        self.raw.y = y.into();
    }

    pub fn set_z<'m>(& mut self, z: ViewportCoordinateSwizzleNv) {
        self.raw.z = z.into();
    }

    pub fn set_w<'m>(& mut self, w: ViewportCoordinateSwizzleNv) {
        self.raw.w = w.into();
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


impl From<vks::VkViewportSwizzleNV> for ViewportSwizzleNv {
    fn from(f: vks::VkViewportSwizzleNV) -> ViewportSwizzleNv {
        ViewportSwizzleNv { raw: f, }
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

    pub fn get_x<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.x.into()
    }

    pub fn get_y<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.y.into()
    }

    pub fn get_z<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.z.into()
    }

    pub fn get_w<'a>(&'a self) -> ViewportCoordinateSwizzleNv {
        self.raw.w.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineViewportSwizzleStateCreateFlagsNv {
        PipelineViewportSwizzleStateCreateFlagsNv::from_bits(self.raw.flags)
            .expect("PipelineViewportSwizzleStateCreateInfoNv::flags: error converting flags")
    }

    pub fn viewport_swizzles<'a>(&'a self) -> &'a [ViewportSwizzleNv] {
        unsafe { slice::from_raw_parts(self.raw.pViewportSwizzles as *const _, self.raw.viewportCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineViewportSwizzleStateCreateFlagsNv) {
        self.raw.flags = flags.bits();
    }

    pub fn set_viewport_swizzles<'m, 'a>(&'s mut self, viewport_swizzles: &'a [ViewportSwizzleNv]) {
        assert!(self.raw.viewportCount == 0 || self.raw.viewportCount == viewport_swizzles.len() as _, 
            "count inconsistency found when specifying `PipelineViewportSwizzleStateCreateInfoNv::viewport_swizzles`.");
        self.raw.viewportCount = viewport_swizzles.len() as _;
        self.raw.pViewportSwizzles = viewport_swizzles.as_ptr() as *const vks::VkViewportSwizzleNV as *const _;
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


impl<'s> From<vks::VkPipelineViewportSwizzleStateCreateInfoNV> for PipelineViewportSwizzleStateCreateInfoNv<'s> {
    fn from(f: vks::VkPipelineViewportSwizzleStateCreateInfoNV) -> PipelineViewportSwizzleStateCreateInfoNv<'s> {
        PipelineViewportSwizzleStateCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn viewport_swizzles<'m, 'a>(mut self, viewport_swizzles: &'a [ViewportSwizzleNv]) -> PipelineViewportSwizzleStateCreateInfoNvBuilder<'b> {
        assert!(self.raw.viewportCount == 0 || self.raw.viewportCount == viewport_swizzles.len() as _, 
            "count inconsistency found when specifying `PipelineViewportSwizzleStateCreateInfoNv::viewport_swizzles`.");
        self.raw.viewportCount = viewport_swizzles.len() as _;
        self.raw.pViewportSwizzles = viewport_swizzles.as_ptr() as *const vks::VkViewportSwizzleNV as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineViewportSwizzleStateCreateFlagsNv {
        PipelineViewportSwizzleStateCreateFlagsNv::from_bits(self.raw.flags)
            .expect("PipelineViewportSwizzleStateCreateInfoNv::flags: error converting flags")
    }

    pub fn get_viewport_swizzles<'a>(&'a self) -> &'a [ViewportSwizzleNv] {
        unsafe { slice::from_raw_parts(self.raw.pViewportSwizzles as *const _, self.raw.viewportCount as usize) }
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn max_discard_rectangles<'a>(&'a self) -> u32 {
        self.raw.maxDiscardRectangles.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_max_discard_rectangles<'m>(&'s mut self, max_discard_rectangles: u32) {
        self.raw.maxDiscardRectangles = max_discard_rectangles.into();
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


impl<'s> From<vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT> for PhysicalDeviceDiscardRectanglePropertiesExt<'s> {
    fn from(f: vks::VkPhysicalDeviceDiscardRectanglePropertiesEXT) -> PhysicalDeviceDiscardRectanglePropertiesExt<'s> {
        PhysicalDeviceDiscardRectanglePropertiesExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_max_discard_rectangles<'a>(&'a self) -> u32 {
        self.raw.maxDiscardRectangles.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineDiscardRectangleStateCreateFlagsExt {
        PipelineDiscardRectangleStateCreateFlagsExt::from_bits(self.raw.flags)
            .expect("PipelineDiscardRectangleStateCreateInfoExt::flags: error converting flags")
    }

    pub fn discard_rectangle_mode<'a>(&'a self) -> DiscardRectangleModeExt {
        self.raw.discardRectangleMode.into()
    }

    pub fn discard_rectangles<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pDiscardRectangles as *const _, self.raw.discardRectangleCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineDiscardRectangleStateCreateFlagsExt) {
        self.raw.flags = flags.bits();
    }

    pub fn set_discard_rectangle_mode<'m>(&'s mut self, discard_rectangle_mode: DiscardRectangleModeExt) {
        self.raw.discardRectangleMode = discard_rectangle_mode.into();
    }

    pub fn set_discard_rectangles<'m, 'a>(&'s mut self, discard_rectangles: &'a [Rect2d]) {
        assert!(self.raw.discardRectangleCount == 0 || self.raw.discardRectangleCount == discard_rectangles.len() as _, 
            "count inconsistency found when specifying `PipelineDiscardRectangleStateCreateInfoExt::discard_rectangles`.");
        self.raw.discardRectangleCount = discard_rectangles.len() as _;
        self.raw.pDiscardRectangles = discard_rectangles.as_ptr() as *const vks::VkRect2D as *const _;
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


impl<'s> From<vks::VkPipelineDiscardRectangleStateCreateInfoEXT> for PipelineDiscardRectangleStateCreateInfoExt<'s> {
    fn from(f: vks::VkPipelineDiscardRectangleStateCreateInfoEXT) -> PipelineDiscardRectangleStateCreateInfoExt<'s> {
        PipelineDiscardRectangleStateCreateInfoExt { raw: f, _p: PhantomData }
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

    pub fn discard_rectangles<'m, 'a>(mut self, discard_rectangles: &'a [Rect2d]) -> PipelineDiscardRectangleStateCreateInfoExtBuilder<'b> {
        assert!(self.raw.discardRectangleCount == 0 || self.raw.discardRectangleCount == discard_rectangles.len() as _, 
            "count inconsistency found when specifying `PipelineDiscardRectangleStateCreateInfoExt::discard_rectangles`.");
        self.raw.discardRectangleCount = discard_rectangles.len() as _;
        self.raw.pDiscardRectangles = discard_rectangles.as_ptr() as *const vks::VkRect2D as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineDiscardRectangleStateCreateFlagsExt {
        PipelineDiscardRectangleStateCreateFlagsExt::from_bits(self.raw.flags)
            .expect("PipelineDiscardRectangleStateCreateInfoExt::flags: error converting flags")
    }

    pub fn get_discard_rectangle_mode<'a>(&'a self) -> DiscardRectangleModeExt {
        self.raw.discardRectangleMode.into()
    }

    pub fn get_discard_rectangles<'a>(&'a self) -> &'a [Rect2d] {
        unsafe { slice::from_raw_parts(self.raw.pDiscardRectangles as *const _, self.raw.discardRectangleCount as usize) }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn per_view_position_all_components<'a>(&'a self) -> bool {
        self.raw.perViewPositionAllComponents != 0
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


#[cfg(feature = "experimental")]
impl<'s> From<vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX> for PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx<'s> {
    fn from(f: vks::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX) -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx<'s> {
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn surface_handle<'a>(&'a self) -> vks::VkSurfaceKHR {
        self.raw.surface
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_surface<'m, 'a>(&'s mut self, surface: &'a Surface) {
        self.raw.surface = surface.handle();
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


impl<'s> From<vks::VkPhysicalDeviceSurfaceInfo2KHR> for PhysicalDeviceSurfaceInfo2Khr<'s> {
    fn from(f: vks::VkPhysicalDeviceSurfaceInfo2KHR) -> PhysicalDeviceSurfaceInfo2Khr<'s> {
        PhysicalDeviceSurfaceInfo2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_surface_handle<'a>(&'a self) -> vks::VkSurfaceKHR {
        self.raw.surface
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn surface_capabilities<'a>(&'a self) -> SurfaceCapabilitiesKhr {
        self.raw.surfaceCapabilities.into()
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


impl<'s> From<vks::VkSurfaceCapabilities2KHR> for SurfaceCapabilities2Khr<'s> {
    fn from(f: vks::VkSurfaceCapabilities2KHR) -> SurfaceCapabilities2Khr<'s> {
        SurfaceCapabilities2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn surface_format<'a>(&'a self) -> SurfaceFormatKhr {
        self.raw.surfaceFormat.into()
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


impl<'s> From<vks::VkSurfaceFormat2KHR> for SurfaceFormat2Khr<'s> {
    fn from(f: vks::VkSurfaceFormat2KHR) -> SurfaceFormat2Khr<'s> {
        SurfaceFormat2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn shared_present_supported_usage_flags<'a>(&'a self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits(self.raw.sharedPresentSupportedUsageFlags)
            .expect("SharedPresentSurfaceCapabilitiesKhr::shared_present_supported_usage_flags: error converting flags")
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


impl<'s> From<vks::VkSharedPresentSurfaceCapabilitiesKHR> for SharedPresentSurfaceCapabilitiesKhr<'s> {
    fn from(f: vks::VkSharedPresentSurfaceCapabilitiesKHR) -> SharedPresentSurfaceCapabilitiesKhr<'s> {
        SharedPresentSurfaceCapabilitiesKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn storage_buffer_16_bit_access<'a>(&'a self) -> bool {
        self.raw.storageBuffer16BitAccess != 0
    }

    pub fn uniform_and_storage_buffer_16_bit_access<'a>(&'a self) -> bool {
        self.raw.uniformAndStorageBuffer16BitAccess != 0
    }

    pub fn storage_push_constant_16<'a>(&'a self) -> bool {
        self.raw.storagePushConstant16 != 0
    }

    pub fn storage_input_output_16<'a>(&'a self) -> bool {
        self.raw.storageInputOutput16 != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_storage_buffer_16_bit_access<'m>(&'s mut self, storage_buffer_16_bit_access: bool) {
        self.raw.storageBuffer16BitAccess = storage_buffer_16_bit_access as u32;
    }

    pub fn set_uniform_and_storage_buffer_16_bit_access<'m>(&'s mut self, uniform_and_storage_buffer_16_bit_access: bool) {
        self.raw.uniformAndStorageBuffer16BitAccess = uniform_and_storage_buffer_16_bit_access as u32;
    }

    pub fn set_storage_push_constant_16<'m>(&'s mut self, storage_push_constant_16: bool) {
        self.raw.storagePushConstant16 = storage_push_constant_16 as u32;
    }

    pub fn set_storage_input_output_16<'m>(&'s mut self, storage_input_output_16: bool) {
        self.raw.storageInputOutput16 = storage_input_output_16 as u32;
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


impl<'s> From<vks::VkPhysicalDevice16BitStorageFeaturesKHR> for PhysicalDevice16BitStorageFeaturesKhr<'s> {
    fn from(f: vks::VkPhysicalDevice16BitStorageFeaturesKHR) -> PhysicalDevice16BitStorageFeaturesKhr<'s> {
        PhysicalDevice16BitStorageFeaturesKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_storage_buffer_16_bit_access<'a>(&'a self) -> bool {
        self.raw.storageBuffer16BitAccess != 0
    }

    pub fn get_uniform_and_storage_buffer_16_bit_access<'a>(&'a self) -> bool {
        self.raw.uniformAndStorageBuffer16BitAccess != 0
    }

    pub fn get_storage_push_constant_16<'a>(&'a self) -> bool {
        self.raw.storagePushConstant16 != 0
    }

    pub fn get_storage_input_output_16<'a>(&'a self) -> bool {
        self.raw.storageInputOutput16 != 0
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_buffer<'m, 'a>(&'s mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
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


impl<'s> From<vks::VkBufferMemoryRequirementsInfo2KHR> for BufferMemoryRequirementsInfo2Khr<'s> {
    fn from(f: vks::VkBufferMemoryRequirementsInfo2KHR) -> BufferMemoryRequirementsInfo2Khr<'s> {
        BufferMemoryRequirementsInfo2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
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


impl<'s> From<vks::VkImageMemoryRequirementsInfo2KHR> for ImageMemoryRequirementsInfo2Khr<'s> {
    fn from(f: vks::VkImageMemoryRequirementsInfo2KHR) -> ImageMemoryRequirementsInfo2Khr<'s> {
        ImageMemoryRequirementsInfo2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
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


impl<'s> From<vks::VkImageSparseMemoryRequirementsInfo2KHR> for ImageSparseMemoryRequirementsInfo2Khr<'s> {
    fn from(f: vks::VkImageSparseMemoryRequirementsInfo2KHR) -> ImageSparseMemoryRequirementsInfo2Khr<'s> {
        ImageSparseMemoryRequirementsInfo2Khr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn memory_requirements<'a>(&'a self) -> MemoryRequirements {
        self.raw.memoryRequirements.into()
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


impl<'s> From<vks::VkMemoryRequirements2KHR> for MemoryRequirements2Khr<'s> {
    fn from(f: vks::VkMemoryRequirements2KHR) -> MemoryRequirements2Khr<'s> {
        MemoryRequirements2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn memory_requirements<'a>(&'a self) -> SparseImageMemoryRequirements {
        self.raw.memoryRequirements.into()
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


impl<'s> From<vks::VkSparseImageMemoryRequirements2KHR> for SparseImageMemoryRequirements2Khr<'s> {
    fn from(f: vks::VkSparseImageMemoryRequirements2KHR) -> SparseImageMemoryRequirements2Khr<'s> {
        SparseImageMemoryRequirements2Khr { raw: f, _p: PhantomData }
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn prefers_dedicated_allocation<'a>(&'a self) -> bool {
        self.raw.prefersDedicatedAllocation != 0
    }

    pub fn requires_dedicated_allocation<'a>(&'a self) -> bool {
        self.raw.requiresDedicatedAllocation != 0
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


impl<'s> From<vks::VkMemoryDedicatedRequirementsKHR> for MemoryDedicatedRequirementsKhr<'s> {
    fn from(f: vks::VkMemoryDedicatedRequirementsKHR) -> MemoryDedicatedRequirementsKhr<'s> {
        MemoryDedicatedRequirementsKhr { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_image<'m, 'a>(&'s mut self, image: &'a Image) {
        self.raw.image = image.handle();
    }

    pub fn set_buffer<'m, 'a>(&'s mut self, buffer: &'a Buffer) {
        self.raw.buffer = buffer.handle();
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


impl<'s> From<vks::VkMemoryDedicatedAllocateInfoKHR> for MemoryDedicatedAllocateInfoKhr<'s> {
    fn from(f: vks::VkMemoryDedicatedAllocateInfoKHR) -> MemoryDedicatedAllocateInfoKhr<'s> {
        MemoryDedicatedAllocateInfoKhr { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_image_handle<'a>(&'a self) -> vks::VkImage {
        self.raw.image
    }

    pub fn get_buffer_handle<'a>(&'a self) -> vks::VkBuffer {
        self.raw.buffer
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn supports_texture_gather_lo_dbias_am_d<'a>(&'a self) -> bool {
        self.raw.supportsTextureGatherLODBiasAMD != 0
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


impl<'s> From<vks::VkTextureLODGatherFormatPropertiesAMD> for TextureLODGatherFormatPropertiesAmd<'s> {
    fn from(f: vks::VkTextureLODGatherFormatPropertiesAMD) -> TextureLODGatherFormatPropertiesAmd<'s> {
        TextureLODGatherFormatPropertiesAmd { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineCoverageToColorStateCreateFlagsNv {
        PipelineCoverageToColorStateCreateFlagsNv::from_bits(self.raw.flags)
            .expect("PipelineCoverageToColorStateCreateInfoNv::flags: error converting flags")
    }

    pub fn coverage_to_color_enable<'a>(&'a self) -> bool {
        self.raw.coverageToColorEnable != 0
    }

    pub fn coverage_to_color_location<'a>(&'a self) -> u32 {
        self.raw.coverageToColorLocation.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineCoverageToColorStateCreateFlagsNv) {
        self.raw.flags = flags.bits();
    }

    pub fn set_coverage_to_color_enable<'m>(&'s mut self, coverage_to_color_enable: bool) {
        self.raw.coverageToColorEnable = coverage_to_color_enable as u32;
    }

    pub fn set_coverage_to_color_location<'m>(&'s mut self, coverage_to_color_location: u32) {
        self.raw.coverageToColorLocation = coverage_to_color_location.into();
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


impl<'s> From<vks::VkPipelineCoverageToColorStateCreateInfoNV> for PipelineCoverageToColorStateCreateInfoNv<'s> {
    fn from(f: vks::VkPipelineCoverageToColorStateCreateInfoNV) -> PipelineCoverageToColorStateCreateInfoNv<'s> {
        PipelineCoverageToColorStateCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineCoverageToColorStateCreateFlagsNv {
        PipelineCoverageToColorStateCreateFlagsNv::from_bits(self.raw.flags)
            .expect("PipelineCoverageToColorStateCreateInfoNv::flags: error converting flags")
    }

    pub fn get_coverage_to_color_enable<'a>(&'a self) -> bool {
        self.raw.coverageToColorEnable != 0
    }

    pub fn get_coverage_to_color_location<'a>(&'a self) -> u32 {
        self.raw.coverageToColorLocation.into()
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn filter_minmax_single_component_formats<'a>(&'a self) -> bool {
        self.raw.filterMinmaxSingleComponentFormats != 0
    }

    pub fn filter_minmax_image_component_mapping<'a>(&'a self) -> bool {
        self.raw.filterMinmaxImageComponentMapping != 0
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


impl<'s> From<vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT> for PhysicalDeviceSamplerFilterMinmaxPropertiesExt<'s> {
    fn from(f: vks::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT) -> PhysicalDeviceSamplerFilterMinmaxPropertiesExt<'s> {
        PhysicalDeviceSamplerFilterMinmaxPropertiesExt { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn reduction_mode<'a>(&'a self) -> SamplerReductionModeExt {
        self.raw.reductionMode.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_reduction_mode<'m>(&'s mut self, reduction_mode: SamplerReductionModeExt) {
        self.raw.reductionMode = reduction_mode.into();
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


impl<'s> From<vks::VkSamplerReductionModeCreateInfoEXT> for SamplerReductionModeCreateInfoExt<'s> {
    fn from(f: vks::VkSamplerReductionModeCreateInfoEXT) -> SamplerReductionModeCreateInfoExt<'s> {
        SamplerReductionModeCreateInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_reduction_mode<'a>(&'a self) -> SamplerReductionModeExt {
        self.raw.reductionMode.into()
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

    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn advanced_blend_coherent_operations<'a>(&'a self) -> bool {
        self.raw.advancedBlendCoherentOperations != 0
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *mut c_void) {
        self.raw.pNext = next;
    }

    pub fn set_advanced_blend_coherent_operations<'m>(&'s mut self, advanced_blend_coherent_operations: bool) {
        self.raw.advancedBlendCoherentOperations = advanced_blend_coherent_operations as u32;
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


impl<'s> From<vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT> for PhysicalDeviceBlendOperationAdvancedFeaturesExt<'s> {
    fn from(f: vks::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT) -> PhysicalDeviceBlendOperationAdvancedFeaturesExt<'s> {
        PhysicalDeviceBlendOperationAdvancedFeaturesExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn get_advanced_blend_coherent_operations<'a>(&'a self) -> bool {
        self.raw.advancedBlendCoherentOperations != 0
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
    pub fn next<'a>(&'a self) -> *mut c_void {
        self.raw.pNext
    }

    pub fn advanced_blend_max_color_attachments<'a>(&'a self) -> u32 {
        self.raw.advancedBlendMaxColorAttachments.into()
    }

    pub fn advanced_blend_independent_blend<'a>(&'a self) -> bool {
        self.raw.advancedBlendIndependentBlend != 0
    }

    pub fn advanced_blend_non_premultiplied_src_color<'a>(&'a self) -> bool {
        self.raw.advancedBlendNonPremultipliedSrcColor != 0
    }

    pub fn advanced_blend_non_premultiplied_dst_color<'a>(&'a self) -> bool {
        self.raw.advancedBlendNonPremultipliedDstColor != 0
    }

    pub fn advanced_blend_correlated_overlap<'a>(&'a self) -> bool {
        self.raw.advancedBlendCorrelatedOverlap != 0
    }

    pub fn advanced_blend_all_operations<'a>(&'a self) -> bool {
        self.raw.advancedBlendAllOperations != 0
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


impl<'s> From<vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT> for PhysicalDeviceBlendOperationAdvancedPropertiesExt<'s> {
    fn from(f: vks::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT) -> PhysicalDeviceBlendOperationAdvancedPropertiesExt<'s> {
        PhysicalDeviceBlendOperationAdvancedPropertiesExt { raw: f, _p: PhantomData }
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn src_premultiplied<'a>(&'a self) -> bool {
        self.raw.srcPremultiplied != 0
    }

    pub fn dst_premultiplied<'a>(&'a self) -> bool {
        self.raw.dstPremultiplied != 0
    }

    pub fn blend_overlap<'a>(&'a self) -> BlendOverlapExt {
        self.raw.blendOverlap.into()
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_src_premultiplied<'m>(&'s mut self, src_premultiplied: bool) {
        self.raw.srcPremultiplied = src_premultiplied as u32;
    }

    pub fn set_dst_premultiplied<'m>(&'s mut self, dst_premultiplied: bool) {
        self.raw.dstPremultiplied = dst_premultiplied as u32;
    }

    pub fn set_blend_overlap<'m>(&'s mut self, blend_overlap: BlendOverlapExt) {
        self.raw.blendOverlap = blend_overlap.into();
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


impl<'s> From<vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT> for PipelineColorBlendAdvancedStateCreateInfoExt<'s> {
    fn from(f: vks::VkPipelineColorBlendAdvancedStateCreateInfoEXT) -> PipelineColorBlendAdvancedStateCreateInfoExt<'s> {
        PipelineColorBlendAdvancedStateCreateInfoExt { raw: f, _p: PhantomData }
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

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_src_premultiplied<'a>(&'a self) -> bool {
        self.raw.srcPremultiplied != 0
    }

    pub fn get_dst_premultiplied<'a>(&'a self) -> bool {
        self.raw.dstPremultiplied != 0
    }

    pub fn get_blend_overlap<'a>(&'a self) -> BlendOverlapExt {
        self.raw.blendOverlap.into()
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

    pub fn next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn flags<'a>(&'a self) -> PipelineCoverageModulationStateCreateFlagsNv {
        PipelineCoverageModulationStateCreateFlagsNv::from_bits(self.raw.flags)
            .expect("PipelineCoverageModulationStateCreateInfoNv::flags: error converting flags")
    }

    pub fn coverage_modulation_mode<'a>(&'a self) -> CoverageModulationModeNv {
        self.raw.coverageModulationMode.into()
    }

    pub fn coverage_modulation_table_enable<'a>(&'a self) -> bool {
        self.raw.coverageModulationTableEnable != 0
    }

    pub fn coverage_modulation_table<'a>(&'a self) -> &'a [f32] {
        unsafe { slice::from_raw_parts(self.raw.pCoverageModulationTable as *const _, self.raw.coverageModulationTableCount as usize) }
    }

    pub unsafe fn set_next<'m>(&'s mut self, next: *const c_void) {
        self.raw.pNext = next;
    }

    pub fn set_flags<'m>(&'s mut self, flags: PipelineCoverageModulationStateCreateFlagsNv) {
        self.raw.flags = flags.bits();
    }

    pub fn set_coverage_modulation_mode<'m>(&'s mut self, coverage_modulation_mode: CoverageModulationModeNv) {
        self.raw.coverageModulationMode = coverage_modulation_mode.into();
    }

    pub fn set_coverage_modulation_table_enable<'m>(&'s mut self, coverage_modulation_table_enable: bool) {
        self.raw.coverageModulationTableEnable = coverage_modulation_table_enable as u32;
    }

    pub fn set_coverage_modulation_table<'m, 'a>(&'s mut self, coverage_modulation_table: &'a [f32]) {
        assert!(self.raw.coverageModulationTableCount == 0 || self.raw.coverageModulationTableCount == coverage_modulation_table.len() as _, 
            "count inconsistency found when specifying `PipelineCoverageModulationStateCreateInfoNv::coverage_modulation_table`.");
        self.raw.coverageModulationTableCount = coverage_modulation_table.len() as _;
        self.raw.pCoverageModulationTable = coverage_modulation_table.as_ptr() as *const f32 as *const _;
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


impl<'s> From<vks::VkPipelineCoverageModulationStateCreateInfoNV> for PipelineCoverageModulationStateCreateInfoNv<'s> {
    fn from(f: vks::VkPipelineCoverageModulationStateCreateInfoNV) -> PipelineCoverageModulationStateCreateInfoNv<'s> {
        PipelineCoverageModulationStateCreateInfoNv { raw: f, _p: PhantomData }
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

    pub fn coverage_modulation_table<'m, 'a>(mut self, coverage_modulation_table: &'a [f32]) -> PipelineCoverageModulationStateCreateInfoNvBuilder<'b> {
        assert!(self.raw.coverageModulationTableCount == 0 || self.raw.coverageModulationTableCount == coverage_modulation_table.len() as _, 
            "count inconsistency found when specifying `PipelineCoverageModulationStateCreateInfoNv::coverage_modulation_table`.");
        self.raw.coverageModulationTableCount = coverage_modulation_table.len() as _;
        self.raw.pCoverageModulationTable = coverage_modulation_table.as_ptr() as *const f32 as *const _;
        self
    }

    pub fn get_next<'a>(&'a self) -> *const c_void {
        self.raw.pNext
    }

    pub fn get_flags<'a>(&'a self) -> PipelineCoverageModulationStateCreateFlagsNv {
        PipelineCoverageModulationStateCreateFlagsNv::from_bits(self.raw.flags)
            .expect("PipelineCoverageModulationStateCreateInfoNv::flags: error converting flags")
    }

    pub fn get_coverage_modulation_mode<'a>(&'a self) -> CoverageModulationModeNv {
        self.raw.coverageModulationMode.into()
    }

    pub fn get_coverage_modulation_table_enable<'a>(&'a self) -> bool {
        self.raw.coverageModulationTableEnable != 0
    }

    pub fn get_coverage_modulation_table<'a>(&'a self) -> &'a [f32] {
        unsafe { slice::from_raw_parts(self.raw.pCoverageModulationTable as *const _, self.raw.coverageModulationTableCount as usize) }
    }

    pub fn build(self) -> PipelineCoverageModulationStateCreateInfoNv<'b> {
        PipelineCoverageModulationStateCreateInfoNv {
            raw: self.raw,
            _p: PhantomData,
        }
    }

}


