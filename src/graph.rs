use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use crate::app::App;
use chrono::Datelike;
pub struct GraphGrid<'a> {
    app: &'a App,
}

impl<'a> GraphGrid<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
    fn render_dims(&self) -> (u16, u16) {
        let render_width = 2 * 53 + 1;
        let render_height = 7;

        (render_width, render_height)
    }
    fn graph_color_green(&self) -> Color {
        // RGB values for a medium green color
        // You can adjust these values to get the exact shade of green you want
        Color::Rgb(0, 128, 0)
    }
    fn graph_color_red(&self) -> Color {
        Color::Rgb(128, 0, 0)
    }

    fn graph_color_yellow(&self) -> Color {
        Color::Rgb(255, 255, 0)
    }
}

impl<'a> Widget for GraphGrid<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (rw, rh) = self.render_dims();
        if rw > area.width {
            buf.set_string(
                area.left(),
                area.top(),
                "increase terminal width ",
                Style::default().fg(Color::Red),
            );
            return;
        }

        if rh > area.height {
            buf.set_string(
                area.left(),
                area.top(),
                "increase terminal height ",
                Style::default().fg(Color::Red),
            );
            return;
        }

        let hor_pad = (area.width - rw) / 2; //to centre
        let ver_pad = (area.height - rh) / 2;
        let skip = self.app.start_date.weekday().num_days_from_sunday() as usize;

        for days in 0..self.app.no_of_days + skip {
            let x = days / 7;
            let y = days % 7;

            let render_x = area.left() + hor_pad + x as u16 * 2 + 1;
            let render_y = area.top() + ver_pad + y as u16;
            if days >= skip {
                let actual_day = days - skip;
                let style = if actual_day == self.app.pos {
                    Style::default()
                        .fg(self.graph_color_yellow())
                        .add_modifier(Modifier::SLOW_BLINK)
                } else if self.app.commits[actual_day] > 0 {
                    Style::default().fg(self.graph_color_green())
                } else {
                    Style::default().fg(self.graph_color_red())
                };
                buf.set_string(render_x, render_y, "■", style);
            }
        }
    }
}
