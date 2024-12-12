use crate::components::IntoClass;

use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Clear {
    #[default]
    Clear,
}

impl IntoClass for Clear {
    fn into_class() -> String {
        "clear".to_owned()
    }
}
