use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub id: String,
    pub name: String,
}

#[function_component(DeckListElement)]
pub fn deck_list_element(props: &Props) -> Html {
    let navigator = use_navigator().unwrap(); // TODO fix

    let route = Route::DeckView {
        deck_id: props.id.clone(),
    };

    let onclick = Callback::from(move |_| navigator.push(&route));

    return html! {
        <div {onclick}>
            <h2>{props.name.clone()} </h2>
        </div>
    };
}
