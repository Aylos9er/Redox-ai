#![crate_name = "orbtk"]
#![crate_type = "lib"]
// #![deny(warnings)]
#![feature(const_fn)]

extern crate dces;

pub use dces::prelude::*;

extern crate cssparser;
#[macro_use]
extern crate lazy_static;

pub use application::*;
pub use backend::*;
pub use cell::CloneCell;
pub use enums::*;
pub use error::*;
pub use event::*;
pub use layout_object::*;
pub use render_object::*;
pub use state::*;
pub use structs::*;
pub use systems::*;
pub use theme::{Selector, Theme, DEFAULT_THEME_CSS, LIGHT_THEME_CSS};
pub use tree::*;
pub use widget::*;

pub mod application;
pub mod backend;
pub mod cell;
pub mod enums;
pub mod error;
pub mod event;
pub mod layout_object;
pub mod render_object;
pub mod state;
pub mod structs;
pub mod systems;
pub mod theme;
pub mod tree;
pub mod widget;

extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
pub use orbclient::color::Color;

// todo workspace -> backend, core, widgets

// todo: Use entity 0 to store singelton values like focues entity
