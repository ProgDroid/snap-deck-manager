use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: AttrValue,
    pub value: AttrValue,
    pub name: AttrValue,
    pub on_input: Callback<InputEvent>,
}

#[function_component(Textbox)]
pub fn textbox(props: &Props) -> Html {
    // TODO add placeholder

    html! {
        <input class="input" type="text" id={props.id.clone()} value={props.value.clone()} name={props.name.clone()} oninput={props.on_input.clone()}/>
    }
}
