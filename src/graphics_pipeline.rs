
use std::sync::Arc;
// use std::ptr;
// use std::ffi::CStr;
use std::marker::PhantomData;
use smallvec::SmallVec;
// use vks;
use ::{VooResult, Device, PipelineLayoutHandle, PipelineHandle, RenderPassHandle,
    Handle, GraphicsPipelineCreateInfo, AnyPipelineHandle};



#[derive(Debug)]
struct Inner {
    handle: PipelineHandle,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct GraphicsPipeline {
    inner: Arc<Inner>,
}

impl GraphicsPipeline {
    /// Returns a new `GraphicsPipelineBuilder`.
    pub fn builder<'b>() -> GraphicsPipelineBuilder<'b> {
        GraphicsPipelineBuilder::new()
    }

    /// Creates several graphics pipelines at once.
    #[deprecated(note = "use `Device::create_graphics_pipelines`")]
    pub fn create<'b, Gpb>(device: &Device, builders: &[Gpb])
            -> VooResult<SmallVec<[GraphicsPipeline; 8]>>
            where Gpb: AsRef<::GraphicsPipelineCreateInfo<'b>> {
        let mut create_infos = SmallVec::<[GraphicsPipelineCreateInfo; 8]>::new();
        // let mut pipeline_handles = SmallVec::<[PipelineHandle; 8]>::new();
        let mut pipelines = SmallVec::<[GraphicsPipeline; 8]>::new();
        create_infos.reserve_exact(builders.len());
        // pipeline_handles.reserve_exact(builders.len());
        pipelines.reserve_exact(builders.len());

        for builder in builders {
            create_infos.push(builder.as_ref().clone());
        }

        let pipeline_handles = unsafe { device.create_graphics_pipelines(None, &create_infos, None)? };

        for handle in pipeline_handles {
            pipelines.push(
                GraphicsPipeline {
                    inner: Arc::new(Inner {
                        handle,
                        device: device.clone(),
                    })
                }
            );
        }

        Ok(pipelines)
    }

    pub fn handle(&self) -> PipelineHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'g> Handle for &'g GraphicsPipeline {
    type Target = PipelineHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

// impl<'g> AnyPipelineHandle for &'g GraphicsPipeline {
//     type Target = PipelineHandle;

//     fn handle(&self) -> Self::Target {
//         self.inner.handle
//     }
// }
unsafe impl<'g> AnyPipelineHandle for &'g GraphicsPipeline {}


impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            // self.device.proc_addr_loader().core.vkDestroyPipeline(self.device.handle().0,
            //     self.handle.0, ptr::null());
            self.device.destroy_pipeline(self.handle, None);
        }
    }
}


/// A builder for `GraphicsPipeline`.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct GraphicsPipelineBuilder<'b> {
    create_info: GraphicsPipelineCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> GraphicsPipelineBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> GraphicsPipelineBuilder<'b> {
        GraphicsPipelineBuilder {
            create_info: GraphicsPipelineCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies how the pipeline will be generated.
    pub fn flags<'s>(&'s mut self, flags: ::PipelineCreateFlags)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// Specifies the number of entries in the pStages array. `stages` is a
    /// list of  structures describing the set of the shader stages to be
    /// included in the graphics pipeline.
    pub fn stages<'s, 'p>(&'s mut self,
            stages: &'p [::PipelineShaderStageCreateInfo])
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        // self.create_info.stageCount(stages.len() as u32);
        self.create_info.set_stages(stages);
        self
    }

    /// Specifies the vertex input state details.
    pub fn vertex_input_state<'s, 'p>(&'s mut self,
            vertex_input_state: &'p ::PipelineVertexInputStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_vertex_input_state(vertex_input_state);
        self
    }

    /// Specifies the input assembly behavior, as described in Drawing
    /// Commands.
    pub fn input_assembly_state<'s, 'p>(&'s mut self, input_assembly_state:
            &'p ::PipelineInputAssemblyStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_input_assembly_state(input_assembly_state);
        self
    }

    /// Specifies the tessellation state and is ignored if the pipeline does
    /// not include a tessellation control shader stage and tessellation
    /// evaluation shader stage.
    pub fn tessellation_state<'s, 'p>(&'s mut self,
            tessellation_state: &'p ::PipelineTessellationStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_tessellation_state(tessellation_state);
        self
    }

    /// Specifies the viewport state and is ignored if the pipeline has
    /// rasterization disabled.
    pub fn viewport_state<'s, 'p>(&'s mut self,
            viewport_state: &'p ::PipelineViewportStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_viewport_state(viewport_state);
        self
    }

    /// Specifies the rasterization state.
    pub fn rasterization_state<'s, 'p>(&'s mut self,
            rasterization_state: &'p ::PipelineRasterizationStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_rasterization_state(rasterization_state);
        self
    }

    /// Specifies the multisample state and is ignored if the pipeline has
    /// rasterization disabled.
    pub fn multisample_state<'s, 'p>(&'s mut self,
            multisample_state: &'p ::PipelineMultisampleStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_multisample_state(multisample_state);
        self
    }

    /// Specifies the depth stencil state and is ignored if the pipeline has
    /// rasterization disabled or if the subpass of the render pass the
    /// pipeline is created against does not use a depth/stencil attachment.
    pub fn depth_stencil_state<'s, 'p>(&'s mut self,
            depth_stencil_state: &'p ::PipelineDepthStencilStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_depth_stencil_state(depth_stencil_state);
        self
    }

    /// Specifies the color blend state and is ignored if the pipeline has
    /// rasterization disabled or if the subpass of the render pass the
    /// pipeline is created against does not use any color attachments.
    pub fn color_blend_state<'s, 'p>(&'s mut self,
            color_blend_state: &'p ::PipelineColorBlendStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_color_blend_state(color_blend_state);
        self
    }

    /// Specifies which properties of the pipeline state object are dynamic
    /// and can be changed independently of the pipeline state. If not
    /// specified, no state in the pipeline is considered dynamic.
    pub fn dynamic_state<'s, 'p>(&'s mut self,
            dynamic_state: &'p ::PipelineDynamicStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.set_dynamic_state(dynamic_state);
        self
    }

    /// Specifies the binding locations used by both the pipeline and
    /// descriptor sets used with the pipeline.
    pub fn layout<'s, H>(&'s mut self, layout: H) -> &'s mut GraphicsPipelineBuilder<'b>
            where H: Handle<Target=PipelineLayoutHandle> {
        self.create_info.set_layout(layout);
        self
    }

    /// Specifies the environment in which the pipeline will be used; the
    /// pipeline must only be used with an instance of any render pass
    /// compatible with the one provided.
    pub fn render_pass<'s, H>(&'s mut self, render_pass: H)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where H: Handle<Target=RenderPassHandle> {
        self.create_info.set_render_pass(render_pass);
        self
    }

    /// Specifies the index of the subpass in the render pass where this
    /// pipeline will be used.
    pub fn subpass<'s>(&'s mut self, subpass: u32)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.set_subpass(subpass);
        self
    }

    /// Specifies the pipeline to derive from.
    pub fn base_pipeline<'s, H>(&'s mut self, base_pipeline: H)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where H: Handle<Target=PipelineHandle> {
        self.create_info.set_base_pipeline_handle(base_pipeline);
        self
    }

    /// Specifies the index into the pCreateInfos parameter to use as a
    /// pipeline to derive from.
    pub fn base_pipeline_index<'s>(&'s mut self, base_pipeline_index: i32)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.set_base_pipeline_index(base_pipeline_index);
        self
    }

    /// Creates and returns a new `GraphicsPipeline`. Use
    /// `GraphicsPipeline::create` to create multiple pipelines in one call.
    pub fn build(&self, device: Device) -> VooResult<GraphicsPipeline> {
        let handle = unsafe {
            let create_infos = ::std::slice::from_raw_parts(&self.create_info, 1);
            *device.create_graphics_pipelines(None, create_infos, None)?.get_unchecked(0)
        };

        Ok(GraphicsPipeline {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}

impl<'b> AsRef<GraphicsPipelineBuilder<'b>> for GraphicsPipelineBuilder<'b> {
    fn as_ref(&self) -> &GraphicsPipelineBuilder<'b> {
        self
    }
}

impl<'b> AsRef<::GraphicsPipelineCreateInfo<'b>> for GraphicsPipelineBuilder<'b> {
    fn as_ref(&self) -> &::GraphicsPipelineCreateInfo<'b> {
        &self.create_info
    }
}