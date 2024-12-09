use crate::{
    api::packages::list as get_packages, components::packages::list_element::PackageListElement,
};

use yew::prelude::*;

#[function_component(Packages)]
pub fn packages() -> Html {
    let packages = use_state(Vec::default);
    {
        let packages = packages.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_packages) = get_packages().await {
                    packages.set(fetched_packages);
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    return html! {
        <>
            <h1>{"Packages"}</h1>
            <div class="packages-container">
                {
                    (*packages).iter().map(|package| {
                        html! {
                            <PackageListElement id={package.id.clone().unwrap()} name={package.name.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    };
}
