use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ButtonComponentProps {
    #[prop_or_default]
    pub text_color: String,
    #[prop_or_default]
    pub bg_color: String,
    #[prop_or_default]
    pub border_color: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub handle_click: Callback<()>,
}

#[styled_component(ButtonComponent)]
pub fn button_component(props: &ButtonComponentProps) -> Html {
    html! {
        <div
            style={format!(
                "
                    --c-text-color: {text_color};
                    --c-bg-color: {bg_color};
                    --c-border-color: {border_color};
                    {style}
                ",
                text_color = props.text_color,
                bg_color = props.bg_color,
                border_color = props.border_color,
                style = props.style,
            )}
            class={classes!(
                css!("
                    display: inline-block;
                    border-radius: .4rem;
                    padding: .8rem 1.6rem;
                    background-color: var(--c-bg-color);
                    color: var(--c-text-color);
                    border: .2rem solid var(--c-border-color);
                    cursor: pointer;

                    &:hover {
                        background-color: color-mix(in hsl, var(--c-bg-color), #0000001A);
                        border-color: color-mix(in hsl, var(--c-border-color), #0000001A);
                    }
                    &:active {
                        background-color: color-mix(in hsl, var(--c-bg-color), #00000033);
                        border-color: color-mix(in hsl, var(--c-border-color), #00000033);
                    }
                "),
                props.class.clone(),
            )}
            onclick={
                let handle_click = props.handle_click.to_owned();
                move |_| {
                    handle_click.emit(());
                }
            }
        >
            {props.children.clone()}
        </div>
    }
}
