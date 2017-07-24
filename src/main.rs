#![allow(dead_code)]

extern crate sdl2;
extern crate byteorder;

mod error;
mod sdl;
mod game;
mod fps;

use fps::FpsTimer;

fn main() {

    let mut sdl = sdl::Sdl::new().unwrap();

    // initialize controllers
    sdl.open_game_controllers();

    // inital loop state
    let mut fps_timer = FpsTimer::new(60.0);
    let mut last_sec = 0;

    'main: loop {
        // loop start time
        // NOTE: This is just to show the FPS once per second in the log
        fps_timer.tick();
        let tick = fps_timer.get_epoch().elapsed().as_secs();
        if tick > last_sec {
            println!("fps: {:?}", fps_timer.get_last_fps());
            last_sec = tick;
        }

        if sdl.handle_events() {
            break 'main;
        }

        // start frame timing calculations
        fps_timer.sleep_til_next_tick();

    }
}
