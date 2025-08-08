use crate::rss_feed::RssFeed;
use crate::url_utils::UrlUtils;
use eframe::{App, CreationContext};
use egui::{CentralPanel, Layout, TextEdit};
use std::collections::HashMap;
use url::Url;

pub struct MainWindow {
    title_input: String,
    url_input: String,
    feed_windows: HashMap<String, RssFeed>,
}

impl MainWindow {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            title_input: String::new(),
            url_input: String::new(),
            feed_windows: HashMap::new(),
        }
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("RSS Viewer");
            ui.add_space(8.);

            ui.group(|ui| {
                ui.label("Add RSS Feed");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.add(
                        TextEdit::singleline(&mut self.title_input)
                            .hint_text("Enter feed title (optional)")
                            .desired_width(300.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("URL: ");
                    ui.add(
                        TextEdit::singleline(&mut self.url_input)
                            .hint_text("Enter RSS feed URL")
                            .desired_width(300.0),
                    );
                });

                ui.add_space(16.);

                ui.label("Presets");
                ui.horizontal(|ui| {
                    if ui.button("bobman.dev").clicked() {
                        self.title_input = "bobman.dev".to_string();
                        self.url_input = "https://bobman.dev/rss.xml".to_string();
                    }
                });

                ui.add_space(8.);

                ui.horizontal(|ui| {
                    let url_valid = !self.url_input.trim().is_empty()
                        && UrlUtils::is_valid_url(&self.url_input);

                    ui.add_enabled_ui(url_valid, |ui| {
                        if ui.button("Open Feed").clicked() {
                            if let Ok(cleaned_url) = UrlUtils::clean_url(&self.url_input) {
                                // Use cleaned URL as the key to prevent duplicates
                                if !self.feed_windows.contains_key(&cleaned_url) {
                                    let title = if self.title_input.trim().is_empty() {
                                        // Extract domain name as default title
                                        if let Ok(parsed) = Url::parse(&cleaned_url) {
                                            parsed.host_str().unwrap_or("RSS Feed").to_string()
                                        } else {
                                            "RSS Feed".to_string()
                                        }
                                    } else {
                                        self.title_input.clone()
                                    };

                                    let feed_window = RssFeed::new(title, cleaned_url.clone());
                                    self.feed_windows.insert(cleaned_url, feed_window);

                                    // create new run_native here

                                    // Clear inputs after successful addition
                                    self.title_input.clear();
                                    self.url_input.clear();
                                }
                            }
                        }
                    });

                    if !url_valid && !self.url_input.trim().is_empty() {
                        ui.colored_label(egui::Color32::RED, "❌ Invalid URL");
                    }
                });
            });

            // Show currently open feeds
            if !self.feed_windows.is_empty() {
                ui.add_space(16.);

                ui.group(|ui| {
                    ui.label("Open Feeds:");
                    ui.separator();

                    let mut feeds_to_remove = Vec::new();

                    for (url, feed) in &self.feed_windows {
                        ui.horizontal(|ui| {
                            ui.label(format!("📡 {}", feed.title()));
                            ui.label(format!("({})", url));

                            if ui.small_button("❌").clicked() {
                                feeds_to_remove.push(url.clone());
                            }
                        });
                    }

                    // Remove closed feeds
                    for url in feeds_to_remove {
                        self.feed_windows.remove(&url);
                    }
                });
            }

            ui.add_space(16.);

            ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("Close").clicked() {
                    let ctx = ctx.clone();

                    std::thread::spawn(move || {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    });
                }
            });
        });

        // Update all feed windows and remove closed ones
        let mut closed_feeds = Vec::new();

        for (url, feed) in &mut self.feed_windows {
            feed.ui(ctx);
            if !feed.is_open() {
                closed_feeds.push(url.clone());
            }
        }

        // Remove closed feed windows
        for url in closed_feeds {
            self.feed_windows.remove(&url);
        }
    }
}
