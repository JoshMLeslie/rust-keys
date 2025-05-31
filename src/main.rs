use dotenv::dotenv;
use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::{Write, stdin, stdout};
// ---
mod io;
mod types;

fn select_input(midi: MidiInput) -> () {
    let mut input = String::new();
    let mut selection: u8 = 0;

    print!("Select path [(c)onnect | (t)est | (o)pts]: ");

    while selection == 0 {
        input.clear();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        selection = match input.trim() {
            "c" | "connect" => 1,
            "t" | "test" => 2,
            "o" | "opts" => 3,
            _ => continue,
        }
    }

    match selection {
        1 => io::connect::select_port(midi),
        2 => io::tests::select_test(),
        3 => io::opts::select_opt(),
        _ => None,
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

    select_input(midi);

    let mut input = String::new();
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit.").unwrap();
    stdout.flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();

    println!("Shutting down...");
    // let (_, log_all_bytes) = conn_in.close();
    // println!("Received final bytes: {:?}", log_all_bytes);
    // println!("Terminated.");
    Ok(())
}
