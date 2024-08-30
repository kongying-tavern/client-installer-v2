use crate::languages::lang::SupportedLanguages;

#[cfg(windows)]
pub fn get_app_language() -> SupportedLanguages {
    use super::locale::get_sys_language;

    let lang = get_sys_language();
    match lang.code.as_str() {
        "zh-Hans" => SupportedLanguages::ZhHans,
        "zh-CN" => SupportedLanguages::ZhHans,
        "zh-SG" => SupportedLanguages::ZhHans,
        "zh-MO" => SupportedLanguages::ZhHans,
        _ => match lang.primary.as_str() {
            "zh" => SupportedLanguages::ZhHans,
            _ => SupportedLanguages::EnUs,
        },
    }
}

#[cfg(not(windows))]
pub fn get_app_language() -> SupportedLanguages {
    unimplemented!()
}
