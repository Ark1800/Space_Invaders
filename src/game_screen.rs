/*
By: <Your Name Here>
Date: 2026-03-03
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::modules::player;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager, _assets: Vec<&str>) -> String {
    use_virtual_resolution(virtual_width, virtual_height);
    //MODULESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut player = player::Player::new("assets/player_ship.png", virtual_width / 2.0, virtual_height - 160.0).await;
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
    if let Some(preloaded) = tm.get_preload("assets/spaceinvadersbg.png") {
       bg_img.set_preload(preloaded);
    } else {
       bg_img.set_preload(tm.get_preload_by_index(5).unwrap()); //set to background galaxy texture if error
    }
    let wall_l = StillImage::new(
        "",
        20.0,  // width
        virtual_height, // height
        -20.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let wall_r = StillImage::new(
        "",
        20.0,  // width
        virtual_height, // height
        virtual_width,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
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
        player.handle_keypresses();
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
        player.draw();
        next_frame().await;
    }
}