use gpui::{Context, IntoElement, Render, Window};

use crate::ui::text::H1;

pub struct RssApp {}

impl RssApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for RssApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        H1::new("RSS Viewer".to_string())
    }
}
