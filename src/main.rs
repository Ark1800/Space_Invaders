/*
By: <Your Name Here>
Date: 2026-03-03
Program Details: <Program Description Here>
*/

mod modules;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Space_Invaders".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "screen1".to_string();
    let mut last_switch = get_time() - 0.02;
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "screen1" => screen1::run().await,
                "screen2" => screen2::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
