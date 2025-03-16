use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub inner_class: AttrValue,
}

#[function_component(VerticalCentreContainer)]
pub fn vertical_centre_container(props: &Props) -> Html {
    let inner_class = format!("vertical-centre-inner carousel {}", props.inner_class);

    html! {
        <div class="vertical-centre-container">
            <div class={inner_class}>
                {props.children.clone()}
            </div>
        </div>
    }
}
