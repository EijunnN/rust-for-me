#![allow(dead_code)]
use leptos::prelude::*;

use crate::models::progress::ProgressStatus;

/// Displays a small badge indicating progress status.
#[component]
pub fn ProgressBadge(#[prop(into)] status: ProgressStatus) -> impl IntoView {
    let (label, color_class) = match status {
        ProgressStatus::Locked => ("Locked", "bg-gray-400 text-gray-800"),
        ProgressStatus::Available => ("Available", "bg-blue-500 text-white"),
        ProgressStatus::InProgress => ("In Progress", "bg-yellow-500 text-black"),
        ProgressStatus::Completed => ("Completed", "bg-green-500 text-white"),
    };

    view! {
        <span class=format!(
            "inline-block px-2 py-0.5 rounded-full text-xs font-medium {color_class}"
        )>{label}</span>
    }
}
