use midir::{MidiInput, MidiInputConnection};
use std::io::{Write, stdin, stdout};
use std::sync::mpsc::Sender;
// ---
use crate::rk_io::watcher::spawn_watcher;
use crate::rk_ui::ui::run_app;
use crate::types;

fn print_ports(midi: &MidiInput) {
    let ports: Vec<midir::MidiInputPort> = midi.ports();
    println!("Available MIDI input ports:");
    for (i, port) in ports.iter().enumerate() {
        let port_result: Result<String, midir::PortInfoError> = midi.port_name(port);
        match port_result {
            Ok(port) => println!("{}: {:?}", i, port),
            Err(err) => panic!("{:?}", err),
        }
    }
}

fn open_conn(
    midi: MidiInput,
    port: &midir::MidiInputPort,
    tx: Sender<types::midi::Message>,
) -> MidiInputConnection<()> {
    println!("Opening connection...");
    return midi
        .connect(
            &port,
            "midir-read-input",
            move |now: u64, message: &[u8], _| {
                if let Ok(msg) = <[u8; 3]>::try_from(message) {
                    tx.send((now, msg)).ok();
                } else if let Ok(msg) = <[u8; 1]>::try_from(message) {
                    tx.send((now, [msg[0], 0, 0])).ok();
                } else {
                    panic!(
                        "Midi message out of bounds! ts: {} data: {:?}",
                        now, message
                    );
                }
            },
            (),
        )
        .unwrap();
}

pub fn select_device(midi: MidiInput) -> Option<MidiInputConnection<()>> {
    let ports = midi.ports();
    let mut input = String::new();

    let (tx, rx) = spawn_watcher();

    print_ports(&midi);
    match ports.len() {
        0 => panic!("No ports available"),
        _ => loop {
            print!("Select input port number: ");
            input.clear();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(index) if index < ports.len() => {
                    let conn = open_conn(midi, &ports[index], tx);
                    if let Err(e) = run_app(rx) {
                        eprintln!("UI error: {}", e);
                        return None;
                    }
                    return Some(conn);
                }
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
    };
}
