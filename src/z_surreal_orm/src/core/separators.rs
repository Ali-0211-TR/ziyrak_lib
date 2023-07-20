use std::collections::BTreeMap;
use serde_json::{self, Value};


pub fn vec_to_string_without_scope<T: ToString>(v: &Vec<T>) -> String {
    let mut s = String::new();

    for (i, l) in v.iter().enumerate() {
        let l = l.to_string();
        s.push_str(l.as_str());
        if i < (v.len() - 1) {
            s.push_str(", ");
        }
    }

    s
}
pub fn obj_to_string<T: serde::Serialize>(obj: T) -> serde_json::Result<String> {
    serde_json::to_string(&obj)
}
pub fn separate_fields_and_values<T: serde::Serialize>(
    obj: T,
) -> (Vec<String>, Vec<String>) {
    let json = serde_json::to_value(obj).unwrap();

    let fields: Vec<String> = json
        .as_object()
        .unwrap()
        .keys()
        .map(|k| k.to_string())
        .collect();


    let values: Vec<String> = json
        .as_object()
        .unwrap()
        .values()
        .map(|v| v.to_string())
        .collect();


    (fields, values)
}


pub fn json_to_map(json_str: &str) -> std::result::Result<BTreeMap<String, String>, String> {
    let json_value: Value = serde_json::from_str(json_str).expect("Error epta");
    if let Value::Object(obj) = json_value {
        let mut map = BTreeMap::new();
        for (key, value) in obj {
            // if let Value::String(s) = value {
            //     map.insert(key, s);
            // }
            match value {
                _ => {map.insert(key, value.to_string())}
            };
        }
        Ok(map)
    } else {
        Err("Error".to_string())
    }
}

pub fn separate_fields(
    json_obj_str: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let btm = json_to_map(json_obj_str.as_str())?;
    let fields: Vec<String> = btm.keys().map(|v| v.clone()).collect();
    Ok(fields)
}
pub fn separate_values(
    json_obj_str: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let btm = json_to_map(json_obj_str.as_str())?;
    let values: Vec<String> = btm.values().map(|v| v.clone()).collect();
    Ok(values)
}