use crate::languages::lang::SupportedLanguages;
use crate::languages::translations::{Translatable, Translation};
use std::rc::Rc;
use yew::prelude::*;

/** Global state reducer part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalState {
    pub language: SupportedLanguages,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalStateAction {
    SetLanguage(SupportedLanguages),
}

impl Reducible for GlobalState {
    type Action = GlobalStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            GlobalStateAction::SetLanguage(lang) => GlobalState { language: lang },
        }
        .into()
    }
}

pub type GlobalStateReducer = UseReducerHandle<GlobalState>;

/** Global state computed part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStateComputed {
    pub translation: Translation,
}

pub type GlobalStateMemo = Rc<GlobalStateComputed>;

/** Global state context part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStateContext {
    pub state: GlobalStateReducer,
    pub computed: GlobalStateMemo,
}

#[derive(Debug, Properties, PartialEq)]
pub struct GlobalStateProviderProps {
    pub language: SupportedLanguages,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn GlobalStateProvider(props: &GlobalStateProviderProps) -> Html {
    let state = use_reducer(|| GlobalState {
        language: props.language,
    });
    let computed = use_memo(state.language, |lang| GlobalStateComputed {
        translation: lang.to_translations().unwrap(),
    });
    let ctx = GlobalStateContext { state, computed };

    html! {
        <ContextProvider<GlobalStateContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<GlobalStateContext>>
    }
}
