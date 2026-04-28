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
    // 获取 AppUserModelID
    let app_id = get_app_user_model_id()?;
    
    // 创建 XML 内容
    let xml_content = format!(r#"
        <toast>
            <visual>
                <binding template="ToastText02">
                    <text id="1">{}</text>
                    <text id="2">{}</text>
                </binding>
            </visual>
        </toast>
    "#, escape_xml(title), escape_xml(message));
    
    // 创建 XmlDocument
    let xml_document = XmlDocument::new()
        .map_err(|e| format!("创建 XML 文档失败：{:?}", e))?;
    
    // 加载 XML 内容
    xml_document.LoadXml(&HSTRING::from(xml_content.as_str()))
        .map_err(|e| format!("加载 XML 失败：{:?}", e))?;
    
    // 创建 ToastNotification
    let toast = ToastNotification::CreateToastNotification(&xml_document)
        .map_err(|e| format!("创建通知失败：{:?}", e))?;
    
    // 获取 ToastNotifier
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(app_id))
        .map_err(|e| format!("创建通知器失败：{:?}", e))?;
    
    // 显示通知
    notifier.Show(&toast)
        .map_err(|e| format!("显示通知失败：{:?}", e))?;
    
    tracing::info!("[通知] 已显示 Windows 通知：{} - {}", title, message);
    Ok(())
}

/// 获取 AppUserModelID
fn get_app_user_model_id() -> Result<String, String> {
    // 使用应用名称作为 AppUserModelID
    Ok("com.camfc.client".to_string())
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
