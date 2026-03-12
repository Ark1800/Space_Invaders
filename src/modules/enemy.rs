use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Enemy {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl Enemy {
    pub async fn new(image_path: (Texture2D, Option<Vec<u8>>, String), width: f32, height: f32, x: f32, y: f32) -> Self {
        let mut view = StillImage::new(
            "",
            width,  // width 
            height,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        view.set_preload(image_path);
        Enemy {
            view,
            move_speed: 200.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
        }
    }

    pub fn movement(&mut self, wall_l: &StillImage, wall_r: &StillImage) -> bool {
        let  mut move_down = false;
        self.movement.x = self.move_speed * get_frame_time();
        if check_collision(&self.view, wall_l, 1) || check_collision(&self.view, wall_r, 1) {
            self.move_speed = -self.move_speed;
            move_down = true;
        }
        self.view.set_x(self.view.get_x() + self.movement.x);
        move_down
    }

     pub fn draw(&self) {
        self.view.draw();
    }

    pub fn move_down(&mut self) {
        self.view.set_y(self.view.get_y() + 50.0);
    }

    pub fn view_enemy(&self) -> &StillImage {
        &self.view
    }
}
