use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: ActionParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionParameters {
    pub action: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<Vec<f64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate1: Option<Vec<f64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate2: Option<Vec<f64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixels: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

pub struct ToolCallParser;

impl ToolCallParser {
    pub fn new() -> Self {
        ToolCallParser
    }

    pub fn extract_tool_calls(&self, text: &str) -> Vec<ToolCall> {
        let re = Regex::new(r"(?s)<tool_call>(.*?)</tool_call>").unwrap();
        let mut tool_calls = Vec::new();

        for cap in re.captures_iter(text) {
            if let Some(content) = cap.get(1) {
                let content_str = content.as_str().trim();
                println!("[解析] 尝试解析：{}", content_str);
                match serde_json::from_str::<ToolCall>(content_str) {
                    Ok(tool_call) => {
                        println!("[解析] 成功：{} - {}", tool_call.name, tool_call.arguments.action);
                        tool_calls.push(tool_call);
                    },
                    Err(e) => {
                        eprintln!("[解析失败] {} | 内容：{}", e, content_str);
                    }
                }
            }
        }

        tool_calls
    }

    pub fn transform_coordinates(
        &self,
        action: &mut ToolCall,
        resized_width: u32,
        resized_height: u32,
    ) {
        let keys = ["coordinate", "coordinate1", "coordinate2"];

        for key in &keys {
            let coords = match *key {
                "coordinate" => action.arguments.coordinate.as_mut(),
                "coordinate1" => action.arguments.coordinate1.as_mut(),
                "coordinate2" => action.arguments.coordinate2.as_mut(),
                _ => None,
            };

            if let Some(coord) = coords {
                if coord.len() >= 2 {
                    coord[0] = (coord[0] / 1000.0 * resized_width as f64) as f64;
                    coord[1] = (coord[1] / 1000.0 * resized_height as f64) as f64;
                }
            }
        }
    }

    pub fn get_coordinate(&self, action: &ToolCall) -> Option<(i32, i32)> {
        action.arguments.coordinate.as_ref().map(|c| {
            if c.len() >= 2 {
                (c[0] as i32, c[1] as i32)
            } else {
                (0, 0)
            }
        })
    }

    pub fn get_action_type<'a>(&self, action: &'a ToolCall) -> &'a str {
        &action.arguments.action
    }

    pub fn get_keys(&self, action: &ToolCall) -> Option<Vec<String>> {
        action.arguments.keys.clone()
    }

    pub fn get_text(&self, action: &ToolCall) -> Option<String> {
        action.arguments.text.clone()
    }

    pub fn get_pixels(&self, action: &ToolCall) -> i32 {
        action.arguments.pixels.unwrap_or(1)
    }

    pub fn get_wait_time(&self, action: &ToolCall) -> f64 {
        action.arguments.time.unwrap_or(2.0)
    }

    pub fn get_status(&self, action: &ToolCall) -> Option<String> {
        action.arguments.status.clone()
    }
}