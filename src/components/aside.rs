use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct AsideComponentProps {
    pub image: String,
    pub height: String,
}

#[styled_component(AsideComponent)]
pub fn aside_component(props: &AsideComponentProps) -> Html {
    html! {
        <aside
            style={format!("--aside-height: {}", props.height)}
            class={css!("
                position: absolute;
                width: calc(.85 * var(--aside-height));
                height: var(--aside-height);
                overflow: hidden;
                z-index: 9999;
                mask-repeat: no-repeat;
                mask-position: center left, center right;
                mask-size: calc(100% - .1768 * var(--aside-height)) 100%, contain;
                mask-image:
                    url(/public/imgs/bg/aside_rect.svg),
                    url(/public/imgs/bg/aside_mask.svg);
            ")}>
            <img
                class={css!("
                    filter: saturate(130%);
                    height: 100%;
                ")}
                src={props.image.clone()} />
            <div
                class={css!("
                    --rcolor-start: hsl(from var(--brand-color) h calc(s + 15) calc(l + 15));
                    --rcolor-end: hsl(from var(--brand-color) h calc(s - 2) calc(l - 3));
                    position: absolute;
                    right: 0;
                    top: 0;
                    height: 100%;
                    aspect-ratio: 20.2 / 100;
                    background-image: linear-gradient(to bottom, var(--rcolor-start), var(--rcolor-end));
                    mask-image: url(/public/imgs/bg/aside_border.svg);
                ")}
                src="/public/imgs/bg/aside_border.svg">
            </div>
        </aside>
    }
}
