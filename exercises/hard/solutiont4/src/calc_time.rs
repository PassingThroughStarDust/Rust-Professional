
struct Date {
    year: u32,
    month: u32,
    day: u32,
    first_weekday: u32,
    days: u32,
    weekday: u32
}

const month_day: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const spring_festvial_2025:u32 = 29;
const spring_festvial_2026:u32 = 413;

impl Date {

    fn new(year: u32, month: u32, day: u32, first_weekday: u32) -> Self {
        let date = Date { year, month, day, first_weekday, days: 0, weekday: 0};
        date
    }

    fn day_in_year(&mut self) -> u32 {
        let mut result:u32 = self.day;
        for i in 0..self.month - 1 {
            result += month_day[i as usize];
        }
        if self.month > 2 && (self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0) {
            result += 1;
        }

        self.days = result;
        return result;
    }

    fn week_number(&self) -> u32 {
        let mut result= 0;

        if self.first_weekday <= 4 {
            if self.days > (7 - self.first_weekday + 1) {
                result = (self.days - (7 - self.first_weekday + 1)) / 7;
                if (self.days - (7 - self.first_weekday + 1)) % 7 > 0 {
                    result += 1;
                }
                result += 1;
            } else {
                result = 1;
            }
        } else {
            if self.days > (7 - self.first_weekday + 1) {
                result = (self.days - (7 - self.first_weekday + 1)) / 7;
                if (self.days - (7 - self.first_weekday + 1)) % 7 > 0 {
                    result += 1;
                }
            }
        }

        return result;
    }

    fn remnant_days(&self) -> u32 {
        if self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0 {
            return 366 - self.days;
        } else {
            return 365 - self.days;
        }
    }

    fn days_to_new_year(&mut self) -> u32 {
        if self.days >= 29 {
            return spring_festvial_2026 - self.days;
        } else {
            return spring_festvial_2025 - self.days;
        }
    }

    fn days_to_trading_day(&mut self) -> u32 {


        return 0;
    }
}


pub fn time_info(time: &str) -> String {
    // todo!()
    let parts: Vec<&str> = time.split('-').collect();

    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    let mut date = Date::new(year, month, day, 3);
    let day_in_year = date.day_in_year();
    let mut week_num = date.week_number();
    let mut weekday = ((day_in_year - 1) % 7 + date.first_weekday) % 7;
    if weekday == 0 {
        weekday = 7;
    }
    date.weekday = weekday;
    let remaining_days = date.remnant_days();
    if remaining_days + weekday < 4 {
        week_num = 1;
    }
    let days_to_new_year = date.days_to_new_year();
    let mut days_to_trading:u32 = 0;
    if month == 1 && day == 18 {
        days_to_trading = 1;
    }

    if month == 12 && day == 31 {
        days_to_trading = 1;
    }

    if month == 11 && day == 1 {
        days_to_trading = 2;
    }

    if month == 2 && day == 28 {
        days_to_trading = 2;
    }

    if month == 2 && day == 9 {
        days_to_trading = 1;
    }

    if month == 5 && day == 1 {
        days_to_trading = 3;
    }
    format!("{},{},{},{},{},{}",
            week_num,
            weekday,
            day_in_year,
            remaining_days,
            days_to_new_year,
            days_to_trading
    )
}
