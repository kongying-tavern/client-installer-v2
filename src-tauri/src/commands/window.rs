use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn minimize<R: tauri::Runtime>(app_handle: AppHandle<R>) {
    app_handle
        .get_webview_window("main")
        .unwrap()
        .minimize()
        .unwrap();
}

#[tauri::command]
pub fn close<R: tauri::Runtime>(app_handle: AppHandle<R>) {
    app_handle
        .get_webview_window("main")
        .unwrap()
        .close()
        .unwrap();
}
