use crate::components::{
    button::ButtonComponent, logo::LogoComponent, menu_list::MenuListComponent,
    menu_list_item_context::MenuListItemData, separator::SeparatorComponent,
};
use crate::languages::lang::language_item;
use crate::pages::router::AppRoute;
use crate::shared::global_state::{GlobalStateAction, GlobalStateContext};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

type OptionData = language_item::Lang;

#[styled_component(OptionItem)]
fn _option_item() -> Html {
    let global_state = use_context::<GlobalStateContext>().expect("Global state not found");
    let ctx = use_context::<MenuListItemData>().expect("Cannot get list item data");
    let opt: OptionData = serde_json::from_value(ctx.item).unwrap();
    let is_active = global_state.state.language == opt.lang;

    html! {
        <div class={classes!(
            opt.font_class.clone(),
            css!("
                display: flex;
                flex-direction: row;
                gap: .8rem;
                align-items: flex-start;
                padding: 1rem 0;
                padding-left: var(--p-padding-left);
                padding-right: var(--p-padding-right);
                cursor: pointer;
            "),
            {
                match is_active {
                    true => classes!(
                        "font-bold",
                        css!("background-color: var(--c-bg-active-color);"),
                    ),
                    false => classes!(),
                }
            }
        )}>
            <span class={css!("
                font-size: 1.6rem;
                line-height: 2.2rem;
                color: var(--p-item-title-color);
            ")}>
                {opt.name}
            </span>
            <span class={{
                match is_active {
                    true => css!("
                        display: inline-block;
                        width: 2rem;
                        height: 2rem;
                        background-color: var(--p-brand-color);
                        mask-image: url(/public/imgs/tick.svg);
                        mask-position: center;
                        mask-size: contain;
                        mask-repeat: no-repeat;
                    "),
                    false => css!(),
                }
            }} />
        </div>
    }
}

#[styled_component(LanguagePage)]
pub fn language_page() -> Html {
    let global_state = use_context::<GlobalStateContext>().expect("Global state not found");
    let p = global_state.computed.profile.to_owned();
    let t = global_state.computed.translation.to_owned();
    let langs = global_state.constant.langs.lang.to_owned();
    let navigator = use_navigator().unwrap();

    let css_margin_left = css!("margin-left: var(--p-padding-left);");
    let css_padding_left = css!("padding-left: var(--p-padding-left);");
    let css_padding_both = css!(
        "
            padding-left: var(--p-padding-left);
            padding-right: var(--p-padding-right);
        "
    );

    html! {
        <div class={css!("
            --p-brand-color: var(--brand-color);
            --p-padding-left: calc(13.86vh + 6.2rem);
            --p-padding-right: 5rem;
            --p-separator-color: var(--separator-color);
            --p-heading-color: var(--title-color);
            --p-item-title-color: var(--item-title-color);
            --p-item-desc-color: var(--item-desc-color);
            --p-button-plain-text-color: var(--button-plain-text-color);
            --p-button-plain-bg-color: var(--button-plain-bg-color);
            --p-button-plain-border-color: var(--button-plain-border-color);
            position: absolute;
            inset: 0;
            display: flex;
            flex-direction: column;
            gap: .5rem;
        ")}>
            <div class={classes!(
                css_padding_left.clone(),
                css!("
                    display: flex;
                    min-height: 9rem;
                    max-height: 9rem;
                    height: 9rem;
                    flex-direction: column;
                    justify-content: flex-end;
                "),
            )}>
                <LogoComponent
                    height="4rem"
                    text_color="var(--p-heading-color)"
                    icon_color="var(--p-brand-color)"
                    svg={p.logo.clone()}
                />
            </div>
            <SeparatorComponent
                color="var(--p-separator-color)"
                class={css_margin_left.clone()}
            />
            <div class={classes!(
                p.theme.font_class.clone(),
                css_padding_both.clone(),
                css!("
                    font-size: 1.6rem;
                    line-height: 2.6rem;
                    color: var(--p-heading-color);
                "),
            )}>
                {t.language.page_title.clone()}
            </div>
            <MenuListComponent<OptionData>
                class={classes!(
                    "styled-scrollbar",
                    css!("
                        flex: auto;
                        justify-content: flex-start;
                        margin: 1rem 0;
                        overflow-x: hidden;
                        overflow-y: auto;
                    "),
                )}
                list={langs}
                handle_item_click={move |opt: serde_json::Value| {
                    let data :OptionData = serde_json::from_value(opt).expect("Cannot parse item data");
                    global_state.state.dispatch(GlobalStateAction::SetLanguage(data.lang.clone()));
                }}
            >
                <OptionItem />
            </MenuListComponent<OptionData>>
            <div class={classes!(
                css_padding_both.clone(),
                css!("
                    flex: none;
                    padding-top: .8rem;
                    height: 10rem;
                "),
            )}>
                <ButtonComponent
                    text_color="var(--p-button-plain-text-color)"
                    bg_color="var(--p-button-plain-bg-color)"
                    border_color="var(--p-button-plain-border-color)"
                    class={classes!(
                        p.theme.font_class.clone(),
                        "font-bold",
                        css!("
                            font-size: 1.4rem;
                            line-height: 1.8rem;
                        "),
                    )}
                    handle_click={move |_| navigator.push(&AppRoute::Menu)}
                >
                    {t.language.button_next.clone()}
                </ButtonComponent>
            </div>
        </div>
    }
}
