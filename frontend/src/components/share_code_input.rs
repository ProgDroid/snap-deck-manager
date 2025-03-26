use crate::{
    api::cards::get_from_share_code,
    components::{button::Button, label::Label, submit::Submit, textbox::Textbox},
};
use common::card::Card;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub submit_cards: Callback<Vec<Card>>,
}

#[function_component(ShareCodeInput)]
pub fn share_code_input(props: &Props) -> Html {
    let input_value = use_state(String::default);
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value());
            }
        })
    };

    let cards = use_state(Vec::<Card>::default);
    {
        let cards = cards.clone();
        let input_value = input_value.clone();

        use_effect_with(input_value, move |value| {
            let value = value.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(fetched_cards) = get_from_share_code((*value).clone()).await {
                    cards.set(fetched_cards);
                } else {
                    // TODO log e
                }
            });
            || ()
        });
    }

    let disabled = !(*input_value).is_empty() && (*cards).is_empty();

    html! {
        <>
            <Label for_prop={"share-code-input"} text={"Share Code"} />
            <Textbox id={"share-code-input"} value={(*input_value).clone()} name={"share-code-input"} on_input={on_input} />

            <Button<Submit>
                disabled={disabled}
                on_click={
                    let submit_cards = props.submit_cards.clone();

                    move |_| submit_cards.emit((*cards).clone())
                }
                value={Submit::Submit}
                selected=false
                class={"primary"}
            />
        </>
    }
}
