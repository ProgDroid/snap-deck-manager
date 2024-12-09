use crate::{api::packages::get as get_package, route::Route};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct ViewProps {
    pub package_id: String,
}

#[function_component(PackageView)]
pub fn package_view(props: &ViewProps) -> Html {
    let package_id = props.package_id.clone();
    let package = use_state(|| None);
    {
        let package = package.clone();
        let package_id = package_id.clone();

        use_effect_with((), move |()| {
            let package = package.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_package) = get_package(&package_id).await {
                    package.set(Some(fetched_package));
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    let navigator = use_navigator().unwrap(); // TODO fix

    let route = Route::PackageEdit { package_id };

    let onclick = Callback::from(move |_| navigator.push(&route));

    if let Some(package) = &*package {
        return html! {
            <>
                <table>
                    <tr>
                        <td>{"Package Name"}</td>
                        <td>{package.name.clone()}</td>
                    </tr>
                </table>
                <button {onclick}>{"Edit"}</button>
            </>
        };
    }

    return html! { <div>{"Loading..."}</div> };
}
