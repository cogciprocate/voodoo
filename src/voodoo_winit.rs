use winit::Window;
use ::{VooResult, Instance, SurfaceKhr, wl_display, wl_surface, };

pub use winit;


#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
pub fn create_surface(instance: Instance, window: &Window) -> VooResult<SurfaceKhr> {
    use winit::os::unix::WindowExt;

    let mut sb = SurfaceKhr::builder();
    unsafe {
        if let (Some(display), Some(window)) = (window.get_xlib_display(), window.get_xlib_window()) {
            sb.xlib(display as _, window as _);
        } else if let (Some(display), Some(surface)) = (window.get_wayland_display(), window.get_wayland_surface()) {
            sb.wayland(display as *mut wl_display, surface as *mut wl_surface);
        }
    }

        

    sb.build(instance)
}


#[cfg(target_os = "windows")]
pub fn create_surface(instance: Instance, window: &Window) -> VooResult<SurfaceKhr> {
    use winit::os::windows::WindowExt;
    use std::ptr;

    unsafe {
        SurfaceKhr::builder()
            .win32(ptr::null_mut(), window.get_hwnd() as *mut _)
            .build(instance)
    }
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: Instance, window: &Window) -> VooResult<SurfaceKhr> {
    use winit::os::android::WindowExt;

    unsafe {
        SurfaceKhr::builder()
            .android(window.get_native_window())
            .build(instance)
    }
}

#[cfg(target_os = "macos")]
pub fn create_surface(instance: Instance, window: &Window) -> VooResult<SurfaceKhr> {
    use winit::os::macos::WindowExt;
    let wnd: cocoa_id = mem::transmute(window.get_nswindow());

    let layer = CAMetalLayer::new();
    layer.set_edge_antialiasing_mask(0);
    layer.set_presents_with_transaction(false);
    layer.remove_all_animations();

    let view = wnd.contentView();
    view.setWantsLayer(YES);
    view.setLayer(mem::transmute(layer.0));

    unsafe {
        SurfaceKhr::builder()
            .macos(window.get_nsview() as *const _)
            .build(instance)
    }
}
