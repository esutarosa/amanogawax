//! Common button primitive with variants, sizes, and slots.

mod classlist;
mod component;
mod props;
pub mod styles;
mod variants;

pub use component::Button;
pub use props::ButtonProps;
pub use variants::{AdornmentKind, ButtonSize, ButtonVariant};
