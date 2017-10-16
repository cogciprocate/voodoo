use std::sync::Arc;
use std::marker::PhantomData;
use smallvec::SmallVec;
use vks;
use ::{VdResult, Device, RenderPass, ImageView, Handle, FramebufferCreateInfo,
    FramebufferCreateFlags};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct FramebufferHandle(pub(crate) vks::VkFramebuffer);

impl FramebufferHandle {
    pub fn to_raw(&self) -> vks::VkFramebuffer {
        self.0
    }
}

unsafe impl Handle for FramebufferHandle {
    type Target = FramebufferHandle;

    /// Returns this object's handle.
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

    /// Returns this object's handle.
    pub fn handle(&self) -> FramebufferHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

unsafe impl<'h> Handle for &'h Framebuffer {
    type Target = FramebufferHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            // self.device.proc_addr_loader().vk.vkDestroyFramebuffer(self.device.handle().0,
            //     self.handle.0, ptr::null());
            self.device.destroy_framebuffer(self.handle, None);
        }
    }
}


/// A builder for `Framebuffer`.
#[derive(Debug, Clone)]
pub struct FramebufferBuilder<'b> {
    create_info: FramebufferCreateInfo<'b>,
    render_pass: Option<&'b RenderPass>,
    attachments: Option<&'b [&'b ImageView]>,
    _p: PhantomData<&'b ()>,
}

impl<'b> FramebufferBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> FramebufferBuilder<'b> {
        FramebufferBuilder {
            create_info: FramebufferCreateInfo::default(),
            render_pass: None,
            attachments: None,
            _p: PhantomData,
        }
    }

    /// Reserved for future use.
    pub fn flags<'s>(&'s mut self, flags: FramebufferCreateFlags)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// Specifies a render pass that defines what render passes the
    /// framebuffer will be compatible with.
    pub fn render_pass<'s, 'p>(&'s mut self, render_pass: &'p RenderPass)
            -> &'s mut FramebufferBuilder<'b>
            where 'p: 'b {
        self.create_info.set_render_pass(render_pass.handle());
        self.render_pass = Some(render_pass);
        self
    }

    /// Specifies the image views, each of which will be used as the
    /// corresponding attachment in a render pass instance.
    pub fn attachments<'s, 'p>(&'s mut self, attachments: &'p [&'p ImageView])
            -> &'s mut FramebufferBuilder<'b>
            where 'p: 'b {
        self.attachments = Some(attachments);
        self
    }

    /// Specifies the width of the framebuffer.
    pub fn width<'s>(&'s mut self, width: u32)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.set_width(width);
        self
    }

    /// Specifies the height of the framebuffer.
    pub fn height<'s>(&'s mut self, height: u32)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.set_height(height);
        self
    }

    /// Specifies the number of layers of the framebuffer.
    pub fn layers<'s>(&'s mut self, layers: u32)
            -> &'s mut FramebufferBuilder<'b> {
        self.create_info.set_layers(layers);
        self
    }

    /// Creates and returns a new `Framebuffer`
    pub fn build(&self, device: Device) -> VdResult<Framebuffer> {
        let render_pass = self.render_pass.cloned()
            .expect("unable to create framebuffer: no render pass specified");
        let attachments = self.attachments.as_ref()
            .expect("unable to create framebuffer: no attachments specified")
            .iter().map(|&att| att.clone()).collect();
        let attachment_handles: SmallVec<[_; 8]> = self.attachments.as_ref().unwrap()
            .iter().map(|&att| att.handle()).collect();

        let mut create_info = self.create_info.clone();
        create_info.set_attachments(&attachment_handles);
        let handle = unsafe { device.create_framebuffer(&create_info, None)? };

        Ok(Framebuffer {
            inner: Arc::new(Inner {
                handle,
                device,
                render_pass,
                attachments,
            })
        })
    }
}
