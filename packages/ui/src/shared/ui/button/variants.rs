#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Visual themes supported by the button component.
pub enum ButtonVariant {
    Default,
    Secondary,
    Outline,
    Accent,
}

impl ButtonVariant {
    /// Base CSS class for the variant.
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
/// Supported button sizing schemes.
pub enum ButtonSize {
    Default,
    Icon,
}

impl ButtonSize {
    /// Base CSS class for the size.
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Default => "button--size-default",
            Self::Icon => "button--size-icon",
        }
    }
}
