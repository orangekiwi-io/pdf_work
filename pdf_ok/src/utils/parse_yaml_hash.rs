use std::collections::HashMap;

use serde_yaml::{Mapping, Value};

/// ## Function: `parse_yaml_hash` - Parses a YAML hash into a `HashMap` of key-value pairs
///
/// This function parses a YAML hash into a `HashMap` of key-value pairs.
///
/// ### Arguments
///
/// * `yaml_hash` - The YAML hash to parse into a `HashMap` of key-value
/// pairs
///
/// ### Returns
///
/// A `HashMap` of key-value pairs representing the YAML hash.
/// If the YAML hash is not valid, an error is returned.
///
pub fn yaml_mapping_to_hashmap(yaml: &Value) -> Option<HashMap<String, Value>> {
    match yaml {
        Value::Mapping(mapping) => {
            let mut hashmap = HashMap::new();
            for (key, value) in mapping.iter() {
                if let (Value::String(key), value) = (key, value) {
                    hashmap.insert(key.clone(), value.clone());
                } else {
                    // Handle non-string keys or non-scalar values
                    return None;
                }
            }
            Some(hashmap)
        }
        _ => None,
    }
}