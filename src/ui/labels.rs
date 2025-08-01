use egui::{Color32, RichText, Widget};

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
        let rt = RichText::new(self.text)
            .color(Color32::BLACK)
            .size(40.)
            .strong();

        ui.label(rt)
    }
}

pub struct H2 {
    text: String,
}

impl H2 {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Widget for H2 {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let rt = RichText::new(self.text)
            .color(Color32::BLACK)
            .size(28.)
            .strong();

        ui.label(rt)
    }
}
