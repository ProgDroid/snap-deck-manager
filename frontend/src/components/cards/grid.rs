use crate::components::cards::grid_element::{Display, GridElement};
use common::card::Card;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cards: Vec<Card>,
    pub excluded_cards: Vec<Card>,
    pub display: Display,
    pub on_click: Callback<Vec<Card>>,
    #[prop_or_default]
    pub hover_interaction: bool,
}

#[function_component(CardGrid)]
pub fn card_grid(props: &Props) -> Html {
    html! {
        <div class="grid grid-cols-3 md:grid-cols-6 gap-4">
        {
            props.cards.iter().map(|card| {
                let excluded = props.excluded_cards.contains(card);

                html! {
                    <GridElement
                        card={card.clone()}
                        display={props.display.clone()}
                        on_click={if excluded { Callback::from(|_| {}) } else { props.on_click.clone() }}
                        excluded={excluded}
                        hover_interaction={props.hover_interaction}
                    />
                }
            }).collect::<Html>()
        }
        </div>
    }
}
