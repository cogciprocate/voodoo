use std::sync::Arc;
use std::ffi::{self, CStr};
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use smallvec::SmallVec;
use libc::{c_char, c_void};
use vks;
use ::{VooResult, Loader, ApplicationInfo, PhysicalDevice, CharStrs};


unsafe extern "system" fn __debug_callback(_flags: vks::VkDebugReportFlagsEXT,
        _obj_type: vks::VkDebugReportObjectTypeEXT, _obj: u64, _location: usize, _code: i32,
        _layer_prefix: *const c_char, msg: *const c_char, _user_data: *mut c_void) -> u32
{
    println!("{}", CStr::from_ptr(msg).to_str().unwrap());
    vks::VK_FALSE
}

// pub unsafe fn extension_names<'en>(extensions: &'en [vks::VkExtensionProperties]) -> Vec<&'en CStr> {
//     extensions.iter().map(|ext| {
//         let name = CStr::from_ptr(&ext.extensionName as *const c_char);
//         println!("Enabling instance extension: '{}' (version: {})",
//             name.to_str().unwrap(), ext.specVersion);
//         name
//     }).collect()
// }

fn enumerate_physical_devices(instance: vks::VkInstance, loader: &vks::InstanceProcAddrLoader)
        -> SmallVec<[vks::VkPhysicalDevice; 16]> {
    let mut device_count = 0;
    let mut devices_raw = SmallVec::new();
    unsafe {
        ::check(loader.core.vkEnumeratePhysicalDevices(instance, &mut device_count, ptr::null_mut()));
        if device_count == 0 { panic!("No physical devices found."); }
        assert!(device_count as usize <= devices_raw.inline_size());
        devices_raw.set_len(device_count as usize);
        ::check(loader.core.vkEnumeratePhysicalDevices(instance, &mut device_count, devices_raw.as_mut_ptr()));
    }
    println!("Available devices: {:?}", devices_raw);
    devices_raw
}


#[derive(Debug)]
struct Inner {
    handle: vks::VkInstance,
    loader: Loader,
    debug_callback: Option<vks::VkDebugReportCallbackEXT>,
    // physical_devices: SmallVec<[PhysicalDevice; 16]>,
}

#[derive(Debug, Clone)]
pub struct Instance {
    inner: Arc<Inner>,
}

impl Instance {
    #[inline]
    pub fn builder<'ib>() -> InstanceBuilder<'ib> {
        InstanceBuilder::new()
    }

    // #[inline]
    // pub fn vk(&self) -> &vks::InstanceProcAddrLoader {
    //     self.inner.loader.loader()
    // }

    #[inline]
    pub fn proc_addr_loader(&self) -> &vks::InstanceProcAddrLoader {
        self.inner.loader.loader()
    }

    #[inline]
    pub fn handle(&self) -> vks::VkInstance {
        self.inner.handle
    }

    #[inline]
    pub fn physical_devices(&self) -> SmallVec<[PhysicalDevice; 16]> {
        enumerate_physical_devices(self.inner.handle, self.inner.loader.loader())
            .iter().map(|&pdr| PhysicalDevice::new(self.clone(), pdr)).collect()
    }

    #[inline]
    pub fn loader(&self) -> &Loader {
        &self.inner.loader
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            println!("Destroying debug callback...");
            if let Some(callback) = self.debug_callback {
                self.loader.loader().ext_debug_report.vkDestroyDebugReportCallbackEXT(self.handle, callback, ptr::null());
            }

            println!("Destroying instance...");
            self.loader.loader().core.vkDestroyInstance(self.handle, ptr::null());
        }
    }
}



/// A builder used to create an `Instance`.
//
// typedef struct VkInstanceCreateInfo {
//     VkStructureType             sType;
//     const void*                 pNext;
//     VkInstanceCreateFlags       flags;
//     const VkApplicationInfo*    pApplicationInfo;
//     uint32_t                    enabledLayerCount;
//     const char* const*          ppEnabledLayerNames;
//     uint32_t                    enabledExtensionCount;
//     const char* const*          ppEnabledExtensionNames;
// } VkInstanceCreateInfo;
//
#[derive(Debug, Clone)]
pub struct InstanceBuilder<'ib> {
    create_info: vks::VkInstanceCreateInfo,
    // enabled_layer_name_ptrs: SmallVec<[*const c_char; 128]>,
    // enabled_extension_name_ptrs: SmallVec<[*const c_char; 128]>,
    enabled_layer_names: Option<CharStrs<'ib>>,
    enabled_extension_names: Option<CharStrs<'ib>>,
    _p: PhantomData<&'ib ()>,
}

impl<'ib> InstanceBuilder<'ib> {
    /// Returns a new instance builder.
    pub fn new() -> InstanceBuilder<'ib> {
        InstanceBuilder {
            create_info: vks::VkInstanceCreateInfo::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            _p: PhantomData,
        }
    }

    /// Sets the application info.
    pub fn application_info<'ai, 's>(&'s mut self, application_info: &'ai ApplicationInfo)
            -> &'s mut InstanceBuilder<'ib>
            where 'ai: 'ib {
        self.create_info.pApplicationInfo = application_info.raw();
        self
    }

    /// Sets the enabled layer names.
    // pub fn enabled_layer_names<'eln, 's>(&'s mut self, enabled_layer_names: &'eln [&'eln CStr])
    pub fn enabled_layer_names<'s, 'cs, Cs>(&'s mut self, enabled_layer_names: Cs)
            -> &'s mut InstanceBuilder<'ib>
            where 'cs: 'ib, Cs: 'cs + Into<CharStrs<'cs>> {
        // let enabled_layer_names = enabled_layer_names.into();
        self.enabled_layer_names = Some(enabled_layer_names.into());
        {
            let elns = self.enabled_layer_names.as_ref().unwrap();
            // match enabled_layer_names {
            //     CharStrs::OwnedPtr { ref ptrs } => {
            //         println!("#### Enabled layer names: {:?}", ptrs);
            //     }
            //     _ => unreachable!(),
            // }
            self.create_info.ppEnabledLayerNames = elns.as_ptr();
            self.create_info.enabledLayerCount = elns.len() as u32;
        }
        // self.enabled_layer_names = Some(enabled_layer_names);
        self
    }

    /// Sets the enabled extension names.
    ///
    /// May not be used with `::enabled_extensions`.
    // pub fn enabled_extension_names<'een, 's>(&'s mut self, enabled_extension_names: &'een [&'een CStr])
    //         -> &'s mut InstanceBuilder<'ib>
    //         where 'een: 'ib {
    pub fn enabled_extension_names<'s, 'cs, Cs>(&'s mut self, enabled_extension_names: Cs)
            -> &'s mut InstanceBuilder<'ib>
            // where 'een: 'ib {
            where 'cs: 'ib, Cs: 'cs + Into<CharStrs<'cs>> {
        if !self.create_info.ppEnabledExtensionNames.is_null() {
            panic!("Enabled extension names have already been set.");
        }
        // let enabled_extension_names = enabled_extension_names.into();
        // for en in enabled_extension_names.into() {
        //     self.enabled_extension_name_ptrs.push(en.as_ptr());
        // }
        self.enabled_extension_names = Some(enabled_extension_names.into());
        {
            let eens = self.enabled_extension_names.as_ref().unwrap();
            self.create_info.ppEnabledExtensionNames = eens.as_ptr();
            self.create_info.enabledExtensionCount = eens.len() as u32;
        }
        // self.enabled_extension_names = Some(enabled_extension_names);
        self
    }

    /// Sets the enabled extension names by providing a list of extensions.
    ///
    /// May not be used with `::enabled_extension_lists`.
    pub fn enabled_extensions<'een, 's>(&'s mut self, enabled_extensions: &'een [vks::VkExtensionProperties])
            -> &'s mut InstanceBuilder<'ib>
            where 'een: 'ib {
        if !self.create_info.ppEnabledExtensionNames.is_null() {
            panic!("Enabled extension names have already been set.");
        }
        // let mut enabled_extension_name_ptrs: SmallVec<[*const c_char; 8]> = SmallVec::new();
        // // let mut enabled_extension_name_ptrs: Vec<*const c_char> = Vec::new();
        // for eext in enabled_extensions {
        //     println!("Enabling instance extension: '{}' (version: {})",
        //         unsafe { CStr::from_ptr(&eext.extensionName as *const c_char).to_str().unwrap() },
        //             eext.specVersion);
        //     enabled_extension_name_ptrs.push(eext.extensionName.as_ptr());
        // }

        let enabled_extension_name_ptrs: SmallVec<[_; 8]> = enabled_extensions.iter().map(|eext| {
        // let enabled_extension_name_ptrs: Vec<_> = enabled_extensions.iter().map(|eext| {
            println!("Enabling instance extension: '{}' (version: {})",
                unsafe { CStr::from_ptr(&eext.extensionName as *const c_char).to_str().unwrap() },
                    eext.specVersion);
            eext.extensionName.as_ptr()
        }).collect();

        self.enabled_extension_names = Some(CharStrs::OwnedPtr { ptrs: enabled_extension_name_ptrs });
        {
            let eens = self.enabled_extension_names.as_ref().unwrap();
            self.create_info.ppEnabledExtensionNames = eens.as_ptr();
            self.create_info.enabledExtensionCount = eens.len() as u32;
        }
        // self.enabled_extension_names = Some(CharStrs::OwnedPtr { ptrs: enabled_extension_name_ptrs });
        self
    }

    /// Builds and returns a new `Instance`.
    pub fn build(&self, mut loader: Loader, enable_debug_callback: bool) -> VooResult<Instance> {
        let mut handle = ptr::null_mut();

        unsafe {
            ::check(loader.core_global().vkCreateInstance(&self.create_info, ptr::null(), &mut handle));
            // [FIXME: do this properly] Load extension function pointers:
            loader.loader_mut().load_core(handle);
            loader.loader_mut().load_khr_surface(handle);
            loader.loader_mut().load_khr_win32_surface(handle);
            loader.loader_mut().load_khr_get_physical_device_properties2(handle);
            loader.loader_mut().load_khr_external_memory_capabilities(handle);
        }

        // TODO: Ensure that the debug extension is enabled by consulting the
        // enabled extension list instead.
        if enable_debug_callback { unsafe { loader.loader_mut().load_ext_debug_report(handle); } }

        // TODO: Ensure that the debug extension is enabled by consulting the
        // enabled extension list instead.
        let debug_callback = if enable_debug_callback {
            let create_info = vks::VkDebugReportCallbackCreateInfoEXT {
                sType:  vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
                pNext: ptr::null(),
                flags: vks::VK_DEBUG_REPORT_ERROR_BIT_EXT | vks::VK_DEBUG_REPORT_WARNING_BIT_EXT,
                pfnCallback: Some(__debug_callback),
                pUserData: ptr::null_mut(),
            };

            let mut callback: vks::VkDebugReportCallbackEXT = 0;
            if unsafe { loader.loader().ext_debug_report.vkCreateDebugReportCallbackEXT(handle,
                    &create_info, ptr::null(), &mut callback) } != vks::VK_SUCCESS
            {
                panic!("failed to set up debug callback");
            } else {
                println!("Debug report callback initialized.");
            }
            Some(callback)
        } else {
            None
        };

        // // Device:
        // let physical_devices = unsafe { enumerate_physical_devices(handle, loader.loader()) };

        Ok(Instance {
            inner: Arc::new(Inner {
                handle,
                loader,
                debug_callback,
                // physical_devices,
            }),
        })
    }
}
