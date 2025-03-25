use crate::{components::clickable::Clickable, route::Route};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    let current_route: Route = use_route().unwrap();
    let selected = current_route == Route::HomePage;

    let class = format!(
        "btn btn-ghost btn-circle avatar{}",
        if selected { " btn-disabled" } else { "" }
    );

    html! {
        <div class={class}>
            <div class="w-10 rounded-full">
                <Clickable route={Route::HomePage}>
                    <img
                        class="app-logo"
                        src="https://images.rawpixel.com/image_png_800/cHJpdmF0ZS9sci9pbWFnZXMvd2Vic2l0ZS8yMDIyLTA1L2pvYjcyNC0xODMtcC5wbmc.png"
                    />
                </Clickable>
            </div>
        </div>
    }
}
