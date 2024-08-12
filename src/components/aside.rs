use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AsideComponentProps {
    pub image: String,
    pub height: String,
}

#[styled_component(AsideComponent)]
pub fn aside_component(props: &AsideComponentProps) -> Html {
    html! {
        <aside
            style={format!("--aside-height: {height};", height = props.height)}
            class={css!("
                --aside-box-height: 404;
                --aside-outer-left: -441;
                --aside-outer-left-rel: calc((var(--aside-outer-left) / var(--aside-box-height)) * 100cqh);
                --aside-outer-top: -165;
                --aside-outer-top-rel: calc((var(--aside-outer-top) / var(--aside-box-height)) * 100cqh);
                --aside-outer-d: 786;
                --aside-outer-d-rel: calc((var(--aside-outer-d) / var(--aside-box-height)) * 100cqh);
                --aside-outer-width: calc(var(--aside-outer-left) + var(--aside-outer-d));
                --aside-inner-left: -430;
                --aside-inner-left-rel: calc((var(--aside-inner-left) / var(--aside-box-height)) * 100cqh);
                --aside-inner-top: -175;
                --aside-inner-top-rel: calc((var(--aside-inner-top) / var(--aside-box-height)) * 100cqh);
                --aside-inner-d: 755;
                --aside-inner-d-rel: calc((var(--aside-inner-d) / var(--aside-box-height)) * 100cqh);
                --aside-inner-width: calc(var(--aside-inner-left) + var(--aside-inner-d));
                --aside-color-start: hsl(from var(--brand-color) h calc(s * 1.25) calc(l * 1.4));
                --aside-color-end: var(--brand-color);

                position: absolute;
                top: 0;
                left: 0;
                z-index: 9999;
                height: var(--aside-height);
                aspect-ratio: var(--aside-outer-width) / var(--aside-box-height);
                overflow: hidden;
                container-type: size;
                filter: saturate(125%);

                &::before {
                    content: '';
                    display: block;
                    position: absolute;
                    overflow: hidden;
                    left: var(--aside-outer-left-rel);
                    top: var(--aside-outer-top-rel);
                    height: var(--aside-outer-d-rel);
                    aspect-ratio: 1;
                    border-radius: 50%;
                    z-index: 50;
                    background-image: linear-gradient(to bottom, var(--aside-color-start), var(--aside-color-end));
                }
            ")}>
            <img
                class={css!("
                    --aside-image-cr: calc(var(--aside-inner-d-rel) / 2);
                    --aside-image-cx: calc(var(--aside-inner-left-rel) + var(--aside-image-cr));
                    --aside-image-cy: calc(var(--aside-inner-top-rel) + var(--aside-image-cr));
                    position: absolute;
                    left: 0;
                    top: 0;
                    height: 100%;
                    width: auto;
                    z-index: 100;
                    clip-path: circle(var(--aside-image-cr) at var(--aside-image-cx) var(--aside-image-cy));
                ")}
                src={props.image.clone()} />
        </aside>
    }
}
