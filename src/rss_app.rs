use gpui::{Bounds, Context, IntoElement, Render, Window, WindowBounds, div, px, size};

pub struct RssApp {}

impl RssApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for RssApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
