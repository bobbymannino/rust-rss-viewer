mod rss_app;
mod rss_source;
mod ui;

use crate::rss_app::RssApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 500.0])
            .with_min_inner_size([500.0, 500.0])
            .with_title("RSS Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "RSS Viewier",
        options,
        Box::new(|_cc| Ok(Box::new(RssApp::new()))),
    )
}
