use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct Props {
    pub for_prop: AttrValue,
    pub text: AttrValue,
}

#[function_component(Label)]
pub fn label(props: &Props) -> Html {
    html! {
        <label for={props.for_prop.clone()}>{format!("{}:", props.text.clone())}</label>
    }
}
