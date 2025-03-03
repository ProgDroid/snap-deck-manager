use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub visible: bool,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal-container")
        .unwrap_or_else(|| panic!("Expected to find a #modal-container element"));

    // TODO
    // modal_host.set_class_name(value);

    html! {
        <div class="modal-content">
        {
            if props.visible {
                create_portal(props.children.clone(), modal_host)
            } else {
                create_portal(html! { "" }, modal_host)
            }
        }
        </div>
    }
}
