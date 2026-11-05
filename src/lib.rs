#[derive(Debug, Clone, PartialEq)]
pub enum TaskIntent {
    Refactor,
    GenerateTests,
    OptimizePerformance,
    FixBug,
}

pub struct DeepSeekRequest {
    pub prompt: String,
    pub intent: TaskIntent,
    pub max_tokens: usize,
}

impl DeepSeekRequest {
    pub fn new(prompt: impl Into<String>, intent: TaskIntent) -> Self {
        Self {
            prompt: prompt.into(),
            intent,
            max_tokens: 4096,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepseek_request() {
        // Arrange & Act
        let req = DeepSeekRequest::new("Implement LRU cache", TaskIntent::OptimizePerformance);
        // Assert
        assert_eq!(req.intent, TaskIntent::OptimizePerformance);
        assert_eq!(req.max_tokens, 4096);
    }
}
