#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Secondary,
    Outline,
    Accent,
}

impl ButtonVariant {
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Default => "button--default",
            Self::Secondary => "button--secondary",
            Self::Outline => "button--outline",
            Self::Accent => "button--accent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Default,
    Icon,
}

impl ButtonSize {
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Default => "button--size-default",
            Self::Icon => "button--size-icon",
        }
    }
}
