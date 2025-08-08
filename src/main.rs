use eframe::NativeOptions;
use egui::ViewportBuilder;

use crate::rss_app::RssApp;

mod rss_app;
mod rss_source;

fn main() {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default().with_min_inner_size([500., 500.]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "RSS Viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(RssApp::new(cc)))),
    );
}
