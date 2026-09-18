
#[derive(Debug)]
struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}
impl Time {
    fn new(timestamp:u32) -> Time {
        let total_days = timestamp / 86400;
        let remain_seconds = timestamp % 86400;
        let hour = remain_seconds / 3600;
        let minute = (remain_seconds % 3600) / 60;
        let second = remain_seconds % 60;
        let mut year = 1970u32;
        let mut remain_days = total_days;
        loop {
            let days = if Self::is_leap(year) { 366 } else { 365 };
            if remain_days >= days {
                remain_days -= days;
                year += 1;
            } else {
                break;
            }
        }
        let mut month = 1u32;
        loop {
            let d = Self::days_in_month(year, month);
            if remain_days >= d {
                remain_days -= d;
                month += 1;
            } else {
                break;
            }
        }
        let day = remain_days + 1;
        Time {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
    fn is_leap(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }
    fn days_in_month(year:u32, month:u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap(year) { 29 } else { 28 },
            _ => 0,
        }
    }
    fn format_display(&self) -> String {
        format!("{:02}:{:02}:{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

fn main() {
    let t0 = Time::new(1_700_000_001u32);
    let time = t0.format_display();
    println!("{}", time);
}
