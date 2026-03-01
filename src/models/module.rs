#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Module {
    pub id: String,
    pub order: u32,
    pub title_es: String,
    pub title_en: String,
    pub description_es: String,
    pub description_en: String,
    pub lessons: Vec<LessonMeta>,
    pub icon: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LessonMeta {
    pub id: String,
    pub order: u32,
    pub title_es: String,
    pub title_en: String,
}
