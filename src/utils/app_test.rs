#[cfg(test)]
mod tests {
    #[cfg(windows)]
    #[test]
    fn test_get_app_language() {
        use crate::utils::app::get_app_language;

        let lang = get_app_language();
        println!("App default lang is: {:#?}", lang);
    }
}
