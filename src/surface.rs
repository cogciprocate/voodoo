// use libc::c_void;
// use std::ptr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
// use winit;
use vks;
use ::{VdResult, Instance, Handle, XlibSurfaceCreateInfoKhr, XcbSurfaceCreateInfoKhr,
    WaylandSurfaceCreateInfoKhr, MirSurfaceCreateInfoKhr, Win32SurfaceCreateInfoKhr,
    AndroidSurfaceCreateInfoKhr, IosSurfaceCreateInfoMvk, MacOsSurfaceCreateInfoMvk,
    ViSurfaceCreateInfoNn};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SurfaceKhrHandle(pub(crate) vks::VkSurfaceKHR);

impl SurfaceKhrHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkSurfaceKHR {
        self.0
    }
}

unsafe impl Handle for SurfaceKhrHandle {
    type Target = SurfaceKhrHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: SurfaceKhrHandle,
    instance: Instance,
    active: AtomicBool,
}

#[derive(Debug, Clone)]
pub struct SurfaceKhr {
    inner: Arc<Inner>,
}

impl SurfaceKhr {
    pub fn builder<'b>() -> SurfaceKhrBuilder<'b> {
        SurfaceKhrBuilder::new()
    }

    pub unsafe fn from_raw(instance: Instance, handle: SurfaceKhrHandle) -> VdResult<SurfaceKhr> {
        Ok(SurfaceKhr {
            inner: Arc::new(Inner {
                handle: handle,
                instance: instance,
                active: AtomicBool::new(false),
            })
        })
    }

    pub fn handle(&self) -> SurfaceKhrHandle {
        self.inner.handle
    }
}

unsafe impl<'s> Handle for &'s SurfaceKhr {
    type Target = SurfaceKhrHandle;

    #[inline(always)]
    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        // unsafe {
        //     self.instance.proc_addr_loader().khr_surface.vkDestroySurfaceKHR(self.instance.handle().0,
        //         self.handle.0, ptr::null());
        // }
        unsafe { self.instance.destroy_surface_khr(self.handle, None); }
    }
}


#[allow(dead_code)]
enum CreateInfo<'c> {
    Xlib(XlibSurfaceCreateInfoKhr<'c>),
    Xcb(XcbSurfaceCreateInfoKhr<'c>),
    Wayland(WaylandSurfaceCreateInfoKhr<'c>),
    Mir(MirSurfaceCreateInfoKhr<'c>),
    Win32(Win32SurfaceCreateInfoKhr<'c>),
    Android(AndroidSurfaceCreateInfoKhr<'c>),
    Ios(IosSurfaceCreateInfoMvk<'c>),
    MacOs(MacOsSurfaceCreateInfoMvk<'c>),
    Vi(ViSurfaceCreateInfoNn<'c>),
    None,
}

/// A builder used to create a `Surface`.
pub struct SurfaceKhrBuilder<'b> {
    create_info: CreateInfo<'b>,
}

impl<'b> SurfaceKhrBuilder<'b> {
    /// Returns a new surface builder.
    pub fn new() -> SurfaceKhrBuilder<'b> {
        SurfaceKhrBuilder {
            create_info: CreateInfo::None,
            // _p: PhantomData,
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn xlib<'s>(&'s mut self, dpy: *mut vks::Display, window: vks::Window)
            -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = XlibSurfaceCreateInfoKhr::default();
        ci.set_dpy(dpy);
        ci.set_window(window);
        self.create_info = CreateInfo::Xlib(ci);
        self
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn xcb<'s>(&'s mut self, connection: *mut vks::xcb_connection_t,
            window: vks::xcb_window_t) -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = XcbSurfaceCreateInfoKhr::default();
        ci.set_connection(connection);
        ci.set_window(window);
        self.create_info = CreateInfo::Xcb(ci);
        self
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn wayland<'s>(&'s mut self, display: *mut vks::wl_display, surface: *mut vks::wl_surface)
            -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = WaylandSurfaceCreateInfoKhr::default();
        ci.set_display(display);
        ci.set_surface(surface);
        self.create_info = CreateInfo::Wayland(ci);
        self
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn mir<'s>(&'s mut self, connection: *mut vks::MirConnection,
            mir_surface: *mut vks::MirSurface) -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = MirSurfaceCreateInfoKhr::default();
        ci.set_connection(connection);
        ci.set_mir_surface(mir_surface);
        self.create_info = CreateInfo::Mir(ci);
        self
    }


    #[cfg(target_os = "windows")]
    pub unsafe fn win32<'s>(&'s mut self, hinstance: *mut vks::HINSTANCE_T, hwnd: *mut vks::HWND_T)
            -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = Win32SurfaceCreateInfoKhr::default();
        ci.set_hinstance(hinstance);
        ci.set_hwnd(hwnd);
        self.create_info = CreateInfo::Win32(ci);
        self
    }

    #[cfg(target_os = "android")]
    pub unsafe fn android<'s>(&'s mut self, flags: AndroidSurfaceCreateFlagsKhr,
            window: *mut vks::ANativeWindow) -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = AndroidSurfaceCreateInfoKhr::default();
        ci.set_flags(flags);
        ci.set_window(window);
        self.create_info = CreateInfo::Android(ci);
        self
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn ios<'s>(&'s mut self, view: *const c_void)
            -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = IOSSurfaceCreateInfoMVK::default();
        ci.set_view(view);
        self.create_info = CreateInfo::Ios(ci);
        self
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn macos<'s>(&'s mut self, view: *const c_void)
            -> &'s mut SurfaceKhrBuilder<'b> {
        let mut ci = MacOSSurfaceCreateInfoMVK::default();
        ci.set_view(view);
        self.create_info = CreateInfo::MacOs(ci);
        self
    }

    /// Builds and returns a new `SurfaceKhr`.
    pub fn build(&self, instance: Instance) -> VdResult<SurfaceKhr> {
        let handle = unsafe {
            match self.create_info {
                CreateInfo::Xlib(ref ci) => instance.create_xlib_surface_khr(ci, None)?,
                CreateInfo::Xcb(ref ci) => instance.create_xcb_surface_khr(ci, None)?,
                CreateInfo::Wayland(ref ci) => instance.create_wayland_surface_khr(ci, None)?,
                CreateInfo::Mir(ref ci) => instance.create_mir_surface_khr(ci, None)?,
                CreateInfo::Win32(ref ci) => instance.create_win32_surface_khr(ci, None)?,
                CreateInfo::Android(ref ci) => instance.create_android_surface_khr(ci, None)?,
                CreateInfo::Ios(ref ci) => instance.create_ios_surface_mvk(ci, None)?,
                CreateInfo::MacOs(ref ci) => instance.create_mac_os_surface_mvk(ci, None)?,
                CreateInfo::Vi(ref ci) => instance.create_vi_surface_nn(ci, None)?,
                CreateInfo::None => panic!("no surface window information provided"),
            }
        };

        Ok(SurfaceKhr {
            inner: Arc::new(Inner {
                handle,
                instance: instance,
                active: AtomicBool::new(false),
            })
        })
    }
}
