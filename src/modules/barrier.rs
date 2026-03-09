/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: barrier module, handles barrier movement, drawing, and collisions
*/

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Barrier {
    view: StillImage,
    count: i32
}

impl Barrier {
    pub async fn new(image_path: &str, x: f32, y: f32) -> Self {
        let view = StillImage::new(
            image_path,
            150.0,  // width 
            150.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;

        Barrier {
            view,
            count: 0
        }
    }

    pub async fn check_collision(&mut self, img2: &StillImage) {
        if check_collision(self.view_barrier(), img2, 1) {
            self.count += 1;
            self.change_image(self.count).await;
        }
    }

    pub async fn change_image(&mut self, count: i32) {
        let new_image_path = format!("assets/barrier_{}.png", count);
        self.view.set_texture(&new_image_path).await;
    }

    pub fn view_barrier(&self) -> &StillImage {
        &self.view
    }

    pub fn draw(&self) {
        self.view.draw();
    }
}