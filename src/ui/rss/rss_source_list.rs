use egui::{Color32, Layout, Widget};

use crate::{rss_source::RssSource, ui::labels::H2};

pub struct RssSourceList<'a> {
    sources: &'a Vec<RssSource>,
}

impl<'a> RssSourceList<'a> {
    pub fn new(sources: &'a Vec<RssSource>) -> Self {
        Self { sources }
    }
}

impl<'a> Widget for RssSourceList<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut response = ui.add(H2::new(format!("Sources ({})", self.sources.len())));

        for (index, source) in self.sources.iter().enumerate() {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(&format!("{}.", index + 1));
                    ui.strong(source.title());
                    ui.label("-");
                    ui.monospace(source.url());
                });

                ui.horizontal(|ui| {
                    if ui.button("View").clicked() {}
                    if ui.button("Edit").clicked() {}
                    if ui.button("Delete").clicked() {}
                })
            });
        }

        if self.sources.is_empty() {
            response = ui.label("No RSS sources added yet.");
        }

        response
    }
}
