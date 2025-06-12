use std::collections::HashSet;

use midir::MidiInputConnection;
use ratatui::{layout::Rect, style::Color};

use crate::types::midi::{Message, MessageData};

// todo
enum PianoKeyCount {
    A = 88,
    B = 76,
    C = 61,
    D = 49,
}
pub struct UiEngine {
    pub falling_notes: Vec<NoteBar>,
    pub piano_keys: Vec<bool>, // Simple array for which keys are pressed
    pub should_quit: bool,
}

pub struct NoteBar {
    pub note: u8,     // MessageData[1]
    pub velocity: u8, // MessageData[2]
    pub y_position: f32, // Current position (calculated from elapsed time)
                      // pub spawn_time: u64, // Original timestamp from Message
                      // pub is_hit: bool,
                      // pub timing_feedback: Option<usize>, // ?
}

pub struct AppState {
    pub falling_notes: Vec<NoteBar>,
    pub fall_speed: f32,
    pub piano_keys: PianoKeyboard,
    pub connection: Option<MidiInputConnection<()>>,
    pub last_processed_time: u64, // processing message queue
}

pub struct PianoKeyboard {
    pub active_keys: HashSet<u8>, // Currently pressed MIDI notes
    pub key_range: (u8, u8),      // e.g., (60, 72) for middle C octave
}

pub struct KeyDrawInfo {
    pub rect: Rect,
    pub bg_color: Color,
    pub fg_color: Color,
    pub key_name: String,
}

pub struct NoteContext {
    pub octave: i32,
    pub is_active: bool,
}

pub struct KeyContext {
    pub key_index: usize,
    pub key_x: u16,
}

pub struct RenderContext {
    pub available_width: u16,
    pub key_height: u16,
    pub key_width: u16,
    pub inner_area: Rect,
}
