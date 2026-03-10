/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: displaying highscores
*/

use macroquad::prelude::*;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::preload_image::TextureManager;

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager) -> String {
    use_virtual_resolution(virtual_width, virtual_height);
    loop {
        clear_background(BLUE);
        next_frame().await;
    }
}