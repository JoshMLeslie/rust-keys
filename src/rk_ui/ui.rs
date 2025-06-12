use crate::{
    rk_ui::{
        constants::PIANO_PATTERN,
        render_piano::{self},
        types::{NoteBar, UiEngine},
        util::count_white_keys_in_range,
    },
    types::midi::Message,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    prelude::Color,
    style::Style,
    widgets::{Block, Borders},
};
use std::{process::exit, sync::mpsc::Receiver};

pub fn run_app(midi_receiver: Receiver<Vec<Message>>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut engine = UiEngine::new();

    loop {
        // Process MIDI messages (non-blocking)
        while let Ok(message) = midi_receiver.try_recv() {
            process_midi_message(&mut engine, message);
        }

        // Update falling notes positions
        update_falling_notes(&mut engine);

        // Render UI
        terminal.draw(|f| ui(f, &mut engine))?;

        // Handle keyboard input (for quitting, etc.)
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        engine.should_quit = true;
                    }
                    _ => (),
                }
            }
        }

        if engine.should_quit {
            break;
        }
        // Small sleep to prevent excessive CPU usage
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn process_midi_message(engine: &mut UiEngine, messages: Vec<Message>) {
    for (_timestamp, [status, note, velocity]) in messages {
        match status {
            0x90..=0x9f if velocity > 0 => {
                // 144..159 midi NOTE_ON for channel_x
                engine.add_note(note, velocity);
                engine.try_press_key(note);
            }
            0x80..=0x8f | 0x90..=0x9f if velocity == 0 => {
                // 128..143 NOTE_OFF | 144..159 midi NOTE_ON but vel = 0 for channel_x
                engine.try_release_key(note);
            }
            _ => {
                eprintln!("Unhandled midi status");
                exit(1);
            }
        }
    }
}

fn update_falling_notes(engine: &mut UiEngine) {
    let fall_speed = 0.02; // Adjust this to control speed (higher = faster)
    engine.update_pos(fall_speed);
}

fn ui(f: &mut Frame, engine: &mut UiEngine) {
    let chunks = Layout::vertical([
        Constraint::Percentage(75), // Falling notes area
        Constraint::Percentage(25), // Piano keyboard area
    ])
    .split(f.area());

    render_falling_notes(f, engine, chunks[0]);
    render_piano::render(f, engine, chunks[1], 21, 108);
}

fn render_falling_notes(f: &mut Frame, engine: &UiEngine, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title(" Falling Notes ")
        .borders(Borders::ALL);
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Convert notes to visual positions
    for &NoteBar {
        note,
        y_position,
        velocity,
    } in &engine.falling_notes
    {
        let x_pos = map_note_to_x_position(note, inner_area.width);

        // Convert y_position (0.0-1.0) to actual screen coordinates
        let y_pos = (y_position * inner_area.height as f32) as u16;

        if y_pos < inner_area.height {
            let color = match velocity {
                0..=42 => Color::Blue,
                43..=84 => Color::Green,
                85..=127 => Color::White,
                _ => Color::Black,
            };

            let note_widget = Block::default().style(Style::default().bg(color));

            let note_area = ratatui::layout::Rect {
                x: inner_area.x + x_pos,
                y: inner_area.y + y_pos,
                width: 2,
                height: 1,
            };

            f.render_widget(note_widget, note_area);
        }
    }
}

// Map MIDI notes to screen width
fn map_note_to_x_position(midi_note: u8, screen_width: u16) -> u16 {
    let min_note = 21; // e.g 60 is Middle C
    let max_note = 108; // 84 is C, two octaves up

    if midi_note < min_note {
        return 0;
    }
       let white_key_count = count_white_keys_in_range(min_note, max_note);
    let available_width = screen_width;

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

    let left_padding = width_remainder / 2;

    // Check if this note is a white key
    let key_index = (midi_note % 12) as usize;
    
    if PIANO_PATTERN[key_index] {
        // White key - calculate position
        // Count white keys BEFORE this note (not including this note)
        let white_keys_before = if midi_note > min_note {
            count_white_keys_in_range(min_note, midi_note - 1)
        } else {
            0
        };
        let white_key_x = left_padding + white_keys_before * white_key_width;
        white_key_x + white_key_width / 2 // Return center of white key
    } else {
        // Black key - calculate position between adjacent white keys
        let (left_white, _right_white) = match key_index {
            1 => (midi_note - 1, midi_note + 1),   // C# between C and D
            3 => (midi_note - 1, midi_note + 1),   // D# between D and E  
            6 => (midi_note - 1, midi_note + 1),   // F# between F and G
            8 => (midi_note - 1, midi_note + 1),   // G# between G and A
            10 => (midi_note - 1, midi_note + 1),  // A# between A and B
            _ => return 0,
        };

        // Calculate positions of adjacent white keys
        let left_white_keys = if left_white >= min_note {
            count_white_keys_in_range(min_note, left_white - 1)
        } else {
            0
        };
        let left_white_x = left_padding + left_white_keys * white_key_width;
        
        // Position black key at 2/3 into the left white key (matching your piano renderer)
        left_white_x + white_key_width * 2 / 3
    }
}
