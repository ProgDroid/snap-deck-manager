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

    create_portal(
        if props.visible {
            modal_host.set_class_name("visible");

            html! {
                <div class="modal-content">
                {
                    props.children.clone()
                }
                </div>
            }
        } else {
            modal_host.set_class_name("");

            html! { "" }
        },
        modal_host,
    )
}
