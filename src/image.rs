
use std::sync::Arc;
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use vks;
use ::{util, VooResult, Device, DeviceMemory, PRINT};

#[derive(Debug)]
struct Inner {
    handle: vks::VkImage,
    // device_memory: Option<DeviceMemory>,
    memory_requirements: vks::VkMemoryRequirements,
    device: Device,
}

#[derive(Debug, Clone)]
pub struct Image {
    inner: Arc<Inner>,
}

impl Image {
    /// Returns a new `ImageBuilder`.
    pub fn builder<'b>() -> ImageBuilder<'b> {
        ImageBuilder::new()
    }

    // pub fn new(device: Device, extent: vks::VkExtent3D, format: vks::VkFormat,
    //         tiling: vks::VkImageTiling, usage: vks::VkImageUsageFlags,
    //         memory_properties: vks::VkMemoryPropertyFlags) -> VooResult<Image> {
    //     let create_info = vks::VkImageCreateInfo {
    //         sType: vks::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
    //         pNext: ptr::null(),
    //         flags: 0,
    //         imageType: vks::VK_IMAGE_TYPE_2D,
    //         // format: vks::VK_FORMAT_R8G8B8A8_UNORM,
    //         format,
    //         extent: extent,
    //         mipLevels: 1,
    //         arrayLayers: 1,
    //         samples: vks::VK_SAMPLE_COUNT_1_BIT,
    //         // tiling: vks::VK_IMAGE_TILING_OPTIMAL,
    //         tiling,
    //         // usage: vks::VK_IMAGE_USAGE_TRANSFER_DST_BIT | vks::VK_IMAGE_USAGE_SAMPLED_BIT,
    //         usage,
    //         sharingMode: vks::VK_SHARING_MODE_EXCLUSIVE,
    //         queueFamilyIndexCount: 0,
    //         pQueueFamilyIndices: ptr::null(),
    //         initialLayout: vks::VK_IMAGE_LAYOUT_UNDEFINED,
    //     };

    //     let mut handle = 0;
    //     unsafe {
    //         ::check(device.proc_addr_loader().vkCreateImage(device.handle(), &create_info,
    //             ptr::null(), &mut handle));
    //     }

    //     // Memory Requirements:
    //     let mut memory_requirements: vks::VkMemoryRequirements;
    //     unsafe {
    //         memory_requirements = mem::uninitialized();
    //         device.proc_addr_loader().core.vkGetImageMemoryRequirements(device.handle(), handle,
    //             &mut memory_requirements);
    //     }

    //     if PRINT { println!("Image: {:?}", memory_requirements); }

    //     Ok(Image {
    //         inner: Arc::new(Inner {
    //             handle,
    //             // device_memory,
    //             memory_requirements,
    //             device,
    //         })
    //     })
    // }

    pub fn handle(&self) -> vks::VkImage {
        self.inner.handle
    }

    // pub fn device_memory(&self) -> &DeviceMemory {
    //     &self.inner.device_memory
    // }

    pub fn memory_requirements(&self) -> &vks::VkMemoryRequirements {
        &self.inner.memory_requirements
    }

    /// Binds this image to device memory. `offset` is the start offset of the
    /// region of memory which is to be bound. The number of bytes returned in
    /// the VkMemoryRequirements::size member in memory, starting from
    /// memoryOffset bytes, will be bound to the specified image.
    pub fn bind_memory(&self, device_memory: &DeviceMemory, offset: vks::VkDeviceSize) -> VooResult<()> {
        unsafe {
            ::check(self.inner.device.proc_addr_loader().vkBindImageMemory(
                self.inner.device.handle(), self.inner.handle, device_memory.handle(), offset));
        }
        Ok(())
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.proc_addr_loader().vkDestroyImage(self.device.handle(), self.handle, ptr::null());
        }
    }
}


/// A builder for `Image`.
//
// typedef struct VkImageCreateInfo {
//     VkStructureType          sType;
//     const void*              pNext;
//     VkImageCreateFlags       flags;
//     VkImageType              imageType;
//     VkFormat                 format;
//     VkExtent3D               extent;
//     uint32_t                 mipLevels;
//     uint32_t                 arrayLayers;
//     VkSampleCountFlagBits    samples;
//     VkImageTiling            tiling;
//     VkImageUsageFlags        usage;
//     VkSharingMode            sharingMode;
//     uint32_t                 queueFamilyIndexCount;
//     const uint32_t*          pQueueFamilyIndices;
//     VkImageLayout            initialLayout;
// } VkImageCreateInfo;
//
#[derive(Debug, Clone)]
pub struct ImageBuilder<'b> {
    create_info: vks::VkImageCreateInfo,
    _p: PhantomData<&'b ()>,
}

impl<'b> ImageBuilder<'b> {
    /// Returns a new render pass builder.
    pub fn new() -> ImageBuilder<'b> {
        ImageBuilder {
            create_info: vks::VkImageCreateInfo::default(),
            _p: PhantomData,
        }
    }

    /// flags is a bitmask of VkImageCreateFlagBits describing additional
    /// parameters of the image.
    pub fn flags<'s>(&'s mut self, flags: vks::VkImageCreateFlags)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.flags = flags;
        self
    }

    /// imageType is a VkImageType value specifying the basic dimensionality
    /// of the image. Layers in array textures do not count as a dimension for
    /// the purposes of the image type.
    pub fn image_type<'s>(&'s mut self, image_type: vks::VkImageType)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.imageType = image_type;
        self
    }

    /// format is a VkFormat describing the format and type of the data
    /// elements that will be contained in the image.
    pub fn format<'s>(&'s mut self, format: vks::VkFormat)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.format = format;
        self
    }

    /// extent is a VkExtent3D describing the number of data elements in each
    /// dimension of the base level.
    pub fn extent<'s>(&'s mut self, extent: vks::VkExtent3D)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.extent = extent;
        self
    }

    /// mipLevels describes the number of levels of detail available for
    /// minified sampling of the image.
    pub fn mip_levels<'s>(&'s mut self, mip_levels: u32)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.mipLevels = mip_levels;
        self
    }

    /// arrayLayers is the number of layers in the image.
    pub fn array_layers<'s>(&'s mut self, array_layers: u32)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.arrayLayers = array_layers;
        self
    }

    /// samples is the number of sub-data element samples in the image as
    /// defined in VkSampleCountFlagBits. See Multisampling.
    pub fn samples<'s>(&'s mut self, samples: vks::VkSampleCountFlagBits)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.samples = samples;
        self
    }

    /// tiling is a VkImageTiling value specifying the tiling arrangement of
    /// the data elements in memory.
    pub fn tiling<'s>(&'s mut self, tiling: vks::VkImageTiling)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.tiling = tiling;
        self
    }

    /// usage is a bitmask of VkImageUsageFlagBits describing the intended
    /// usage of the image.
    pub fn usage<'s>(&'s mut self, usage: vks::VkImageUsageFlags)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.usage = usage;
        self
    }

    /// sharingMode is a VkSharingMode value specifying the sharing mode of
    /// the image when it will be accessed by multiple queue families.
    pub fn sharing_mode<'s>(&'s mut self, sharing_mode: vks::VkSharingMode)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.sharingMode = sharing_mode;
        self
    }

    /// queueFamilyIndexCount is the number of entries in the
    /// pQueueFamilyIndices array.
    /// pQueueFamilyIndices is a list of queue families that will access this
    /// image (ignored if sharingMode is not VK_SHARING_MODE_CONCURRENT).
    pub fn queue_family_indices<'s, 'p>(&'s mut self,
            queue_family_indices: &'p [u32])
            -> &'s mut ImageBuilder<'b>
            where 'p: 'b {
        self.create_info.queueFamilyIndexCount = queue_family_indices.len() as u32;
        self.create_info.pQueueFamilyIndices = queue_family_indices.as_ptr();
        self
    }

    /// initialLayout is a VkImageLayout value specifying the initial
    /// VkImageLayout of all image subresources of the image. See Image
    /// Layouts.
    pub fn initial_layout<'s>(&'s mut self, initial_layout: vks::VkImageLayout)
            -> &'s mut ImageBuilder<'b> {
        self.create_info.initialLayout = initial_layout;
        self
    }


    //// Creates and returns a new `Image`
    pub fn build(&self, device: Device) -> VooResult<Image> {
        let mut handle = 0;
        unsafe {
            ::check(device.proc_addr_loader().core.vkCreateImage(device.handle(),
                &self.create_info, ptr::null(), &mut handle));
        }

        // Memory Requirements:
        let mut memory_requirements: vks::VkMemoryRequirements;
        unsafe {
            memory_requirements = mem::uninitialized();
            device.proc_addr_loader().core.vkGetImageMemoryRequirements(device.handle(), handle,
                &mut memory_requirements);
        }

        Ok(Image {
            inner: Arc::new(Inner {
                handle,
                // device_memory,
                memory_requirements,
                device,
            })
        })
    }
}