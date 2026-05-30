use super::types::GrowthSystem;
use serde_json;
use std::fs;
use std::path::PathBuf;

/// 存储服务
pub struct GrowthStorage {
    file_path: PathBuf,
}

impl GrowthStorage {
    /// 创建存储服务实例
    pub fn new() -> Result<Self, String> {
        let config_dir = dirs::config_dir()
            .ok_or("无法获取配置目录")?;
        
        let app_dir = config_dir.join("CodeWindowPet");
        
        // 确保目录存在
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }
        
        Ok(Self {
            file_path: app_dir.join("growth.json"),
        })
    }

    /// 加载成长数据
    pub fn load(&self) -> Result<GrowthSystem, String> {
        if !self.file_path.exists() {
            // 文件不存在，返回默认值
            return Ok(GrowthSystem::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        
        let data: GrowthSystem = serde_json::from_str(&content)
            .map_err(|e| format!("解析 JSON 失败: {}", e))?;
        
        Ok(data)
    }

    /// 保存成长数据
    pub fn save(&self, data: &GrowthSystem) -> Result<(), String> {
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| format!("序列化 JSON 失败: {}", e))?;
        
        fs::write(&self.file_path, content)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        
        Ok(())
    }

    /// 获取文件路径
    pub fn get_path(&self) -> &PathBuf {
        &self.file_path
    }
}

impl Default for GrowthStorage {
    fn default() -> Self {
        Self::new().expect("无法创建存储服务")
    }
}
