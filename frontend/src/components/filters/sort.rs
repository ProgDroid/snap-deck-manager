use crate::components::IntoClass;

use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Sort {
    Alphabetical,
    #[default]
    Cost,
    Power,
}

impl IntoClass for Sort {
    fn into_class() -> String {
        "sort".to_owned()
    }
}

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Order {
    #[default]
    Asc,
    Desc,
}

impl IntoClass for Order {
    fn into_class() -> String {
        "order".to_owned()
    }
}
