use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes_since_midnight: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(1440);
        Clock {
            minutes_since_midnight: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes_since_midnight + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes_since_midnight / 60;
        let minutes = self.minutes_since_midnight % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
