use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Enemy {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    enemy_shotdelay: f64,
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
            enemy_shotdelay: 1.5, // Time in seconds between shots
        }
    }

    pub fn movement(&mut self, enemy_dir: f32) {
        self.movement.x = self.move_speed * get_frame_time() * enemy_dir;
        self.view.set_x(self.view.get_x() + self.movement.x);
    }

    pub fn hit_wall(&mut self, wall_l: &StillImage, wall_r: &StillImage) -> bool {
        let mut hit_wall = false; 
        if check_collision(&self.view, wall_l, 1) || check_collision(&self.view, wall_r, 1) {
            hit_wall = true;
        }
        hit_wall
    }

     pub fn draw(&self) {
        self.view.draw();
    }

    pub fn move_down(&mut self) {
        self.view.set_y(self.view.get_y() + 20.0);
    }

    pub fn view_enemy(&self) -> &StillImage {
        &self.view
    }

    pub fn get_shot_delay(&self) -> f64 {
        self.enemy_shotdelay
    }

    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    pub fn increase_stats(&mut self, count: f32) {
        self.move_speed += count * 250.0; // Increase movement speed by 25 pixels per second (count is in 0.1)
        self.enemy_shotdelay -= count as f64; // Decrease shot delay by 0.1 seconds
        if self.enemy_shotdelay <= 0.1 {
            self.enemy_shotdelay = 0.1; // Set a minimum shot delay to prevent it from becoming too fast
        }
    }
}
