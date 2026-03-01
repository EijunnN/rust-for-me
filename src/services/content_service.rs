use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::models::exercise::{Exercise, ExerciseMeta, HintsI18n, PredictOption};
use crate::models::lesson::{ContentBlock, I18nText, Lesson, LessonMetadata};
use crate::models::module::{LessonMeta, Module};
use crate::models::project::{Project, ProjectStep};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

// ── Tauri invoke helpers ────────────────────────────────────────────

/// Call the `load_content` Tauri command and return the raw TOML string.
async fn load_content_raw(path: &str) -> Result<String, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"path".into(), &path.into())
        .map_err(|e| format!("{:?}", e))?;

    let result = invoke("load_content", args.into())
        .await
        .map_err(|e| {
            let s = format!("{:?}", e);
            s
        })?;

    result
        .as_string()
        .ok_or_else(|| "load_content did not return a string".to_string())
}

/// Call the `list_content_dir` Tauri command and return directory entries.
async fn list_content_dir(path: &str) -> Result<Vec<DirEntry>, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"path".into(), &path.into())
        .map_err(|e| format!("{:?}", e))?;

    let result = invoke("list_content_dir", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    let json_str = result
        .as_string()
        .ok_or_else(|| "list_content_dir did not return a string".to_string())?;

    serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse dir listing: {e}"))
}

#[derive(Deserialize)]
struct DirEntry {
    name: String,
    is_dir: bool,
}

// ── TOML intermediate deserialization types ─────────────────────────
// These mirror the TOML structure and get converted to the app's model types.

#[derive(Deserialize)]
struct ModuleToml {
    meta: ModuleMetaToml,
    title: I18nText,
    description: I18nText,
    lessons: Vec<LessonMetaToml>,
}

#[derive(Deserialize)]
struct ModuleMetaToml {
    id: String,
    order: u32,
    icon: String,
}

#[derive(Deserialize)]
struct LessonMetaToml {
    id: String,
    order: u32,
    title_es: String,
    title_en: String,
}

#[derive(Deserialize)]
struct LessonToml {
    meta: LessonMetadata,
    title: I18nText,
    blocks: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ExerciseToml {
    meta: ExerciseMeta,
    title: I18nText,
    description: I18nText,
    starter_code: StarterCodeToml,
    validation: ValidationToml,
    hints: HintsI18n,
    solution: SolutionToml,
    #[serde(default)]
    broken_code: Option<BrokenCodeToml>,
    #[serde(default)]
    compiler_error: Option<I18nText>,
    #[serde(default)]
    options: Option<Vec<PredictOption>>,
}

#[derive(Deserialize)]
struct BrokenCodeToml {
    code: String,
}

#[derive(Deserialize)]
struct StarterCodeToml {
    code: String,
}

#[derive(Deserialize)]
struct ValidationToml {
    expected_output: String,
}

#[derive(Deserialize)]
struct SolutionToml {
    code: String,
}

#[derive(Deserialize)]
struct ProjectToml {
    meta: ProjectMetaToml,
    title: I18nText,
    description: I18nText,
    steps: Vec<ProjectStepMetaToml>,
}

#[derive(Deserialize)]
struct ProjectMetaToml {
    id: String,
    difficulty: String,
    #[serde(default)]
    prerequisites: Vec<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ProjectStepMetaToml {
    id: String,
    order: u32,
    title_es: String,
    title_en: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ProjectStepToml {
    meta: ProjectStepMetaFieldToml,
    title: I18nText,
    #[serde(default)]
    description: Option<I18nText>,
    #[serde(default)]
    blocks: Vec<ContentBlock>,
    #[serde(default)]
    starter_code: Option<StarterCodeToml>,
    #[serde(default)]
    validation: Option<ValidationToml>,
    #[serde(default)]
    hints: Option<HintsI18n>,
    #[serde(default)]
    solution: Option<SolutionToml>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ProjectStepMetaFieldToml {
    id: String,
    project: String,
    order: u32,
}

// ── Public helpers ──────────────────────────────────────────────────

/// List directory names in a subdirectory of content/.
/// Returns just the names of directories (not files).
pub async fn list_content_dir_raw(path: &str) -> Result<Vec<String>, String> {
    let entries = list_content_dir(path).await?;
    Ok(entries.into_iter().filter(|e| e.is_dir).map(|e| e.name).collect())
}

// ── Public API ──────────────────────────────────────────────────────

/// Load all theory modules from `content/theory/*/\_module.toml`.
pub async fn load_all_modules() -> Result<Vec<Module>, String> {
    let dirs = list_content_dir("theory").await?;

    let mut modules = Vec::new();
    for entry in dirs {
        if !entry.is_dir {
            continue;
        }
        let path = format!("theory/{}/_module.toml", entry.name);
        let toml_str = match load_content_raw(&path).await {
            Ok(s) => s,
            Err(_) => continue, // skip dirs without _module.toml
        };
        let parsed: ModuleToml =
            toml::from_str(&toml_str).map_err(|e| format!("Failed to parse {path}: {e}"))?;

        modules.push(Module {
            id: parsed.meta.id,
            order: parsed.meta.order,
            title_es: parsed.title.es,
            title_en: parsed.title.en,
            description_es: parsed.description.es,
            description_en: parsed.description.en,
            lessons: parsed
                .lessons
                .into_iter()
                .map(|l| LessonMeta {
                    id: l.id,
                    order: l.order,
                    title_es: l.title_es,
                    title_en: l.title_en,
                })
                .collect(),
            icon: parsed.meta.icon,
        });
    }

    modules.sort_by_key(|m| m.order);
    Ok(modules)
}

/// Load a single lesson by its module directory name and lesson filename.
/// E.g. `load_lesson("m01_introduction", "01_what_is_rust")`.
pub async fn load_lesson(module_dir: &str, lesson_file: &str) -> Result<Lesson, String> {
    let path = format!("theory/{}/{}.toml", module_dir, lesson_file);
    let toml_str = load_content_raw(&path).await?;
    let parsed: LessonToml =
        toml::from_str(&toml_str).map_err(|e| format!("Failed to parse {path}: {e}"))?;

    Ok(Lesson {
        meta: parsed.meta,
        title: parsed.title,
        blocks: parsed.blocks,
    })
}

/// Load all exercises from `content/exercises/*/` directories.
pub async fn load_all_exercises() -> Result<Vec<Exercise>, String> {
    let module_dirs = list_content_dir("exercises").await?;

    let mut exercises = Vec::new();
    for dir in module_dirs {
        if !dir.is_dir {
            continue;
        }
        let subpath = format!("exercises/{}", dir.name);
        let files = list_content_dir(&subpath).await?;
        for file in files {
            if file.is_dir || !file.name.ends_with(".toml") {
                continue;
            }
            let path = format!("{}/{}", subpath, file.name);
            let toml_str = match load_content_raw(&path).await {
                Ok(s) => s,
                Err(_) => continue,
            };
            let parsed: ExerciseToml = toml::from_str(&toml_str)
                .map_err(|e| format!("Failed to parse {path}: {e}"))?;

            exercises.push(Exercise {
                meta: parsed.meta,
                title: parsed.title,
                description: parsed.description,
                starter_code: parsed.starter_code.code,
                expected_output: parsed.validation.expected_output,
                hints: parsed.hints,
                solution: parsed.solution.code,
                broken_code: parsed.broken_code.map(|bc| bc.code),
                compiler_error: parsed.compiler_error,
                options: parsed.options,
            });
        }
    }

    exercises.sort_by_key(|e| (e.meta.module.clone(), e.meta.order));
    Ok(exercises)
}

/// Load a single exercise by module and exercise filename.
/// E.g. `load_exercise("m01", "e01_hello")`.
pub async fn load_exercise(module: &str, exercise_file: &str) -> Result<Exercise, String> {
    let path = format!("exercises/{}/{}.toml", module, exercise_file);
    let toml_str = load_content_raw(&path).await?;
    let parsed: ExerciseToml =
        toml::from_str(&toml_str).map_err(|e| format!("Failed to parse {path}: {e}"))?;

    Ok(Exercise {
        meta: parsed.meta,
        title: parsed.title,
        description: parsed.description,
        starter_code: parsed.starter_code.code,
        expected_output: parsed.validation.expected_output,
        hints: parsed.hints,
        solution: parsed.solution.code,
        broken_code: parsed.broken_code.map(|bc| bc.code),
        compiler_error: parsed.compiler_error,
        options: parsed.options,
    })
}

/// Load all projects from `content/projects/*/\_project.toml`.
pub async fn load_all_projects() -> Result<Vec<Project>, String> {
    let dirs = list_content_dir("projects").await?;

    let mut projects = Vec::new();
    for entry in dirs {
        if !entry.is_dir {
            continue;
        }
        let meta_path = format!("projects/{}/_project.toml", entry.name);
        let toml_str = match load_content_raw(&meta_path).await {
            Ok(s) => s,
            Err(_) => continue,
        };
        let parsed: ProjectToml =
            toml::from_str(&toml_str).map_err(|e| format!("Failed to parse {meta_path}: {e}"))?;

        // Build lightweight ProjectStep entries from the step metadata.
        // Full content is loaded on demand via load_project_step.
        let steps: Vec<ProjectStep> = parsed
            .steps
            .into_iter()
            .map(|s| ProjectStep {
                order: s.order,
                title: I18nText {
                    es: s.title_es,
                    en: s.title_en,
                },
                content: Vec::new(),
                starter_code: None,
                expected_output: None,
            })
            .collect();

        projects.push(Project {
            id: parsed.meta.id,
            title: parsed.title,
            description: parsed.description,
            difficulty: parsed.meta.difficulty,
            steps,
            prerequisites: parsed.meta.prerequisites,
        });
    }

    Ok(projects)
}

/// Load a single project step with full content.
/// `project_dir` is e.g. `"p01_calculator"`, `step_file` is e.g. `"step01"`.
pub async fn load_project_step(
    project_dir: &str,
    step_file: &str,
) -> Result<ProjectStep, String> {
    let path = format!("projects/{}/{}.toml", project_dir, step_file);
    let toml_str = load_content_raw(&path).await?;
    let parsed: ProjectStepToml =
        toml::from_str(&toml_str).map_err(|e| format!("Failed to parse {path}: {e}"))?;

    Ok(ProjectStep {
        order: parsed.meta.order,
        title: parsed.title,
        content: parsed.blocks,
        starter_code: parsed.starter_code.map(|sc| sc.code),
        expected_output: parsed.validation.map(|v| v.expected_output),
    })
}

/// Resolve a lesson id (e.g. "m01_l01") to (module_dir, lesson_file) by
/// scanning the theory directory.
pub async fn resolve_lesson_path(lesson_id: &str) -> Result<(String, String), String> {
    let dirs = list_content_dir("theory").await?;

    for entry in dirs {
        if !entry.is_dir {
            continue;
        }
        let subpath = format!("theory/{}", entry.name);
        let files = list_content_dir(&subpath).await?;
        for file in files {
            if file.is_dir || !file.name.ends_with(".toml") || file.name.starts_with('_') {
                continue;
            }
            let file_stem = file.name.trim_end_matches(".toml");
            // Try loading just the meta to check the id
            let path = format!("{}/{}", subpath, file.name);
            if let Ok(toml_str) = load_content_raw(&path).await {
                // Quick check: parse just the meta.id field
                if let Ok(parsed) = toml::from_str::<LessonToml>(&toml_str) {
                    if parsed.meta.id == lesson_id {
                        return Ok((entry.name.clone(), file_stem.to_string()));
                    }
                }
            }
        }
    }

    Err(format!("Lesson not found: {lesson_id}"))
}

/// Resolve an exercise id (e.g. "m01_e01") to (module_dir, exercise_file).
pub async fn resolve_exercise_path(exercise_id: &str) -> Result<(String, String), String> {
    let dirs = list_content_dir("exercises").await?;

    for entry in dirs {
        if !entry.is_dir {
            continue;
        }
        let subpath = format!("exercises/{}", entry.name);
        let files = list_content_dir(&subpath).await?;
        for file in files {
            if file.is_dir || !file.name.ends_with(".toml") {
                continue;
            }
            let file_stem = file.name.trim_end_matches(".toml");
            let path = format!("{}/{}", subpath, file.name);
            if let Ok(toml_str) = load_content_raw(&path).await {
                if let Ok(parsed) = toml::from_str::<ExerciseToml>(&toml_str) {
                    if parsed.meta.id == exercise_id {
                        return Ok((entry.name.clone(), file_stem.to_string()));
                    }
                }
            }
        }
    }

    Err(format!("Exercise not found: {exercise_id}"))
}
