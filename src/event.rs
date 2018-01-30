use std::sync::Arc;
use vks;
use ::{VdResult, CallResult, Handle, Device, EventCreateFlags, EventCreateInfo};

pub enum EventStatus {
    Signaled,
    Unsignaled,
    Error(CallResult),
}

impl From<CallResult> for EventStatus {
    fn from(res: CallResult) -> EventStatus {
        match res {
            CallResult::EventSet => EventStatus::Signaled,
            CallResult::EventReset => EventStatus::Unsignaled,
            _ => EventStatus::Error(res),
        }
    }
}

impl From<i32> for EventStatus {
    fn from(res: i32) -> EventStatus {
        CallResult::from(res).into()
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EventHandle(pub(crate) vks::VkEvent);

impl EventHandle {
    #[inline(always)]
    pub fn to_raw(&self) -> vks::VkEvent {
        self.0
    }
}

unsafe impl Handle for EventHandle {
    type Target = EventHandle;

    fn handle(&self) -> Self::Target {
        *self
    }
}


#[derive(Debug)]
struct Inner {
    handle: EventHandle,
    device: Device,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_event(self.handle, None);
        }
    }
}


/// An event.
///
///
/// ### Destruction
/// 
/// Dropping this `Event` will cause `Device::destroy_event` to be called, 
/// automatically releasing any resources associated with it.
///
#[derive(Debug, Clone)]
pub struct Event {
    inner: Arc<Inner>,
}

impl Event {
    pub fn new(device: Device, flags: EventCreateFlags) -> VdResult<Event> {
        let create_info = EventCreateInfo::builder()
            .flags(flags)
            .build();

        let handle = unsafe { device.create_event(&create_info, None)? };

        Ok(Event {
            inner: Arc::new(Inner {
                handle,
                device,
            })
        })
    }

    /// Returns this object's handle.
    pub fn handle(&self) -> EventHandle {
        self.inner.handle
    }

    /// Returns a reference to this object's associated device.
    pub fn device(&self) -> &Device {
        &self.inner.device
    }

    /// Sets this event to signaled state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkSetEvent.html
    ///
    pub fn set(&self) -> VdResult<()> {
        unsafe { self.inner.device.set_event(self.handle()) }
    }

    /// Resets this event to non-signaled state.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkResetEvent.html
    ///
    pub fn reset(&self) -> VdResult<()> {
        unsafe { self.inner.device.reset_event(self.handle()) }
    }

    /// Retrieves the status of this event object.
    ///
    /// https://www.khronos.org/registry/vulkan/specs/1.0/man/html/vkGetEventStatus.html
    ///
    pub fn status(&self) -> VdResult<EventStatus> {
        unsafe { Ok(self.inner.device.get_event_status(self.handle())?.into()) }
    }
}

unsafe impl<'h> Handle for &'h Event {
    type Target = EventHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

