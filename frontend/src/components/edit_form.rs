use crate::{
    api::packages::list as get_all_packages,
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

use super::{CardCollection, IntoClass, IntoLabel, ToRoute};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props<T>
where
    T: CardCollection + PartialEq + Eq + Clone + IntoClass + ToRoute + IntoLabel,
{
    pub given_object: Option<T>,
}

#[function_component(EditForm)]
pub fn edit_form<T>(props: &Props<T>) -> Html
where
    T: CardCollection + PartialEq + Eq + Clone + IntoClass + ToRoute + IntoLabel,
{
    let input_value = use_state(|| {
        props
            .given_object
            .as_ref()
            .map_or_else(String::default, CardCollection::name)
    });

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value());
            }
        })
    };

    let map = props
        .given_object
        .as_ref()
        .map_or_else(HashMap::<String, Card>::default, |object| {
            let mut map: HashMap<String, Card> = HashMap::default();

            object.cards().iter().for_each(|card| {
                map.insert(card.id.clone(), card.clone());
            });

            map
        });

    let selected_cards = use_state(|| map);

    let selected_ids = (*selected_cards).values().map(|card| card.id.clone());

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

    let object_id = use_state(|| None);
    let given_object = props.given_object.clone();

    let submit = {
        let name = input_value.clone();
        let selected_cards = selected_cards;
        let id = given_object.map(|value| value.id());
        let object_id = object_id.clone();

        Callback::from(move |_: Submit| {
            let name = name.clone();
            let selected_cards = selected_cards.clone();
            let id = id.clone();
            let object_id = object_id.clone();

            match id {
                Some(id) => {
                    wasm_bindgen_futures::spawn_local(async move {
                        object_id.set(
                            T::update(
                                id,
                                (*name).clone(),
                                (*selected_cards).values().cloned().collect::<Vec<Card>>(),
                            )
                            .await,
                        );
                    });
                }
                None => {
                    wasm_bindgen_futures::spawn_local(async move {
                        object_id.set(
                            T::create(
                                (*name).clone(),
                                (*selected_cards).values().cloned().collect::<Vec<Card>>(),
                            )
                            .await,
                        );
                    });
                }
            }
        })
    };

    if let Some(id) = (*object_id).clone() {
        return html! { <Redirect<Route> to={T::into_route(id)} /> };
    }

    let class_prefix = T::into_class();
    let label = format!("{} Name", T::into_label());

    return html! {
        <>
            <p>{(*input_value).clone()}</p>
            <Form class_prefix={class_prefix.clone()}>
                <FormField id={format!("{}-name-create", class_prefix)} class={""} label={label}>
                    <Textbox id={format!("{}-name", class_prefix)} value={(*input_value).clone()} name="card-filter" on_input={on_input} />
                </FormField>

                <FormField id={"selected-cards-container"} class={"selected-cards-container"} label={"Selected Cards"}>
                    <CardGrid cards={sorted_selected_cards.clone()} display={Display::Simple} on_click={deselect_cards} />
                </FormField>
            </Form>

            <Button<Submit> on_click={submit} value={Submit::Submit} selected=false />

            {
                props.given_object.as_ref().map_or_else(|| html! {}, |object| if object.display_packages() {
                    html! {
                        <>
                            <h2>{"Packages"}</h2>
                            {
                                (*all_packages).iter().map(|package| {
                                    html! {
                                        <h3 onclick={
                                            let select_cards = select_cards.clone();

                                            let package = package.clone();

                                            move |_| {
                                                let mut cards: Vec<Card> = Vec::default();

                                                package.cards.iter().for_each(|card| {
                                                    cards.push(card.clone());
                                                });

                                                select_cards.emit(cards);
                                            }
                                        }>{package.name.clone()}</h3>
                                    }
                                }).collect::<Html>()
                            }
                        </>
                    }
                } else {
                    html! {}
                })
            }

            <h2>{"Card List"}</h2>

            <CardPicker excluded_cards={sorted_selected_cards.clone()} on_click={select_cards} />
        </>
    };
}
