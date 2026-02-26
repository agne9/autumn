use std::collections::HashSet;
use std::sync::Arc;

use autumn_database::Database;
use autumn_llm::LlmService;
use tokio::sync::RwLock;

pub type Error = anyhow::Error;

/// Set of message IDs to suppress from user-log recording.
/// Used by purge and word-filter to prevent logging bot-initiated deletions.
pub type SuppressedDeletes = Arc<RwLock<HashSet<u64>>>;

#[derive(Clone, Debug)]
pub struct Data {
    pub db: Database,
    pub llm: Option<LlmService>,
    pub suppressed_deletes: SuppressedDeletes,
}

pub type Context<'a> = poise::Context<'a, Data, Error>;
