#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    Es,
    En,
}

impl Locale {
    pub fn name(&self) -> &'static str {
        match self {
            Locale::Es => "Español",
            Locale::En => "English",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Locale::Es => Locale::En,
            Locale::En => Locale::Es,
        }
    }
}
