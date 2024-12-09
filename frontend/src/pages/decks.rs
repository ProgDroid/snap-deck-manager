use crate::{api::decks::list as get_decks, components::deck::list_element::DeckListElement};

use yew::prelude::*;

#[function_component(Decks)]
pub fn decks() -> Html {
    let decks = use_state(Vec::default);
    {
        let decks = decks.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_decks) = get_decks().await {
                    decks.set(fetched_decks);
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    return html! {
        <>
            <div class="decks-container">
                {
                    (*decks).iter().map(|deck| {
                        html! {
                            <DeckListElement id={deck.id.clone().unwrap()} name={deck.name.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    };
}
