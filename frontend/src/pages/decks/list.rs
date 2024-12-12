use crate::{
    api::decks::list as get_decks,
    components::{card_preview::CardPreview, clickable::Clickable},
    route::Route,
};

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
                            <>
                                <Clickable route={Route::DeckView { deck_id: deck.id.clone().unwrap() }}>
                                    <h2>{deck.name.clone()}</h2>
                                    <p>{format!("{} cards", deck.cards.len())}</p>
                                    <CardPreview cards={deck.cards.clone()} />
                                </Clickable>
                            </>
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    };
}
