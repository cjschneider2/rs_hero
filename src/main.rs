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

    // initialize controllers
    sdl.open_game_controllers();

    // inital state
    let mut game = game::Game::new(width, height);
    let mut fps_timer = FpsTimer::new(60.0);
    // let mut last_sec = 0;

    let mut texture = ::std::cell::RefCell::new(
        sdl.texture_creator.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::ABGR8888,
        width, height).unwrap());

    'main: loop {
        // loop start time
        // NOTE: This is just to show the FPS once per second in the log
        fps_timer.tick();
        // let tick = fps_timer.get_epoch().elapsed().as_secs();
        // if tick > last_sec {
        //     println!("fps: {:?}", fps_timer.get_last_fps());
        //     last_sec = tick;
        // }

        // handle events
        let (exit, dim) = sdl.handle_events(&mut game);
        if exit { break 'main; }
        if let Some((x, y)) = dim {
            texture = ::std::cell::RefCell::new(
                sdl.texture_creator.create_texture_streaming(
                    sdl2::pixels::PixelFormatEnum::ABGR8888,
                    x as u32, y as u32).unwrap());
            game.resize_buffer(x as u32, y as u32);
        }

        // poll controllers for input??? TODO: Isn't this in handle_inputs?
        //sdl.poll_controller_input();

        // Update game
        game.update_and_render();

        // sound output test

        // render our window
        let res = sdl::draw_buffer(&mut sdl.canvas,
                                  &game.render_buffer.memory,
                                  &mut texture.borrow_mut(),
                                  game.render_buffer.width,
                                  game.render_buffer.height,
                                  game.render_buffer.pitch as usize);
        match res {
            Ok(_) => (),
            Err(e) => {println!("{:?}", e); panic!();}
        }

        // start frame timing calculations
        fps_timer.sleep_til_next_tick();

    }
}
