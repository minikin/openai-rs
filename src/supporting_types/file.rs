use serde::Deserialize;

// An uploaded file.
//
// Files are used to upload documents that can be used across features like
// Answers, Search, and Classifications.
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
  /// A unique identifier for the file.
  pub id: String,

  // The filename.
  pub filename: String,

  /// The size in bytes.
  pub size: u64,

  pub created: u64,
}

// An uploaded file.
//
// Files are used to upload documents that can be used across features like
// Answers, Search, and Classifications.
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Purpose {
  /// The purpose of the file.
  Search,

  /// Answering questions.
  Answers,

  /// Classification.
  Classifications,
}
