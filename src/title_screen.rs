/*
By: <Your Name Here>
Date: 2026-03-03
Program Details: <Program Description Here>
*/

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;
use crate::modules;
use crate::modules::grid::draw_grid;

pub async fn run() -> String {
    let mut lbl_howto = Label::new("Welcome to Space Invaders! \n Use A and D to move across the screen and dodge lasers! \n Press space to fire back! \n Don't shoot your barriers \n and don't let the enemies either! \n Good luck! and don't let them get too close...", 50.0, 400.0, 30);
    lbl_howto.with_colors(WHITE, Some(WHITE));
    lbl_howto.with_alignment(modules::label::TextAlign::Center);
    let title_img = StillImage::new(
        "assets/SpaceInvadersLogo.png",
        600.0,  // width
        260.0,  // height
        100.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let btn_play = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Click Me",
        BLUE,
        GREEN,
        30
    );
    loop {
        clear_background(BLUE);
        draw_grid(50.0, BLACK);

        title_img.draw();
        lbl_howto.draw();
        next_frame().await;
    }
}