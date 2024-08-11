use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component(AsideComponent)]
pub fn aside_component() -> Html {
    html! {
        <aside class={css!("
            position: absolute;
            height: 100vh;
            width: 85vh;
            overflow: hidden;
            z-index: 9999;
            mask-repeat: no-repeat;
            mask-position: center left, center right;
            mask-size: calc(100% - 17.68vh) 100%, contain;
            mask-image:
                url(/public/imgs/bg/aside_rect.svg),
                url(/public/imgs/bg/aside_mask.svg);
        ")}>
            <img
                class={css!("
                    filter: saturate(130%);
                    height: 100%;
                ")}
                src="/public/imgs/bg/aside.jpg" />
            <div
                class={css!("
                    --rcolor-start: hsl(from var(--brand-color) h calc(s + 15) calc(l + 15));
                    --rcolor-end: hsl(from var(--brand-color) h calc(s - 2) calc(l - 3));
                    position: absolute;
                    right: 0;
                    top: 0;
                    width: 20.2vh;
                    height: 100vh;
                    background-image: linear-gradient(to bottom, var(--rcolor-start), var(--rcolor-end));
                    mask-image: url(/public/imgs/bg/aside_border.svg);
                ")}
                src="/public/imgs/bg/aside_border.svg">
            </div>
        </aside>
    }
}
