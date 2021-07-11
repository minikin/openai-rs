use serde::{Deserialize}

/// The result of a classification request.
///
/// Given a query and a set of labeled examples,
/// the model will predict the most likely label for the query.
/// Useful as a drop-in replacement for any ML classification or text-to-label task.
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Classification {
    /// The completion for the classification.
    pub completion: String,

    /// The classification label assigned by the model.
    pub label: String,

    /// The engine used to perform classification.
    pub engine: EngineId,

    /// The engine used for searching.
    pub search_engine: EngineId,

    pub example: Example,
}

 /// A classification example.
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Example {

    /// The source of the example.
    pub  source: Source

    /// The classification label for the example.
    pub  label: String

    /// The text of the example.
    pub  text: String
}

/// The source of an example
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Source {
    /// A document identified by its position.
    Document(u64)

    /// A file
    File(File)
}