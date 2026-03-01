use leptos::prelude::*;

/// A circular progress ring displaying a percentage.
/// Color changes based on progress level:
/// - gray < 25%, yellow < 50%, green < 75%, orange >= 75%
#[component]
pub fn ProgressRing(
    /// Progress percentage (0-100).
    #[prop(into)]
    percent: f64,
    /// Size of the ring in pixels.
    #[prop(default = 80)]
    size: u32,
) -> impl IntoView {
    let radius = (size as f64 / 2.0) - 5.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let offset = circumference - (percent / 100.0) * circumference;
    let center = size as f64 / 2.0;

    let color_class = if percent < 25.0 {
        "text-gray-400"
    } else if percent < 50.0 {
        "text-yellow-500"
    } else if percent < 75.0 {
        "text-green-500"
    } else {
        "text-orange-500"
    };

    view! {
        <div class="relative inline-flex items-center justify-center">
            <svg
                width=size.to_string()
                height=size.to_string()
                class="transform -rotate-90"
            >
                // Background circle
                <circle
                    cx=center.to_string()
                    cy=center.to_string()
                    r=radius.to_string()
                    stroke="currentColor"
                    stroke-width="6"
                    fill="none"
                    class="text-gray-200 dark:text-gray-700"
                />
                // Progress circle
                <circle
                    cx=center.to_string()
                    cy=center.to_string()
                    r=radius.to_string()
                    stroke="currentColor"
                    stroke-width="6"
                    fill="none"
                    stroke-dasharray=circumference.to_string()
                    stroke-dashoffset=offset.to_string()
                    stroke-linecap="round"
                    class=format!("{color_class} transition-all duration-500 ease-out")
                />
            </svg>
            <span class="absolute text-sm font-bold text-gray-700 dark:text-gray-300">
                {format!("{:.0}%", percent)}
            </span>
        </div>
    }
}
