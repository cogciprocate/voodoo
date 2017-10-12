extern crate voodoo;

use voodoo::{Result as VdResult, Instance, ApplicationInfo, Loader};
use std::ffi::CString;

/// Initializes and returns a new loader and instance with all available
/// extension function pointers loaded.
fn init_instance() -> VdResult<Instance> {
    let app_name = CString::new("Hello!")?;

    let app_info = ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version((1, 0, 0))
        .api_version((1, 0, 0))
        .build();

    let loader = Loader::new()?;

    Instance::builder()
        .application_info(&app_info)
        .enabled_extensions(&loader.enumerate_instance_extension_properties()?)
        .build(loader)
}

fn main() {
    let _instance = init_instance().unwrap();
}