use dotenv::dotenv;
use io::user_input::get_input;
use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::{Write, stdin, stdout};
// ---
mod io;
mod test;
mod types;

#[derive(Clone)]
enum InputPath {
    Connect,
    Test,
    Options,
}

fn select_input(midi: MidiInput) -> Option<usize> {
    let result = get_input(
        "Select path [(c)onnect | (t)est | (o)ptions]: ",
        &[
            ("c", InputPath::Connect),
            ("connect", InputPath::Connect),
            ("t", InputPath::Test),
            ("test", InputPath::Test),
            ("o", InputPath::Options),
            ("options", InputPath::Options),
        ],
    );

    return match result.unwrap() {
        InputPath::Connect => io::connect::select_device(midi),
        InputPath::Test => io::tests::select_test(midi),
        InputPath::Options => io::opts::select_opt(),
    };
}

fn pause_for_enter() {
    let mut input = String::new();
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit.").unwrap();
    stdout.flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();
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

    let is_listening = select_input(midi);

    if is_listening.is_none() {
        return Ok(());
    }
    pause_for_enter();
    println!("Shutting down...");
    // let (_, log_all_bytes) = conn_in.close();
    // println!("Received final bytes: {:?}", log_all_bytes);
    // println!("Terminated.");
    Ok(())
}
