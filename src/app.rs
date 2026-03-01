use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::layout::app_shell::AppShell;
use crate::components::playground::playground_context::provide_playground;
use crate::i18n::provide_i18n;
use crate::pages::dashboard::Dashboard;
use crate::pages::practice::exercise_list::ExerciseList;
use crate::pages::practice::exercise_view::ExerciseView;
use crate::pages::projects::project_list::ProjectList;
use crate::pages::projects::project_view::ProjectView;
use crate::pages::settings::Settings;
use crate::pages::theory::lesson_view::LessonView;
use crate::pages::theory::module_list::ModuleList;

/// Theme context for dark/light mode
#[derive(Clone)]
pub struct ThemeContext {
    pub dark_mode: RwSignal<bool>,
}

fn provide_theme() {
    let ctx = ThemeContext {
        dark_mode: RwSignal::new(false),
    };
    provide_context(ctx);
}

pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

#[component]
pub fn App() -> impl IntoView {
    provide_i18n();
    provide_theme();
    provide_playground();

    view! {
        <Router>
            <AppShell>
                <Routes fallback=|| view! { <p>"Page not found"</p> }>
                    <Route path=path!("/") view=Dashboard />
                    <Route path=path!("/theory") view=ModuleList />
                    <Route path=path!("/theory/:module_id") view=ModuleList />
                    <Route path=path!("/theory/:module_id/:lesson_id") view=LessonView />
                    <Route path=path!("/practice") view=ExerciseList />
                    <Route path=path!("/practice/:exercise_id") view=ExerciseView />
                    <Route path=path!("/projects") view=ProjectList />
                    <Route path=path!("/projects/:project_id") view=ProjectView />
                    <Route path=path!("/settings") view=Settings />
                </Routes>
            </AppShell>
        </Router>
    }
}
