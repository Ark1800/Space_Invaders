/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: player module, handles player movement, drawing, and collisions
*/
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Bullet {
    view: StillImage,
    move_speed: f32,
    movement: f32,
}

impl Bullet {
    pub async fn new(image_path: (Texture2D, Option<Vec<u8>>, String), x: f32, y: f32) -> Self {
        let mut view = StillImage::new(
            "",
            10.0,  // width 
            30.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        view.set_preload(image_path);
        Bullet {
            view,
            move_speed: 700.0, // Movement speed in pixels per second
            movement: 0.0
        }
    }

    pub fn moving(&mut self, direction : f32) {
        let movement = direction * self.move_speed * get_frame_time();
        self.movement = movement;
        self.view.set_y(self.view.get_y() + self.movement);
    }

    pub fn check_collision(&self, img2: &StillImage) -> bool {
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_bullet(), img2, 1) {
            collided = true;
        }
        collided
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_bullet(&self) -> &StillImage {
        &self.view
    }

    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

}