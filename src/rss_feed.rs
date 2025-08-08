use crate::rss_item::RssItem;
use egui::{Context, ScrollArea, Window};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct RssFeed {
    title: String,
    url: String,
    open: bool,
    items: Arc<Mutex<Vec<RssItem>>>,
    loading: Arc<Mutex<bool>>,
    error_message: Arc<Mutex<Option<String>>>,
    loaded: Arc<Mutex<bool>>,
}

impl RssFeed {
    pub fn new(title: String, url: String) -> Self {
        let feed = Self {
            title: title.clone(),
            url: url.clone(),
            open: true,
            items: Arc::new(Mutex::new(Vec::new())),
            loading: Arc::new(Mutex::new(false)),
            error_message: Arc::new(Mutex::new(None)),
            loaded: Arc::new(Mutex::new(false)),
        };

        // Start loading RSS feed in background
        feed.load_rss_feed();
        feed
    }

    fn load_rss_feed(&self) {
        let url = self.url.clone();
        let items = Arc::clone(&self.items);
        let loading = Arc::clone(&self.loading);
        let error_message = Arc::clone(&self.error_message);
        let loaded = Arc::clone(&self.loaded);

        // Set loading state
        *loading.lock().unwrap() = true;
        *error_message.lock().unwrap() = None;

        thread::spawn(move || {
            match Self::fetch_rss_feed(&url) {
                Ok(feed_items) => {
                    *items.lock().unwrap() = feed_items;
                    *loaded.lock().unwrap() = true;
                }
                Err(e) => {
                    *error_message.lock().unwrap() =
                        Some(format!("Failed to load RSS feed: {}", e));
                }
            }
            *loading.lock().unwrap() = false;
        });
    }

    fn fetch_rss_feed(url: &str) -> Result<Vec<RssItem>, Box<dyn std::error::Error + Send + Sync>> {
        // Make HTTP request
        let response = reqwest::blocking::get(url)?;
        let content = response.text()?;

        // Parse RSS
        let channel = content.parse::<rss::Channel>()?;

        // Convert RSS items to our RssItem struct
        let items: Vec<RssItem> = channel.items().iter().map(RssItem::from_rss_item).collect();

        Ok(items)
    }

    pub fn ui(&mut self, ctx: &Context) {
        let loading = *self.loading.lock().unwrap();
        let loaded = *self.loaded.lock().unwrap();
        let error_message = self.error_message.lock().unwrap().clone();
        let items = if loaded {
            Some(self.items.lock().unwrap().clone())
        } else {
            None
        };

        let should_reload = Window::new(&self.title)
            .open(&mut self.open)
            .resizable(true)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Feed URL:");
                    ui.label(&self.url);
                });

                ui.separator();

                let mut should_reload = false;

                if loading {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Loading RSS feed...");
                    });
                } else if let Some(error) = error_message {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                    ui.separator();
                    if ui.button("Retry").clicked() {
                        should_reload = true;
                    }
                } else if let Some(items) = &items {
                    ui.label(format!("Items: {}", items.len()));
                    ui.separator();

                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for item in items.iter() {
                                item.ui(ui);
                            }
                        });
                }

                should_reload
            })
            .map(|r| r.inner.unwrap_or(false))
            .unwrap_or(false);

        if should_reload {
            self.load_rss_feed();
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
