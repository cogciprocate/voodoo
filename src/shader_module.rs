use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};
use vks;
use ::{VooResult, Device, Handle, ShaderModuleCreateInfo};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ShaderModuleHandle(pub(crate) vks::VkShaderModule);

impl ShaderModuleHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkShaderModule {
        self.0
    }
}

impl Handle for ShaderModuleHandle {
    type Target = ShaderModuleHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: ShaderModuleHandle,
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
    pub fn new(device: Device, code: &[u32]) -> VooResult<ShaderModule> {
        // let create_info = vks::VkShaderModuleCreateInfo {
        //     sType: vks::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        //     pNext: ptr::null(),
        //     flags: 0,
        //     codeSize: code.len(),
        //     pCode: code.as_ptr() as *const u32,
        // };

        let create_info = ShaderModuleCreateInfo::builder()
            .code(code)
            .build();

        // let mut handle = 0;
        // unsafe {
        //     ::check(device.proc_addr_loader().core.vkCreateShaderModule(device.handle().0, &create_info,
        //         ptr::null(), &mut handle));
        // }
        let handle = unsafe { device.create_shader_module(&create_info, None)? };

        Ok(ShaderModule {
            inner: Arc::new(Inner {
                // handle: ShaderModuleHandle(handle),
                handle,
                device,
            })
        })
    }

    pub fn handle(&self) -> ShaderModuleHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl<'h> Handle for &'h ShaderModule {
    type Target = ShaderModuleHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            // self.device.proc_addr_loader().core.vkDestroyShaderModule(self.device.handle().0,
            //     self.handle.0, ptr::null());
            self.device.destroy_shader_module(self.handle, None);
        }
    }
}

