use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::label::Label;
use crate::modules::image_button::ImageButton;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32) -> (String, TextureManager, i32, i32) {

let textcolor = Color::from_hex(0x20062E);
let bordercolor = Color::from_hex(0x6F2D8A);
let backgroundcolor = Color::from_hex(0xAD81C7);

    let mut btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut lbl_menu = Label::new("Gem Collector", 295.0, 335.0, 110);
    lbl_menu.with_colors(textcolor, Some(bordercolor));
    lbl_menu.with_round(10.0);

    let mut lbl_help = Label::new("Collect each gem and\ndeposit them in the minecart", 350.0, 535.0, 40);
    lbl_help.with_colors(textcolor, Some(bordercolor));
    lbl_help.with_round(10.0);

    let mut lbl_start = Label::new("Press space to start", 370.0, 735.0, 55);
    lbl_start.with_colors(textcolor, Some(bordercolor));
    lbl_start.with_round(10.0);

    loop {
        clear_background(backgroundcolor);

        if is_key_pressed(KeyCode::Space) {
            return ("gemcollector".to_string(), tm, chickenclickerscore, gemcollectorscore);
        }

        if btn_arrowbutton.click() {
            return ("menu".to_string(), tm, chickenclickerscore, gemcollectorscore);
        }

        lbl_menu.draw();
        lbl_help.draw();
        lbl_start.draw();
        next_frame().await;
    }
}