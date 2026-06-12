use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::image_button::ImageButton;
use crate::modules::still_image::StillImage;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32) -> (String, TextureManager, i32, i32, i32) {

let textcolor = Color::from_hex(0x4D1641);
let bordercolor = Color::from_hex(0xC959AB);
let backgroundcolor = Color::from_hex(0xFFA8E9);

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut img_skillcheck = StillImage::new(
        "",
        700.0,  // width
        300.0,  // height
        250.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_skillcheck.set_preload(tm.get_preload("assets/skillchecktitle.png").unwrap());

    let mut lbl_help = Label::new("Press your spacebar when the\n bar is over the green box.\n   Reach 25 points to win", 350.0, 500.0, 40);
    lbl_help.with_colors(textcolor, Some(bordercolor));
    lbl_help.with_round(10.0);

    let mut lbl_start = Label::new("Press space to start", 350.0, 735.0, 55);
    lbl_start.with_colors(textcolor, Some(bordercolor));
    lbl_start.with_round(10.0);
    loop {
        clear_background(backgroundcolor);

        if is_key_pressed(KeyCode::Space) {
            return ("skillcheck".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

        if btn_arrowbutton.click() {
            return ("menu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }
    


        img_skillcheck.draw();
        lbl_help.draw();
        lbl_start.draw();
        next_frame().await;
    }
}