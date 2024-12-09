use crate::components::cards::grid_element::{Display, GridElement};
use common::card::Card;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cards: Vec<Card>,
    pub display: Display,
    pub on_click: Callback<Vec<Card>>,
}

#[function_component(CardGrid)]
pub fn card_grid(props: &Props) -> Html {
    html! {
        <div class="card-grid">
            {
                props.cards.iter().map(|card| {
                    html! {
                        <GridElement
                            card={card.clone()}
                            display={props.display.clone()}
                            on_click={props.on_click.clone()}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
