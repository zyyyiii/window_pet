pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> ParsedInput {
        let lower = input.to_lowercase();
        
        if lower.contains("feed") || lower.contains("hungry") {
            ParsedInput::Feed
        } else if lower.contains("play") || lower.contains("game") {
            ParsedInput::Play
        } else if lower.contains("hello") || lower.contains("hi") {
            ParsedInput::Greeting
        } else if lower.contains("how are you") || lower.contains("status") {
            ParsedInput::StatusQuery
        } else {
            ParsedInput::Unknown(input.to_string())
        }
    }
}

pub enum ParsedInput {
    Feed,
    Play,
    Greeting,
    StatusQuery,
    Unknown(String),
}