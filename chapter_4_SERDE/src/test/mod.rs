use serde_json::Value;

pub fn test_serde_values() {
    let v: Value = serde_json::from_str(data)?;
    println!("{}", v["user"]["name"]);
}
