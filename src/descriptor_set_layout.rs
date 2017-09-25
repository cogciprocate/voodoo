
use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device};

#[derive(Debug)]
struct Inner {
    handle: vks::VkDescriptorSetLayout,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct DescriptorSetLayout {
    inner: Arc<Inner>,
}

impl DescriptorSetLayout {
    pub fn new(device: Device) -> VooResult<DescriptorSetLayout> {
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

        let create_info = vks::VkDescriptorSetLayoutCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            bindingCount: bindings.len() as u32,
            pBindings: bindings.as_ptr(),
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().vkCreateDescriptorSetLayout(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        Ok(DescriptorSetLayout {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkDescriptorSetLayout {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().vkDestroyDescriptorSetLayout(self.device.handle(), self.handle, ptr::null());
        }
    }
}