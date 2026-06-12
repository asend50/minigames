use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::image_button::ImageButton;
use crate::modules::still_image::StillImage;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32) -> (String, TextureManager, i32, i32, i32) {
    
let textcolor = Color::from_hex(0x573D1A);
let bordercolor = Color::from_hex(0x9C7E4E);
let backgroundcolor = Color::from_hex(0xD1B78E);

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut img_chickenclicker = StillImage::new(
        "",
        800.0,  // width
        300.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_chickenclicker.set_preload(tm.get_preload("assets/chickenclickertitle.png").unwrap());

    let mut lbl_help = Label::new("Click on the chicken 100\ntimes to collect the egg.", 375.0, 535.0, 40);
    lbl_help.with_colors(textcolor, Some(bordercolor));
    lbl_help.with_round(10.0);

    let mut lbl_start = Label::new("Press space to start", 350.0, 735.0, 55);
    lbl_start.with_colors(textcolor, Some(bordercolor));
    lbl_start.with_round(10.0);

    loop {
        clear_background(backgroundcolor);

        if is_key_pressed(KeyCode::Space) {
            return ("clicker".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

        if btn_arrowbutton.click() {
            return ("menu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

        img_chickenclicker.draw();
        lbl_help.draw();
        lbl_start.draw();
        next_frame().await;
    }
}