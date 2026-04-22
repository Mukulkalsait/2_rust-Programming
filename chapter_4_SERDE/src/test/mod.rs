use serde_json::Value;

pub fn test_serde_values() {
    let data = r#"
    {
        "user": {
            "name": "Mukul"
        }
    }
    "#;
    // let v: Value = serde_json::from_str(data);
    // println!("{}", v["user"]["name"]);

    match serde_json::from_str::<Value>(data) {
        Ok(v) => println!("{}", v["user"]["name"]),
        Err(e) => println!("{}", e),
    }
}
