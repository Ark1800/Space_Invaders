/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: player module, handles player movement, drawing, and collisions
*/
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Player {
    view: StillImage,
    move_speed: f32,
    movement: f32,
}

impl Player {
    pub async fn new(image_path: &str, x: f32, y: f32) -> Self {
        let view = StillImage::new(
            image_path,
            90.0,  // width 
            90.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;

        Player {
            view,
            move_speed: 350.0, // Movement speed in pixels per second
            movement: 0.0
        }
    }
    //movement functions
    pub fn handle_keypresses(&mut self, playercurrenttime: f64) -> bool {
        let mut shot = false;
        let mut move_dir= 0.0;
        if is_key_down(KeyCode::D) {
            move_dir += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir -= 1.0;
        }
        let movement = move_dir * self.move_speed * get_frame_time();
        self.movement = movement;
        if playercurrenttime > 0.5 {
            if is_key_down(KeyCode::W) {
            shot = true;
            }
        }
        shot
    }

    pub fn move_x(&mut self) {
        self.view.set_x(self.view.get_x() + self.movement);
    }

    pub fn check_collision(&self, img2: &StillImage) -> bool {
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        }
        collided
    }
    
    //general functions
    pub fn get_oldpos(&self) -> f32 {
        self.view.get_x()
    }

    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
}