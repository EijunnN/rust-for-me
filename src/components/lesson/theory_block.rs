use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::i18n::locale::Locale;

/// Renders a localized theory text block with basic formatting support.
/// Supports **bold** and `code` inline formatting.
#[component]
pub fn TheoryBlock(
    #[prop(into)] es: String,
    #[prop(into)] en: String,
) -> impl IntoView {
    let i18n = use_i18n();
    let es = es.clone();
    let en = en.clone();

    view! {
        <div class="prose dark:prose-invert max-w-none mb-6">
            {move || {
                let text = match i18n.locale.get() {
                    Locale::Es => es.clone(),
                    Locale::En => en.clone(),
                };
                // Simple formatting: convert **bold** and `code` to HTML
                let formatted = format_text(&text);
                view! {
                    <div class="text-gray-700 dark:text-gray-300 leading-relaxed text-base" inner_html=formatted />
                }
            }}
        </div>
    }
}

/// Simple text formatter that converts **bold** and `code` markers to HTML.
fn format_text(text: &str) -> String {
    let mut result = text.to_string();

    // Convert **bold** to <strong>
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let bold_text = &result[start + 2..start + 2 + end].to_string();
            let replacement = format!("<strong class=\"font-semibold text-gray-900 dark:text-white\">{bold_text}</strong>");
            result = format!("{}{}{}", &result[..start], replacement, &result[start + 2 + end + 2..]);
        } else {
            break;
        }
    }

    // Convert `code` to <code>
    while let Some(start) = result.find('`') {
        if let Some(end) = result[start + 1..].find('`') {
            let code_text = &result[start + 1..start + 1 + end].to_string();
            let replacement = format!("<code class=\"px-1.5 py-0.5 bg-gray-100 dark:bg-gray-800 text-orange-600 dark:text-orange-400 rounded text-sm font-mono\">{code_text}</code>");
            result = format!("{}{}{}", &result[..start], replacement, &result[start + 1 + end + 1..]);
        } else {
            break;
        }
    }

    // Convert newlines to <br> for paragraph breaks
    result = result.replace("\n\n", "</p><p class=\"mb-4\">");
    result = result.replace('\n', "<br/>");

    format!("<p class=\"mb-4\">{result}</p>")
}
