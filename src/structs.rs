use std::ptr;
use std::ffi::{CString, CStr};
use std::ops::Deref;
use std::marker::PhantomData;
use ::Version;
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
pub struct ApplicationInfo<'a> {
    raw: vks::VkApplicationInfo,
    application_name: Option<CString>,
    engine_name: Option<CString>,
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

    // /// Specifies the application name as a null-terminated byte slice.
    // ///
    // /// Panics if the provided string contains interior null bytes or is not
    // /// null-terminated.
    // pub fn application_name_bytes<'cs, Cs>(mut self, application_name: Cs)
    //         -> ApplicationInfo<'a>
    //         where Cs: 'cs + AsRef<[u8]>, 'cs: 'a {

    //     self.raw.pApplicationName = CStr::from_bytes_with_nul(application_name.as_ref())
    //         .expect("application name provided is not a valid C string").as_ptr();
    //     self
    // }

    /// Specifies the application name without any allocation.
    pub fn application_name_c_str<'cs, Cs>(mut self, application_name: Cs)
            -> ApplicationInfo<'a>
            where Cs: 'cs + AsRef<CStr>, 'cs: 'a {
        if !self.raw.pApplicationName.is_null() { panic!("application name already set") }
        self.raw.pApplicationName = application_name.as_ref().as_ptr();
        self
    }

    /// Specifies the application name.
    pub fn application_name<S>(mut self, application_name: S)
            -> ApplicationInfo<'a>
            where S: AsRef<str> {
        if !self.raw.pApplicationName.is_null() { panic!("application name already set") }
        self.application_name = Some(CString::new(application_name.as_ref())
            .expect("application name contains an interior null byte"));
        self.raw.pApplicationName = self.application_name.as_ref().unwrap().as_ptr();
        self
    }

    /// Specifies the application version.
    pub fn application_version<V: Into<Version>>(mut self, application_version: V)
            -> ApplicationInfo<'a> {
        self.raw.applicationVersion = application_version.into().into();
        self
    }

    // /// Specifies the engine name.
    // ///
    // /// Panics if the provided string contains interior null bytes or is not
    // /// null-terminated.
    // pub fn engine_name_bytes<'cs, Cs>(mut self, engine_name: Cs)
    //         -> ApplicationInfo<'a>
    //         where Cs: 'cs + AsRef<[u8]>, 'cs: 'a {
    //     self.raw.pEngineName = CStr::from_bytes_with_nul(engine_name.as_ref())
    //         .expect("engine name provided is not a valid C string").as_ptr();
    //     self
    // }

    /// Specifies the engine name without any allocation.
    pub fn engine_name_c_str<'cs, Cs>(mut self, engine_name: Cs)
            -> ApplicationInfo<'a>
            where Cs: 'cs + AsRef<CStr>, 'cs: 'a {
        if !self.raw.pEngineName.is_null() { panic!("engine name already set") }
        self.raw.pEngineName = engine_name.as_ref().as_ptr();
        self
    }

    /// Specifies the engine name.
    pub fn engine_name<S>(mut self, engine_name: S)
            -> ApplicationInfo<'a>
            where S: AsRef<str> {
        if !self.raw.pEngineName.is_null() { panic!("engine name already set") }
        self.engine_name = Some(CString::new(engine_name.as_ref())
            .expect("engine name contains an interior null byte"));
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
