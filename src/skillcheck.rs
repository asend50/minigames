use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::image_button::ImageButton;
use crate::modules::still_image::StillImage;
use macroquad::input::KeyCode;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32) -> (String, TextureManager, i32, i32, i32) {

let mut skillcheckpointsvalue = skillcheckpoints;
let mut skillcheckscore = 0;

let textcolor = Color::from_hex(0x134700);
let bordercolor = Color::from_hex(0x9ADE78);
let backgroundcolor = Color::from_hex(0x2D5418);

let textcolorred = Color::from_hex(0x470000);
let bordercolorred = Color::from_hex(0xE04A4A);

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut img_bar = StillImage::new(
        "",
        700.0,  // width
        125.0,  // height
        200.0,  // x position
        800.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_bar.set_preload(tm.get_preload("assets/bar.png").unwrap());
    let mut bar = img_bar;

    let mut img_greenbar = StillImage::new(
        "",
        150.0,  // width
        125.0,  // height
        475.0,  // x position
        800.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_greenbar.set_preload(tm.get_preload("assets/greenbar.png").unwrap());
    let mut greenbar = img_greenbar;

    let mut img_blackbar = StillImage::new(
        "",
        10.0,  // width
        175.0,  // height
        100.0,  // x position
        775.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_blackbar.set_preload(tm.get_preload("assets/blackbar.png").unwrap());
    let mut blackbar = img_blackbar;

    let mut img_redbackground = StillImage::new(
        "",
        1300.0,  // width
        1300.0,  // height
        1400.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_redbackground.set_preload(tm.get_preload("assets/redbackground.png").unwrap());
    let mut redbackground = img_redbackground;

    let mut lbl_skillcheckpoints = Label::new("Points: 0", 400.0, 535.0, 80);
    lbl_skillcheckpoints.with_colors(textcolor, Some(bordercolor));
    lbl_skillcheckpoints.with_round(10.0);

    const MOVE_SPEED: f32 = 200.0;
    let mut x= 2.0;
    let mut y= 0.0;
    let mut redbackgroundx = redbackground.pos();

    loop {
        clear_background(backgroundcolor);
        redbackground.draw();
        bar.draw();
        greenbar.draw();
        blackbar.draw();
        lbl_skillcheckpoints.draw();

        if btn_arrowbutton.click() {
            return ("skillcheckmenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpointsvalue);
        }

        let mut pos = blackbar.pos();

            if pos.x == 100.0 && pos.y == 775.0 {
                x= 5.0;
                y= 0.0;
            }

            else if pos.x == 950.0 && pos.y == 775.0 {
                x= -5.0;
                y= 0.0;
            }

            pos.y +=y;
            pos.x +=x;
            blackbar.set_position(pos);


            if is_key_pressed(KeyCode::Space) && blackbar.get_x() >= 475.0 && blackbar.get_x() <= 625.0 {
                skillcheckscore += 1;
                lbl_skillcheckpoints.set_text(format!("Points: {}", skillcheckscore));
                lbl_skillcheckpoints.with_colors(textcolor, Some(bordercolor));
                redbackgroundx.x = 1400.0;
                redbackground.set_position(redbackgroundx);
     
            }

            if is_key_pressed(KeyCode::Space) && blackbar.get_x() >= 200.0 && blackbar.get_x() <= 475.0 {
                skillcheckscore = 0;
                lbl_skillcheckpoints.set_text(format!("Points: {}", skillcheckscore));
                lbl_skillcheckpoints.with_colors(textcolorred, Some(bordercolorred));
                redbackgroundx.x = 0.0;
                redbackground.set_position(redbackgroundx);
            }

            if is_key_pressed(KeyCode::Space) && blackbar.get_x() >= 625.0 && blackbar.get_x() <= 900.0 {
                skillcheckscore = 0;
                lbl_skillcheckpoints.set_text(format!("Points: {}", skillcheckpointsvalue));
                lbl_skillcheckpoints.with_colors(textcolorred, Some(bordercolorred));
                redbackgroundx.x = 0.0;
                redbackground.set_position(redbackgroundx);
            }

            if skillcheckscore == 25 {
                skillcheckpointsvalue = 1;
                return ("menu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpointsvalue);
            }

        next_frame().await;
    }
}