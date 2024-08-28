use super::menu_list_item::MenuListItemComponent;
use super::menu_list_item_context::MenuListItemProvider;
use stylist::yew::styled_component;
use yew::prelude::*;

type ItemType = serde_json::Value;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MenuListComponentProps<T>
where
    T: Clone + PartialEq,
{
    #[prop_or("#000000".to_string())]
    pub title_primary_color: String,
    #[prop_or("#00000080".to_string())]
    pub title_sub_color: String,
    #[prop_or("var(--brand-color)".to_string())]
    pub icon_color: String,
    #[prop_or("transparent".to_string())]
    pub bg_norm_color: String,
    #[prop_or("#0000001A".to_string())]
    pub bg_hover_color: String,
    #[prop_or("#00000033".to_string())]
    pub bg_active_color: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub list: Vec<T>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub handle_item_click: Callback<ItemType>,
}

#[styled_component(MenuListComponent)]
pub fn menu_list_component<T>(props: &MenuListComponentProps<T>) -> Html
where
    T: Clone + PartialEq + serde::Serialize,
{
    let json_list = props
        .list
        .iter()
        .map(|v| serde_json::json!(v))
        .collect::<Vec<ItemType>>();

    html! {
        <div
            style={format!(
                "
                    --c-title-primary-color: {title_primary_color};
                    --c-title-sub-color: {title_sub_color};
                    --c-icon-color: {icon_color};
                    --c-bg-norm-color: {bg_norm_color};
                    --c-bg-hover-color: {bg_hover_color};
                    --c-bg-active-color: {bg_active_color};
                    {style}
                ",
                title_primary_color = props.title_primary_color,
                title_sub_color = props.title_sub_color,
                icon_color = props.icon_color,
                bg_norm_color = props.bg_norm_color,
                bg_hover_color = props.bg_hover_color,
                bg_active_color = props.bg_active_color,
                style = props.style,
            )}
            class={classes!(
                css!("
                    display: flex;
                    flex-direction: column;
                "),
                props.class.clone(),
            )}
        >
            {
                json_list
                    .iter()
                    .map(|item| {
                        html! {
                            <MenuListItemProvider item={item.clone()}>
                                <MenuListItemComponent handle_click={props.handle_item_click.clone()}>
                                    {props.children.clone()}
                                </MenuListItemComponent>
                            </MenuListItemProvider>
                        }
                    })
                    .collect::<Html>()
            }
        </div>
    }
}
