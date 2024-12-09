use crate::{
    components::{
        icon_link::{IconLink, Type},
        vertical_centre_container::VerticalCentreContainer,
    },
    route::Route,
};

use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    return html! {
        <VerticalCentreContainer inner_class="icon-container">
            <IconLink icon_type={Type::Star} label={"Decks"} route={Route::Decks} />
            <IconLink icon_type={Type::Deck} label={"New Deck"} route={Route::DeckCreate} />
            <IconLink icon_type={Type::Package} label={"Packages"} route={Route::Packages} />
            <IconLink icon_type={Type::Package} label={"New Package"} route={Route::PackageCreate} />
            // <IconLink icon_type={Type::Cog} label={"Settings"} route={Route::CardsView} />
        </VerticalCentreContainer>
    };
}
