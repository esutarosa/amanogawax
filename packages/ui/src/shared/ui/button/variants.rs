#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Visual themes supported by the button component.
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Alternative,
}

impl ButtonVariant {
    /// Base CSS class for the variant.
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Primary => "button--primary",
            Self::Secondary => "button--secondary",
            Self::Outline => "button--outline",
            Self::Alternative => "button--alternative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Supported button sizing schemes.
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Icon,
}

impl ButtonSize {
    /// Base CSS class for the size.
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Xs => "button--size-xs",
            Self::Sm => "button--size-sm",
            Self::Md => "button--size-md",
            Self::Icon => "button--size-icon",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Optional styling for adornments (`before`/`after`).
pub enum AdornmentKind {
    Plain,
    Pill,
}
