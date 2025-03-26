use strum::{Display, EnumIter};
use yew::prelude::*;

#[derive(Eq, PartialEq, Default, Clone, Display, EnumIter)]
pub enum DefaultValue {
    #[default]
    DefaultValue,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Class {
    Info,
    Error,
    Success,
    Ghost,
}

impl Class {
    pub fn to_class_string(&self) -> String {
        String::from(match self {
            Self::Info => "badge-info",
            Self::Error => "badge-error",
            Self::Success => "badge-success",
            Self::Ghost => "badge-ghost",
        })
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + Clone + ToString + 'static,
{
    pub class: Class,
    pub value: T,
    pub content: AttrValue,
    pub on_click: Callback<T>,
    #[prop_or_default]
    pub hover_interaction: bool,
}

#[function_component(Pill)]
pub fn pill<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + ToString + 'static,
{
    let class = format!(
        "font-semibold transition delay-50 duration-150 badge {}{}",
        props.class.to_class_string(),
        if props.hover_interaction {
            " select-none hover:cursor-pointer hover:scale-105"
        } else {
            ""
        }
    );

    html! {
        <div onclick={
            let on_click = props.on_click.clone();
            let value = props.value.clone();

            move |_| on_click.emit(value.clone())
        }>
            <span class={class.clone()}>{props.content.clone()}</span>
        </div>
    }
}
