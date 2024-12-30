use serde_json::Value as JsonValue;
use ext_php_rs::types::{ZendHashTable, Zval};

/// Converts a JSON value into a PHP `Zval`.
pub fn convert_json_to_zval(json: JsonValue) -> Zval {
    let mut zv = Zval::new();
    match json {
        JsonValue::String(s) => {
            zv.set_string(&s, false).unwrap();
        }
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                zv.set_long(i);
            } else if let Some(f) = n.as_f64() {
                zv.set_double(f);
            }
        }
        JsonValue::Bool(b) => {
            zv.set_bool(b);
        }
        JsonValue::Array(arr) => {
            let mut ht = ZendHashTable::with_capacity(arr.len().try_into().unwrap());
            for value in arr {
                ht.push(convert_json_to_zval(value)).unwrap();
            }
            zv.set_hashtable(ht);
        }
        JsonValue::Object(obj) => {
            let mut ht = ZendHashTable::with_capacity(obj.len().try_into().unwrap());
            for (key, value) in obj {
                ht.insert(&key, convert_json_to_zval(value)).unwrap();
            }
            zv.set_hashtable(ht);
        }
        JsonValue::Null => {
            zv.set_null();
        }
    }
    zv
}
