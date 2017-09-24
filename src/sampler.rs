
use std::sync::Arc;
use std::ptr;
use vks;
use ::{util, VooResult, Device};

#[derive(Debug)]
struct Inner {
    handle: vks::VkSampler,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Sampler {
    inner: Arc<Inner>,
}

impl Sampler {
    pub fn new(device: Device) -> VooResult<Sampler> {
        let create_info = vks::VkSamplerCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            magFilter: vks::VK_FILTER_LINEAR,
            minFilter: vks::VK_FILTER_LINEAR,
            mipmapMode: vks::VK_SAMPLER_MIPMAP_MODE_LINEAR,
            addressModeU: vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
            addressModeV: vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
            addressModeW: vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
            mipLodBias: 0.,
            // anisotropyEnable: vks::VK_FALSE,
            // maxAnisotropy: 1.,
            anisotropyEnable: vks::VK_TRUE,
            maxAnisotropy: 16.,
            compareEnable: vks::VK_FALSE,
            compareOp: vks::VK_COMPARE_OP_ALWAYS,
            minLod: 0.,
            maxLod: 0.,
            borderColor: vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
            unnormalizedCoordinates: vks::VK_FALSE,
        };

        let mut handle = 0;
        unsafe {
            ::check(device.vk().vkCreateSampler(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        Ok(Sampler {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkSampler {
        self.inner.handle
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.vk().vkDestroySampler(self.device.handle(), self.handle, ptr::null());
        }
    }
}