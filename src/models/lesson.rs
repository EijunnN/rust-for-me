#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Lesson {
    pub meta: LessonMetadata,
    pub title: I18nText,
    pub blocks: Vec<ContentBlock>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LessonMetadata {
    pub id: String,
    pub order: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct I18nText {
    pub es: String,
    pub en: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { es: String, en: String },
    #[serde(rename = "code")]
    Code {
        language: String,
        runnable: bool,
        code: String,
    },
    #[serde(rename = "callout")]
    Callout {
        variant: String,
        es: String,
        en: String,
    },
    #[serde(rename = "quiz")]
    Quiz {
        es: String,
        en: String,
        options: Vec<QuizOption>,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuizOption {
    pub es: String,
    pub en: String,
    pub correct: bool,
}
