use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::image_button::ImageButton;
use crate::modules::collision::check_collision;
use macroquad::input::KeyCode;


pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32) -> (String, TextureManager, i32, i32, i32) {

    let mut gemcollectorscorevalue = gemcollectorscore;

    let backgroundbrown = Color::from_hex(0x42240B);

    let mut img_cave = StillImage::new(
        "",
        1300.0,  // width
        1300.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_cave.set_preload(tm.get_preload("assets/cave.png").unwrap());
    let mut cave = img_cave;

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut img_stevesprite = StillImage::new(
        "",
        160.0,  // width
        210.0,  // height
        400.0,  // x position
        200.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_stevesprite.set_preload(tm.get_preload("assets/stevespriteright.png").unwrap());
    let mut steve = img_stevesprite;

    let mut img_minecart = StillImage::new(
        "",
        130.0,  // width
        100.0,  // height
        320.0,  // x position
        225.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_minecart.set_preload(tm.get_preload("assets/minecart.png").unwrap());
    let mut minecart = img_minecart;

    let mut img_diamond = StillImage::new(
        "",
        50.0,  // width
        50.0,  // height
        900.0,  // x position
        775.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_diamond.set_preload(tm.get_preload("assets/diamond.png").unwrap());
    let mut diamond = img_diamond;

    let mut img_gold = StillImage::new(
        "",
        50.0,  // width
        50.0,  // height
        720.0,  // x position
        600.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_gold.set_preload(tm.get_preload("assets/gold.png").unwrap());
    let mut gold = img_gold;

    let mut img_emerald = StillImage::new(
        "",
        50.0,  // width
        50.0,  // height
        150.0,  // x position
        450.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_emerald.set_preload(tm.get_preload("assets/emerald.png").unwrap());
    let mut emerald = img_emerald;

    let mut img_amethyst = StillImage::new(
        "",
        50.0,  // width
        50.0,  // height
        150.0,  // x position
        850.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_amethyst.set_preload(tm.get_preload("assets/amethyst.png").unwrap());
    let mut amethyst = img_amethyst;
    

// Speed of movement in pixels per second
    const MOVE_SPEED: f32 = 200.0;

    let mut steveright = 0;
    let mut steveleft = 0;

    let mut gemhold = 0;

    loop {
        clear_background(backgroundbrown);
        cave.draw();

        if btn_arrowbutton.click() {
            return ("gemcollectormenu".to_string(), tm, chickenclickerscore, gemcollectorscorevalue, skillcheckpoints);
        }
// player movement

    // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 4.0;
        steve.set_preload(tm.get_preload("assets/stevespriteright.png").unwrap());
        steveright = 1;
        steveleft = 0;
        
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 4.0;
        steve.set_preload(tm.get_preload("assets/stevespriteleft.png").unwrap());
        steveright = 0;
        steveleft = 1;
        
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.y += 4.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.y -= 4.0;
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = steve.pos();

        // Move X first
        if movement.x != 0.0 {
            steve.set_x(steve.get_x() + movement.x);
            if check_collision(&steve, &cave, 1) {
                steve.set_x(old_pos.x); // Undo if collision happens
            }
            if check_collision(&steve, &minecart, 1) {
                steve.set_x(old_pos.x); // Undo if collision happens
            }
        }

        // Move Y next
        if movement.y != 0.0 {
            steve.set_y(steve.get_y() + movement.y);
            if check_collision(&steve, &cave, 1) {
                steve.set_y(old_pos.y); // Undo if collision happens
            }
            if check_collision(&steve, &minecart, 1) {
                steve.set_y(old_pos.y); // Undo if collision happens
            }
        }
        

        // Diamond Grabbing
    if check_collision(&steve, &diamond, 1) {
            if steveright == 1 {
            diamond.set_x(steve.get_x() + 90.0);
            diamond.set_y(steve.get_y() + 90.0);
            }
            
            else if steveleft == 1 {
            diamond.set_x(steve.get_x() + 20.0);
            diamond.set_y(steve.get_y() + 90.0);
            }
            }

    if check_collision(&diamond, &minecart, 1) {
            diamond.set_x(320.0);
            diamond.set_y(200.0);
            }

        // Gold Grabbing
    if check_collision(&steve, &gold, 1) {
            if steveright == 1 {
            gold.set_x(steve.get_x() + 90.0);
            gold.set_y(steve.get_y() + 90.0);
            }
            
            else if steveleft == 1 {
            gold.set_x(steve.get_x() + 20.0);
            gold.set_y(steve.get_y() + 90.0);
            }
            }

    if check_collision(&gold, &minecart, 1) {
            gold.set_x(350.0);
            gold.set_y(200.0);
            }

        // Emerald Grabbing
    if check_collision(&steve, &emerald, 1) {
            if steveright == 1 {
            emerald.set_x(steve.get_x() + 90.0);
            emerald.set_y(steve.get_y() + 90.0);
            }
            
            else if steveleft == 1 {
            emerald.set_x(steve.get_x() + 20.0);
            emerald.set_y(steve.get_y() + 90.0);
            }
            }

    if check_collision(&emerald, &minecart, 1) {
            emerald.set_x(380.0);
            emerald.set_y(200.0);
            }

        // Amethyst Grabbing
    if check_collision(&steve, &amethyst, 1) {
            if steveright == 1 {
            amethyst.set_x(steve.get_x() + 90.0);
            amethyst.set_y(steve.get_y() + 90.0);
            }
            
            else if steveleft == 1 {
            amethyst.set_x(steve.get_x() + 20.0);
            amethyst.set_y(steve.get_y() + 90.0);
            }
            }

    if check_collision(&amethyst, &minecart, 1) {
            amethyst.set_x(410.0);
            amethyst.set_y(200.0);
            }

    if diamond.get_x() == 320.0 && diamond.get_y() == 200.0 && gold.get_x() == 350.0 && gold.get_y() == 200.0 && emerald.get_x() == 380.0 && emerald.get_y() == 200.0 && amethyst.get_x() == 410.0 && amethyst.get_y() == 200.0 {
    gemcollectorscorevalue += 1;
    return ("menu".to_string(), tm, chickenclickerscore, gemcollectorscorevalue, skillcheckpoints);
    }

        steve.draw();
        minecart.draw();
        diamond.draw();
        gold.draw();
        emerald.draw();
        amethyst.draw();
        next_frame().await;
    }
}
