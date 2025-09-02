use ncontrol_plus::settings::{load_config, save_config, Config};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, allow_hyphen_values = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Set {
        #[arg(
            value_parser = clap::value_parser!(u8).range(0..=127),
            required = true
        )]
        channels: Vec<u8>,
    },
    Show,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Set { channels }) => {
            let mut sorted = channels.clone();
            sorted.sort_unstable();
            sorted.dedup();

            let config = Config { channels: sorted };
            let _ = save_config(&config);
            println!("MIDI channels set to {:?}", config);
            return;
        }
        Some(Command::Show) => {
            let config = load_config();
            println!("Current MIDI channels: {:?}", config);
            return;
        }
        None => {
            println!("Use `set-channels` to configure MIDI channels. Try --help for options.");
        }
    }
}
