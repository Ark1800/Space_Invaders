/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: barrier module, handles barrier movement, drawing, and collisions
*/

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
use crate::modules::preload_image::TextureManager;

pub struct Barrier {
    view: StillImage,
    count: i32
}

impl Barrier {
    pub async fn new(image_path: (Texture2D, Option<Vec<u8>>, String), x: f32, y: f32) -> Self {
        let mut view = StillImage::new(
            "",
            150.0,  // width 
            150.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        view.set_preload(image_path);

        Barrier {
            view,
            count: 1
        }
    }

    pub async fn check_collision(&mut self, img2: &StillImage, tm: &TextureManager) -> bool {
        let mut collided = false;
        if check_collision(self.view_barrier(), img2, 1) {
            self.count += 1;
            self.change_image(self.count, tm).await;
            collided = true;
        }
        collided
    }

    pub async fn change_image(&mut self, count: i32, tm: &TextureManager) {
        if count <= 10 { //so barrier can take 10 hits and then dissapears
            let image_path = format!("assets/barrier_{}.png", count);
            self.view.set_preload(tm.get_preload(&image_path).unwrap());
        }
        else {
            self.view.clear(); //removes image after 9 hits ("" image not working so clear whole thing works too)
        }
    }

    pub fn view_barrier(&self) -> &StillImage {
        &self.view
    }

    pub fn draw(&self) {
        self.view.draw();
    }
}