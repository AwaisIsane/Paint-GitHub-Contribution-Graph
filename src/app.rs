use chrono::{Datelike, Days, Local, NaiveDate};

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
    pub no_of_days: usize,
    pub start_date: Option<NaiveDate>,
}

impl App {
    pub fn new() -> Self {
        let current_local = Local::now().date_naive();
        let weekday = current_local.weekday().num_days_from_sunday() + 1;
        let current_commit_size = 52 * 7 + weekday as usize;
        let start_date = current_local.checked_sub_days(Days::new(52 * 7 + weekday as u64 - 1));

        return App {
            no_of_days: current_commit_size as usize,
            pos: 0,
            commits: vec![0; current_commit_size as usize], //52 weeks + till current day of week
            no_of_commits: 0,
            current_screen: CurrentScreen::Main,
            is_editing: true,
            start_date,
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
            self.commits[self.pos as usize] = self.no_of_commits;
        }
    }

    pub fn toggle_editing(&mut self) {
        self.is_editing = !self.is_editing;
    }

    pub fn move_up(&mut self) {
        self.pos = self.pos.checked_sub(1).unwrap_or(self.no_of_days - 1);
    }
    pub fn move_down(&mut self) {
        if (self.pos + 1) > (self.no_of_days - 1) {
            self.pos = 0;
        } else {
            self.pos = self.pos + 1
        }
    }
    pub fn move_left(&mut self) {
        self.pos = self
            .pos
            .checked_sub(7)
            .unwrap_or(std::cmp::min(self.pos + 52 * 7, self.no_of_days - 1));
    }
    pub fn move_right(&mut self) {
        if (self.pos + 7) > (self.no_of_days - 1) {
            self.pos = self.pos % 7;
        } else {
            self.pos = self.pos + 7
        }
    }
}
