use crate::{
    components::{logo::Logo, nav_link::NavLink},
    route::Route,
};

use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="sticky top-0 z-50">
            <div class="navbar bg-primary text-primary-content">
                <div class="flex-1">
                    <Logo />
                </div>
                <div class="flex gap-2">
                    <NavLink label={"Decks"} route={Route::Decks} />
                    <NavLink label={"Packages"} route={Route::Packages} />
                </div>
            </div>
        </header>
    }
}
