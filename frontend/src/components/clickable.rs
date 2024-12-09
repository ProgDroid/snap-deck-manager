use crate::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
    pub route: Route,
}

#[function_component(Clickable)]
pub fn clickable(props: &Props) -> Html {
    let navigator = use_navigator().unwrap(); // TODO fix
    let current_route: Route = use_route().unwrap();
    let selected = current_route == props.route;

    let class = format!("clickable{}", if selected { "" } else { " click-enabled" });

    let onclick: Callback<MouseEvent> = if selected {
        Callback::default()
    } else {
        let route = props.route.clone();

        Callback::from(move |_| navigator.push(&route))
    };

    return html! {
        <>
            <div class={class} {onclick}>
                {props.children.clone()}
            </div>
        </>
    };
}
