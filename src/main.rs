extern crate piston_window;

use piston_window::{WindowSettings, PistonWindow};

use piston_window::clear;

fn main() {
    println!("Hello, world!");

    let opengl = piston_window::OpenGL::V3_3;

    let mut window: PistonWindow = WindowSettings::new("Tetris", (640, 480))
        .fullscreen(false)
        .vsync(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let window_mat = piston_window::math::scale(1.0 / 320.0, -1.0 / 240.0);

    let tetris_color = [1.0, 0.0, 0.0, 1.0];
    let tetris_size = 10.0;

    let timer_base = 1.0;
    let mut timer = timer_base;

    let mut x = 4.0;
    let mut y = 0.0;

    while let Some(e) = window.next() {
        use piston_window::{UpdateEvent, RenderEvent};
        if let Some(u) = e.update_args() {
            timer -= u.dt;
            while timer < 0.0 {
                timer += timer_base;
                y += 1.0;
            }
        }

        if let Some(r) = e.render_args() {
            window.draw_2d(&e, |_c, g| {
                clear([0.5, 1.0, 0.5, 1.0], g);

                use piston_window::*;
                let tetris_field = [-5.0, -10.0, 10.0, 20.0];
                let tetris_mat = math::multiply(math::multiply(window_mat,
                    math::scale(tetris_size, tetris_size)),
                    math::translate([tetris_field[0], tetris_field[1]]));
                rectangle([0.0, 0.35, 0.85, 1.0],
                    [-1.0,
                    -1.0,
                    tetris_field[2] + 2.0,
                    tetris_field[3] + 2.0,],
                    tetris_mat, g);
                rectangle([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, tetris_field[2], tetris_field[3]], tetris_mat, g);

                let piece = 0;
                let rotation = 0;

                match piece {
                    0 => {
                        // Straight piece
                        rectangle(tetris_color, [x, y, 1.0, 1.0], tetris_mat, g);
                        rectangle(tetris_color, [x + 1.0, y, 1.0, 1.0], tetris_mat, g);
                        rectangle(tetris_color, [x + 2.0, y, 1.0, 1.0], tetris_mat, g);
                        rectangle(tetris_color, [x - 1.0, y, 1.0, 1.0], tetris_mat, g);
                    },
                    _ => unimplemented!(),
                }

            });
        }
    }
}
