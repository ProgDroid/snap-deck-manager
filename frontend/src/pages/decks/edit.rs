use crate::{api::decks::get as get_deck, components::edit_form::EditForm};

use common::deck::Deck;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub deck_id: String,
}

#[function_component(DeckEdit)]
pub fn deck_edit(props: &Props) -> Html {
    // TODO skeleton, might be hard as it is now

    let deck_id = props.deck_id.clone();

    let deck = use_state(|| None);
    {
        let deck = deck.clone();

        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_deck) = get_deck(&deck_id).await {
                    deck.set(Some(fetched_deck));
                } else {
                    // TODO log e
                }
            });
        });
    }

    if (*deck).is_none() {
        return html! {};
    }

    return html! {
        <EditForm<Deck> given_object={(*deck).clone()} />
    };
}
