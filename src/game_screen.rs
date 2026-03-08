/*
By: <Your Name Here>
Date: 2026-03-03
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::modules::scale::use_virtual_resolution;

pub async fn run(virtual_width: f32, virtual_height: f32) -> String {
    use_virtual_resolution(virtual_width, virtual_height);
    loop {
        clear_background(RED);
        next_frame().await;
    }
}