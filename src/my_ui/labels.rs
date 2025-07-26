use eframe::egui::{Response, RichText, Ui, Widget};

pub struct H1 {
    pub text: String,
}

pub struct H2 {
    pub text: String,
}

impl Widget for H1 {
    fn ui(self, ui: &mut Ui) -> Response {
        let rich_text = RichText::new(self.text).strong().size(40.0);

        ui.label(rich_text)
    }
}

impl Widget for H2 {
    fn ui(self, ui: &mut Ui) -> Response {
        let rich_text = RichText::new(self.text).strong().size(32.0);

        ui.label(rich_text)
    }
}
