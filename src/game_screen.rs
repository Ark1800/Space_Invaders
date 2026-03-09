/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: main game loop, where the player, barriers, and enemies will be drawn and updated, also handles collisions and score
*/

use macroquad::prelude::*;
use crate::modules::{barrier, player};
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager, _assets: Vec<&str>) -> String {
    //VARIABLESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut bullets: Vec<String> = vec![];
    let mut bullets_dir: Vec<f32> = vec![];
    let mut score = 0;
    use_virtual_resolution(virtual_width, virtual_height);
    //MODULESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut player = player::Player::new("assets/player_ship.png", virtual_width / 2.0, virtual_height - 160.0).await;
    let mut barrier_1 = barrier::Barrier::new("assets/barrier_1.png", 87.5, virtual_height - 300.0).await;
    let mut barrier_2 = barrier::Barrier::new("assets/barrier_1.png", 325.0, virtual_height - 300.0).await;
    let mut barrier_3 = barrier::Barrier::new("assets/barrier_1.png", 562.5, virtual_height - 300.0).await;
    //LABELSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut lbl_score_str = Label::new("Score", 20.0, 40.0, 60);
    let mut lbl_score_num = Label::new("0", 170.0, 40.0, 60);
    lbl_score_str.with_colors(WHITE, Some(DARKGRAY));
    lbl_score_num.with_colors(WHITE, Some(DARKGRAY));
    //IMAGESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut bg_img = StillImage::new(
        "",
        virtual_width,  // width
        virtual_height, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut wall_l = StillImage::new(
        "",
        20.0,  // width
        virtual_height, // height
        -20.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut wall_r = StillImage::new(
        "",
        20.0,  // width
        virtual_height, // height
        virtual_width,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    if let Some(preloaded) = tm.get_preload("assets/spaceinvadersbg.png") {
       bg_img.set_preload(preloaded.clone());
       wall_l.set_preload(preloaded.clone());
       wall_r.set_preload(preloaded);
    } else {
       bg_img.set_preload(tm.get_preload_by_index(5).unwrap()); //set to background galaxy texture if error
    }
    let mut heart_1 = StillImage::new(
        "",
        60.0,  // width
        60.0, // height
        virtual_width-210.0,    // x position
        20.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut heart_2 = StillImage::new(
        "",
        60.0,  // width
        60.0, // height
        virtual_width-140.0,    // x position
        20.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut heart_3 = StillImage::new(
        "",
        60.0,  // width
        60.0, // height
        virtual_width-70.0,    // x position
        20.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    if let Some(preloaded) = tm.get_preload("assets/player_heart.png") {
       heart_1.set_preload(preloaded.clone()); 
       heart_2.set_preload(preloaded.clone());
       heart_3.set_preload(preloaded);
    
    } else {
       bg_img.set_preload(tm.get_preload_by_index(5).unwrap()); //set to background galaxy texture if error
    }
    loop {
        //PLAYERRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR
        let oldpos = player.get_oldpos();
        let shot = player.handle_keypresses();
        if shot {

        }
        player.move_x();
        if player.check_collision(&wall_l) {
            player.set_x(oldpos);
        }
        if player.check_collision(&wall_r) {
            player.set_x(oldpos);
        }

        //DRAWINGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
        bg_img.draw();
        wall_l.draw();
        wall_r.draw();
        heart_1.draw();
        heart_2.draw();
        heart_3.draw();
        barrier_1.draw();
        barrier_2.draw();
        barrier_3.draw();
        lbl_score_num.draw();
        lbl_score_str.draw();
        player.draw();
        next_frame().await;
    }
}