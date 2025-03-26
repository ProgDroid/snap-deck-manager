use crate::{
    api::packages::get as get_package,
    components::{
        cards::{grid::CardGrid, grid_element::Display},
        with_route::WithRoute,
    },
    route::Route,
};
use common::card::Card;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub package_id: String,
}

#[function_component(PackageView)]
pub fn package_view(props: &Props) -> Html {
    let loading = use_state(|| true);

    let package_id = props.package_id.clone();
    let package = use_state(|| None);
    {
        let package = package.clone();
        let package_id = package_id.clone();
        let loading = loading.clone();

        use_effect_with((), move |()| {
            let package = package.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_package) = get_package(&package_id).await {
                    package.set(Some(fetched_package));
                } else {
                    // TODO log e
                }

                loading.set(false);
            });
            || ()
        });
    }

    let route = Route::PackageEdit { package_id };

    if let Some(package) = &*package {
        let mut cards = package.cards.clone();
        cards.sort_unstable_by_key(|card| (card.cost, card.power, card.name.to_lowercase()));

        return html! {
            <>
                <h1 class="text-3xl font-bold" >{package.name.clone()}</h1>
                // <h2 class="text-xl font-semibold">{"Share Code"}</h2> // TODO package share code?
                // <p class="break-all text-wrap">{share_code}</p>
                <h2 class="text-xl font-semibold">{"Cards"}</h2>
                <CardGrid cards={cards} excluded_cards={Vec::<Card>::default()} display={Display::Simple} on_click={Callback::from(|_| {})}/>
                <div class="divider"></div>
                <WithRoute route={route}>
                    <div class="btn btn-primary">
                        <button>{"Edit"}</button>
                    </div>
                </WithRoute>
            </>
        };
    }

    return html! { <div>{"Loading..."}</div> };
}
