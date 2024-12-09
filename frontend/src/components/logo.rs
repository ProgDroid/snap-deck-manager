use yew::prelude::*;

use crate::{components::clickable::Clickable, route::Route};

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <Clickable route={Route::HomePage}>
            <img
                class="app-logo"
                src="https://images.rawpixel.com/image_png_800/cHJpdmF0ZS9sci9pbWFnZXMvd2Vic2l0ZS8yMDIyLTA1L2pvYjcyNC0xODMtcC5wbmc.png"
            />
        </Clickable>
    }
}
