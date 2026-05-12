// 保留所有权利
//
// Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
// Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504594170@qq.com
//
// Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
// Email: abc.cxh2009@foxmail.com
//
// Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
// Email: 1220594170@qq.com
//
// Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
// Email: admin@mc666.top

use windows::Data::Xml::Dom::XmlDocument;
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};
use windows::core::HSTRING;

/// 显示 Windows 原生通知
/// 
/// # 参数
/// * `title` - 通知标题
/// * `message` - 通知内容
pub fn show_notification(title: &str, message: &str) -> Result<(), String> {
    show_notification_with_config(title, message, None, None)
}

/// 显示 Windows 原生通知（带配置）
/// 
/// # 参数
/// * `title` - 通知标题
/// * `message` - 通知内容
/// * `app_id` - 可选的应用 ID，默认为 "CAMFC"
/// * `duration` - 可选的显示持续时间，"short" 或 "long"，默认为 "short"
pub fn show_notification_with_config(
    title: &str, 
    message: &str,
    app_id: Option<&str>,
    duration: Option<&str>,
) -> Result<(), String> {
    let app_id = app_id.unwrap_or("CAMFC");
    
    let duration_attr = match duration.unwrap_or("short") {
        "long" => r#"duration="long""#,
        _ => r#"duration="short""#,
    };
    
    let xml_content = format!(r#"
        <toast {duration_attr}>
            <visual>
                <binding template="ToastText02">
                    <text id="1">{}</text>
                    <text id="2">{}</text>
                </binding>
            </visual>
        </toast>
    "#, escape_xml(title), escape_xml(message));
    
    let xml_document = XmlDocument::new()
        .map_err(|e| format!("创建 XML 文档失败：{:?}", e))?;
    
    xml_document.LoadXml(&HSTRING::from(xml_content.as_str()))
        .map_err(|e| format!("加载 XML 失败：{:?}", e))?;
    
    let toast = ToastNotification::CreateToastNotification(&xml_document)
        .map_err(|e| format!("创建通知失败：{:?}", e))?;
    
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(app_id))
        .map_err(|e| format!("创建通知器失败：{:?}", e))?;
    
    notifier.Show(&toast)
        .map_err(|e| format!("显示通知失败：{:?}", e))?;
    
    tracing::info!("[通知] 已显示 Windows 通知：{} - {}", title, message);
    Ok(())
}

/// 转义 XML 特殊字符
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("Hello & World"), "Hello &amp; World");
        assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
    }
}
