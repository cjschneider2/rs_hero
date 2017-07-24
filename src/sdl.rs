use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

use error::Error;

pub struct Sdl {
    context: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    controller: sdl2::GameControllerSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    audio: sdl2::AudioSubsystem,
    last_event: Option<sdl2::event::Event>,
}

impl Sdl {
    pub fn new() -> Result<Sdl, Error> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("rs_hero", 400, 400)
                          .position_centered()
                          .opengl()
                          .build()?;
        let canvas = window.into_canvas().present_vsync().build()?;
        let controller = context.game_controller()?;
        let event_pump = context.event_pump()?;
        let audio = context.audio()?;
        let sdl = Sdl {
            context: context,
            video: video,
            controller: controller,
            canvas: canvas,
            event_pump: event_pump,
            audio: audio,
            last_event: None,
        };
        Ok(sdl)
    }

    pub fn open_game_controllers(&mut self) {
    }

    pub fn audio_init(
        &mut self,
        samples_per_second: usize,
        buffer_size: usize
    ) {
        let mut audio_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),
            samples: None
        };
    }

    pub fn handle_events(&mut self) -> bool {
        let mut should_quit = false;
        let new_event = self.event_pump.poll_event();
        if new_event != self.last_event {
            if let Some(ref event) = new_event {
                match event {
                    &Event::Quit { .. }
                    | &Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                    => {
                        should_quit = true;
                    },
                    &Event::KeyDown { keycode: Some(key), .. } => {
                        match key {
                            Keycode::A => (),
                            Keycode::S => (),
                            Keycode::D => (),
                            Keycode::F => (),
                            Keycode::Q => (),
                            Keycode::E => (),
                            Keycode::Up => (),
                            Keycode::Down => (),
                            Keycode::Right => (),
                            Keycode::Left => (),
                            Keycode::Space => (),
                            _ => (),
                        }
                    },
                    &Event::Window { win_event: w_event, ..} => {
                        match w_event {
                            WindowEvent::Enter => (),
                            WindowEvent::Leave => (),
                            WindowEvent::SizeChanged(new_x, new_y) => {
                                println!("Window size changed to: ({},{})",
                                         new_x, new_y);
                            },
                            _ => (),
                        }
                    },
                    _ => {
                        println!("recieved: {:?}", event);
                    }
                }

                if should_quit { return true; };
            }
        }
        if new_event.is_some() {
            self.last_event = new_event;
        }
        should_quit
    }

}

//  EXTRA STRUCTS -------------------------------------------------------------

pub struct OffscreenBuffer<'a> {
    texture: sdl2::render::Texture<'a>,
    memory: (), // NOTE(CJS): in the SDL c program it's a void *.... 
    width: usize,
    height: usize,
    pitch: usize,
}

pub struct AudioRingBuffer {
    pub data: Box<[u8]>,
    pub write_cursor: usize,
    pub play_cursor: usize,
}

pub struct SoundOutpu {
    pub samples_per_second: usize,
    pub running_sample_index: usize,
    pub bytes_per_sample: usize,
    pub secondary_buffer_size: usize,
    pub latency_sample_count: usize,
}

pub struct WindowDimension {
    pub width: usize,
    pub height: usize,
}

//  EXTRA FUNCTIONS -----------------------------------------------------------

// TODO(CJS): Find a much more "rusty" way to do this without having to dip into
//            unsafe buffers...
pub fn audio_callback(
    ring_buffer: &mut AudioRingBuffer,
    data: &[u8],
    length: usize
) {
    let ring_buffer_size = ring_buffer.data.len();

    if ring_buffer.play_cursor + length > ring_buffer_size {
        // NOTE(CJS): this handles the wrap-around case...
        let len_1 = ring_buffer_size - ring_buffer.play_cursor;
        let len_2 = length - len_1;
        unsafe {
            // copy to end of buffer
            let ring_ptr = ring_buffer.data.as_mut_ptr();
            let src_ptr = data.as_ptr();
            let dst_ptr = ring_ptr.offset(ring_buffer.play_cursor as isize);
            let len = len_1;
            ::std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);

            // copy the rest from the beginning of the buffer
            let src_ptr = data.as_ptr();
            let dst_ptr = ring_buffer.data.as_mut_ptr();
            let len = len_2;
            ::std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        }
    } else {
        let len = data.len();
        unsafe {
            let ring_ptr = ring_buffer.data.as_mut_ptr();
            let src_ptr = data.as_ptr();
            let dst_ptr = ring_ptr.offset(ring_buffer.play_cursor as isize);
            ::std::ptr::copy_nonoverlapping (src_ptr, dst_ptr, len);
        }
    }

}