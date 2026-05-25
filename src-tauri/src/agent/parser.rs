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
    pub scroll_amount: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
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
        let mut tool_calls = Vec::new();

        // 尝试格式 0: AI Agent 格式 - Action: 描述 + JSON 对象
        // 例如: Action: 点击开始菜单\n{"name": "computer_use", "arguments": {"action": "left_click", "coordinate": [50, 990]}}
        if tool_calls.is_empty() {
            let re0 = Regex::new(r#"Action:[^\n]*\n*(\{.*\})"#).unwrap();
            if let Some(cap) = re0.captures(text) {
                if let Some(json_match) = cap.get(1) {
                    let content_str = json_match.as_str().trim();
                    println!("[解析] 尝试解析 AI Agent 格式：{}", content_str);
                    if let Ok(tool_call) = serde_json::from_str::<ToolCall>(content_str) {
                        println!("[解析] AI Agent 格式成功：{} - {}", tool_call.name, tool_call.arguments.action);
                        tool_calls.push(tool_call);
                    }
                }
            }
        }

        // 尝试格式 1: <tool_call>...</tool_call>
        let re1 = Regex::new(r"(?s)<tool_call>(.*?)</tool_call>").unwrap();
        for cap in re1.captures_iter(text) {
            if let Some(content) = cap.get(1) {
                let content_str = content.as_str().trim();
                println!("[解析] 尝试解析 <tool_call> 格式：{}", content_str);
                if let Ok(tool_call) = serde_json::from_str::<ToolCall>(content_str) {
                    println!("[解析] 成功：{} - {}", tool_call.name, tool_call.arguments.action);
                    tool_calls.push(tool_call);
                }
            }
        }

        // 尝试格式 2: 纯 JSON 对象 { "name": ..., "arguments": ... }
        if tool_calls.is_empty() {
            let re2 = Regex::new(r#"\{[^{}]*"name"[^{}]*"arguments"[^{}]*\}"#).unwrap();
            if let Some(cap) = re2.find(text) {
                let content_str = cap.as_str().trim();
                println!("[解析] 尝试解析纯 JSON 格式：{}", content_str);
                if let Ok(tool_call) = serde_json::from_str::<ToolCall>(content_str) {
                    println!("[解析] 成功：{} - {}", tool_call.name, tool_call.arguments.action);
                    tool_calls.push(tool_call);
                }
            }
        }

        // 尝试格式 3: JSON 数组 [...其中某个元素是 ToolCall]
        if tool_calls.is_empty() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                if let Some(arr) = json.as_array() {
                    for item in arr {
                        if let Ok(tool_call) = serde_json::from_value::<ToolCall>(item.clone()) {
                            println!("[解析] 从数组中解析成功：{} - {}", tool_call.name, tool_call.arguments.action);
                            tool_calls.push(tool_call);
                        }
                    }
                }
            }
        }

        // 尝试格式 4: 尝试直接解析整个文本为 ToolCall
        if tool_calls.is_empty() {
            println!("[解析] 尝试直接解析整个响应");
            if let Ok(tool_call) = serde_json::from_str::<ToolCall>(text) {
                println!("[解析] 直接解析成功：{} - {}", tool_call.name, tool_call.arguments.action);
                tool_calls.push(tool_call);
            }
        }

        if tool_calls.is_empty() {
            eprintln!("[解析] 未提取到有效操作，原始文本：{}", text);
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
        action.arguments.scroll_amount.or(action.arguments.pixels).unwrap_or(1)
    }

    pub fn get_wait_time(&self, action: &ToolCall) -> f64 {
        action.arguments.duration.or(action.arguments.time).unwrap_or(2.0)
    }

    pub fn get_status(&self, action: &ToolCall) -> Option<String> {
        action.arguments.status.clone()
    }
}