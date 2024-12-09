use crate::{
    components::{logo::Logo, nav_link::NavLink},
    route::Route,
};

use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div class="logo-container">
                <Logo />
            </div>
            <div class="nav-links-container">
                <NavLink label={"Decks"} route={Route::Decks} />
                <NavLink label={"Packages"} route={Route::Packages} />
            </div>
        </header>
    }
}
