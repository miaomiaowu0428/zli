//! Building HTML views using templates.
//!
//! # Supported template engines
//!
//! The following optional features are available:
//!
//! | Feature flag     | Description                                          | Default? |
//! |------------------|------------------------------------------------------|----------|
//! | `view-minijinja` | Enables the `minijinja` template engine.             | No       |
//! | `view-tera`      | Enables the `tera` template engine.                  | No       |

use crate::{application::Application, extension::TomlTableExt};

cfg_if::cfg_if! {
    if #[cfg(feature = "view-tera")] {
        mod tera;

        use self::tera::load_templates;
        pub use self::tera::render;
    } else {
        mod minijinja;

        use self::minijinja::load_templates;
        pub use self::minijinja::render;
    }
}

/// Intializes view engine.
pub(crate) fn init<APP: Application + ?Sized>() {
    let app_state = APP::shared_state();
    let mut template_dir = "templates";
    if let Some(view) = app_state.get_config("view") {
        if let Some(dir) = view.get_str("template-dir") {
            template_dir = dir;
        }
    }
    load_templates(app_state, APP::parse_path(template_dir));
}
