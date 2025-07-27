mod rss_app;
mod rss_source;
mod ui;

use gpui::{
    AppContext, Application, Bounds, KeyBinding, WindowBounds, WindowOptions, actions, px, size,
};

use crate::rss_app::RssApp;

actions!(window, [Quit]);

fn main() {
    Application::new().run(|app| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), app);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        app.open_window(options, |window, cx| {
            cx.new(|cx| {
                cx.observe_window_bounds(window, move |_, window, _| {
                    println!("Window bounds changed: {:?}", window.bounds());
                })
                .detach();

                RssApp::new()
            })
        })
        .unwrap();

        app.activate(true);
        app.on_action(|_: &Quit, app| app.quit());
        app.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
