use serde::Deserialize;
use std::fmt;
///  Engines describe and provide access to the various models available in the API.
///
///  OpenAI’s API provides access to several different engines - Ada, Babbage, Curie and Davinci.
///
///  While Davinci is generally the most capable engine,
///  the other engines can perform certain tasks extremely well and in some cases significantly faster.
///  The other engines have cost advantages.
///  For example, Curie can perform many of the same tasks as Davinci,
///  but faster and for 1/10th the cost.
///  We encourage developers to experiment with using the other models
///  and try to find the one that’s the most efficient for your application.
///
///  https://beta.openai.com/docs/engines
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Engine {
    pub id: EngineId,
    pub owner: String,
    pub ready: bool,
    created: Option<u64>,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {:?})",
            self.id, self.owner, self.ready, self.created,
        )
    }
}

/// A unique identifier for the engine.
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum EngineId {
    //  Ada is usually the fastest model and can perform tasks like parsing text, address correction
    //  and certain kinds of classification tasks that don’t require too much nuance.
    //  Ada’s performance can often be improved by providing more context.
    //
    //  Good at: Parsing text, simple classification, address correction, keywords
    //
    //  - Note: Any task performed by a faster model like Ada
    //  can be performed by a more powerful model like Curie or Davinci
    Ada,
    // Babbage can perform straightforward tasks like simple classification.
    // It’s also quite capable when it comes to Semantic Search ranking
    // how well documents match up with search queries.
    //
    // Good at: Moderate classification, semantic search classification
    Babbage,
    /// Curie is extremely powerful, yet very fast.
    /// While Davinci is stronger when it comes to analyzing complicated text,
    /// Curie is quite capable for many nuanced tasks like sentiment classification and summarization.
    /// Curie is also quite good at answering questions
    /// and performing Q&A and as a general service chatbot.
    Curie,
    // Davinci is the most capable engine and can perform any task the other models can perform
    // and often with less instruction.
    // For applications requiring a lot of understanding of the content,
    // like summarization for a specific audience and content creative generation,
    // Davinci is going to produce the best results.
    // The trade-off with Davinci is that it costs more to use per API call
    // and other engines are faster.
    //
    // Another area where Davinci shines is in understanding the intent of text.
    // Davinci is quite good at solving many kinds of logic problems
    // and explaining the motives of characters.
    // Davinci has been able to solve some of the most challenging AI problems
    // involving cause and effect.
    //
    // Good at: Language translation, complex classification, text sentiment, summarization
    Davinci,
    Other(String),
}

impl fmt::Display for EngineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self,)
    }
}
