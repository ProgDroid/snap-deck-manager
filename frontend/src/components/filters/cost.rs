use strum::{Display, EnumIter};

use crate::components::IntoClass;

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Cost {
    #[default]
    Unset,
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

impl IntoClass for Cost {
    fn into_class() -> String {
        "cost".to_owned()
    }
}

impl Cost {
    pub const fn compare(&self, value: i8) -> bool {
        match self {
            Self::Unset => true,
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
