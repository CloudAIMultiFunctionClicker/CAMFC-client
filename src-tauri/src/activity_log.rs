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
