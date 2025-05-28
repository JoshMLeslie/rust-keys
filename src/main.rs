use midir::{Ignore, MidiInput};
use std::cell::RefCell;
use std::error::Error;
use std::io::{Write, stdin, stdout};

fn print_ports(midi: &MidiInput) -> () {
    let ports = midi.ports();
    println!("Available MIDI input ports:");
    for (i, port) in ports.iter().enumerate() {
        let port_result: Result<String, midir::PortInfoError> = midi.port_name(port);
        match port_result {
            Ok(port) => println!("{}: {:?}", i, port),
            Err(err) => panic!("{:?}", err),
        }
    }
}

fn select_input(midi: &MidiInput) -> std::io::Result<usize> {
    let ports: Vec<midir::MidiInputPort> = midi.ports();
    let mut input: String = String::new();

    print_ports(&midi);

    match ports.len() {
        0 => panic!("No ports available"),
        1 => {
            println!(
                "Choosing only available output port: {}",
                midi.port_name(&ports[0]).unwrap()
            );
            return Ok(0);
        }
        _ => loop {
            print!("Select input port number: ");
            input.clear();
            stdout().flush()?;
            stdin().read_line(&mut input)?;

            match input.trim().parse::<usize>() {
                Ok(index) if index < ports.len() => return Ok(index),
                Ok(index) => println!(
                    "Invalid selection: {}. Must be less than {}.",
                    index,
                    ports.len()
                ),
                Err(e) => println!(
                    "Invalid selection: {:?}. Must be a number less than {}.",
                    e.kind(),
                    ports.len()
                ),
            }
            println!("Try again.\n");
            print_ports(&midi);
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut midi: MidiInput = MidiInput::new("midir input")?;
    midi.ignore(Ignore::All); // sys-log messages, other data persists

    let ports: Vec<midir::MidiInputPort> = midi.ports();
    if ports.is_empty() {
        println!("No MIDI input ports found.");
        return Ok(());
    }

    let conn_port_i = select_input(&midi)?;
    let batch: RefCell<Vec<_>> = RefCell::new(Vec::new());
    let last_stamp: RefCell<u64> = RefCell::new(0u64);

    println!("Opening connection...");
    let conn_in: midir::MidiInputConnection<()> = midi.connect(
        &ports[conn_port_i],
        "midir-read-input",
        move |stamp: u64, message: &[u8], _| {
            // group notes within threshold_ms effectively as concurrent
            let threshold_ms: u64 = 10000;

            let mut last: std::cell::RefMut<'_, u64> = last_stamp.borrow_mut();
            let mut b: std::cell::RefMut<'_, Vec<(u64, Vec<u8>)>> = batch.borrow_mut();

            println!("Note played:");
            println!("{}: {:?} ({} byte_s)", stamp, message, message.len());

            if (*last == 0) || (stamp - *last) > threshold_ms {
                if !b.is_empty() {
                    println!("Note(s):");
                    for (t, msg) in b.iter() {
                        println!("  {:.3}: {:?}", t, msg);
                    }
                    println!("--");
                    b.clear();
                }
            }

            b.push((stamp, message.to_vec()));
            *last = stamp;
        },
        (),
    )?;

    let mut input: String = String::new();
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit.").unwrap();
    stdout.flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();

    println!("Shutting down...");
    let (_, log_all_bytes) = conn_in.close();
    println!("Received final bytes: {:?}", log_all_bytes);
    println!("Terminated.");
    Ok(())
}
