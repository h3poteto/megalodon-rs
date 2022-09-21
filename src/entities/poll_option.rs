#[derive(Debug, Clone)]
pub struct PollOption {
    pub title: String,
    pub votes_count: Option<u32>,
}
