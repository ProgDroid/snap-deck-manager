use crate::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
    pub route: Route,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(WithRoute)]
pub fn with_route(props: &Props) -> Html {
    let navigator = use_navigator().unwrap(); // TODO fix
    let current_route: Route = use_route().unwrap();
    let selected = current_route == props.route;
    let class = props.class.clone().unwrap_or_default();

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
