use common::card::Card;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct Props {
    pub cards: Vec<Card>,
}

#[function_component(CardPreview)]
pub fn card_preview(props: &Props) -> Html {
    html! {
        <div class="stack stack-end stack-top size-28 grid-cols-[8px_9px_1fr_9px_8px]">
            {
                props.cards.iter().map(|card| {
                    let url = card.art();

                    html! {
                        <img src={url} />
                    }
                }).take(3).collect::<Html>()
            }
        </div>
    }
}
