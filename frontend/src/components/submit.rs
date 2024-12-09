use crate::components::IntoClass;

use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Submit {
    #[default]
    Submit,
}

impl IntoClass for Submit {
    fn into_class() -> String {
        "submit".to_owned()
    }
}
