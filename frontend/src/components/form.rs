use yew::prelude::*;

use crate::components::label::Label;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    pub class_prefix: AttrValue,
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    html! {
        <div class={format!("{}-form-container", props.class_prefix.clone())}>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps {
    pub children: Html,
    pub id: AttrValue,
    pub class: AttrValue,
    pub label: AttrValue,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
    html! {
        <div class={format!("form-field {}", props.class.clone())}>
            <Label for_prop={props.id.clone()} text={props.label.clone()} />
            {props.children.clone()}
        </div>
    }
}
