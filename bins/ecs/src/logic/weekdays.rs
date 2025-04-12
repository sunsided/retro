use std::fmt::{Debug, Display};

/// The days of the week.
pub enum Weekday {
    /// Derived from “Willo’s Day” for purpose and intent.
    Willoday,
    /// From “Rand’s Day,” echoing a rough, unplanned start.
    Randay,
    /// Reflecting quiet resolve.
    Stillday,
    /// From “Hardy’s Day,” signifying industrious spirit.
    Harday,
    /// Evolved from “Chance’s Day,” capturing the day’s inherent unpredictability.
    Chanday,
    /// A natural shortening of “Earnest’s Day,” emphasizing sincere effort.
    Earnesday,
    /// From “Gloam’s Day,” evoking the waning light with poetic nuance.
    Gloamday,
    /// A streamlined version of “Justis’ Day,” where the suffix drops off for a crisp sound.
    Justis,
    /// Directly from “Veil’s Day,” retaining a hint of mystery.
    Veilday,
    /// From “Olden’s Day,” nodding to the weight of time and memory.
    Oldenday,
    /// The day that can never happen.
    Netherday,
}

impl From<u32> for Weekday {
    fn from(value: u32) -> Self {
        match value % 10 {
            0 => Self::Willoday,
            1 => Self::Randay,
            2 => Self::Stillday,
            3 => Self::Harday,
            4 => Self::Chanday,
            5 => Self::Earnesday,
            6 => Self::Gloamday,
            7 => Self::Justis,
            8 => Self::Veilday,
            9 => Self::Oldenday,
            _ => Self::Netherday,
        }
    }
}

impl From<f32> for Weekday {
    fn from(value: f32) -> Self {
        (value as u32).into()
    }
}

impl Weekday {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Weekday::Willoday => "Willoday",
            Weekday::Randay => "Randay",
            Weekday::Stillday => "Stillday",
            Weekday::Harday => "Harday",
            Weekday::Chanday => "Chanday",
            Weekday::Earnesday => "Earnesday",
            Weekday::Gloamday => "Gloamday",
            Weekday::Justis => "Justis",
            Weekday::Veilday => "Veilday",
            Weekday::Oldenday => "Oldenday",
            Weekday::Netherday => "Netherday",
        }
    }

    pub const fn day_of_week(&self) -> usize {
        match self {
            Weekday::Willoday => 0,
            Weekday::Randay => 1,
            Weekday::Stillday => 2,
            Weekday::Harday => 3,
            Weekday::Chanday => 4,
            Weekday::Earnesday => 5,
            Weekday::Gloamday => 6,
            Weekday::Justis => 7,
            Weekday::Veilday => 8,
            Weekday::Oldenday => 9,
            // Rumors have it there once was an eleventh day.
            Weekday::Netherday => 10,
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())?;
        f.write_str(", the ")?;
        Debug::fmt(&(self.day_of_week() + 1), f)?;
        f.write_str(match self.day_of_week() {
            0 => "st",
            1 => "nd",
            2 => "rd",
            _ => "th",
        })?;
        f.write_str(" day of the week")
    }
}
