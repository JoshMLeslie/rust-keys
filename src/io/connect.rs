use midir::MidiInput;
use std::env;
use std::io::{Write, stdin, stdout};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
// ---
use crate::types;

fn spawn_watcher() -> std::sync::mpsc::Sender<types::midi::Message> {
    let threshold_micro_sec = env::var("THRESHOLD_MICRO_SEC")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let (tx, rx) = channel::<types::midi::Message>();
    let batch = Arc::new(Mutex::new(Vec::new()));
    let batch_clone = Arc::clone(&batch);

    thread::spawn(move || {
        let mut last = Instant::now();

        loop {
            if let Ok(msg) = rx.recv_timeout(Duration::from_micros(threshold_micro_sec)) {
                let mut b = batch_clone.lock().unwrap();
                b.push(msg);
                last = Instant::now();
            }

            if last.elapsed() > Duration::from_micros(threshold_micro_sec) {
                let mut b = batch_clone.lock().unwrap();
                if !b.is_empty() {
                    println!("Note(s):");
                    for (t, msg) in b.iter() {
                        println!("  {:.3}: {:?}", t, msg);
                    }
                    println!("--");
                    b.clear();
                }

                last = Instant::now(); // reset to avoid repeated flush
            }
        }
    });

    return tx;
}
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

fn open_conn(midi: MidiInput, port: &midir::MidiInputPort, tx: Sender<types::midi::Message>) {
    println!("Opening connection...");
    let _conn_in: midir::MidiInputConnection<()> = midi
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

pub fn select_device(midi: MidiInput) -> Option<usize> {
    let ports = midi.ports();
    let mut input = String::new();

    let tx = spawn_watcher().clone();

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
                    open_conn(midi, &ports[index], tx);
                    return Some(1);
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
    }
}
