
use std::sync::Arc;
use vks;
use ::{VooResult, CallResult, Handle, Device, EventCreateFlags, EventCreateInfo};

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


#[derive(Debug, Clone)]
pub struct Event {
    inner: Arc<Inner>,
}

impl Event {
    pub fn new(device: Device, flags: EventCreateFlags) -> VooResult<Event> {
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

    pub fn handle(&self) -> EventHandle {
        self.inner.handle
    }

    pub fn set(&self) -> VooResult<()> {
        unsafe { self.inner.device.set_event(self.handle()) }
    }

    pub fn reset(&self) -> VooResult<()> {
        unsafe { self.inner.device.reset_event(self.handle()) }
    }

    pub fn status(&self) -> VooResult<EventStatus> {
        unsafe { Ok(self.inner.device.get_event_status(self.handle())?.into()) }
    }
}

unsafe impl<'h> Handle for &'h Event {
    type Target = EventHandle;

    fn handle(&self) -> Self::Target {
        self.inner.handle
    }
}

