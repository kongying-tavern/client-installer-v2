use yew::prelude::*;

type ItemType = serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MenuListItemData {
    pub item: ItemType,
}

#[derive(Debug, Properties, PartialEq)]
pub struct MenuListItemProviderProps {
    pub item: ItemType,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn MenuListItemProvider(props: &MenuListItemProviderProps) -> Html {
    let ctx = use_memo(props.item.clone(), |item| MenuListItemData {
        item: item.clone(),
    });

    html! {
        <ContextProvider<MenuListItemData> context={(*ctx).clone()}>
            {props.children.clone()}
        </ContextProvider<MenuListItemData>>
    }
}
