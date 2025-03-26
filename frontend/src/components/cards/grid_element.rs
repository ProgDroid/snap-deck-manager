use crate::components::pill::{Class, DefaultValue, Pill};
use common::card::Card;
use yew::prelude::*;

// TODO this might have to be at card level? hard to tell where this should live
#[derive(Clone, PartialEq, Eq)]
pub enum Display {
    Detailed,
    Simple,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub card: Card,
    pub display: Display,
    pub on_click: Callback<Vec<Card>>,
    pub excluded: bool,
}

#[function_component(GridElement)]
pub fn grid_element(props: &Props) -> Html {
    let class = if props.excluded { " grayscale" } else { "" };

    let ability = Html::from_html_unchecked(props.card.description.clone().into());

    let on_click: Callback<DefaultValue> = Callback::default();

    return html! {
        <div class={class} key={props.card.id.clone()} onclick={
            let on_click = props.on_click.clone();
            let card = props.card.clone();

            move |_| on_click.emit(vec![card.clone()])
        }>
            <div key={props.card.id.clone()} class="join join-vertical items-center">
                <img class="join-item" src={props.card.art()} />
                <h3 class="join-item">{props.card.name.clone()}</h3>
                if props.display == Display::Detailed {
                    <p class="join-item">{ability}</p>
                    if !props.card.abilities.is_empty() {
                        <span class="card-abilities join-item">{
                            props.card.abilities.clone().into_iter().map(|ability| html! { <Pill<DefaultValue> class={Class::Info} content={ability} on_click={on_click.clone()} value={DefaultValue::default()} /> }).collect::<Vec<Html>>()
                        }</span>
                    }
                    { if props.card.released { html! {} } else { html! { <Pill<DefaultValue> class={Class::Error} content={"Unreleased"} on_click={on_click.clone()} value={DefaultValue::default()} /> } } }
                }
            </div>
        </div>
    };
}
