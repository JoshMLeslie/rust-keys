use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;

use super::types::PianoKey;

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
