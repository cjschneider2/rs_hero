extern crate sdl2;

mod fps;
mod events;

use fps::FpsTimer;
use events::handle_event;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let controller_subsystem = sdl_context.game_controller().unwrap();

    let window = video_subsystem
        .window("rs_hero", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut _canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // initialize controllers
    //open_game_controllers();

    // inital loop state
    let mut fps_timer = FpsTimer::new(60.0);
    let mut last_sec = 0;
    let mut last_event = None;

    'main: loop {
        // loop start time
        fps_timer.tick();
        let tick = fps_timer.get_epoch().elapsed().as_secs();
        if tick > last_sec {
            println!("fps: {:?}", fps_timer.get_last_fps());
            last_sec = tick;
        }

        // start event handler
        let new_event = event_pump.poll_event();
        if new_event != last_event {
            if let Some(ref event) = new_event {
                let should_exit = handle_event(&event);
                if should_exit { break 'main; };
            }
        }
        if new_event.is_some() {
            last_event = new_event;
        }
        // end of event handler

        // start frame timing calculations
        fps_timer.sleep_til_next_tick();

    }
}
