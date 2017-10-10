//! vkc - Vulkan Compute

#![allow(unused_extern_crates, /*unused_imports,*/ dead_code, unused_variables)]

extern crate libloading as lib;
extern crate smallvec;
// extern crate nalgebra;
// extern crate cgmath;
extern crate vks as vks_;
extern crate libc;
extern crate tobj;
extern crate ordered_float;
#[macro_use]
extern crate bitflags as bitflags_;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;
pub extern crate winit;

mod error;
mod version;
mod loader;
pub mod instance; // temporarily public
mod physical_device;
mod swapchain;
mod image_view;
mod pipeline_layout;
mod shader_module;
mod render_pass;
mod graphics_pipeline;
mod framebuffer;
mod surface;
pub mod queue;
mod command_pool;
mod command_buffer;
mod semaphore;
mod buffer;
mod image;
mod sampler;
mod device_memory;
mod descriptor_set_layout;
mod descriptor_pool;
mod structs;
mod enums;
mod bitflags;

pub mod vks {
    pub use vks_::*;
    pub use vks_::core::*;
    pub use vks_::amd_draw_indirect_count::*;
    pub use vks_::amd_gcn_shader::*;
    pub use vks_::amd_gpu_shader_half_float::*;
    pub use vks_::amd_gpu_shader_int16::*;
    pub use vks_::amd_mixed_attachment_samples::*;
    pub use vks_::amd_negative_viewport_height::*;
    pub use vks_::amd_rasterization_order::*;
    pub use vks_::amd_shader_ballot::*;
    pub use vks_::amd_shader_explicit_vertex_parameter::*;
    pub use vks_::amd_shader_trinary_minmax::*;
    pub use vks_::amd_texture_gather_bias_lod::*;
    pub use vks_::ext_acquire_xlib_display::*;
    pub use vks_::ext_blend_operation_advanced::*;
    pub use vks_::ext_debug_marker::*;
    pub use vks_::ext_debug_report::*;
    pub use vks_::ext_depth_range_unrestricted::*;
    pub use vks_::ext_direct_mode_display::*;
    pub use vks_::ext_discard_rectangles::*;
    pub use vks_::ext_display_control::*;
    pub use vks_::ext_display_surface_counter::*;
    pub use vks_::ext_hdr_metadata::*;
    pub use vks_::ext_post_depth_coverage::*;
    pub use vks_::ext_sampler_filter_minmax::*;
    pub use vks_::ext_shader_stencil_export::*;
    pub use vks_::ext_shader_subgroup_ballot::*;
    pub use vks_::ext_shader_subgroup_vote::*;
    pub use vks_::ext_shader_viewport_index_layer::*;
    pub use vks_::ext_swapchain_colorspace::*;
    pub use vks_::ext_validation_flags::*;
    pub use vks_::google_display_timing::*;
    pub use vks_::img_filter_cubic::*;
    pub use vks_::img_format_pvrtc::*;
    pub use vks_::khr_16bit_storage::*;
    pub use vks_::khr_android_surface::*;
    pub use vks_::khr_dedicated_allocation::*;
    pub use vks_::khr_descriptor_update_template::*;
    pub use vks_::khr_display::*;
    pub use vks_::khr_display_swapchain::*;
    pub use vks_::khr_external_fence::*;
    pub use vks_::khr_external_fence_capabilities::*;
    pub use vks_::khr_external_fence_fd::*;
    pub use vks_::khr_external_fence_win32::*;
    pub use vks_::khr_external_memory::*;
    pub use vks_::khr_external_memory_capabilities::*;
    pub use vks_::khr_external_memory_fd::*;
    pub use vks_::khr_external_memory_win32::*;
    pub use vks_::khr_external_semaphore::*;
    pub use vks_::khr_external_semaphore_capabilities::*;
    pub use vks_::khr_external_semaphore_fd::*;
    pub use vks_::khr_external_semaphore_win32::*;
    pub use vks_::khr_get_memory_requirements2::*;
    pub use vks_::khr_get_physical_device_properties2::*;
    pub use vks_::khr_get_surface_capabilities2::*;
    pub use vks_::khr_incremental_present::*;
    pub use vks_::khr_maintenance1::*;
    pub use vks_::khr_mir_surface::*;
    pub use vks_::khr_push_descriptor::*;
    pub use vks_::khr_relaxed_block_layout::*;
    pub use vks_::khr_sampler_mirror_clamp_to_edge::*;
    pub use vks_::khr_shader_draw_parameters::*;
    pub use vks_::khr_shared_presentable_image::*;
    pub use vks_::khr_storage_buffer_storage_class::*;
    pub use vks_::khr_surface::*;
    pub use vks_::khr_swapchain::*;
    pub use vks_::khr_variable_pointers::*;
    pub use vks_::khr_wayland_surface::*;
    pub use vks_::khr_win32_keyed_mutex::*;
    pub use vks_::khr_win32_surface::*;
    pub use vks_::khr_xcb_surface::*;
    pub use vks_::khr_xlib_surface::*;
    pub use vks_::mvk_ios_surface::*;
    pub use vks_::mvk_macos_surface::*;
    pub use vks_::nn_vi_surface::*;
    pub use vks_::nv_clip_space_w_scaling::*;
    pub use vks_::nv_dedicated_allocation::*;
    pub use vks_::nv_external_memory::*;
    pub use vks_::nv_external_memory_capabilities::*;
    pub use vks_::nv_external_memory_win32::*;
    pub use vks_::nv_fill_rectangle::*;
    pub use vks_::nv_fragment_coverage_to_color::*;
    pub use vks_::nv_framebuffer_mixed_samples::*;
    pub use vks_::nv_geometry_shader_passthrough::*;
    pub use vks_::nv_glsl_shader::*;
    pub use vks_::nv_sample_mask_override_coverage::*;
    pub use vks_::nv_viewport_array2::*;
    pub use vks_::nv_viewport_swizzle::*;
    pub use vks_::nv_win32_keyed_mutex::*;

    pub use vks_::android_types::*;
    pub use vks_::mir_types::*;
    pub use vks_::wayland_types::*;
    pub use vks_::win32_types::*;
    pub use vks_::xcb_types::*;
    pub use vks_::xlib_types::*;

    #[cfg(feature = "experimental")]
    pub use vks_::experimental::*;
}

// pub mod vulkan_h;
pub mod device;
pub mod util;
pub mod voodoo_winit;

// use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
// use libc::c_void;
use std::mem;
// use std::ptr;
use ordered_float::OrderedFloat;
use error::Result as VooResult;
pub use util::{CharStr, CharStrs};
pub use loader::Loader;
pub use error::{Error, Result};
pub use version::Version;
pub use instance::{InstanceHandle, Instance, InstanceBuilder};
pub use physical_device::{PhysicalDeviceHandle, PhysicalDevice};
pub use device::{DeviceHandle, Device, DeviceBuilder};
pub use surface::{SurfaceKhrHandle, SurfaceKhr, SurfaceKhrBuilder};
pub use queue::{queue_families, QueueHandle, Queue};
pub use swapchain::{SwapchainKhrHandle, SwapchainKhr, SwapchainKhrBuilder, SwapchainSupportDetails};
pub use image_view::{ImageViewHandle, ImageView, ImageViewBuilder};
pub use shader_module::{ShaderModuleHandle, ShaderModule};
pub use pipeline_layout::{PipelineLayoutHandle, PipelineLayout, PipelineLayoutBuilder};
pub use render_pass::{RenderPassHandle, RenderPass, RenderPassBuilder};
pub use graphics_pipeline::{GraphicsPipeline, GraphicsPipelineBuilder};
pub use framebuffer::{FramebufferHandle, Framebuffer, FramebufferBuilder};
pub use command_pool::{CommandPoolHandle, CommandPool, CommandPoolBuilder};
pub use command_buffer::{CommandBufferHandle, CommandBuffer};
pub use semaphore::{SemaphoreHandle, Semaphore};
pub use buffer::{BufferHandle, Buffer, BufferBuilder};
pub use image::{ImageHandle, Image, ImageBuilder};
pub use sampler::{SamplerHandle, Sampler, SamplerBuilder};
pub use device_memory::{DeviceMemoryHandle, DeviceMemory, DeviceMemoryBuilder};
pub use descriptor_set_layout::{DescriptorSetLayoutHandle, DescriptorSetLayout,
    DescriptorSetLayoutBuilder};
pub use descriptor_pool::{DescriptorPoolHandle, DescriptorPool, DescriptorPoolBuilder};
pub use structs::*;
pub use enums::*;
pub use bitflags::*;


pub trait Handle {
    type Target;

    fn handle(&self) -> Self::Target;
}

pub trait AnyPipelineHandle: Handle<Target=PipelineHandle> {}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct FenceHandle(pub(crate) vks::VkFence);

impl FenceHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkFence {
        self.0
    }
}

impl Handle for FenceHandle {
    type Target = FenceHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EventHandle(pub(crate) vks::VkEvent);

impl EventHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkEvent {
        self.0
    }
}

impl Handle for EventHandle {
    type Target = EventHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct QueryPoolHandle(pub(crate) vks::VkQueryPool);

impl QueryPoolHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkQueryPool {
        self.0
    }
}

impl Handle for QueryPoolHandle {
    type Target = QueryPoolHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct BufferViewHandle(pub(crate) vks::VkBufferView);

impl BufferViewHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkBufferView {
        self.0
    }
}

impl Handle for BufferViewHandle {
    type Target = BufferViewHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PipelineCacheHandle(pub(crate) vks::VkPipelineCache);

impl PipelineCacheHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkPipelineCache {
        self.0
    }
}

impl Handle for PipelineCacheHandle {
    type Target = PipelineCacheHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PipelineHandle(pub(crate) vks::VkPipeline);

impl PipelineHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkPipeline {
        self.0
    }
}

impl Handle for PipelineHandle {
    type Target = PipelineHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DescriptorSetHandle(pub(crate) vks::VkDescriptorSet);

impl DescriptorSetHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDescriptorSet {
        self.0
    }
}

impl Handle for DescriptorSetHandle {
    type Target = DescriptorSetHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DisplayKhrHandle(pub(crate) vks::VkDisplayKHR);

impl DisplayKhrHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDisplayKHR {
        self.0
    }
}

impl Handle for DisplayKhrHandle {
    type Target = DisplayKhrHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DisplayModeKhrHandle(pub(crate) vks::VkDisplayModeKHR);

impl DisplayModeKhrHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDisplayModeKHR {
        self.0
    }
}

impl Handle for DisplayModeKhrHandle {
    type Target = DisplayModeKhrHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DescriptorUpdateTemplateHandle(pub(crate) vks::VkDescriptorUpdateTemplateKHR);

impl DescriptorUpdateTemplateHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDescriptorUpdateTemplateKHR {
        self.0
    }
}

impl Handle for DescriptorUpdateTemplateHandle {
    type Target = DescriptorUpdateTemplateHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DebugReportCallbackExtHandle(pub(crate) vks::VkDebugReportCallbackEXT);

impl DebugReportCallbackExtHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkDebugReportCallbackEXT {
        self.0
    }
}

impl Handle for DebugReportCallbackExtHandle {
    type Target = DebugReportCallbackExtHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SamplerYcbcrConversionKhrHandle(pub(crate) u64);

impl SamplerYcbcrConversionKhrHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> u64 {
        self.0
    }
}

impl Handle for SamplerYcbcrConversionKhrHandle {
    type Target = SamplerYcbcrConversionKhrHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ObjectTableNvxHandle(pub(crate) u64);

impl ObjectTableNvxHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> u64 {
        self.0
    }
}

impl Handle for ObjectTableNvxHandle {
    type Target = ObjectTableNvxHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct IndirectCommandsLayoutNvxHandle(pub(crate) u64);

impl IndirectCommandsLayoutNvxHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> u64 {
        self.0
    }
}

impl Handle for IndirectCommandsLayoutNvxHandle {
    type Target = IndirectCommandsLayoutNvxHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ValidationCacheExtHandle(pub(crate) u64);

impl ValidationCacheExtHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> u64 {
        self.0
    }
}

impl Handle for ValidationCacheExtHandle {
    type Target = ValidationCacheExtHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DescriptorSet(DescriptorSetHandle);

impl DescriptorSet {
    pub fn handle(&self) -> DescriptorSetHandle {
        self.0
    }
}

impl Handle for DescriptorSet {
    type Target = DescriptorSetHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}

impl<'h> Handle for &'h DescriptorSet {
    type Target = DescriptorSetHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct BufferView(BufferViewHandle);

impl BufferView {
    pub fn handle(&self) -> BufferViewHandle {
        self.0
    }
}

impl<'h> Handle for &'h BufferView {
    type Target = BufferViewHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct Pipeline(PipelineHandle);

impl Pipeline {
    pub fn handle(&self) -> PipelineHandle {
        self.0
    }
}

impl<'h> Handle for &'h Pipeline {
    type Target = PipelineHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct QueryPool(QueryPoolHandle);

impl QueryPool {
    pub fn handle(&self) -> QueryPoolHandle {
        self.0
    }
}

impl<'h> Handle for &'h QueryPool {
    type Target = QueryPoolHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct Fence(FenceHandle);

impl Fence {
    pub fn handle(&self) -> FenceHandle {
        self.0
    }
}

impl<'h> Handle for &'h Fence {
    type Target = FenceHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct Event(EventHandle);

impl Event {
    pub fn handle(&self) -> EventHandle {
        self.0
    }
}

impl<'h> Handle for &'h Event {
    type Target = EventHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct DisplayModeKhr(DisplayModeKhrHandle);

impl DisplayModeKhr {
    pub fn handle(&self) -> DisplayModeKhrHandle {
        self.0
    }
}

impl<'h> Handle for &'h DisplayModeKhr {
    type Target = DisplayModeKhrHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


#[derive(Clone, Debug)]
pub struct DisplayKhr(DisplayKhrHandle);

impl DisplayKhr {
    pub fn handle(&self) -> DisplayKhrHandle {
        self.0
    }
}

impl<'h> Handle for &'h DisplayKhr {
    type Target = DisplayKhrHandle;

    fn handle(&self) -> Self::Target {
        self.0
    }
}


// typedef union VkClearColorValue {
//     float       float32[4];
//     int32_t     int32[4];
//     uint32_t    uint32[4];
// } VkClearColorValue;
// pub enum ClearColorValue {
//     Float([f32; 4]),
//     I32([i32; 4]),
//     U32([u32; 4]),
// }

pub type ClearColorValue = vks::VkClearColorValue;

// pub type SurfaceKhr = vks::VkSurfaceKHR;
// pub type SwapchainKhr = vks::VkSwapchainKHR;
pub type DeviceSize = vks::VkDeviceSize;
pub type Display = vks::Display;
pub type MirConnection = vks::MirConnection;
pub type MirSurface = vks::MirSurface;
pub type ANativeWindow = vks::ANativeWindow;
#[allow(non_camel_case_types)]
pub type wl_display = vks::wl_display;
#[allow(non_camel_case_types)]
pub type wl_surface = vks::wl_surface;
#[allow(non_camel_case_types)]
pub type HINSTANCE = vks::HINSTANCE;
#[allow(non_camel_case_types)]
pub type HWND = vks::HWND;
#[allow(non_camel_case_types)]
pub type xcb_connection_t = vks::xcb_connection_t;
#[allow(non_camel_case_types)]
pub type xcb_window_t = vks::xcb_window_t;
#[allow(non_camel_case_types)]
pub type HANDLE = vks::HANDLE;
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = vks::SECURITY_ATTRIBUTES;
#[allow(non_camel_case_types)]
pub type DWORD = vks::DWORD;
#[allow(non_camel_case_types)]
pub type LPCWSTR = vks::LPCWSTR;


// TODO: MAKE THESE UNIONS ENUMS:
pub type ClearValue = vks::VkClearValue;


pub const LOD_CLAMP_NONE: f32 = 1000.0f32;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32= !0;
pub const WHOLE_SIZE: u64 = !0;
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const UUID_SIZE: usize = 16;
pub const MAX_MEMORY_TYPES: usize = 32;
pub const MAX_MEMORY_HEAPS: usize = 16;
pub const MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const MAX_DESCRIPTION_SIZE: usize = 256;


//////////////////////////////////////

const PRINT: bool = false;

pub static VALIDATION_LAYER_NAMES: &[&[u8]] = &[
    b"VK_LAYER_LUNARG_standard_validation\0"
];


#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize } as u32
    }
}


// TODO: MOVE TO `PhysicalDevice`.
// pub fn memory_type_index(device: &Device, type_filter: u32, properties: vks::VkMemoryPropertyFlags)
//         -> u32 {
//     let mut mem_props = device.physical_device().memory_properties();

//     for i in 0..mem_props.memoryTypeCount {
//         if (type_filter & (1 << i)) != 0 &&
//             (mem_props.memoryTypes[i as usize].propertyFlags & properties) == properties
//         {
//             return i;
//         }
//     }
//     panic!("Failed to find suitable memory type.");
// }



#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl Vertex {
    pub fn binding_description() -> VertexInputBindingDescription {
        VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Vertex>() as u32)
            // * VERTEX_INPUT_RATE_VERTEX: Move to the next data entry
            //   after each vertex
            // * VERTEX_INPUT_RATE_INSTANCE: Move to the next data entry
            //   after each instance
            // .input_rate(vks::VK_VERTEX_INPUT_RATE_VERTEX)
            .input_rate(VertexInputRate::Vertex)
            .build()
    }

    pub fn attribute_descriptions() -> [VertexInputAttributeDescription; 3] {
        [
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                // .format(vks::VK_FORMAT_R32G32B32_SFLOAT)
                .format(Format::R32G32B32Sfloat)
                .offset(offset_of!(Vertex, pos))
                .build(),
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                // .format(vks::VK_FORMAT_R32G32B32_SFLOAT)
                .format(Format::R32G32B32Sfloat)
                .offset(offset_of!(Vertex, color))
                .build(),
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                // .format(vks::VK_FORMAT_R32G32_SFLOAT)
                .format(Format::R32G32Sfloat)
                .offset(offset_of!(Vertex, tex_coord))
                .build(),
        ]
    }
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pos = [OrderedFloat(self.pos[0]), OrderedFloat(self.pos[1]),
            OrderedFloat(self.pos[2])];
        let color = [OrderedFloat(self.color[0]), OrderedFloat(self.color[1]),
            OrderedFloat(self.color[2])];
        let tex_coord = [OrderedFloat(self.tex_coord[0]), OrderedFloat(self.tex_coord[1])];
        pos.hash(state);
        color.hash(state);
        tex_coord.hash(state);
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.pos == other.pos && self.color == other.color && self.tex_coord == other.tex_coord
    }
}

impl Eq for Vertex {}



#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UniformBufferObject {
    pub model: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
}


pub fn check(code: i32) {
    if code != vks::VK_SUCCESS { panic!("VkResult error code: {}", code); }
}


///////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////// TEMPLATE /////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

// use std::sync::Arc;
// use std::ptr;
// use vks;
// use ::{util, VooResult, Device};

// #[derive(Debug)]
// struct Inner {
//     handle: vks::VkAbstractTemplate,
//     device: Device,
// }

// #[derive(Debug, Clone)]
// pub struct AbstractTemplate {
//     inner: Arc<Inner>,
// }

// impl AbstractTemplate {
//     /// Returns a new `AbstractTemplateBuilder`.
//     pub fn builder<'b>() -> AbstractTemplateBuilder<'b> {
//         AbstractTemplateBuilder::new()
//     }

//     pub fn new() -> VooResult<AbstractTemplate> {
//         let mut handle = 0;
//         unsafe {
//             ::check(device.proc_addr_loader().vkCreateAbstractTemplate(device.handle().0, &create_info,
//                 ptr::null(), &mut handle));
//         }

//         Ok(AbstractTemplate {
//             inner: Arc::new(Inner {
//                 handle,
//                 device,
//             })
//         })
//     }

//     pub fn handle(&self) -> vks::VkAbstractTemplate {
//         self.inner.handle
//     }

    /// Returns a reference to the associated device.
//     pub fn device(&self) -> &Device {
//         &self.inner.device
//     }
// }

// impl Drop for Inner {
//     fn drop(&mut self) {
//         unsafe {
//             self.device.proc_addr_loader().vkDestroyAbstractTemplate(self.device.handle().0, self.handle, ptr::null());
//         }
//     }
// }

///////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// BUILDER TEMPLATE ///////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

// use std::marker::PhantomData;

    // /// Returns a new `AbstractTemplateBuilder`.
    // pub fn builder<'b>() -> AbstractTemplateBuilder<'b> {
    //     AbstractTemplateBuilder::new()
    // }

// /// A builder for `AbstractTemplate`.
// #[derive(Debug, Clone)]
// pub struct AbstractTemplateBuilder<'b> {
//     create_info: vks::VkAbstractTemplateCreateInfo,
//     _p: PhantomData<&'b ()>,
// }

// impl<'b> AbstractTemplateBuilder<'b> {
//     /// Returns a new render pass builder.
//     pub fn new() -> AbstractTemplateBuilder<'b> {
//         AbstractTemplateBuilder {
//             create_info: vks::VkAbstractTemplateCreateInfo::default(),
//             _p: PhantomData,
//         }
//     }

//     pub fn fffffffffffff<'s>(&'s mut self, fffffffffffff: vks::VkAbstractTemplateCreateFlags)
//             -> &'s mut AbstractTemplateBuilder<'b> {
//         self.create_info.fffffffffffff = fffffffffffff;
//         self
//     }

//     pub fn sssssssssssss<'s, 'p>(&'s mut self, sssssssssssss: &'p [DeviceQueueCreateInfo])
//             -> &'s mut AbstractTemplateBuilder<'b>
//             where 'p: 'b {
//         self.create_info.SeeSsssee = sssssssssssss;
//         self
//     }

//     pub fn pppppppppppppp<'s, 'p>(&'s mut self,
//             pppppppppppppp: &'p [vks::VkAttachmentDescription])
//             -> &'s mut AbstractTemplateBuilder<'b>
//             where 'p: 'b {
//         self.create_info.ppppppppppppppCount = pppppppppppppp.len() as u32;
//         self.create_info.pApppppppp = pppppppppppppp.as_ptr();
//         self
//     }

//     /// Creates and returns a new `AbstractTemplate`
//     pub fn build(&self, device: Device) -> VooResult<AbstractTemplate> {
//         let mut handle = 0;
//         unsafe {
//             ::check(device.proc_addr_loader().core.vkCreateAbstractTemplate(device.handle().0,
//                 &self.create_info, ptr::null(), &mut handle));
//         }

//         Ok(AbstractTemplate {
//             inner: Arc::new(Inner {
//                 handle,
//                 device,
//             })
//         })
//     }
// }

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}


// camelCase to snake_case:
// ([a-z])([A-Z]+)
// --->
// $1_\l$2