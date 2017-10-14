use std::sync::Arc;
use vks;
use ::{VdResult, Device, Handle, ShaderModuleCreateInfo};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ShaderModuleHandle(pub(crate) vks::VkShaderModule);

impl ShaderModuleHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkShaderModule {
        self.0
    }
}

unsafe impl Handle for ShaderModuleHandle {
    type Target = ShaderModuleHandle;

    /// Returns this object's handle.
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
    /// Creates and returns a new `ShaderModule`.
    pub fn new(device: Device, code: &[u32]) -> VdResult<ShaderModule> {
        let create_info = ShaderModuleCreateInfo::builder()
            .code(code)
            .build();

        let handle = unsafe { device.create_shader_module(&create_info, None)? };

        Ok(ShaderModule {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> ShaderModuleHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'h> Handle for &'h ShaderModule {
    type Target = ShaderModuleHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.handle, None);
        }
    }
}

