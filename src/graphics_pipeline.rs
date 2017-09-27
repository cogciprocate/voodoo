
use std::sync::Arc;
use std::ptr;
use std::ffi::CStr;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{util, VooResult, Device, ShaderModule, PipelineLayout, RenderPass, Vertex};



#[derive(Debug)]
struct Inner {
    handle: vks::VkPipeline,
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
    pub fn create<'b, Gpb>(device: &Device, builders: &[Gpb])
            -> VooResult<SmallVec<[GraphicsPipeline; 8]>>
            where Gpb: AsRef<GraphicsPipelineBuilder<'b>> {
        let mut create_infos = SmallVec::<[vks::VkGraphicsPipelineCreateInfo; 8]>::new();
        let mut pipeline_handles = SmallVec::<[vks::VkPipeline; 8]>::new();
        let mut pipelines = SmallVec::<[GraphicsPipeline; 8]>::new();
        create_infos.reserve_exact(builders.len());
        pipeline_handles.reserve_exact(builders.len());
        pipelines.reserve_exact(builders.len());

        for builder in builders {
            create_infos.push(builder.as_ref().create_info.clone());
        }

        unsafe {
            pipeline_handles.set_len(builders.len());
            ::check(device.proc_addr_loader().core.vkCreateGraphicsPipelines(device.handle(),
                0, create_infos.len() as u32, create_infos.as_ptr(), ptr::null(),
                pipeline_handles.as_mut_ptr()));
        }

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

    pub fn handle(&self) -> vks::VkPipeline {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyPipeline(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `GraphicsPipeline`.
//
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
//
#[derive(Debug, Clone)]
pub struct GraphicsPipelineBuilder<'b> {
    create_info: vks::VkGraphicsPipelineCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> GraphicsPipelineBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> GraphicsPipelineBuilder<'b> {
        GraphicsPipelineBuilder {
            create_info: vks::VkGraphicsPipelineCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Specifies how the pipeline will be generated.
    pub fn flags<'s>(&'s mut self, flags: vks::VkPipelineCreateFlags)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// Specifies the number of entries in the pStages array. `stages` is a
    /// list of  structures describing the set of the shader stages to be
    /// included in the graphics pipeline.
    pub fn stages<'s, 'p>(&'s mut self,
            stages: &'p [vks::VkPipelineShaderStageCreateInfo])
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.stageCount = stages.len() as u32;
        self.create_info.pStages = stages.as_ptr();
        self
    }

    /// Specifies the vertex input state details.
    pub fn vertex_input_state<'s, 'p>(&'s mut self,
            vertex_input_state: &'p vks::VkPipelineVertexInputStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pVertexInputState = vertex_input_state;
        self
    }

    /// Specifies the input assembly behavior, as described in Drawing
    /// Commands.
    pub fn input_assembly_state<'s, 'p>(&'s mut self, input_assembly_state:
            &'p vks::VkPipelineInputAssemblyStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pInputAssemblyState = input_assembly_state;
        self
    }

    /// Specifies the tessellation state and is ignored if the pipeline does
    /// not include a tessellation control shader stage and tessellation
    /// evaluation shader stage.
    pub fn tessellation_state<'s, 'p>(&'s mut self,
            tessellation_state: &'p vks::VkPipelineTessellationStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pTessellationState = tessellation_state;
        self
    }

    /// Specifies the viewport state and is ignored if the pipeline has
    /// rasterization disabled.
    pub fn viewport_state<'s, 'p>(&'s mut self,
            viewport_state: &'p vks::VkPipelineViewportStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pViewportState = viewport_state;
        self
    }

    /// Specifies the rasterization state.
    pub fn rasterization_state<'s, 'p>(&'s mut self,
            rasterization_state: &'p vks::VkPipelineRasterizationStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pRasterizationState = rasterization_state;
        self
    }

    /// Specifies the multisample state and is ignored if the pipeline has
    /// rasterization disabled.
    pub fn multisample_state<'s, 'p>(&'s mut self,
            multisample_state: &'p vks::VkPipelineMultisampleStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pMultisampleState = multisample_state;
        self
    }

    /// Specifies the depth stencil state and is ignored if the pipeline has
    /// rasterization disabled or if the subpass of the render pass the
    /// pipeline is created against does not use a depth/stencil attachment.
    pub fn depth_stencil_state<'s, 'p>(&'s mut self,
            depth_stencil_state: &'p vks::VkPipelineDepthStencilStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pDepthStencilState = depth_stencil_state;
        self
    }

    /// Specifies the color blend state and is ignored if the pipeline has
    /// rasterization disabled or if the subpass of the render pass the
    /// pipeline is created against does not use any color attachments.
    pub fn color_blend_state<'s, 'p>(&'s mut self,
            color_blend_state: &'p vks::VkPipelineColorBlendStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pColorBlendState = color_blend_state;
        self
    }

    /// Specifies which properties of the pipeline state object are dynamic
    /// and can be changed independently of the pipeline state. If not
    /// specified, no state in the pipeline is considered dynamic.
    pub fn dynamic_state<'s, 'p>(&'s mut self,
            dynamic_state: &'p vks::VkPipelineDynamicStateCreateInfo)
            -> &'s mut GraphicsPipelineBuilder<'b>
            where 'p: 'b {
        self.create_info.pDynamicState = dynamic_state;
        self
    }

    /// Specifies the binding locations used by both the pipeline and
    /// descriptor sets used with the pipeline.
    pub fn layout<'s>(&'s mut self, layout: &PipelineLayout)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.layout = layout.handle();
        self
    }

    /// Specifies the environment in which the pipeline will be used; the
    /// pipeline must only be used with an instance of any render pass
    /// compatible with the one provided.
    pub fn render_pass<'s>(&'s mut self, render_pass: &RenderPass)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.renderPass = render_pass.handle();
        self
    }

    /// Specifies the index of the subpass in the render pass where this
    /// pipeline will be used.
    pub fn subpass<'s>(&'s mut self, subpass: u32)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.subpass = subpass;
        self
    }

    /// Specifies the pipeline to derive from.
    pub fn base_pipeline_handle<'s>(&'s mut self, base_pipeline_handle: vks::VkPipeline)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.basePipelineHandle = base_pipeline_handle;
        self
    }

    /// Specifies the index into the pCreateInfos parameter to use as a
    /// pipeline to derive from.
    pub fn base_pipeline_index<'s>(&'s mut self, base_pipeline_index: i32)
            -> &'s mut GraphicsPipelineBuilder<'b> {
        self.create_info.basePipelineIndex = base_pipeline_index;
        self
    }

    /// Creates and returns a new `GraphicsPipeline`. Use
    /// `GraphicsPipeline::create` to create multiple pipelines in one call.
    pub fn build(&self, device: Device) -> VooResult<GraphicsPipeline> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateGraphicsPipelines(device.handle(),
                0, 1, &self.create_info, ptr::null(), &mut handle));
        }

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