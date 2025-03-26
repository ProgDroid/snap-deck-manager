use yew::prelude::*;

use crate::components::label::Label;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    pub legend: AttrValue,
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    html! {
        <fieldset class={"fieldset"}>
            <legend class="fieldset-legend">{props.legend.clone()}</legend>
            {props.children.clone()}
        </fieldset>
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
