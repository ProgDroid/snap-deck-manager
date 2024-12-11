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
    let class = format!(
        "card-grid-container{}",
        if props.excluded { " excluded" } else { "" }
    );

    let ability = Html::from_html_unchecked(props.card.description.clone().into());

    return html! {
        <div class={class} key={props.card.id.clone()} onclick={
            let on_click = props.on_click.clone();
            let card = props.card.clone();

            move |_| on_click.emit(vec![card.clone()])
        }>
            <div key={props.card.id.clone()} class="card-container">
                <img src={props.card.art()} />
                <h3>{props.card.name.clone()}</h3>
                if props.display == Display::Detailed {
                    <p>{ability}</p>
                    if !props.card.abilities.is_empty() {
                        <span>{
                            props.card.abilities.clone().into_iter().map(|ability| html! { <span class="pill info">{ability}</span> }).collect::<Vec<Html>>()
                        }</span>
                    }
                }
            </div>
        </div>
    };
}
