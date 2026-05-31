use chrono::Utc;
use std::sync::Mutex;

use super::scorer::ActivityScorer;
use super::types::{ActivityAnalysis, ActivitySnapshot, ActivityState, StateSmoother};

/// 用户活动检测器
pub struct UserActivityDetector {
    /// 评分引擎
    scorer: ActivityScorer,
    /// 状态平滑器
    smoother: StateSmoother,
    /// 最后一次分析结果
    last_analysis: Option<ActivityAnalysis>,
    /// 是否启用
    enabled: bool,
}

impl UserActivityDetector {
    pub fn new() -> Self {
        Self {
            scorer: ActivityScorer::new(),
            smoother: StateSmoother::new(3),
            last_analysis: None,
            enabled: true,
        }
    }

    /// 执行一次检测
    pub fn detect(&mut self) -> ActivityAnalysis {
        if !self.enabled {
            return ActivityAnalysis::default();
        }

        // 1. 采集原始数据
        let snapshot = self.probe_snapshot();

        // 2. 推断状态
        let raw_state = self.scorer.infer_state(&snapshot);

        // 3. 平滑处理
        let smoothed_state = self.smoother.smooth(raw_state);

        // 4. 计算分数
        let scores = self.scorer.score(&snapshot);

        let analysis = ActivityAnalysis {
            state: smoothed_state,
            scores,
            snapshot,
        };

        self.last_analysis = Some(analysis.clone());
        analysis
    }

    /// 采集活动快照
    fn probe_snapshot(&self) -> ActivitySnapshot {
        let window_title = Self::get_foreground_window_title();
        let process_name = Self::get_foreground_process_name();
        let idle_seconds = Self::get_idle_seconds();

        ActivitySnapshot {
            window_title,
            process_name,
            idle_seconds,
            timestamp: Utc::now().timestamp(),
        }
    }

    /// 获取前台窗口标题
    #[cfg(target_os = "windows")]
    fn get_foreground_window_title() -> String {
        use windows::Win32::UI::WindowsAndMessaging::{
            GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
        };

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_invalid() {
                return String::new();
            }

            let length = GetWindowTextLengthW(hwnd);
            if length == 0 {
                return String::new();
            }

            let mut buffer = vec![0u16; (length + 1) as usize];
            let written = GetWindowTextW(hwnd, &mut buffer);

            if written > 0 {
                String::from_utf16_lossy(&buffer[..written as usize])
            } else {
                String::new()
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_foreground_window_title() -> String {
        String::new()
    }

    /// 获取前台进程名称
    #[cfg(target_os = "windows")]
    fn get_foreground_process_name() -> String {
        use windows::Win32::Foundation::CloseHandle;
        use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
        use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
        use windows::Win32::UI::WindowsAndMessaging::{
            GetForegroundWindow, GetWindowThreadProcessId,
        };

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_invalid() {
                return String::new();
            }

            let mut process_id: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process_id));

            if process_id == 0 {
                return String::new();
            }

            let handle = match OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                false,
                process_id,
            ) {
                Ok(h) => h,
                Err(_) => return String::new(),
            };

            let mut buffer = vec![0u16; 260];
            let length = GetModuleBaseNameW(handle, None, &mut buffer);
            let _ = CloseHandle(handle);

            if length > 0 {
                let name = String::from_utf16_lossy(&buffer[..length as usize]);
                // 去掉 .exe 扩展名并转小写
                name.trim_end_matches(".exe")
                    .trim_end_matches(".EXE")
                    .to_lowercase()
            } else {
                String::new()
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_foreground_process_name() -> String {
        String::new()
    }

    /// 获取系统空闲时间（秒）
    #[cfg(target_os = "windows")]
    fn get_idle_seconds() -> u64 {
        use windows::Win32::System::SystemInformation::GetTickCount;
        use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

        unsafe {
            let mut last_input = LASTINPUTINFO {
                cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                dwTime: 0,
            };

            if GetLastInputInfo(&mut last_input).as_bool() {
                let current_tick = GetTickCount();
                let idle_ticks = current_tick.saturating_sub(last_input.dwTime);
                idle_ticks as u64 / 1000
            } else {
                0
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_idle_seconds() -> u64 {
        0
    }

    /// 获取最后一次分析结果
    pub fn get_last_analysis(&self) -> Option<&ActivityAnalysis> {
        self.last_analysis.as_ref()
    }

    /// 获取评分引擎的可变引用
    pub fn get_scorer_mut(&mut self) -> &mut ActivityScorer {
        &mut self.scorer
    }

    /// 启用/禁用检测
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 重置平滑器
    pub fn reset_smoother(&mut self) {
        self.smoother.reset();
    }
}

impl Default for UserActivityDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// 线程安全的检测器包装
pub struct SharedActivityDetector {
    detector: Mutex<UserActivityDetector>,
}

impl SharedActivityDetector {
    pub fn new() -> Self {
        Self {
            detector: Mutex::new(UserActivityDetector::new()),
        }
    }

    pub fn detect(&self) -> Option<ActivityAnalysis> {
        self.detector.lock().ok().map(|mut d| d.detect())
    }

    pub fn get_last_analysis(&self) -> Option<ActivityAnalysis> {
        self.detector
            .lock()
            .ok()
            .and_then(|d| d.get_last_analysis().cloned())
    }

    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut d) = self.detector.lock() {
            d.set_enabled(enabled);
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.detector
            .lock()
            .ok()
            .map(|d| d.is_enabled())
            .unwrap_or(false)
    }

    pub fn get_rules(&self) -> Option<Vec<super::types::ScoringRule>> {
        self.detector
            .lock()
            .ok()
            .map(|mut d| d.get_scorer_mut().get_rule_manager().get_rules().to_vec())
    }

    pub fn add_rule(&self, rule: super::types::ScoringRule) -> bool {
        self.detector
            .lock()
            .ok()
            .map(|mut d| {
                d.get_scorer_mut().get_rule_manager_mut().add_rule(rule);
                true
            })
            .unwrap_or(false)
    }

    pub fn remove_rule(&self, rule_id: &str) -> bool {
        self.detector
            .lock()
            .ok()
            .map(|mut d| {
                d.get_scorer_mut()
                    .get_rule_manager_mut()
                    .remove_rule(rule_id)
            })
            .unwrap_or(false)
    }
}

impl Default for SharedActivityDetector {
    fn default() -> Self {
        Self::new()
    }
}
