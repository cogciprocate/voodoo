use libc::c_void;
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use winit;
use vks;
use ::{VooResult, Instance};


#[derive(Debug)]
struct Inner {
    handle: vks::khr_surface::VkSurfaceKHR,
    instance: Instance,
    active: AtomicBool,
}

#[derive(Debug, Clone)]
pub struct Surface {
    inner: Arc<Inner>,
}

impl Surface {
    pub fn new(instance: Instance, window: &winit::Window) -> VooResult<Surface> {
        use winit::os::windows::WindowExt;
        let mut handle = 0;

        let create_info = vks::khr_win32_surface::VkWin32SurfaceCreateInfoKHR {
            sType: vks::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: 0,
            hinstance: ptr::null_mut(),
            hwnd: window.get_hwnd() as *mut vks::win32_types::HWND_T,
        };

        unsafe {
            ::check(instance.vk().khr_win32_surface.vkCreateWin32SurfaceKHR(instance.handle(), &create_info, ptr::null(),
                &mut handle));
        }

        Ok(Surface {
            inner: Arc::new(Inner {
                handle: handle,
                instance: instance,
                active: AtomicBool::new(false),
            })
        })
    }

    pub fn handle(&self) -> vks::VkSurfaceKHR {
        self.inner.handle
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            println!("Destroying surface...");
            self.instance.vk().khr_surface.vkDestroySurfaceKHR(self.instance.handle(), self.handle, ptr::null());
        }
    }
}