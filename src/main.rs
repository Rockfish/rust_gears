use crate::gears::*;
use macroquad::prelude::*;

mod gears;

#[macroquad::main("Gears")]
async fn main() {
    let mut config = GearConfig {
        x_offset: 300.0,
        y_offset: 300.0,
        ..Default::default()
    };

    println!("Original config: \n{:?}", config);

    let mut config2 = GearConfig {
        x_offset: 300.0,
        y_offset: 300.0,
        ..Default::default()
    };

    set_involute_offset(&mut config2);

    println!("After config: \n{:?}", config2);
    println!("root_angle in degrees: {}", config2.root_angle.to_degrees());

    loop {

        if let Some(key) = get_last_key_pressed() {
            if key == KeyCode::Escape {
                break;
            }
        }

        clear_background(GRAY);

        // draw_gear(&config, BLACK);
        draw_gear(&config2, DARKBROWN);
        draw_spokes(&config2, DARKGREEN);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
