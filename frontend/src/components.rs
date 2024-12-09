pub mod button;
pub mod card_list;
pub mod card_picker;
pub mod cards;
pub mod clickable;
pub mod deck;
pub mod filters;
pub mod footer;
pub mod form;
pub mod header;
pub mod icon_link;
pub mod logo;
pub mod nav_link;
pub mod packages;
pub mod submit;
pub mod textbox;
pub mod vertical_centre_container;

pub trait IntoClass {
    fn into_class() -> String;
}
