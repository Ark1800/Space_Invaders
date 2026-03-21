/*
--------------------------------------------
modules/mod.rs
This file lists which modules (Rust files) are part of the "modules" folder.

This file just tells Rust what's available. It's like a directory of all the tools.

Example:
pub mod grid;

Once listed here, you can import from main.rs:
use crate::modules::grid::draw_grid;
--------------------------------------------
*/
// Add modules below

pub mod label;
pub mod text_button;
pub mod still_image;
pub mod grid;
pub mod scale;
pub mod collision;
pub mod player;
pub mod preload_image;
pub mod barrier;
pub mod bullets;
pub mod enemy; 
pub mod text_input;
pub mod listview;
pub mod textfiles;