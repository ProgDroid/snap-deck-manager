use crate::api::cards::update_all_cards;

use yew::prelude::*;

#[function_component(UpdateCards)]
pub fn update_cards() -> Html {
    let ok = use_state(|| false);
    {
        let ok = ok.clone();

        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(_) = update_all_cards().await {
                    ok.set(true);
                } else {
                    // TODO log e
                }
            });
        });
    }

    if *ok {
        html! { <p>{"All Done!"}</p> }
    } else {
        html! { <p>{"Waiting..."}</p>}
    }
}
