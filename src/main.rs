use crate::main_window::MainWindow;
use eframe::NativeOptions;
use egui::ViewportBuilder;

mod main_window;
mod rss_feed;
mod rss_item;
mod url_utils;

fn main() {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size([600.0, 400.0])
            .with_title("RSS Viewer"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "RSS Viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
    );
}
