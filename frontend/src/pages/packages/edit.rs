use crate::{api::packages::get as get_package, components::edit_form::EditForm};

use common::package::Package;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub package_id: String,
}

#[function_component(PackageEdit)]
pub fn package_edit(props: &Props) -> Html {
    let package_id = props.package_id.clone();

    let package = use_state(|| None);
    {
        let package = package.clone();

        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_package) = get_package(&package_id).await {
                    package.set(Some(fetched_package));
                } else {
                    // TODO log e
                }
            });
        });
    }

    if (*package).is_none() {
        return html! {};
    }

    return html! {
        <EditForm<Package> given_object={(*package).clone()} />
    };
}
