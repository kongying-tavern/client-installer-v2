use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct AsideComponentProps {
    pub color: String,
    pub image: String,
    pub height: String,
}

#[styled_component(AsideComponent)]
pub fn aside_component(props: &AsideComponentProps) -> Html {
    html! {
        <aside
            style={format!(
                "
                    --c-aside-color: {color};
                    --c-aside-height: {height};
                ",
                color = props.color,
                height = props.height,
            )}
            class={css!("
                --c-aside-box-height: 404;
                --c-aside-outer-left: -441;
                --c-aside-outer-left-rel: calc((var(--c-aside-outer-left) / var(--c-aside-box-height)) * 100cqh);
                --c-aside-outer-top: -165;
                --c-aside-outer-top-rel: calc((var(--c-aside-outer-top) / var(--c-aside-box-height)) * 100cqh);
                --c-aside-outer-d: 786;
                --c-aside-outer-d-rel: calc((var(--c-aside-outer-d) / var(--c-aside-box-height)) * 100cqh);
                --c-aside-outer-width: calc(var(--c-aside-outer-left) + var(--c-aside-outer-d));
                --c-aside-inner-left: -430;
                --c-aside-inner-left-rel: calc((var(--c-aside-inner-left) / var(--c-aside-box-height)) * 100cqh);
                --c-aside-inner-top: -175;
                --c-aside-inner-top-rel: calc((var(--c-aside-inner-top) / var(--c-aside-box-height)) * 100cqh);
                --c-aside-inner-d: 755;
                --c-aside-inner-d-rel: calc((var(--c-aside-inner-d) / var(--c-aside-box-height)) * 100cqh);
                --c-aside-inner-width: calc(var(--c-aside-inner-left) + var(--c-aside-inner-d));
                --c-aside-color-start: hsl(from var(--c-aside-color) h calc(s * 1.25) calc(l * 1.4));
                --c-aside-color-end: var(--c-aside-color);

                position: absolute;
                top: 0;
                left: 0;
                z-index: 9999;
                height: var(--c-aside-height);
                aspect-ratio: var(--c-aside-outer-width) / var(--c-aside-box-height);
                overflow: hidden;
                container-type: size;
                filter: saturate(125%);

                &::before {
                    content: '';
                    display: block;
                    position: absolute;
                    overflow: hidden;
                    left: var(--c-aside-outer-left-rel);
                    top: var(--c-aside-outer-top-rel);
                    height: var(--c-aside-outer-d-rel);
                    aspect-ratio: 1;
                    border-radius: 50%;
                    z-index: 50;
                    background-image: linear-gradient(to bottom, var(--c-aside-color-start), var(--c-aside-color-end));
                }
            ")}>
            <img
                class={css!("
                    --c-aside-image-cr: calc(var(--c-aside-inner-d-rel) / 2);
                    --c-aside-image-cx: calc(var(--c-aside-inner-left-rel) + var(--c-aside-image-cr));
                    --c-aside-image-cy: calc(var(--c-aside-inner-top-rel) + var(--c-aside-image-cr));
                    position: absolute;
                    left: 0;
                    top: 0;
                    height: 100%;
                    width: auto;
                    z-index: 100;
                    clip-path: circle(var(--c-aside-image-cr) at var(--c-aside-image-cx) var(--c-aside-image-cy));
                ")}
                src={props.image.clone()}
            />
        </aside>
    }
}
