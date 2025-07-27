use crate::{rss_source::RssSource, ui};

use eframe::{App, Frame};
use egui::{CentralPanel, Context};

pub struct RssApp {
    rss_sources: Vec<RssSource>,
}

impl RssApp {
    pub fn new() -> Self {
        Self {
            rss_sources: vec![],
        }
    }
}

impl App for RssApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(ui::labels::H1 {
                text: "RSS Viewier".to_string(),
            });

            ui.separator();

            ui.add(ui::labels::H2 {
                text: format!("Sources ({})", self.rss_sources.len()),
            });
        });
    }
}
