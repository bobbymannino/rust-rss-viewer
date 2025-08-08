use eframe::{App, CreationContext};
use egui::{Align, CentralPanel, Layout};

use crate::rss_source::RssSource;

pub struct RssApp {
    sources: Vec<RssSource>,
}

impl RssApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            sources: vec![RssSource::bobman()],
        }
    }
}

impl App for RssApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("RSS Viewier");

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                let button = ui.button("Close");
                if button.clicked() {
                    let ctx = ctx.clone();
                    std::thread::spawn(move || {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    });
                }
            });
        });
    }
}
