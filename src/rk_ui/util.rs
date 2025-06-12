use ratatui::style::Color;

use crate::rk_ui::constants::PIANO_PATTERN;

pub fn count_white_keys_in_range(start_note: u8, end_note: u8) -> u16 {
    let mut count: u16 = 0;
    for note in start_note..=end_note {
        let key_index = (note % 12) as usize;
        if PIANO_PATTERN[key_index] {
            count += 1;
        }
    }
    return count;
}

pub fn get_key_colors(is_white: bool, is_active: bool) -> (Color, Color) {
    match (is_white, is_active) {
        (true, false) => (Color::White, Color::Black), // Normal white key
        (true, true) => (Color::Gray, Color::Black),   // Active white key
        (false, false) => (Color::Black, Color::White), // Normal black key
        (false, true) => (Color::Gray, Color::White),  // Active black key
    }
}