#![allow(dead_code)]

use core::f32::consts::*;
use macroquad::math::{bool, f32};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct GearConfig {
    // pub radius: f32,
    pub step_angle: f32,
    pub outside_circle: f32,
    pub pitch_circle: f32,
    pub base_circle: f32,
    pub root_circle: f32,
    pub involute_offset: f32,
    pub pressure_angle: f32,
    pub pitch: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    // for animating rotation
    // pub spin: f32,
    pub root_angle: f32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Front,
    Back,
}

impl Default for GearConfig {
    fn default() -> Self {
        Self {
            // radius: 200.0,
            step_angle: 24.0,
            outside_circle: 230.0,
            pitch_circle: 200.0,
            base_circle: 0.0,
            root_circle: 175.0,
            involute_offset: 0.0,
            pressure_angle: 20.0,
            pitch: 30.0,
            x_offset: 50.0,
            y_offset: 50.0,
            // spin: 0.0,
            root_angle: 0.0,
        }
    }
}

fn y_coordinate(a: f32, r: f32) -> f32 {
    a.to_radians().cos() * r
}

fn x_coordinate(a: f32, r: f32) -> f32 {
    a.to_radians().sin() * r
}

fn draw_radius(config: GearConfig, angle: f32, radius_var: f32) {
    let x = x_coordinate(angle, radius_var);
    let y = y_coordinate(angle, radius_var);
    // g.draw(new Line2D.Double(x_offset, y_offset, x+x_offset, y+y_offset));
    draw_line(
        config.x_offset,
        config.y_offset,
        x + config.x_offset,
        y + config.y_offset,
        15.0,
        BLUE,
    );
}

fn draw_tangent(angle: f32, radius: f32, step: f32) {
    let x = x_coordinate(angle, radius);
    let y = y_coordinate(angle, radius);
    let tan_angle = angle + 90.0;
    let tan_len = step / 180.0 * PI * radius;
    let mut tx = x_coordinate(tan_angle, tan_len);
    let mut ty = y_coordinate(tan_angle, tan_len);
    tx += x;
    ty += y;
    // g.draw(new Line2D.Double(x, y, tx, ty));
}

fn circle(radius: f32) {
    for a in (0..361).step_by(3) {
        let angle = a as f32 / 180.0 * PI;
        let x = angle.cos() * radius;
        let y = angle.cos() * radius;
        // if a == 0 {
        //     p.moveTo(x, y);
        // } else {
        //     p.lineTo(x, y);
        // }
    }
    // p.closePath();
    // g.draw(p);
}

//
// To draw the involute in the right position we need to know where to start so that it intersects
// the pitch circle in the right place.
//
// Calculate length of tangent from root_circle to pitch_circle:
//
// 		Start with right triangle who's base, or adjacent, length equals root_circle radius
//      and a hypotenuse that equals pitch_circle radius.
//      The acute angle of the triangle is arccos(root_circle/pitch_cirle).
//
//      From the angle, the opposite side, which is the tangent equals sin(angle)*pitch_circle.
//
// The tangent length is proportional to the angle on the root_circle for the value of the involute at that point.
//
//		The angle for the involute off the root_circle is the root_angle which equals tangent_length/root_circle in radians.
//		That angle is length of the arc on the root circle between the involute's origin and where it intersects the pitch circle.
//
// The difference between the root_angle and the original angle is the offset we need to know to start the involute
// so that it will intersect the pitch circle at the right point.
//
//

pub fn set_involute_offset(config: &mut GearConfig) {

    // Set the root_circle from the pitch_circle and pressure_angle
    config.base_circle = config.pitch_circle * config.pressure_angle.to_radians().cos();

    println!("base_circle: {}", config.base_circle);

    // Outside circle intersection
    // Length of tangent from root circle to pitch circle.
    let tan_intersect_angle = (config.root_circle / config.pitch_circle).acos();
    let tangent_length = tan_intersect_angle.sin() * config.pitch_circle;

    // Angle = arc / radius
    // Angle in radians where the involute intersects the outside circle
    // Can use this angle to calculate the endpoints of the involute curves.
    config.root_angle = tangent_length / config.root_circle;

    // Offset is the distance between the angle on the pitch circle and
    // where the involute needs to start at to intersect pitch circle correctly.
    config.involute_offset = (config.root_angle - tan_intersect_angle).to_degrees();
}

fn involute(config: &GearConfig, angle: f32, len: f32, direction: Direction) -> Vec<Vec2> {
    let mut line_path: Vec<Vec2> = Vec::new();

    let starting_angle: f32;
    let inc: f32;
    let right_angle: f32;

    if direction == Direction::Back {
        starting_angle = angle;
        inc = 5.0;
        right_angle = -90.0;
    } else {
        starting_angle = angle + config.step_angle;
        inc = -5.0;
        right_angle = 90.0;
    }

    let mut current_angle = starting_angle;
    // let first = true;

    loop {
        // base position - offsets
        let bx = x_coordinate(current_angle, len);
        let by = y_coordinate(current_angle, len);

        let tan_angle = current_angle + right_angle;

        // Actually the length of the arc from the starting_angle to the current_angle
        // And since this is an involute, the arc length straighten out will be the length of tangent segment
        let tan_len = (starting_angle - current_angle).abs().to_radians() * config.root_circle;

        // Get the tangent vector x, y 
        let tx = x_coordinate(tan_angle, tan_len);
        let ty = y_coordinate(tan_angle, tan_len);

        let involute_x = bx + tx;
        let involute_y = by + ty;

        // check involute length is still less then the outside circle radius
        let involute_radius = (involute_x * involute_x + involute_y * involute_y).sqrt();
        if involute_radius > config.outside_circle {
            break;
        }

        current_angle += inc;

        line_path.push(Vec2::new(involute_x, involute_y));
    }

    if direction == Direction::Back {
        line_path.reverse();
    }
    line_path
}

fn add_tooth(config: &GearConfig, line_path: &mut Vec<Vec2>, angle: f32, len: f32) {
    let mut front_side = involute(
        config,
        angle, // + config.involute_offset,
        len,
        Direction::Front,
    );
    let mut back_side = involute(
        config,
        angle + config.step_angle / 2.0 - config.involute_offset, // - 5.0,
        len,
        Direction::Back,
    );

    line_path.append(&mut front_side);
    line_path.append(&mut back_side);
}

pub fn gear_path(config: &GearConfig) -> Vec<Vec2> {
    let mut line_path: Vec<Vec2> = Vec::new();

    for i in (0..15).rev() {
        let angle = (i as f32) * config.step_angle;
        add_tooth(config, &mut line_path, angle, config.root_circle);
    }

    line_path
}

pub fn gear_spokes(config: &GearConfig) -> Vec<(Vec2, Vec2)> {
    let mut spokes: Vec<(Vec2, Vec2)> = Vec::new();

    for i in (0..15).rev() {
        let angle = (i as f32) * config.step_angle;
        let (sin, cos) = angle.to_radians().sin_cos();
        let v1 = Vec2::new(config.x_offset,config.y_offset);
        let v2 = Vec2::new(config.outside_circle*cos + config.x_offset, config.outside_circle*sin + config.y_offset);
        spokes.push((v1, v2));
    }
    spokes
}

pub fn draw_gear(config: &GearConfig, color: Color) {
    let line_path = gear_path(&config);
    let num_segments = line_path.len() - 1;

    for i in 0..num_segments {
        let v1 = line_path[i] + Vec2::new(config.x_offset, config.y_offset);
        let v2 = line_path[i + 1] + Vec2::new(config.x_offset, config.y_offset);
        draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, color);
    }

    let v1 = line_path[num_segments] + Vec2::new(config.x_offset, config.y_offset);
    let v2 = line_path[0] + Vec2::new(config.x_offset, config.y_offset);
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, color);

    draw_circle_lines(config.x_offset, config.y_offset, 15.0, 2.0, color);

}

pub fn draw_spokes(config: &GearConfig, color: Color) {
    let spokes = gear_spokes(&config);
    for s in spokes {
        draw_line(s.0.x, s.0.y, s.1.x, s.1.y, 2.0, color);
    }
}