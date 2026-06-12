use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::image_button::ImageButton;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32) -> (String, TextureManager, i32, i32, i32) {
    let mut switchvalue = 0;

    let mut lbl_title = Label::new("Epic Minigames", 250.0, 200.0, 100);

    let mut lbl_description = Label::new("Beat each minigame to collect each item.\nCollect every item in order to beat the game.", 220.0, 300.0, 35);

    let mut lbl_chickenclicker = Label::new("Chicken Clicker", 90.0, 815.0, 30);

    let mut lbl_gemcollector = Label::new("Gem Collector", 350.0, 815.0, 30);

    let mut lbl_skillcheck = Label::new("Skill Check", 620.0, 815.0, 30);

    let mut img_background = StillImage::new(
        "",
        1450.0,  // width
        1300.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_background.set_preload(tm.get_preload("assets/desktopbackground.png").unwrap());
    

    let mut btn_filefolder1 = ImageButton::new(
        125.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;

    let mut btn_filefolder2 = ImageButton::new(
        375.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;

    let mut btn_filefolder3 = ImageButton::new(
        625.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;

    let mut btn_filefolder4 = ImageButton::new(
        875.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;
    
    let mut img_egg = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        200.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_gem = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        400.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_gear = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        600.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    loop {
        clear_background(WHITE);
        img_background.draw();
        img_egg.draw();
        img_gem.draw();
        img_gear.draw();
        lbl_title.draw();
        lbl_description.draw();
        lbl_chickenclicker.draw();
        lbl_gemcollector.draw();
        lbl_skillcheck.draw();

        if btn_filefolder1.click() {
            return ("clickermenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

        if btn_filefolder2.click() {
            return ("gemcollectormenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

        if btn_filefolder3.click() {
            return ("skillcheckmenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

        if btn_filefolder4.click() {
            return ("gemcollectormenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints);
        }

    if chickenclickerscore == 1 {
        img_egg.set_preload(tm.get_preload("assets/pixelegg.png").unwrap());
        btn_filefolder1.enabled = false;
    }

    if gemcollectorscore == 1 {
        img_gem.set_preload(tm.get_preload("assets/diamond.png").unwrap());
        btn_filefolder2.enabled = false;
    }

    if skillcheckpoints == 1 {
        img_gear.set_preload(tm.get_preload("assets/gear.png").unwrap());
        btn_filefolder3.enabled = false;
    }


        next_frame().await;
    }
}