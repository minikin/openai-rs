use serde::Deserialize;

use crate::supporting_types::EngineId;

/// The result of a completion request.
///
/// Given a prompt, the model will return one or more predicted completions,
/// and can also return the probabilities of alternative tokens at each position.
#[derive(Deserialize, Debug, Clone)]
pub struct Completion {
    /// A unique identifier for the completion.
    pub id: String,

    /// The completion choices.
    pub choices: Vec<Choice>,

    /// Unix timestamp when the completion was generated.
    pub created: u64,

    /// The engine used to generate the completion.
    pub engine: EngineId,
}

/// A completion choice.
#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    /// The text of the completion choice.
    pub text: String,

    /// The index of the completion choice.
    pub index: u64,

    /// The reason why the completion finished.
    pub finish_reason: FinishReason,
}

/// The reason why the completion finished.
#[derive(Deserialize, Debug, Clone)]
pub enum FinishReason {
    /// The completion finished because it reached a maximum token limit.
    Length,

    /// The completion finished because it encountered a stop word.
    Stop,
}
