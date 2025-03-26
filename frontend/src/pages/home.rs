use crate::{
    components::{
        button::Button,
        icon_link::{IconLink, Type},
        modal::Modal,
        submit::Submit,
        vertical_centre_container::VerticalCentreContainer,
    },
    route::Route,
};

use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    // let modal_visible = use_state(|| false);

    // let dismiss_modal: Callback<Submit> = {
    //     let modal_visible = modal_visible.clone();

    //     Callback::from(move |_: Submit| {
    //         modal_visible.set(false);
    //     })
    // };

    // let show_modal: Callback<Submit> = {
    //     let modal_visible = modal_visible.clone();

    //     Callback::from(move |_: Submit| {
    //         modal_visible.set(true);
    //     })
    // };

    return html! {
        <>
        // <Modal visible={*modal_visible}>
        //     <div class={"modal-header"}>
        //         <h2>{"Test Modal"}</h2>
        //     </div>
        //     <div class={"modal-body"}>
        //         <p>{"This is a test modal"}</p>
        //     </div>
        //     <div class={"modal-footer"}>
        //         <Button<Submit> on_click={dismiss_modal} value={Submit::Submit} selected={false} />
        //     </div>
        // </Modal>
        <div class="content-center h-full">
            <div class="flex flex-row justify-center items-center flex-wrap gap-4">
                <IconLink icon_type={Type::Star} label={"Decks"} route={Route::Decks} />
                <IconLink icon_type={Type::Deck} label={"New Deck"} route={Route::DeckCreate} />
                <IconLink icon_type={Type::Package} label={"Packages"} route={Route::Packages} />
                <IconLink icon_type={Type::Package} label={"New Package"} route={Route::PackageCreate} />
                <IconLink icon_type={Type::Cog} label={"Update Cards"} route={Route::UpdateCards} />
            </div>
        </div>
        // <VerticalCentreContainer inner_class="icon-container">
        //     <IconLink icon_type={Type::Star} label={"Decks"} route={Route::Decks} />
        //     <IconLink icon_type={Type::Deck} label={"New Deck"} route={Route::DeckCreate} />
        //     <IconLink icon_type={Type::Package} label={"Packages"} route={Route::Packages} />
        //     <IconLink icon_type={Type::Package} label={"New Package"} route={Route::PackageCreate} />
        //     <IconLink icon_type={Type::Cog} label={"Update Cards"} route={Route::UpdateCards} />
        // </VerticalCentreContainer>
        </>
    };
}
