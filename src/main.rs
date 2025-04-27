use std::f32::consts::{FRAC_PI_2, PI};

use macroquad::prelude::*;

const W: f32 = 80.0;

#[macroquad::main("Lissajous Curve Table")]
async fn main() {
    request_new_screen_size(700.0, 700.0);

    let cols = (screen_width() / W).floor() as i32 - 1;
    let rows = cols;
    let mut col_circles = vec![];
    let mut row_circles = vec![];
    let mut curves = vec![];

    // Create the column circles
    for i in 0..cols {
        let cx = (i as f32 + 1.5) * W;
        let cy = W / 2.0;
        let d = W / 2.0 - 10.0;
        let color = Color::from_rgba((i as f32 * 256.0/cols as f32) as u8, 255, 0, 255);
        col_circles.push(Circle::new(cx, cy, d, color));
    }

    // Create the row circles
    for j in 0..rows {
        let cy = (j as f32 + 1.5) * W;
        let cx = W / 2.0;
        let d = W / 2.0 - 10.0;
        let color = Color::from_rgba(255, 0, (j as f32 * 256.0/cols as f32) as u8, 255);
        row_circles.push(Circle::new(cx, cy, d, color));
    }

    // Create cols * rows amount of curves
    for _ in 0..rows {
        let mut row = vec![];
        for _ in 0..cols {
            row.push(Curve::new());
        }
        curves.push(row);
    }

    let mut scene = Scene { rows: row_circles, cols: col_circles, curves, angle: 0.0 };

    loop {
        clear_background(BLACK);
        scene.show();
        next_frame().await;
        scene.update();
    }
}

struct Scene {
    rows: Vec<Circle>,
    cols: Vec<Circle>,
    curves: Vec<Vec<Curve>>,
    angle: f32,
}

impl Scene {
    fn show(&mut self) {
        for i in 0..self.cols.len() {
            let circle = &self.cols[i];
            draw_circle_lines(circle.x, circle.y, circle.d, 1.0, circle.color);
            let angle = self.angle * (i + 1) as f32;
            let x = circle.x + circle.d * (angle + FRAC_PI_2).cos();
            let y = circle.y + circle.d * (angle + FRAC_PI_2).sin();
            draw_circle(x, y, 3.0, WHITE);
            draw_line(x, 0.0, x, screen_height(), 1.0, WHITE.with_alpha(0.2));

            for j in 0..self.rows.len() {
                self.curves[j][i].set_x(x, self.rows[j].color);
            }
        }

        for j in 0..self.rows.len() {
            let circle = &self.rows[j];
            draw_circle_lines(circle.x, circle.y, circle.d, 1.0, circle.color);
            let angle = self.angle * (j + 1) as f32;
            let x = circle.x + circle.d * (angle + FRAC_PI_2).cos();
            let y = circle.y + circle.d * (angle + FRAC_PI_2).sin();
            draw_circle(x, y, 3.0, WHITE);
            draw_line(0.0, y, screen_width(), y, 1.0, WHITE.with_alpha(0.2));

            for i in 0..self.cols.len() {
                self.curves[j][i].set_y(y, self.cols[i].color);
            }
        }

        for j in 0..self.rows.len() {
            for i in 0..self.cols.len() {
                self.curves[j][i].add();
                self.curves[j][i].show();
            }
        }
    }

    fn update(&mut self) {
        self.angle -= PI / 400.0;
        
        if self.angle < -2.0 * PI {
            for row in self.curves.iter_mut() {
                for curve in row.iter_mut() {
                    curve.reset();
                }
            }
            self.angle = 0.0;
        }
    }
}

struct Circle {
    x: f32,
    y: f32,
    d: f32,
    color: Color,
}

impl Circle {
    fn new(x: f32, y: f32, d: f32, color: Color) -> Circle {
        Circle { x, y, d, color }
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