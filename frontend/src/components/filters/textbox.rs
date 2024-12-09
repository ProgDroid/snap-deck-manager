use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Search,
}

impl Type {
    fn class(&self) -> String {
        format!(
            "filter-{}",
            match self {
                Self::Search => "search",
            }
        )
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub filter_type: Type,
    pub value: AttrValue,
    pub on_input: Callback<InputEvent>,
}

#[function_component(FilterTextbox)]
pub fn filter_textbox(props: &Props) -> Html {
    html! {
        <div class={props.filter_type.class()}>
            <label for="card-filter">{"Filter:"}</label>
            <input type="text" id="card-filter" value={props.value.clone()} name="card-filter" oninput={props.on_input.clone()}/>
        </div>
    }
}
