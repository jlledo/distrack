use std::ffi::CStr;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};

pub fn nsstring(s: &str) -> id /* NSString */ {
    unsafe { NSString::alloc(nil).init_str(s).autorelease() }
}

pub trait ToString {
    fn to_string(self) -> String;
}

impl<S: NSString> ToString for S {
    fn to_string(self) -> String {
        unsafe {
            CStr::from_ptr(self.UTF8String())
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
