use yew::prelude::*;

use crate::{components::with_route::WithRoute, route::Route};

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Star,
    Deck,
    Cog,
    Package,
}

impl Type {
    fn link(&self) -> String {
        String::from(match self {
            Self::Star => "https://upload.wikimedia.org/wikipedia/commons/f/fd/A_star.png",
            Self::Deck => "https://images.rawpixel.com/image_png_800/cHJpdmF0ZS9sci9pbWFnZXMvd2Vic2l0ZS8yMDIyLTA1L2pvYjcyNC0xODMtcC5wbmc.png",
            Self::Cog => "https://upload.wikimedia.org/wikipedia/commons/4/42/Cogwheel.png",
            Self::Package => "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR640OE9MnaLEjJ3Y2pKRtnyyVuuqRJKzU-1Q&s",
        })
    }

    fn class(&self) -> String {
        String::from(match self {
            Self::Star => "star",
            Self::Deck => "deck",
            Self::Cog => "cog",
            Self::Package => "package",
        })
    }
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub icon_type: Type,
    pub label: AttrValue,
    pub route: Route,
}

#[function_component(IconLink)]
pub fn icon_link(props: &Props) -> Html {
    let class = format!(
        "icon-link join-item join join-vertical justify-center items-center w-100 lg:max-w-100 max-w-[80vw] {}",
        props.icon_type.class()
    );

    return html! {
        <>
            <div class={class}>
                <div class="btn btn-ghost px-0 h-full w-full items-center">
                    <WithRoute route={props.route.clone()}>
                        <img class="h-full w-full object-fill" src={props.icon_type.link()} />
                    </WithRoute>
                </div>
                <h2 class="text-lg font-semibold">{props.label.clone()}</h2>
            </div>
        </>
    };
}
