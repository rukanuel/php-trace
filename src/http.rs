use reqwest::blocking::get;

/// Fetches the content of a URL as a string.
/// Returns `Ok(String)` on success or `Err(u8)` on failure.
pub fn fetch_url_content(url: &str) -> Result<String, u8> {
    match get(url) {
        Ok(res) if res.status().is_success() => res
            .text()
            .map_err(|_| 0), // Handle text conversion errors
        _ => Err(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_url_content_success() {
        let url = "https://httpbin.org/get";
        let result = fetch_url_content(url);
        assert!(result.is_ok(), "Expected OK, got {:?}", result);
    }

    #[test]
    fn test_fetch_url_content_failure() {
        let url = "https://nonexistent.url";
        let result = fetch_url_content(url);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }
}