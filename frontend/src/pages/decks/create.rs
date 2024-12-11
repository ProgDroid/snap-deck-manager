use crate::components::edit_form::EditForm;

use common::deck::Deck;
use yew::prelude::*;

#[function_component(DeckCreate)]
pub fn deck_create() -> Html {
    return html! {
        <EditForm<Deck> given_object={None} />
    };
}
