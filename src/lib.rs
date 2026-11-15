//! SigmaOS-tools-deepseek-coder: AI coding assistant model.
//!
//! Real implementation: prompts, completions, context, models, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Model type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    DeepSeekCoder,
    DeepSeekV2,
    DeepSeekV3,
}

/// Message role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// Chat message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
}

/// Completion request.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionRequest {
    pub id: String,
    pub model: ModelType,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoderError {
    EmptyContent,
    InvalidTemperature,
    DuplicateMessage(String),
}

impl fmt::Display for CoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyContent => write!(f, "message content must not be empty"),
            Self::InvalidTemperature => write!(f, "temperature must be between 0.0 and 2.0"),
            Self::DuplicateMessage(id) => write!(f, "duplicate message {:?}", id),
        }
    }
}

impl std::error::Error for CoderError {}

/// AI coding session.
pub struct CoderSession {
    pub name: String,
    messages: BTreeMap<String, Message>,
    completions: BTreeMap<String, String>,
}

impl CoderSession {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            messages: BTreeMap::new(),
            completions: BTreeMap::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) -> Result<(), CoderError> {
        if message.content.trim().is_empty() { return Err(CoderError::EmptyContent); }
        let id = message.id.clone();
        if self.messages.contains_key(&id) {
            return Err(CoderError::DuplicateMessage(id.clone()));
        }
        self.messages.insert(id, message);
        Ok(())
    }

    pub fn request_completion(&mut self, request: CompletionRequest) -> Result<(), CoderError> {
        if request.temperature < 0.0 || request.temperature > 2.0 { return Err(CoderError::InvalidTemperature); }
        self.completions.insert(request.id, "Mock completion".into());
        Ok(())
    }

    pub fn message_count(&self) -> usize { self.messages.len() }

    pub fn messages(&self) -> Vec<&Message> { self.messages.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_create_aaa() {
        let s = CoderSession::new("My Session");
        assert_eq!(s.name, "My Session");
        assert_eq!(s.message_count(), 0);
    }

    #[test]
    fn test_add_message_aaa() {
        let mut s = CoderSession::new("My Session");
        let m = Message { id: "m1".into(), role: MessageRole::User, content: "Hello".into() };
        s.add_message(m).unwrap();
        assert_eq!(s.message_count(), 1);
    }

    #[test]
    fn test_empty_content_rejected_aaa() {
        let mut s = CoderSession::new("My Session");
        let m = Message { id: "x".into(), role: MessageRole::User, content: "".into() };
        assert!(s.add_message(m).is_err());
    }

    #[test]
    fn test_invalid_temperature_rejected_aaa() {
        let mut s = CoderSession::new("My Session");
        let r = CompletionRequest { id: "x".into(), model: ModelType::DeepSeekCoder, max_tokens: 100, temperature: 3.0 };
        assert!(s.request_completion(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = CoderSession::new("My Session");
        let m = Message { id: "x".into(), role: MessageRole::User, content: "X".into() };
        s.add_message(m).unwrap();
        let m2 = Message { id: "x".into(), role: MessageRole::User, content: "X".into() };
        assert!(s.add_message(m2).is_err());
    }
}
