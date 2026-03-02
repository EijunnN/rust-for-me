use leptos::prelude::*;
use pulldown_cmark::{Parser, Options, html};
use crate::i18n::use_i18n;
use crate::i18n::locale::Locale;

/// Renders a localized theory text block with full Markdown support.
#[component]
pub fn TheoryBlock(
    #[prop(into)] es: String,
    #[prop(into)] en: String,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="prose dark:prose-invert max-w-none mb-6">
            {move || {
                let text = match i18n.locale.get() {
                    Locale::Es => es.clone(),
                    Locale::En => en.clone(),
                };
                let rendered = render_markdown(&text);
                view! {
                    <div class="text-gray-700 dark:text-gray-300 leading-relaxed text-base
                                [&_h1]:text-2xl [&_h1]:font-bold [&_h1]:text-gray-900 [&_h1]:dark:text-white [&_h1]:mb-4 [&_h1]:mt-6
                                [&_h2]:text-xl [&_h2]:font-bold [&_h2]:text-gray-900 [&_h2]:dark:text-white [&_h2]:mb-3 [&_h2]:mt-5
                                [&_h3]:text-lg [&_h3]:font-semibold [&_h3]:text-gray-900 [&_h3]:dark:text-white [&_h3]:mb-2 [&_h3]:mt-4
                                [&_strong]:font-semibold [&_strong]:text-gray-900 [&_strong]:dark:text-white
                                [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:bg-gray-100 [&_code]:dark:bg-gray-800 [&_code]:text-orange-600 [&_code]:dark:text-orange-400 [&_code]:rounded [&_code]:text-sm [&_code]:font-mono
                                [&_pre_code]:block [&_pre_code]:p-4 [&_pre_code]:bg-gray-900 [&_pre_code]:text-gray-100 [&_pre_code]:rounded-lg [&_pre_code]:overflow-x-auto [&_pre_code]:text-sm
                                [&_ul]:list-disc [&_ul]:pl-6 [&_ul]:mb-4 [&_ul]:space-y-1
                                [&_ol]:list-decimal [&_ol]:pl-6 [&_ol]:mb-4 [&_ol]:space-y-1
                                [&_li]:text-gray-700 [&_li]:dark:text-gray-300
                                [&_p]:mb-4
                                [&_blockquote]:border-l-4 [&_blockquote]:border-orange-400 [&_blockquote]:pl-4 [&_blockquote]:italic [&_blockquote]:text-gray-600 [&_blockquote]:dark:text-gray-400
                                [&_a]:text-orange-600 [&_a]:dark:text-orange-400 [&_a]:underline"
                        inner_html=rendered
                    />
                }
            }}
        </div>
    }
}

/// Renders Markdown text to HTML using pulldown-cmark.
fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
