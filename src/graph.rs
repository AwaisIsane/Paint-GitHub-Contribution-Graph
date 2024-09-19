use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

pub struct GraphGrid;

impl GraphGrid {
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
}

impl Widget for GraphGrid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let weeks = 53;
        let days = 7;
        let (rw, rh) = self.render_dims();
        if rw > area.width || rh > area.height {
            buf.set_string(
                area.left(),
                area.top(),
                "increase terminal width ",
                Style::default().fg(Color::Red),
            );
            return;
        }

        let hor_pad = (area.width - rw) / 2; //to centre
        let ver_pad = (area.height - rh) / 2;

        for y in 0..days {
            for x in 0..weeks {
                let render_x = area.left() + hor_pad + x as u16 * 2 + 1;
                let render_y = area.top() + ver_pad + y as u16;
                let style = if x == 0 && y == 0 {
                    Style::default()
                        .fg(self.graph_color_green())
                        .add_modifier(Modifier::SLOW_BLINK)
                } else {
                    Style::default().fg(self.graph_color_red())
                };
                buf.set_string(render_x, render_y, "■", style);
            }
        }
    }
}

//"■"
