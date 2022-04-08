use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub enum Value {
    HOUR = 24,
    MINUTE = 60
}

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

//TODO: implement methods to use + and - for adding 2 Clocks or a Clock and an i32

impl Clock {

    fn sanitize_neg_value(v: i32, t: &Value) -> (i32, i32) {
        let mut value = v;
        let mut counter = 0;
        while value < 0 {
            value += *t as i32;
            counter += 1;
        }
        (value, counter)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (sanitized_h, _) = Clock::sanitize_neg_value(hours, &Value::HOUR);
        let (sanitized_m, h_rolls) = Clock::sanitize_neg_value(minutes, &Value::MINUTE);
        let h = ((sanitized_h + (sanitized_m / 60)) % 24) - (h_rolls % 24);
        let m = sanitized_m % 60;
        Clock {
            hours: if h < 0 { h + 24 } else { h },
            minutes: m
        }
    }

    pub fn to_string(&self) -> String {
        let h_str = if self.hours < 10 { format!("0{}", self.hours) }
                            else { self.hours.to_string() };
        let m_str = if self.minutes < 10 { format!("0{}", self.minutes) }
                            else { self.minutes.to_string() };
        format!("{}:{}", h_str, m_str)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
