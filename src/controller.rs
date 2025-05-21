#[allow(unused_imports)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::send_cc::{establish_connection, send_cc_msg};
use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn launch() {
    let mut conn = establish_connection().unwrap();

    let volume = Arc::new(Mutex::new(73));
    let mute_pressed = Arc::new(Mutex::new(false));

    let ccs = vec![16, 17];
    let channel = 0;

    let device_state = DeviceState::new();

    loop {
        let keys = device_state.get_keys();
        let command_pressed = keys.contains(&Keycode::Meta);

        if command_pressed && keys.contains(&Keycode::F12) {
            let mut val = volume.lock().unwrap();
            if *val < 127 {
                *val += 2;
                for cc in &ccs {
                    let _ = send_cc_msg(&mut conn, *cc, *val, channel);
                }
            }
        }

        if command_pressed && keys.contains(&Keycode::F11) {
            let mut val = volume.lock().unwrap();
            if *val < 127 {
                *val -= 2;
                for cc in &ccs {
                    let _ = send_cc_msg(&mut conn, *cc, *val, channel);
                }
            }
        }
        if command_pressed && keys.contains(&Keycode::F10) {
            let mut mute_pressed = mute_pressed.lock().unwrap();

            if *mute_pressed {
                continue
            }

            for cc in &ccs {
                let _ = send_cc_msg(&mut conn, *cc, 127, 1);
            }
            *mute_pressed = true;
        }
        if !command_pressed || !keys.contains(&Keycode::F10) {
            let mut mute_pressed = mute_pressed.lock().unwrap();
            *mute_pressed = false;
        }
        thread::sleep(Duration::from_millis(75));
    }
}
