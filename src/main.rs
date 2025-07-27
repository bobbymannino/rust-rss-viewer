mod rss_app;
mod rss_source;

use gpui::{
    AppContext, Application, Bounds, KeyBinding, WindowBounds, WindowOptions, actions, px, size,
};

use crate::rss_app::RssApp;

actions!(window, [Quit]);

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            cx.new(|cx| {
                cx.observe_window_bounds(window, move |_, window, _| {
                    println!("Window bounds changed: {:?}", window.bounds());
                })
                .detach();

                RssApp::new()
            })
        })
        .unwrap();

        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
