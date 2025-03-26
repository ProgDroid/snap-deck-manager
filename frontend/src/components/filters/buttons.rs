use strum::IntoEnumIterator;

use crate::components::{button::Button as FilterButton, label::Label};

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + Clone,
{
    pub label: AttrValue,
    pub select: Callback<T>,
    pub selected: T,
}

#[function_component(FilterButtons)]
pub fn filter_buttons<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + IntoEnumIterator + ToString + 'static,
{
    html! {
        <div class={""}>
            <Label for_prop={""} text={props.label.clone()} />
            {
                T::iter().map(|value| {
                    html! {
                        <FilterButton<T>
                            selected={value == props.selected}
                            on_click={props.select.clone()}
                            value={value}
                            class={"ghost"}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
