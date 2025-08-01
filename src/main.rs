use eframe::NativeOptions;

use crate::rss_app::RssApp;

mod rss_app;
mod rss_source;
mod ui;

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "RSS Viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(RssApp::new(cc)))),
    );
}
