#[cfg(test)]
mod tests {
    #[cfg(windows)]
    #[test]
    fn test_get_sys_language() {
        use crate::utils::locale::get_sys_language;

        let lang_id = get_sys_language();
        println!("Language ID is: {:?}", lang_id);
    }
}
