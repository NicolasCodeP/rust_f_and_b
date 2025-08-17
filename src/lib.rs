#![warn(clippy::all, rust_2018_idioms)]

mod android_keyboard;
mod app;

pub use android_keyboard::{
    check_ui_keyboard_focus, hide_soft_input, set_android_app, set_android_app_from_raw,
    show_soft_input, try_init_from_eframe,
};
pub use app::TemplateApp;
