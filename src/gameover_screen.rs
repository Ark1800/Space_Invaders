/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: gameover screen, when player loses
*/

use macroquad::prelude::*;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::preload_image::TextureManager;
use crate::modules::text_button::TextButton;
use crate::modules::still_image::StillImage;
use crate::modules::text_input::TextInput;
use crate::modules::label::Label;
use crate::modules::textfiles::TextFile;

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager, score: i32) -> String {
    //VARIABLESSSSSSSSSSSS
    use_virtual_resolution(virtual_width, virtual_height);
    //LABELLLSSSSSSSSSSSSSSSS AND INPUTSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut lbl_score = Label::new(format!("High Score: {}", score), 250.0, 700.0, 60);
    lbl_score.with_colors(WHITE, Some(DARKGRAY));
    let mut txt_entername = TextInput::new(50.0, 750.0, 325.0, 90.0, 40.0);
    txt_entername.set_enabled(false);
    txt_entername.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_entername.set_prompt("Enter your name...");
    txt_entername.set_prompt_color(PURPLE);
    let mut name_input_ready = false;
    //BUTTONSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS AND IMAGESSSSSSSSSSSSSSSSS
    let mut gameover_img = StillImage::new(
        "", // image path
        virtual_width,  // width
        virtual_height/2.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    gameover_img.set_preload(tm.get_preload("assets/gameover_bg.png").unwrap());
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
    let mut btn_savehighscore = TextButton::new(
        400.0,
        750.0,
        350.0,
        100.0,
        "Save High Score",
        BLACK,
        RED,
        30
    );
    btn_savehighscore.with_text_color(YELLOW);
    btn_savehighscore.with_round(10.0);
    let mut btn_titlescreen = TextButton::new(
        50.0,
        875.0,
        700.0,
        100.0,
        "Title Screen",
        BLACK,
        RED,
        30
    );
    btn_titlescreen.with_text_color(YELLOW); 
    btn_titlescreen.with_round(10.0);
    let mut btn_exit = TextButton::new(
        50.0,
        1000.0,
        700.0,
        100.0,
        "Exit",
        BLACK,
        RED,
        30
    );
    btn_exit.with_text_color(YELLOW);
    btn_exit.with_round(10.0);
    loop { //AI helped with this idea, WASD inputs was carrying over to next screen during transition, so it is only enabled when= keys stop being held
        if !name_input_ready { //second boolean so once it passes (after keys are released) it doesn't keep running and cause issues with text input
            while get_char_pressed().is_some() {}
            let any_key_held = !get_keys_down().is_empty();
            if !any_key_held {
                txt_entername.set_enabled(true);
                name_input_ready = true;
            }
        }
        bg_img.draw();
        gameover_img.draw();
        lbl_score.draw();
        txt_entername.draw();
        if btn_titlescreen.click() {
            return "title_screen".to_string();
        }
        if btn_exit.click() {
            return "exit".to_string();
        }
        if btn_savehighscore.click() {
            let name = txt_entername.get_text();
            let entry = format!("{}: {}\n", name, score);
            let result = TextFile::load_strings("highscores.txt").await;
            let mut scores: Vec<String> = result.unwrap_or_default();
            scores.push(entry);
            let result = TextFile::save_strings("highscores.txt", scores).await;
            if let Err(e) = result {
                println!("Error saving scores: {}", e);
            }
            btn_savehighscore.set_text("Saved!");
            btn_savehighscore.with_text_color(BLACK);
            btn_savehighscore.set_all_colors(GREEN, GREEN, BLACK, BLACK);
            btn_savehighscore.set_enabled(false); //Disable the button after saving
        }
        next_frame().await;
    }
} 
