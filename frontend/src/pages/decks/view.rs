use crate::{
    api::decks::get as get_deck,
    components::{
        cards::{grid::CardGrid, grid_element::Display},
        pill::{Class, DefaultValue, Pill},
    },
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

        let on_click: Callback<DefaultValue> = Callback::default();

        // TODO fix formatting

        return html! {
            <>
                <h1>{deck.name.clone()}</h1>
                <h2>{"Share Code"}</h2>
                <p>{share_code}</p>
                <h2>{"Cards"}</h2>
                <div class="selected-cards-container">
                    <CardGrid cards={cards} excluded_cards={Vec::<Card>::default()} display={Display::Simple} on_click={Callback::from(|_| {})}/>
                </div>
                <h2>{"Game Modes"}</h2>
                <div>
                {
                    deck.game_modes.iter().map(|game_mode| {
                        html! {
                            <Pill<DefaultValue> class={Class::Success} content={game_mode.name.clone()} on_click={on_click.clone()} value={DefaultValue::default()} />
                        }
                    }).collect::<Html>()
                }
                </div>
                <br />
                <div>
                    <button {onclick}>{"Edit"}</button>
                </div>
            </>
        };
    }

    return html! { <div>{"Loading..."}</div> };
}
