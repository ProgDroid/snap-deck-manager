use strum::{Display, EnumIter};

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Sort {
    Alphabetical,
    #[default]
    Cost,
    Power,
}

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum Order {
    #[default]
    Asc,
    Desc,
}
