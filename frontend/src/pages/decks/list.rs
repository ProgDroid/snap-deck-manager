use crate::{
    api::decks::list as get_decks,
    components::{card_preview::CardPreview, with_route::WithRoute},
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

    // TODO refactor into more generic components
    return html! {
        <>
            <div class="content-center h-full">
                <div class="flex flex-row justify-center items-center flex-wrap gap-4">
                {
                    (*decks).iter().map(|deck| {
                        html! {
                            <>
                                <div class="btn w-72 lg:max-w-72 max-w-[80vw] h-full">
                                    <WithRoute route={Route::DeckView { deck_id: deck.id.clone().unwrap() }}>
                                        <h2 class="text-lg font-semibold">{deck.name.clone()}</h2>
                                        <p>{format!("{} cards", deck.cards.len())}</p>
                                        <CardPreview cards={deck.cards.clone()} />
                                    </WithRoute>
                                </div>
                            </>
                        }
                    }).collect::<Html>()
                }
                </div>
            </div>
        </>
    };
}
