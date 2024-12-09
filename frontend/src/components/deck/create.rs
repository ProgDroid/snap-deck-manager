use crate::{
    api::{
        cards::get_selectable as get_selectable_cards, decks::create as create_deck,
        packages::list as get_all_packages,
    },
    components::{
        card_picker::CardPicker,
        cards::{grid::CardGrid, grid_element::Display},
    },
    route::Route,
};

use std::collections::HashMap;

use common::card::Card;
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[function_component(DeckCreate)]
pub fn deck_create() -> Html {
    let input_value = use_state(String::default);

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value());
            }
        })
    };

    let selected_cards = use_state(HashMap::<String, Card>::default);

    let selected_ids: Vec<String> = (*selected_cards)
        .values()
        .map(|card| card.id.clone())
        .collect();

    let select_cards = if selected_ids.len() >= 12 {
        Callback::from(|_| {})
    } else {
        {
            let selected_cards = selected_cards.clone();

            Callback::from(move |cards: Vec<Card>| {
                let mut new_selected_cards = (*selected_cards).clone();

                for card in &cards {
                    new_selected_cards.insert(card.id.clone(), (*card).clone());
                }

                selected_cards.set(new_selected_cards);
            })
        }
    };

    let deselect_cards = {
        let selected_cards = selected_cards.clone();

        Callback::from(move |cards: Vec<Card>| {
            let mut new_selected_cards = (*selected_cards).clone();

            for card in &cards {
                let _ = new_selected_cards.remove(card.id.as_str());
            }

            selected_cards.set(new_selected_cards);
        })
    };

    let all_cards = use_state(|| None);
    {
        let all_cards = all_cards.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_cards) = get_selectable_cards().await {
                    all_cards.set(Some(fetched_cards));
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    let mut filtered_cards: Vec<Card> =
        (*all_cards)
            .as_ref()
            .map_or_else(Vec::default, |fetched_cards| {
                fetched_cards
                    .iter()
                    .filter(|card| !selected_ids.contains(&card.id))
                    .cloned()
                    .collect()
            });

    filtered_cards.sort_unstable_by_key(|card| (card.cost, card.power, card.name.to_lowercase()));

    let mut sorted_selected_cards = (*selected_cards).values().cloned().collect::<Vec<Card>>();

    sorted_selected_cards
        .sort_unstable_by_key(|card| (card.cost, card.power, card.name.to_lowercase()));

    let all_packages = use_state(Vec::default);
    {
        let all_packages = all_packages.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_packages) = get_all_packages().await {
                    all_packages.set(fetched_packages);
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    let deck_id = use_state(String::default);

    let submit = {
        let deck_name = input_value.clone();
        let selected_cards = selected_cards;
        let deck_id = deck_id.clone();

        Callback::from(move |()| {
            let deck_name = deck_name.clone();
            let selected_cards = selected_cards.clone();
            let deck_id = deck_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = create_deck(
                    (*deck_name).clone(),
                    (*selected_cards).values().cloned().collect::<Vec<Card>>(),
                )
                .await
                {
                    deck_id.set(response.id);
                } else {
                    // TODO log e
                }
            });
        })
    };

    if !(*deck_id).is_empty() {
        return html! { <Redirect<Route> to={Route::DeckView { deck_id: (*deck_id).clone() }} /> };
    }

    return html! {
        <>
            <div class="deck-form-container">
                <div class="deck-name-create">
                    <label for="deck-name">{"Deck Name:"}</label>
                    <input type="text" id="deck-name" value={(*input_value).clone()} name="card-filter" oninput={on_input.clone()}/>
                </div>

                <div class="selected-cards-container">
                    <h2>{"Selected Cards:"}</h2>
                    <CardGrid cards={sorted_selected_cards.clone()} display={Display::Simple} on_click={deselect_cards} />
                </div>
            </div>

            <button onclick={
                let submit = submit.clone();
                move |_| submit.emit(())
            }>{"Submit"}</button>

            <h2>{"Packages"}</h2>
            {
                (*all_packages).iter().map(|package| {
                    html! {
                        <h3 onclick={
                            let select_cards = select_cards.clone();
                            let all_cards = all_cards.clone();

                            let package = package.clone();

                            move |_| {
                                let mut cards: Vec<Card> = Vec::default();

                                cards.append(
                                    &mut (*all_cards).as_ref().map_or_else(Vec::default, |fetched_cards| {
                                        fetched_cards.iter().filter(|card| package.cards.contains(&card.id)).cloned().collect()
                                    })
                                );

                                select_cards.emit(cards);
                            }
                        }>{package.name.clone()}</h3>
                    }
                }).collect::<Html>()
            }

            <h2>{"Card List"}</h2>

            <CardPicker excluded_cards={sorted_selected_cards.clone()} on_click={select_cards} />
        </>
    };
}
