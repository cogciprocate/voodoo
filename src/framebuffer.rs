
use std::sync::Arc;
use std::ptr;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{util, VooResult, Device, RenderPass, ImageViewHandle, ImageView, Handle, RenderPassHandle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct FramebufferHandle(pub vks::VkFramebuffer);

impl Handle for FramebufferHandle {
    type Target = FramebufferHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: FramebufferHandle,
    device: Device,
    render_pass: RenderPass,
    attachments: SmallVec<[ImageView; 8]>,
}

#[derive(Debug, Clone)]
pub struct Framebuffer {
    inner: Arc<Inner>,
}

impl Framebuffer {
    /// Returns a new `FramebufferBuilder`.
    pub fn builder<'b>() -> FramebufferBuilder<'b> {
        FramebufferBuilder::new()
    }

    pub fn handle(&self) -> FramebufferHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl<'h> Handle for &'h Framebuffer {
    type Target = FramebufferHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().core.vkDestroyFramebuffer(self.device.handle().0,
                self.handle.0, ptr::null());
        }
    }
}


/// A builder for `Framebuffer`.
//
// typedef struct VkFramebufferCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkFramebufferCreateFlags    flags;
//     VkRenderPass                renderPass;
//     uint32_t                    attachmentCount;
//     const VkImageView*          pAttachments;
//     uint32_t                    width;
//     uint32_t                    height;
//     uint32_t                    layers;
// } VkFramebufferCreateInfo;
//
//
#[derive(Debug, Clone)]
pub struct FramebufferBuilder<'b> {
    create_info: vks::VkFramebufferCreateInfo,
    render_pass: Option<&'b RenderPass>,
    attachments: Option<&'b [&'b ImageView]>,
    attachment_handles: Option<Vec<ImageViewHandle>>,
    _p: PhantomData<&'b ()>,
}

impl<'b> FramebufferBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> FramebufferBuilder<'b> {
        FramebufferBuilder {
            create_info: vks::VkFramebufferCreateInfo::default(),
            render_pass: None,
            attachments: None,
            attachment_handles: None,
            _p: PhantomData,
        }
    }

    /// Reserved for future use.
    pub fn flags<'s>(&'s mut self, flags: vks::VkFramebufferCreateFlags)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// Specifies a render pass that defines what render passes the
    /// framebuffer will be compatible with.
    pub fn render_pass<'s, 'p>(&'s mut self, render_pass: &'p RenderPass)
            -> &'s mut FramebufferBuilder<'b>
            where 'p: 'b {
        self.create_info.renderPass = render_pass.handle().0;
        self.render_pass = Some(render_pass);
        self
    }

    /// Specifies the image views, each of which will be used as the
    /// corresponding attachment in a render pass instance.
    pub fn attachments<'s, 'p>(&'s mut self, attachments: &'p [&'p ImageView])
            -> &'s mut FramebufferBuilder<'b>
            where 'p: 'b {
        self.attachment_handles = Some(attachments.iter().map(|att| att.handle()).collect());
        if let Some(ref att_hnd) = self.attachment_handles {
            self.create_info.attachmentCount = att_hnd.len() as u32;
            self.create_info.pAttachments = att_hnd.as_ptr() as *const vks::VkImageView;
        }
        self.attachments = Some(attachments);
        self
    }

    /// Specifies the width of the framebuffer.
    pub fn width<'s>(&'s mut self, width: u32)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.width = width;
        self
    }

    /// Specifies the height of the framebuffer.
    pub fn height<'s>(&'s mut self, height: u32)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.height = height;
        self
    }

    /// Specifies the number of layers of the framebuffer.
    pub fn layers<'s>(&'s mut self, layers: u32)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.layers = layers;
        self
    }

    /// Creates and returns a new `Framebuffer`
    pub fn build(&self, device: Device) -> VooResult<Framebuffer> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateFramebuffer(device.handle().0,
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(Framebuffer {
            inner: Arc::new(Inner {
                handle: FramebufferHandle(handle),
                device,
                render_pass: self.render_pass.cloned()
                    .expect("unable to create framebuffer: no render pass specified"),
                attachments: self.attachments.as_ref()
                    .expect("unable to create framebuffer: no attachments specified")
                    .iter().map(|&att| att.clone()).collect(),
            })
        })
    }
}
