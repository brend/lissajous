use std::f32::consts::{FRAC_PI_2, PI};

use macroquad::prelude::*;

#[macroquad::main("Lissajous Curve Table")]
async fn main() {
    request_new_screen_size(700.0, 700.0);

    let w = 80.0;
    let cols = (screen_width() / w).floor() as i32 - 1;
    let rows = cols;
    let mut angle = 0.0;
    let mut curves = vec![];

    for _ in 0..rows {
        let mut row = vec![];
        for _ in 0..cols {
            row.push(Curve::new());
        }
        curves.push(row);
    }

    loop {
        clear_background(BLACK);
        draw(rows, cols, w ,angle, &mut curves);
        next_frame().await;
        angle -= PI / 300.0;
    }
}

fn draw(rows: i32, cols: i32, w: f32, angle: f32, curves: &mut Vec<Vec<Curve>>) {
    for i in 0..cols {
        let cx = (i as f32 + 1.5) * w;
        let cy = w / 2.0;
        let d = w / 2.0 - 10.0;
        draw_circle_lines(cx, cy, d, 1.0, WHITE);
        let angle = angle * (i + 1) as f32;
        let x = cx + d * (angle + FRAC_PI_2).cos();
        let y = cy + d * (angle + FRAC_PI_2).sin();
        draw_circle(x, y, 3.0, WHITE);
        draw_line(x, 0.0, x, screen_height(), 1.0, WHITE.with_alpha(0.2));

        for j in 0..rows {
            curves[j as usize][i as usize].set_x(x);
        }
    }

    for j in 0..cols {
        let cy = (j as f32 + 1.5) * w;
        let cx = w / 2.0;
        let d = w / 2.0 - 10.0;
        draw_circle_lines(cx, cy, d, 1.0, WHITE);
        let angle = angle * (j + 1) as f32;
        let x = cx + d * (angle + FRAC_PI_2).cos();
        let y = cy + d * (angle + FRAC_PI_2).sin();
        draw_circle(x, y, 3.0, WHITE);
        draw_line(0.0, y, screen_width(), y, 1.0, WHITE.with_alpha(0.2));

        for i in 0..cols {
            curves[j as usize][i as usize].set_y(y);
        }
    }

    for j in 0..rows {
        for i in 0..cols {
            curves[j as usize][i as usize].add();
            curves[j as usize][i as usize].show();
        }
    }
}

struct Curve {
    path: Vec<Vec2>,
    current: Vec2,
}

impl Curve {
    fn new() -> Curve {
        Curve { path: vec![], current: Vec2::ZERO }
    }

    fn set_x(&mut self, value: f32) {
        self.current.x = value;
    }

    fn set_y(&mut self, value: f32) {
        self.current.y = value;
    }

    fn add(&mut self) {
        self.path.push(self.current);
    }

    fn show(&self) {
        // draw the curve as a sequence of lines
        for i in 0..self.path.len() - 1 {
            let j = (i + 1) % self.path.len();
            let p = self.path[i];
            let q = self.path[j];
            draw_line(p.x, p.y, q.x, q.y, 1.0, WHITE);
        }
        
        // highlight the current point in the curve
        if let Some(q) = self.path.last() {
            draw_circle(q.x, q.y, 2.0, WHITE);
        }
    }
}