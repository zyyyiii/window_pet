use rand::seq::SliceRandom;
use rand::thread_rng;

use super::types::{StudyDialogueType, StudyMode};

/// 学习对话内容库
#[derive(Debug)]
pub struct StudyDialogue {
    /// 学习提醒消息
    study_reminders: Vec<String>,
    /// 休息提醒消息
    break_reminders: Vec<String>,
    /// 鼓励消息
    encouragements: Vec<String>,
    /// 进度询问消息
    progress_questions: Vec<String>,
    /// 单词测试提示
    word_test_prompts: Vec<String>,
}

impl StudyDialogue {
    pub fn new() -> Self {
        Self {
            study_reminders: vec![
                "今天的线代进度怎么样？".to_string(),
                "要不要做一道题？".to_string(),
                "已经学习30分钟了，继续加油！".to_string(),
                "专注学习，效率更高哦~".to_string(),
                "学习使我快乐！".to_string(),
                "来，我们一起学习吧！".to_string(),
                "今天的任务完成了多少？".to_string(),
                "休息够了，该学习啦~".to_string(),
            ],
            break_reminders: vec![
                "休息一下吧，别太累了~".to_string(),
                "记得喝水哦！".to_string(),
                "站起来活动活动~".to_string(),
                "看看远处，保护眼睛~".to_string(),
                "深呼吸，放松一下~".to_string(),
                "学习很重要，休息也很重要！".to_string(),
                "来，伸个懒腰~".to_string(),
                "休息是为了更好地学习！".to_string(),
            ],
            encouragements: vec![
                "你已经很努力了！".to_string(),
                "坚持就是胜利！".to_string(),
                "你真棒！继续加油！".to_string(),
                "学习的路上，我一直陪着你~".to_string(),
                "每一点进步都值得庆祝！".to_string(),
                "相信自己，你可以的！".to_string(),
                "今天的你比昨天更优秀！".to_string(),
                "知识就是力量！".to_string(),
            ],
            progress_questions: vec![
                "今天学了多久了？".to_string(),
                "学到什么有趣的内容了吗？".to_string(),
                "有没有遇到什么困难？".to_string(),
                "今天的计划完成了多少？".to_string(),
                "需要我帮你复习吗？".to_string(),
                "有没有什么想分享的？".to_string(),
            ],
            word_test_prompts: vec![
                "来测试一下你的单词吧！".to_string(),
                "准备好了吗？单词测试开始！".to_string(),
                "考考你，这个单词是什么意思？".to_string(),
                "单词时间到！".to_string(),
                "来，我们一起复习单词~".to_string(),
            ],
        }
    }

    /// 获取随机学习提醒
    pub fn get_study_reminder(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.study_reminders.choose(&mut rng).cloned()
    }

    /// 获取随机休息提醒
    pub fn get_break_reminder(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.break_reminders.choose(&mut rng).cloned()
    }

    /// 获取随机鼓励消息
    pub fn get_encouragement(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.encouragements.choose(&mut rng).cloned()
    }

    /// 获取随机进度询问
    pub fn get_progress_question(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.progress_questions.choose(&mut rng).cloned()
    }

    /// 获取随机单词测试提示
    pub fn get_word_test_prompt(&self) -> Option<String> {
        let mut rng = thread_rng();
        self.word_test_prompts.choose(&mut rng).cloned()
    }

    /// 根据对话类型获取消息
    pub fn get_dialogue(&self, dialogue_type: &StudyDialogueType) -> Option<String> {
        match dialogue_type {
            StudyDialogueType::StudyReminder => self.get_study_reminder(),
            StudyDialogueType::BreakReminder => self.get_break_reminder(),
            StudyDialogueType::WordTest => self.get_word_test_prompt(),
            StudyDialogueType::Encouragement => self.get_encouragement(),
            StudyDialogueType::Progress => self.get_progress_question(),
        }
    }

    /// 根据模式获取合适的对话类型
    pub fn get_random_dialogue_type(&self, mode: &StudyMode) -> Option<StudyDialogueType> {
        let mut rng = thread_rng();

        match mode {
            StudyMode::Study => {
                let types = vec![
                    StudyDialogueType::StudyReminder,
                    StudyDialogueType::Encouragement,
                    StudyDialogueType::Progress,
                ];
                types.choose(&mut rng).cloned()
            }
            StudyMode::Break => {
                let types = vec![
                    StudyDialogueType::BreakReminder,
                    StudyDialogueType::Encouragement,
                ];
                types.choose(&mut rng).cloned()
            }
            StudyMode::Normal => {
                // 普通模式下偶尔也会有学习相关对话
                let types = vec![
                    StudyDialogueType::Encouragement,
                    StudyDialogueType::Progress,
                ];
                types.choose(&mut rng).cloned()
            }
        }
    }

    /// 添加自定义学习提醒
    pub fn add_study_reminder(&mut self, message: String) {
        self.study_reminders.push(message);
    }

    /// 添加自定义休息提醒
    pub fn add_break_reminder(&mut self, message: String) {
        self.break_reminders.push(message);
    }

    /// 添加自定义鼓励消息
    pub fn add_encouragement(&mut self, message: String) {
        self.encouragements.push(message);
    }
}

impl Default for StudyDialogue {
    fn default() -> Self {
        Self::new()
    }
}
