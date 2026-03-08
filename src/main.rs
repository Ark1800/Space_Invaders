/*
By: <Your Name Here>
Date: 2026-03-03
Program Details: <Program Description Here>
*/

mod modules;
mod game_screen;
mod title_screen;
mod highscores_screen;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_string(),
        window_width: 800,
        window_height: 1200,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

// Virtual resolution constants
const VIRTUAL_WIDTH: f32 = 800.0;
const VIRTUAL_HEIGHT: f32 = 1200.0;

#[macroquad::main(window_conf)]
async fn main() {
    //PRELOADEEDDDDDDDDD
    let all_assets = vec!["enemy_1.png", "enemy_2.png", "enemy_3.png", "player_ship.png", "player_heart.png", "spaceinvadersbg.png", "SpaceInvadersLogo.png", "barrier_1.png", "barrier_2.png", "barrier_3.png", "barrier_4.png", "barrier_5.png", "barrier_6.png", "barrier_7.png", "barrier_8.png", "barrier_9.png", "barrier_10.png"];
    let tm = TextureManager::new();
    // Using custom loading screen appearance
    let loading_options = LoadingScreenOptions {
       title: Some("Maze Game".to_string()),
       background_color: BLACK,
       bar_fill_color: RED,
       // Use default values for other options
       ..Default::default()
    };
    tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    //SCREENSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut current_screen = "title_screen".to_string();
    let mut last_switch = get_time() - 0.02;
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "title_screen" => title_screen::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT).await,
                "game_screen" => game_screen::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &tm, all_assets.clone()).await,
                "highscores_screen" => highscores_screen::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
