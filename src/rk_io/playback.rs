use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, Data, Stream};
use midir::{MidiInput, MidiInputConnection};
use std::io::{Write, stdin, stdout};
use std::usize;
// ---
use crate::rk_io::watcher::spawn_watcher;
use crate::test::basic_tune;
use crate::types::midi::MessageLog;

use super::audio_out::spawn_audio_loop;

fn run_test<const L: usize>(test_data: &MessageLog<L>) {
    let data = test_data.data;

    for (t, msg) in data.iter() {
        println!("{:>6}: {:?}", t, msg);
    }
}

const TEST_NAMES: [&str; 1] = ["Basic Tune"];

fn print_playback_opts() {
    println!("Select index of available options:");
    for (index, name) in TEST_NAMES.iter().enumerate() {
        println!("{} - {}", index, name);
    }
}

pub fn select_playback(midi: MidiInput) -> Option<MidiInputConnection<()>> {
    // TODO / HALF DONE
    let ports = midi.ports();
    let mut input = String::new();

    let tx = spawn_watcher();

    let stream = spawn_audio_loop();

    print_playback_opts();
    input.clear();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<usize>() {
        Ok(index) if index <= 0 => match index {
            0 => run_test(&basic_tune::LOG),
            _ => (),
        },
        Ok(index) => println!("Invalid selection: {}. Must be less than 1.", index,),
        Err(e) => println!(
            "Invalid selection: {:?}. Must be a number less than 1.",
            e.kind(),
        ),
    }

    return None;
}
