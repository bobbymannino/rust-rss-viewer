use gpui::{FontWeight, IntoElement, ParentElement, Styled, div};

pub struct H1 {}

impl H1 {
    pub fn new(text: String) -> impl IntoElement {
        div().child(text).text_3xl().font_weight(FontWeight::BOLD)
    }
}
