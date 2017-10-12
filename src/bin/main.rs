#![allow(/*unused_imports,*/ dead_code, unused_variables)]

extern crate voodoo as voo;
extern crate cgmath;
extern crate image;
extern crate smallvec;
extern crate libc;
extern crate tobj;

use std::mem;
use std::time;
use std::path::Path;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::cmp;
use libc::c_char;
use smallvec::SmallVec;
use cgmath::{Matrix3, Matrix4};
use voo::{voodoo_winit, vks, util, queue, Result as VooResult, Instance, Device, SurfaceKhr,
    SwapchainKhr, ImageView, PipelineLayout, RenderPass, GraphicsPipeline, Framebuffer,
    CommandPool, Semaphore, Buffer, DeviceMemory, Vertex, DescriptorSetLayout, UniformBufferObject,
    DescriptorPool, Image, Sampler, Loader, SwapchainSupportDetails, PhysicalDevice,
    PhysicalDeviceFeatures, ShaderModule, QueueFlags, Format, ApplicationInfo,
    DeviceQueueCreateInfo, SurfaceFormatKhr, ColorSpaceKhr, PresentModeKhr, SurfaceCapabilitiesKhr,
    Extent2d, ImageUsageFlags, CompositeAlphaFlagsKhr, SharingMode, ImageViewType,
    ComponentMapping, ImageSubresourceRange, ImageAspectFlags, ImageTiling, FormatFeatureFlags,
    AttachmentDescription, SampleCountFlags, AttachmentLoadOp, AttachmentStoreOp, ImageLayout,
    AttachmentReference, SubpassDescription, PipelineBindPoint, SubpassDependency,
    PipelineStageFlags, AccessFlags, DescriptorSetLayoutBinding, DescriptorType, ShaderStageFlags,
    DescriptorPoolSize, DescriptorSet, DescriptorBufferInfo, DescriptorImageInfo,
    WriteDescriptorSet, PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
    PipelineInputAssemblyStateCreateInfo, PrimitiveTopology, Viewport, Rect2d, Offset2d,
    PipelineViewportStateCreateInfo, PipelineRasterizationStateCreateInfo, PolygonMode,
    CullModeFlags, FrontFace, PipelineMultisampleStateCreateInfo, StencilOpState, StencilOp,
    CompareOp, PipelineDepthStencilStateCreateInfo, PipelineColorBlendAttachmentState, BlendFactor,
    BlendOp, ColorComponentFlags, PipelineColorBlendStateCreateInfo, LogicOp, CommandBuffer,
    CommandBufferLevel, CommandBufferUsageFlags, SubmitInfo, ImageMemoryBarrier, DependencyFlags,
    ImageSubresourceLayers, BufferImageCopy, Offset3d, Extent3d, DeviceSize, BufferCopy,
    BufferUsageFlags, MemoryPropertyFlags, MemoryMapFlags, ImageType, Filter, SamplerMipmapMode,
    SamplerAddressMode, BorderColor, CommandBufferHandle, CommandBufferBeginInfo, ClearValue,
    ClearColorValue, RenderPassBeginInfo, SubpassContents, IndexType, SemaphoreCreateFlags,
    CallResult, PresentInfoKhr, ErrorKind, VALIDATION_LAYER_NAMES};
use voodoo_winit::winit::{EventsLoop, WindowBuilder, Window, Event, WindowEvent};

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

// static REQUIRED_INSTANCE_EXTENSIONS: &[&[u8]] = &[
//     b"VK_KHR_surface\0",
//     b"VK_KHR_win32_surface\0",
// ];

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
    if ENABLE_VALIDATION_LAYERS && !loader.check_validation_layer_support().unwrap() {
        panic!("Unable to enable validation layers.");
    }
    if ENABLE_VALIDATION_LAYERS {
         VALIDATION_LAYER_NAMES.iter().map(|lyr_name|
            unsafe { CStr::from_ptr(lyr_name.as_ptr() as *const c_char) }).collect()
    } else {
        SmallVec::new()
    }
}

/// Initializes a loader and returns a new instance.
fn init_instance() -> VooResult<Instance> {
    let app_name = CString::new("Hello Triangle")?;
    let eng_name = CString::new("None")?;

    let app_info = ApplicationInfo::builder()
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
        .enabled_extensions(loader.enumerate_instance_extension_properties()?.as_slice())
        .build(loader)
}

/// Returns true if the specified physical device has the required features,
/// extensions, queue families and if the supported swap chain has the correct
/// presentation modes.
fn device_is_suitable(instance: &Instance, surface: &SurfaceKhr,
        physical_device: &PhysicalDevice, queue_family_flags: QueueFlags) -> VooResult<bool> {
    let device_features = physical_device.features();

    let reqd_exts: SmallVec<[_; 16]> = (&REQUIRED_DEVICE_EXTENSIONS[..]).iter().map(|ext_name| {
        CStr::from_bytes_with_nul(ext_name).expect("invalid required extension name")
    }).collect();

    let extensions_supported = physical_device.verify_extensions_support(&reqd_exts[..])?;

    let mut swap_chain_adequate = false;
    if extensions_supported {
        let swap_chain_details = SwapchainSupportDetails::new(surface,
            &physical_device)?;
        swap_chain_adequate = !swap_chain_details.formats.is_empty() &&
            !swap_chain_details.present_modes.is_empty()
    }

    let queue_family_indices = queue::queue_families(surface,
        &physical_device, queue_family_flags)?;

    Ok(queue_family_indices.is_complete() &&
        extensions_supported &&
        swap_chain_adequate &&
        device_features.sampler_anisotropy())
}

/// Returns a physical device from the list of available physical devices if
/// it meets the criteria specified in the above function.
fn choose_physical_device(instance: &Instance, surface: &SurfaceKhr,
        queue_family_flags: QueueFlags) -> VooResult<PhysicalDevice> {
    let mut preferred_device = None;
    for device in instance.physical_devices() {
        if device_is_suitable(instance, surface, &device, queue_family_flags)? {
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
    let queue_family_idx = queue::queue_families(surface,
        &physical_device, queue_familiy_flags)?.family_idxs()[0] as u32;

    let queue_priorities = [1.0];
    let queue_create_info = DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_idx)
        .queue_priorities(&queue_priorities)
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

fn choose_swap_surface_format(available_formats: &[SurfaceFormatKhr])
        -> SurfaceFormatKhr {
    if available_formats.len() == 1 && available_formats[0].format() == Format::Undefined {
        return SurfaceFormatKhr::builder()
            .format(Format::B8G8R8A8Unorm)
            .color_space(ColorSpaceKhr::SrgbNonlinearKhr)
            .build();
    }
    for available_format in available_formats {
        if available_format.format() == Format::B8G8R8A8Unorm &&
                available_format.color_space() == ColorSpaceKhr::SrgbNonlinearKhr {
            return SurfaceFormatKhr::builder()
                .format(Format::B8G8R8A8Unorm)
                .color_space(ColorSpaceKhr::SrgbNonlinearKhr)
                .build();
        }
    }
    SurfaceFormatKhr::builder()
        .format(available_formats[0].format())
        .color_space(available_formats[0].color_space())
        .build()
}

fn choose_swap_present_mode(available_present_modes: &[PresentModeKhr])
        -> PresentModeKhr {
    let mut best_mode = PresentModeKhr::MailboxKhr;
    for &available_present_mode in available_present_modes {
        if available_present_mode == PresentModeKhr::FifoKhr {
            return available_present_mode;
        } else if available_present_mode == PresentModeKhr::ImmediateKhr {
            best_mode = available_present_mode;
        }
    }
    best_mode
}

fn choose_swap_extent(capabilities: &SurfaceCapabilitiesKhr,
        window_size: Option<Extent2d>) -> Extent2d {
    if capabilities.current_extent().width() != u32::max_value() {
        return capabilities.current_extent().clone();
    } else {

        let mut actual_extent = window_size
            .unwrap_or(Extent2d::builder().width(1024).height(768).build());
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
        window_size: Option<Extent2d>, old_swapchain: Option<&SwapchainKhr>)
        -> VooResult<SwapchainKhr> {
    let swapchain_details = SwapchainSupportDetails::new(&surface, device.physical_device())?;
    let surface_format = choose_swap_surface_format(&swapchain_details.formats);
    let present_mode = choose_swap_present_mode(&swapchain_details.present_modes);
    let extent = choose_swap_extent(&swapchain_details.capabilities, window_size);

    // TODO: REVISIT THIS: https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Swap_chain
    let mut image_count = swapchain_details.capabilities.min_image_count() + 1;
    if swapchain_details.capabilities.max_image_count() > 0 &&
            image_count > swapchain_details.capabilities.max_image_count() {
        image_count = swapchain_details.capabilities.max_image_count();
    }
    let indices = queue::queue_families(&surface, device.physical_device(), queue_family_flags)?;
    let queue_family_indices = [indices.flag_idxs[0] as u32,
        indices.presentation_support_idxs[0] as u32];

    let mut bldr = SwapchainKhr::builder();
    bldr.surface(&surface)
        .min_image_count(image_count)
        .image_format(surface_format.format())
        .image_color_space(surface_format.color_space())
        .image_extent(extent.clone())
        .image_array_layers(1)
        .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
        .pre_transform(swapchain_details.capabilities.current_transform())
        .composite_alpha(CompositeAlphaFlagsKhr::OPAQUE)
        .present_mode(present_mode)
        .clipped(true);

    if let Some(old_sc) = old_swapchain {
        bldr.old_swapchain(old_sc.handle());
    }

    if queue_family_indices[0] != queue_family_indices[1] {
        bldr.image_sharing_mode(SharingMode::Concurrent);
        bldr.queue_family_indices(&queue_family_indices[..]);
    } else {
        bldr.image_sharing_mode(SharingMode::Exclusive);
    }
    bldr.build(device)
}

pub fn create_image_views(swapchain: &SwapchainKhr) -> VooResult<Vec<ImageView>> {
    swapchain.images().iter().map(|&image| {
        ImageView::builder()
            .image(image)
            // .view_type(IMAGE_VIEW_TYPE_2D)
            .view_type(ImageViewType::Type2d)
            .format(swapchain.image_format())
            .components(ComponentMapping::default())
            .subresource_range(ImageSubresourceRange::builder()
                .aspect_mask(ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
                .build()
            )
            .build(swapchain.device().clone(), Some(swapchain.clone()))

    }).collect::<Result<Vec<_>, _>>()
}

fn find_supported_format(device: &Device, candidates: &[Format], tiling: ImageTiling,
        features: FormatFeatureFlags) -> VooResult<Format> {
    for &format in candidates {
        let props = device.physical_device().format_properties(format);

        if tiling == ImageTiling::Linear &&
                props.linear_tiling_features().contains(features) {
            return Ok(format);
        } else if tiling == ImageTiling::Optimal &&
                props.optimal_tiling_features().contains(features) {
            return Ok(format);
        }
    }

    panic!("Failed to find supported format.")
}

fn find_depth_format(device: &Device) -> VooResult<Format> {
    find_supported_format(device, &[Format::D32Sfloat, Format::D32SfloatS8Uint,
        Format::D24UnormS8Uint], ImageTiling::Optimal,
        FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
}

fn create_render_pass(device: Device, swapchain_image_format: Format)
        -> VooResult<RenderPass> {
    let depth_image_format = find_depth_format(&device)?;

    let color_attachment = AttachmentDescription::builder()
        .format(swapchain_image_format)
        .samples(SampleCountFlags::COUNT_1)
        .load_op(AttachmentLoadOp::Clear)
        .store_op(AttachmentStoreOp::Store)
        .stencil_load_op(AttachmentLoadOp::DontCare)
        .stencil_store_op(AttachmentStoreOp::DontCare)
        .initial_layout(ImageLayout::Undefined)
        .final_layout(ImageLayout::PresentSrcKhr)
        .build();

    let depth_attachment = AttachmentDescription::builder()
        .format(depth_image_format)
        .samples(SampleCountFlags::COUNT_1)
        .load_op(AttachmentLoadOp::Clear)
        .store_op(AttachmentStoreOp::DontCare)
        .stencil_load_op(AttachmentLoadOp::DontCare)
        .stencil_store_op(AttachmentStoreOp::DontCare)
        .initial_layout(ImageLayout::Undefined)
        .final_layout(ImageLayout::DepthStencilAttachmentOptimal)
        .build();

    let color_attachment_ref = AttachmentReference::builder()
        .attachment(0)
        .layout(ImageLayout::ColorAttachmentOptimal)
        .build();

    let depth_attachment_ref = AttachmentReference::builder()
        .attachment(1)
        .layout(ImageLayout::DepthStencilAttachmentOptimal)
        .build();

    let color_attachments = [color_attachment_ref];

    let subpass = SubpassDescription::builder()
        .pipeline_bind_point(PipelineBindPoint::Graphics)
        .color_attachments(&color_attachments[..])
        .depth_stencil_attachment(&depth_attachment_ref)
        .build();

    let dependency = SubpassDependency::builder()
        .src_subpass(voo::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_READ |
            AccessFlags::COLOR_ATTACHMENT_WRITE)
        .build();

    RenderPass::builder()
        .attachments(&[color_attachment, depth_attachment])
        .subpasses(&[subpass])
        .dependencies(&[dependency])
        .build(device)
}

fn create_descriptor_set_layout(device: Device) -> VooResult<DescriptorSetLayout> {
    let ubo_layout_binding = DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(DescriptorType::UniformBuffer)
        .descriptor_count(1)
        .stage_flags(ShaderStageFlags::VERTEX)
        .build();

    let sampler_layout_binding = DescriptorSetLayoutBinding::builder()
        .binding(1)
        .descriptor_type(DescriptorType::CombinedImageSampler)
        .descriptor_count(1)
        .stage_flags(ShaderStageFlags::FRAGMENT)
        .build();

    let bindings = [ubo_layout_binding, sampler_layout_binding];

    DescriptorSetLayout::builder()
        .bindings(&bindings)
        .build(device)
}

fn create_descriptor_pool(device: Device) -> VooResult<DescriptorPool> {
    let pool_sizes = [
        DescriptorPoolSize::builder()
            .type_of(DescriptorType::UniformBuffer)
            .descriptor_count(1)
            .build(),
        DescriptorPoolSize::builder()
            .type_of(DescriptorType::CombinedImageSampler)
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
        texture_sampler: &Sampler) -> VooResult<SmallVec<[DescriptorSet; 8]>> {
    let descriptor_sets = pool.allocate_descriptor_sets(&[layout.handle()])?;

    let buffer_info = DescriptorBufferInfo::builder()
        .buffer(uniform_buffer)
        .offset(0)
        .range(mem::size_of::<UniformBufferObject>() as u64)
        .build();

    let image_info = DescriptorImageInfo::builder()
        .sampler(texture_sampler)
        .image_view(texture_image_view)
        .image_layout(ImageLayout::ShaderReadOnlyOptimal)
        .build();

    let descriptor_writes = [
        WriteDescriptorSet::builder()
            .dst_set(&descriptor_sets[0])
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::UniformBuffer)
            .buffer_info(&buffer_info)
            .build(),
        WriteDescriptorSet::builder()
            .dst_set(&descriptor_sets[0])
            .dst_binding(1)
            .dst_array_element(0)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::CombinedImageSampler)
            .image_info(&image_info)
            .build(),
    ];

    pool.update_descriptor_sets(&descriptor_writes, &[]);

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
        render_pass: &RenderPass, swap_chain_extent: Extent2d, vert_shader_code: &[u32],
        frag_shader_code: &[u32]) -> VooResult<GraphicsPipeline> {
    let vert_shader_module = ShaderModule::new(device.clone(), vert_shader_code)?;
    let frag_shader_module = ShaderModule::new(device.clone(), frag_shader_code)?;

    let fn_name = CStr::from_bytes_with_nul(b"main\0").unwrap();

    let vert_shader_stage_info = PipelineShaderStageCreateInfo::builder()
        .stage(ShaderStageFlags::VERTEX)

        .module(&vert_shader_module)
        .name(fn_name)
        .build();

    let frag_shader_stage_info = PipelineShaderStageCreateInfo::builder()
        .stage(ShaderStageFlags::FRAGMENT)
        .module(&frag_shader_module)
        .name(fn_name)
        .build();

    let binding_descriptions = [Vertex::binding_description()];
    let attribute_descriptions = Vertex::attribute_descriptions();

    let vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(&binding_descriptions[..])
        .vertex_attribute_descriptions(&attribute_descriptions[..])
        .build();

    let input_assembly = PipelineInputAssemblyStateCreateInfo::builder()
        // .sType(STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO)
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
        // .topology(PRIMITIVE_TOPOLOGY_TRIANGLE_LIST)
        .topology(PrimitiveTopology::TriangleList)
        .primitive_restart_enable(false)
        .build();

    let viewport = Viewport::builder()
        .x(0.0f32)
        .y(0.0f32)
        .width(swap_chain_extent.width() as f32)
        .height(swap_chain_extent.height() as f32)
        .min_depth(0.0f32)
        .max_depth(1.0f32)
        .build();

    let scissor = Rect2d::builder()
        .offset(Offset2d::builder().x(0).y(0).build())
        .extent(swap_chain_extent)
        .build();

    let viewports = [viewport];
    let scissors = [scissor];

    let viewport_state = PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports[..])
        .scissors(&scissors[..])
        .build();

    let rasterizer = PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(PolygonMode::Fill)
        .cull_mode(CullModeFlags::NONE)
        .front_face(FrontFace::CounterClockwise)
        .depth_bias_enable(false)
        .depth_bias_constant_factor(0.0f32)
        .depth_bias_clamp(0.0f32)
        .depth_bias_slope_factor(0.0f32)
        .line_width(1.0f32)
        .build();

    let multisampling = PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(SampleCountFlags::COUNT_1)
        .sample_shading_enable(false)
        .min_sample_shading(1.0f32)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build();

    let stencil_op_state = StencilOpState::builder()
        .fail_op(StencilOp::Keep)
        .pass_op(StencilOp::Keep)
        .depth_fail_op(StencilOp::Keep)
        .compare_op(CompareOp::Never)
        .compare_mask(0)
        .write_mask(0)
        .reference(0)
        .build();

    let depth_stencil = PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(CompareOp::Less)
        .depth_bounds_test_enable(false)
        .stencil_test_enable(false)
        .front(stencil_op_state.clone())
        .back(stencil_op_state)
        .min_depth_bounds(0.0)
        .max_depth_bounds(1.0)
        .build();

    let color_blend_attachment = PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .src_color_blend_factor(BlendFactor::One)
        .dst_color_blend_factor(BlendFactor::Zero)
        .color_blend_op(BlendOp::Add)
        .src_alpha_blend_factor(BlendFactor::One)
        .dst_alpha_blend_factor(BlendFactor::Zero)
        .alpha_blend_op(BlendOp::Add)
        .color_write_mask(ColorComponentFlags::R | ColorComponentFlags::G |
            ColorComponentFlags::B | ColorComponentFlags::A)
        .build();

    // ///////////////////////////////////////////////
    // /////////// KEEPME (ALPHA BLENDING) ///////////
    // let color_blend_attachment = PipelineColorBlendAttachmentState::builder()
    //     blendEnable(false)
    //     srcColorBlendFactor(BLEND_FACTOR_SRC_ALPHA)
    //     dstColorBlendFactor(BLEND_FACTOR_ONE_MINUS_SRC_ALPHA)
    //     colorBlendOp(BLEND_OP_ADD)
    //     srcAlphaBlendFactor(BLEND_FACTOR_ONE)
    //     dstAlphaBlendFactor(BLEND_FACTOR_ZERO)
    //     alphaBlendOp(BLEND_OP_ADD)
    //     colorWriteMask(COLOR_COMPONENT_R_BIT | COLOR_COMPONENT_G_BIT | COLOR_COMPONENT_B_BIT | COLOR_COMPONENT_A_BIT)
    // }; ////////////////////////////////////////////
    // ///////////////////////////////////////////////

    let attachments = [color_blend_attachment];

    let color_blending = PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(LogicOp::Copy)
        .attachments(&attachments)
        .blend_constants([0.0f32; 4])
        .build();

    // ///////////////////////////////////////////////
    // /////////// KEEPME (DYNAMIC STATES) ///////////
    // let dynamic_states = [DYNAMIC_STATE_VIEWPORT) DYNAMIC_STATE_LINE_WIDTH];
    // let dynamic_state = PipelineDynamicStateCreateInfo::builder()
    //     sType(STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO)
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
    let queue_family_idx = voo::queue_families(surface, device.physical_device(),
        queue_family_flags)?.family_idxs()[0] as u32;

    CommandPool::builder()
        .queue_family_index(queue_family_idx)
        .build(device)
}

pub fn create_framebuffers(device: &Device, render_pass: &RenderPass,
        swapchain_image_views: &[ImageView], depth_image_view: &ImageView,
        swapchain_extent: Extent2d) -> VooResult<Vec<Framebuffer>> {
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
        -> VooResult<CommandBuffer> {
    let command_buffer = command_pool.allocate_command_buffer(CommandBufferLevel::Primary)?;
    command_buffer.begin(CommandBufferUsageFlags::ONE_TIME_SUBMIT)?;
    Ok(command_buffer)
}

fn end_single_time_commands(device: &Device, command_pool: &CommandPool,
        command_buffer: CommandBuffer) -> VooResult<()> {
    command_buffer.end()?;

    let command_buffers = [command_buffer.handle()];
    let submit_info = SubmitInfo::builder()
        .command_buffers(&command_buffers[..])
        .build();
    let cmd_buf_handles = [command_buffer.handle().to_raw()];

    unsafe { device.queue_submit(device.queue(0), &[submit_info], None)?; }
    device.queue_wait_idle(device.queue(0));

    Ok(())
}

fn has_stencil_component(format: Format) -> bool {
    format == Format::D32SfloatS8Uint || format == Format::D24UnormS8Uint
}

fn transition_image_layout(device: &Device, command_pool: &CommandPool, image: &Image,
        format: Format, old_layout: ImageLayout, new_layout: ImageLayout)
         -> VooResult<()> {
    let command_buffer = begin_single_time_commands(command_pool)?;

    let subresource_range = ImageSubresourceRange::builder()
        .aspect_mask(ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let mut barrier = ImageMemoryBarrier::builder()
        .src_access_mask(AccessFlags::empty())
        .dst_access_mask(AccessFlags::empty())
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(voo::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(voo::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(subresource_range)
        .build();

    if new_layout == ImageLayout::DepthStencilAttachmentOptimal {
        barrier.subresource_range_mut().set_aspect_mask(ImageAspectFlags::DEPTH);
        if has_stencil_component(format) {
            let aspect_mask = barrier.subresource_range().aspect_mask() |
                ImageAspectFlags::STENCIL;
            barrier.subresource_range_mut().set_aspect_mask(aspect_mask);
        }
    } else {
        barrier.subresource_range_mut().set_aspect_mask(ImageAspectFlags::COLOR);
    }

    let source_stage: PipelineStageFlags;
    let destination_stage: PipelineStageFlags;

    if old_layout == ImageLayout::Undefined &&
            new_layout == ImageLayout::TransferDstOptimal
    {
        barrier.set_src_access_mask(AccessFlags::empty());
        barrier.set_dst_access_mask(AccessFlags::TRANSFER_WRITE);
        source_stage = PipelineStageFlags::TOP_OF_PIPE;
        destination_stage = PipelineStageFlags::TRANSFER;
    } else if old_layout == ImageLayout::TransferDstOptimal &&
            new_layout == ImageLayout::ShaderReadOnlyOptimal
    {
        barrier.set_src_access_mask(AccessFlags::TRANSFER_WRITE);
        barrier.set_dst_access_mask(AccessFlags::SHADER_READ);
        source_stage = PipelineStageFlags::TRANSFER;
        destination_stage = PipelineStageFlags::FRAGMENT_SHADER;
    } else if old_layout == ImageLayout::Undefined &&
            new_layout == ImageLayout::DepthStencilAttachmentOptimal
        {
        barrier.set_src_access_mask(AccessFlags::empty());
        barrier.set_dst_access_mask(AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ |
            AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);
        source_stage = PipelineStageFlags::TOP_OF_PIPE;
        destination_stage = PipelineStageFlags::EARLY_FRAGMENT_TESTS;
    } else {
        panic!("unsupported layout transition");
    }

    unsafe {
        device.cmd_pipeline_barrier(command_buffer.handle(),
            source_stage, destination_stage, DependencyFlags::empty(),
            &[], &[], &[barrier]);
    }

    end_single_time_commands(device, command_pool, command_buffer)
}

fn copy_buffer_to_image(device: &Device, command_pool: &CommandPool, buffer: &Buffer,
        image: &Image, width: u32, height: u32)  -> VooResult<()> {
    let command_buffer = begin_single_time_commands(command_pool)?;

    let image_subresource_layers = ImageSubresourceLayers::builder()
        .aspect_mask(ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let region = BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(image_subresource_layers)
        .image_offset(Offset3d::builder().x(0).y(0).z(0).build())
        .image_extent(Extent3d::builder().width(width).height(height).depth(1).build())
        .build();

    unsafe {
        device.cmd_copy_buffer_to_image(command_buffer.handle(), buffer.handle(), image.handle(),
            ImageLayout::TransferDstOptimal, &[region]);
    }

    end_single_time_commands(device, command_pool, command_buffer)
}

fn copy_buffer(device: &Device, command_pool: &CommandPool, src_buffer: &Buffer,
        dst_buffer: &Buffer, size: DeviceSize)  -> VooResult<()> {
    // TODO: Look into creating a separate command pool with the
    // `VK_COMMAND_POOL_CREATE_TRANSIENT_BIT` flag for short lived command
    // buffers like this.
    let command_buffer = begin_single_time_commands(command_pool)?;

    let copy_region = BufferCopy::builder()
        .src_offset(0)
        .dst_offset(0)
        .size(size)
        .build();

    unsafe { device.cmd_copy_buffer(command_buffer.handle(), src_buffer.handle(),
            dst_buffer.handle(), &[copy_region]); }

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
        .usage(BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(SharingMode::Exclusive)
        .build(device.clone())?;


    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT)?;
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, buffer_bytes, MemoryMapFlags::empty())?;
    data.copy_from_slice(vertices);
    staging_buffer_memory.unmap(data);

    let vertex_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(BufferUsageFlags::TRANSFER_DST | BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = vertex_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::DEVICE_LOCAL)?;
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
        .usage(BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::HOST_VISIBLE)?;
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, buffer_bytes, MemoryMapFlags::empty())?;
    data.copy_from_slice(indices);
    staging_buffer_memory.unmap(data);

    let index_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(BufferUsageFlags::TRANSFER_DST | BufferUsageFlags::INDEX_BUFFER)
        .sharing_mode(SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = index_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::DEVICE_LOCAL)?;
    let index_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    index_buffer.bind_memory(&index_buffer_memory, 0)?;

    copy_buffer(device, command_pool, &staging_buffer, &index_buffer, buffer_bytes)?;

    Ok((index_buffer, index_buffer_memory))
}

fn create_uniform_buffer(device: &Device, command_pool: &CommandPool, _extent: Extent2d)
        -> VooResult<(Buffer, DeviceMemory)> {
    let buffer_bytes = mem::size_of::<UniformBufferObject>() as u64;
    let uniform_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(BufferUsageFlags::UNIFORM_BUFFER)
        .sharing_mode(SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = uniform_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT)?;
    let uniform_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    uniform_buffer.bind_memory(&uniform_buffer_memory, 0)?;

    Ok((uniform_buffer, uniform_buffer_memory))
}

fn create_depth_resources(device: &Device, command_pool: &CommandPool,
        swapchain_extent: Extent2d) -> VooResult<(Image, DeviceMemory, ImageView)> {
    let depth_format = find_depth_format(device)?;
    let extent = Extent3d::builder()
        .width(swapchain_extent.width())
        .height(swapchain_extent.height())
        .depth(1)
        .build();

    let depth_image = Image::builder()
        .image_type(ImageType::Type2d)
        .format(depth_format)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(SampleCountFlags::COUNT_1)
        .tiling(ImageTiling::Optimal)
        .usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        .sharing_mode(SharingMode::Exclusive)
        .initial_layout(ImageLayout::Undefined)
        .build(device.clone())?;

    let memory_requirements = depth_image.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::DEVICE_LOCAL)?;
    let depth_image_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    depth_image.bind_memory(&depth_image_memory, 0)?;

    let depth_image_view = ImageView::builder()
        .image(depth_image.handle())
        .view_type(ImageViewType::Type2d)
        .format(depth_format)
        .components(ComponentMapping::default())
        .subresource_range(ImageSubresourceRange::builder()
            .aspect_mask(ImageAspectFlags::DEPTH)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build())
        .build(device.clone(), None)?;

    transition_image_layout(device, command_pool, &depth_image, depth_format,
        ImageLayout::Undefined, ImageLayout::DepthStencilAttachmentOptimal)?;

    Ok((depth_image, depth_image_memory, depth_image_view))
}

fn create_texture_image(device: &Device, command_pool: &CommandPool)
        -> VooResult<(Image, DeviceMemory)> {
    let pixels = image::open(TEXTURE_PATH).unwrap().to_rgba();
    let (tex_width, tex_height) = pixels.dimensions();
    let image_bytes = (tex_width * tex_height * 4) as u64;

    let staging_buffer = Buffer::builder()
        .size(image_bytes)
        .usage(BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(SharingMode::Exclusive)
        .build(device.clone())?;

    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT)?;
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, image_bytes, MemoryMapFlags::empty())?;
    data.copy_from_slice(&pixels);
    staging_buffer_memory.unmap(data);

    let extent = Extent3d::builder().width(tex_width).height(tex_height).depth(1).build();

    let texture_image = Image::builder()
        .image_type(ImageType::Type2d)
        .format(Format::R8G8B8A8Unorm)
        .extent(extent.clone())
        .mip_levels(1)
        .array_layers(1)
        .samples(SampleCountFlags::COUNT_1)
        .tiling(ImageTiling::Optimal)
        .usage(ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED)
        .sharing_mode(SharingMode::Exclusive)
        .initial_layout(ImageLayout::Undefined)
        .build(device.clone())?;

    let memory_requirements = texture_image.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memory_type_bits(),
        MemoryPropertyFlags::DEVICE_LOCAL)?;
    let texture_image_memory = DeviceMemory::new(device.clone(), memory_requirements.size(),
        memory_type_index)?;
    texture_image.bind_memory(&texture_image_memory, 0)?;

    transition_image_layout(device, command_pool, &texture_image, Format::R8G8B8A8Unorm,
        ImageLayout::Undefined, ImageLayout::TransferDstOptimal)?;

    copy_buffer_to_image(device, command_pool, &staging_buffer, &texture_image,
        extent.width(), extent.height())?;

    transition_image_layout(device, command_pool, &texture_image, Format::R8G8B8A8Unorm,
        ImageLayout::TransferDstOptimal, ImageLayout::ShaderReadOnlyOptimal)?;

    Ok((texture_image, texture_image_memory))
}

fn create_texture_image_view(device: Device, image: &Image) -> VooResult<ImageView> {
    ImageView::builder()
        .image(image.handle())
        .view_type(ImageViewType::Type2d)
        .format(Format::R8G8B8A8Unorm)
        .components(ComponentMapping::default())
        .subresource_range(ImageSubresourceRange::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build())
        .build(device, None)
}

fn create_texture_sampler(device: Device) -> VooResult<Sampler> {
    Sampler::builder()
        .mag_filter(Filter::Linear)
        .min_filter(Filter::Linear)
        .mipmap_mode(SamplerMipmapMode::Linear)
        .address_mode_u(SamplerAddressMode::Repeat)
        .address_mode_v(SamplerAddressMode::Repeat)
        .address_mode_w(SamplerAddressMode::Repeat)
        .mip_lod_bias(0.)
        .anisotropy_enable(true)
        .max_anisotropy(16.)
        .compare_enable(false)
        .compare_op(CompareOp::Always)
        .min_lod(0.)
        .max_lod(0.)
        .border_color(BorderColor::IntOpaqueBlack)
        .unnormalized_coordinates(false)
        .build(device)
}

pub fn create_command_buffers(device: &Device, command_pool: &CommandPool,
        render_pass: &RenderPass, graphics_pipeline: &GraphicsPipeline,
        swapchain_framebuffers: &[Framebuffer], swapchain_extent: &Extent2d,
        vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: u32,
        index_count: u32, pipeline_layout: &PipelineLayout,
        descriptor_set: DescriptorSet)
        -> VooResult<SmallVec<[CommandBuffer; 16]>>
{
    let command_buffers = command_pool.allocate_command_buffers(CommandBufferLevel::Primary,
            swapchain_framebuffers.len() as u32)?;

    let vertex_buffers = [vertex_buffer];
    let offsets = [0];
    let descriptor_sets = [&descriptor_set];

    for (cmd_buf, swapchain_framebuffer) in command_buffers.iter()
            .zip(swapchain_framebuffers.iter())
    {
        let begin_info = CommandBufferBeginInfo::builder()
            // * COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: The command buffer
            //   will be rerecorded right after executing it once.
            // * COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: This is a
            //   secondary command buffer that will be entirely within a
            //   single render pass.
            // * COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: The command buffer
            //   can be resubmitted while it is also already pending
            //   execution.
            .flags(CommandBufferUsageFlags::SIMULTANEOUS_USE)
            .build();

        unsafe {
            device.begin_command_buffer(cmd_buf.handle(), &begin_info)?;
        }

        let clear_values = &[
            ClearValue { color: ClearColorValue {
                float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32] } },
            ClearValue { depthStencil: vks::VkClearDepthStencilValue {
                depth: 1.0, stencil: 0, } },
        ];

        let render_pass_info = RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(swapchain_framebuffer)
            .render_area(Rect2d::builder()
                .offset(Offset2d::builder().x(0).y(0).build())
                .extent(swapchain_extent.clone())
                .build())
            .clear_values(clear_values)
            .build();

        cmd_buf.begin_render_pass(&render_pass_info, SubpassContents::Inline);
        cmd_buf.bind_pipeline(PipelineBindPoint::Graphics, &graphics_pipeline);
        cmd_buf.bind_vertex_buffers(0, &vertex_buffers, &offsets);
        cmd_buf.bind_index_buffer(index_buffer, 0, IndexType::Uint32);
        cmd_buf.bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline_layout,
            0, &descriptor_sets, &[]);

        cmd_buf.draw_indexed(index_count, 1, 0, 0, 0);

        cmd_buf.end_render_pass();
        cmd_buf.end()?;
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
    vert_shader_code: Vec<u32>,
    frag_shader_code: Vec<u32>,
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
    descriptor_sets: SmallVec<[DescriptorSet; 8]>,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    start_time: time::Instant,
    swapchain: Option<SwapchainKhr>,
    swapchain_components: Option<SwapchainComponents>,
    command_buffers: Option<SmallVec<[CommandBuffer; 16]>>,
    command_buffer_handles: Option<SmallVec<[CommandBufferHandle; 16]>>,
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
        let vert_shader_code = util::read_spir_v_file("/src/voodoo/shaders/vert.spv")?;
        let frag_shader_code = util::read_spir_v_file("/src/voodoo/shaders/frag.spv")?;
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
        let image_available_semaphore = Semaphore::new(device.clone(),
            SemaphoreCreateFlags::empty())?;
        let render_finished_semaphore = Semaphore::new(device.clone(),
            SemaphoreCreateFlags::empty())?;
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

        let command_buffer_handles = command_buffers.iter().map(|cb| cb.handle()).collect();

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
            command_buffer_handles: Some(command_buffer_handles),
        })
    }

    fn cleanup_swapchain(&mut self) {
        self.swapchain = None;
        self.swapchain_components = None;
        self.command_buffers = None;
    }

    fn recreate_swapchain(&mut self, current_extent: Extent2d) -> VooResult<()> {
        self.device.wait_idle();

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
        let command_buffer_handles = command_buffers.iter().map(|cb| cb.handle()).collect();

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
        self.command_buffer_handles = Some(command_buffer_handles);

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
            mem::size_of::<UniformBufferObject>() as u64, MemoryMapFlags::empty())?;
        data.copy_from_slice(&[ubo]);
        self.uniform_buffer_memory.unmap(data);

        Ok(())
    }

    fn draw_frame(&mut self) -> VooResult<()> {
        let image_index = unsafe {
            match self.device.acquire_next_image_khr(
                    self.swapchain.as_ref().unwrap().handle(), u64::max_value(),
                    Some(self.image_available_semaphore.handle()), None, 0) {
                Ok(idx) => idx,
                Err(res) => {
                    if let ErrorKind::ApiCall(call_res, _fn_name) = res.kind {
                        if call_res == CallResult::ErrorOutOfDateKhr {
                            let dims = self.window.get_inner_size_pixels().unwrap();
                            self.recreate_swapchain(Extent2d::builder()
                                .height(dims.0).width(dims.1).build())?;
                            return Ok(());
                        } else {
                            panic!("Unable to present swap chain image");
                        }
                    } else {
                        panic!("Unable to present swap chain image");
                    }
                }
            }
        };

        let wait_semaphores = [self.image_available_semaphore.handle()];
        let wait_stages = PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let signal_semaphores = [self.render_finished_semaphore.handle()];
        let command_buffer_handles = [self.command_buffer_handles.as_ref().unwrap()
            .get(image_index as usize).unwrap().clone()];

        let submit_info = SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores[..])
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffer_handles[..])
            .signal_semaphores(&signal_semaphores[..])
            .build();

        unsafe {
            self.device.queue_submit(self.device.queue(0), &[submit_info], None)?;
        }

        let swapchains = [self.swapchain.as_ref().unwrap().handle()];
        let image_indices = [image_index];

        let present_info = PresentInfoKhr::builder()
            .wait_semaphores(&signal_semaphores[..])
            .swapchains(&swapchains[..])
            .image_indices(&image_indices)
            .build();

        unsafe {
            self.device.queue_present_khr(self.device.queue(0), &present_info)?;
            self.device.queue_wait_idle(self.device.queue(0));
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
                        current_extent = Extent2d::builder().width(w).height(h).build();
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

        self.device.wait_idle();
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("Goodbye triangle.");
    }
}


fn main() {
    println!("Hello triangle!");
    unsafe {
        let mut app = App::new().unwrap();
        app.main_loop().unwrap();
    }
}