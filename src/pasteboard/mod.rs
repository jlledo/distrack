pub use crate::pasteboard::manager::Manager;

use cocoa::appkit::{NSPasteboard, NSPasteboardItem, NSPasteboardTypeString};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSArray, NSAutoreleasePool, NSInteger};
use thiserror::Error;

use crate::util::{self, ToString};

mod manager;

pub struct Pasteboard {
    inner: id, // NSPasteboard
}

impl Pasteboard {
    pub fn new(inner: id /* NSPasteboard */) -> Self {
        Pasteboard { inner }
    }

    pub fn get_string(&self) -> Option<String> {
        unsafe {
            let items = self.inner.pasteboardItems();
            if items.is_null() || items.count() == 0 {
                return None;
            }

            let item = items.objectAtIndex(0);
            let string = NSPasteboardItem::stringForType(item, NSPasteboardTypeString);
            if string.is_null() {
                return None;
            }

            Some(string.to_string())
        }
    }

    pub fn set_string(&mut self, s: &str) -> Result<(), PasteboardError> {
        let result = unsafe {
            let objects = NSArray::arrayWithObject(nil, util::nsstring(s)).autorelease();
            self.inner.clearContents();
            self.inner.writeObjects(objects)
        };

        if result == NO {
            return Err(PasteboardError);
        }

        Ok(())
    }

    pub fn change_count(&self) -> NSInteger {
        unsafe { self.inner.changeCount() }
    }
}

impl From<id /* NSPasteboard */> for Pasteboard {
    fn from(pasteboard: id) -> Self {
        Pasteboard { inner: pasteboard }
    }
}

#[derive(Error, Debug)]
#[error("failed to set pasteboard contents")]
pub struct PasteboardError;
