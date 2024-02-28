use std::fs;

use serde_json::{Result, Value};

fn main() -> Result<()> {
    let file_path = "/Users/guimal/Library/Application Support/Slack/storage/root-state.json";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let json: Value = serde_json::from_str(&contents)?;

    let teams = &json["webapp"]["teams"];

    let team_id = &teams
        .as_object()
        .unwrap()
        .keys()
        .next()
        .expect("Couldn't find team_id, bailing.");

    let direct_msgs = &teams[&team_id]["unreads"]["unreadHighlights"]
        .as_u64()
        .expect("Couldn't find direct messages, bailing");

    let indirect_msgs = &teams[&team_id]["unreads"]["unreads"]
        .as_u64()
        .expect("Couldn find indirect messages, bailing");

    println!("󰒱 {direct_msgs}|{indirect_msgs}");

    Ok(())
}
