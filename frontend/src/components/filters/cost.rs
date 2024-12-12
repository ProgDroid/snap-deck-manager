use std::fmt::Display;

use strum::EnumIter;

use crate::components::IntoClass;

#[derive(Eq, PartialEq, Clone, EnumIter, Hash)]
pub enum Cost {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Display for Cost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zero => "0",
                Self::One => "1",
                Self::Two => "2",
                Self::Three => "3",
                Self::Four => "4",
                Self::Five => "5",
                Self::Six => "6",
                Self::Seven => "7",
                Self::Eight => "8",
            }
        )
    }
}

impl IntoClass for Cost {
    fn into_class() -> String {
        "cost".to_owned()
    }
}

impl Cost {
    pub const fn compare(&self, value: i8) -> bool {
        match self {
            Self::Zero => value == 0,
            Self::One => value == 1,
            Self::Two => value == 2,
            Self::Three => value == 3,
            Self::Four => value == 4,
            Self::Five => value == 5,
            Self::Six => value == 6,
            Self::Seven => value == 7,
            Self::Eight => value == 8,
        }
    }
}
