use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum View {
    #[default]
    Grid,
    List,
}
