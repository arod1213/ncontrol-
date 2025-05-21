mod controller;
mod send_cc;

extern crate cocoa;
extern crate core_graphics;
extern crate objc;

use controller::launch;
use cocoa::appkit::{
    NSApp, NSApplication, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem, NSVariableStatusItemLength, NSButton,
};
use cocoa::base::{nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSString};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        // 👇 Spawn the background controller logic
        std::thread::spawn(|| {
            launch(); // your MIDI and keyboard logic lives here
        });

        // 🎛 Setup the menu bar icon
        let status_bar = NSStatusBar::systemStatusBar(nil);
        let status_item = status_bar.statusItemWithLength_(NSVariableStatusItemLength);

        let title = NSString::alloc(nil).init_str("🎛");
        status_item.setTitle_(title);

        let menu = NSMenu::new(nil).autorelease();
        let quit_title = NSString::alloc(nil).init_str("Quit");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                quit_title,
                selector("terminate:"),
                NSString::alloc(nil).init_str("q"),
            )
            .autorelease();
        menu.addItem_(quit_item);

        status_item.setMenu_(menu);

        // 🚀 Start the Cocoa app loop (blocks forever)
        app.run();
    }
}
