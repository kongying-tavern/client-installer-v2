use stylist::yew::styled_component;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RawComponentProps {
    pub html: String,
}

#[styled_component(RawComponent)]
pub fn raw_component(props: &RawComponentProps) -> Html {
    VNode::from_html_unchecked(AttrValue::from(props.html.clone()))
}
