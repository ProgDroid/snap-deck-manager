use crate::{
    api::cards::get_selectable as get_cards,
    components::{
        cards::{grid::CardGrid, grid_element::Display},
        filters::{
            buttons::FilterButtons,
            cost::Cost,
            released_only::ReleasedOnly,
            sort::{Order as SortOrder, Sort},
            textbox::{FilterTextbox, Type as TextboxType},
        },
    },
};
use common::card::Card;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub excluded_cards: Vec<Card>,
    pub on_click: Callback<Vec<Card>>,
}

#[function_component(CardPicker)]
pub fn card_picker(props: &Props) -> Html {
    let cards = use_state(|| None);
    {
        let cards = cards.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_cards) = get_cards().await {
                    cards.set(Some(fetched_cards));
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    let input_value = use_state(String::default);
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value());
            }
        })
    };

    let filter_value = (*input_value).clone();

    let selected_cost = use_state(Cost::default);

    let cost_filter_select = {
        let selected_cost = selected_cost.clone();

        Callback::from(move |cost: Cost| {
            selected_cost.set(cost);
        })
    };

    let selected_sort = use_state(Sort::default);

    let sort_filter_select = {
        let selected_sort = selected_sort.clone();

        Callback::from(move |sort: Sort| {
            selected_sort.set(sort);
        })
    };

    let selected_sort_order = use_state(SortOrder::default);

    let sort_order_filter_select = {
        let selected_sort_order = selected_sort_order.clone();

        Callback::from(move |sort_order: SortOrder| {
            selected_sort_order.set(sort_order);
        })
    };

    let selected_released_only = use_state(ReleasedOnly::default);

    let released_only_select = {
        let selected_released_only = selected_released_only.clone();

        Callback::from(move |released_only: ReleasedOnly| {
            selected_released_only.set(released_only);
        })
    };

    if let Some(cards) = &*cards {
        let mut filtered_cards: Vec<Card> = cards
            .iter()
            .filter(|card| match *selected_released_only {
                ReleasedOnly::No => true,
                ReleasedOnly::Yes => card.released,
            })
            .filter(|card| (*selected_cost).compare(card.cost))
            .filter(|card| {
                card.name
                    .to_lowercase()
                    .contains(&(filter_value.to_lowercase()))
            })
            .cloned()
            .collect();

        #[allow(clippy::cast_possible_wrap)]
        match *selected_sort {
            Sort::Cost => {
                filtered_cards
                    .sort_unstable_by_key(|card| (card.cost, card.power, card.name.to_lowercase()));
            }
            Sort::Power => {
                filtered_cards
                    .sort_unstable_by_key(|card| (card.power, card.cost, card.name.to_lowercase()));
            }
            Sort::Alphabetical => filtered_cards.sort_by_key(|card| card.name.to_lowercase()),
        };

        if *selected_sort_order == SortOrder::Desc {
            filtered_cards.reverse();
        }

        return html! {
            <>
                <div id="card-picker" class="scroll-box-container">
                    <FilterButtons<Sort> label={"Sort"} select={sort_filter_select} selected={(*selected_sort).clone()} />
                    <FilterButtons<SortOrder> label={"Sort Order"} select={sort_order_filter_select} selected={(*selected_sort_order).clone()} />
                    <FilterTextbox filter_type={TextboxType::Search} value={(*input_value).clone()} on_input={on_input} />
                    <FilterButtons<Cost> label={"Cost"} select={cost_filter_select} selected={(*selected_cost).clone()} />
                    <FilterButtons<ReleasedOnly> label={"Released Only"} select={released_only_select} selected={(*selected_released_only).clone()} />

                    <div class="scroll-box">
                    {
                        html! {
                            <CardGrid cards={filtered_cards.clone()} excluded_cards={props.excluded_cards.clone()} display={Display::Detailed} on_click={props.on_click.clone()}/>
                        }
                    }
                    </div>
                </div>
            </>
        };
    }

    return html! { <div>{"Loading..."}</div> };
}
