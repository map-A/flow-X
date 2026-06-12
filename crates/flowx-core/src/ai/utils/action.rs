//! Action parsing and validation

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

pub fn parse_action_sequence(response: &str) -> Result<Vec<Action>, Box<dyn std::error::Error>> {
    // Try direct JSON parse
    if let Ok(actions) = serde_json::from_str::<Vec<Action>>(response) {
        return Ok(actions);
    }

    // Extract JSON from code block
    if let Some(start) = response.find("```json") {
        let start = start + 7;
        if let Some(end) = response[start..].find("```") {
            let json_str = &response[start..start + end].trim();
            if let Ok(actions) = serde_json::from_str::<Vec<Action>>(json_str) {
                return Ok(actions);
            }
        }
    }

    // Extract [...] array
    use regex::Regex;
    let re = Regex::new(r"\[.*\]")?;
    if let Some(mat) = re.find(response) {
        if let Ok(actions) = serde_json::from_str::<Vec<Action>>(mat.as_str()) {
            return Ok(actions);
        }
    }

    Err("Failed to parse action sequence".into())
}

pub fn validate_action(action: &Action) -> bool {
    match action.action.as_str() {
        "open_app" => action.package.is_some(),
        "click" => action.target.is_some(),
        "input" => action.text.is_some(),
        "swipe" => action.direction.is_some(),
        "wait" => action.target.is_some(),
        "back" | "home" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direct_json() {
        let json = r#"[{"action": "click", "target": "button"}]"#;
        let actions = parse_action_sequence(json).unwrap();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "click");
    }

    #[test]
    fn test_validate_action() {
        let action = Action {
            action: "click".to_string(),
            package: None,
            target: Some("button".to_string()),
            text: None,
            direction: None,
            duration: None,
            timeout: None,
        };
        assert!(validate_action(&action));
    }
}
