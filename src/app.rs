use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <aside class="aside">
                <img class="image" src="/public/imgs/bg/aside.jpg" />
                <div class="border" src="/public/imgs/bg/aside_border.svg"></div>
            </aside>
        </main>
    }
}
