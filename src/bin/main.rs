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
use std::ffi::CStr;
use std::cmp;
use libc::c_char;
use smallvec::SmallVec;
use image::{ImageFormat, DynamicImage};
use cgmath::{SquareMatrix, One, Rotation, Rotation3, Basis3, Matrix3, Matrix4, Vector3};
use voo::winit::{EventsLoop, WindowBuilder, Window, Event, WindowEvent};
use voo::{voodoo_winit, vks, util, device, queue, Result as VooResult, Version, Instance, Device,
    Surface, Swapchain,
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


fn main() {
    println!("Hello triangle!");
    unsafe {
        let mut app = App::new().unwrap();
        app.main_loop().unwrap();
    }
}

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
    let app_info = voo::ApplicationInfo::new()
        .application_name("Hello Triangle")
        .application_version((1, 0, 0))
        .engine_name("No Engine")
        .engine_version((1, 0, 0))
        .api_version((1, 0, 0));

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
fn device_is_suitable(instance: &Instance, surface: &Surface,
        physical_device: &PhysicalDevice, queue_flags: QueueFlags) -> bool {
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

    queue::queue_families(instance, surface, &physical_device, queue_flags).is_complete() &&
        extensions_supported &&
        swap_chain_adequate &&
        device_features.samplerAnisotropy != 0
}

/// Returns a physical device from the list of available physical devices if
/// it meets the criteria specified in the above function.
fn choose_physical_device(instance: &Instance, surface: &Surface, queue_flags: QueueFlags)
        -> VooResult<PhysicalDevice> {
    let mut preferred_device = None;

    for device in instance.physical_devices() {
        if device_is_suitable(instance, surface, &device, queue_flags) {
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

fn create_device(instance: Instance, surface: &Surface, physical_device: PhysicalDevice,
        queue_familiy_flags: QueueFlags) -> VooResult<Device> {
    let queue_family_idx = queue::queue_families(&instance, surface,
        &physical_device, queue_familiy_flags).family_idxs()[0] as u32;

    let queue_create_info = voo::DeviceQueueCreateInfo::new()
        .queue_family_index(queue_family_idx)
        .queue_priorities(&[1.0]);

    let features = PhysicalDeviceFeatures::new()
        .sampler_anisotropy(true);

    Device::builder()
        .queue_create_infos(&[queue_create_info.clone()])
        .enabled_extension_names(REQUIRED_DEVICE_EXTENSIONS)
        .enabled_features(&features)
        .build(physical_device)
}

fn choose_swap_surface_format(available_formats: &[vks::khr_surface::VkSurfaceFormatKHR])
        -> vks::khr_surface::VkSurfaceFormatKHR {
    if available_formats.len() == 1 && available_formats[0].format == vks::VK_FORMAT_UNDEFINED {
        return vks::khr_surface::VkSurfaceFormatKHR {
            format: vks::VK_FORMAT_B8G8R8A8_UNORM,
            colorSpace: vks::khr_surface::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        };
    }
    for available_format in available_formats {
        if available_format.format == Format::B8G8R8A8Unorm as u32 &&
                available_format.colorSpace ==
                vks::khr_surface::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR {
            return vks::khr_surface::VkSurfaceFormatKHR {
                format: vks::VK_FORMAT_B8G8R8A8_UNORM,
                colorSpace: vks::khr_surface::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            };
        }
    }
    vks::khr_surface::VkSurfaceFormatKHR {
        format: available_formats[0].format,
        colorSpace: available_formats[0].colorSpace,
    }
}

fn choose_swap_present_mode(available_present_modes: &[vks::khr_surface::VkPresentModeKHR])
        -> vks::khr_surface::VkPresentModeKHR {
    let mut best_mode = vks::khr_surface::VK_PRESENT_MODE_MAILBOX_KHR;
    for &available_present_mode in available_present_modes {
        if available_present_mode == vks::khr_surface::VK_PRESENT_MODE_FIFO_KHR {
            return available_present_mode;
        } else if available_present_mode == vks::khr_surface::VK_PRESENT_MODE_IMMEDIATE_KHR {
            best_mode = available_present_mode;
        }
    }
    best_mode
}

fn choose_swap_extent(capabilities: &vks::khr_surface::VkSurfaceCapabilitiesKHR,
        window_size: Option<vks::VkExtent2D>) -> vks::VkExtent2D {
    if capabilities.currentExtent.width != u32::max_value() {
        return capabilities.currentExtent.clone();
    } else {
        let mut actual_extent = window_size
            .unwrap_or(vks::VkExtent2D { width: 1024, height: 768 });
        actual_extent.width = cmp::max(capabilities.minImageExtent.width,
            cmp::min(capabilities.maxImageExtent.width, actual_extent.width));
        actual_extent.height = cmp::max(capabilities.minImageExtent.height,
            cmp::min(capabilities.maxImageExtent.height, actual_extent.height));
        return actual_extent
    }
}

fn create_swapchain(surface: Surface, device: Device, queue_flags: QueueFlags,
        window_size: Option<vks::VkExtent2D>, old_swapchain: Option<Swapchain>)
        -> VooResult<Swapchain> {
    let swapchain_details: SwapchainSupportDetails = SwapchainSupportDetails::new(
        device.instance(), &surface, device.physical_device());
    let surface_format = choose_swap_surface_format(&swapchain_details.formats);
    let present_mode = choose_swap_present_mode(&swapchain_details.present_modes);
    let extent = choose_swap_extent(&swapchain_details.capabilities, window_size);

    // TODO: REVISIT THIS: https://vulkan-tutorial.com/Drawing_a_triangle/Presentation/Swap_chain
    let mut image_count = swapchain_details.capabilities.minImageCount + 1;
    if swapchain_details.capabilities.maxImageCount > 0 &&
            image_count > swapchain_details.capabilities.maxImageCount {
        image_count = swapchain_details.capabilities.maxImageCount;
    }
    let indices = queue::queue_families(device.instance(), &surface,
        device.physical_device(), queue_flags);
    let queue_family_indices = [indices.flag_idxs[0] as u32,
        indices.presentation_support_idxs[0] as u32];

    let mut bldr = Swapchain::builder();
    bldr.surface(&surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.colorSpace)
        .image_extent(extent.clone())
        .image_array_layers(1)
        .image_usage(vks::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT)
        .pre_transform(swapchain_details.capabilities.currentTransform)
        .composite_alpha(vks::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(old_swapchain.map(|sc| sc.handle()).unwrap_or(0));

    if queue_family_indices[0] != queue_family_indices[1] {
        bldr.image_sharing_mode(vks::VK_SHARING_MODE_CONCURRENT);
        bldr.queue_family_indices(&queue_family_indices[..]);
    } else {
        bldr.image_sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE);
    }
    bldr.build(device)
}

pub fn create_image_views(swapchain: &Swapchain) -> VooResult<Vec<ImageView>> {
    swapchain.images().iter().map(|&image| {
        ImageView::builder()
            .image(image)
            .view_type(vks::VK_IMAGE_VIEW_TYPE_2D)
            .format(swapchain.image_format())
            .components(vks::VkComponentMapping::default())
            .subresource_range(vks::VkImageSubresourceRange {
                aspectMask: vks::VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            })
            .build(swapchain.device().clone(), Some(swapchain.clone()))

    }).collect::<Result<Vec<_>, _>>()
}

fn find_depth_format(device: &Device) -> VooResult<vks::VkFormat> {
    find_supported_format(device, &[vks::VK_FORMAT_D32_SFLOAT, vks::VK_FORMAT_D32_SFLOAT_S8_UINT,
        vks::VK_FORMAT_D24_UNORM_S8_UINT], vks::VK_IMAGE_TILING_OPTIMAL,
        vks::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT)
}

fn create_render_pass(device: Device, swapchain_image_format: vks::VkFormat)
        -> VooResult<RenderPass> {
    let depth_image_format = find_depth_format(&device)?;

    let color_attachment = vks::VkAttachmentDescription {
        flags: 0,
        format: swapchain_image_format,
        samples: vks::VK_SAMPLE_COUNT_1_BIT,
        loadOp: vks::VK_ATTACHMENT_LOAD_OP_CLEAR,
        storeOp: vks::VK_ATTACHMENT_STORE_OP_STORE,
        stencilLoadOp: vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        stencilStoreOp: vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
        initialLayout: vks::VK_IMAGE_LAYOUT_UNDEFINED,
        finalLayout: vks::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
    };

    let depth_attachment = vks::VkAttachmentDescription {
        flags: 0,
        format: depth_image_format,
        samples: vks::VK_SAMPLE_COUNT_1_BIT,
        loadOp: vks::VK_ATTACHMENT_LOAD_OP_CLEAR,
        storeOp: vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
        stencilLoadOp: vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        stencilStoreOp: vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
        initialLayout: vks::VK_IMAGE_LAYOUT_UNDEFINED,
        finalLayout: vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    };

    let color_attachment_ref = vks::VkAttachmentReference {
        attachment: 0,
        layout: vks::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    };

    let depth_attachment_ref = vks::VkAttachmentReference {
        attachment: 1,
        layout: vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    };

    let subpass = vks::VkSubpassDescription {
        flags: 0,
        pipelineBindPoint: vks::VK_PIPELINE_BIND_POINT_GRAPHICS,
        inputAttachmentCount: 0,
        pInputAttachments: ptr::null(),
        colorAttachmentCount: 1,
        pColorAttachments: &color_attachment_ref,
        pResolveAttachments: ptr::null(),
        pDepthStencilAttachment: &depth_attachment_ref,
        preserveAttachmentCount: 0,
        pPreserveAttachments: ptr::null(),
    };

    let dependency = vks::VkSubpassDependency {
        dependencyFlags: 0,
        srcSubpass: vks::VK_SUBPASS_EXTERNAL,
        dstSubpass: 0,
        srcStageMask: vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        srcAccessMask: 0,
        dstStageMask: vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        dstAccessMask: vks::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT | vks::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
    };

    RenderPass::builder()
        .attachments(&[color_attachment, depth_attachment])
        .subpasses(&[subpass])
        .dependencies(&[dependency])
        .build(device)
}

fn create_descriptor_set_layout(device: Device) -> VooResult<DescriptorSetLayout> {
    let ubo_layout_binding = vks::VkDescriptorSetLayoutBinding {
        binding: 0,
        descriptorType: vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
        descriptorCount: 1,
        stageFlags: vks::VK_SHADER_STAGE_VERTEX_BIT,
        pImmutableSamplers: ptr::null(),
    };

    let sampler_layout_binding = vks::VkDescriptorSetLayoutBinding {
        binding: 1,
        descriptorType: vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        descriptorCount: 1,
        stageFlags: vks::VK_SHADER_STAGE_FRAGMENT_BIT,
        pImmutableSamplers: ptr::null(),
    };

    let bindings = [ubo_layout_binding, sampler_layout_binding];

    DescriptorSetLayout::builder()
        .bindings(&bindings)
        .build(device)
}

fn create_descriptor_pool(device: Device) -> VooResult<DescriptorPool> {
    let pool_sizes = [
        vks::VkDescriptorPoolSize {
            type_: vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptorCount: 1,
        },
        vks::VkDescriptorPoolSize {
            type_: vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            descriptorCount: 1,
        },
    ];

    DescriptorPool::builder()
        .max_sets(1)
        .pool_sizes(&pool_sizes)
        .build(device)
}

fn create_descriptor_sets(device: &Device, layout: &DescriptorSetLayout,
        pool: &DescriptorPool, uniform_buffer: &Buffer, texture_image_view: &ImageView,
        texture_sampler: &Sampler) -> VooResult<SmallVec<[vks::VkDescriptorSet; 8]>> {
    let descriptor_sets = pool.allocate_descriptor_sets(&[layout][..]);

    let buffer_info = vks::VkDescriptorBufferInfo {
        buffer: uniform_buffer.handle(),
        offset: 0,
        range: mem::size_of::<UniformBufferObject>() as u64,
    };

    let image_info = vks::VkDescriptorImageInfo {
        sampler: texture_sampler.handle(),
        imageView: texture_image_view.handle(),
        imageLayout: vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
    };

    let descriptor_writes = [
        vks::VkWriteDescriptorSet {
            sType: vks::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: descriptor_sets[0],
            dstBinding: 0,
            dstArrayElement: 0,
            descriptorCount: 1,
            descriptorType: vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            pImageInfo: ptr::null(),
            pBufferInfo: &buffer_info,
            pTexelBufferView: ptr::null(),
        },
        vks::VkWriteDescriptorSet {
            sType: vks::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: descriptor_sets[0],
            dstBinding: 1,
            dstArrayElement: 0,
            descriptorCount: 1,
            descriptorType: vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            pImageInfo: &image_info,
            pBufferInfo: ptr::null(),
            pTexelBufferView: ptr::null(),
        },
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
        render_pass: &RenderPass, swap_chain_extent: vks::VkExtent2D, vert_shader_code: &[u8],
        frag_shader_code: &[u8]) -> VooResult<GraphicsPipeline> {
    let vert_shader_module = ShaderModule::new(device.clone(), vert_shader_code)?;
    let frag_shader_module = ShaderModule::new(device.clone(), frag_shader_code)?;

    let fn_name = CStr::from_bytes_with_nul(b"main\0").unwrap();

    let vert_shader_stage_info = vks::VkPipelineShaderStageCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        stage: vks::VK_SHADER_STAGE_VERTEX_BIT,
        module: vert_shader_module.handle(),
        pName: fn_name.as_ptr(),
        pSpecializationInfo: ptr::null(),
    };

    let frag_shader_stage_info = vks::VkPipelineShaderStageCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        stage: vks::VK_SHADER_STAGE_FRAGMENT_BIT,
        module: frag_shader_module.handle(),
        pName: fn_name.as_ptr(),
        pSpecializationInfo: ptr::null(),
    };

    let shader_stages = [vert_shader_stage_info, frag_shader_stage_info];

    let binding_description = Vertex::binding_description();
    let attribute_descriptions = Vertex::attribute_descriptions();

    let vertex_input_info = vks::VkPipelineVertexInputStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        vertexBindingDescriptionCount: 1,
        pVertexBindingDescriptions: &binding_description,
        vertexAttributeDescriptionCount: attribute_descriptions.len() as u32,
        pVertexAttributeDescriptions: attribute_descriptions.as_ptr(),
    };

    let input_assembly = vks::VkPipelineInputAssemblyStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
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
        topology: vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        primitiveRestartEnable: vks::VK_FALSE,
    };

    let viewport = vks::VkViewport {
        x: 0.0f32,
        y: 0.0f32,
        width: swap_chain_extent.width as f32,
        height: swap_chain_extent.height as f32,
        minDepth: 0.0f32,
        maxDepth: 1.0f32,
    };

    let scissor = vks::VkRect2D {
        offset: vks::VkOffset2D {
            x: 0,
            y: 0,
        },
        extent: swap_chain_extent,
    };

    let viewport_state = vks::VkPipelineViewportStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        viewportCount: 1,
        pViewports: &viewport,
        scissorCount: 1,
        pScissors: &scissor,
    };

    let rasterizer = vks::VkPipelineRasterizationStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        depthClampEnable: vks::VK_FALSE,
        rasterizerDiscardEnable: vks::VK_FALSE,
        polygonMode: vks::VK_POLYGON_MODE_FILL,
        // cullMode: vks::VK_CULL_MODE_BACK_BIT,
        cullMode: vks::VK_CULL_MODE_NONE,
        // frontFace: vks::VK_FRONT_FACE_CLOCKWISE,
        frontFace: vks::VK_FRONT_FACE_COUNTER_CLOCKWISE,
        depthBiasEnable: vks::VK_FALSE,
        depthBiasConstantFactor: 0.0f32,
        depthBiasClamp: 0.0f32,
        depthBiasSlopeFactor: 0.0f32,
        lineWidth: 1.0f32,
    };

    let multisampling = vks::VkPipelineMultisampleStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        rasterizationSamples: vks::VK_SAMPLE_COUNT_1_BIT,
        sampleShadingEnable: vks::VK_FALSE,
        minSampleShading: 1.0f32,
        pSampleMask: ptr::null(),
        alphaToCoverageEnable: vks::VK_FALSE,
        alphaToOneEnable: vks::VK_FALSE,
    };

    let stencil_op_state = vks::VkStencilOpState {
        failOp: 0,
        passOp: 0,
        depthFailOp: 0,
        compareOp: 0,
        compareMask: 0,
        writeMask: 0,
        reference: 0,
    };

    let depth_stencil = vks::VkPipelineDepthStencilStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        depthTestEnable: vks::VK_TRUE,
        depthWriteEnable: vks::VK_TRUE,
        depthCompareOp: vks::VK_COMPARE_OP_LESS,
        depthBoundsTestEnable: vks::VK_FALSE,
        stencilTestEnable: vks::VK_FALSE,
        front: stencil_op_state.clone(),
        back: stencil_op_state,
        minDepthBounds: 0.0,
        maxDepthBounds: 1.0,
    };

    let color_blend_attachment = vks::VkPipelineColorBlendAttachmentState {
        blendEnable: vks::VK_FALSE,
        srcColorBlendFactor: vks::VK_BLEND_FACTOR_ONE,
        dstColorBlendFactor: vks::VK_BLEND_FACTOR_ZERO,
        colorBlendOp: vks::VK_BLEND_OP_ADD,
        srcAlphaBlendFactor: vks::VK_BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: vks::VK_BLEND_FACTOR_ZERO,
        alphaBlendOp: vks::VK_BLEND_OP_ADD,
        colorWriteMask: vks::VK_COLOR_COMPONENT_R_BIT | vks::VK_COLOR_COMPONENT_G_BIT |
            vks::VK_COLOR_COMPONENT_B_BIT | vks::VK_COLOR_COMPONENT_A_BIT,
    };

    // ///////////////////////////////////////////////
    // /////////// KEEPME (ALPHA BLENDING) ///////////
    // let color_blend_attachment = vks::VkPipelineColorBlendAttachmentState {
    //     blendEnable: vks::VK_FALSE,
    //     srcColorBlendFactor: vks::VK_BLEND_FACTOR_SRC_ALPHA,
    //     dstColorBlendFactor: vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
    //     colorBlendOp: vks::VK_BLEND_OP_ADD,
    //     srcAlphaBlendFactor: vks::VK_BLEND_FACTOR_ONE,
    //     dstAlphaBlendFactor: vks::VK_BLEND_FACTOR_ZERO,
    //     alphaBlendOp: vks::VK_BLEND_OP_ADD,
    //     colorWriteMask: vks::VK_COLOR_COMPONENT_R_BIT | vks::VK_COLOR_COMPONENT_G_BIT | vks::VK_COLOR_COMPONENT_B_BIT | vks::VK_COLOR_COMPONENT_A_BIT,
    // }; ////////////////////////////////////////////
    // ///////////////////////////////////////////////

    let color_blending = vks::VkPipelineColorBlendStateCreateInfo {
        sType: vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        logicOpEnable: vks::VK_FALSE,
        logicOp: vks::VK_LOGIC_OP_COPY,
        attachmentCount: 1,
        pAttachments: &color_blend_attachment,
        blendConstants: [0.0f32; 4],
    };

    // ///////////////////////////////////////////////
    // /////////// KEEPME (DYNAMIC STATES) ///////////
    // let dynamic_states = [vks::VK_DYNAMIC_STATE_VIEWPORT, vks::VK_DYNAMIC_STATE_LINE_WIDTH];
    // let dynamic_state = vks::VkPipelineDynamicStateCreateInfo {
    //     sType: vks::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    //     pNext: ptr::null(),
    //     flags: 0,
    //     dynamicStateCount: 2,
    //     pDynamicStates: dynamic_states.as_ptr(),
    // }; ////////////////////////////////////////////
    // ///////////////////////////////////////////////

    GraphicsPipeline::builder()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input_info)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterizer)
        .multisample_state(&multisampling)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blending)
        // .dynamic_state(&dynamic_state)
        .layout(&pipeline_layout)
        .render_pass(&render_pass)
        .subpass(0)
        .base_pipeline_handle(0)
        .base_pipeline_index(-1)
        .build(device)
}

fn create_command_pool(device: Device, surface: &Surface, queue_family_flags: QueueFlags)
        -> VooResult<CommandPool> {
    let queue_family_idx = voo::queue_families(device.instance(), surface,
        device.physical_device(), queue_family_flags).family_idxs()[0] as u32;

    CommandPool::builder()
        .queue_family_index(queue_family_idx)
        .build(device)
}

pub fn create_framebuffers(device: &Device, render_pass: &RenderPass,
        swapchain_image_views: &[ImageView], depth_image_view: &ImageView,
        swapchain_extent: vks::VkExtent2D) -> VooResult<Vec<Framebuffer>> {
    swapchain_image_views.iter().map(|image_view| {
        let attachments = [image_view, depth_image_view];
        Framebuffer::builder()
            .render_pass(&render_pass)
            .attachments(&attachments[..])
            .width(swapchain_extent.width)
            .height(swapchain_extent.height)
            .layers(1)
            .build(device.clone())
    }).collect::<Result<Vec<_>, _>>()
}


fn begin_single_time_commands(device: &Device, command_pool: &CommandPool)
        -> VooResult<vks::VkCommandBuffer> {
    let alloc_info = vks::VkCommandBufferAllocateInfo {
        sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: ptr::null(),
        commandPool: command_pool.handle(),
        level: vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: 1,
    };

    let mut command_buffer = ptr::null_mut();
    unsafe {
        voo::check(device.proc_addr_loader().core.vkAllocateCommandBuffers(device.handle(),
            &alloc_info, &mut command_buffer));
    }

    let begin_info = vks::VkCommandBufferBeginInfo {
        sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        pNext: ptr::null(),
        flags: vks::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        pInheritanceInfo: ptr::null(),
    };

    unsafe { device.proc_addr_loader().core.vkBeginCommandBuffer(command_buffer, &begin_info); }
    Ok(command_buffer)
}

fn end_single_time_commands(device: &Device, command_pool: &CommandPool,
        command_buffer: vks::VkCommandBuffer) -> VooResult<()> {
    unsafe { voo::check(device.proc_addr_loader().core.vkEndCommandBuffer(command_buffer)); }

    let submit_info = vks::VkSubmitInfo {
        sType: vks::VK_STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: ptr::null(),
        waitSemaphoreCount: 0,
        pWaitSemaphores: ptr::null(),
        pWaitDstStageMask: ptr::null(),
        commandBufferCount: 1,
        pCommandBuffers: &command_buffer,
        signalSemaphoreCount: 0,
        pSignalSemaphores: ptr::null(),
    };

    unsafe {
        voo::check(device.proc_addr_loader().core.vkQueueSubmit(device.queue(0), 1,
            &submit_info, 0));
        voo::check(device.proc_addr_loader().core.vkQueueWaitIdle(device.queue(0)));
        device.proc_addr_loader().core.vkFreeCommandBuffers(device.handle(),
            command_pool.handle(), 1, &command_buffer);
    }

    Ok(())
}

fn has_stencil_component(format: vks::VkFormat) -> bool {
    format == vks::VK_FORMAT_D32_SFLOAT_S8_UINT || format == vks::VK_FORMAT_D24_UNORM_S8_UINT
}

fn transition_image_layout(device: &Device, command_pool: &CommandPool, image: &Image,
        format: vks::VkFormat, old_layout: vks::VkImageLayout, new_layout: vks::VkImageLayout)
         -> VooResult<()> {
    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let subresource_range = vks::VkImageSubresourceRange {
        aspectMask: vks::VK_IMAGE_ASPECT_COLOR_BIT,
        baseMipLevel: 0,
        levelCount: 1,
        baseArrayLayer: 0,
        layerCount: 1,
    };

    let mut barrier = vks::VkImageMemoryBarrier {
        sType: vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
        pNext: ptr::null(),
        srcAccessMask: 0,
        dstAccessMask: 0,
        oldLayout: old_layout,
        newLayout: new_layout,
        srcQueueFamilyIndex: vks::VK_QUEUE_FAMILY_IGNORED,
        dstQueueFamilyIndex: vks::VK_QUEUE_FAMILY_IGNORED,
        image: image.handle(),
        subresourceRange: subresource_range,
    };

    if new_layout == vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL {
        barrier.subresourceRange.aspectMask = vks::VK_IMAGE_ASPECT_DEPTH_BIT;
        if has_stencil_component(format) {
            barrier.subresourceRange.aspectMask |= vks::VK_IMAGE_ASPECT_STENCIL_BIT;
        }
    } else {
        barrier.subresourceRange.aspectMask = vks::VK_IMAGE_ASPECT_COLOR_BIT;
    }

    let source_stage: vks::VkPipelineStageFlags;
    let destination_stage: vks::VkPipelineStageFlags;

    if old_layout == vks::VK_IMAGE_LAYOUT_UNDEFINED &&
            new_layout == vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL
    {
        barrier.srcAccessMask = 0;
        barrier.dstAccessMask = vks::VK_ACCESS_TRANSFER_WRITE_BIT;
        source_stage = vks::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT;
        destination_stage = vks::VK_PIPELINE_STAGE_TRANSFER_BIT;
    } else if old_layout == vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL &&
            new_layout == vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
    {
        barrier.srcAccessMask = vks::VK_ACCESS_TRANSFER_WRITE_BIT;
        barrier.dstAccessMask = vks::VK_ACCESS_SHADER_READ_BIT;
        source_stage = vks::VK_PIPELINE_STAGE_TRANSFER_BIT;
        destination_stage = vks::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT;
    } else if old_layout == vks::VK_IMAGE_LAYOUT_UNDEFINED &&
            new_layout == vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL {
        barrier.srcAccessMask = 0;
        barrier.dstAccessMask = vks::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT |
            vks::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
        source_stage = vks::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT;
        destination_stage = vks::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    } else {
        panic!("unsupported layout transition");
    }

    unsafe {
        device.proc_addr_loader().vkCmdPipelineBarrier(
            command_buffer,
            source_stage, destination_stage,
            0,
            0, ptr::null(),
            0, ptr::null(),
            1, &barrier
        );
    }

    end_single_time_commands(device, command_pool, command_buffer)
}

fn copy_buffer_to_image(device: &Device, command_pool: &CommandPool, buffer: &Buffer,
        image: &Image, width: u32, height: u32)  -> VooResult<()> {
    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let image_subresource_layers = vks::VkImageSubresourceLayers {
        aspectMask: vks::VK_IMAGE_ASPECT_COLOR_BIT,
        mipLevel: 0,
        baseArrayLayer: 0,
        layerCount: 1,
    };

    let region = vks::VkBufferImageCopy {
        bufferOffset: 0,
        bufferRowLength: 0,
        bufferImageHeight: 0,
        imageSubresource: image_subresource_layers,
        imageOffset: vks::VkOffset3D { x: 0, y: 0, z: 0 },
        imageExtent: vks::VkExtent3D { width, height, depth: 1 },
    };

    unsafe {
        device.proc_addr_loader().vkCmdCopyBufferToImage(
            command_buffer,
            buffer.handle(),
            image.handle(),
            vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            1,
            &region
        );
    }

    end_single_time_commands(device, command_pool, command_buffer)
}

fn copy_buffer(device: &Device, command_pool: &CommandPool, src_buffer: &Buffer,
        dst_buffer: &Buffer, size: vks::VkDeviceSize)  -> VooResult<()> {
    // TODO: Look into creating a separate command pool with the
    // `VK_COMMAND_POOL_CREATE_TRANSIENT_BIT` flag for short lived command
    // buffers like this.
    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let copy_region = vks::VkBufferCopy {
        srcOffset: 0,
        dstOffset: 0,
        size: size,
    };

    unsafe { device.proc_addr_loader().core.vkCmdCopyBuffer(command_buffer, src_buffer.handle(),
        dst_buffer.handle(), 1, &copy_region); }

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
        .usage(vks::VK_BUFFER_USAGE_TRANSFER_SRC_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .build(device.clone())?;


    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | vks::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, buffer_bytes, 0)?;
    data.copy_from_slice(vertices);
    staging_buffer_memory.unmap(data);

    let vertex_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(vks::VK_BUFFER_USAGE_TRANSFER_DST_BIT | vks::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .build(device.clone())?;

    let memory_requirements = vertex_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    let vertex_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
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
        .usage(vks::VK_BUFFER_USAGE_TRANSFER_SRC_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .build(device.clone())?;

    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT);
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, buffer_bytes, 0)?;
    data.copy_from_slice(indices);
    staging_buffer_memory.unmap(data);

    let index_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(vks::VK_BUFFER_USAGE_TRANSFER_DST_BIT | vks::VK_BUFFER_USAGE_INDEX_BUFFER_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .build(device.clone())?;

    let memory_requirements = index_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    let index_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    index_buffer.bind_memory(&index_buffer_memory, 0)?;

    copy_buffer(device, command_pool, &staging_buffer, &index_buffer, buffer_bytes)?;

    Ok((index_buffer, index_buffer_memory))
}

fn create_uniform_buffer(device: &Device, command_pool: &CommandPool, _extent: vks::VkExtent2D)
        -> VooResult<(Buffer, DeviceMemory)> {
    let buffer_bytes = mem::size_of::<UniformBufferObject>() as u64;
    let uniform_buffer = Buffer::builder()
        .size(buffer_bytes)
        .usage(vks::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .build(device.clone())?;

    let memory_requirements = uniform_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | vks::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    let uniform_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    uniform_buffer.bind_memory(&uniform_buffer_memory, 0)?;

    Ok((uniform_buffer, uniform_buffer_memory))
}

fn find_supported_format(device: &Device, candidates: &[vks::VkFormat], tiling: vks::VkImageTiling,
        features: vks::VkFormatFeatureFlags) -> VooResult<vks::VkFormat> {
    for &format in candidates {
        let mut props: vks::VkFormatProperties;
        unsafe {
            props = mem::uninitialized();
            device.instance().proc_addr_loader().vkGetPhysicalDeviceFormatProperties(
                device.physical_device().handle(), format, &mut props);
        }

        if tiling == vks::VK_IMAGE_TILING_LINEAR &&
                (props.linearTilingFeatures & features) == features
        {
            return Ok(format);
        } else if tiling == vks::VK_IMAGE_TILING_OPTIMAL &&
                (props.optimalTilingFeatures & features) == features
        {
            return Ok(format);
        }
    }

    panic!("Failed to find supported format.")
}

fn create_depth_resources(device: &Device, command_pool: &CommandPool,
        swapchain_extent: vks::VkExtent2D) -> VooResult<(Image, DeviceMemory, ImageView)> {
    let depth_format = find_depth_format(device)?;
    let extent = vks::VkExtent3D { width: swapchain_extent.width,
        height: swapchain_extent.height, depth: 1 };

    let depth_image = Image::builder()
        .image_type(vks::VK_IMAGE_TYPE_2D)
        .format(depth_format)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(vks::VK_SAMPLE_COUNT_1_BIT)
        .tiling(vks::VK_IMAGE_TILING_OPTIMAL)
        .usage(vks::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .initial_layout(vks::VK_IMAGE_LAYOUT_UNDEFINED)
        .build(device.clone())?;

    let memory_requirements = depth_image.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    let depth_image_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    depth_image.bind_memory(&depth_image_memory, 0)?;

    let depth_image_view = ImageView::builder()
        .image(depth_image.handle())
        .view_type(vks::VK_IMAGE_VIEW_TYPE_2D)
        .format(depth_format)
        .components(vks::VkComponentMapping::default())
        .subresource_range(vks::VkImageSubresourceRange {
            aspectMask: vks::VK_IMAGE_ASPECT_DEPTH_BIT,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        })
        .build(device.clone(), None)?;

    transition_image_layout(device, command_pool, &depth_image, depth_format,
        vks::VK_IMAGE_LAYOUT_UNDEFINED, vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL)?;

    Ok((depth_image, depth_image_memory, depth_image_view))
}

fn create_texture_image(device: &Device, command_pool: &CommandPool)
        -> VooResult<(Image, DeviceMemory)> {
    let pixels = image::open(TEXTURE_PATH).unwrap().to_rgba();
    let (tex_width, tex_height) = pixels.dimensions();
    let image_bytes = (tex_width * tex_height * 4) as u64;

    let staging_buffer = Buffer::builder()
        .size(image_bytes)
        .usage(vks::VK_BUFFER_USAGE_TRANSFER_SRC_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .build(device.clone())?;

    let memory_requirements = staging_buffer.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | vks::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    let staging_buffer_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    staging_buffer.bind_memory(&staging_buffer_memory, 0)?;

    let mut data = staging_buffer_memory.map(0, image_bytes, 0)?;
    data.copy_from_slice(&pixels);
    staging_buffer_memory.unmap(data);

    let extent = vks::VkExtent3D { width: tex_width, height: tex_height, depth: 1 };

    let texture_image = Image::builder()
        .image_type(vks::VK_IMAGE_TYPE_2D)
        .format(vks::VK_FORMAT_R8G8B8A8_UNORM)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(vks::VK_SAMPLE_COUNT_1_BIT)
        .tiling(vks::VK_IMAGE_TILING_OPTIMAL)
        .usage(vks::VK_IMAGE_USAGE_TRANSFER_DST_BIT | vks::VK_IMAGE_USAGE_SAMPLED_BIT)
        .sharing_mode(vks::VK_SHARING_MODE_EXCLUSIVE)
        .initial_layout(vks::VK_IMAGE_LAYOUT_UNDEFINED)
        .build(device.clone())?;

    let memory_requirements = texture_image.memory_requirements().clone();
    let memory_type_index = device.memory_type_index(memory_requirements.memoryTypeBits,
        vks::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    let texture_image_memory = DeviceMemory::new(device.clone(), memory_requirements.size,
        memory_type_index)?;
    texture_image.bind_memory(&texture_image_memory, 0)?;

    transition_image_layout(device, command_pool, &texture_image, vks::VK_FORMAT_R8G8B8A8_UNORM,
        vks::VK_IMAGE_LAYOUT_UNDEFINED, vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL)?;

    copy_buffer_to_image(device, command_pool, &staging_buffer, &texture_image,
        extent.width, extent.height)?;

    transition_image_layout(device, command_pool, &texture_image, vks::VK_FORMAT_R8G8B8A8_UNORM,
        vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL)?;

    Ok((texture_image, texture_image_memory))
}

fn create_texture_image_view(device: Device, image: &Image) -> VooResult<ImageView> {
    ImageView::builder()
        .image(image.handle())
        .view_type(vks::VK_IMAGE_VIEW_TYPE_2D)
        .format(vks::VK_FORMAT_R8G8B8A8_UNORM)
        .components(vks::VkComponentMapping::default())
        .subresource_range(vks::VkImageSubresourceRange {
            aspectMask: vks::VK_IMAGE_ASPECT_COLOR_BIT,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        })
        .build(device, None)
}

fn create_texture_sampler(device: Device) -> VooResult<Sampler> {
    Sampler::builder()
        .mag_filter(vks::VK_FILTER_LINEAR)
        .min_filter(vks::VK_FILTER_LINEAR)
        .mipmap_mode(vks::VK_SAMPLER_MIPMAP_MODE_LINEAR)
        .address_mode_u(vks::VK_SAMPLER_ADDRESS_MODE_REPEAT)
        .address_mode_v(vks::VK_SAMPLER_ADDRESS_MODE_REPEAT)
        .address_mode_w(vks::VK_SAMPLER_ADDRESS_MODE_REPEAT)
        .mip_lod_bias(0.)
        .anisotropy_enable(true)
        .max_anisotropy(16.)
        .compare_enable(false)
        .compare_op(vks::VK_COMPARE_OP_ALWAYS)
        .min_lod(0.)
        .max_lod(0.)
        .border_color(vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false)
        .build(device)
}

pub fn create_command_buffers(device: &Device, command_pool: &CommandPool,
        render_pass: &RenderPass, graphics_pipeline: &GraphicsPipeline,
        swapchain_framebuffers: &[Framebuffer], swapchain_extent: &vks::VkExtent2D,
        vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: u32,
        index_count: u32, pipeline_layout: &PipelineLayout,
        descriptor_set: vks::VkDescriptorSet)
        -> VooResult<Vec<vks::VkCommandBuffer>>
{
    let mut command_buffers = Vec::with_capacity(swapchain_framebuffers.len());
    unsafe { command_buffers.set_len(swapchain_framebuffers.len()); }

    let alloc_info = vks::VkCommandBufferAllocateInfo {
        sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: ptr::null(),
        commandPool: command_pool.handle(),
        // * COMMAND_BUFFER_LEVEL_PRIMARY: Can be submitted to a queue for
        //   execution, but cannot be called from other command buffers.
        // * COMMAND_BUFFER_LEVEL_SECONDARY: Cannot be submitted directly, but
        //   can be called from primary command buffers.
        level: vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: command_buffers.len() as u32,
    };

    unsafe {
        voo::check(device.proc_addr_loader().vkAllocateCommandBuffers(device.handle(),
            &alloc_info, command_buffers.as_mut_ptr()));
    }

    for (&command_buffer, swapchain_framebuffer) in command_buffers.iter()
            .zip(swapchain_framebuffers.iter())
    {
        let begin_info = vks::VkCommandBufferBeginInfo {
            sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            // * COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: The command buffer
            //   will be rerecorded right after executing it once.
            // * COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: This is a
            //   secondary command buffer that will be entirely within a
            //   single render pass.
            // * COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: The command buffer
            //   can be resubmitted while it is also already pending
            //   execution.
            flags: vks::VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
            pInheritanceInfo: ptr::null(),
        };

        unsafe {
            voo::check(device.proc_addr_loader().core.vkBeginCommandBuffer(command_buffer,
                &begin_info));
        }

        // let clear_color = vks::VkClearValue {
        //     color: vks::VkClearColorValue { float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32] }
        // };

        let clear_values = [
            vks::VkClearValue { color: vks::VkClearColorValue {
                float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32] } },
            vks::VkClearValue { depthStencil: vks::VkClearDepthStencilValue {
                depth: 1.0, stencil: 0, } },
        ];

        let render_pass_info = vks::VkRenderPassBeginInfo {
            sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: ptr::null(),
            renderPass: render_pass.handle(),
            framebuffer:swapchain_framebuffer.handle(),
            renderArea: vks::VkRect2D {
                offset: vks::VkOffset2D { x: 0, y: 0, },
                extent: swapchain_extent.clone(),
            },
            clearValueCount: clear_values.len() as u32,
            pClearValues: clear_values.as_ptr(),
        };

        unsafe {
            device.proc_addr_loader().core.vkCmdBeginRenderPass(command_buffer,
                &render_pass_info, vks::VK_SUBPASS_CONTENTS_INLINE);
            device.proc_addr_loader().core.vkCmdBindPipeline(command_buffer,
                vks::VK_PIPELINE_BIND_POINT_GRAPHICS, graphics_pipeline.handle());

            let vertex_buffers = [vertex_buffer.handle()];
            let offsets = [0];
            device.proc_addr_loader().core.vkCmdBindVertexBuffers(
                command_buffer, 0, 1, vertex_buffers.as_ptr(), offsets.as_ptr());
            device.proc_addr_loader().core.vkCmdBindIndexBuffer(command_buffer,
                index_buffer.handle(), 0, vks::VK_INDEX_TYPE_UINT32);

            device.proc_addr_loader().core.vkCmdBindDescriptorSets(command_buffer,
                vks::VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline_layout.handle(), 0, 1,
                &descriptor_set, 0, ptr::null());

            // // * vertexCount: Even though we don't have a vertex buffer, we
            // //   technically still have 3 vertices to draw.
            // // * instanceCount: Used for instanced rendering, use 1 if you're
            // //   not doing that.
            // // * firstVertex: Used as an offset into the vertex buffer,
            // //   defines the lowest value of gl_VertexIndex.
            // // * firstInstance: Used as an offset for instanced rendering,
            // //   defines the lowest value of gl_InstanceIndex.
            // device.proc_addr_loader().core.vkCmdDraw(command_buffer, vertex_count, 1, 0, 0);
            device.proc_addr_loader().core.vkCmdDrawIndexed(command_buffer, index_count,
                1, 0, 0, 0);

            device.proc_addr_loader().core.vkCmdEndRenderPass(command_buffer);
            device.proc_addr_loader().core.vkEndCommandBuffer(command_buffer);
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
    surface: Surface,
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
    descriptor_sets: SmallVec<[vks::VkDescriptorSet; 8]>,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    start_time: time::Instant,
    swapchain: Option<Swapchain>,
    swapchain_components: Option<SwapchainComponents>,
    command_buffers: Option<Vec<vks::VkCommandBuffer>>,
}

impl App {
    #[allow(unused_unsafe)]
    pub unsafe fn new() -> VooResult<App> {
        let instance = init_instance()?;
        let (window, events_loop) = init_window();
        let surface = voodoo_winit::create_surface(instance.clone(), &window)?;
        // let queue_family_flags = vks::VK_QUEUE_GRAPHICS_BIT;
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
            vertices.len() as u32, vertices.len() as u32, &pipeline_layout, descriptor_sets[0])?;
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
            self.device.proc_addr_loader().core.vkFreeCommandBuffers(self.device.handle(),
                self.command_pool.handle(),
                self.command_buffers.as_ref().unwrap().len() as u32,
                self.command_buffers.as_mut().unwrap().as_mut_ptr());
        }
        self.command_buffers = None;
    }

    fn recreate_swapchain(&mut self, current_extent: vks::VkExtent2D) -> VooResult<()> {
        unsafe { voo::check(self.device.proc_addr_loader().vkDeviceWaitIdle(
            self.device.handle())); }

        let swapchain = create_swapchain(self.surface.clone(), self.device.clone(),
            self.queue_family_flags, Some(current_extent), self.swapchain.take())?;

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
            self.indices.len() as u32, &self.pipeline_layout, self.descriptor_sets[0])?;

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
            extent.width as f32 / extent.height as f32, 0.1, 10.0);
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
                self.device.handle(), self.swapchain.as_ref().unwrap().handle(),
                u64::max_value(), self.image_available_semaphore.handle(), 0, &mut image_index)
        };
        if acq_res == vks::VK_ERROR_OUT_OF_DATE_KHR {
            let dims = self.window.get_inner_size_pixels().unwrap();
            self.recreate_swapchain(vks::VkExtent2D { height: dims.0, width: dims.1 } )?;
            return Ok(());
        } else if acq_res != vks::VK_SUCCESS && acq_res != vks::VK_SUBOPTIMAL_KHR {
            panic!("Unable to present swap chain image");
        }
        let wait_semaphores = [self.image_available_semaphore.handle()];
        let wait_stages = [vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
        let signal_semaphores = [self.render_finished_semaphore.handle()];

        let submit_info = vks::VkSubmitInfo {
            sType: vks::VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: wait_semaphores.len() as u32,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            pWaitDstStageMask: wait_stages.as_ptr(),
            commandBufferCount: 1,
            pCommandBuffers: self.command_buffers.as_ref().unwrap()
                .get(image_index as usize).unwrap(),
            signalSemaphoreCount: signal_semaphores.len() as u32,
            pSignalSemaphores: signal_semaphores.as_ptr(),
        };

        unsafe { voo::check(self.device.proc_addr_loader().core.vkQueueSubmit(
            self.device.queue(0), 1, &submit_info, 0)); }

        let swapchains = [self.swapchain.as_ref().unwrap().handle()];

        let present_info = vks::VkPresentInfoKHR {
            sType: vks::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: ptr::null(),
            waitSemaphoreCount: signal_semaphores.len() as u32,
            pWaitSemaphores: signal_semaphores.as_ptr(),
            swapchainCount: swapchains.len() as u32,
            pSwapchains: swapchains.as_ptr(),
            pImageIndices: &image_index,
            pResults: ptr::null_mut(),
        };

        unsafe {
            voo::check(self.device.proc_addr_loader().khr_swapchain.vkQueuePresentKHR(
                self.device.queue(0), &present_info));
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
                        current_extent = vks::VkExtent2D { width: w, height: h };
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
            self.device.handle())); }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("Goodbye triangle.");
    }
}

