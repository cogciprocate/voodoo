use std::ptr;
use std::ffi::{CString, CStr};
use std::ops::Deref;
use std::marker::PhantomData;
use ::Version;
use vks;


pub struct ApplicationInfo<'a> {
    raw: vks::VkApplicationInfo,
    _names: PhantomData<&'a CStr>
}

impl<'a> ApplicationInfo<'a> {
    pub fn new() -> ApplicationInfo<'a> {
        ApplicationInfo {
            raw: vks::VkApplicationInfo::default(),
            _names: PhantomData,
        }
    }

    pub fn application_name<'cs, Cs>(mut self, application_name: Cs)
            -> ApplicationInfo<'a>
            where Cs: 'cs + AsRef<CStr>, 'cs: 'a {
        self.raw.pApplicationName = application_name.as_ref().as_ptr();
        self
    }

    pub fn application_version(mut self, application_version: Version)
            -> ApplicationInfo<'a> {
        self.raw.applicationVersion = application_version.into();
        self
    }

    pub fn engine_name<'cs, Cs>(mut self, engine_name: Cs)
            -> ApplicationInfo<'a>
            where Cs: 'cs + AsRef<CStr>, 'cs: 'a {
        self.raw.pEngineName = engine_name.as_ref().as_ptr();
        self
    }

    pub fn engine_version(mut self, engine_version: Version)
            -> ApplicationInfo<'a> {
        self.raw.engineVersion = engine_version.into();
        self
    }

    pub fn api_version(mut self, api_version: Version)
            -> ApplicationInfo<'a> {
        self.raw.apiVersion = api_version.into();
        self
    }

    pub fn get_application_name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    pub fn get_application_version(&self) -> Version {
        self.raw.applicationVersion.into()
    }

    pub fn get_engine_name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.raw.pApplicationName) }
    }

    pub fn get_engine_version(&self) -> Version {
        self.raw.engineVersion.into()
    }

    pub fn get_api_version(&self) -> Version {
        self.raw.apiVersion.into()
    }

    pub fn raw(&self) -> &vks::VkApplicationInfo {
        &self.raw
    }
}
