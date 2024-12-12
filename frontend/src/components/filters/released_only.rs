use crate::components::IntoClass;

use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum ReleasedOnly {
    #[default]
    No,
    Yes,
}

impl IntoClass for ReleasedOnly {
    fn into_class() -> String {
        "released-only".to_owned()
    }
}
