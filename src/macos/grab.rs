#![allow(improper_ctypes_definitions)]
use crate::macos::common::*;
use crate::rdev::{Event, GrabError};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use core_graphics::event::{CGEventTapLocation, CGEventType};
use std::os::raw::c_void;

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event) -> Option<Event>>> = None;

#[link(name = "Cocoa", kind = "framework")]
unsafe extern "C" {}

unsafe extern "C" fn raw_callback(
    _proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEventRef,
    _user_info: *mut c_void,
) -> CGEventRef {
    let opt = KEYBOARD_STATE.lock();
    if let Ok(mut keyboard) = opt {
        unsafe {
            if let Some(event) = convert(_type, &cg_event, &mut keyboard) {
                // Reborrowing the global callback pointer.
                // Totally UB. but not sure there's a great alternative.
                let ptr = &raw mut GLOBAL_CALLBACK;
                if let Some(callback) = &mut *ptr {
                    if callback(event).is_none() {
                        cg_event.set_type(CGEventType::Null);
                    }
                }
            }
        }
    }
    cg_event
}

pub fn grab<T>(callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        let _pool = NSAutoreleasePool::new(nil);
        let tap = CGEventTapCreate(
            CGEventTapLocation::HID, // HID, Session, AnnotatedSession,
            kCGHeadInsertEventTap,
            CGEventTapOption::Default,
            kCGEventMaskForAllEvents,
            raw_callback,
            nil,
        );
        if tap.is_null() {
            return Err(GrabError::EventTapError);
        }
        let _loop = CFMachPortCreateRunLoopSource(nil, tap, 0);
        if _loop.is_null() {
            return Err(GrabError::LoopSourceError);
        }

        let current_loop = CFRunLoopGetCurrent();
        CFRunLoopAddSource(current_loop, _loop, kCFRunLoopCommonModes);

        CGEventTapEnable(tap, true);
        CFRunLoopRun();
    }
    Ok(())
}
