use crate::languages::lang::{Language, LanguageManifest, SupportedLanguages};
use crate::languages::profile::{AppProfile, Profile};
use crate::languages::translations::{Translatable, Translation};
use std::rc::Rc;
use yew::prelude::*;

/** Global state reducer part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStateData {
    pub language: SupportedLanguages,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalStateAction {
    SetLanguage(SupportedLanguages),
}

impl Reducible for GlobalStateData {
    type Action = GlobalStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            GlobalStateAction::SetLanguage(lang) => GlobalStateData { language: lang },
        }
        .into()
    }
}

pub type GlobalStateReducer = UseReducerHandle<GlobalStateData>;

/** Global state static part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStateStatic {
    pub langs: Language,
}

/** Global state computed part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStateComputed {
    pub profile: Profile,
    pub translation: Translation,
}

pub type GlobalStateMemo = Rc<GlobalStateComputed>;

/** Global state context part */
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStateContext {
    pub state: GlobalStateReducer,
    pub constant: GlobalStateStatic,
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
    let state = use_reducer(|| GlobalStateData {
        language: props.language,
    });
    let constant = GlobalStateStatic {
        langs: Language::to_lang_manifest().unwrap(),
    };
    let computed = use_memo(state.language, |lang| GlobalStateComputed {
        profile: lang.to_profile().unwrap(),
        translation: lang.to_translations().unwrap(),
    });
    let ctx = GlobalStateContext {
        state,
        constant,
        computed,
    };

    html! {
        <ContextProvider<GlobalStateContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<GlobalStateContext>>
    }
}
