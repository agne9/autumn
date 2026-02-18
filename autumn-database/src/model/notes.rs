#[derive(Clone, Debug)]
pub struct UserNote {
    pub id: u64,
    pub guild_id: u64,
    pub target_user_id: u64,
    pub author_user_id: u64,
    pub content: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
