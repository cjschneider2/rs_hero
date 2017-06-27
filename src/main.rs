extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time;

fn main() {

   let sdl_context = sdl2::init().unwrap();
   let video_subsystem = sdl_context.video().unwrap();

   let window = video_subsystem.window("rs_hero", 400, 400, )
                               .position_centered()
                               .opengl()
                               .build()
                               .unwrap();

   let mut canvas = window.into_canvas().present_vsync().build().unwrap();
   let mut event_pump = sdl_context.event_pump().unwrap();

   // inital loop state
   let time_epoch = time::Instant::now();
   let fps = 60.0f64;
   let mut last_sec = 0;
   let mut last_event = None;
   let mut frames = 0;
   let mut fps_as_ns = (1.0 / fps) * 1_000_000_000f64;

   'main: loop {
      // loop start time
      let time_start = time::Instant::now();
      let time_sec = time_epoch.elapsed().as_secs();
      if  time_sec > last_sec {
         println!("fps: {:?}", frames);
         frames = 0; 
         last_sec = time_sec;
      } else {
         frames += 1;
      }

      // start event handler
      let new_event = event_pump.poll_event();
      if new_event != last_event {
         if let Some(ref event) = new_event {
            match event {
                 &Event::Quit { .. }
               | &Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                  break 'main;
               },
               _ => {
                  println!("recieved: {:?}", event);
               }
            }
         }
      }

      if new_event.is_some() {
         last_event = new_event;
      }
      // end of event handler

      // start frame timing calculations
      {
         let t = time_start.elapsed();
         let frame_time = t.as_secs() * 1_000_000_000 + t.subsec_nanos() as u64;
         let diff = fps_as_ns - frame_time as f64;
         if diff > 0.0 {
            std::thread::sleep(time::Duration::new(0, diff as u32));
         }
      }

   }
}
