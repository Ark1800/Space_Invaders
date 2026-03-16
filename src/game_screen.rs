/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: main game loop, where the player, barriers, and enemies will be drawn and updated, also handles collisions and score
*/

use macroquad::prelude::*;
use crate::modules::player::Player;
use crate::modules::barrier::Barrier;
use crate::modules::bullets::Bullet;
use crate::modules::enemy::Enemy;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;

//to dooooo
//1. enemy shooting
//2. player lives and game over screen
//3. high score and score
//4. score tracking and saving 

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager) -> String {
    //VARIABLESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut bullets: Vec<Bullet> = vec![];
    let mut bullets_dir: Vec<f32> = vec![];
    let mut enemies: Vec<Enemy> = vec![];
    let mut barriers: Vec<Barrier> = vec![];
    let mut enemy_dir = 1.0;
    let mut enemy_hitwall = false;
    let mut score = 0;
    use_virtual_resolution(virtual_width, virtual_height);
    //MODULESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut player = Player::new(tm.get_preload("assets/player_ship.png").unwrap(), virtual_width / 2.0, virtual_height - 160.0).await;
    let mut barrier_1 = Barrier::new(tm.get_preload("assets/barrier_1.png").unwrap(), 87.5, virtual_height - 300.0).await;
    let mut barrier_2 = Barrier::new(tm.get_preload("assets/barrier_1.png").unwrap(), 325.0, virtual_height - 300.0).await;
    let mut barrier_3 = Barrier::new(tm.get_preload("assets/barrier_1.png").unwrap(), 562.5, virtual_height - 300.0).await;
    barriers.push(barrier_1);
    barriers.push(barrier_2);
    barriers.push(barrier_3);
    let mut enemy_x = 80.0;
    let mut enemy_y = 100.0;
    let mut enemy_image: (Texture2D, Option<Vec<u8>>, String); //setting first enemy image type
    for i in 0..4 {
        match i {
            0 | 2 => enemy_image = tm.get_preload("assets/enemy_1.png").unwrap(),
            1 | 3 => enemy_image = tm.get_preload("assets/enemy_2.png").unwrap(),
            _ => enemy_image = tm.get_preload("assets/enemy_1.png").unwrap()
        };
        for j in 0..6 {
            let enemy = Enemy::new(enemy_image.clone(), 50.0, 50.0, 100.0 + enemy_x, 100.0 + enemy_y).await;
            enemies.push(enemy);
            enemy_x += 75.0;
        }
        enemy_x = 80.0;
        enemy_y += 75.0;
    }
    //LABELSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS(SSSSSSSS(SSS
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
    bg_img.set_preload(tm.get_preload("assets/spaceinvadersbg.png").unwrap());
    let mut wall_l = StillImage::new(
        "",
        20.0,  // width
        virtual_height, // height
        -20.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    wall_l.set_preload(tm.get_preload("assets/spaceinvadersbg.png").unwrap());
    let mut wall_r = StillImage::new(
        "",
        20.0,  // width
        virtual_height, // height
        virtual_width,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    wall_r.set_preload(tm.get_preload("assets/spaceinvadersbg.png").unwrap());
    let mut heart_1 = StillImage::new(
        "",
        60.0,  // width
        60.0, // height
        virtual_width-210.0,    // x position
        20.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    heart_1.set_preload(tm.get_preload("assets/player_heart.png").unwrap());
    let mut heart_2 = StillImage::new(
        "",
        60.0,  // width
        60.0, // height
        virtual_width-140.0,    // x position
        20.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    heart_2.set_preload(tm.get_preload("assets/player_heart.png").unwrap());
    let mut heart_3 = StillImage::new(
        "",
        60.0,  // width
        60.0, // height
        virtual_width-70.0,    // x position
        20.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    heart_3.set_preload(tm.get_preload("assets/player_heart.png").unwrap());
    let shot_cooldown_secs = 0.20;
    let mut last_shot_time = get_time() - shot_cooldown_secs;
    let mut enemy_shot_time = get_time()- shot_cooldown_secs;
    loop {
        //PLAYERRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR
        let oldpos = player.get_oldpos();
        let mut shot = player.handle_keypresses(get_time() - last_shot_time);
        if shot {
            let bullet = Bullet::new(tm.get_preload("assets/player_bullet.png").unwrap(), player.view_player().get_x() + 40.0, player.view_player().get_y()).await;
            bullets.push(bullet);
            bullets_dir.push(-1.0);
            shot = false;
            last_shot_time = get_time();
        }
        if get_time() - enemy_shot_time >= 1.0 {
            //random_rng = 
            //let bullet = Bullet::new(tm.get_preload("assets/player_bullet.png").unwrap(), player.view_player().get_x() + 40.0, player.view_player().get_y()).await;
            //bullets.push(bullet);
           // bullets_dir.push(-1.0);
            enemy_shot_time = get_time()
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
        //bullet drawing must be in between draws to be above bg. to avoid lag I don't loop through bullets twice, movement and drawing is handled simultaniously.
        let mut rbc = 0; //(remove bullet counter), needs to be kept track of so index doesnt change mid for loop when bullets are removed
        //BULLET LOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
        for i in 0..bullets.len() {
            //if i or index is less then rbc, that means the bullet was removed and loop should stop
            if i < rbc {
                continue;
            }
            let bullet_idx = i - rbc;
            if bullet_idx >= bullets.len() || bullet_idx >= bullets_dir.len() {
                continue;
            }
            //move the bullets and draw them
            bullets[bullet_idx].moving(bullets_dir[bullet_idx]);
            bullets[bullet_idx].draw();
            //CHECK EACH TIME IF A BULLET IS REMOVED and delete each time (only needed for for loop collisions)
            let mut bullet_removed = false;
            //first bullet check if off screen then remove
            if bullets[bullet_idx].get_y() < -40.0 { //bullet size is 30 so make sure its off screen
                bullets.remove(bullet_idx);
                bullets_dir.remove(bullet_idx);
                rbc += 1;
                bullet_removed = true;
                continue;
            }
            //second bullet check if collision with enemy, delete both
            for j in  0..enemies.len() {
                if bullets[bullet_idx].check_collision(enemies[j].view_enemy()) {
                    bullets.remove(bullet_idx);
                    bullets_dir.remove(bullet_idx);
                    enemies.remove(j);
                    rbc += 1;
                    bullet_removed = true;
                    score += 10;
                    lbl_score_num.set_text(score.to_string());
                    break; //breaks out of enemy loop to avoid multiple collisions with one bullet
                }
            }
            if bullet_removed {
                continue;
            }
            for x in 0..barriers.len() {
                let collided = barriers[x].check_collision(bullets[bullet_idx].view_bullet(), tm).await;
                if collided {
                    bullets.remove(bullet_idx);
                    bullets_dir.remove(bullet_idx);
                    rbc += 1;
                    break; //breaks out of barrier loop to avoid multiple collisions with one bullet
                }
            }
            if bullet_removed {
                continue;
            }
        }
        //ENEMY LOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
        for i in 0..enemies.len() {
            enemies[i].movement(enemy_dir);
        }
        if enemies.iter_mut().any(|enemy| enemy.hit_wall(&wall_l, &wall_r)) {
            if !enemy_hitwall {
                enemy_dir = -enemy_dir;
                for i in 0..enemies.len() {
                    enemies[i].move_down();
                }
                enemy_hitwall = true;
            }
        } else {
            enemy_hitwall = false;
        }
        for i in 0..enemies.len() { //so enemies don't dissapear for a bit they have to move and draw seperatly
            enemies[i].draw();
        }
        //BARRIER DRAWING
        for i in 0..barriers.len() {
            barriers[i].draw();
        }
        //REST OF DRAWINGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
        heart_1.draw();
        heart_2.draw();
        heart_3.draw();
        lbl_score_num.draw();
        lbl_score_str.draw();
        player.draw();
        next_frame().await;
    }
}
