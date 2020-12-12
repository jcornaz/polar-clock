use chrono::{DateTime, Datelike, TimeZone, Timelike};

const DAY_BY_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const CUMULATIVE_DAY_BY_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

pub(crate) trait DateTimeExt {
    fn millisecond_of_second(&self) -> u32;
    fn millisecond_of_minute(&self) -> u32;
    fn millisecond_of_hour(&self) -> u32;
    fn second_of_day(&self) -> u32;
    fn minute_of_month(&self) -> u32;
    fn day_of_year(&self) -> u32;
    fn hour_of_year(&self) -> u32;

    fn max_minute_of_month(&self) -> u32;
    fn max_day_of_year(&self) -> u32;
    fn max_hour_of_year(&self) -> u32;
}

impl<T: TimeZone> DateTimeExt for DateTime<T> {
    fn millisecond_of_second(&self) -> u32 {
        self.nanosecond() / 1_000_000
    }

    fn millisecond_of_minute(&self) -> u32 {
        (self.second() * 1_000) + self.millisecond_of_second()
    }

    fn millisecond_of_hour(&self) -> u32 {
        (self.minute() * 60_000) + self.millisecond_of_minute()
    }

    fn second_of_day(&self) -> u32 {
        (self.hour() * 3_600) + (self.minute() * 60) + self.second()
    }

    fn minute_of_month(&self) -> u32 {
        ((self.day0()) * 1440) + (self.hour() * 60) + self.minute()
    }

    fn day_of_year(&self) -> u32 {
        let month = self.month0();
        let mut day_in_past_months = CUMULATIVE_DAY_BY_MONTH[month as usize];
        if month > 1 && is_leap_year(self.year()) {
            day_in_past_months += 1;
        }
        day_in_past_months + self.day0()
    }

    fn hour_of_year(&self) -> u32 {
        (self.day_of_year() * 24) + self.hour()
    }

    fn max_minute_of_month(&self) -> u32 {
        let month = self.month0();
        let mut max_day_in_month = DAY_BY_MONTH[month as usize];
        if month == 1 && is_leap_year(self.year()) {
            max_day_in_month += 1;
        }
        max_day_in_month * 1440
    }

    fn max_day_of_year(&self) -> u32 {
        if is_leap_year(self.year()) {
            366
        } else {
            365
        }
    }

    fn max_hour_of_year(&self) -> u32 {
        self.max_day_of_year() * 24
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}
