use super::menu::MenuPage;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq, Serialize, Deserialize)]
pub enum AppRoute {
    #[not_found]
    #[at("/")]
    Menu,
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Menu => html! { <MenuPage /> },
    }
}

#[styled_component(RouterView)]
pub fn router_view() -> Html {
    html! {
        <div class={css!("
            position: absolute;
            inset: 0;
            left: 66vh;
            z-index: 10;
        ")}>
            <BrowserRouter>
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </div>
    }
}
