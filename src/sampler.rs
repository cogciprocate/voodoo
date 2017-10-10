
use std::sync::Arc;
use std::marker::PhantomData;
use vks;
use ::{VooResult, Device, Handle};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SamplerHandle(pub(crate) vks::VkSampler);

impl SamplerHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkSampler {
        self.0
    }
}

impl Handle for SamplerHandle {
    type Target = SamplerHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: SamplerHandle,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Sampler {
    inner: Arc<Inner>,
}

impl Sampler {
    /// Returns a new `SamplerBuilder`.
    pub fn builder<'b>() -> SamplerBuilder<'b> {
        SamplerBuilder::new()
    }

    // pub fn new(device: Device) -> VooResult<Sampler> {
    //     let create_info = vks::VkSamplerCreateInfo {
    //         sType: vks::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
    //         pNext: ptr::null(),
    //         flags: 0,
    //         magFilter: vks::VK_FILTER_LINEAR,
    //         minFilter: vks::VK_FILTER_LINEAR,
    //         mipmapMode: vks::VK_SAMPLER_MIPMAP_MODE_LINEAR,
    //         addressModeU: vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
    //         addressModeV: vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
    //         addressModeW: vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
    //         mipLodBias: 0.,
    //         // anisotropyEnable: vks::VK_FALSE,
    //         // maxAnisotropy: 1.,
    //         anisotropyEnable: vks::VK_TRUE,
    //         maxAnisotropy: 16.,
    //         compareEnable: vks::VK_FALSE,
    //         compareOp: vks::VK_COMPARE_OP_ALWAYS,
    //         minLod: 0.,
    //         maxLod: 0.,
    //         borderColor: vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
    //         unnormalizedCoordinates: vks::VK_FALSE,
    //     };

    //     let mut handle = 0;
    //     unsafe {
    //         ::check(device.proc_addr_loader().vkCreateSampler(device.handle().0, &create_info,
    //             ptr::null(), &mut handle));
    //     }

    //     Ok(Sampler {
    //         inner: Arc::new(Inner {
    //             handle,
    //             device,
    //         })
    //     })
    // }

    pub fn handle(&self) -> SamplerHandle {
        self.inner.handle
    }

    /// Returns a reference to the associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl<'h> Handle for &'h Sampler {
    type Target = SamplerHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            // self.device.proc_addr_loader().vkDestroySampler(self.device.handle().0,
            //     self.handle.0, ptr::null());
            self.device.destroy_sampler(self.handle, None);
        }
    }
}


/// A builder for `Sampler`.
#[derive(Debug, Clone)]
pub struct SamplerBuilder<'b> {
    create_info: ::SamplerCreateInfo<'b>,
    _p: PhantomData<&'b ()>,
}

impl<'b> SamplerBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> SamplerBuilder<'b> {
        SamplerBuilder {
            create_info: ::SamplerCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Reserved for future use.
    pub fn flags<'s>(&'s mut self, flags: ::SamplerCreateFlags)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_flags(flags);
        self
    }

    /// Specifies the magnification filter to
    /// apply to lookups.
    pub fn mag_filter<'s>(&'s mut self, mag_filter: ::Filter)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_mag_filter(mag_filter);
        self
    }

    /// Specifies the minification filter to apply to lookups.
    pub fn min_filter<'s>(&'s mut self, min_filter: ::Filter)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_min_filter(min_filter);
        self
    }

    /// Specifies the mipmap filter to apply to lookups.
    pub fn mipmap_mode<'s>(&'s mut self, mipmap_mode: ::SamplerMipmapMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_mipmap_mode(mipmap_mode);
        self
    }

    /// Specifies the addressing mode for outside [0..1] range for U
    /// coordinate.
    pub fn address_mode_u<'s>(&'s mut self, address_mode_u: ::SamplerAddressMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_address_mode_u(address_mode_u);
        self
    }

    /// Specifies the addressing mode for outside [0..1] range for V
    /// coordinate.
    pub fn address_mode_v<'s>(&'s mut self, address_mode_v: ::SamplerAddressMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_address_mode_v(address_mode_v);
        self
    }

    /// Specifies the addressing mode for outside [0..1] range for W
    /// coordinate.
    pub fn address_mode_w<'s>(&'s mut self, address_mode_w: ::SamplerAddressMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_address_mode_w(address_mode_w);
        self
    }

    /// Specifies the the bias to be added to mipmap LOD calculation and bias
    /// provided by image sampling functions in SPIR-V, as described in the
    /// Level-of-Detail Operation section [TODO: INSERT LINK].
    pub fn mip_lod_bias<'s>(&'s mut self, mip_lod_bias: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_mip_lod_bias(mip_lod_bias);
        self
    }

    /// Specifies whether or not to enable anisotropic filtering, as described
    /// in the Texel Anisotropic Filtering section.
    pub fn anisotropy_enable<'s>(&'s mut self, anisotropy_enable: bool)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_anisotropy_enable(anisotropy_enable);
        self
    }

    /// Specifies the anisotropy value clamp.
    pub fn max_anisotropy<'s>(&'s mut self, max_anisotropy: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_max_anisotropy(max_anisotropy);
        self
    }

    /// Specifies whether or not to enable comparison against a reference
    /// value during lookups.
    ///
    /// Note: Some implementations will default to shader state if this member
    /// does not match.
    pub fn compare_enable<'s>(&'s mut self, compare_enable: bool)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_compare_enable(compare_enable);
        self
    }

    /// Specifies the comparison function to apply to fetched data before
    /// filtering as described in the Depth Compare Operation section.
    pub fn compare_op<'s>(&'s mut self, compare_op: ::CompareOp)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_compare_op(compare_op);
        self
    }

    /// Specifies the minimum value used to clamp the computed level-of-detail
    /// value, as described in the Level-of-Detail Operation section.
    /// `max_lod` must be greater than or equal to `min_lod`.
    pub fn min_lod<'s>(&'s mut self, min_lod: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_min_lod(min_lod);
        self
    }

    /// Specifies the maximum value used to clamp the computed level-of-detail
    /// value, as described in the Level-of-Detail Operation section. `max_lod`
    /// must be greater than or equal to `min_lod`.
    pub fn max_lod<'s>(&'s mut self, max_lod: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_max_lod(max_lod);
        self
    }

    /// Specifies the predefined border color to use.
    pub fn border_color<'s>(&'s mut self, border_color: ::BorderColor)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_border_color(border_color);
        self
    }

    /// Specifies whether to use unnormalized or normalized texel coordinates
    /// to address texels of the image. When set to VK_TRUE, the range of the
    /// image coordinates used to lookup the texel is in the range of zero to
    /// the image dimensions for x, y and z. When set to VK_FALSE the range of
    /// image coordinates is zero to one. When unnormalizedCoordinates is
    /// VK_TRUE, samplers have the following requirements:
    ///
    ///   * minFilter and magFilter must be equal.
    ///   * mipmapMode must be VK_SAMPLER_MIPMAP_MODE_NEAREST.
    ///   * minLod and maxLod must be zero.
    ///   * addressModeU and addressModeV must each be either
    ///     VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or
    ///     VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER.
    ///   * anisotropyEnable must be VK_FALSE.
    ///   * compareEnable must be VK_FALSE.
    ///   * The sampler must not enable sampler Y’CBCR conversion.
    ///
    /// * When unnormalizedCoordinates is VK_TRUE, images the sampler is used
    ///   with in the shader have the following requirements:
    ///
    ///   * The viewType must be either VK_IMAGE_VIEW_TYPE_1D or
    ///     VK_IMAGE_VIEW_TYPE_2D.
    ///   * The image view must have a single layer and a single mip level.
    ///
    /// * When unnormalizedCoordinates is VK_TRUE, image built-in functions in
    ///   the shader that use the sampler have the following requirements:
    ///
    ///   * The functions must not use projection.
    ///   * The functions must not use offsets.
    pub fn unnormalized_coordinates<'s>(&'s mut self, unnormalized_coordinates: bool)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.set_unnormalized_coordinates(unnormalized_coordinates);
        self
    }

    /// Creates and returns a new `Sampler`
    pub fn build(&self, device: Device) -> VooResult<Sampler> {
        // let mut handle = 0;
        // unsafe {
        //     ::check(device.proc_addr_loader().core.vkCreateSampler(device.handle().0,
        //         self.create_info.as_raw(), ptr::null(), &mut handle));
        // }

        let handle = unsafe { device.create_sampler(&self.create_info, None)? };

        Ok(Sampler {
            inner: Arc::new(Inner {
                // handle: SamplerHandle(handle),
                handle,
                device,
            })
        })
    }
}
