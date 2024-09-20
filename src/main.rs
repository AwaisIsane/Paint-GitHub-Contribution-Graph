use app::CurrentScreen;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};
use std::env;
use std::{error::Error, io};

mod app;
mod graph;
mod ui;
use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let year = args.get(1).map(String::as_str).unwrap_or("default_value");
    // setup terminal

    let mut terminal = ratatui::init();
    terminal.clear()?;

    // create app and run it
    let mut app = App::new(year);
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    ratatui::restore();

    if let Ok(result) = res {
        if result == true {
            print!("added commit ")
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('w') | KeyCode::Up => {
                        app.move_up();
                        app.add_commits();
                    }
                    KeyCode::Char('a') | KeyCode::Left => {
                        app.move_left();
                        app.add_commits();
                    }
                    KeyCode::Char('s') | KeyCode::Down => {
                        app.move_down();
                        app.add_commits();
                    }
                    KeyCode::Char('d') | KeyCode::Right => {
                        app.move_right();
                        app.add_commits();
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('e') => {
                        app.toggle_editing();
                        app.add_commits();
                    }
                    KeyCode::Char('k') => {
                        app.decrease_no_of_commits();
                        app.add_commits();
                    }
                    KeyCode::Char('l') => {
                        app.increase_no_of_commits();
                        app.add_commits();
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        return Ok(false);
                    }
                    KeyCode::Backspace => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
            }
        }
    }
}
