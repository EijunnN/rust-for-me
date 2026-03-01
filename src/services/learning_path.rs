use crate::models::progress::{ProgressStatus, UserProgress};

/// Check whether all prerequisites for an item are completed.
/// `prerequisites` is the list of IDs (modules or exercises) required.
/// `all_progress` is the full progress list from the database.
pub fn is_unlocked(prerequisites: &[String], all_progress: &[UserProgress]) -> bool {
    if prerequisites.is_empty() {
        return true;
    }
    prerequisites.iter().all(|prereq| {
        all_progress.iter().any(|p| p.id == *prereq && p.status == ProgressStatus::Completed)
    })
}

/// Describes a recommended next item for the dashboard.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RecommendedNext {
    pub id: String,
    pub category: String, // "lesson", "exercise", "project"
    pub title_es: String,
    pub title_en: String,
    pub href: String,
}

/// Determine the next recommended content item for the user.
/// Priority: first incomplete lesson, then first unlocked incomplete exercise,
/// then first unlocked incomplete project.
pub fn get_next_recommended(
    modules: &[crate::models::module::Module],
    exercises: &[crate::models::exercise::Exercise],
    projects: &[crate::models::project::Project],
    all_progress: &[UserProgress],
) -> Option<RecommendedNext> {
    // 1. Find first incomplete lesson (by module order, then lesson order)
    for module in modules {
        for lesson in &module.lessons {
            let completed = all_progress.iter().any(|p| {
                p.id == lesson.id && p.category == "lesson" && p.status == ProgressStatus::Completed
            });
            if !completed {
                return Some(RecommendedNext {
                    id: lesson.id.clone(),
                    category: "lesson".to_string(),
                    title_es: lesson.title_es.clone(),
                    title_en: lesson.title_en.clone(),
                    href: format!("/theory/{}/{}", module.id, lesson.id),
                });
            }
        }
    }

    // 2. Find first unlocked incomplete exercise
    for exercise in exercises {
        let completed = all_progress.iter().any(|p| {
            p.id == exercise.meta.id
                && p.category == "exercise"
                && p.status == ProgressStatus::Completed
        });
        if !completed && is_unlocked(&exercise.meta.prerequisites, all_progress) {
            return Some(RecommendedNext {
                id: exercise.meta.id.clone(),
                category: "exercise".to_string(),
                title_es: exercise.title.es.clone(),
                title_en: exercise.title.en.clone(),
                href: format!("/practice/{}", exercise.meta.id),
            });
        }
    }

    // 3. Find first unlocked incomplete project
    for project in projects {
        let completed = all_progress.iter().any(|p| {
            p.id == project.id
                && p.category == "project"
                && p.status == ProgressStatus::Completed
        });
        if !completed && is_unlocked(&project.prerequisites, all_progress) {
            return Some(RecommendedNext {
                id: project.id.clone(),
                category: "project".to_string(),
                title_es: project.title.es.clone(),
                title_en: project.title.en.clone(),
                href: format!("/projects/{}", project.id),
            });
        }
    }

    None
}

/// Calculate overall completion percentage across all content.
pub fn get_completion_percentage(
    modules: &[crate::models::module::Module],
    exercises: &[crate::models::exercise::Exercise],
    projects: &[crate::models::project::Project],
    all_progress: &[UserProgress],
) -> f64 {
    let total_lessons: usize = modules.iter().map(|m| m.lessons.len()).sum();
    let total_exercises = exercises.len();
    let total_projects = projects.len();
    let total = total_lessons + total_exercises + total_projects;

    if total == 0 {
        return 0.0;
    }

    let completed = all_progress
        .iter()
        .filter(|p| {
            p.status == ProgressStatus::Completed
                && (p.category == "lesson" || p.category == "exercise" || p.category == "project")
        })
        .count();

    (completed as f64 / total as f64 * 100.0).round()
}
