use crate::{
    api::packages::list as get_packages,
    components::{card_preview::CardPreview, with_route::WithRoute},
    route::Route,
};

use yew::prelude::*;

#[function_component(Packages)]
pub fn packages() -> Html {
    let loading = use_state(|| true);

    let packages = use_state(Vec::default);
    {
        let packages = packages.clone();
        let loading = loading.clone();

        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_packages) = get_packages().await {
                    packages.set(fetched_packages);
                } else {
                    // TODO log e
                }

                loading.set(false);
            });
            || ()
        });
    }

    // TODO refactor into more generic components
    return html! {
        <div class="content-center h-full">
            <div class="flex flex-row justify-center items-center flex-wrap gap-4">
            {
                if *loading {
                    (0..3).map(|_| {
                        html! {
                            <div class="skeleton w-72 lg:max-w-72 max-w-[80vw] min-h-42"></div>
                        }
                    }).collect::<Html>()
                } else if packages.len() > 0 {
                    (*packages).iter().map(|package| {
                        html! {
                            <div class="btn w-72 lg:max-w-72 max-w-[80vw] h-full">
                                <WithRoute route={Route::PackageView { package_id: package.id.clone().unwrap() }}>
                                    <h2 class="text-lg font-semibold">{package.name.clone()}</h2>
                                    <p>{format!("{} cards", package.cards.len())}</p>
                                    <CardPreview cards={package.cards.clone()} />
                                </WithRoute>
                            </div>
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        // TODO button to create your first one
                        <p>{"No packages found"}</p>
                    }
                }
            }
            </div>
        </div>
    };
}
