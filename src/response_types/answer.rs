use std::{collections::HashMap, fmt::Display};
use serde::{Deserialize, Serialize}

/// The result of a question answering request.
///
/// Given a question, a set of documents, and some examples,
/// the API generates an answer to the question
/// based on the information in the set of documents.
/// This is useful for question-answering applications on sources of truth,
/// like company documentation or a knowledge base.
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Answer {
     /// The completion for the answer.
    pub completion: String,

    /// The possible answers.
    pub answers: Vec<String>,

    /// The engine used to answer the question
    pub engine: EngineId,

    /// The engine used for searching.
    pub search_engine: EngineId,

    /// The documents selected for each answer.
    pub selectedDocuments: HashMap<i64, String>,
}