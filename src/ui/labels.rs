use egui::{RichText, Widget};

pub struct H1 {
    text: String,
}

impl H1 {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Widget for H1 {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(RichText::new(self.text).strong())
    }
}
