use crate::locale::i18n::Language;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalState {
    pub language: Language,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalStateAction {
    SetLanguage(Language),
}

impl Reducible for GlobalState {
    type Action = GlobalStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        GlobalState {
            language: match action {
                GlobalStateAction::SetLanguage(lang) => lang,
            },
        }
        .into()
    }
}

pub type GlobalStateContext = UseReducerHandle<GlobalState>;

#[derive(Debug, Properties, PartialEq)]
pub struct GlobalStateProviderProps {
    pub language: Language,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn GlobalStateProvider(props: &GlobalStateProviderProps) -> Html {
    let ctx = use_reducer(|| GlobalState {
        language: props.language,
    });

    html! {
        <ContextProvider<GlobalStateContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<GlobalStateContext>>
    }
}
