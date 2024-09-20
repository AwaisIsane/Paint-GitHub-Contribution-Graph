use std::{error::Error, io};

use app::CurrentScreen;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod graph;
mod ui;
use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(result) = res {
        println!("{result:?}");
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
                        app.add_commits();
                        app.move_up();
                    }
                    KeyCode::Char('a') | KeyCode::Left => {
                        app.add_commits();
                        app.move_left();
                    }
                    KeyCode::Char('s') | KeyCode::Down => {
                        app.add_commits();
                        app.move_down();
                    }
                    KeyCode::Char('d') | KeyCode::Right => {
                        app.add_commits();
                        app.move_right();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Exiting;
                        return Ok(true); //for now
                    }
                    KeyCode::Char('e') => {
                        app.toggle_editing();
                    }
                    KeyCode::Char('k') => {
                        app.decrease_no_of_commits();
                    }
                    KeyCode::Char('l') => {
                        app.increase_no_of_commits();
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
            }
        }
    }
}
