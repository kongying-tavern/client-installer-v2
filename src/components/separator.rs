use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SeparatorComponentProps {
    pub color: String,
    #[prop_or("1.6rem".to_string())]
    pub width: String,
    #[prop_or(".2rem".to_string())]
    pub height: String,
}

#[styled_component(SeparatorComponent)]
pub fn separator_component(props: &SeparatorComponentProps) -> Html {
    html! {
        <div
            style={format!(
                "
                    --c-separator-color: {color};
                    --c-separator-width: {width};
                    --c-separator-height: {height};
                ",
                color = props.color,
                width = props.width,
                height = props.height,
            )}
            class={css!("
                width: var(--c-separator-width);
                height: var(--c-separator-height);
                background-color: var(--c-separator-color);
            ")}
        />
    }
}
