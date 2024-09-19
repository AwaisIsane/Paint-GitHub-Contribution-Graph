use chrono::{DateTime, Datelike, Local};

pub enum CurrentScreen {
    Main,
    Exiting,
}
pub struct App {
    pub pos: usize,
    pub commits: Vec<i32>,
    pub is_editing: bool,
    pub no_of_commits: i32,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        let current_local: DateTime<Local> = Local::now();
        let current_commit_size = 52 * 7 + current_local.ordinal() % 7;
        return App {
            pos: 0,
            commits: vec![0, current_commit_size as i32], //52 weeks + till current day of week
            no_of_commits: 0,
            current_screen: CurrentScreen::Main,
            is_editing: true,
        };
    }

    pub fn increase_no_of_commits(&mut self) {
        self.no_of_commits += 1;
    }

    pub fn decrease_no_of_commits(&mut self) {
        if self.no_of_commits > 0 {
            self.no_of_commits -= 1;
        }
    }

    pub fn add_commits(&mut self) {
        if self.is_editing {
            self.commits[self.pos] = self.no_of_commits;
        }
    }

    pub fn toggle_editing(&mut self) {
        self.is_editing = !self.is_editing;
    }

    pub fn move_up(&mut self) {
        self.pos = std::cmp::max(0, self.pos - 1);
    }
    pub fn move_down(&mut self) {
        self.pos = std::cmp::min(self.commits.len() - 1, self.pos + 1);
    }
    pub fn move_left(&mut self) {
        self.pos = std::cmp::max(0, self.pos - 7);
    }
    pub fn move_right(&mut self) {
        self.pos = std::cmp::min(self.commits.len() - 1, self.pos + 7);
    }
}
