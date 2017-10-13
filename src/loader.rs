// use std::ffi::OsStr;
// use std::mem;
use std::ptr;
use std::fmt;
use std::ffi::CStr;
use libc::{c_char};
use lib;
use smallvec::SmallVec;
use vks::{self};
use ::{error, VdResult, Handle, InstanceHandle, CallResult};

const PRINT: bool = false;

pub struct Loader {
    _vk_lib: lib::Library,
    vk_get_instance_proc_addr: vks::PFN_vkGetInstanceProcAddr,
    instance_proc_addr_loader: vks::InstanceProcAddrLoader,
}

impl Loader {
    pub fn new() -> VdResult<Loader> {
        let lib_filename = if cfg!(not(any(target_os = "macos", target_os = "ios"))) {
            if cfg!(all(unix, not(target_os = "android"), not(target_os = "macos"))) { "libvulkan.so.1" }
            else if cfg!(target_os = "android") { "libvulkan.so" }
            else if cfg!(windows) { "vulkan-1.dll" }
            else { unimplemented!("unknown operating system") }
        } else {
            unimplemented!("macos not implemented");
        };
        let vk_lib = lib::Library::new(lib_filename).unwrap();

        let vk_get_instance_proc_addr = unsafe {
            let fn_name = "vkGetInstanceProcAddr";

            let get_proc_addr: lib::Symbol<vks::PFN_vkGetInstanceProcAddr> = vk_lib.get(fn_name.as_bytes()).unwrap();
            *get_proc_addr
        };

        let mut instance_proc_addr_loader = vks::InstanceProcAddrLoader::from_get_instance_proc_addr(vk_get_instance_proc_addr);
        unsafe {
            instance_proc_addr_loader.load_core_global();
        }

        Ok(Loader { _vk_lib: vk_lib, vk_get_instance_proc_addr, instance_proc_addr_loader })
    }

    #[inline]
    pub fn get_instance_proc_addr(&self)
            -> Option<unsafe extern "system" fn(*mut vks::VkInstance_T, *const i8)
                -> Option<unsafe extern "system" fn()>> {
        self.vk_get_instance_proc_addr
    }

    #[inline]
    pub fn core_global(&self) -> &vks::instance_proc_addr_loader::CoreGlobal {
        &self.instance_proc_addr_loader.core_global
    }

    #[inline]
    pub fn instance_proc_addr_loader(&self) -> &vks::InstanceProcAddrLoader {
        &self.instance_proc_addr_loader
    }

    #[inline]
    pub fn instance_proc_addr_loader_mut(&mut self) -> &mut vks::InstanceProcAddrLoader {
        &mut self.instance_proc_addr_loader
    }

    /// Returns all available instance layers.
    pub fn enumerate_instance_layer_properties(&self) -> VdResult<SmallVec<[vks::VkLayerProperties; 64]>> {
        let mut property_count = 0u32;
        let mut properties = SmallVec::new();
        unsafe {
            let result = self.core_global().vkEnumerateInstanceLayerProperties(&mut property_count,
                ptr::null_mut());
            error::check(result, "vkEnumerateInstanceLayerProperties", ())?;
            properties.reserve_exact(property_count as usize);
            properties.set_len(property_count as usize);
            loop {
                let result = self.core_global().vkEnumerateInstanceLayerProperties(&mut property_count,
                    properties.as_mut_ptr());
                if result != CallResult::Incomplete as i32 {
                    error::check(result, "vkEnumerateInstanceLayerProperties", ())?;
                    break;
                }
            }
        }
        Ok(properties)
    }

    /// Returns all available instance extensions.
    pub fn enumerate_instance_extension_properties(&self) -> VdResult<SmallVec<[vks::VkExtensionProperties; 64]>> {
        let mut property_count = 0u32;
        let mut properties = SmallVec::new();
        unsafe {
            let result = self.core_global().vkEnumerateInstanceExtensionProperties(ptr::null(),
                &mut property_count, ptr::null_mut());
            error::check(result, "vkEnumerateInstanceExtensionProperties", ())?;
            properties.reserve_exact(property_count as usize);
            properties.set_len(property_count as usize);
            loop {
                let result = self.core_global().vkEnumerateInstanceExtensionProperties(ptr::null(),
                    &mut property_count, properties.as_mut_ptr());
                if result != CallResult::Incomplete as i32 {
                    error::check(result, "vkEnumerateInstanceExtensionProperties", ())?;
                    break;
                }
            }
            // Print available:
            for ext in properties.iter() {
                let name = (&ext.extensionName) as *const c_char;
                if PRINT { println!("Available instance extension: '{}' (version: {})",
                    CStr::from_ptr(name).to_str().unwrap(), ext.specVersion); }
            }
        }
        Ok(properties)
    }

    /// Verifies that each layer name listed is available.
    pub fn check_layer_availability(&self, layer_names: &[&[u8]]) -> VdResult<bool> {
        let available_layers = self.enumerate_instance_layer_properties()?;
        // Print available layers:
        for layer_props in &available_layers {
            unsafe {
                if PRINT { println!("Available layer: '{}'",
                    CStr::from_ptr(layer_props.layerName.as_ptr()).to_str().unwrap()); }
            }
        }

        // Verify that validation layer is available:
        for &layer_name in layer_names {
            let mut layer_found = false;
            for layer_props in &available_layers {
                unsafe {
                    if CStr::from_ptr(layer_name.as_ptr() as *const c_char) ==
                        CStr::from_ptr(layer_props.layerName.as_ptr())
                    {
                        if PRINT { println!("Layer validated: '{}'", CStr::from_ptr(
                            layer_name.as_ptr() as *const c_char).to_str().unwrap()); }
                        layer_found = true;
                        break;
                    }
                }
            }
            if !layer_found { return Ok(false); }
        }
        Ok(true)
    }

    // *PFN_vkEnumeratePhysicalDevices)(VkInstance instance, uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices);
    pub fn enumerate_physical_devices<I>(&self, instance: I)
            -> VdResult<SmallVec<[vks::VkPhysicalDevice; 16]>>
            where I: Handle<Target=InstanceHandle> {
        let mut device_count = 0;
        let mut devices_raw = SmallVec::new();
        unsafe {
            error::check(self.instance_proc_addr_loader.core.vkEnumeratePhysicalDevices(instance.handle().0,
                &mut device_count, ptr::null_mut()), "vkEnumeratePhysicalDevices", ())?;
            if device_count == 0 { panic!("No physical devices found."); }
            assert!(device_count as usize <= devices_raw.inline_size());
            devices_raw.set_len(device_count as usize);
            loop {
                let result = self.instance_proc_addr_loader.core.vkEnumeratePhysicalDevices(instance.handle().0,
                    &mut device_count, devices_raw.as_mut_ptr());
                if result != CallResult::Incomplete as i32 {
                        error::check(result, "vkEnumeratePhysicalDevices", ())?;
                        break;
                    }
                }
            }
        if PRINT { println!("Available devices: {:?}", devices_raw); }
        Ok(devices_raw)
    }

                // *PFN_vkCreateInstance)(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance);

    pub fn create_instance(&self) {

    }

                // *PFN_vkDestroyInstance)(VkInstance instance, const VkAllocationCallbacks* pAllocator);

    pub fn destroy_instance(&self) {

    }
}

impl fmt::Debug for Loader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Loader")
    }
}