use std::sync::Arc;
use std::marker::PhantomData;
use vks;
use ::{VdResult, Device,  Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RenderPassHandle(pub(crate) vks::VkRenderPass);

impl RenderPassHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkRenderPass {
        self.0
    }
}

unsafe impl Handle for RenderPassHandle {
    type Target = RenderPassHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: RenderPassHandle,
    device: Device,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_render_pass(self.handle, None);
        }
    }
}




/// A render pass.
///
///
/// ### Destruction
/// 
/// Dropping this `RenderPass` will cause `Device::destroy_render_pass` to be called, 
/// automatically releasing any resources associated with it.
///
#[derive(Debug, Clone)]
pub struct RenderPass {
    inner: Arc<Inner>,
}

impl RenderPass {
    /// Returns a new `RenderPassBuilder`.
    pub fn builder<'b>() -> RenderPassBuilder<'b> {
        RenderPassBuilder::new()
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> RenderPassHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'h> Handle for &'h RenderPass {
    type Target = RenderPassHandle;

    /// Returns this object's handle.
    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}


/// A builder for `RenderPass`.
#[derive(Debug, Clone)]
pub struct RenderPassBuilder<'b> {
    create_info: ::RenderPassCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> RenderPassBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> RenderPassBuilder<'b> {
        RenderPassBuilder {
            create_info: ::RenderPassCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// pAttachments points to an array of attachmentCount number of
    /// VkAttachmentDescription structures describing properties of the
    /// attachments, or NULL if attachmentCount is zero.
    pub fn attachments<'s, 'ad>(&'s mut self,
            attachments: &'ad [::AttachmentDescription])
            -> &'s mut RenderPassBuilder<'b>
            where 'ad: 'b {
        self.create_info.set_attachments(attachments);
        self
    }

    /// pSubpasses points to an array of subpassCount number of
    /// VkSubpassDescription structures describing properties of the
    /// subpasses.
    pub fn subpasses<'s, 'ad>(&'s mut self, subpasses: &'ad [::SubpassDescription])
            -> &'s mut RenderPassBuilder<'b>
            where 'ad: 'b {
        self.create_info.set_subpasses(subpasses);
        self
    }

    /// pDependencies points to an array of dependencyCount number of
    /// VkSubpassDependency structures describing dependencies between pairs
    /// of subpasses, or NULL if dependencyCount is zero.
    pub fn dependencies<'s, 'ad>(&'s mut self,
            dependencies: &'ad [::SubpassDependency])
            -> &'s mut RenderPassBuilder<'b>
            where 'ad: 'b {
        self.create_info.set_dependencies(dependencies);
        self
    }

    /// Builds and returns a new `RenderPass`
    pub fn build(&self, device: Device) -> VdResult<RenderPass> {
        let handle = unsafe { device.create_render_pass(&self.create_info, None)? };

        Ok(RenderPass {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}