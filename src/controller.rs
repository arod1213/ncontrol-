use crate::{
    send_cc::{establish_connection, send_cc_msg},
    settings::Config,
};
use device_query::{DeviceQuery, DeviceState, Keycode};
use midir::MidiOutputConnection;
use std::thread;
use std::time::Duration;

struct KeyCommands {
    mute: Keycode,
    vol_up: Keycode,
    vol_down: Keycode,
}

struct KeyPressInput<'a> {
    val: &'a mut u8,
    ccs: &'a Vec<u8>,
    keys: &'a Vec<Keycode>,
    conn: &'a mut MidiOutputConnection,
    key_commands: &'a KeyCommands,
}

fn handle_key_press(input: KeyPressInput) {
    let KeyPressInput {
        mut val,
        ccs,
        keys,
        mut conn,
        key_commands,
    } = input;

    let channel = 0;

    if *val > 127 {
        return;
    }

    match keys {
        v if v.contains(&key_commands.vol_up) => {
            println!("setting val to {:?}", val.saturating_add(2));
            *val = (*val + 2).min(127);
            for cc in ccs.iter() {
                let _ = send_cc_msg(&mut conn, *cc, *val, channel);
            }
        }
        v if v.contains(&key_commands.vol_down) => {
            *val = val.saturating_sub(2);
            for cc in ccs.iter() {
                let _ = send_cc_msg(&mut conn, *cc, *val, channel);
            }
        }
        v if v.contains(&key_commands.mute) => {
            for cc in ccs.iter() {
                let _ = send_cc_msg(&mut conn, *cc, 127, 1);
            }
        }
        _ => (),
    }
}

fn keys_are_commands(keys: &Vec<Keycode>, commands: &KeyCommands) -> bool {
    keys.contains(&commands.mute)
        || keys.contains(&commands.vol_down)
        || keys.contains(&commands.vol_up)
}

pub fn launch(config: Config) {
    let key_commands = KeyCommands {
        mute: Keycode::F10,
        vol_down: Keycode::F11,
        vol_up: Keycode::F12,
    };

    let mut conn = establish_connection().unwrap();
    let device_state = DeviceState::new();

    let mut val: u8 = 73;
    let ccs = config.channels;

    loop {
        let keys = device_state.get_keys();
        let input = KeyPressInput {
            val: &mut val,
            ccs: &ccs,
            keys: &keys,
            conn: &mut conn,
            key_commands: &key_commands,
        };
        handle_key_press(input);

        {
            if !keys.contains(&key_commands.mute) && keys_are_commands(&keys, &key_commands) {
                println!("SHOULD SLEEP");
                thread::sleep(Duration::from_millis(75));
                continue;
            }
        }

        while device_state.get_keys().contains(&key_commands.mute)
            && keys.contains(&key_commands.mute)
        {
            ();
        }
    }
}
