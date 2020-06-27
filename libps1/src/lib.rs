#![feature(type_alias_impl_trait)]

pub use ansi_term::{Color, Style};

mod themes;
pub use themes::Theme;

pub mod helpers;
pub mod module;
pub mod prompt;
