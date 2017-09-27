
use std::sync::Arc;
use std::ptr;
use std::marker::PhantomData;
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
    //         ::check(device.proc_addr_loader().vkCreateSampler(device.handle(), &create_info,
    //             ptr::null(), &mut handle));
    //     }

    //     Ok(Sampler {
    //         inner: Arc::new(Inner {
    //             handle,
    //             device,
    //         })
    //     })
    // }

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
            self.device.proc_addr_loader().vkDestroySampler(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `Sampler`.
//
// typedef struct VkSamplerCreateInfo {
//     VkStructureType         sType;
//     const void*             pNext;
//     VkSamplerCreateFlags    flags;
//     VkFilter                magFilter;
//     VkFilter                minFilter;
//     VkSamplerMipmapMode     mipmapMode;
//     VkSamplerAddressMode    addressModeU;
//     VkSamplerAddressMode    addressModeV;
//     VkSamplerAddressMode    addressModeW;
//     float                   mipLodBias;
//     VkBool32                anisotropyEnable;
//     float                   maxAnisotropy;
//     VkBool32                compareEnable;
//     VkCompareOp             compareOp;
//     float                   minLod;
//     float                   maxLod;
//     VkBorderColor           borderColor;
//     VkBool32                unnormalizedCoordinates;
// } VkSamplerCreateInfo;
//
#[derive(Debug, Clone)]
pub struct SamplerBuilder<'b> {
    create_info: vks::VkSamplerCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> SamplerBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> SamplerBuilder<'b> {
        SamplerBuilder {
            create_info: vks::VkSamplerCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// Reserved for future use.
    pub fn flags<'s>(&'s mut self, flags: vks::VkSamplerCreateFlags)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// Specifies the magnification filter to
    /// apply to lookups.
    pub fn mag_filter<'s>(&'s mut self, mag_filter: vks::VkFilter)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.magFilter = mag_filter;
        self
    }

    /// Specifies the minification filter to apply to lookups.
    pub fn min_filter<'s>(&'s mut self, min_filter: vks::VkFilter)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.minFilter = min_filter;
        self
    }

    /// Specifies the mipmap filter to apply to lookups.
    pub fn mipmap_mode<'s>(&'s mut self, mipmap_mode: vks::VkSamplerMipmapMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.mipmapMode = mipmap_mode;
        self
    }

    /// Specifies the addressing mode for outside [0..1] range for U
    /// coordinate.
    pub fn address_mode_u<'s>(&'s mut self, address_mode_u: vks::VkSamplerAddressMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.addressModeU = address_mode_u;
        self
    }

    /// Specifies the addressing mode for outside [0..1] range for V
    /// coordinate.
    pub fn address_mode_v<'s>(&'s mut self, address_mode_v: vks::VkSamplerAddressMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.addressModeV = address_mode_v;
        self
    }

    /// Specifies the addressing mode for outside [0..1] range for W
    /// coordinate.
    pub fn address_mode_w<'s>(&'s mut self, address_mode_w: vks::VkSamplerAddressMode)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.addressModeW = address_mode_w;
        self
    }

    /// Specifies the the bias to be added to mipmap LOD calculation and bias
    /// provided by image sampling functions in SPIR-V, as described in the
    /// Level-of-Detail Operation section [TODO: INSERT LINK].
    pub fn mip_lod_bias<'s>(&'s mut self, mip_lod_bias: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.mipLodBias = mip_lod_bias;
        self
    }

    /// Specifies whether or not to enable anisotropic filtering, as described
    /// in the Texel Anisotropic Filtering section.
    pub fn anisotropy_enable<'s>(&'s mut self, anisotropy_enable: bool)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.anisotropyEnable =
            if anisotropy_enable { vks::VK_TRUE } else { vks::VK_FALSE };
        self
    }

    /// Specifies the anisotropy value clamp.
    pub fn max_anisotropy<'s>(&'s mut self, max_anisotropy: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.maxAnisotropy = max_anisotropy;
        self
    }

    /// Specifies whether or not to enable comparison against a reference
    /// value during lookups.
    ///
    /// Note: Some implementations will default to shader state if this member
    /// does not match.
    pub fn compare_enable<'s>(&'s mut self, compare_enable: bool)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.compareEnable =
            if compare_enable { vks::VK_TRUE } else { vks::VK_FALSE };
        self
    }

    /// Specifies the comparison function to apply to fetched data before
    /// filtering as described in the Depth Compare Operation section.
    pub fn compare_op<'s>(&'s mut self, compare_op: vks::VkCompareOp)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.compareOp = compare_op;
        self
    }

    /// Specifies the minimum value used to clamp the computed level-of-detail
    /// value, as described in the Level-of-Detail Operation section.
    /// `max_lod` must be greater than or equal to `min_lod`.
    pub fn min_lod<'s>(&'s mut self, min_lod: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.minLod = min_lod;
        self
    }

    /// Specifies the maximum value used to clamp the computed level-of-detail
    /// value, as described in the Level-of-Detail Operation section. `max_lod`
    /// must be greater than or equal to `min_lod`.
    pub fn max_lod<'s>(&'s mut self, max_lod: f32)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.maxLod = max_lod;
        self
    }

    /// Specifies the predefined border color to use.
    pub fn border_color<'s>(&'s mut self, border_color: vks::VkBorderColor)
            -> &'s mut SamplerBuilder<'b> {
        self.create_info.borderColor = border_color;
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
        self.create_info.unnormalizedCoordinates =
            if unnormalized_coordinates { vks::VK_TRUE } else { vks::VK_FALSE };
        self
    }

    /// Creates and returns a new `Sampler`
    pub fn build(&self, device: Device) -> VooResult<Sampler> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateSampler(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        Ok(Sampler {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }
}
