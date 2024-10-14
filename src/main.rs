#![allow(unused)]

use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};
use wry::WebViewBuilder;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "icons/"]
struct Asset;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    event_loop.set_device_event_filter(tao::event_loop::DeviceEventFilter::Never);

    let window = WindowBuilder::new()
        .with_title("Resilio Sync")
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new().with_url("http://127.0.0.1:8888/gui/");

    //Linux specific build
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    let tray_menu = Menu::new();

    let open = MenuItem::new("Open", true, None);
    let quit = MenuItem::new("Quit", true, None);

    tray_menu.append_items(&[&open, &PredefinedMenuItem::separator(), &quit]);

    let mut tray_icon = None;

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        // We add delay of 16 ms (60fps) to event_loop to reduce cpu load.
        // This can be removed to allow ControlFlow::Poll to poll on each cpu cycle
        // Alternatively, you can set ControlFlow::Wait or use TrayIconEvent::set_event_handler,
        // see https://github.com/tauri-apps/tray-icon/issues/83#issuecomment-1697773065
        *control_flow = ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(16),
        );

        if let Event::NewEvents(StartCause::Init) = event {
            let icon = load_icon();

            // We create the icon once the event loop is actually running
            // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
            tray_icon = Some(
                TrayIconBuilder::new()
                    .with_menu(Box::new(tray_menu.clone()))
                    .with_tooltip("Resilio Sync") //TODO update tooltip and icon based on service status
                    .with_icon(icon)
                    .build()
                    .unwrap(),
            );
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == open.id() {
                window.set_visible(true);
            } else if event.id == quit.id() {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
        }

        if let Ok(event) = tray_channel.try_recv() {
            //Doesnt seem to do anything?
        }

        //Minimize to tray instead of closing
        if let Event::WindowEvent { window_id, event: WindowEvent::CloseRequested, .. } = event {
            window.set_visible(false);
        }
    })
}

fn load_icon() -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let asset = Asset::get("icon.png").unwrap();
        let raw = asset.data.as_ref();
        let image = image::load_from_memory(raw)
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
