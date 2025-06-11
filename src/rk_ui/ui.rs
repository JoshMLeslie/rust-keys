use crate::types::midi::Message;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::Color,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use std::sync::mpsc::Receiver;

struct App {
    falling_notes: Vec<NoteBar>,
    piano_keys: Vec<bool>, // Simple array for which keys are pressed
    should_quit: bool,
}

struct NoteBar {
    note: u8,
    y_position: f32,
    velocity: u8,
}

pub fn run_app(midi_receiver: Receiver<Vec<Message>>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        falling_notes: Vec::new(),
        piano_keys: vec![false; 88], // 88 piano keys
        should_quit: false,
    };

    loop {
        // Process MIDI messages (non-blocking)
        while let Ok(message) = midi_receiver.try_recv() {
            process_midi_message(&mut app, message);
        }

        // Update falling notes positions
        update_falling_notes(&mut app);

        // Render UI
        terminal.draw(|f| ui(f, &app))?;

        // Handle keyboard input (for quitting, etc.)
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    _ => (),
                }
            }
        }

        if app.should_quit {
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

fn process_midi_message(app: &mut App, messages: Vec<Message>) {
    for (_timestamp, [_status, note, velocity]) in messages {
        if velocity > 0 {
            // spawn falling note
            app.falling_notes.push(NoteBar {
                note,
                y_position: 0.0,
                velocity,
            });
        }
    }
}

fn update_falling_notes(app: &mut App) {
    let fall_speed = 0.02; // Adjust this to control speed (higher = faster)

    for note in &mut app.falling_notes {
        note.y_position += fall_speed;
    }

    // Remove notes that have fallen off the bottom
    app.falling_notes.retain(|note| note.y_position < 1.0);
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Percentage(80), // Falling notes area
        Constraint::Percentage(20), // Piano keyboard area
    ])
    .split(f.area());

    render_falling_notes(f, app, chunks[0]);
    render_piano(f, app, chunks[1]);
}

fn render_falling_notes(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Falling Notes")
        .borders(Borders::ALL);
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Convert notes to visual positions
    for &NoteBar {
        note,
        y_position,
        velocity,
    } in &app.falling_notes
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
    if midi_note > max_note {
        return screen_width.saturating_sub(1);
    }

    let note_range = max_note - min_note;
    let position_ratio = (midi_note - min_note) as f32 / note_range as f32;
    return (position_ratio * (screen_width - 1) as f32) as u16;
}

fn render_piano(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let block = Block::default().title("Piano").borders(Borders::ALL);
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Simple piano representation for now - we can make this fancier later
    let piano_text = "C  C# D  D# E  F  F# G  G# A  A# B ";
    let piano_widget = Paragraph::new(piano_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(piano_widget, inner_area);
}
