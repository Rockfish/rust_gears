use crate::gears::*;
use macroquad::prelude::*;

mod gears;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gears".to_owned(),
        // fullscreen: true,
        window_width: 1200,
        window_height: 1000,
        ..Default::default()
    }
}

//noinspection RsMainFunctionNotFound
#[macroquad::main(window_conf)]
async fn main() {
    let mut config = GearConfig {
        root_circle_radius: 10.0,
        pitch_circle_radius: 11.0,
        outside_circle_radius: 13.0,
        x_offset: 0.0,
        y_offset: 0.0,
        ..Default::default()
    };

    set_involute_offset(&mut config);

    println!("Gear config: \n{:?}", config);
    println!("root_angle in degrees: {}", config.root_angle.to_degrees());

    let rust_logo = load_texture("assets/rust.png").await.unwrap();
    let texture: Texture2D = load_texture("assets/strips.png").await.unwrap();

    loop {
        if let Some(key) = get_last_key_pressed() {
            if key == KeyCode::Escape {
                break;
            }
        }

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 20., 20.),
            up: vec3(0., 1., 0.),
            target: vec3(0.0, 0.0, 0.),
            ..Default::default()
        });

        let color = Color::new(1.0, 1.0, 1.0, 1.00);

        draw_gear_mesh(&config, color, Some(&texture));

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube(vec3(-5., 1., -2.), vec3(5., 5., 5.), Some(&rust_logo), WHITE);


        // draw_gear(&config, BLACK);
        // draw_gear(&config, DARKBROWN);
        // draw_spokes(&config, DARKGREEN);
        //
        // let color = Color::new(0.75, 0.75, 0.00, 1.00);
        // draw_gear_points(&config, color);
        // draw_gear_triangles(&config, color);


        // draw_config_circles(&config);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

fn draw_config_circles(config: &GearConfig) {
    let color = Color::new(0.75, 0.75, 0.00, 1.00);
    draw_poly_lines(
        config.x_offset,
        config.y_offset,
        100,
        config.root_circle_radius,
        0.0,
        1.0,
        color,
    );
    draw_poly_lines(
        config.x_offset,
        config.y_offset,
        100,
        config.pitch_circle_radius,
        0.0,
        1.0,
        color,
    );
    draw_poly_lines(
        config.x_offset,
        config.y_offset,
        100,
        config.outside_circle_radius,
        0.0,
        1.0,
        color,
    );
}
