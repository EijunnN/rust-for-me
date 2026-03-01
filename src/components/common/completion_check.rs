use leptos::prelude::*;

/// A simple checkmark icon that shows when an item is completed.
#[component]
pub fn CompletionCheck(
    /// Whether the item is completed.
    #[prop(into)]
    completed: bool,
) -> impl IntoView {
    view! {
        {if completed {
            view! {
                <span class="inline-flex items-center justify-center w-6 h-6 rounded-full bg-green-500 text-white">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M5 13l4 4L19 7"
                        />
                    </svg>
                </span>
            }
                .into_any()
        } else {
            view! {
                <span class="inline-flex items-center justify-center w-6 h-6 rounded-full border-2 border-gray-300 dark:border-gray-600"></span>
            }
                .into_any()
        }}
    }
}
