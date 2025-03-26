use crate::{
    api::decks::get as get_deck,
    components::{
        cards::{grid::CardGrid, grid_element::Display},
        pill::{Class, DefaultValue, Pill},
        with_route::WithRoute,
    },
    route::Route,
};
use common::card::Card;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub deck_id: String,
}

#[function_component(DeckView)]
pub fn deck_view(props: &Props) -> Html {
    let loading = use_state(|| true);

    let deck_id = props.deck_id.clone();
    let deck = use_state(|| None);
    {
        let deck = deck.clone();
        let deck_id = deck_id.clone();
        let loading = loading.clone();

        use_effect_with((), move |()| {
            let deck = deck.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_deck) = get_deck(&deck_id).await {
                    deck.set(Some(fetched_deck));
                } else {
                    // TODO log e
                }

                loading.set(false);
            });
            || ()
        });
    }

    let route = Route::DeckEdit { deck_id };

    if let Some(deck) = &*deck {
        let share_code = deck.share_code.clone();

        let mut cards = deck.cards.clone();
        cards.sort_unstable_by_key(|card| (card.cost, card.power, card.name.to_lowercase()));

        let on_click: Callback<DefaultValue> = Callback::default();

        // TODO fix formatting

        return html! {
            <>
                <h1 class="text-3xl font-bold" >{deck.name.clone()}</h1>
                <h2 class="text-xl font-semibold">{"Share Code"}</h2>
                <p class="break-all text-wrap">{share_code}</p>
                <h2 class="text-xl font-semibold">{"Cards"}</h2>
                <CardGrid cards={cards} excluded_cards={Vec::<Card>::default()} display={Display::Simple} on_click={Callback::from(|_| {})}/>
                <h2 class="text-xl font-semibold">{"Game Modes"}</h2>
                <div class="flex gap-4">
                {
                    deck.game_modes.iter().map(|game_mode| {
                        html! {
                            <Pill<DefaultValue> class={Class::Success} content={game_mode.name.clone()} on_click={on_click.clone()} value={DefaultValue::default()} />
                        }
                    }).collect::<Html>()
                }
                </div>
                <div class="divider"></div>
                <WithRoute route={route}>
                    <div class="btn btn-primary">
                        <button>{"Edit"}</button>
                    </div>
                </WithRoute>
            </>
        };
    }

    return if *loading {
        html! {
            <>
                <div class="skeleton w-32 h-7"></div>
                <h2 class="text-xl font-semibold">{"Share Code"}</h2>
                <div class="skeleton w-128 h-4"></div>
                <h2 class="text-xl font-semibold">{"Cards"}</h2>
                <div class="grid grid-cols-3 md:grid-cols-6 gap-4">
                {
                    (0..12).map(|_| {
                        html! {
                            <div class="skeleton w-40 h-56"></div>
                        }
                    }).collect::<Html>()
                }
                </div>
                <h2 class="text-xl font-semibold">{"Game Modes"}</h2>
                <div class="flex gap-4">
                {
                    (0..2).map(|_| {
                        html! {
                            <div class="skeleton w-32 h-6"></div>
                        }
                    }).collect::<Html>()
                }
                </div>
                <div class="divider"></div>
                <div>
                    <div class="btn btn-disabled" disabled={true}>
                        <button>{"Edit"}</button>
                    </div>
                </div>
            </>
        }
    } else {
        html! {"Could not load deck. Please try again later."} // TODO improve
    };
}
