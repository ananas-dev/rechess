use redis_async::resp::RespValue;
use std::collections::HashMap;

pub fn get_hashmap(redis_hash: RespValue) -> Option<HashMap<String, String>> {
    if let RespValue::Array(redis_array) = redis_hash {
        let purged_array: Vec<String> = redis_array
            .iter()
            .filter_map(|item| match item {
                RespValue::BulkString(encoded_str) => {
                    Some(String::from_utf8_lossy(encoded_str).to_string())
                }
                _ => None,
            })
            .collect();

        // TODO: Write this in a more functional way

        let mut is_key = true;
        let mut last_key: String = String::new();
        let mut result = HashMap::new();

        if purged_array.len() % 2 == 0 {
            for item in purged_array {
                if is_key {
                    last_key = item;
                } else {
                    result.insert(last_key.clone(), item);
                }
                is_key = !is_key;
            }
        }
        Some(result)
    } else {
        None
    }
}
