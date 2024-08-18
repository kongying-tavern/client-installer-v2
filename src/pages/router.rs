use super::menu::MenuPage;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Menu,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Menu => html! { <MenuPage /> },
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
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}
