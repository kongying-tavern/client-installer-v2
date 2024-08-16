use stylist::yew::styled_component;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LogoComponentProps {
    pub text_color: String,
    pub icon_color: String,
    pub height: String,
    pub svg: String,
}

#[styled_component(LogoComponent)]
pub fn logo_component(props: &LogoComponentProps) -> Html {
    html! {
        <div
            style={format!(
                "
                    --c-logo-text-color: {text_color};
                    --c-logo-icon-color: {icon_color};
                    --c-logo-height: {height};
                ",
                text_color = props.text_color,
                icon_color = props.icon_color,
                height = props.height,
            )}
            class={css!("
                position: relative;
                color: var(--c-logo-text-color);
                height: var(--c-logo-height);
                container-type: size;

                &::before {
                    content: '';
                    display: block;
                    position: absolute;
                    bottom: 0;
                    left: -62.5cqh;
                    height: 120cqh;
                    aspect-ratio: 1;
                    background-color: hsl(from var(--c-logo-icon-color) h s l / 20%);
                    mask-image: url(/public/imgs/star.svg);
                    mask-position: center;
                    mask-size: contain;
                    mask-repeat: no-repeat;
                }
            ")}>
            <div class={css!("
                position: absolute;
                inset: 0;
                z-index: 5;

                svg {
                    height: 100%;
                }
                .image {
                    display: inline-block;
                    height: 100%;
                    aspect-ratio: 21.616 / 40;
                    padding-left: .2rem;
                    background-color: var(--c-logo-text-color);
                    mask-image: url(/public/imgs/trademark.svg);
                    mask-position: center right;
                    mask-size: contain;
                    mask-repeat: no-repeat;
                }
            ")}>
                {VNode::from_html_unchecked(AttrValue::from(props.svg.clone()))}
                <div class="image" />
            </div>
        </div>
    }
}
