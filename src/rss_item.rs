#[derive(Debug, Clone)]
pub struct RssItem {
    pub title: String,
    pub description: String,
    pub link: Option<String>,
    pub pub_date: Option<String>,
    pub author: Option<String>,
}

impl RssItem {
    pub fn from_rss_item(item: &rss::Item) -> Self {
        Self {
            title: item.title().unwrap_or("No Title").to_string(),
            description: item.description().unwrap_or("No Description").to_string(),
            link: item.link().map(|s| s.to_string()),
            pub_date: item.pub_date().map(|s| s.to_string()),
            author: item.author().map(|s| s.to_string()),
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(&self.title);

            ui.horizontal(|ui| {
                if let Some(date) = &self.pub_date {
                    ui.label(date);
                }

                if let Some(author) = &self.author {
                    ui.label(author);
                }
            });

            ui.separator();

            let cleaned_description = self.clean_html(&self.description);
            ui.label(cleaned_description);

            if let Some(link) = &self.link {
                if ui.button("Read More").clicked() {
                    if let Err(e) = webbrowser::open(link) {
                        eprintln!("Failed to open link: {}", e);
                    }
                }
            }
        });

        ui.add_space(5.0);
    }

    fn clean_html(&self, text: &str) -> String {
        // Simple HTML tag removal - replace with more sophisticated parsing if needed
        let mut result = text.to_string();

        // Remove common HTML tags
        let tags_to_remove = [
            "<p>",
            "</p>",
            "<br>",
            "<br/>",
            "<div>",
            "</div>",
            "<span>",
            "</span>",
            "<strong>",
            "</strong>",
            "<b>",
            "</b>",
            "<em>",
            "</em>",
            "<i>",
            "</i>",
            "<a>",
            "</a>",
        ];

        for tag in &tags_to_remove {
            result = result.replace(tag, " ");
        }

        // Remove any remaining HTML tags using a simple regex-like approach
        while let Some(start) = result.find('<') {
            if let Some(end) = result[start..].find('>') {
                result.replace_range(start..start + end + 1, " ");
            } else {
                break;
            }
        }

        // Clean up whitespace
        result = result.split_whitespace().collect::<Vec<_>>().join(" ");

        // Truncate if too long
        if result.len() > 500 {
            result.truncate(497);
            result.push_str("...");
        }

        result
    }
}
