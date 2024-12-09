use crate::components::IntoClass;

use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum View {
    #[default]
    Grid,
    List,
}

impl IntoClass for View {
    fn into_class() -> String {
        "view".to_owned()
    }
}
