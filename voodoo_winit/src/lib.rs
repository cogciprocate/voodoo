pub extern crate winit;
pub extern crate voodoo;

use winit::Window as WinitWindow;
use voodoo::{Result as VdResult, Instance, SurfaceKhr};


#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
pub fn create_surface(instance: Instance, window: &WinitWindow) -> VdResult<SurfaceKhr> {
    use winit::os::unix::WindowExt;
    use voodoo::{wl_display, wl_surface};

    let mut sb = SurfaceKhr::builder();
    unsafe {
        if let (Some(display), Some(xlib_window)) = (window.get_xlib_display(),
                window.get_xlib_window()) {
            sb.xlib(display as _, xlib_window as _);
        } else if let (Some(display), Some(surface)) = (window.get_wayland_display(),
                window.get_wayland_surface()) {
            sb.wayland(display as *mut wl_display, surface as *mut wl_surface);
        }
    }
    sb.build(instance)
}

#[cfg(target_os = "windows")]
pub fn create_surface(instance: Instance, window: &WinitWindow) -> VdResult<SurfaceKhr> {
    use winit::os::windows::WindowExt;
    use std::ptr;

    unsafe {
        SurfaceKhr::builder()
            .win32(ptr::null_mut(), window.get_hwnd() as *mut _)
            .build(instance)
    }
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: Instance, window: &WinitWindow) -> VdResult<SurfaceKhr> {
    use winit::os::android::WindowExt;

    unsafe {
        SurfaceKhr::builder()
            .android(window.get_native_window())
            .build(instance)
    }
}

#[cfg(target_os = "macos")]
pub fn create_surface(instance: Instance, window: &WinitWindow) -> VdResult<SurfaceKhr> {
    use winit::os::macos::WindowExt;

    unimplemented!();
}
