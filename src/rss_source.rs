pub struct RssSource {
    title: String,
    url: String,
}

impl RssSource {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }

    pub fn update(&mut self, title: String, url: String) {
        self.title = title;
        self.url = url;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
