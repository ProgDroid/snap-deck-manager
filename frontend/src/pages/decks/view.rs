use crate::{
    api::decks::get as get_deck,
    components::cards::{grid::CardGrid, grid_element::Display},
    route::Route,
};
use common::card::Card;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub deck_id: String,
}

#[function_component(DeckView)]
pub fn deck_view(props: &Props) -> Html {
    let deck_id = props.deck_id.clone();
    let deck = use_state(|| None);
    {
        let deck = deck.clone();
        let deck_id = deck_id.clone();

        use_effect_with((), move |()| {
            let deck = deck.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_deck) = get_deck(&deck_id).await {
                    deck.set(Some(fetched_deck));
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    let navigator = use_navigator().unwrap(); // TODO fix

    let route = Route::DeckEdit { deck_id };

    let onclick = Callback::from(move |_| navigator.push(&route));

    if let Some(deck) = &*deck {
        let share_code = deck.share_code.clone();

        let mut cards = deck.cards.clone();
        cards.sort_unstable_by_key(|card| (card.cost, card.power, card.name.to_lowercase()));

        return html! {
            <>
                <table>
                    <tr>
                        <td>{"Deck Name"}</td>
                        <td>{deck.name.clone()}</td>
                    </tr>
                    <tr>
                        <td>{"Deck Share Code"}</td>
                        <td>{share_code}</td>
                    </tr>
                </table>
                <button {onclick}>{"Edit"}</button>
                <div class="selected-cards-container">
                    <CardGrid cards={cards} excluded_cards={Vec::<Card>::default()} display={Display::Simple} on_click={Callback::from(|_| {})}/>
                </div>
            </>
        };
    }

    return html! { <div>{"Loading..."}</div> };
}
