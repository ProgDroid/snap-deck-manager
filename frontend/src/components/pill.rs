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
    Secondary,
}

impl Class {
    pub fn to_class_string(&self) -> String {
        String::from(match self {
            Self::Info => "badge-info",
            Self::Error => "badge-error",
            Self::Success => "badge-success",
            Self::Secondary => "badge-secondary",
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
}

#[function_component(Pill)]
pub fn pill<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + ToString + 'static,
{
    let class = format!("badge {}", props.class.to_class_string());

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
