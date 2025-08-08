use url::Url;

pub struct UrlUtils {}

impl UrlUtils {
    pub fn clean_url(url: &str) -> Result<String, url::ParseError> {
        let trimmed = url.trim();

        // If no protocol is specified, assume https
        let url_with_protocol = if trimmed.starts_with("http://") || trimmed.starts_with("https://")
        {
            trimmed.to_string()
        } else {
            format!("https://{}", trimmed)
        };

        // Parse and validate URL
        let parsed = Url::parse(&url_with_protocol)?;
        Ok(parsed.to_string())
    }

    pub fn is_valid_url(url: &str) -> bool {
        Self::clean_url(url).is_ok()
    }
}
