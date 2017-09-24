//! vkc - Vulkan Compute

#![allow(unused_extern_crates, unused_imports, dead_code, unused_variables)]

extern crate libloading as lib;
extern crate smallvec;
// extern crate nalgebra;
// extern crate cgmath;
extern crate vks as vks_;
extern crate libc;
extern crate tobj;
extern crate ordered_float;
pub extern crate winit;

mod error;
mod version;
mod loader;
pub mod instance; // temporarily public
mod swapchain;
mod image_view;
mod pipeline_layout;
mod shader_module;
mod render_pass;
mod graphics_pipeline;
mod framebuffer;
mod surface;
mod queue;
mod command_pool;
mod command_buffers;
mod semaphore;
mod buffer;
mod image;
mod sampler;
mod device_memory;
mod descriptor_set_layout;
mod descriptor_pool;
mod structs;

pub mod vks {
    pub use vks_::*;
    pub use vks_::core::*;
    pub use vks_::amd_rasterization_order::*;
    pub use vks_::ext_debug_marker::*;
    pub use vks_::ext_debug_report::*;
    pub use vks_::ext_validation_flags::*;
    pub use vks_::khr_android_surface::*;
    pub use vks_::khr_display::*;
    pub use vks_::khr_display_swapchain::*;
    pub use vks_::khr_get_physical_device_properties2::*;
    pub use vks_::khr_mir_surface::*;
    pub use vks_::khr_surface::*;
    pub use vks_::khr_swapchain::*;
    pub use vks_::khr_wayland_surface::*;
    pub use vks_::khr_win32_surface::*;
    pub use vks_::khr_xcb_surface::*;
    pub use vks_::khr_xlib_surface::*;
    pub use vks_::nv_dedicated_allocation::*;
    pub use vks_::nv_external_memory::*;
    pub use vks_::nv_external_memory_capabilities::*;
    pub use vks_::nv_external_memory_win32::*;
    pub use vks_::nv_win32_keyed_mutex::*;
}

// pub mod vulkan_h;
pub mod device;
pub mod util;

use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
use libc::c_void;
use std::mem;
use std::ptr;
use winit::{EventsLoop, WindowBuilder, Window, CreationError, ControlFlow, Event, WindowEvent};
use ordered_float::OrderedFloat;
use error::Result as VooResult;
pub use loader::Loader;
pub use error::{Error, Result};
pub use version::Version;
pub use instance::Instance;
pub use device::Device;
pub use surface::Surface;
pub use queue::{queue_families, Queue};
pub use swapchain::{Swapchain, SwapchainSupportDetails};
pub use image_view::{create_image_views, ImageView};
pub use shader_module::ShaderModule;
pub use pipeline_layout::PipelineLayout;
pub use render_pass::RenderPass;
pub use graphics_pipeline::GraphicsPipeline;
pub use framebuffer::{create_framebuffers, Framebuffer};
pub use command_pool::CommandPool;
pub use command_buffers::create_command_buffers;
pub use semaphore::Semaphore;
pub use buffer::Buffer;
pub use image::Image;
pub use sampler::Sampler;
pub use device_memory::DeviceMemory;
pub use descriptor_set_layout::DescriptorSetLayout;
pub use descriptor_pool::DescriptorPool;
pub use structs::*;



#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;


#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize } as u32
    }
}



pub fn find_memory_type(device: &Device, type_filter: u32, properties: vks::VkMemoryPropertyFlags)
        -> u32 {
    let mut mem_properties: vks::VkPhysicalDeviceMemoryProperties;
    unsafe {
        mem_properties = mem::uninitialized();
        device.instance().vk().core.vkGetPhysicalDeviceMemoryProperties(device.physical_device(),
            &mut mem_properties);
    }

    for i in 0..mem_properties.memoryTypeCount {
        if (type_filter & (1 << i)) != 0 &&
            (mem_properties.memoryTypes[i as usize].propertyFlags & properties) == properties
        {
            return i;
        }
    }
    panic!("Failed to find suitable memory type.");
}



#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl Vertex {
    pub fn binding_description() -> vks::VkVertexInputBindingDescription {
        vks::VkVertexInputBindingDescription {
            binding: 0,
            stride: mem::size_of::<Vertex>() as u32,
            // * VERTEX_INPUT_RATE_VERTEX: Move to the next data entry
            //   after each vertex
            // * VERTEX_INPUT_RATE_INSTANCE: Move to the next data entry
            //   after each instance
            inputRate: vks::VK_VERTEX_INPUT_RATE_VERTEX,
        }
    }

    pub fn attribute_descriptions() -> [vks::VkVertexInputAttributeDescription; 3] {
        [
            vks::VkVertexInputAttributeDescription {
                binding: 0,
                location: 0,
                format: vks::VK_FORMAT_R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, pos),
            },
            vks::VkVertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vks::VK_FORMAT_R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, color),
            },
            vks::VkVertexInputAttributeDescription {
                binding: 0,
                location: 2,
                format: vks::VK_FORMAT_R32G32_SFLOAT,
                offset: offset_of!(Vertex, tex_coord),
            },
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



#[derive(Debug)]
#[repr(C)]
pub struct UniformBufferObject {
    pub model: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
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
//     pub fn new() -> VooResult<AbstractTemplate> {

//         let mut handle = 0;
//         unsafe {
//             ::check(device.vk().vkCreateAbstractTemplate(device.handle(), &create_info,
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

//     pub fn device(&self) -> &Device {
//         &self.inner.device
//     }
// }

// impl Drop for Inner {
//     fn drop(&mut self) {
//         unsafe {
//             self.device.vk().vkDestroyAbstractTemplate(self.device.handle(), self.handle, ptr::null());
//         }
//     }
// }
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////



pub fn check(code: i32) {
    if code != vks::VK_SUCCESS { panic!("Error code: {}", code); }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
