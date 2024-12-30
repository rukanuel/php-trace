use ext_php_rs::prelude::*;
use ext_php_rs::exception::PhpException;
use ext_php_rs::types::Zval;
use serde_json::Value as JsonValue;

mod zval;
mod http;

use zval::convert_json_to_zval;
use http::fetch_url_content;

/// Fetches metadata from Cloudflare's Meta API and parses it into JSON.
fn fetch_cloudflare_meta() -> Result<JsonValue, PhpException> {
    match fetch_url_content("https://speed.cloudflare.com/meta") {
        Ok(content) => serde_json::from_str::<JsonValue>(&content)
            .map_err(|_| PhpException::default("Failed to parse JSON response.".to_string())),
        Err(_) => Err(PhpException::default("Failed to send request to Cloudflare.".to_string())),
    }
}

/// PHP function: `trace(key: Option<String>)`
///
/// Fetches metadata from Cloudflare and optionally retrieves the value for the given key.
#[php_function]
pub fn trace(key: Option<String>) -> Result<Zval, PhpException> {
    let json_value = fetch_cloudflare_meta()?;

    if let Some(k) = key {
        match json_value.get(&k) {
            Some(value) => Ok(convert_json_to_zval(value.clone())),
            None => Err(PhpException::default(format!(
                "Error: Key '{}' not found in JSON response.",
                k
            ))),
        }
    } else {
        Ok(convert_json_to_zval(json_value))
    }
}

/// Registers the PHP extension module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
