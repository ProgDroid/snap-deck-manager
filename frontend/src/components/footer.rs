use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer fixed bottom-0 sm:footer-horizontal footer-center bg-base-300 text-base-content p-4">
            <aside>
                <p>{"Marvel Snap Deck Manager"}</p>
            </aside>
        </footer>
    }
}
