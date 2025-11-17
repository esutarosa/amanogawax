#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// HTML tags the typography component can render.
pub enum TypographyTag {
    P,
    Span,
    Div,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Supported typography size tokens.
pub enum TypographySize {
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
}

impl TypographySize {
    /// Corresponding CSS class for the size token.
    pub fn class_name(&self) -> &'static str {
        match self {
            TypographySize::Sm => "typo-size-sm",
            TypographySize::Md => "typo-size-md",
            TypographySize::Lg => "typo-size-lg",
            TypographySize::Xl => "typo-size-xl",
            TypographySize::Xl2 => "typo-size-2xl",
            TypographySize::Xl3 => "typo-size-3xl",
            TypographySize::Xl4 => "typo-size-4xl",
            TypographySize::Xl5 => "typo-size-5xl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Supported font-weight presets.
pub enum TypographyWeight {
    Regular,
    Medium,
}

impl TypographyWeight {
    /// Corresponding CSS class for the weight token.
    pub fn class_name(&self) -> &'static str {
        match self {
            TypographyWeight::Regular => "typo-weight-regular",
            TypographyWeight::Medium => "typo-weight-medium",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Text alignment options.
pub enum TypographyAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl TypographyAlign {
    /// Corresponding CSS class for the alignment.
    pub fn class_name(&self) -> &'static str {
        match self {
            TypographyAlign::Left => "typo-align-left",
            TypographyAlign::Center => "typo-align-center",
            TypographyAlign::Right => "typo-align-right",
            TypographyAlign::Justify => "typo-align-justify",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Text transform options.
pub enum TypographyTransform {
    Uppercase,
    Lowercase,
    Capitalize,
}

impl TypographyTransform {
    /// Corresponding CSS class for the transform.
    pub fn class_name(&self) -> &'static str {
        match self {
            TypographyTransform::Uppercase => "typo-transform-uppercase",
            TypographyTransform::Lowercase => "typo-transform-lowercase",
            TypographyTransform::Capitalize => "typo-transform-capitalize",
        }
    }
}
