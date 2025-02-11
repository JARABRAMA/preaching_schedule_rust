use super::SEPARATOR;
use std::{fmt::Display, str::FromStr};
use time::{Date, Month};

#[derive(Debug, PartialEq)]
pub struct PreachingDay {
    id: i32,
    date: Date,
    minutes: i64,
}

/// Duration(hours, minutes)
/// if we have preached tree hours and 45 minutes we well have
/// Duration(3, 45)
pub struct Duration(i64, i64);

impl Display for PreachingDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day {}, hours: {}", self.get_day(), self.get_hours())
    }
}

impl PreachingDay {
    pub fn new(date: Date, minutes: i64) -> Self {
        Self {
            id: rand::random(),
            date,
            minutes,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    fn get_day(&self) -> String {
        format!(
            "{} {} of {}",
            self.date.weekday(),
            self.date.day(),
            self.date.month()
        )
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}{SEPARATOR}{}{SEPARATOR}{}{SEPARATOR}{}{SEPARATOR}{}",
            self.id,
            self.date.year(),
            self.date.month(),
            self.date.day(),
            self.minutes
        )
    }

    pub fn from_string(string: &str) -> Result<Self, String> {
        let att = string
            .split(SEPARATOR)
            .map(|it| it.trim())
            .collect::<Vec<&str>>();

        if att.len() != 5 {
            return Err(String::from("Not expected arguments"));
        }
        let id = att[0].parse::<i32>().map_err(|err| err.to_string())?;
        let year = att[1].parse::<i32>().map_err(|err| err.to_string())?;
        let month = Month::from_str(att[2]).map_err(|err| err.to_string())?;
        let day = att[3].parse::<u8>().map_err(|err| err.to_string())?;
        let minutes = att[4].parse::<i64>().map_err(|err| err.to_string())?;

        let date = Date::from_calendar_date(year, month, day).map_err(|err| err.to_string())?;

        Ok(Self { id, date, minutes })
    }

    fn get_hours(&self) -> String {
        let duration = parse_minutes(self.minutes);
        format!("{}:{}", duration.0, duration.1)
    }
}

fn parse_minutes(minutes: i64) -> Duration {
    let hours = minutes / 60;
    let minutes = minutes % 60;
    Duration(hours, minutes)
}

#[cfg(test)]
mod test {
    use super::*;
    use time::{Date, Month};

    #[test]
    fn new_preacing_day() {
        let date = Date::from_calendar_date(2022, Month::February, 10).unwrap();
        let preaching_day = PreachingDay::new(date, 135);

        assert_eq!(preaching_day.minutes, 135);
        assert_eq!(preaching_day.date, date);
    }

    #[test]
    fn to_string() {
        let expected = "1,2019,March,10,60";
        let preaching_day = PreachingDay {
            id: 1,
            date: Date::from_calendar_date(2019, Month::March, 10).unwrap(),
            minutes: 60,
        };

        assert_eq!(preaching_day.to_str(), expected)
    }

    #[test]
    fn preaching_day_from_string() {
        let string = "1,2019,March,10,60";
        let expected = PreachingDay {
            id: 1,
            date: Date::from_calendar_date(2019, Month::March, 10).unwrap(),
            minutes: 60,
        };

        assert_eq!(PreachingDay::from_string(string), Ok(expected))
    }

    #[test]
    fn preaching_day_from_string_not_trimed() {
        let string = "1 , 2019, March,10,60";
        let expected = PreachingDay {
            id: 1,
            date: Date::from_calendar_date(2019, Month::March, 10).unwrap(),
            minutes: 60,
        };

        assert_eq!(PreachingDay::from_string(string), Ok(expected))
    }
}
