#[derive(Clone, Debug)]
pub struct CaseSummary {
    pub case_number: u64,
    pub case_code: String,
    pub action_case_number: u64,
    pub target_user_id: Option<u64>,
    pub moderator_user_id: u64,
    pub action: String,
    pub reason: String,
    pub duration_seconds: Option<u64>,
    pub created_at: u64,
}

#[derive(Clone, Debug)]
pub struct ModerationCase {
    pub id: u64,
    pub case_number: u64,
    pub case_code: String,
    pub action_case_number: u64,
    pub guild_id: u64,
    pub target_user_id: Option<u64>,
    pub moderator_user_id: u64,
    pub action: String,
    pub reason: String,
    pub status: String,
    pub duration_seconds: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug)]
pub struct CaseEvent {
    pub event_type: String,
    pub actor_user_id: u64,
    pub old_reason: Option<String>,
    pub new_reason: Option<String>,
    pub note: Option<String>,
    pub created_at: u64,
}
