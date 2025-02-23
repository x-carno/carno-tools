use serde_json::{to_string_pretty, Value};

// JSON Format
#[tauri::command]
pub fn format_json(json: String) -> Result<String, String> {
    println!("{}", json);
    // let text = "这是一段包含“弯引号”的字符串";
    let replaced_text: String = json
        .chars()
        .map(|c| match c {
            '“' | '”' => '"',
            _ => c,
        })
        .collect();
    let parsed: Value = serde_json::from_str(&replaced_text).map_err(|e| e.to_string())?;
    to_string_pretty(&parsed).map_err(|e| e.to_string())
}
