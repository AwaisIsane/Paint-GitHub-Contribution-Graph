use chrono::{Datelike, Days, Local, NaiveDate, Weekday};

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
    pub start_date: NaiveDate,
}

impl App {
    pub fn new(year: &str) -> Self {
        let current_local = Local::now().date_naive();
        let weekday = current_local.weekday().num_days_from_sunday() + 1;
        let current_commit_size = 52 * 7 + weekday as usize;

        let start_date = if year == "default_value" {
            current_local
                .checked_sub_days(Days::new(52 * 7 + weekday as u64 - 1))
                .unwrap()
        } else {
            let year = year.parse::<i32>().unwrap();
            let mut dt = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            if dt.leap_year() && dt.weekday() == Weekday::Sat {
                dt = NaiveDate::from_ymd_opt(year, 1, 1).unwrap(); //case where there are 54 weeks in github graph github removes the 1 saturday eg year 2000
            }
            dt
        };

        let no_of_days = if year == "default_value" {
            52 * 7 + weekday
        } else if start_date.leap_year() && start_date.weekday() != Weekday::Sat {
            366
        } else {
            365
        };

        return App {
            no_of_days: no_of_days as usize,
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
