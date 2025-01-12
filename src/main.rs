#![allow(unused)]

use tao::event::{Event, StartCause, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
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

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent)
}

fn main() -> wry::Result<()> {
    let mut event_loop_builder = EventLoopBuilder::<UserEvent>::with_user_event();
    let event_loop = event_loop_builder.build();

    // set a menu event handler that wakes up the event loop
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
    }));
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
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
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == open.id() {
                    window.set_visible(true);
                } else if event.id == quit.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                //Doesnt seem to do anything?
            }
            //Minimize to tray instead of closing
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested, .. } => {
                window.set_visible(false);
            }
            _ => {}
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
