use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24), // 0 <= final_hours < 24
            minutes: minutes.rem_euclid(60),                        // 0 <= final_minutes < 60
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        let extra_hours = new_minutes.div_euclid(60);

        Self::new(
            (self.hours + extra_hours).rem_euclid(24), // 0 <= final_hours < 24
            new_minutes.rem_euclid(60),                // 0 <= final_minutes < 60
        )
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
