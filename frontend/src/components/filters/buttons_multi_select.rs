use strum::IntoEnumIterator;

use crate::components::{button::Button as FilterButton, filters::clear::Clear, label::Label};

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + Clone,
{
    pub label: AttrValue,
    pub select: Callback<T>,
    pub deselect: Callback<T>,
    pub clear: Callback<Clear>,
    pub selected: Vec<T>,
}

#[function_component(FilterButtonsMultiSelect)]
pub fn filter_buttons_multi_select<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + IntoEnumIterator + ToString + 'static,
{
    html! {
        <div class={""}>
            <Label for_prop={""} text={props.label.clone()} />
            <FilterButton<Clear>
                selected=false
                on_click={props.clear.clone()}
                value={Clear::Clear}
                class={"ghost"}
            />
            {
                T::iter().map(|value| {
                    let selected = props.selected.contains(&value);

                    let on_click = if selected {
                        props.deselect.clone()
                    } else {
                        props.select.clone()
                    };

                    html! {
                        <FilterButton<T>
                            selected={selected}
                            on_click={on_click}
                            value={value}
                            class={"ghost"}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
