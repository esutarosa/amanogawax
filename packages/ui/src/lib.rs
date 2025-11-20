//! Shared UI components arranged roughly along FSD principles.

pub mod pages;
pub mod shared;

pub use pages::home::HomePage;

pub use shared::icons::Logo;

pub use shared::styles::GlobalStyles;

pub use shared::ui::button::Button;
pub use shared::ui::container::Container;
pub use shared::ui::icon::IconProps;
pub use shared::ui::section::Section;
pub use shared::ui::typography::Typography;

pub use shared::widgets::layout::Layout;
pub use shared::widgets::navbar::Navbar;
