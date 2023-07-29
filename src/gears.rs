#![allow(dead_code)]

use core::f32::consts::*;
use macroquad::math::{bool, f32};
use macroquad::models::Vertex;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct GearConfig {
    // pub radius: f32,
    pub step_angle: f32,
    pub outside_circle_radius: f32,
    pub pitch_circle_radius: f32,
    pub base_circle_radius: f32,
    pub root_circle_radius: f32,
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
enum ToothFace {
    Front,
    Back,
}

impl Default for GearConfig {
    fn default() -> Self {
        Self {
            // radius: 200.0,
            root_circle_radius: 300.0,
            pitch_circle_radius: 350.0,
            outside_circle_radius: 390.0,
            base_circle_radius: 0.0,
            step_angle: 24.0,
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

fn draw_radius(config: GearConfig, angle: f32, radius: f32) {
    let (sin, cos) = angle.to_radians().sin_cos();
    let x = radius * cos;
    let y = radius * sin;
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

fn get_x_y(angle: f32, radius: f32) -> (f32, f32) {
    let (sin, cos) = angle.to_radians().sin_cos();
    let x = radius * cos;
    let y = radius * sin;
    (x, y)
}

fn draw_tangent(angle: f32, radius: f32, step: f32) {
    let (x, y) = get_x_y(angle, radius);
    let tan_angle = angle + 90.0;
    let tan_len = step / 180.0 * PI * radius;
    let (mut tx, mut ty) = get_x_y(tan_angle, tan_len);
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
    config.base_circle_radius =
        config.pitch_circle_radius * config.pressure_angle.to_radians().cos();

    println!("base_circle: {}", config.base_circle_radius);

    // Outside circle intersection
    // Length of tangent from root circle to pitch circle.
    let tan_intersect_angle = (config.root_circle_radius / config.pitch_circle_radius).acos();
    let tangent_length = tan_intersect_angle.sin() * config.pitch_circle_radius;

    // Angle = arc / radius
    // Angle in radians where the involute intersects the outside circle
    // Can use this angle to calculate the endpoints of the involute curves.
    config.root_angle = tangent_length / config.root_circle_radius;

    // Offset is the distance between the angle on the pitch circle and
    // where the involute needs to start at to intersect pitch circle correctly.
    config.involute_offset = (config.root_angle - tan_intersect_angle).to_degrees();
}

fn involute(config: &GearConfig, angle: f32, base_radius: f32, tooth_face: ToothFace) -> Vec<Vec2> {
    let mut line_path: Vec<Vec2> = Vec::new();

    let starting_angle: f32;
    let inc: f32;
    let right_angle: f32;

    if tooth_face == ToothFace::Front {
        //starting_angle = angle + config.step_angle;
        starting_angle = angle;
        inc = 5.0;
        right_angle = -90.0;
    } else {
        starting_angle = angle + config.step_angle / 3.0; // todo: calculate angle offset for back side
        inc = -5.0;
        right_angle = 90.0;
    }

    let mut current_angle = starting_angle;
    // let first = true;

    loop {
        // Get position on the base circle for the tangent vector
        let (sin, cos) = current_angle.to_radians().sin_cos();
        let base_x = base_radius * cos;
        let base_y = base_radius * sin;

        // The tangent is at a right angle to the current angle
        let tangent_angle = current_angle + right_angle;

        // Get the length of the arc from the starting angle to the current angle.
        // The length of the arc is the length of the tangent segment for the involute.
        let tangent_segment_length =
            (starting_angle - current_angle).abs().to_radians() * config.root_circle_radius;

        // Get the position on the involute curve the tangent segment points to.
        let (sin, cos) = tangent_angle.to_radians().sin_cos();
        let tangent_endpoint_x = tangent_segment_length * cos;
        let tangent_endpoint_y = tangent_segment_length * sin;

        let involute_x = base_x + tangent_endpoint_x;
        let involute_y = base_y + tangent_endpoint_y;

        // check involute length is still less then the outside circle radius
        let involute_radius = (involute_x * involute_x + involute_y * involute_y).sqrt();
        if involute_radius > config.outside_circle_radius {
            break;
        }

        line_path.push(Vec2::new(involute_x, involute_y));
        current_angle += inc;
    }

    if tooth_face == ToothFace::Back {
        line_path.reverse();
    }
    line_path
}

fn add_tooth(config: &GearConfig, line_path: &mut Vec<Vec2>, angle: f32, radius: f32) {
    let mut front_side = involute(
        config,
        angle, // + config.involute_offset,
        radius,
        ToothFace::Front,
    );
    let mut back_side = involute(
        config,
        angle + config.step_angle / 2.0 - config.involute_offset, // - 5.0,
        radius,
        ToothFace::Back,
    );

    line_path.append(&mut front_side);
    line_path.append(&mut back_side);
}

pub fn gear_path(config: &GearConfig) -> Vec<Vec2> {
    let mut line_path: Vec<Vec2> = Vec::new();

    // for i in (0..15).rev() {
    for i in 0..15 {
        let angle = (i as f32) * config.step_angle;
        add_tooth(config, &mut line_path, angle, config.root_circle_radius);
    }

    line_path
}

pub fn gear_spokes(config: &GearConfig) -> Vec<(Vec2, Vec2)> {
    let mut spokes: Vec<(Vec2, Vec2)> = Vec::new();

    // for i in (0..15).rev() {
    for i in 0..15 {
        let angle = (i as f32) * config.step_angle;
        let (sin, cos) = angle.to_radians().sin_cos();
        let v1 = Vec2::new(config.x_offset, config.y_offset);
        let v2 = Vec2::new(
            config.outside_circle_radius * cos + config.x_offset,
            config.outside_circle_radius * sin + config.y_offset,
        );
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

    // draw_circle_lines(config.x_offset, config.y_offset, 15.0, 2.0, color);
}

pub fn draw_gear_points(config: &GearConfig, color: Color) {
    let line_paths = gear_path(&config);
    for p in line_paths {
        draw_poly_lines(
            p.x + config.x_offset,
            p.y + config.y_offset,
            4,
            2.0,
            0.0,
            1.0,
            color,
        );
    }
}

pub fn draw_gear_triangles(config: &GearConfig, color: Color) {
    let line_paths = gear_path(&config);
    for p in line_paths {
        draw_line(
            config.x_offset,
            config.y_offset,
            p.x + config.x_offset,
            p.y + config.y_offset,
            2.0,
            color,
        );
    }
}

pub fn draw_spokes(config: &GearConfig, color: Color) {
    let spokes = gear_spokes(&config);
    for s in spokes {
        draw_line(s.0.x, s.0.y, s.1.x, s.1.y, 2.0, color);
    }
}


pub fn draw_gear_mesh(config: &GearConfig, color: Color, texture: impl Into<Option<Texture2D>>) {
    let mut vertices = vec![Vertex {
        position: Vec3 {
            x: config.x_offset,
            // y: config.y_offset,
            // z: 0.0,
            y: 0.0,
            z: config.y_offset,
        },
        uv: Vec2 { x: 0.0, y: 0.0 },
        color,
    }];

    vertices.extend(gear_path(&config).iter().enumerate().map(|iv| {
        let v = iv.1;
        let i = iv.0 as i32;
        let uv = if i % 2 == 0 { Vec2{ x: 1.0 , y: 0.0 } } else { Vec2 { x: 1.0, y: 1.0 } };
        Vertex {
            position: Vec3 {
                x: config.x_offset + v.x,
                // y: config.y_offset + v.y,
                // z: 0.0,
                y: 0.0,
                z: config.y_offset + v.y,
            },
            uv,
            color,
        }
    }));

    let num_vertices = vertices.len();
    let mut indices: Vec<u16> = Vec::new();

    for i in 1..num_vertices - 1 {
        indices.push(0);
        indices.push(i as u16);
        indices.push((i + 1) as u16);
    }

    indices.push(0);
    indices.push(1);
    indices.push((num_vertices - 1) as u16);


    let mesh = Mesh {
        vertices,
        indices,
        texture: texture.into()
    };

    draw_mesh(&mesh);
}
