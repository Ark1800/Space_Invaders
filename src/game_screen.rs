/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: main game loop, where the player, barriers, and enemies will be drawn and updated, also handles collisions and score
*/

use std::usize;

use macroquad::prelude::*;
use crate::modules::collision::check_collision;
use crate::modules::player::Player;
use crate::modules::barrier::Barrier;
use crate::modules::bullets::Bullet;
use crate::modules::enemy::Enemy;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;
use miniquad::date;
//to do
//1. add what happens if enemies reach barriers
//2. fix textbox taking text during game

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager) -> (String, i32) {
    //VARIABLESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut bullets: Vec<Bullet> = vec![];
    let mut bullets_dir: Vec<f32> = vec![];
    let mut barriers: Vec<Barrier> = vec![];
    let mut enemy_dir = 1.0;
    let mut enemy_hitwall = false;
    let mut enemy_level_count: f32 = 0.0;
    let mut score = 0;
    let mut playerhealth = 3;
    let mut currentlevel = 1;
    rand::srand(date::now() as u64);
    use_virtual_resolution(virtual_width, virtual_height);
    //MODULESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut player = Player::new(tm.get_preload("assets/player_ship.png").unwrap(), virtual_width / 2.0, virtual_height - 160.0).await;
    let barrier_1 = Barrier::new(tm.get_preload("assets/barrier_1.png").unwrap(), 87.5, virtual_height - 300.0).await;
    let barrier_2 = Barrier::new(tm.get_preload("assets/barrier_1.png").unwrap(), 325.0, virtual_height - 300.0).await;
    let barrier_3 = Barrier::new(tm.get_preload("assets/barrier_1.png").unwrap(), 562.5, virtual_height - 300.0).await;
    let (mut enemies, mut column_1, mut column_2, mut column_3, mut column_4, mut column_5, mut column_6) = enemy_creation(tm).await;
    barriers.push(barrier_1);
    barriers.push(barrier_2);
    barriers.push(barrier_3);
    //LABELSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS(SSSSSSSS(SSS
    let mut lbl_score = Label::new("Score: 0", 20.0, 40.0, 60);
    let mut lbl_levelclear = Label::new("Level Clear!", virtual_width / 2.0 - 200.0, virtual_height / 2.0 - 50.0, 80);
    let mut lbl_level = Label::new("Level: 1", 20.0, 100.0, 60);
    lbl_level.with_colors(WHITE, Some(DARKGRAY));
    lbl_levelclear.with_colors(WHITE, Some(DARKGRAY));
    lbl_score.with_colors(WHITE, Some(DARKGRAY));
    lbl_levelclear.set_visible(false);
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
    let mut hearts: Vec<StillImage> = vec![];
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
    hearts.push(heart_1);
    hearts.push(heart_2);
    hearts.push(heart_3);
    let mut last_shot_time = get_time();
    let mut enemy_shot_time = get_time();
    let mut levelclear_time = 0.0;
    loop {
        let mut pause = false;
        if get_time() - levelclear_time < 2.0 {
            pause = true;
        }
        else {
            pause = false
        }
        if get_time() <= 2.0 { //if the program hasn't even been open for two seconds pause is false (if player goes to game screen to quick)
            pause = false;
        }
        lbl_levelclear.set_visible(pause);
        if !pause {
            //PLAYERRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR
            let oldpos = player.get_oldpos();
            let mut shot = player.handle_keypresses(get_time() - last_shot_time);
            if shot {
                let bullet = Bullet::new(tm.get_preload("assets/player_bullet.png").unwrap(), player.view_player().get_x()+40.0, player.view_player().get_y()-35.0).await;
                bullets.push(bullet);
                bullets_dir.push(-1.0);
                shot = false;
                last_shot_time = get_time();
            }
            //ENEMYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
            if get_time() - enemy_shot_time >= enemies[0].get_shot_delay() { 
                let mut column_indexpass = true;
                let mut index = 0;
                while column_indexpass {
                    let random_num = rand::gen_range(0, 6);
                    match random_num {
                        0 => {
                            if !column_1.is_empty() {
                                index = column_1[column_1.len() - 1];
                                column_indexpass = false;
                            }
                        }
                        1 => {
                            if !column_2.is_empty() {
                                index = column_2[column_2.len() - 1];
                                column_indexpass = false;
                            }
                        }
                        2 => {
                            if !column_3.is_empty() {
                                index = column_3[column_3.len() - 1];
                                column_indexpass = false;
                            }
                        }
                        3 => {
                            if !column_4.is_empty() {
                                index = column_4[column_4.len() - 1];
                                column_indexpass = false;
                            }
                        }
                        4 => {
                            if !column_5.is_empty() {
                                index = column_5[column_5.len() - 1];
                                column_indexpass = false;
                            }
                        }
                        _ => {
                            if !column_6.is_empty() {
                                index = column_6[column_6.len() - 1];
                                column_indexpass = false;
                            }
                        }
                    }

                    if column_1.is_empty() && column_2.is_empty() && column_3.is_empty() && column_4.is_empty() && column_5.is_empty() && column_6.is_empty(){
                        break; //break if every column is empty
                    }
                }

                if !column_1.is_empty() || !column_2.is_empty() || !column_3.is_empty() || !column_4.is_empty() || !column_5.is_empty() || !column_6.is_empty()
                {
                    let x = enemies[index].view_enemy().get_x();
                    let y = enemies[index].view_enemy().get_y();
                    let bullet = Bullet::new(tm.get_preload("assets/player_bullet.png").unwrap(), x + 25.0, y + 60.0).await; //+ numbers for middle and below enemy
                    bullets.push(bullet);
                    bullets_dir.push(1.0);
                    enemy_shot_time = get_time();
                }
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
            let mut rbc = 0; //[remove bullet counter], needs to be kept track of so index doesnt change mid for loop when bullets are removed
        //BULLET LOOPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
            for i in 0..bullets.len() {
                //if i or index is less then rbc, that means the bullet was removed and loop should stop
                if i < rbc {
                    continue;
                }
                let bullet_index = i - rbc;
                if bullet_index >= bullets.len() || bullet_index >= bullets_dir.len() {
                    continue;
                }
                //move the bullets and draw them
                bullets[bullet_index].moving(bullets_dir[bullet_index]);
                bullets[bullet_index].draw();
                //CHECK EACH TIME IF A BULLET IS REMOVED and delete each time [only needed for for loop collisions]
                let mut bullet_removed = false;
                //first bullet check if off screen then remove
                if bullets[bullet_index].get_y() < -40.0 { //bullet size is 30 so make sure its off screen
                    bullets.remove(bullet_index);
                    bullets_dir.remove(bullet_index);
                    rbc += 1;
                    bullet_removed = true;
                    continue;
                }
                //player collision with bullets
                if check_collision(player.view_player(), bullets[bullet_index].view_bullet(), 1) {
                    if playerhealth > 0 {
                        playerhealth -= 1;
                    }
                    if playerhealth <= 0 {
                        return ("gameover_screen".to_string(), score);
                    }
                    bullets.remove(bullet_index);
                    bullets_dir.remove(bullet_index);
                    rbc += 1;
                    bullet_removed = true;
                    continue;
                }
                //second bullet check if collision with enemy, delete both
                for j in  0..enemies.len() {
                    if bullets[bullet_index].check_collision(enemies[j].view_enemy()) {
                        bullets.remove(bullet_index);
                        bullets_dir.remove(bullet_index);
                        let columns = [&mut column_1, &mut column_2, &mut column_3, &mut column_4, &mut column_5, &mut column_6,];
                            for column in columns { //.iter gives each item a string, .position checks everyting, if x == j, CHOP THAT SHIT
        
                                if let Some(position) = column.iter().position(|&index| index == j) {
                                    column.remove(position);
                                }                      
                                for index in column.iter_mut() {
                                    if *index > j {
                                        *index -= 1;
                                        }
                                }
                            }
                        enemies.remove(j);
                        rbc += 1;
                        bullet_removed = true;
                        score += 10;
                        lbl_score.set_text(format!("Score: {}", score));
                        break; //breaks out of enemy loop to avoid multiple collisions with one bullet
                    }
                }
                if bullet_removed {
                    continue;
                }
                for x in 0..barriers.len() {
                    let collided = barriers[x].check_collision(bullets[bullet_index].view_bullet(), tm).await;
                    if collided {
                        bullets.remove(bullet_index);
                        bullets_dir.remove(bullet_index);
                        rbc += 1;
                        bullet_removed = true;
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
            }        //AI helped with iter.mut idea, goes through every enemy and checks if they hit the wal individually, avoiding conflict of simultaneous collision
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
            //RESETINGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
            for i in 0..enemies.len() {
                if enemies[i].get_y() >= virtual_height - 300.0 { //if enemy reaches barrier line, game over
                    return ("gameover_screen".to_string(), score);
                }
            }
            if enemies.is_empty() {
                bullets.clear();
                bullets_dir.clear();
                for i in 0..barriers.len() {
                    barriers[i].reset_count(); //resets barrier health for next level
                    barriers[i].change_image(1, tm).await; //resets barriers for next level
                }
                (enemies, column_1, column_2, column_3, column_4, column_5, column_6) = enemy_creation(tm).await;
                enemy_level_count += 0.1;
                for i in 0..enemies.len() {
                    enemies[i].increase_stats(enemy_level_count); //increase enemy stats each level
                }
                currentlevel += 1;
                lbl_level.set_text(format!("Level: {}", currentlevel));
                levelclear_time = get_time();
            }
        }
        if pause { //only draw in this order when pause
            bg_img.draw();
            wall_l.draw();
            wall_r.draw();
        }
        //ENEMIES DRAWING
        for i in 0..enemies.len() { //so enemies don't dissapear for a bit they have to move and draw seperatly
            enemies[i].draw();
        }
        //BARRIER DRAWING
        for i in 0..barriers.len() {
            barriers[i].draw();
        }
        //REST OF DRAWINGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
        for heart in &hearts[0..playerhealth] {
            if playerhealth > 0 {
                heart.draw();
            }
        }
        lbl_score.draw();
        lbl_levelclear.draw();
        lbl_level.draw();
        player.draw();
        next_frame().await;
    }
}

async fn enemy_creation(tm: &TextureManager) -> (Vec<Enemy>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut enemies: Vec<Enemy> = vec![];
    let mut enemy_x = 80.0;
    let mut enemy_y = 100.0;
    let mut enemy_image: (Texture2D, Option<Vec<u8>>, String); //setting first enemy image type
    let mut column_1: Vec<usize> = vec![];
    let mut column_2: Vec<usize> = vec![];
    let mut column_3: Vec<usize> = vec![];
    let mut column_4: Vec<usize> = vec![];
    let mut column_5: Vec<usize> = vec![];
    let mut column_6: Vec<usize> = vec![];
    for i in 0..6 {
        match i {
            0 | 2 => enemy_image = tm.get_preload("assets/enemy_1.png").unwrap(),
            1 | 3 => enemy_image = tm.get_preload("assets/enemy_2.png").unwrap(),
            _ => enemy_image = tm.get_preload("assets/enemy_1.png").unwrap()
        };
        for j in 0..4 {
            let enemy = Enemy::new(enemy_image.clone(), 50.0, 50.0, 100.0 + enemy_x, 100.0 + enemy_y).await;
            enemies.push(enemy);
            let j = enemies.len() - 1;
            enemy_y += 75.0;
                match i {
                    0 => column_1.push(j), //needs to store as index cause enemy can't be cloned
                    1 => column_2.push(j),
                    2 => column_3.push(j), 
                    3 => column_4.push(j),
                    4 => column_5.push(j),
                    _ => column_6.push(j)
                };
        }
        enemy_y = 100.0;
        enemy_x += 75.0;
    }
    (enemies, column_1, column_2, column_3, column_4, column_5, column_6)
}