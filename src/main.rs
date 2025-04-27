use std::f32::consts::{FRAC_PI_2, PI};

use macroquad::prelude::*;

#[macroquad::main("Lissajous Curve Table")]
async fn main() {
    request_new_screen_size(700.0, 700.0);

    let w = 80.0;
    let cols = (screen_width() / w).floor() as i32 - 1;
    let rows = cols;
    let mut angle = 0.0;
    let mut col_circles = vec![];
    let mut row_circles = vec![];
    let mut curves = vec![];

    // Create the column circles
    for i in 0..cols {
        let color = Color::from_rgba((i as f32 * 256.0/cols as f32) as u8, 255, 0, 255);
        col_circles.push(Circle::new(color));
    }

    // Create the row circles
    for j in 0..rows {
        let color = Color::from_rgba(255, 0, (j as f32 * 256.0/cols as f32) as u8, 255);
        row_circles.push(Circle::new(color));
    }

    // Create cols * rows amount of curves
    for _ in 0..rows {
        let mut row = vec![];
        for _ in 0..cols {
            row.push(Curve::new());
        }
        curves.push(row);
    }

    loop {
        clear_background(BLACK);
        draw(rows, cols, w ,angle, &row_circles, &col_circles, &mut curves);
        next_frame().await;

        angle -= PI / 400.0;
        
        if angle < -2.0 * PI {
            for row in curves.iter_mut() {
                for curve in row.iter_mut() {
                    curve.reset();
                }
            }
            angle = 0.0;
        }
    }
}

fn draw(
    rows: i32, cols: i32, w: f32, angle: f32, 
    row_circles: &Vec<Circle>, col_circles: &Vec<Circle>, 
    curves: &mut Vec<Vec<Curve>>) {
    for i in 0..cols {
        let cx = (i as f32 + 1.5) * w;
        let cy = w / 2.0;
        let d = w / 2.0 - 10.0;
        draw_circle_lines(cx, cy, d, 1.0, col_circles[i as usize].color);
        let angle = angle * (i + 1) as f32;
        let x = cx + d * (angle + FRAC_PI_2).cos();
        let y = cy + d * (angle + FRAC_PI_2).sin();
        draw_circle(x, y, 3.0, WHITE);
        draw_line(x, 0.0, x, screen_height(), 1.0, WHITE.with_alpha(0.2));

        for j in 0..rows {
            curves[j as usize][i as usize].set_x(x, row_circles[j as usize].color);
        }
    }

    for j in 0..cols {
        let cy = (j as f32 + 1.5) * w;
        let cx = w / 2.0;
        let d = w / 2.0 - 10.0;
        draw_circle_lines(cx, cy, d, 1.0, row_circles[j as usize].color);
        let angle = angle * (j + 1) as f32;
        let x = cx + d * (angle + FRAC_PI_2).cos();
        let y = cy + d * (angle + FRAC_PI_2).sin();
        draw_circle(x, y, 3.0, WHITE);
        draw_line(0.0, y, screen_width(), y, 1.0, WHITE.with_alpha(0.2));

        for i in 0..cols {
            curves[j as usize][i as usize].set_y(y, col_circles[i as usize].color);
        }
    }

    for j in 0..rows {
        for i in 0..cols {
            curves[j as usize][i as usize].add();
            curves[j as usize][i as usize].show();
        }
    }
}

struct Circle {
    color: Color,
}

impl Circle {
    fn new(color: Color) -> Circle {
        Circle { color }
    }
}

struct ColoredPoint {
    x: f32,
    y: f32, 
    color: Color,
}

impl ColoredPoint {
    fn new(x: f32, y: f32, color: Color) -> ColoredPoint {
        ColoredPoint { x, y, color }
    }
}

/// A Lissajous curve represented as a sequence of points
struct Curve {
    path: Vec<ColoredPoint>,
    current: Vec2,
    color_x: Color,
    color_y: Color,
}

impl Curve {
    /// Create an empty curve
    fn new() -> Curve {
        Curve { path: vec![], current: Vec2::ZERO, color_x: WHITE, color_y: WHITE }
    }

    fn reset(&mut self) {
        self.path.clear();
    }

    /// Sets the x value of the next point to add
    fn set_x(&mut self, value: f32, color: Color) {
        self.current.x = value;
        self.color_x = color;
    }

    /// Sets the y value of the next point to add
    fn set_y(&mut self, value: f32, color: Color) {
        self.current.y = value;
        self.color_y = color;
    }

    /// Add a new point to the curve
    fn add(&mut self) {
        let color = lerp(&self.color_x, &self.color_y);
        self.path.push(ColoredPoint::new(self.current.x, self.current.y, color));
    }

    /// Draw the curve on the screen
    fn show(&self) {
        // draw the curve as a sequence of lines
        for i in 0..self.path.len() - 1 {
            let j = (i + 1) % self.path.len();
            let p = &self.path[i];
            let q = &self.path[j];
            draw_line(p.x, p.y, q.x, q.y, 1.0, q.color);
        }
        
        // highlight the current point in the curve
        if let Some(q) = self.path.last() {
            draw_circle(q.x, q.y, 2.0, WHITE);
        }
    }
}

fn lerp(c1: &Color, c2: &Color) -> Color {
    let r = ((c1.r + c2.r) / 2.0) * 255.0;
    let g = ((c1.g + c2.g) / 2.0) * 255.0;
    let b = ((c1.b + c2.b) / 2.0) * 255.0;
    Color::from_rgba(r as u8, g as u8, b as u8, 255)
}