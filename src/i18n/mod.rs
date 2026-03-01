use leptos::prelude::*;

pub mod locale;
pub mod translations;

pub use locale::Locale;
pub use translations::Translations;

#[derive(Clone, Copy)]
pub struct I18nContext {
    pub locale: RwSignal<Locale>,
}

pub fn provide_i18n() {
    let ctx = I18nContext {
        locale: RwSignal::new(Locale::Es),
    };
    provide_context(ctx);
}

pub fn use_i18n() -> I18nContext {
    expect_context::<I18nContext>()
}

