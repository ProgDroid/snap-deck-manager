use common::card::Card;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct Props {
    pub cards: Vec<Card>,
}

#[function_component(CardPreview)]
pub fn card_preview(props: &Props) -> Html {
    html! {
        <div class="cards-preview">
            {
                props.cards.iter().enumerate().map(|(i, card)| {
                    let class = format!("cards-preview-{i}");
                    let url = card.art();

                    html! {
                        <div class={class}>
                            <img src={url} />
                        </div>
                    }
                }).take(3).collect::<Html>()
            }
        </div>
    }
}
