use cocoa::appkit::NSPasteboard;
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;

use distrack::PasteboardManager;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let pasteboard = NSPasteboard::generalPasteboard(nil);
        let mut manager = PasteboardManager::new(pasteboard.into());
        manager.watch();
    }
}
