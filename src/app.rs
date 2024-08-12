use client_installer_ui::components::aside::AsideComponent;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <AsideComponent
                image="/public/imgs/aside.jpg"
                height="100vh" />
        </main>
    }
}
