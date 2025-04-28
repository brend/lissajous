use std::{cmp, f32::consts::{FRAC_PI_2, PI}};
use std::time::{Duration, Instant};

use macroquad::prelude::*;

/// Size of a grid cell (circle)
const W: f32 = 80.0;

/// Target frames per second
const TARGET_FPS: Option<f32> = None;
/// Uncomment the line below to set a target frame rate,
/// for example if your machine slows down towards the end of the animation
// const TARGET_FPS: Option<f32> = Some(30.0);

#[macroquad::main("Lissajous Curve Table")]
async fn main() {
    // Scene setup
    let mut scene = Scene::new();
    let mut sw = screen_width();
    let mut sh = screen_height();

    // Compute the ideal duration of a single frame
    let frame_time = match TARGET_FPS {
        Some(tfps) => Some(Duration::from_secs_f32(1.0 / tfps)),
        None => None
    };

    // Drawing and updating loop
    loop {
        let frame_start = Instant::now();

        clear_background(BLACK);
        scene.show();
        next_frame().await;

        // Update the scene (mostly the angle)
        scene.update();

        // Manually limit the frame rate
        let elapsed = frame_start.elapsed();
        if let Some(frame_time) = frame_time {
            if elapsed < frame_time {
                let sleep_duration = frame_time - elapsed;
                std::thread::sleep(std::time::Duration::from_millis(sleep_duration.as_millis() as u64));
            }
        }

        // If the screen size has changed,
        // reset the scene to fit the new size
        if screen_width() != sw || screen_height() != sh {
            sw = screen_width();
            sh = screen_height();
            scene = Scene::new();
        }
    }
}

/// Data structure to hold all objects in the scene
struct Scene {
    /// Row circles
    rows: Vec<Circle>,
    /// Column circles
    cols: Vec<Circle>,
    /// The rows and columns of curves being drawn
    curves: Vec<Vec<Curve>>,
    /// The angle that marks the circles' progress
    angle: f32,
}

impl Scene {
    /// Creates a new scene
    fn new() -> Scene {
        // Compute the number of rows and columns on screen
        let cols = (screen_width() / W).floor() as i32 - 1;
        let rows = (screen_height() / W).floor() as i32 - 1;
        let cols = cmp::min(cols, rows);
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

        Scene {
            rows: row_circles,
            cols: col_circles,
            curves,
            angle: 0.0,
        }
    }

    /// Draws the scene on the screen
    fn show(&mut self) {
        // Draw the horizontal row of circles
        // Also update the circle progress
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

        // Draw the vertical column of circles
        // Also update the circle progress
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

        // Update the curves with the current circle progress
        for j in 0..self.rows.len() {
            for i in 0..self.cols.len() {
                self.curves[j][i].add();
                self.curves[j][i].show();
            }
        }
    }

    /// Update the scene by changing the angle
    /// Angle is reset occasionally
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

/// A colored circle
struct Circle {
    /// X coordinate of the center of the circle
    x: f32,
    /// Y coordinate of the center of the circle
    y: f32,
    /// Diameter of the circle
    d: f32,
    /// Color of the circle
    color: Color,
}

impl Circle {
    /// Creates a new circle with a position, diameter, and color
    fn new(x: f32, y: f32, d: f32, color: Color) -> Circle {
        Circle { x, y, d, color }
    }
}

/// A colored point
struct ColoredPoint {
    /// X coordinate of the point
    x: f32,
    /// Y coordinate of the point
    y: f32, 
    /// Color of the point
    color: Color,
}

impl ColoredPoint {
    /// Creates a new colored point with x, y, and color
    fn new(x: f32, y: f32, color: Color) -> ColoredPoint {
        ColoredPoint { x, y, color }
    }
}

/// A Lissajous curve represented as a sequence of points
struct Curve {
    /// The points that form the curve
    path: Vec<ColoredPoint>,
    /// The point currently being constructed by the circles' progress
    current: Vec2,
    /// The color of the circle that contributes the curve's x coordinate
    color_x: Color,
    /// The color of the circle that contributes the curve's y coordinate
    color_y: Color,
}

impl Curve {
    /// Create an empty curve
    fn new() -> Curve {
        Curve { path: vec![], current: Vec2::ZERO, color_x: WHITE, color_y: WHITE }
    }

    /// Reset the curve by removing all of its points
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
        if self.path.len() < 2 {
            return;
        }

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

/// Performs linear interpolation between to colors
fn lerp(c1: &Color, c2: &Color) -> Color {
    let r = ((c1.r + c2.r) / 2.0) * 255.0;
    let g = ((c1.g + c2.g) / 2.0) * 255.0;
    let b = ((c1.b + c2.b) / 2.0) * 255.0;
    Color::from_rgba(r as u8, g as u8, b as u8, 255)
}