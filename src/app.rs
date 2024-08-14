use client_installer_ui::components::{aside::AsideComponent, titlebar::TitlebarComponent};
use client_installer_ui::pages::router::RouterView;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <TitlebarComponent />
            <AsideComponent
                color="var(--brand-color)"
                image="/public/imgs/aside.jpg"
                height="100vh" />
            <RouterView />
        </main>
    }
}
