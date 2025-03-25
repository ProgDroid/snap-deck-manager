use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::clickable::Clickable, route::Route};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub label: AttrValue,
    pub route: Route,
}

#[function_component(NavLink)]
pub fn nav_link(props: &Props) -> Html {
    let current_route: Route = use_route().unwrap();
    let selected = current_route == props.route;

    let class = format!(
        "btn btn-ghost text-xl normal-case nav-link{}",
        if selected {
            " btn-active btn-disabled"
        } else {
            ""
        }
    );

    return html! {
        <>
            <Clickable route={props.route.clone()}>
                <button class={class}>{props.label.clone()}</button>
            </Clickable>
        </>
    };
}
