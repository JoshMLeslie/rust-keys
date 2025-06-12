use crate::rk_ui::types::{NoteBar, UiEngine};

impl UiEngine {
    // --- INIT ---
    pub fn new() -> Self {
        Self {
            falling_notes: Vec::new(),
            piano_keys: vec![false; 128],
            should_quit: false,
        }
    }
		
    // --- API ---
    pub fn add_note(&mut self, note: u8, velocity: u8) {
        self.falling_notes.push(NoteBar {
            note,
            y_position: 0.0,
            velocity,
        });
    }

    pub fn note_active(&mut self, note: u8) -> &bool {
        return self.piano_keys.get(note as usize).unwrap_or(&false);
    }

    pub fn try_press_key(&mut self, note: u8) {
        if let Some(key) = self.piano_keys.get_mut(note as usize) {
            *key = true;
        }
    }

    pub fn try_release_key(&mut self, note: u8) {
        if let Some(key) = self.piano_keys.get_mut(note as usize) {
            *key = false;
        }
    }

    pub fn update_pos(&mut self, fall_speed: f32) {
        self.falling_notes
            .iter_mut()
            .for_each(|note| note.y_position += fall_speed);
        // Remove notes that have fallen off the bottom
        self.falling_notes.retain(|note| note.y_position < 1.0);
    }
}
