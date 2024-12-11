use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + Clone + ToString + 'static,
{
    pub on_click: Callback<T>,
    pub value: T,
    pub selected: bool,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + ToString + 'static,
{
    let class = if props.selected { "selected" } else { "" };

    html! {
        <button disabled={props.disabled} class={class} onclick={
            let on_click = props.on_click.clone();
            let value = props.value.clone();

            move |_| on_click.emit(value.clone())
        }>{props.value.to_string()}</button>
    }
}
