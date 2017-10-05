use libc::c_void;
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use winit;
use vks;
use ::{VooResult, Instance};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SurfaceHandle(pub(crate) vks::VkSurfaceKHR);

impl SurfaceHandle {
    pub fn raw(&self) -> vks::VkSurfaceKHR {
        self.0
    }
}


#[derive(Debug)]
struct Inner {
    handle: SurfaceHandle,
    instance: Instance,
    active: AtomicBool,
}

#[derive(Debug, Clone)]
pub struct Surface {
    inner: Arc<Inner>,
}

impl Surface {
    pub fn builder() -> SurfaceBuilder {
        SurfaceBuilder::new()
    }

    pub unsafe fn from_raw(instance: Instance, handle: SurfaceHandle) -> VooResult<Surface> {
        Ok(Surface {
            inner: Arc::new(Inner {
                handle: handle,
                instance: instance,
                active: AtomicBool::new(false),
            })
        })
    }

    pub fn handle(&self) -> SurfaceHandle {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.instance.proc_addr_loader().khr_surface.vkDestroySurfaceKHR(self.instance.handle(), self.handle.0, ptr::null());
        }
    }
}


enum CreateInfo {
    Xlib(vks::VkXlibSurfaceCreateInfoKHR),
    Xcb(vks::VkXcbSurfaceCreateInfoKHR),
    Wayland(vks::VkWaylandSurfaceCreateInfoKHR),
    Mir(vks::VkMirSurfaceCreateInfoKHR),
    Win32(vks::VkWin32SurfaceCreateInfoKHR),
    Android(vks::VkAndroidSurfaceCreateInfoKHR),
    Ios(vks::VkIOSSurfaceCreateInfoMVK),
    MacOs(vks::VkMacOSSurfaceCreateInfoMVK),
    Vi(vks::VkViSurfaceCreateInfoNN),
    None,
}

/// A builder used to create a `Surface`.
pub struct SurfaceBuilder {
    // create_info: vks::VkWin32SurfaceCreateInfoKHR,
    create_info: CreateInfo,
    // enabled_layer_name_ptrs: SmallVec<[*const c_char; 128]>,
    // enabled_extension_name_ptrs: SmallVec<[*const c_char; 128]>,
    // _p: PhantomData<&'ib ()>,
}

impl SurfaceBuilder {
    /// Returns a new surface builder.
    pub fn new() -> SurfaceBuilder {
        SurfaceBuilder {
            create_info: CreateInfo::None,
            // _p: PhantomData,
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn xlib<'s>(&'s mut self, dpy: *mut vks::Display, window: vks::Window)
            -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkXlibSurfaceCreateInfoKHR::default();
        ci.dpy = dpy;
        ci.window = window;
        self.create_info = CreateInfo::Xlib(ci);
        self
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn xcb<'s>(&'s mut self, connection: *mut vks::xcb_connection_t,
            window: vks::xcb_window_t) -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkXcbSurfaceCreateInfoKHR::default();
        ci.connection = connection;
        ci.window = window;
        self.create_info = CreateInfo::Xcb(ci);
        self
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn wayland<'s>(&'s mut self, display: *mut vks::wl_display, surface: *mut vks::wl_surface)
            -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkWaylandSurfaceCreateInfoKHR::default();
        ci.display = display;
        ci.surface = surface;
        self.create_info = CreateInfo::Wayland(ci);
        self
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub unsafe fn mir<'s>(&'s mut self, connection: *mut vks::MirConnection,
            mir_surface: *mut vks::MirSurface) -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkMirSurfaceCreateInfoKHR::default();
        ci.connection = connection;
        ci.mirSurface = mir_surface;
        self.create_info = CreateInfo::Mir(ci);
        self
    }


    #[cfg(target_os = "windows")]
    pub unsafe fn win32<'s>(&'s mut self, hinstance: *mut vks::HINSTANCE_T, hwnd: *mut vks::HWND_T)
            -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkWin32SurfaceCreateInfoKHR::default();
        ci.hinstance = hinstance;
        ci.hwnd = hwnd;
        self.create_info = CreateInfo::Win32(ci);
        self
    }

    #[cfg(target_os = "android")]
    pub unsafe fn android<'s>(&'s mut self, flags: vks::VkAndroidSurfaceCreateFlagsKHR,
            window: *mut vks::ANativeWindow) -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkAndroidSurfaceCreateInfoKHR::default();
        ci.flags = flags;
        ci.window = window;
        self.create_info = CreateInfo::Android(ci);
        self
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn ios<'s>(&'s mut self, view: *const c_void)
            -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkIOSSurfaceCreateInfoMVK::default();
        ci.view = view;
        self.create_info = CreateInfo::Ios(ci);
        self
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn macos<'s>(&'s mut self, view: *const c_void)
            -> &'s mut SurfaceBuilder {
        let mut ci = vks::VkMacOSSurfaceCreateInfoMVK::default();
        ci.view = view;
        self.create_info = CreateInfo::MacOs(ci);
        self
    }

    /// Builds and returns a new `Surface`.
    pub fn build(&self, instance: Instance) -> VooResult<Surface> {
        let mut handle = 0;

        unsafe {
            match self.create_info {
                CreateInfo::Win32(ref ci) => {
                    ::check(instance.proc_addr_loader().khr_win32_surface.vkCreateWin32SurfaceKHR(instance.handle(),
                        ci, ptr::null(), &mut handle));
                },
                CreateInfo::None => panic!("no surface window information provided"),
                _ => unimplemented!(),
            }
        }

        Ok(Surface {
            inner: Arc::new(Inner {
                handle: SurfaceHandle(handle),
                instance: instance,
                active: AtomicBool::new(false),
            })
        })
    }
}
