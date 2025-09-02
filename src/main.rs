mod controller;
mod send_cc;
mod settings;

use controller::launch;
use settings::{load_config, Config};

extern crate cocoa;
extern crate core_graphics;
extern crate objc;

use cocoa::appkit::{
    NSApp, NSApplication, NSButton, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem,
    NSVariableStatusItemLength,
};
use cocoa::base::{nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSString};

fn app_loop(config: Config) {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        std::thread::spawn(|| {
            launch(config);
        });

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

        app.run();
    }
}

fn main() {
    let config = load_config();
    app_loop(config);
}
