use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use vks;
use smallvec::SmallVec;
use ::{util, VooResult, Device, ShaderModule, DescriptorSetLayout};


#[derive(Debug)]
struct Inner {
    handle: vks::VkPipelineLayout,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct PipelineLayout {
    inner: Arc<Inner>,
}

impl PipelineLayout {
    pub fn new(device: Device, descriptor_set_layout: Option<&DescriptorSetLayout>)
            -> VooResult<PipelineLayout>
    {
        let mut layouts = SmallVec::<[_; 16]>::new();
        if let Some(dsl) = descriptor_set_layout {
            layouts.push(dsl.handle());
        }

        let pipeline_layout_info = vks::VkPipelineLayoutCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            setLayoutCount: layouts.len() as u32,
            pSetLayouts: layouts.as_ptr(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: ptr::null(),
        };

        let mut handle = 0;
        unsafe {
            ::check(device.vk().core.vkCreatePipelineLayout(device.handle(),
                &pipeline_layout_info, ptr::null(), &mut handle));
        }

        Ok(PipelineLayout {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkPipelineLayout {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}


impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.vk().core.vkDestroyPipelineLayout(self.device.handle(), self.handle, ptr::null());
        }
    }
}