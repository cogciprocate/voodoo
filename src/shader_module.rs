use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};
use vks;
use ::{VooResult, Device};


#[derive(Debug)]
struct Inner {
    handle: vks::VkShaderModule,
    device: Device,
}


// The following object types are consumed when they are passed into a Vulkan
// command and not further accessed by the objects they are used to create.
// They must not be destroyed in the duration of any API command they are
// passed into:
//
// * VkShaderModule
// * VkPipelineCache
//
#[derive(Debug, Clone)]
pub struct ShaderModule {
    inner: Arc<Inner>,
}

impl ShaderModule {
    pub fn new(device: Device, code: &[u8]) -> VooResult<ShaderModule> {
        let create_info = vks::VkShaderModuleCreateInfo {
            sType: vks::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            codeSize: code.len(),
            pCode: code.as_ptr() as *const u32,
        };

        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateShaderModule(device.handle(), &create_info,
                ptr::null(), &mut handle));
        }

        Ok(ShaderModule {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> vks::VkShaderModule {
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
            self.device.proc_addr_loader().core.vkDestroyShaderModule(self.device.handle(), self.handle, ptr::null());
        }
    }
}

