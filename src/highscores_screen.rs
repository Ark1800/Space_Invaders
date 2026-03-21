/*
By: Andrew Campbell
Date: 2026-03-03
Program Details: displaying highscores
*/

use macroquad::prelude::*;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::preload_image::TextureManager;
use crate::modules::listview::ListView;
use crate::modules::textfiles::TextFile;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager) -> String {
    use_virtual_resolution(virtual_width, virtual_height);
    let result = TextFile::load_strings("highscores.txt").await;
    let mut scores: Vec<String> = result.unwrap_or_default();
    let mut score_entries: Vec<(i32, String)> = Vec::new();
    //split up the numbers from the string
    for score in &scores {
        let (_, score_num) = score.split_once(':').unwrap_or((score, "0"));
        let parsed_score = score_num.trim().parse::<i32>().unwrap_or(0);
        score_entries.push((parsed_score, score.clone()));
    }
    //sort i32 score and clones
    score_entries.sort();
    //sorted list
    let mut sorted_scores: Vec<String> = Vec::new();
    //reverse the order to have highest scores at the top and go through
    for i in (0..score_entries.len()).rev() { //.1 is help from AI it only adds 2nd half of tuple (number), so list can be whole again (not just numbers) (.1 adds name, and entries has the numbers)
        sorted_scores.push(score_entries[i].1.clone());
    }
    scores = sorted_scores;
    let mut list_view = ListView::new(&scores, 50.0, 200.0, 60);
    list_view.with_colors(YELLOW, Some(BLACK), Some(LIGHTGRAY));
    list_view.set_width(700.0);
    list_view.with_max_visible_items(11);
    //LABELLSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut lbl_highscores = Label::new(format!("High Scores"), 250.0, 100.0, 60);
    lbl_highscores.with_colors(WHITE, Some(DARKGRAY));
    //IMAGESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS AND BUTTONSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
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
    let mut btn_return = TextButton::new(
        50.0,
        1000.0,
        700.0,
        100.0,
        "Return",
        BLACK,
        RED,
        30
    );
    btn_return.with_text_color(YELLOW);
    btn_return.with_round(10.0);
    loop {
        bg_img.draw();
        list_view.draw();
        lbl_highscores.draw();
        if btn_return.click() {
            return "title_screen".to_string();
        }
        next_frame().await;
    }
}