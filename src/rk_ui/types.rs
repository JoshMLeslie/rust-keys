use std::collections::HashSet;

use midir::MidiInputConnection;

use crate::types::midi::{Message, MessageData};

pub struct NoteBar {
    note: u8,        // MessageData[1]
    velocity: u8,    // MessageData[2]
    spawn_time: u64, // Original timestamp from Message
    y_position: f32, // Current position (calculated from elapsed time)
    is_hit: bool,
    timing_feedback: Option<usize>, // ?
}

pub struct AppState {
    falling_notes: Vec<NoteBar>,
    fall_speed: f32,
    piano_keys: PianoKeyboard,
    connection: Option<MidiInputConnection<()>>,
    last_processed_time: u64, // processing message queue
}

pub struct PianoKeyboard {
    active_keys: HashSet<u8>, // Currently pressed MIDI notes
    key_range: (u8, u8),      // e.g., (60, 72) for middle C octave
}
