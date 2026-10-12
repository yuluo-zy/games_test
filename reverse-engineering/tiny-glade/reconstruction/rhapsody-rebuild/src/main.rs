use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let result = line
            .map_err(|e| e.to_string())
            .and_then(|s| {
                serde_json::from_str::<rhapsody::app::Scenario>(&s).map_err(|e| e.to_string())
            })
            .and_then(|s| rhapsody::app::run(&s));
        match result {
            Ok(value) => println!("{value}"),
            Err(error) => println!(
                "{}",
                serde_json::json!({"status":"invalid_input","error":error})
            ),
        }
    }
}
