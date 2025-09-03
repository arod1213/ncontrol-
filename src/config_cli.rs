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
            value_parser = clap::value_parser!(u8).range(1..=128),
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
            sorted = sorted.iter().map(|x| x.saturating_sub(1)).collect();

            let config = Config { channels: sorted };
            match save_config(&config) {
                Ok(_) => {
                    println!("MIDI channels set to {:?}", config);
                }
                Err(e) => {
                    eprintln!("error: {:?}", e);
                }
            };
        }
        Some(Command::Show) => {
            let config = load_config();
            println!("Current MIDI channels: {:?}", config);
        }
        None => {
            println!("Use `set-channels` to configure MIDI channels. Try --help for options.");
        }
    }
}
