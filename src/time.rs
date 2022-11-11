use std::ops::{Add, Mul, Sub};

use chrono::Timelike;

const HOUR_COUNT: u16 = 24;
const MINUTE_COUNT: u16 = 60;
const TOTAL_MINUTE_COUNT: u16 = HOUR_COUNT * MINUTE_COUNT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeOutOfRange {
    pub input_total_minutes: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InvalidHourMinute {
    pub hours: u16,
    pub minutes: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Time {
    minutes: u16,
}

impl Time {
    pub fn new_cycling(total_minutes: u16) -> Self {
        Self { minutes: total_minutes % TOTAL_MINUTE_COUNT }
    }

    pub fn new_cycling_wide(total_minutes: u32) -> Self {
        Self { minutes: (total_minutes % u32::from(TOTAL_MINUTE_COUNT)) as u16 }
    }

    pub fn from_total_minutes(total_minutes: u16) -> Self {
        Self::try_from_total_minutes(total_minutes)
            .expect("total minutes out of range")
    }

    pub fn try_from_total_minutes(
        total_minutes: u16,
    ) -> Result<Self, TimeOutOfRange> {
        if total_minutes < TOTAL_MINUTE_COUNT {
            Ok(Self { minutes: total_minutes })
        } else {
            Err(TimeOutOfRange { input_total_minutes: total_minutes })
        }
    }

    pub fn from_hour_minute(hour: u16, minute: u16) -> Self {
        Self::try_from_hour_minute(hour, minute)
            .expect("either hour or minute out of range")
    }

    pub fn try_from_hour_minute(
        hour: u16,
        minute: u16,
    ) -> Result<Self, InvalidHourMinute> {
        if hour < HOUR_COUNT && minute < MINUTE_COUNT {
            Ok(Self::from_total_minutes(hour * MINUTE_COUNT + minute))
        } else {
            Err(InvalidHourMinute { hours: hour, minutes: minute })
        }
    }

    pub fn from_crono<T>(timelike: &T) -> Self
    where
        T: Timelike + ?Sized,
    {
        Self::try_from_crono(timelike)
            .expect("either hour or minute out of range")
    }

    pub fn try_from_crono<T>(timelike: &T) -> Result<Self, InvalidHourMinute>
    where
        T: Timelike + ?Sized,
    {
        let hour = u16::try_from(timelike.hour()).unwrap_or(u16::MAX);
        let minute = u16::try_from(timelike.minute()).unwrap_or(u16::MAX);
        Self::try_from_hour_minute(hour, minute)
    }

    pub fn total_minutes(self) -> u16 {
        self.minutes
    }

    pub fn minute(self) -> u16 {
        self.minutes % MINUTE_COUNT
    }

    pub fn hour(self) -> u16 {
        self.minutes / MINUTE_COUNT
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_cycling(self.total_minutes() + rhs.total_minutes())
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_cycling(self.total_minutes() - rhs.total_minutes())
    }
}

impl Mul for Time {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = u32::from(self.total_minutes());
        let rhs = u32::from(rhs.total_minutes());
        Self::new_cycling_wide(lhs * rhs)
    }
}
