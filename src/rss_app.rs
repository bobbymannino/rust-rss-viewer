use eframe::{App, CreationContext};
use egui::CentralPanel;

use crate::ui::labels::H1;

#[derive(Default)]
pub struct RssApp {}

impl RssApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for RssApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            //.VolumeIcon.icns
            ui.label("Hello there");
            ui.add(H1::new("Heading 1".to_string()));

            if ui.button("Close").clicked() {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                });
            }
        });
    }
}
