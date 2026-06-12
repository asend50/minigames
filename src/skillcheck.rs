use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::image_button::ImageButton;
use crate::modules::still_image::StillImage;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32) -> (String, TextureManager, i32, i32) {

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    loop {
        clear_background(WHITE);

        if btn_arrowbutton.click() {
            return ("skillcheckmenu".to_string(), tm, chickenclickerscore, gemcollectorscore);
        }

        next_frame().await;
    }
}