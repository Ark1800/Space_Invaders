/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: initial title screen with instructions and buttons to play, view highscores, or exit
*/
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;
use crate::modules::label::TextAlign;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;


pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager) -> String {
    //VIRTUAL W AND H
    use_virtual_resolution(virtual_width, virtual_height);
    //LABELS AND BUTTONS AND IMAGES SETUPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP
    let mut lbl_howto = Label::new("Welcome to Space Invaders! \n Use A and D to move across the screen and dodge lasers! \n Press W or Space to fire back! \n Don't shoot your barriers \n and don't let the enemies either! \n Good luck! and don't let them get too close...", 30.0, 420.0, 30);
    lbl_howto.with_colors(YELLOW, Some(BLACK));
    lbl_howto.with_fixed_size(750.0, 250.0);
    lbl_howto.with_alignment(TextAlign::Center);
    lbl_howto.with_round(10.0);
    let mut title_img = StillImage::new(
        "",
        600.0,  // width
        260.0,  // height
        100.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    title_img.set_preload(tm.get_preload("assets/SpaceInvadersLogo.png").unwrap());
    let mut bg_img = StillImage::new(
        "",
        virtual_width,  // width
        virtual_height, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    bg_img.set_preload(tm.get_preload("assets/spaceinvadersbg.png").unwrap());
    let mut btn_play = TextButton::new(
        50.0,
        750.0,
        300.0,
        150.0,
        "Play Game",
        BLACK,
        RED,
        30
    );
    btn_play.with_text_color(YELLOW); 
    btn_play.with_round(10.0);
    let mut btn_highscores = TextButton::new(
        450.0,
        750.0,
        300.0,
        150.0,
        "High Scores",
        BLACK,
        RED,
        30
    );
    btn_highscores.with_text_color(YELLOW); 
    btn_highscores.with_round(10.0);
    let mut btn_exit = TextButton::new(
        250.0,
        950.0,
        300.0,
        150.0,
        "Exit",
        BLACK,
        RED,
        30
    );
    btn_exit.with_text_color(YELLOW); 
    btn_exit.with_round(10.0);
    loop {
        bg_img.draw();
        if btn_play.click() {
            return "game_screen".to_string();
        }
        if btn_highscores.click() {
            return "highscores_screen".to_string();
        }
        if btn_exit.click() {
            return "exit".to_string();
        }
        title_img.draw();
        lbl_howto.draw();
        next_frame().await;
    }
}