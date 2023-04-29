#[cfg(feature = "v2_4")]
use glib::ToVariant;
use gtk::{prelude::*, Inhibit, Window, WindowType};
use std::{env, fs};
use webkit2gtk::{
    traits::{SettingsExt, WebViewExt},
    WebContext, WebView,
};
#[cfg(feature = "v2_6")]
use webkit2gtk::{UserContentManager, WebViewExtManual};

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("RUST + WEBKIT + GTK");
    window.set_default_size(800, 600);
    let context = WebContext::default().unwrap();

    #[cfg(feature = "v2_4")]
    context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
    #[cfg(feature = "v2_6")]
    let webview =
        WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
    #[cfg(not(feature = "v2_6"))]
    let webview = WebView::with_context(&context);

    load_html(&webview);
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn load_html(webview: &WebView) {
    let mut location = env::current_dir().unwrap();
    location.push("dist");
    location.push("index.html");
    let file = fs::read_to_string(location.to_str().unwrap());
    match file {
        Err(_) => {}
        Ok(data) => {
            location.pop(); // remove index.html from pathbuf
            webview.load_html(&data, location.to_str());
        }
    };
}
