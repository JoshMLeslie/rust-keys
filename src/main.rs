use dotenv::dotenv;
use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::{Write, stdin, stdout};
// ---
mod io;
mod test;
mod types;

fn select_input(midi: MidiInput) -> Option<usize> {
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

    return match selection {
        1 => io::connect::select_device(midi),
        2 => io::tests::select_test(midi),
        3 => io::opts::select_opt(),
        _ => None,
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
