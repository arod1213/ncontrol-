#[allow(unused_imports)]

use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;


pub fn establish_connection() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let target_port_name = "IAC Driver NControl";

    let midi_out = MidiOutput::new("Main MIDI Out")?;
    let ports = midi_out.ports();

    for port in &ports {
        let name = midi_out.port_name(port)?;
        println!("port name is {}", name);
        if name == target_port_name {
            let conn = midi_out.connect(port, "Target Connection")?;
            return Ok(conn)
        }
    }
    Err("Could not find the specified MIDI port".into())
}

pub fn send_cc_msg(output: &mut MidiOutputConnection, cc: u8, value: u8, channel: u8) -> Result<(), Box<dyn Error>> {
    let msg = vec![0xB0 | (channel & 0x0F), cc, value];
    let res = output.send(&msg);
    match res {
        Err(e) => eprintln!("Error sending CC message: {}", e),
        Ok(_) => println!("Sent channel: {} cc: {} value: {}", channel, cc, value)
    }
    Ok(())
}

