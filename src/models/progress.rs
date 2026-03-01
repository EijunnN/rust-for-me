#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserProgress {
    pub id: String,
    pub category: String,
    pub status: ProgressStatus,
    pub score: i32,
    pub attempts: i32,
    pub completed_at: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressStatus {
    Locked,
    Available,
    InProgress,
    Completed,
}
