use super::lesson::{ContentBlock, I18nText};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: String,
    pub title: I18nText,
    pub description: I18nText,
    pub difficulty: String,
    pub steps: Vec<ProjectStep>,
    /// IDs of modules or exercises that must be completed before this project is unlocked.
    #[serde(default)]
    pub prerequisites: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectStep {
    pub order: u32,
    pub title: I18nText,
    pub content: Vec<ContentBlock>,
    pub starter_code: Option<String>,
    pub expected_output: Option<String>,
}
