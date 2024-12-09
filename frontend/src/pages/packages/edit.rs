use crate::{
    api::{
        cards::get_selectable as get_selectable_cards,
        packages::{get as get_package, update as update_package},
    },
    components::{
        button::Button,
        card_picker::CardPicker,
        cards::{grid::CardGrid, grid_element::Display},
        form::{Field as FormField, Form},
        submit::Submit,
        textbox::Textbox,
    },
    route::Route,
};

use std::collections::HashMap;

use common::card::Card;
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub package_id: String,
}

#[function_component(PackageEdit)]
pub fn package_edit(props: &Props) -> Html {
    let package_id = props.package_id.clone();

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

    let select_cards = {
        let selected_cards = selected_cards.clone();

        Callback::from(move |cards: Vec<Card>| {
            let mut new_selected_cards = (*selected_cards).clone();

            for card in &cards {
                new_selected_cards.insert(card.id.clone(), (*card).clone());
            }

            selected_cards.set(new_selected_cards);
        })
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

    let package = use_state(|| None);
    let all_cards = use_state(|| None);
    {
        // let package = package.clone();
        let package_id = package_id.clone();
        let selected_cards = selected_cards.clone();
        let input_value = input_value.clone();
        let all_cards = all_cards.clone();

        use_effect_with((), move |()| {
            let package = package.clone();
            let package_id = package_id.clone();
            let selected_cards = selected_cards.clone();
            let input_value = input_value.clone();
            // let all_cards = all_cards.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_cards) = get_selectable_cards().await {
                    all_cards.set(Some(fetched_cards.clone()));

                    let package = package.clone();
                    let package_id = package_id.clone();
                    let selected_cards = selected_cards.clone();
                    let input_value = input_value.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(fetched_package) = get_package(&package_id).await {
                            package.set(Some(fetched_package.clone()));

                            let mut map: HashMap<String, Card> = HashMap::default();

                            for card in fetched_cards {
                                if fetched_package.cards.contains(&card.id) {
                                    map.insert(card.id.clone(), card.clone());
                                }
                            }

                            selected_cards.set(map);
                            input_value.set(fetched_package.name);
                        } else {
                            // TODO log e
                        }
                    });
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

    let updated = use_state(|| false);

    let submit = {
        let package_name = input_value.clone();
        let selected_cards = selected_cards;
        let package_id = package_id.clone();
        let updated = updated.clone();

        Callback::from(move |_: Submit| {
            let package_name = package_name.clone();
            let selected_cards = selected_cards.clone();
            let package_id = package_id.clone();
            let updated = updated.clone();

            #[allow(clippy::equatable_if_let)]
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(()) = update_package(
                    package_id.clone(),
                    (*package_name).clone(),
                    (*selected_cards).values().cloned().collect::<Vec<Card>>(),
                )
                .await
                {
                    updated.set(true);
                } else {
                    // TODO log e
                }
            });
        })
    };

    if *updated {
        return html! { <Redirect<Route> to={Route::PackageView { package_id }} /> };
    }

    return html! {
        <>
            <Form class_prefix={"package"}>
                <FormField id={"package-name-create"} class={""} label={"Package Name"}>
                    <Textbox id="package-name" value={(*input_value).clone()} name="card-filter" on_input={on_input} />
                </FormField>

                <FormField id={"selected-cards-container"} class={"selected-cards-container"} label={"Selected Cards"}>
                    <CardGrid cards={sorted_selected_cards.clone()} display={Display::Simple} on_click={deselect_cards} />
                </FormField>
            </Form>

            <Button<Submit> on_click={submit} value={Submit::Submit} selected=false />

            <h2>{"Card List"}</h2>

            <CardPicker excluded_cards={sorted_selected_cards.clone()} on_click={select_cards} />
        </>
    };
}
