use serde::de::DeserializeOwned;

pub fn json_string_to_struct<T: DeserializeOwned>(stringy_json: String) -> Result<T, ()> {
    let v: serde_json::Value = serde_json::from_str(&stringy_json).unwrap();

    println!("start time {}", v["time"]["start"]);

    let structy_value = serde_json::from_str(&stringy_json).map_err(|_e| {
        println!("{}", _e.to_string());
        return ();
    })?;

    Ok(structy_value)
}
