use super::lesson::I18nText;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseType {
    WriteCode,
    FixBug,
    PredictOutput,
}

impl Default for ExerciseType {
    fn default() -> Self {
        ExerciseType::WriteCode
    }
}

impl ExerciseType {
    pub fn label_es(&self) -> &'static str {
        match self {
            ExerciseType::WriteCode => "Escribir Codigo",
            ExerciseType::FixBug => "Corregir Error",
            ExerciseType::PredictOutput => "Predecir Salida",
        }
    }

    pub fn label_en(&self) -> &'static str {
        match self {
            ExerciseType::WriteCode => "Write Code",
            ExerciseType::FixBug => "Fix the Bug",
            ExerciseType::PredictOutput => "Predict Output",
        }
    }

    pub fn badge_classes(&self) -> &'static str {
        match self {
            ExerciseType::WriteCode => "bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300",
            ExerciseType::FixBug => "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300",
            ExerciseType::PredictOutput => "bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Exercise {
    pub meta: ExerciseMeta,
    pub title: I18nText,
    pub description: I18nText,
    pub starter_code: String,
    pub expected_output: String,
    pub hints: HintsI18n,
    pub solution: String,
    /// The broken code shown to students in FixBug exercises.
    #[serde(default)]
    pub broken_code: Option<String>,
    /// The compiler error message associated with the broken code.
    #[serde(default)]
    pub compiler_error: Option<I18nText>,
    /// Multiple-choice options for PredictOutput exercises.
    #[serde(default)]
    pub options: Option<Vec<PredictOption>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExerciseMeta {
    pub id: String,
    pub module: String,
    pub difficulty: String,
    pub order: u32,
    #[serde(default)]
    pub exercise_type: ExerciseType,
    /// IDs of modules or exercises that must be completed before this exercise is unlocked.
    #[serde(default)]
    pub prerequisites: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HintsI18n {
    pub es: Vec<String>,
    pub en: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PredictOption {
    pub es: String,
    pub en: String,
    pub correct: bool,
}
