use dotenv::dotenv;
use midir::{Ignore, MidiInput, MidiInputConnection};
use rk_io::user_input::{get_input, pause_for_enter};
use std::error::Error;
// ---
mod rk_io;
mod rk_ui;
mod test;
mod types;

#[derive(Clone)]
enum InputPath {
    Connect,
    Test,
    Options,
    Quit,
}

fn select_input(midi: MidiInput) -> Option<MidiInputConnection<()>> {
    let result = get_input(
        "Select path [(c)onnect | (t)est | (o)ptions | (q)uit]: ",
        &[
            ("c", InputPath::Connect),
            ("connect", InputPath::Connect),
            ("t", InputPath::Test),
            ("test", InputPath::Test),
            ("o", InputPath::Options),
            ("options", InputPath::Options),
            ("q", InputPath::Quit),
            ("quit", InputPath::Quit),
        ],
    );

    return match result.unwrap() {
        InputPath::Connect => rk_io::connect::select_device(midi),
        InputPath::Test => rk_io::playback::select_playback(midi),
        InputPath::Options => rk_io::opts::select_opt(),
        InputPath::Quit => std::process::exit(0)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    // env init
    dotenv().ok();
    // end env init

    let mut midi: MidiInput = MidiInput::new("midir input")?;
    midi.ignore(Ignore::All); // sys-log messages, other data persists

    let ports: Vec<midir::MidiInputPort> = midi.ports();
    if ports.is_empty() {
        println!("No MIDI input ports found.");
        return Ok(());
    }

    match select_input(midi) {
        Some(_conn) => {
            println!("Connection established");
            pause_for_enter();
            println!("Shutting down...");
        }
        None => {
            println!("No connection established");
        }
    }

    // let (_, log_all_bytes) = conn_in.close();
    // println!("Received final bytes: {:?}", log_all_bytes);
    // println!("Terminated.");
    Ok(())
}
