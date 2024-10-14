use tauri::tray::TrayIconBuilder;
use tauri::webview::PageLoadEvent;
use tauri::Url;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .setup(|app| {
            let _tray = TrayIconBuilder::new()
            // .icon(app.default_window_icon().unwrap().clone())
            .build(app)?;
            // let window = app.get_webview_window("main").unwrap();
            // window.open_devtools();
            // window.close_devtools();
            Ok(())
        })
        .on_page_load(|webview, payload| {
            if payload.event() == PageLoadEvent::Finished {
                let mut webview_ = webview.clone();
                let url = Url::parse("http://127.0.0.1:8888/gui/").expect("Url invalid");
                if webview_.url().unwrap() != url {
                    webview_.navigate(url).expect("Failed to redirect");
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
