use super::menu_list_item_context::MenuListItemData;
use stylist::yew::styled_component;
use yew::prelude::*;

type ItemType = serde_json::Value;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MenuListItemComponentProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub handle_click: Callback<ItemType>,
}

#[styled_component(MenuListItemComponent)]
pub fn menu_list_item_component(props: &MenuListItemComponentProps) -> Html {
    let ctx = use_context::<MenuListItemData>().expect("Cannot get list item data");
    let item = ctx.item;

    html! {
        <div
            class={classes!(
                "item",
                css!("
                    background-color: var(--c-bg-norm-color);
                    &:hover {
                        background-color: var(--c-bg-hover-color);
                    }
                    &:active, &.active {
                        background-color: var(--c-bg-active-color);
                    }
                "),
            )}
            onclick={
                let handle_click = props.handle_click.to_owned();
                move |_| {
                    handle_click.emit(item.clone());
                }
            }
        >
            {props.children.clone()}
        </div>
    }
}
