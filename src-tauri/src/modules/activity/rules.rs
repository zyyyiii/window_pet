use super::types::{MatchTarget, MatchType, ScoringRule, StateScores};

/// 规则管理器
pub struct RuleManager {
    rules: Vec<ScoringRule>,
}

impl RuleManager {
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
        }
    }

    /// 获取所有规则
    pub fn get_rules(&self) -> &[ScoringRule] {
        &self.rules
    }

    /// 添加规则
    pub fn add_rule(&mut self, rule: ScoringRule) {
        self.rules.push(rule);
    }

    /// 删除规则
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        let len_before = self.rules.len();
        self.rules.retain(|r| r.id != rule_id);
        self.rules.len() < len_before
    }

    /// 启用/禁用规则
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) -> bool {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            rule.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// 获取匹配的规则
    pub fn get_matching_rules(&self, snapshot: &super::types::ActivitySnapshot) -> Vec<&ScoringRule> {
        self.rules
            .iter()
            .filter(|rule| rule.matches(snapshot))
            .collect()
    }

    /// 默认规则表
    fn default_rules() -> Vec<ScoringRule> {
        vec![
            // ===== IDE / 编辑器 =====
            ScoringRule {
                id: "vscode".to_string(),
                name: "VS Code".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "code".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 3.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "visual_studio".to_string(),
                name: "Visual Studio".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "devenv".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 3.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "jetbrains".to_string(),
                name: "JetBrains IDE".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "idea64".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 3.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "pycharm".to_string(),
                name: "PyCharm".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "pycharm64".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 3.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "webstorm".to_string(),
                name: "WebStorm".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "webstorm64".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 3.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "sublime".to_string(),
                name: "Sublime Text".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "sublime_text".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 2.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "notepadpp".to_string(),
                name: "Notepad++".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "notepad++".to_string(),
                scores: StateScores {
                    studying: 1.0,
                    coding: 1.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 笔记 / 文档 =====
            ScoringRule {
                id: "typora".to_string(),
                name: "Typora".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "typora".to_string(),
                scores: StateScores {
                    studying: 3.0,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "notion".to_string(),
                name: "Notion".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "notion".to_string(),
                scores: StateScores {
                    studying: 2.0,
                    coding: 1.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "obsidian".to_string(),
                name: "Obsidian".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "obsidian".to_string(),
                scores: StateScores {
                    studying: 2.0,
                    coding: 1.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "word".to_string(),
                name: "Microsoft Word".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "winword".to_string(),
                scores: StateScores {
                    studying: 3.0,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "excel".to_string(),
                name: "Microsoft Excel".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "excel".to_string(),
                scores: StateScores {
                    studying: 2.0,
                    coding: 1.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "powerpoint".to_string(),
                name: "Microsoft PowerPoint".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "powerpnt".to_string(),
                scores: StateScores {
                    studying: 2.0,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "wps".to_string(),
                name: "WPS Office".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "wps".to_string(),
                scores: StateScores {
                    studying: 2.5,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== PDF 阅读器 =====
            ScoringRule {
                id: "acrobat".to_string(),
                name: "Adobe Acrobat".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "acrobat".to_string(),
                scores: StateScores {
                    studying: 2.5,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "sumatra".to_string(),
                name: "SumatraPDF".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "sumatra".to_string(),
                scores: StateScores {
                    studying: 2.5,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "foxit".to_string(),
                name: "Foxit Reader".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "foxit".to_string(),
                scores: StateScores {
                    studying: 2.5,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 浏览器（中性，因为可能是学习或娱乐） =====
            ScoringRule {
                id: "chrome".to_string(),
                name: "Chrome".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "chrome".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 0.5,
                    entertainment: 0.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "edge".to_string(),
                name: "Microsoft Edge".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "msedge".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 0.5,
                    entertainment: 0.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "firefox".to_string(),
                name: "Firefox".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "firefox".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 0.5,
                    entertainment: 0.5,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 游戏平台 =====
            ScoringRule {
                id: "steam".to_string(),
                name: "Steam".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "steam".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 3.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "wegame".to_string(),
                name: "WeGame".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "wegame".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 3.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "epic_games".to_string(),
                name: "Epic Games".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "epicgames".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 3.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 音乐 =====
            ScoringRule {
                id: "netease_music".to_string(),
                name: "网易云音乐".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "cloudmusic".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 1.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "qq_music".to_string(),
                name: "QQ音乐".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "qqmusic".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 1.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "kugou".to_string(),
                name: "酷狗音乐".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "kugou".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 1.5,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 视频 / 直播 =====
            ScoringRule {
                id: "bilibili_live".to_string(),
                name: "B站直播".to_string(),
                target: MatchTarget::WindowTitle,
                match_type: MatchType::Contains,
                pattern: "哔哩哔哩".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "douyu".to_string(),
                name: "斗鱼直播".to_string(),
                target: MatchTarget::WindowTitle,
                match_type: MatchType::Contains,
                pattern: "斗鱼".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "huya".to_string(),
                name: "虎牙直播".to_string(),
                target: MatchTarget::WindowTitle,
                match_type: MatchType::Contains,
                pattern: "虎牙".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "douyin".to_string(),
                name: "抖音".to_string(),
                target: MatchTarget::WindowTitle,
                match_type: MatchType::Contains,
                pattern: "抖音".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "tencent_video".to_string(),
                name: "腾讯视频".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "qqlive".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "iqiyi".to_string(),
                name: "爱奇艺".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "iqiyi".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "youku".to_string(),
                name: "优酷".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "youku".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "potplayer".to_string(),
                name: "PotPlayer".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "potplayer".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "vlc".to_string(),
                name: "VLC".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "vlc".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 2.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 终端 / 命令行 =====
            ScoringRule {
                id: "terminal".to_string(),
                name: "Windows Terminal".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "windowsterminal".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 2.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "cmd".to_string(),
                name: "CMD".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "cmd".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 1.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "powershell".to_string(),
                name: "PowerShell".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "powershell".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 1.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "git_bash".to_string(),
                name: "Git Bash".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "git-bash".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 1.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 学习平台 =====
            ScoringRule {
                id: "chaoxing".to_string(),
                name: "学习通".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "chaoxing".to_string(),
                scores: StateScores {
                    studying: 3.0,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "zhidao".to_string(),
                name: "知到".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "zhidao".to_string(),
                scores: StateScores {
                    studying: 3.0,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "icourse163".to_string(),
                name: "中国大学MOOC".to_string(),
                target: MatchTarget::WindowTitle,
                match_type: MatchType::Contains,
                pattern: "icourse163".to_string(),
                scores: StateScores {
                    studying: 3.0,
                    coding: 0.0,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },

            // ===== 通讯软件（低分，可能是在摸鱼） =====
            ScoringRule {
                id: "wechat".to_string(),
                name: "微信".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "wechat".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 0.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "qq".to_string(),
                name: "QQ".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Exact,
                pattern: "qq".to_string(),
                scores: StateScores {
                    studying: 0.0,
                    coding: 0.0,
                    entertainment: 0.5,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "dingtalk".to_string(),
                name: "钉钉".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "dingtalk".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 0.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
            ScoringRule {
                id: "feishu".to_string(),
                name: "飞书".to_string(),
                target: MatchTarget::ProcessName,
                match_type: MatchType::Contains,
                pattern: "feishu".to_string(),
                scores: StateScores {
                    studying: 0.5,
                    coding: 0.5,
                    entertainment: 0.0,
                    idle: 0.0,
                },
                enabled: true,
            },
        ]
    }
}

impl Default for RuleManager {
    fn default() -> Self {
        Self::new()
    }
}
