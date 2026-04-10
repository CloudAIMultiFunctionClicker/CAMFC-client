// 保留所有权利
//
// Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
// Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com
//
// Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
// Email: abc.cxh09@foxmail.com
//
// Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
// Email: 1220594170@qq.com
//
// Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
// Email: admin@mc666.top

// 活动日志模块
// 负责记录用户的最近操作（上传、下载、文件访问）
//
// 功能：
// 1. 记录用户的上传、下载、文件访问操作
// 2. 查询最近活动记录
// 3. 支持按类型过滤
// 4. 每个用户最多保留 100 条记录

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use chrono::{DateTime, Utc};

// 导入笔记图片模块
use crate::note_image::{get_user_images, delete_user_image};

// 导入设备管理器
use crate::cpen_device_manager::CpenDeviceManager;
use crate::bluetooth::DeviceInfo;
use crate::storage::get_app_data_dir;
use std::sync::OnceLock;
use std::sync::Arc;
use tokio::sync::Mutex;

// 活动类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    Upload,
    Download,
    Access,
}

// 活动记录结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub timestamp: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
}

// 活动记录响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponse {
    pub user_uuid: String,
    pub total: usize,
    pub limit: usize,
    pub activity_type: Option<String>,
    pub activities: Vec<Activity>,
}

// 活动日志管理器
pub struct ActivityLogManager {
    user_uuid: String,
    max_records: usize,
}

impl ActivityLogManager {
    const MAX_RECORDS: usize = 100;
    const ACTIVITY_LOGS_DIR: &str = ".activity_logs";

    pub fn new(user_uuid: String) -> Self {
        Self {
            user_uuid,
            max_records: Self::MAX_RECORDS,
        }
    }

    // 获取日志目录路径
    async fn get_log_path() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .context("获取应用数据目录失败")?
            .join("CAMFC");

        let log_dir = data_dir.join(Self::ACTIVITY_LOGS_DIR);

        if !log_dir.exists() {
            fs::create_dir_all(&log_dir).await
                .context(format!("创建活动日志目录失败：{:?}", log_dir))?;
        }

        Ok(log_dir)
    }

    // 获取用户活动日志文件路径
    async fn get_user_log_path(&self) -> Result<PathBuf> {
        let log_dir = Self::get_log_path().await?;
        Ok(log_dir.join(format!("{}.json", self.user_uuid)))
    }

    // 加载用户的活动记录
    async fn load_activities(&self) -> Result<Vec<Activity>> {
        let path = self.get_user_log_path().await?;

        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path).await
            .context("读取活动日志文件失败")?;

        let activities: Vec<Activity> = serde_json::from_str(&content)
            .unwrap_or_default();

        Ok(activities)
    }

    // 保存用户的活动记录
    async fn save_activities(&self, activities: &[Activity]) -> Result<()> {
        let path = self.get_user_log_path().await?;

        let content = serde_json::to_string_pretty(activities)
            .context("序列化活动记录失败")?;

        fs::write(&path, content).await
            .context("写入活动日志文件失败")?;

        Ok(())
    }

    // 添加新的活动记录
    pub async fn add_activity(&self, activity_type: ActivityType, file_path: &str, file_size: u64) -> Result<()> {
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(file_path)
            .to_string();

        let activity = Activity {
            timestamp: Utc::now(),
            activity_type,
            file_path: file_path.to_string(),
            file_name,
            file_size,
        };

        let mut activities = self.load_activities().await?;

        activities.push(activity);

        // 保留最近 N 条记录
        if activities.len() > self.max_records {
            activities = activities.split_at(activities.len() - self.max_records).1.to_vec();
        }

        self.save_activities(&activities).await
    }

    // 获取最近活动记录
    pub async fn get_recent_activities(
        &self,
        limit: usize,
        activity_type: Option<ActivityType>,
    ) -> Result<ActivityResponse> {
        let activities = self.load_activities().await?;

        let total = activities.len();

        let filtered_activities: Vec<Activity> = if let Some(ref filter_type) = activity_type {
            activities.into_iter()
                .filter(|a| &a.activity_type == filter_type)
                .collect()
        } else {
            activities
        };

        let limit = limit.min(filtered_activities.len());
        let activities: Vec<Activity> = filtered_activities
            .into_iter()
            .rev()
            .take(limit)
            .collect();

        Ok(ActivityResponse {
            user_uuid: self.user_uuid.clone(),
            total,
            limit,
            activity_type: activity_type.map(|t| match t {
                ActivityType::Upload => "upload".to_string(),
                ActivityType::Download => "download".to_string(),
                ActivityType::Access => "access".to_string(),
            }),
            activities,
        })
    }
}

// Tauri 命令

#[tauri::command]
pub async fn get_recent_activities(
    user_uuid: String,
    limit: Option<usize>,
    activity_type: Option<String>,
) -> Result<serde_json::Value, String> {
    tracing::info!("[ACTIVITY] 获取最近活动记录，用户：{}, limit: {:?}, type: {:?}",
        user_uuid, limit, activity_type);

    let limit = limit.unwrap_or(10).min(100);

    let activity_type = activity_type.map(|t| {
        match t.to_lowercase().as_str() {
            "upload" => Ok(ActivityType::Upload),
            "download" => Ok(ActivityType::Download),
            "access" => Ok(ActivityType::Access),
            _ => Err(format!(
                "Invalid activity_type. Must be one of: ['upload', 'download', 'access']"
            ))
        }
    }).transpose()?;

    let manager = ActivityLogManager::new(user_uuid);

    match manager.get_recent_activities(limit, activity_type).await {
        Ok(response) => {
            tracing::info!("[ACTIVITY] 获取成功，返回 {} 条记录", response.activities.len());
            Ok(serde_json::to_value(response)
                .map_err(|e| format!("序列化响应失败：{}", e))?)
        }
        Err(e) => {
            tracing::error!("[ACTIVITY] 获取失败：{}", e);
            Err(format!("获取活动记录失败：{}", e))
        }
    }
}

#[tauri::command]
pub async fn record_upload_activity(
    user_uuid: String,
    file_path: String,
    file_size: u64,
) -> Result<(), String> {
    tracing::info!("[ACTIVITY] 记录上传活动：user={}, path={}, size={}",
        user_uuid, file_path, file_size);

    let manager = ActivityLogManager::new(user_uuid);

    manager.add_activity(ActivityType::Upload, &file_path, file_size).await
        .map_err(|e| format!("记录上传活动失败：{}", e))
}

#[tauri::command]
pub async fn record_download_activity(
    user_uuid: String,
    file_path: String,
    file_size: u64,
) -> Result<(), String> {
    tracing::info!("[ACTIVITY] 记录下载活动：user={}, path={}, size={}",
        user_uuid, file_path, file_size);

    let manager = ActivityLogManager::new(user_uuid);

    manager.add_activity(ActivityType::Download, &file_path, file_size).await
        .map_err(|e| format!("记录下载活动失败：{}", e))
}

#[tauri::command]
pub async fn record_access_activity(
    user_uuid: String,
    file_path: String,
    file_size: u64,
) -> Result<(), String> {
    tracing::info!("[ACTIVITY] 记录访问活动：user={}, path={}, size={}",
        user_uuid, file_path, file_size);

    let manager = ActivityLogManager::new(user_uuid);

    manager.add_activity(ActivityType::Access, &file_path, file_size).await
        .map_err(|e| format!("记录访问活动失败：{}", e))
}

// AI 图片分析命令

#[tauri::command]
pub async fn ai_analysis_hash_status(
    file_path: String,
) -> Result<serde_json::Value, String> {
    use crate::ai_analysis::{
        load_index, get_file_size, get_image_status, 
        get_hash_for_file_path, get_hash_for_filename
    };
    
    tracing::info!("[AI_ANALYSIS] 查询文件哈希状态：path={}", file_path);
    
    let index = match load_index().await {
        Ok(idx) => idx,
        Err(e) => {
            tracing::error!("[AI_ANALYSIS] 加载索引失败：{}", e);
            return Err(format!("加载索引失败：{}", e));
        }
    };
    
    let file_path_str = file_path.as_str();
    
    let hash_result = match get_hash_for_file_path(&index, file_path_str).await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("[AI_ANALYSIS] 获取文件哈希失败：{}", e);
            return Err(format!("获取文件哈希失败：{}", e));
        }
    };
    
    let (file_hash, status_info) = match hash_result {
        Some(hash) => {
            let hash_str = hash.to_string();
            let status = match get_image_status(&index, &hash_str).await {
                Ok(Some(s)) => s,
                Ok(None) => {
                    tracing::warn!("[AI_ANALYSIS] 未找到文件状态：hash={}", hash_str);
                    return Ok(serde_json::json!({
                        "success": false,
                        "detail": "文件未找到"
                    }));
                }
                Err(e) => {
                    tracing::error!("[AI_ANALYSIS] 获取文件状态失败：{}", e);
                    return Err(format!("获取文件状态失败：{}", e));
                }
            };
            (hash_str, status)
        }
        None => {
            tracing::warn!("[AI_ANALYSIS] 文件不在索引中，尝试通过文件名查询：path={}", file_path_str);
            match get_hash_for_filename(&index, file_path_str).await {
                Ok(Some(hash)) => {
                    let status = match get_image_status(&index, &hash).await {
                        Ok(Some(s)) => s,
                        Ok(None) => {
                            tracing::warn!("[AI_ANALYSIS] 未找到文件状态：hash={}", hash);
                            return Ok(serde_json::json!({
                                "success": false,
                                "detail": "文件未找到"
                            }));
                        }
                        Err(e) => {
                            tracing::error!("[AI_ANALYSIS] 获取文件状态失败：{}", e);
                            return Err(format!("获取文件状态失败：{}", e));
                        }
                    };
                    (hash, status)
                }
                Ok(None) => {
                    tracing::warn!("[AI_ANALYSIS] 文件不在索引中：path={}", file_path_str);
                    if file_path_str.starts_with("image_base64_") {
                        tracing::info!("[AI_ANALYSIS] 这是 base64 图片，返回未分析状态");
                        return Ok(serde_json::json!({
                            "success": true,
                            "file_path": file_path,
                            "file_hash": null,
                            "file_size": 0,
                            "analyzed": false,
                            "analysis_info": {
                                "analyzed": false,
                                "analysis_file": null,
                                "last_analyzed_at": null,
                                "file_paths": []
                            },
                            "status": {
                                "file_paths": [],
                                "analyzed": false,
                                "analysis_file": null,
                                "last_analyzed_at": null,
                                "reanalyze_lock": {
                                    "locked": false,
                                    "locked_at": null,
                                    "expires_at": null
                                },
                                "status": "pending",
                                "created_at": null
                            }
                        }));
                    }
                    return Ok(serde_json::json!({
                        "success": false,
                        "detail": "文件不在索引中"
                    }));
                }
                Err(e) => {
                    tracing::error!("[AI_ANALYSIS] 获取文件哈希失败：{}", e);
                    return Err(format!("获取文件哈希失败：{}", e));
                }
            }
        }
    };
    
    let file_size = match get_file_size(&file_path).await {
        Ok(size) => size,
        Err(_) => 0,
    };
    
    tracing::info!("[AI_ANALYSIS] 查询成功：hash={}, analyzed={}", file_hash, status_info.analyzed);
    
    Ok(serde_json::json!({
        "success": true,
        "file_path": file_path,
        "file_hash": file_hash,
        "file_size": file_size,
        "analyzed": status_info.analyzed,
        "analysis_info": {
            "analyzed": status_info.analyzed,
            "analysis_file": status_info.analysis_file,
            "last_analyzed_at": status_info.last_analyzed_at,
            "file_paths": status_info.file_paths
        },
        "status": {
            "file_paths": status_info.file_paths,
            "analyzed": status_info.analyzed,
            "analysis_file": status_info.analysis_file,
            "last_analyzed_at": status_info.last_analyzed_at,
            "reanalyze_lock": status_info.reanalyze_lock,
            "status": status_info.status,
            "created_at": status_info.created_at
        }
    }))
}

#[tauri::command]
pub async fn ai_analysis_reanalyze(
    file_path: String,
) -> Result<serde_json::Value, String> {
    use crate::ai_analysis::{
        load_index, save_index, check_reanalyze_lock, 
        get_hash_for_file_path, get_hash_for_filename, lock_for_reanalyze
    };
    
    tracing::info!("[AI_ANALYSIS] 请求重新解析：path={}", file_path);
    
    let mut index = match load_index().await {
        Ok(idx) => idx,
        Err(e) => {
            tracing::error!("[AI_ANALYSIS] 加载索引失败：{}", e);
            return Err(format!("加载索引失败：{}", e));
        }
    };
    
    let file_path_str = file_path.as_str();
    
    let hash_result = get_hash_for_file_path(&index, file_path_str).await;
    
    let file_hash = match hash_result {
        Ok(Some(hash)) => hash,
        Ok(None) => {
            tracing::warn!("[AI_ANALYSIS] 文件不在索引中，尝试通过文件名查询：path={}", file_path_str);
            let hash_result_by_name = get_hash_for_filename(&index, file_path_str).await;
            match hash_result_by_name {
                Ok(Some(hash)) => hash,
                Ok(None) => {
            tracing::warn!("[AI_ANALYSIS] 文件不在索引中：path={}", file_path_str);
            if file_path_str.starts_with("image_base64_") {
                tracing::info!("[AI_ANALYSIS] 这是 base64 图片，无法重新分析");
                return Err(format!("base64 图片无法重新分析"));
            }
            return Err(format!("文件不在索引中"));
        }
                Err(e) => {
                    tracing::error!("[AI_ANALYSIS] 获取文件哈希失败：{}", e);
                    return Err(format!("获取文件哈希失败：{}", e));
                }
            }
        }
        Err(e) => {
            tracing::error!("[AI_ANALYSIS] 获取文件哈希失败：{}", e);
            return Err(format!("获取文件哈希失败：{}", e));
        }
    };
    
    if !index.images.get(&file_hash).map(|s| s.analyzed).unwrap_or(false) {
        tracing::warn!("[AI_ANALYSIS] 图片尚未完成首次解析：hash={}", file_hash);
        return Err(format!("图片尚未完成首次解析"));
    }
    
    if let Err(e) = check_reanalyze_lock(&index, &file_hash).await {
        let error_msg = e.to_string();
        let remaining = error_msg.split("请等待 ")
            .nth(1)
            .and_then(|s| s.split(" 秒").next())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        
        return Err(format!("解析锁未过期，请等待 {} 秒", remaining));
    }
    
    if let Err(e) = lock_for_reanalyze(&mut index, &file_hash).await {
        tracing::error!("[AI_ANALYSIS] 锁定失败：{}", e);
        return Err(format!("锁定失败：{}", e));
    }
    
    if let Err(e) = save_index(&index).await {
        tracing::error!("[AI_ANALYSIS] 保存索引失败：{}", e);
        return Err(format!("保存索引失败：{}", e));
    }
    
    tracing::info!("[AI_ANALYSIS] 重新解析已触发：hash={}", file_hash);
    
    Ok(serde_json::json!({
        "success": true,
        "message": "已触发重新解析，请稍后查询结果",
        "file_path": file_path,
        "file_hash": file_hash,
        "reanalyze_lock_seconds": 30,
        "note": "30 秒内不能再次请求重新解析"
    }))
}

// 笔记图片上传命令
#[tauri::command]
pub async fn note_image_upload(
    image_data: String,
) -> Result<serde_json::Value, String> {
    use crate::note_image::save_base64_image;
    
    tracing::info!("[NOTE_IMAGE] 上传图片请求");
    
    // 获取用户UUID
    let user_uuid = get_user_uuid_from_device().await?;
    
    // 保存图片
    let filename = match save_base64_image(&user_uuid, &image_data).await {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("[NOTE_IMAGE] 保存图片失败：{}", e);
            return Err(format!("保存图片失败：{}", e));
        }
    };
    
    tracing::info!("[NOTE_IMAGE] 图片上传成功：{}", filename);
    
    Ok(serde_json::json!({
        "success": true,
        "filename": filename,
        "message": "图片上传成功"
    }))
}

// 获取用户UUID的辅助函数
async fn get_user_uuid_from_device() -> Result<String, String> {
    // 使用 OnceLock 确保只初始化一次
    static CPEN_DEVICE_MANAGER: OnceLock<Mutex<CpenDeviceManager>> = OnceLock::new();
    
    let manager = CPEN_DEVICE_MANAGER.get_or_init(|| Mutex::new(CpenDeviceManager::new()));
    let mut manager = manager.lock().await;
    
    match manager.get_user_uuid().await {
        Ok(user_uuid) => {
            tracing::info!("[NOTE_IMAGE] 用户UUID获取成功: {}", user_uuid);
            Ok(user_uuid)
        }
        Err(e) => {
            tracing::error!("[NOTE_IMAGE] 用户UUID获取失败: {}", e);
            Err(format!("获取用户UUID失败: {}", e))
        }
    }
}

// 获取用户图片列表命令
#[tauri::command]
pub async fn note_image_list() -> Result<serde_json::Value, String> {
    tracing::info!("[NOTE_IMAGE] 获取图片列表请求");
    
    // 获取用户UUID
    let user_uuid = get_user_uuid_from_device().await?;
    
    // 获取图片列表
    let result = match get_user_images(&user_uuid).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("[NOTE_IMAGE] 获取图片列表失败：{}", e);
            return Err(format!("获取图片列表失败：{}", e));
        }
    };
    
    tracing::info!("[NOTE_IMAGE] 获取图片列表成功");
    
    Ok(result)
}

// 删除用户图片命令
#[tauri::command]
pub async fn note_image_delete(
    filename: String,
) -> Result<serde_json::Value, String> {
    tracing::info!("[NOTE_IMAGE] 删除图片请求：{}", filename);
    
    // 获取用户UUID
    let user_uuid = get_user_uuid_from_device().await?;
    
    // 删除图片
    let deleted = match delete_user_image(&user_uuid, &filename).await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("[NOTE_IMAGE] 删除图片失败：{}", e);
            return Err(format!("删除图片失败：{}", e));
        }
    };
    
    if deleted {
        tracing::info!("[NOTE_IMAGE] 图片删除成功：{}", filename);
        Ok(serde_json::json!({
            "success": true,
            "message": "图片删除成功"
        }))
    } else {
        tracing::warn!("[NOTE_IMAGE] 图片不存在：{}", filename);
        Ok(serde_json::json!({
            "success": false,
            "message": "图片不存在"
        }))
    }
}
