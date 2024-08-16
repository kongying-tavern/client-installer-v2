use crate::shared::app::invoke;
use stylist::yew::styled_component;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[styled_component(TitlebarComponent)]
pub fn titlebar_component() -> Html {
    let css_button = css!(
        "
            display: flex;
            width: 3.2rem;
            aspect-ratio: 1;
            cursor: pointer;
            border-radius: .4rem;
            justify-content: center;
            align-items: center;
            background-color: var(--c-titlebar-button-color);

            .icon {
                display: block;
                width: 1.4rem;
                aspect-ratio: 1;
                background-color: var(--c-titlebar-icon-color);
                mask-image: var(--c-titlebar-icon-image);
                mask-position: center;
                mask-size: contain;
                mask-repeat: no-repeat;
            }
        "
    );

    html! {
        <div
            data-tauri-drag-region="true"
            class={css!("
                position: absolute;
                display: flex;
                flex-direction: row-reverse;
                gap: 1rem;
                padding: 1rem;
                width: calc(100% - 2rem);
                z-index: 10000;
            ")}>
            <div
                class={classes!(
                    css_button.clone(),
                    css!("
                        --c-titlebar-icon-image: url(/public/imgs/close.svg);
                        --c-titlebar-button-color: transparent;
                        --c-titlebar-icon-color: #7B7B7B;
                        &:hover {
                            --c-titlebar-button-color: #D15C5C;
                            --c-titlebar-icon-color: #FFFFFF;
                        }
                        &:active {
                            --c-titlebar-button-color: transparent;
                            --c-titlebar-icon-color: #7B7B7B80;
                        }
                    "),
                )}
                onclick={move |_| {
                    spawn_local(async move {
                        invoke("close", JsValue::UNDEFINED).await;
                    })
                }}>
                <span class="icon" />
            </div>
            <div
                class={classes!(
                    css_button.clone(),
                    css!("
                        --c-titlebar-icon-image: url(/public/imgs/minimize.svg);
                        --c-titlebar-button-color: transparent;
                        --c-titlebar-icon-color: #7B7B7B;
                        &:hover {
                            --c-titlebar-button-color: #0000003D;
                            --c-titlebar-icon-color: #7B7B7B;
                        }
                        &:active {
                            --c-titlebar-button-color: transparent;
                            --c-titlebar-icon-color: #7B7B7B80;
                        }
                    "),
                )}
                onclick={move |_| {
                    spawn_local(async move {
                        invoke("minimize", JsValue::UNDEFINED).await;
                    })
                }}>
                <span class="icon" />
            </div>
        </div>
    }
}
