#![allow(dead_code)]

extern crate sdl2;
extern crate byteorder;

mod error;
mod sdl;
mod game;
mod fps;

use fps::FpsTimer;

fn main() {

    let width = 400;
    let height = 400;
    let mut sdl = sdl::Sdl::new(width, height).unwrap();
    sdl.init_game_controllers().unwrap();

    // inital state
    let mut game = game::Game::new(width, height);
    let mut fps_timer = FpsTimer::new(60.0);
    // let mut last_sec = 0;

    'main: loop {
        // loop start time
        // NOTE: This is just to show the FPS once per second in the log
        fps_timer.tick();
        /*
        let tick = fps_timer.get_epoch().elapsed().as_secs();
        if tick > last_sec {
            let dur = fps_timer.get_frame_time();
            let (sec, ns) = (dur.as_secs(), dur.subsec_nanos() as f32);
            println!("Frame time: {}(s):{}ms", sec, ns / 1_000_000.0);
            last_sec = tick;
        }
        */

        // handle events
        let (exit, dim) = sdl.handle_events(&mut game);
        if exit { break 'main; }
        if let Some((x, y)) = dim {
            game.resize_buffer(x as u32, y as u32);
        }

        // Update game
        game.update_and_render();

        // sound output test

        // render our window
        sdl.draw_buffer_surface(&game.render_buffer.memory).unwrap();

        // start frame timing calculations
        fps_timer.sleep_til_next_tick();
    }
}
