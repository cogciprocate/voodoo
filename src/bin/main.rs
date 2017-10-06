#![allow(unused_imports, dead_code, unused_variables)]

extern crate voodoo as voo;
extern crate cgmath;
extern crate image;
extern crate smallvec;
extern crate libc;
extern crate tobj;

use std::mem;
use std::ptr;
use std::time;
use std::path::Path;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::cmp;
use libc::c_char;
use smallvec::SmallVec;
use image::{ImageFormat, DynamicImage};
use cgmath::{SquareMatrix, One, Rotation, Rotation3, Basis3, Matrix3, Matrix4, Vector3};
use voo::winit::{EventsLoop, WindowBuilder, Window, Event, WindowEvent};
use voo::{voodoo_winit, vks, util, device, queue, Result as VooResult, Version, Instance, Device,
    SurfaceKhr, SwapchainKhr,
    ImageView, PipelineLayout, RenderPass, GraphicsPipeline, Framebuffer, CommandPool, Semaphore,
    Buffer, DeviceMemory, Vertex, DescriptorSetLayout, UniformBufferObject, DescriptorPool,
    Image, Sampler, Loader, SwapchainSupportDetails, PhysicalDevice, PhysicalDeviceFeatures,
    ShaderModule, QueueFlags, Format};

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

static REQUIRED_INSTANCE_EXTENSIONS: &[&[u8]] = &[
    b"VK_KHR_surface\0",
    b"VK_KHR_win32_surface\0",
];

static REQUIRED_DEVICE_EXTENSIONS: &[&[u8]] = &[
    b"VK_KHR_swapchain\0",
];

static MODEL_PATH: &str = "/src/shared_assets/models/chalet.obj";
// static TEXTURE_PATH: &str = "/src/shared_assets/textures/chalet.jpg";
static TEXTURE_PATH: &str = "/src/shared_assets/textures/texture.jpg";

const VERTICES: [Vertex; 8] =  [
    Vertex { pos: [-0.5, -0.5, 0.25], color: [1.0, 0.0, 0.0], tex_coord: [1.0, 0.0]},
    Vertex { pos: [0.5, -0.5, 0.25], color: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.25], color: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.25], color: [1.0, 1.0, 1.0], tex_coord: [1.0, 1.0] },
    Vertex { pos: [-0.5, -0.5, -0.25], color: [1.0, 0.0, 0.0], tex_coord: [1.0, 0.0]},
    Vertex { pos: [0.5, -0.5, -0.25], color: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, -0.25], color: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, -0.25], color: [1.0, 1.0, 1.0], tex_coord: [1.0, 1.0] },
];

const INDICES: [u32; 12] = [
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4
];


/// Initializes the window and event loop.
fn init_window() -> (Window, EventsLoop) {
    let events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Voodoo - Hello Triangle")
        .build(&events_loop).unwrap();
    (window, events_loop)
}

/// Returns the list of layer names to be enabled.
fn enabled_layer_names<'ln>(loader: &Loader)
        -> SmallVec<[&'ln CStr; 16]> {
    if ENABLE_VALIDATION_LAYERS && !loader.check_validation_layer_support() {
        panic!("Unable to enable validation layers.");
    }
    if ENABLE_VALIDATION_LAYERS {
         (loader.validation_layer_names()).iter().map(|lyr_name|
            unsafe { CStr::from_ptr(lyr_name.as_ptr() as *const c_char) }).collect()
    } else {
        SmallVec::new()
    }
}

/// Initializes a loader and returns a new instance.
fn init_instance() -> VooResult<Instance> {
    let app_name = CString::new("Hello Triangle").unwrap();
    let eng_name = CString::new("None").unwrap();

    let app_info = voo::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version((1, 0, 0))
        .engine_name(&eng_name)
        .engine_version((1, 0, 0))
        .api_version((1, 0, 0))
        .build();

    let loader = Loader::new()?;

    Instance::builder()
        .application_info(&app_info)
        .enabled_layer_names(enabled_layer_names(&loader).as_slice())
        .enabled_extensions(loader.instance_extensions().as_slice())
        .build(loader, ENABLE_VALIDATION_LAYERS)
}

/// Returns true if the specified physical device has the required features,
/// extensions, queue families and if the supported swap chain has the correct
/// presentation modes.
fn device_is_suitable(instance: &Instance, surface: &SurfaceKhr,
        physical_device: &PhysicalDevice, queue_family_flags: QueueFlags) -> bool {
    let device_features = physical_device.features();

    let reqd_exts: SmallVec<[_; 16]> = (&REQUIRED_DEVICE_EXTENSIONS[..]).iter().map(|ext_name| {
        CStr::from_bytes_with_nul(ext_name).expect("invalid required extension name")
    }).collect();

    let extensions_supported = physical_device.verify_extensions_support(&reqd_exts[..]);

    let mut swap_chain_adequate = false;
    if extensions_supported {
        let swap_chain_details = SwapchainSupportDetails::new(instance, surface,
            &physical_device);
        swap_chain_adequate = !swap_chain_details.formats.is_empty() &&
            !swap_chain_details.present_modes.is_empty()
    }

    queue::queue_families(instance, surface, &physical_device, queue_family_flags).is_complete() &&
        extensions_supported &&
        swap_chain_adequate &&
        device_features.sampler_anisotropy()
}

/// Returns a physical device from the list of available physical devices if
/// it meets the criteria specified in the above function.
fn choose_physical_device(instance: &Instance, surface: &SurfaceKhr,
        queue_family_flags: QueueFlags) -> VooResult<PhysicalDevice> {
    let mut preferred_device = None;
    for device in instance.physical_devices() {
        if device_is_suitable(instance, surface, &device, queue_family_flags) {
            preferred_device = Some(device);
            break;
        }
    }
    if let Some(preferred_device) = preferred_device {
        Ok(preferred_device)
    } else {
        panic!("Failed to find a suitable device.");
    }
}

fn create_device(instance: Instance, surface: &SurfaceKhr, physical_device: PhysicalDevice,
        queue_familiy_flags: QueueFlags) -> VooResult<Device> {
    let queue_family_idx = queue::queue_families(&instance, surface,
        &physical_device, queue_familiy_flags).family_idxs()[0] as u32;

    let queue_create_info = voo::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_idx)
        .queue_priorities(&[1.0])
        .build();

    let features = PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true)
        .build();

    Device::builder()
        .queue_create_infos(&[queue_create_info.clone()])
        .enabled_extension_names(REQUIRED_DEVICE_EXTENSIONS)
        .enabled_features(&features)
        .build(physical_device)
}

fn choose_swap_surface_format(available_formats: &[voo::SurfaceFormatKhr])
        -> voo::SurfaceFormatKhr {
    if available_formats.len() == 1 && available_formats[0].format() == voo::Format::Undefined {
        return voo::SurfaceFormatKhr::builder()
            .format(voo::Format::B8G8R8A8Unorm)
            .color_space(voo::ColorSpaceKhr::SrgbNonlinearKhr)
            .build();
    }
    for available_format in available_formats {
        if available_format.format() == Format::B8G8R8A8Unorm &&
                available_format.color_space() == voo::ColorSpaceKhr::SrgbNonlinearKhr {
            return voo::SurfaceFormatKhr::builder()
                .format(voo::Format::B8G8R8A8Unorm)
                .color_space(voo::ColorSpaceKhr::SrgbNonlinearKhr)
                .build();
        }
    }
    voo::SurfaceFormatKhr::builder()
        .format(available_formats[0].format())
        .color_space(available_formats[0].color_space())
        .build()
}

fn choose_swap_present_mode(available_present_modes: &[voo::PresentModeKhr])
        -> voo::PresentModeKhr {
    let mut best_mode = voo::PresentModeKhr::MailboxKhr;
    for &available_present_mode in available_present_modes {
        if available_present_mode == voo::PresentModeKhr::FifoKhr {
            return available_present_mode;
        } else if available_present_mode == voo::PresentModeKhr::ImmediateKhr {
            best_mode = available_present_mode;
        }
    }
    best_mode
}

fn choose_swap_extent(capabilities: &voo::SurfaceCapabilitiesKhr,
        window_size: Option<voo::Extent2d>) -> voo::Extent2d {
    if capabilities.current_extent().width() != u32::max_value() {
        return capabilities.current_extent().clone();
    } else {

        let mut actual_extent = window_size
            .unwrap_or(voo::Extent2d::builder().width(1024).height(768).build());
        let actual_extent_width = actual_extent.width();
        let actual_extent_height = actual_extent.height();
        actual_extent.set_width(cmp::max(capabilities.min_image_extent().width(),
            cmp::min(capabilities.max_image_extent().width(), actual_extent_width)));
        actual_extent.set_height(cmp::max(capabilities.min_image_extent().height(),
            cmp::min(capabilities.max_image_extent().height(), actual_extent_height)));
        return actual_extent
    }
}

fn create_swapchain(surface: SurfaceKhr, device: Device, queue_family_flags: QueueFlags,
        window_size: Option<voo::Extent2d>, old_swapchain: Option<&SwapchainKhr>)
        -> VooResult<SwapchainKhr> {
    let swapchain_details: SwapchainSupportDetails = SwapchainSupportDetails::new(
        device.instance(), &surface, device.physical_device());
    let surface_format = choose_swap_surface_format(&swapchain_details.formats);
    let present_mode = choose_swap_present_mode(&swapchain_details.present_modes);
    let extent = choose_swap_extent(&swapchain_details.capabilities, window_size);

    // TODO: REVISIT THIS: https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Swap_chain
    let mut image_count = swapchain_details.capabilities.min_image_count() + 1;
    if swapchain_details.capabilities.max_image_count() > 0 &&
            image_count > swapchain_details.capabilities.max_image_count() {
        image_count = swapchain_details.capabilities.max_image_count();
    }
    let indices = queue::queue_families(device.instance(), &surface,
        device.physical_device(), queue_family_flags);
    let queue_family_indices = [indices.flag_idxs[0] as u32,
        indices.presentation_support_idxs[0] as u32];

    let mut bldr = SwapchainKhr::builder();
    bldr.surface(&surface)
        .min_image_count(image_count)
        .image_format(surface_format.format())
        .image_color_space(surface_format.color_space())
        .image_extent(extent.clone())
        .image_array_layers(1)
        .image_usage(voo::ImageUsageFlags::COLOR_ATTACHMENT)
        .pre_transform(swapchain_details.capabilities.current_transform())
        .composite_alpha(voo::CompositeAlphaFlagsKhr::OPAQUE)
        .present_mode(present_mode)
        .clipped(true);

    if let Some(old_sc) = old_swapchain {
        bldr.old_swapchain(old_sc.handle());
    }

    if queue_family_indices[0] != queue_family_indices[1] {
        bldr.image_sharing_mode(voo::SharingMode::Concurrent);
        bldr.queue_family_indices(&queue_family_indices[..]);
    } else {
        bldr.image_sharing_mode(voo::SharingMode::Exclusive);
    }
    bldr.build(device)
}

pub fn create_image_views(swapchain: &SwapchainKhr) -> VooResult<Vec<ImageView>> {
    swapchain.images().iter().map(|&image| {
        ImageView::builder()
            .image(image)
            // .view_type(voo::IMAGE_VIEW_TYPE_2D)
            .view_type(voo::ImageViewType::Type2d)
            .format(swapchain.image_format())
            .components(voo::ComponentMapping::default())
            .subresource_range(voo::ImageSubresourceRange::builder()
                .aspect_mask(voo::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
                .build()
            )
            .build(swapchain.device().clone(), Some(swapchain.clone()))

    }).collect::<Result<Vec<_>, _>>()
}

fn find_supported_format(device: &Device, candidates: &[voo::Format], tiling: voo::ImageTiling,
        features: voo::FormatFeatureFlags) -> VooResult<voo::Format> {
    for &format in candidates {
        let props = device.instance().physical_device_format_properties(device.physical_device(),
            format);

        if tiling == voo::ImageTiling::Linear &&
                props.linear_tiling_features().contains(features) {
            return Ok(format);
        } else if tiling == voo::ImageTiling::Optimal &&
                props.optimal_tiling_features().contains(features) {
            return Ok(format);
        }
    }

    panic!("Failed to find supported format.")
}

fn find_depth_format(device: &Device) -> VooResult<voo::Format> {
    find_supported_format(device, &[voo::Format::D32Sfloat, voo::Format::D32SfloatS8Uint,
        voo::Format::D24UnormS8Uint], voo::ImageTiling::Optimal,
        voo::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
}

fn create_render_pass(device: Device, swapchain_image_format: voo::Format)
        -> VooResult<RenderPass> {
    let depth_image_format = find_depth_format(&device)?;

    let color_attachment = voo::AttachmentDescription::builder()
        .format(swapchain_image_format)
        .samples(voo::SampleCountFlags::COUNT_1)
        .load_op(voo::AttachmentLoadOp::Clear)
        .store_op(voo::AttachmentStoreOp::Store)
        .stencil_load_op(voo::AttachmentLoadOp::DontCare)
        .stencil_store_op(voo::AttachmentStoreOp::DontCare)
        .initial_layout(voo::ImageLayout::Undefined)
        .final_layout(voo::ImageLayout::PresentSrcKhr)
        .build();

    let depth_attachment = voo::AttachmentDescription::builder()
        .format(depth_image_format)
        .samples(voo::SampleCountFlags::COUNT_1)
        .load_op(voo::AttachmentLoadOp::Clear)
        .store_op(voo::AttachmentStoreOp::DontCare)
        .stencil_load_op(voo::AttachmentLoadOp::DontCare)
        .stencil_store_op(voo::AttachmentStoreOp::DontCare)
        .initial_layout(voo::ImageLayout::Undefined)
        .final_layout(voo::ImageLayout::DepthStencilAttachmentOptimal)
        .build();

    let color_attachment_ref = voo::AttachmentReference::builder()
        .attachment(0)
        .layout(voo::ImageLayout::ColorAttachmentOptimal)
        .build();

    let depth_attachment_ref = voo::AttachmentReference::builder()
        .attachment(1)
        .layout(voo::ImageLayout::DepthStencilAttachmentOptimal)
        .build();

    let color_attachments = [color_attachment_ref];

    let subpass = voo::SubpassDescription::builder()
        .pipeline_bind_point(voo::PipelineBindPoint::Graphics)
        .color_attachments(&color_attachments[..])
        .depth_stencil_attachment(&depth_attachment_ref)
        .build();

    let dependency = voo::SubpassDependency::builder()
        .src_subpass(voo::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(voo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(voo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(voo::AccessFlags::COLOR_ATTACHMENT_READ |
            voo::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .build();

    RenderPass::builder()
        .attachments(&[color_attachment, depth_attachment])
        .subpasses(&[subpass])
        .dependencies(&[dependency])
        .build(device)
}

fn create_descriptor_set_layout(device: Device) -> VooResult<DescriptorSetLayout> {
    let ubo_layout_binding = voo::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(voo::DescriptorType::UniformBuffer)
        .descriptor_count(1)
        .stage_flags(voo::ShaderStageFlags::VERTEX)
        .build();

    let sampler_layout_binding = voo::DescriptorSetLayoutBinding::builder()
        .binding(1)
        .descriptor_type(voo::DescriptorType::CombinedImageSampler)
        .descriptor_count(1)
        .stage_flags(voo::ShaderStageFlags::FRAGMENT)
        .build();

    let bindings = [ubo_layout_binding, sampler_layout_binding];

    DescriptorSetLayout::builder()
        .bindings(&bindings)
        .build(device)
}

fn create_descriptor_pool(device: Device) -> VooResult<DescriptorPool> {
    let pool_sizes = [
        voo::DescriptorPoolSize::builder()
            .type_of(voo::DescriptorType::UniformBuffer)
            .descriptor_count(1)
            .build(),
        voo::DescriptorPoolSize::builder()
            .type_of(voo::DescriptorType::CombinedImageSampler)
            .descriptor_count(1)
            .build(),
    ];

    DescriptorPool::builder()
        .max_sets(1)
        .pool_sizes(&pool_sizes)
        .build(device)
}

fn create_descriptor_sets(device: &Device, layout: &DescriptorSetLayout,
        pool: &DescriptorPool, uniform_buffer: &Buffer, texture_image_view: &ImageView,
        texture_sampler: &Sampler) -> VooResult<SmallVec<[voo::DescriptorSet; 8]>> {
    let descriptor_sets = pool.allocate_descriptor_sets(&[layout.handle()][..]);

    let buffer_info = voo::DescriptorBufferInfo::builder()
        .buffer(uniform_buffer)
        .offset(0)
        .range(mem::size_of::<UniformBufferObject>() as u64)
        .build();

    let image_info = voo::DescriptorImageInfo::builder()
        .sampler(texture_sampler)
        .image_view(texture_image_view)
        .image_layout(voo::ImageLayout::ShaderReadOnlyOptimal)
        .build();

    let descriptor_writes = [
        voo::WriteDescriptorSet::builder()
            .dst_set(&descriptor_sets[0])
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_count(1)
            .descriptor_type(voo::DescriptorType::UniformBuffer)
            .buffer_info(&buffer_info)
            .build(),
        voo::WriteDescriptorSet::builder()
            .dst_set(&descriptor_sets[0])
            .dst_binding(1)
            .dst_array_element(0)
            .descriptor_count(1)
            .descriptor_type(voo::DescriptorType::CombinedImageSampler)
            .image_info(&image_info)
            .build(),
    ];

    pool.update_descriptor_sets(Some(&descriptor_writes[..]), None);

    Ok(descriptor_sets)
}

fn create_pipeline_layout(device: Device, descriptor_set_layout: Option<&DescriptorSetLayout>)
        -> VooResult<PipelineLayout> {
    let mut layouts = SmallVec::<[_; 8]>::new();
    if let Some(dsl) = descriptor_set_layout {
        layouts.push(dsl.handle());
    }

    PipelineLayout::builder()
        .set_layouts(&layouts)
        .build(device)
}

fn create_graphics_pipeline(device: Device, pipeline_layout: &PipelineLayout,
        render_pass: &RenderPass, swap_chain_extent: voo::Extent2d, vert_shader_code: &[u8],
        frag_shader_code: &[u8]) -> VooResult<GraphicsPipeline> {
    let vert_shader_module = ShaderModule::new(device.clone(), vert_shader_code)?;
    let frag_shader_module = ShaderModule::new(device.clone(), frag_shader_code)?;

    let fn_name = CStr::from_bytes_with_nul(b"main\0").unwrap();

    let vert_shader_stage_info = voo::PipelineShaderStageCreateInfo::builder()
        .stage(voo::ShaderStageFlags::VERTEX)

        .module(&vert_shader_module)
        .name(fn_name)
        .build();

    let frag_shader_stage_info = voo::PipelineShaderStageCreateInfo::builder()
        .stage(voo::ShaderStageFlags::FRAGMENT)
        .module(&frag_shader_module)
        .name(fn_name)
        .build();

    let binding_descriptions = [Vertex::binding_description()];
    let attribute_descriptions = Vertex::attribute_descriptions();

    let vertex_input_info = voo::PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(&binding_descriptions[..])
        .vertex_attribute_descriptions(&attribute_descriptions[..])
        .build();

    let input_assembly = voo::PipelineInputAssemblyStateCreateInfo::builder()
        // .sType(voo::STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO)
        // .pNext(ptr::null())
        // .flags(0)
        // * VK_PRIMITIVE_TOPOLOGY_POINT_LIST: points from vertices
        // * VK_PRIMITIVE_TOPOLOGY_LINE_LIST: line from every 2 vertices
        //   without reuse
        // * VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: the end vertex of every
        //   line is used as start vertex for the next line
        // * VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: triangle from every 3
        //   vertices without reuse
        // * VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: the second and third
        //   vertex of every triangle are used as first two vertices of
        //   the next triangle
        // .topology(voo::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST)
        .topology(voo::PrimitiveTopology::TriangleList)
        .primitive_restart_enable(false)
        .build();

    let viewport = voo::Viewport::builder()
        .x(0.0f32)
        .y(0.0f32)
        .width(swap_chain_extent.width() as f32)
        .height(swap_chain_extent.height() as f32)
        .min_depth(0.0f32)
        .max_depth(1.0f32)
        .build();

    let scissor = voo::Rect2d::builder()
        .offset(voo::Offset2d::builder()
            .x(0)
            .y(0)
            .build())
        .extent(swap_chain_extent)
        .build();

    let viewports = [viewport];
    let scissors = [scissor];

    let viewport_state = voo::PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports[..])
        .scissors(&scissors[..])
        .build();

    let rasterizer = voo::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(voo::PolygonMode::Fill)
        .cull_mode(voo::CullModeFlags::NONE)
        .front_face(voo::FrontFace::CounterClockwise)
        .depth_bias_enable(false)
        .depth_bias_constant_factor(0.0f32)
        .depth_bias_clamp(0.0f32)
        .depth_bias_slope_factor(0.0f32)
        .line_width(1.0f32)
        .build();

    let multisampling = voo::PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(voo::SampleCountFlags::COUNT_1)
        .sample_shading_enable(false)
        .min_sample_shading(1.0f32)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build();

    let stencil_op_state = voo::StencilOpState::builder()
        .fail_op(voo::StencilOp::Keep)
        .pass_op(voo::StencilOp::Keep)
        .depth_fail_op(voo::StencilOp::Keep)
        .compare_op(voo::CompareOp::Never)
        .compare_mask(0)
        .write_mask(0)
        .reference(0)
        .build();

    let depth_stencil = voo::PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(voo::CompareOp::Less)
        .depth_bounds_test_enable(false)
        .stencil_test_enable(false)
        .front(stencil_op_state.clone())
        .back(stencil_op_state)
        .min_depth_bounds(0.0)
        .max_depth_bounds(1.0)
        .build();

    let color_blend_attachment = voo::PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .src_color_blend_factor(voo::BlendFactor::One)
        .dst_color_blend_factor(voo::BlendFactor::Zero)
        .color_blend_op(voo::BlendOp::Add)
        .src_alpha_blend_factor(voo::BlendFactor::One)
        .dst_alpha_blend_factor(voo::BlendFactor::Zero)
        .alpha_blend_op(voo::BlendOp::Add)
        .color_write_mask(voo::ColorComponentFlags::R | voo::ColorComponentFlags::G |
            voo::ColorComponentFlags::B | voo::ColorComponentFlags::A)
        .build();

    // ///////////////////////////////////////////////
    // /////////// KEEPME (ALPHA BLENDING) ///////////
    // let color_blend_attachment = voo::PipelineColorBlendAttachmentState::builder()
    //     blendEnable(false)
    //     srcColorBlendFactor(voo::BLEND_FACTOR_SRC_ALPHA)
    //     dstColorBlendFactor(voo::BLEND_FACTOR_ONE_MINUS_SRC_ALPHA)
    //     colorBlendOp(voo::BLEND_OP_ADD)
    //     srcAlphaBlendFactor(voo::BLEND_FACTOR_ONE)
    //     dstAlphaBlendFactor(voo::BLEND_FACTOR_ZERO)
    //     alphaBlendOp(voo::BLEND_OP_ADD)
    //     colorWriteMask(voo::COLOR_COMPONENT_R_BIT | voo::COLOR_COMPONENT_G_BIT | voo::COLOR_COMPONENT_B_BIT | voo::COLOR_COMPONENT_A_BIT)
    // }; ////////////////////////////////////////////
    // ///////////////////////////////////////////////

    let attachments = [color_blend_attachment];

    let color_blending = voo::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(voo::LogicOp::Copy)
        .attachments(&attachments)
        .blend_constants([0.0f32; 4])
        .build();

    // ///////////////////////////////////////////////
    // /////////// KEEPME (DYNAMIC STATES) ///////////
    // let dynamic_states = [voo::DYNAMIC_STATE_VIEWPORT) voo::DYNAMIC_STATE_LINE_WIDTH];
    // let dynamic_state = voo::PipelineDynamicStateCreateInfo::builder()
    //     sType(voo::STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO)
    //     pNext(ptr::null())
    //     flags(0)
    //     dynamicStateCount(2)
    //     pDynamicStates(dynamic_states.as_ptr())
    // }; ////////////////////////////////////////////
    // ///////////////////////////////////////////////

    let shader_stages = &[vert_shader_stage_info, frag_shader_stage_info];

    GraphicsPipeline::builder()
        .stages(shader_stages)
        .vertex_input_state(&vertex_input_info)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterizer)
        .multisample_state(&multisampling)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blending)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0)
        .base_pipeline_index(-1)
        .build(device)
}

fn create_command_pool(device: Device, surface: &SurfaceKhr, queue_family_flags: QueueFlags)
        -> VooResult<CommandPool> {
    let queue_family_idx = voo::queue_families(device.instance(), surface,
        device.physical_device(), queue_family_flags).family_idxs()[0] as u32;

    CommandPool::builder()
        .queue_family_index(queue_family_idx)
        .build(device)
}

pub fn create_framebuffers(device: &Device, render_pass: &RenderPass,
        swapchain_image_views: &[ImageView], depth_image_view: &ImageView,
        swapchain_extent: voo::Extent2d) -> VooResult<Vec<Framebuffer>> {
    swapchain_image_views.iter().map(|image_view| {
        let attachments = [image_view, depth_image_view];
        Framebuffer::builder()
            .render_pass(&render_pass)
            .attachments(&attachments[..])
            .width(swapchain_extent.width())
            .height(swapchain_extent.height())
            .layers(1)
            .build(device.clone())
    }).collect::<Result<Vec<_>, _>>()
}


fn begin_single_time_commands(command_pool: &CommandPool)
        -> VooResult<voo::CommandBuffer> {
    // let alloc_info = voo::CommandBufferAllocateInfo::builder()
    //     .command_pool(command_pool)
    //     .level(voo::CommandBufferLevel::Primary)
    //     .command_buffer_count(1)
    //     .build();

    // let mut command_buffer = ptr::null_mut();
    // unsafe {
    //     voo::check(command_pool.device().proc_addr_loader().core.vkAllocateCommandBuffers(
    //         command_pool.device().handle().0,
    //         alloc_info.as_raw(), &mut command_buffer));
    // }

    let command_buffer = command_pool.allocate_command_buffer(voo::CommandBufferLevel::Primary)?;


    // let begin_info = voo::CommandBufferBeginInfo::builder()
    //     .flags(voo::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
    //     .build();

    // unsafe {
    //     command_pool.device().proc_addr_loader().core.vkBeginCommandBuffer(command_buffer,
    //         begin_info.as_raw());
    // }

    command_buffer.begin(voo::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    // voo::CommandBuffer::from_parts(command_pool.clone(), voo::CommandBufferHandle(command_buffer))
    Ok(command_buffer)
}

fn end_single_time_commands(device: &Device, command_pool: &CommandPool,
        command_buffer: voo::CommandBuffer) -> VooResult<()> {
    unsafe {
        voo::check(device.proc_addr_loader().core.vkEndCommandBuffer(command_buffer.handle().0));
    }
    let command_buffers = [command_buffer.handle()];

    let submit_info = voo::SubmitInfo::builder()
        .command_buffers(&command_buffers[..])
        .build();

    let cmd_buf_handles = [command_buffer.handle().0];

    unsafe {
        voo::check(device.proc_addr_loader().core.vkQueueSubmit(device.queue(0), 1,
            submit_info.as_raw(), 0));
        voo::check(device.proc_addr_loader().core.vkQueueWaitIdle(device.queue(0)));
        device.proc_addr_loader().core.vkFreeCommandBuffers(device.handle().0,
            command_pool.handle().0, 1, cmd_buf_handles.as_ptr());
    }

    Ok(())
}

fn has_stencil_component(format: voo::Format) -> bool {
    format == voo::Format::D32SfloatS8Uint || format == voo::Format::D24UnormS8Uint
}

fn transition_image_layout(device: &Device, command_pool: &CommandPool, image: &Image,
        format: voo::Format, old_layout: voo::ImageLayout, new_layout: voo::ImageLayout)
         -> VooResult<()> {
    let command_buffer = begin_single_time_commands(command_pool)?;

    let subresource_range = voo::ImageSubresourceRange::builder()
        .aspect_mask(voo::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let mut barrier = voo::ImageMemoryBarrier::builder()
        .src_access_mask(voo::AccessFlags::empty())
        .dst_access_mask(voo::AccessFlags::empty())
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(voo::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(voo::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(subresource_range)
        .build();

    if new_layout == voo::ImageLayout::DepthStencilAttachmentOptimal {
        barrier.subresource_range_mut().set_aspect_mask(voo::ImageAspectFlags::DEPTH);
        if has_stencil_component(format) {
            let aspect_mask = barrier.subresource_range().aspect_mask() |
                voo::ImageAspectFlags::STENCIL;
            barrier.subresource_range_mut().set_aspect_mask(aspect_mask);
        }
    } else {
        barrier.subresource_range_mut().set_aspect_mask(voo::ImageAspectFlags::COLOR);
    }

    let source_stage: voo::PipelineStageFlags;
    let destination_stage: voo::PipelineStageFlags;

    if old_layout == voo::ImageLayout::Undefined &&
            new_layout == voo::ImageLayout::TransferDstOptimal
    {
        barrier.set_src_access_mask(voo::AccessFlags::empty());
        barrier.set_dst_access_mask(voo::AccessFlags::TRANSFER_WRITE);
        source_stage = voo::PipelineStageFlags::TOP_OF_PIPE;
        destination_stage = voo::PipelineStageFlags::TRANSFER;
    } else if old_layout == voo::ImageLayout::TransferDstOptimal &&
            new_layout == voo::ImageLayout::ShaderReadOnlyOptimal
    {
        barrier.set_src_access_mask(voo::AccessFlags::TRANSFER_WRITE);
        barrier.set_dst_access_mask(voo::AccessFlags::SHADER_READ);
        source_stage = voo::PipelineStageFlags::TRANSFER;
        destination_stage = voo::PipelineStageFlags::FRAGMENT_SHADER;
    } else if old_layout == voo::ImageLayout::Undefined &&
            new_layout == voo::ImageLayout::DepthStencilAttachmentOptimal
        {
        barrier.set_src_access_mask(voo::AccessFlags::empty());
        barrier.set_dst_access_mask(voo::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ |
            voo::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);
        source_stage = voo::PipelineStageFlags::TOP_OF_PIPE;
        destination_stage = voo::PipelineStageFlags::EARLY_FRAGMENT_TESTS;
    } else {
        panic!("unsupported layout transition");
    }

    unsafe {
        device.proc_addr_loader().vkCmdPipelineBarrier(
            command_buffer.handle().0,
            source_stage.bits(), destination_stage.bits(),
            0,
            0, ptr::null(),
            0, ptr::null(),
            1, barrier.as_raw() as *const _ as *const vks::VkImageMemoryBarrier
        );
    }

    end_single_time_commands(device, command_pool, command_buffer)
}

fn copy_buffer_to_image(device: &Device, command_pool: &CommandPool, buffer: &Buffer,
        image: &Image, width: u32, height: u32)  -> VooResult<()> {
    let command_buffer = begin_single_time_commands(command_pool)?;

    let image_subresource_layers = voo::ImageSubresourceLayers::builder()
        .aspect_mask(voo::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let region = voo::BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(image_subresource_layers)
        .image_offset(voo::Offset3d::builder().x(0).y(0).z(0).build())
        .image_extent(voo::Extent3d::builder().width(width).height(height).depth(1).build())
        .build();

    unsafe {
        device.proc_addr_loader().vkCmdCopyBufferToImage(
            command_buffer.handle().0,
            buffer.handle().0,
            image.handle().0,
            voo::ImageLayout::TransferDstOptimal as u32,
            1,
            &region as *const _ as *const vks::VkBufferImageCopy
        );
    }

    end_single_time_commands(device, command_pool, command_buffer)
}

fn copy_buffer(device: &Device, command_pool: &CommandPool, src_buffer: &Buffer,
        dst_buffer: &Buffer, size: voo::DeviceSize)  -> VooResult<()> {
    // TODO: Look into creating a separate command pool with the
    // `VK_COMMAND_POOL_CREATE_TRANSIENT_BIT` flag for short lived command
    // buffers like this.
    let command_buffer = begin_single_time_commands(command_pool)?;

    let copy_region = voo::BufferCopy::builder()
        .src_offset(0)
        .dst_offset(0)
        .size(size)
        .build();

    unsafe { device.proc_addr_loader().core.vkCmdCopyBuffer(command_buffer.handle().0,
        src_buffer.handle().0, dst_buffer.handle().0, 1, &copy_region as *const _
        as *const vks::VkBufferCopy); }


    end_single_time_commands(device, command_pool, command_buffer)
}

fn load_model(device: &Device) -> VooResult<(Vec<Vertex>, Vec<u32>)> {
    let (models, materials) = tobj::load_obj(&Path::new(MODEL_PATH))
        .expect("Error loading model");

    let mut vertices = Vec::with_capacity(4096);
    let mut indices = Vec::with_capacity(4096);

    let mut unique_vertices: HashMap<Vertex, u32> = HashMap::with_capacity(1 << 20);

    let mut mesh_id = 0usize;
    for model in models {
        let mesh = &model.mesh;
        for &index in &mesh.indices {
            let index = index as usize;

            let vert_idz = 3 * index;
            let tex_coord_idz = 2 * index;

            let pos = [
                mesh.positions[vert_idz],
                mesh.positions[vert_idz + 1],
                mesh.positions[vert_idz + 2],
            ];

            let tex_coord = [
                mesh.texcoords[tex_coord_idz],
                1.0 - mesh.texcoords[tex_coord_idz + 1],
            ];

            let vertex = Vertex {
                pos,
                color: [1.0, 1.0, 1.0],
                tex_coord,
            };

            if !unique_vertices.contains_key(&vertex) {
                unique_vertices.insert(vertex.clone(), vertices.len() as u32);
                vertices.push(vertex.clone());
            }

            indices.push(unique_vertices[&vertex]);
        }
        mesh_id += 1;
    }
    Ok((vertices, indices))
}

fn create_vertex_buffer(device: &Device, command_pool: &CommandPool, vertices: &[Vertex])
        -> VooResult<(Buffer, DeviceMemory)> {
    let buffer_bytes = (mem::size_of::<Vertex>() * vertices.len()) as u64;

    // Either:
    // * Use a memory heap that is host coherent, indicated with
    //   VK_MEMORY_PROPERTY_HOST_COHERENT_BIT (or)
    // * Call vkFlushMappedMemoryRanges to after writing to the mapped
    //   memory, and call vkInvalidateMappedMemoryRanges before reading from
    //   the mapped memory
    let staging_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(voo::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(voo::SharingMode::Exclusive)
        .build(device.clone())?;


    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::HOST_VISIBLE | voo::MemoryPropertyFlags::HOST_COHERENT);
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, buffer_bytes, 0)?;
    data.copy_from_slice(vertices);
    staging_buffer_memory.unmap(data);

    let vertex_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(voo::BufferUsageFlags::TRANSFER_DST | voo::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(voo::SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = vertex_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::DEVICE_LOCAL);
    let vertex_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    vertex_buffer.bind_memory(&vertex_buffer_memory, 0)?;

    copy_buffer(device, command_pool, &staging_buffer, &vertex_buffer, buffer_bytes)?;

    Ok((vertex_buffer, vertex_buffer_memory))
}

fn create_index_buffer<T: Copy>(device: &Device, command_pool: &CommandPool, indices: &[T])
        -> VooResult<(Buffer, DeviceMemory)> {
    let buffer_bytes = (mem::size_of::<T>() * indices.len()) as u64;

    let staging_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(voo::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(voo::SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::HOST_VISIBLE);
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, buffer_bytes, 0)?;
    data.copy_from_slice(indices);
    staging_buffer_memory.unmap(data);

    let index_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(voo::BufferUsageFlags::TRANSFER_DST | voo::BufferUsageFlags::INDEX_BUFFER)
        .sharing_mode(voo::SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = index_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::DEVICE_LOCAL);
    let index_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    index_buffer.bind_memory(&index_buffer_memory, 0)?;

    copy_buffer(device, command_pool, &staging_buffer, &index_buffer, buffer_bytes)?;

    Ok((index_buffer, index_buffer_memory))
}

fn create_uniform_buffer(device: &Device, command_pool: &CommandPool, _extent: voo::Extent2d)
        -> VooResult<(Buffer, DeviceMemory)> {
    let buffer_bytes = mem::size_of::<UniformBufferObject>() as u64;
    let uniform_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(voo::BufferUsageFlags::UNIFORM_BUFFER)
        .sharing_mode(voo::SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = uniform_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::HOST_VISIBLE | voo::MemoryPropertyFlags::HOST_COHERENT);
    let uniform_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    uniform_buffer.bind_memory(&uniform_buffer_memory, 0)?;

    Ok((uniform_buffer, uniform_buffer_memory))
}

fn create_depth_resources(device: &Device, command_pool: &CommandPool,
        swapchain_extent: voo::Extent2d) -> VooResult<(Image, DeviceMemory, ImageView)> {
    let depth_format = find_depth_format(device)?;
    let extent = voo::Extent3d::builder()
        .width(swapchain_extent.width())
        .height(swapchain_extent.height())
        .depth(1)
        .build();

    let depth_image = Image::builder()
        .image_type(voo::ImageType::Type2d)
        .format(depth_format)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(voo::SampleCountFlags::COUNT_1)
        .tiling(voo::ImageTiling::Optimal)
        .usage(voo::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        .sharing_mode(voo::SharingMode::Exclusive)
        .initial_layout(voo::ImageLayout::Undefined)
        .build(device.clone())?;

    let memory_requirements = depth_image.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::DEVICE_LOCAL);
    let depth_image_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    depth_image.bind_memory(&depth_image_memory, 0)?;

    let depth_image_view = ImageView::builder()
        .image(depth_image.handle())
        .view_type(voo::ImageViewType::Type2d)
        .format(depth_format)
        .components(voo::ComponentMapping::default())
        .subresource_range(voo::ImageSubresourceRange::builder()
            .aspect_mask(voo::ImageAspectFlags::DEPTH)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build())
        .build(device.clone(), None)?;

    transition_image_layout(device, command_pool, &depth_image, depth_format,
        voo::ImageLayout::Undefined, voo::ImageLayout::DepthStencilAttachmentOptimal)?;

    Ok((depth_image, depth_image_memory, depth_image_view))
}

fn create_texture_image(device: &Device, command_pool: &CommandPool)
        -> VooResult<(Image, DeviceMemory)> {
    let pixels = image::open(TEXTURE_PATH).unwrap().to_rgba();
    let (tex_width, tex_height) = pixels.dimensions();
    let image_bytes = (tex_width * tex_height * 4) as u64;

    let staging_buffer = Buffer::builder()
        .size(image_bytes)
        .usage(voo::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(voo::SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::HOST_VISIBLE | voo::MemoryPropertyFlags::HOST_COHERENT);
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, image_bytes, 0)?;
    data.copy_from_slice(&pixels);
    staging_buffer_memory.unmap(data);

    let extent = voo::Extent3d::builder().width(tex_width).height(tex_height).depth(1).build();

    let texture_image = Image::builder()
        .image_type(voo::ImageType::Type2d)
        .format(voo::Format::R8G8B8A8Unorm)
        .extent(extent.clone())
        .mip_levels(1)
        .array_layers(1)
        .samples(voo::SampleCountFlags::COUNT_1)
        .tiling(voo::ImageTiling::Optimal)
        .usage(voo::ImageUsageFlags::TRANSFER_DST | voo::ImageUsageFlags::SAMPLED)
        .sharing_mode(voo::SharingMode::Exclusive)
        .initial_layout(voo::ImageLayout::Undefined)
        .build(device.clone())?;

    let memory_requirements = texture_image.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        voo::MemoryPropertyFlags::DEVICE_LOCAL);
    let texture_image_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    texture_image.bind_memory(&texture_image_memory, 0)?;

    transition_image_layout(device, command_pool, &texture_image, voo::Format::R8G8B8A8Unorm,
        voo::ImageLayout::Undefined, voo::ImageLayout::TransferDstOptimal)?;

    copy_buffer_to_image(device, command_pool, &staging_buffer, &texture_image,
        extent.width(), extent.height())?;

    transition_image_layout(device, command_pool, &texture_image, voo::Format::R8G8B8A8Unorm,
        voo::ImageLayout::TransferDstOptimal, voo::ImageLayout::ShaderReadOnlyOptimal)?;

    Ok((texture_image, texture_image_memory))
}

fn create_texture_image_view(device: Device, image: &Image) -> VooResult<ImageView> {
    ImageView::builder()
        .image(image.handle())
        .view_type(voo::ImageViewType::Type2d)
        .format(voo::Format::R8G8B8A8Unorm)
        .components(voo::ComponentMapping::default())
        .subresource_range(voo::ImageSubresourceRange::builder()
            .aspect_mask(voo::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build())
        .build(device, None)
}

fn create_texture_sampler(device: Device) -> VooResult<Sampler> {
    Sampler::builder()
        .mag_filter(voo::Filter::Linear)
        .min_filter(voo::Filter::Linear)
        .mipmap_mode(voo::SamplerMipmapMode::Linear)
        .address_mode_u(voo::SamplerAddressMode::Repeat)
        .address_mode_v(voo::SamplerAddressMode::Repeat)
        .address_mode_w(voo::SamplerAddressMode::Repeat)
        .mip_lod_bias(0.)
        .anisotropy_enable(true)
        .max_anisotropy(16.)
        .compare_enable(false)
        .compare_op(voo::CompareOp::Always)
        .min_lod(0.)
        .max_lod(0.)
        .border_color(voo::BorderColor::IntOpaqueBlack)
        .unnormalized_coordinates(false)
        .build(device)
}

pub fn create_command_buffers(device: &Device, command_pool: &CommandPool,
        render_pass: &RenderPass, graphics_pipeline: &GraphicsPipeline,
        swapchain_framebuffers: &[Framebuffer], swapchain_extent: &voo::Extent2d,
        vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: u32,
        index_count: u32, pipeline_layout: &PipelineLayout,
        descriptor_set: voo::DescriptorSet)
        -> VooResult<Vec<voo::CommandBufferHandle>>
{
    let mut command_buffers: Vec<voo::CommandBufferHandle> =
        Vec::with_capacity(swapchain_framebuffers.len());
    unsafe { command_buffers.set_len(swapchain_framebuffers.len()); }

    let alloc_info = voo::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        // * COMMAND_BUFFER_LEVEL_PRIMARY: Can be submitted to a queue for
        //   execution) but cannot be called from other command buffers.
        // * COMMAND_BUFFER_LEVEL_SECONDARY: Cannot be submitted directly) but
        //   can be called from primary command buffers.
        .level(voo::CommandBufferLevel::Primary)
        .command_buffer_count(command_buffers.len() as u32)
        .build();

    unsafe {
        voo::check(device.proc_addr_loader().vkAllocateCommandBuffers(device.handle().0,
            alloc_info.as_raw(), command_buffers.as_mut_ptr() as *mut _
            as *mut vks::VkCommandBuffer));
    }

    for (command_buffer_handle, swapchain_framebuffer) in command_buffers.iter()
            .zip(swapchain_framebuffers.iter())
    {
        let begin_info = voo::CommandBufferBeginInfo::builder()
            // * COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: The command buffer
            //   will be rerecorded right after executing it once.
            // * COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: This is a
            //   secondary command buffer that will be entirely within a
            //   single render pass.
            // * COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: The command buffer
            //   can be resubmitted while it is also already pending
            //   execution.
            .flags(voo::CommandBufferUsageFlags::SIMULTANEOUS_USE)
            .build();

        unsafe {
            voo::check(device.proc_addr_loader().core.vkBeginCommandBuffer(
                command_buffer_handle.0, begin_info.as_raw()));
        }

        let clear_values = &[
            voo::ClearValue { color: voo::ClearColorValue {
                float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32] } },
            voo::ClearValue { depthStencil: vks::VkClearDepthStencilValue {
                depth: 1.0, stencil: 0, } },
        ];

        let render_pass_info = voo::RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(swapchain_framebuffer)
            .render_area(voo::Rect2d::builder()
                .offset(voo::Offset2d::builder().x(0).y(0).build())
                .extent(swapchain_extent.clone())
                .build())
            .clear_values(clear_values)
            .build();

        unsafe {
            device.proc_addr_loader().core.vkCmdBeginRenderPass(command_buffer_handle.0,
                render_pass_info.as_raw(), voo::SubpassContents::Inline.into());
            device.proc_addr_loader().core.vkCmdBindPipeline(command_buffer_handle.0,
                voo::PipelineBindPoint::Graphics as u32, graphics_pipeline.handle().0);

            let vertex_buffers = [vertex_buffer.handle()];
            let offsets = [0];
            device.proc_addr_loader().core.vkCmdBindVertexBuffers(
                command_buffer_handle.0, 0, 1, vertex_buffers.as_ptr() as *const vks::VkBuffer,
                offsets.as_ptr());
            device.proc_addr_loader().core.vkCmdBindIndexBuffer(command_buffer_handle.0,
                index_buffer.handle().0, 0, voo::IndexType::Uint32 as u32);

            device.proc_addr_loader().core.vkCmdBindDescriptorSets(command_buffer_handle.0,
                voo::PipelineBindPoint::Graphics as u32, pipeline_layout.handle().0, 0, 1,
                &descriptor_set.handle().0, 0, ptr::null());

            // // * vertexCount: Even though we don't have a vertex buffer, we
            // //   technically still have 3 vertices to draw.
            // // * instanceCount: Used for instanced rendering, use 1 if you're
            // //   not doing that.
            // // * firstVertex: Used as an offset into the vertex buffer,
            // //   defines the lowest value of gl_VertexIndex.
            // // * firstInstance: Used as an offset for instanced rendering,
            // //   defines the lowest value of gl_InstanceIndex.
            // device.proc_addr_loader().core.vkCmdDraw(command_buffer, vertex_count, 1, 0, 0);
            device.proc_addr_loader().core.vkCmdDrawIndexed(command_buffer_handle.0, index_count,
                1, 0, 0, 0);

            device.proc_addr_loader().core.vkCmdEndRenderPass(command_buffer_handle.0);
            device.proc_addr_loader().core.vkEndCommandBuffer(command_buffer_handle.0);
        }
    }
    Ok(command_buffers)
}


struct SwapchainComponents {
    image_views: Vec<ImageView>,
    render_pass: RenderPass,
    graphics_pipeline: GraphicsPipeline,
    depth_image: Image,
    depth_image_memory: DeviceMemory,
    depth_image_view: ImageView,
    framebuffers: Vec<Framebuffer>,
}


struct App {
    instance: Instance,
    window: Window,
    events_loop: EventsLoop,
    queue_family_flags: QueueFlags,
    device: Device,
    surface: SurfaceKhr,
    descriptor_set_layout: DescriptorSetLayout,
    pipeline_layout: PipelineLayout,
    vert_shader_code: Vec<u8>,
    frag_shader_code: Vec<u8>,
    command_pool: CommandPool,
    texture_image: Image,
    texture_image_memory: DeviceMemory,
    texture_image_view: ImageView,
    texture_sampler: Sampler,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
    uniform_buffer: Buffer,
    uniform_buffer_memory: DeviceMemory,
    descriptor_pool: DescriptorPool,
    descriptor_sets: SmallVec<[voo::DescriptorSet; 8]>,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    start_time: time::Instant,
    swapchain: Option<SwapchainKhr>,
    swapchain_components: Option<SwapchainComponents>,
    command_buffers: Option<Vec<voo::CommandBufferHandle>>,
}

impl App {
    #[allow(unused_unsafe)]
    pub unsafe fn new() -> VooResult<App> {
        let instance = init_instance()?;
        let (window, events_loop) = init_window();
        let surface = voodoo_winit::create_surface(instance.clone(), &window)?;
        let queue_family_flags = QueueFlags::GRAPHICS;
        let physical_device = choose_physical_device(&instance, &surface,
            queue_family_flags)?;
        let device = create_device(instance.clone(), &surface, physical_device,
            queue_family_flags)?;
        let swapchain = create_swapchain(surface.clone(), device.clone(), queue_family_flags,
            None, None)?;
        let image_views = create_image_views(&swapchain)?;
        let render_pass = create_render_pass(device.clone(), swapchain.image_format())?;
        let descriptor_set_layout = create_descriptor_set_layout(device.clone())?;
        let pipeline_layout = create_pipeline_layout(device.clone(),
            Some(&descriptor_set_layout))?;
        let vert_shader_code = util::read_file("/src/voodoo/shaders/vert.spv")?;
        let frag_shader_code = util::read_file("/src/voodoo/shaders/frag.spv")?;
        let graphics_pipeline = create_graphics_pipeline(device.clone(), &pipeline_layout,
            &render_pass, swapchain.extent().clone(), &vert_shader_code, &frag_shader_code)?;
        let command_pool = create_command_pool(device.clone(), &surface, queue_family_flags)?;
        let (depth_image, depth_image_memory, depth_image_view) = create_depth_resources(&device,
            &command_pool, swapchain.extent().clone())?;
        let framebuffers = create_framebuffers(&device, &render_pass,
            &image_views, &depth_image_view, swapchain.extent().clone())?;
        let (texture_image, texture_image_memory) = create_texture_image(&device,
            &command_pool)?;
        let texture_image_view = create_texture_image_view(device.clone(),
            &texture_image)?;
        let texture_sampler = create_texture_sampler(device.clone())?;
        // let (vertices, indices) = load_model(&device)?;
        let vertices = VERTICES[..].to_owned();
        let indices = INDICES[..].to_owned();
        let (vertex_buffer, vertex_buffer_memory) = create_vertex_buffer(&device, &command_pool,
            &vertices)?;
        let (index_buffer, index_buffer_memory) = create_index_buffer(&device, &command_pool,
            &indices)?;
        let (uniform_buffer, uniform_buffer_memory) = create_uniform_buffer(&device,
            &command_pool, swapchain.extent().clone())?;
        let descriptor_pool = create_descriptor_pool(device.clone())?;
        let descriptor_sets = create_descriptor_sets(&device, &descriptor_set_layout,
            &descriptor_pool, &uniform_buffer, &texture_image_view, &texture_sampler)?;
        let command_buffers = create_command_buffers(&device, &command_pool, &render_pass,
            &graphics_pipeline, &framebuffers, swapchain.extent(),
            &vertex_buffer, &index_buffer,
            vertices.len() as u32, vertices.len() as u32, &pipeline_layout,
            descriptor_sets[0].clone())?;
        let image_available_semaphore = Semaphore::new(device.clone())?;
        let render_finished_semaphore = Semaphore::new(device.clone())?;
        let start_time = time::Instant::now();

        let swapchain_components = SwapchainComponents {
            image_views: image_views,
            render_pass: render_pass,
            graphics_pipeline: graphics_pipeline,
            depth_image,
            depth_image_memory,
            depth_image_view,
            framebuffers: framebuffers,
        };

        Ok(App {
            instance,
            window: window,
            events_loop: events_loop,
            queue_family_flags,
            device: device,
            surface: surface,
            descriptor_set_layout,
            pipeline_layout,
            vert_shader_code,
            frag_shader_code,
            command_pool,
            texture_image,
            texture_image_memory,
            texture_image_view,
            texture_sampler,
            vertices: vertices,
            indices: indices,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            uniform_buffer,
            uniform_buffer_memory,
            descriptor_pool,
            descriptor_sets,
            image_available_semaphore,
            render_finished_semaphore,
            start_time,
            swapchain: Some(swapchain),
            swapchain_components: Some(swapchain_components),
            command_buffers: Some(command_buffers),
        })
    }

    fn cleanup_swapchain(&mut self) {
        self.swapchain = None;
        self.swapchain_components = None;
        unsafe {
            self.device.proc_addr_loader().core.vkFreeCommandBuffers(self.device.handle().0,
                self.command_pool.handle().0,
                self.command_buffers.as_ref().unwrap().len() as u32,
                self.command_buffers.as_mut().unwrap().as_mut_ptr() as *mut _
                as *mut vks::VkCommandBuffer);
        }
        self.command_buffers = None;
    }

    fn recreate_swapchain(&mut self, current_extent: voo::Extent2d) -> VooResult<()> {
        unsafe { voo::check(self.device.proc_addr_loader().vkDeviceWaitIdle(
            self.device.handle().0)); }

        let swapchain = create_swapchain(self.surface.clone(), self.device.clone(),
            self.queue_family_flags, Some(current_extent), self.swapchain.as_ref().take())?;

        self.cleanup_swapchain();

        let image_views = create_image_views(&swapchain)?;
        let render_pass = create_render_pass(self.device.clone(),
            swapchain.image_format())?;
        let graphics_pipeline = create_graphics_pipeline(self.device.clone(),
            &self.pipeline_layout, &render_pass,
            swapchain.extent().clone(), &self.vert_shader_code, &self.frag_shader_code)?;
        let (depth_image, depth_image_memory, depth_image_view) = create_depth_resources(
            &self.device, &self.command_pool, swapchain.extent().clone())?;
        let framebuffers = create_framebuffers(&self.device,
            &render_pass, &image_views,
            &depth_image_view, swapchain.extent().clone())?;
        let command_buffers = create_command_buffers(&self.device, &self.command_pool,
            &render_pass, &graphics_pipeline,
            &framebuffers, swapchain.extent(),
            &self.vertex_buffer, &self.index_buffer, self.vertices.len() as u32,
            self.indices.len() as u32, &self.pipeline_layout, self.descriptor_sets[0].clone())?;

        self.swapchain = Some(swapchain);
        self.swapchain_components = Some(SwapchainComponents {
            image_views: image_views,
            render_pass: render_pass,
            graphics_pipeline: graphics_pipeline,
            depth_image,
            depth_image_memory,
            depth_image_view,
            framebuffers: framebuffers,
        });
        self.command_buffers = Some(command_buffers);

        Ok(())
    }

    fn update_uniform_buffer(&mut self) -> VooResult<()> {
        let current_time = time::Instant::now();
        let elapsed = current_time.duration_since(self.start_time);
        let time = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32 * 1e-9);

        let extent = self.swapchain.as_ref().unwrap().extent().clone();
        let mut proj = cgmath::perspective(cgmath::Rad(45.0f32.to_radians()),
            extent.width() as f32 / extent.height() as f32, 0.1, 10.0);
        let view = cgmath::Matrix4::look_at(cgmath::Point3::new(2.0, 2.0, 2.0),
            cgmath::Point3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0));
        let scale = cgmath::Matrix4::from_scale(1.5);
        proj[1][1] *= -1.0;
        let rotation = Matrix3::from_angle_z(cgmath::Rad(time)) *
            Matrix3::from_angle_x(cgmath::Rad(time / 2.0));
        let model = Matrix4::from(rotation).into();

        let ubo = UniformBufferObject {
            model: model,
            view: (view * scale).into(),
            proj: proj.into(),
        };

        let mut data = self.uniform_buffer_memory.map(0,
            mem::size_of::<UniformBufferObject>() as u64, 0)?;
        data.copy_from_slice(&[ubo]);
        self.uniform_buffer_memory.unmap(data);

        Ok(())
    }

    fn draw_frame(&mut self) -> VooResult<()> {
        let mut image_index = 0u32;
        let acq_res = unsafe {
            self.device.proc_addr_loader().khr_swapchain.vkAcquireNextImageKHR(
                self.device.handle().0, self.swapchain.as_ref().unwrap().handle().0,
                u64::max_value(), self.image_available_semaphore.handle().0, 0, &mut image_index)
        };
        if acq_res == voo::ResultEnum::ErrorOutOfDateKhr as i32 {
            let dims = self.window.get_inner_size_pixels().unwrap();
            self.recreate_swapchain(voo::Extent2d::builder()
                .height(dims.0).width(dims.1).build())?;
            return Ok(());
        } else if acq_res != voo::ResultEnum::Success as i32 && acq_res !=
                voo::ResultEnum::SuboptimalKhr as i32 {
            panic!("Unable to present swap chain image");
        }
        let wait_semaphores = [self.image_available_semaphore.handle()];
        let wait_stages = voo::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let signal_semaphores = [self.render_finished_semaphore.handle()];
        let command_buffers = [self.command_buffers.as_ref().unwrap().get(image_index as usize).unwrap().clone()];

        let submit_info = voo::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores[..])
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers[..])
            .signal_semaphores(&signal_semaphores[..])
            .build();

        unsafe { voo::check(self.device.proc_addr_loader().core.vkQueueSubmit(
            self.device.queue(0), 1, submit_info.as_raw(), 0)); }

        let swapchains = [self.swapchain.as_ref().unwrap().handle()];
        let image_indices = [image_index];

        let present_info = voo::PresentInfoKhr::builder()
            .wait_semaphores(&signal_semaphores[..])
            .swapchains(&swapchains[..])
            .image_indices(&image_indices)
            .build();

        unsafe {
            voo::check(self.device.proc_addr_loader().khr_swapchain.vkQueuePresentKHR(
                self.device.queue(0), present_info.as_raw()));
            voo::check(self.device.proc_addr_loader().core.vkQueueWaitIdle(
                self.device.queue(0)));
        }

        Ok(())
    }

    fn main_loop(&mut self) -> VooResult<()> {
        let mut exit = false;
        let mut recreate_swap = false;
        let mut current_extent = self.swapchain.as_ref().unwrap().extent().clone();

        loop {
            self.events_loop.poll_events(|event| {
                match event {
                    Event::WindowEvent { event: WindowEvent::Resized(w, h), .. } => {
                        current_extent = voo::Extent2d::builder().width(w).height(h).build();
                        recreate_swap = true;
                    },
                    Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                        exit = true;
                    },
                    _ => ()
                }
            });

            if recreate_swap {
                self.recreate_swapchain(current_extent.clone())?;
                recreate_swap = false;
            };
            if exit { break; }

            self.update_uniform_buffer()?;
            self.draw_frame()?;
        }

        unsafe { voo::check(self.device.proc_addr_loader().core.vkDeviceWaitIdle(
            self.device.handle().0)); }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("Goodbye triangle.");
    }
}


#[allow(unused_unsafe)]
pub unsafe fn make_everything() -> VooResult<()> {
    let instance = init_instance()?;
    let (window, events_loop) = init_window();
    let surface = voodoo_winit::create_surface(instance.clone(), &window)?;
    let queue_family_flags = QueueFlags::GRAPHICS;
    let physical_device = choose_physical_device(&instance, &surface,
        queue_family_flags)?;
    let device = create_device(instance.clone(), &surface, physical_device,
        queue_family_flags)?;
    let swapchain = create_swapchain(surface.clone(), device.clone(), queue_family_flags,
        None, None)?;
    // let image_views = create_image_views(&swapchain)?;

    // let render_pass = create_render_pass_fake(device.clone(), swapchain.image_format())?;
    let render_pass = create_render_pass(device.clone(), swapchain.image_format())?;

    // let descriptor_set_layout = create_descriptor_set_layout(device.clone())?;
    // let pipeline_layout = create_pipeline_layout(device.clone(),
    //     Some(&descriptor_set_layout))?;
    // let vert_shader_code = util::read_file("/src/voodoo/shaders/vert.spv")?;
    // let frag_shader_code = util::read_file("/src/voodoo/shaders/frag.spv")?;
    // let graphics_pipeline = create_graphics_pipeline(device.clone(), &pipeline_layout,
    //     &render_pass, swapchain.extent().clone(), &vert_shader_code, &frag_shader_code)?;
    // let command_pool = create_command_pool(device.clone(), &surface, queue_family_flags)?;
    // let (depth_image, depth_image_memory, depth_image_view) = create_depth_resources(&device,
    //     &command_pool, swapchain.extent().clone())?;
    // let framebuffers = create_framebuffers(&device, &render_pass,
    //     &image_views, &depth_image_view, swapchain.extent().clone())?;
    // let (texture_image, texture_image_memory) = create_texture_image(&device,
    //     &command_pool)?;
    // let texture_image_view = create_texture_image_view(device.clone(),
    //     &texture_image)?;
    // let texture_sampler = create_texture_sampler(device.clone())?;
    // // // let (vertices, indices) = load_model(&device)?;
    // let vertices = VERTICES[..].to_owned();
    // let indices = INDICES[..].to_owned();
    // let (vertex_buffer, vertex_buffer_memory) = create_vertex_buffer(&device, &command_pool,
    //     &vertices)?;
    // let (index_buffer, index_buffer_memory) = create_index_buffer(&device, &command_pool,
    //     &indices)?;
    // let (uniform_buffer, uniform_buffer_memory) = create_uniform_buffer(&device,
    //     &command_pool, swapchain.extent().clone())?;
    // let descriptor_pool = create_descriptor_pool(device.clone())?;
    // let descriptor_sets = create_descriptor_sets(&device, &descriptor_set_layout,
    //     &descriptor_pool, &uniform_buffer, &texture_image_view, &texture_sampler)?;
    // let command_buffers = create_command_buffers(&device, &command_pool, &render_pass,
    //     &graphics_pipeline, &framebuffers, swapchain.extent(),
    //     &vertex_buffer, &index_buffer,
    //     vertices.len() as u32, vertices.len() as u32, &pipeline_layout,
    //     descriptor_sets[0].clone())?;
    // let image_available_semaphore = Semaphore::new(device.clone())?;
    // let render_finished_semaphore = Semaphore::new(device.clone())?;
    // let start_time = time::Instant::now();

    // let swapchain_components = SwapchainComponents {
    //     image_views: image_views,
    //     render_pass: render_pass,
    //     graphics_pipeline: graphics_pipeline,
    //     depth_image,
    //     depth_image_memory,
    //     depth_image_view,
    //     framebuffers: framebuffers,
    // };

    Ok(())
}



fn main() {
    println!("Hello triangle!");
    unsafe {
        // main_debug::make_everything();
        let mut app = App::new().unwrap();
        app.main_loop().unwrap();
    }
}