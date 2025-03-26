use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer footer-center bg-base-300 text-base-content p-4">
            <aside>
                <p>{"Deck Manager"}</p>
            </aside>
        </footer>
    }
}
