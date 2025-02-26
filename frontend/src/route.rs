use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/deck/:deck_id")]
    DeckView { deck_id: String },
    #[at("/deck/create")]
    DeckCreate,
    #[at("/cards")]
    CardsView,
    #[at("/decks")]
    Decks,
    #[at("/deck/:deck_id/edit")]
    DeckEdit { deck_id: String },
    #[at("/packages")]
    Packages,
    #[at("/package/:package_id")]
    PackageView { package_id: String },
    #[at("/package/:package_id/edit")]
    PackageEdit { package_id: String },
    #[at("/package/create")]
    PackageCreate,
    #[at("/cards/update")]
    UpdateCards,
}
