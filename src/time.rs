use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Time {
    pub fn take_time(&self) -> String {
        format!(
            "{}-{}-{} {:02}:{}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn get_time() -> Time {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs();

    let mut days = (now / 86400) as i64;
    let mut year = 1970;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if days >= days_in_year {
            days -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }

    let month_day = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 0;

    loop {
        let mut days_in_month = month_day[month];
        if month == 1 && is_leap(year) {
            days_in_month = 29;
        }

        if days >= days_in_month {
            days -= days_in_month;
            month += 1;
        } else {
            break;
        }
    }

    month += 1;
    let days = days + 1;

    let hour = (now / 3600 % 24 + 7) % 24;
    let minute = (now / 60 % 60) as u32;

    Time {
        year: year as u32,
        month: month as u32,
        day: days as u32,
        hour: hour as u32,
        minute,
    }
}
