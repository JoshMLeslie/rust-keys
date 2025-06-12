use log::debug;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::rk_ui::{
    constants::{KEY_NAMES, PIANO_PATTERN},
    types::{KeyContext, KeyDrawInfo, NoteContext, RenderContext, UiEngine},
    util::{count_white_keys_in_range, get_key_colors},
};

// Store white key positions for black key placement
#[derive(Clone)]
struct WhiteKeyPosition {
    x: u16,
    width: u16,
    note: u8,
}

pub fn render(f: &mut Frame, engine: &mut UiEngine, area: Rect, start_note: u8, end_note: u8) {
    let block = Block::default().title(" Piano ").borders(Borders::ALL);
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let white_key_count = count_white_keys_in_range(start_note, end_note);
    let available_width = inner_area.width;

    // Calculate uniform key width and use remainder as padding
    let white_key_width = if white_key_count > 0 {
        available_width / white_key_count
    } else {
        1
    };
    let width_remainder = if white_key_count > 0 {
        available_width % white_key_count
    } else {
        0
    };

    // Use remainder as left padding to center the keyboard
    let left_padding = width_remainder / 2;

    let white_key_height = inner_area.height;
    let black_key_height = white_key_height * 2 / 3;

    // Collect white key positions during first pass
    let mut white_key_positions = Vec::new();
    let mut white_key_x = left_padding;

    // First pass: Draw white keys and collect positions
    for note in start_note..=end_note {
        let key_index = (note % 12) as usize;

        if PIANO_PATTERN[key_index] {
            let is_active = *engine.note_active(note);
            let octave = (note / 12) as i32 - 1;

            // Store position for black key calculations
            white_key_positions.push(WhiteKeyPosition {
                x: white_key_x,
                width: white_key_width,
                note,
            });

            draw_white_key(
                f,
                &NoteContext { octave, is_active },
                &KeyContext {
                    key_index,
                    key_x: white_key_x,
                },
                &RenderContext {
                    available_width,
                    key_height: white_key_height,
                    key_width: white_key_width,
                    inner_area,
                },
            );

            white_key_x += white_key_width;
        }
    }

    // Second pass: Draw black keys using stored white key positions
    for note in start_note..=end_note {
        let key_index = (note % 12) as usize;

        if !PIANO_PATTERN[key_index] {
            let is_active = *engine.note_active(note);
            let octave = (note / 12) as i32 - 1;

            if let Some((black_key_x, black_key_width)) =
                calculate_black_key_position(note, &white_key_positions)
            {
                draw_black_key(
                    f,
                    &NoteContext { octave, is_active },
                    &KeyContext {
                        key_index,
                        key_x: black_key_x,
                    },
                    &RenderContext {
                        available_width,
                        key_height: black_key_height,
                        key_width: black_key_width,
                        inner_area,
                    },
                );
            }
        }
    }
}

fn calculate_black_key_position(
    black_note: u8,
    white_positions: &[WhiteKeyPosition],
) -> Option<(u16, u16)> {
    // Find the white keys that this black key sits between
    let black_key_index = (black_note % 12) as usize;

    // Map black keys to their adjacent white keys
    let (left_white_note, right_white_note) = match black_key_index {
        1 => (black_note - 1, black_note + 1),  // C# between C and D
        3 => (black_note - 1, black_note + 1),  // D# between D and E
        6 => (black_note - 1, black_note + 1),  // F# between F and G
        8 => (black_note - 1, black_note + 1),  // G# between G and A
        10 => (black_note - 1, black_note + 1), // A# between A and B
        _ => return None,                       // Not a black key
    };

    // Find positions of adjacent white keys
    let left_pos = white_positions
        .iter()
        .find(|pos| pos.note == left_white_note);
    let right_pos = white_positions
        .iter()
        .find(|pos| pos.note == right_white_note);

    // Handle edge cases where adjacent white keys might be out of range
    match (left_pos, right_pos) {
        (Some(left), Some(right)) => {
            // Both adjacent white keys found - normal case
            let left_edge = left.x + left.width * 2 / 3;
            let right_edge = right.x + right.width / 3;
            let black_key_width = (right_edge - left_edge).max(1);
            Some((left_edge, black_key_width))
        }
        (Some(left), None) => {
            // Only left white key found - estimate right edge
            let black_key_width = left.width * 2 / 3;
            let black_key_x = left.x + left.width * 2 / 3;
            Some((black_key_x, black_key_width.max(1)))
        }
        (None, Some(right)) => {
            // Only right white key found - estimate left edge
            let black_key_width = right.width * 2 / 3;
            let black_key_x = right.x.saturating_sub(black_key_width / 2);
            Some((black_key_x, black_key_width.max(1)))
        }
        (None, None) => {
            // Neither adjacent white key found - skip this black key
            None
        }
    }
}

// Custom widget approach - much cleaner than Paragraph
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;

struct PianoKey {
    bg_color: Color,
    fg_color: Color,
    key_name: String,
    show_label: bool,
}

impl Widget for PianoKey {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Fill background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if x < buf.area.right() && y < buf.area.bottom() {
                    buf[(x, y)].set_bg(self.bg_color);
                    buf[(x, y)].set_char(' ');
                }
            }
        }

        // Draw borders
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .render(area, buf);

        // Draw label if requested
        if self.show_label && area.height >= 2 && area.width >= 2 {
            let label_y = area.bottom().saturating_sub(2);
            let label_x = area.left() + 1;

            for (i, ch) in self.key_name.chars().enumerate() {
                let char_x = label_x + (i as u16);
                if char_x < area.right().saturating_sub(1)
                    && char_x < buf.area.right()
                    && label_y < buf.area.bottom()
                {
                    buf[(char_x, label_y)].set_char(ch);
                    buf[(char_x, label_y)].set_fg(self.fg_color);
                    buf[(char_x, label_y)].set_bg(self.bg_color);
                }
            }
        }
    }
}

fn draw_white_key(
    f: &mut Frame,
    note_ctx: &NoteContext,
    key_ctx: &KeyContext,
    render_ctx: &RenderContext,
) {
    let (bg_color, fg_color) = get_key_colors(true, note_ctx.is_active);
    let key_name = format!("{}{}", KEY_NAMES[key_ctx.key_index], note_ctx.octave);

    let key_rect = Rect {
        x: render_ctx.inner_area.x + key_ctx.key_x,
        y: render_ctx.inner_area.y,
        width: render_ctx.key_width,
        height: render_ctx.key_height,
    };

    let show_label = render_ctx.key_width >= 3
        && render_ctx.key_height >= 2
        && key_name.len() <= (key_rect.width.saturating_sub(2)) as usize;

    let piano_key = PianoKey {
        bg_color,
        fg_color,
        key_name,
        show_label,
    };

    f.render_widget(piano_key, key_rect);
}

fn draw_black_key(
    f: &mut Frame,
    note_ctx: &NoteContext,
    key_ctx: &KeyContext,
    render_ctx: &RenderContext,
) {
    let (bg_color, fg_color) = get_key_colors(false, note_ctx.is_active);
    let key_name = format!("{}{}", KEY_NAMES[key_ctx.key_index], note_ctx.octave);

    let key_rect = Rect {
        x: render_ctx.inner_area.x + key_ctx.key_x,
        y: render_ctx.inner_area.y,
        width: render_ctx.key_width,
        height: render_ctx.key_height,
    };

    let show_label = render_ctx.key_width >= 2
        && render_ctx.key_height >= 2
        && key_name.len() <= (key_rect.width.saturating_sub(2)) as usize;

    let piano_key = PianoKey {
        bg_color,
        fg_color,
        key_name,
        show_label,
    };

    f.render_widget(piano_key, key_rect);
}
