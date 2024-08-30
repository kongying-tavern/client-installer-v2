use client_installer_ui::components::{aside::AsideComponent, titlebar::TitlebarComponent};
use client_installer_ui::languages::lang::SupportedLanguages;
use client_installer_ui::pages::router::RouterView;
use client_installer_ui::shared::global_state::GlobalStateProvider;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <GlobalStateProvider language={SupportedLanguages::EnUs}>
            <main class="container">
                <TitlebarComponent />
                <AsideComponent
                    color="var(--brand-color)"
                    image="/public/imgs/aside.webp"
                    height="100vh" />
                <RouterView />
            </main>
        </GlobalStateProvider>
    }
}
