use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::image_button::ImageButton;
pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32) -> (String, TextureManager, i32, i32, i32) {

    let mut chickenclickerscorevalue = chickenclickerscore;

    let mut img_farmbackground = StillImage::new(
        "",     // Empty string creates a transparent image
        2300.0,  // width
        1300.0,  // height
        -300.0,  // x position
        -150.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    img_farmbackground.set_preload(tm.get_preload("assets/pixelfarm.png").unwrap());

    let mut btn_chicken = ImageButton::new(
        450.0,  // width
        750.0,  // height
        250.0,  // x position
        250.0,   // y position
        "assets/pixelchicken.png",        // normal state image
        "assets/pixelchickenhover.png",  // hover state image
    ).await;

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut lbl_chickenpoints = Label::new("0", 550.0, 200.0, 100);

    let mut chickenpoints = 0;

    loop {
        clear_background(WHITE);
        img_farmbackground.draw();
        lbl_chickenpoints.draw();

        if btn_arrowbutton.click() {
            return ("clickermenu".to_string(), tm, chickenclickerscorevalue, gemcollectorscore, skillcheckpoints);
        }

        if btn_chicken.click() {
            chickenpoints += 1;
            lbl_chickenpoints.set_text(chickenpoints.to_string());

            if chickenpoints == 100 {
                chickenclickerscorevalue += 1;
                return ("menu".to_string(), tm, chickenclickerscorevalue, gemcollectorscore, skillcheckpoints);
        }
            
        }

        if is_key_pressed(KeyCode::Space) {
            return ("menu".to_string(), tm, chickenclickerscorevalue, gemcollectorscore, skillcheckpoints);
        }
        next_frame().await;
    }
}