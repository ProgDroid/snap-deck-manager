use crate::{
    components::{footer::Footer, header::Header},
    pages::{
        cards::CardsView,
        decks::{create::DeckCreate, edit::DeckEdit, list::Decks, view::DeckView},
        home::HomePage,
        packages::{create::PackageCreate, edit::PackageEdit, list::Packages, view::PackageView},
    },
    route::Route,
};

use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: Route) -> Html {
    html! {
        <>
            <Header />
            <main>
            {
                match route {
                    Route::HomePage => html! {
                        <HomePage />
                    },
                    Route::DeckView { deck_id } => html! {
                        <DeckView deck_id={deck_id} />
                    },
                    Route::DeckCreate => html! {
                        <DeckCreate />
                    },
                    Route::CardsView => html! {
                        <CardsView />
                    },
                    Route::Decks => html! {
                        <Decks />
                    },
                    Route::DeckEdit { deck_id } => html! {
                        <DeckEdit deck_id={deck_id} />
                    },
                    Route::Packages => html! {
                        <Packages />
                    },
                    Route::PackageView { package_id } => html! {
                        <PackageView package_id={package_id} />
                    },
                    Route::PackageEdit { package_id } => html! {
                        <PackageEdit package_id={package_id} />
                    },
                    Route::PackageCreate => html! {
                        <PackageCreate />
                    },
                }
            }
            </main>
            <Footer />
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
