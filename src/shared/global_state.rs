use crate::languages::lang::SupportedLanguages;
use crate::languages::translations::{Translatable, Translation};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalState {
    pub language: SupportedLanguages,
    pub translation: Translation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalStateAction {
    SetLanguage(SupportedLanguages),
}

impl Reducible for GlobalState {
    type Action = GlobalStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            GlobalStateAction::SetLanguage(lang) => GlobalState {
                language: lang,
                translation: lang.to_translations().unwrap(),
            },
        }
        .into()
    }
}

pub type GlobalStateContext = UseReducerHandle<GlobalState>;

#[derive(Debug, Properties, PartialEq)]
pub struct GlobalStateProviderProps {
    pub language: SupportedLanguages,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn GlobalStateProvider(props: &GlobalStateProviderProps) -> Html {
    let ctx = use_reducer(|| GlobalState {
        language: props.language,
        translation: props.language.to_translations().unwrap(),
    });

    html! {
        <ContextProvider<GlobalStateContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<GlobalStateContext>>
    }
}
