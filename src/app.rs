use yew::prelude::*;
use client_installer_ui::components::aside::AsideComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <AsideComponent />
        </main>
    }
}
