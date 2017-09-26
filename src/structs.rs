use std::ptr;
use std::ffi::{CString, CStr};
use std::ops::Deref;
use std::marker::PhantomData;
use ::{Version, CharStr};
use vks;

/// Application information.
///
/// # Example
/// ```text
/// let app_name = CString::new("Hello Triangle").unwrap();
///    let engine_name = CString::new("No Engine").unwrap();
///
///  let app_info = voo::ApplicationInfo::new()
///        .application_name_c_str(app_name)
///        .application_version((1, 0, 0))
///        .engine_name_c_str(engine_name)
///        .engine_version((1, 0, 0))
///        .api_version((1, 0, 51));
/// ```
//
#[derive(Debug, Clone)]
pub struct ApplicationInfo<'a> {
    raw: vks::VkApplicationInfo,
    // application_name: Option<CString>,
    application_name: Option<CharStr<'a>>,
    // engine_name: Option<CString>,
    engine_name: Option<CharStr<'a>>,
    _names: PhantomData<&'a CStr>
}

impl<'a> ApplicationInfo<'a> {
    /// Returns a new `ApplicationInfo` with default values.
    pub fn new() -> ApplicationInfo<'a> {
        ApplicationInfo {
            raw: vks::VkApplicationInfo::default(),
            application_name: None,
            engine_name: None,
            _names: PhantomData,
        }
    }

    /// Specifies the application name.
    ///
    /// Use `CStr::from_bytes_with_nul` to avoid any extra allocation. (e.g.:
    /// `.application_name(CStr::from_bytes_with_nul(b"Application
    /// Name\0").unwrap())`).
    pub fn application_name<'c, S>(mut self, application_name: S)
            -> ApplicationInfo<'a>
            where 'c: 'a, S: Into<CharStr<'c>> {
        self.application_name = Some(application_name.into());
        self.raw.pApplicationName = self.application_name.as_ref().unwrap().as_ptr();
        self
    }

    /// Specifies the application version.
    pub fn application_version<V: Into<Version>>(mut self, application_version: V)
            -> ApplicationInfo<'a> {
        self.raw.applicationVersion = application_version.into().into();
        self
    }

    /// Specifies the engine name.
    ///
    /// Use `CStr::from_bytes_with_nul` to avoid any extra allocation. (e.g.:
    /// `.engine_name(CStr::from_bytes_with_nul(b"Engine Name\0").unwrap())`).
    pub fn engine_name<'c, S>(mut self, engine_name: S)
            -> ApplicationInfo<'a>
            where 'c: 'a, S: Into<CharStr<'c>> {
        self.engine_name = Some(engine_name.into());
        self.raw.pApplicationName = self.engine_name.as_ref().unwrap().as_ptr();
        self
    }

    /// Specifies the engine version.
    pub fn engine_version<V: Into<Version>>(mut self, engine_version: V)
            -> ApplicationInfo<'a> {
        self.raw.engineVersion = engine_version.into().into();
        self
    }

    /// Specifies the API version.
    pub fn api_version<V: Into<Version>>(mut self, api_version: V)
            -> ApplicationInfo<'a> {
        self.raw.apiVersion = api_version.into().into();
        self
    }

    /// Returns the specified application name.
    pub fn get_application_name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    /// Returns the specified application version.
    pub fn get_application_version(&self) -> Version {
        self.raw.applicationVersion.into()
    }

    /// Returns the specified engine name.
    pub fn get_engine_name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    /// Returns the specified engine version.
    pub fn get_engine_version(&self) -> Version {
        self.raw.engineVersion.into()
    }

    /// Returns the specified API version.
    pub fn get_api_version(&self) -> Version {
        self.raw.apiVersion.into()
    }

    /// Returns a reference to the internal `vks::VkApplicationInfo` struct.
    pub fn raw(&self) -> &vks::VkApplicationInfo {
        &self.raw
    }
}


/// Device queue creation information.
///
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VkDeviceQueueCreateInfo {
//     pub sType: VkStructureType,
//     pub pNext: *const c_void,
//     pub flags: VkDeviceQueueCreateFlags,
//     pub queueFamilyIndex: u32,
//     pub queueCount: u32,
//     pub pQueuePriorities: *const f32,
// }
#[derive(Debug, Clone)]
#[repr(C)]
pub struct DeviceQueueCreateInfo<'ci> {
    raw: vks::VkDeviceQueueCreateInfo,
    _qp: PhantomData<&'ci [f32]>,
}

impl<'ci> DeviceQueueCreateInfo<'ci> {
    /// Returns a new `DeviceQueueCreateInfo`.
    pub fn new() -> DeviceQueueCreateInfo<'ci> {
        DeviceQueueCreateInfo {
            raw: vks::VkDeviceQueueCreateInfo::default(),
            _qp: PhantomData,
        }
    }

    /// Specifies the queue family index to use.
    pub fn queue_family_index(mut self, queue_family_index: u32) -> DeviceQueueCreateInfo<'ci> {
        self.raw.queueFamilyIndex = queue_family_index;
        self
    }

    /// Specifies a list of priority ranking from `0.0` to `1.0` for each
    /// queue.
    ///
    /// Passing `&[1.0]` will create a single queue.
    pub fn queue_priorities<'qp>(mut self, queue_priorities: &'qp [f32])
            -> DeviceQueueCreateInfo<'ci>
            where 'qp: 'ci {
        self.raw.queueCount = queue_priorities.len() as u32;
        self.raw.pQueuePriorities = queue_priorities.as_ptr();
        self
    }

    pub fn get_queue_family_index(&self) -> u32 {
        self.raw.queueFamilyIndex
    }

    /// Returns a reference to the internal `vks::VkDeviceQueueCreateInfo`
    /// struct.
    pub fn raw(&self) -> &vks::VkDeviceQueueCreateInfo {
        &self.raw
    }
}