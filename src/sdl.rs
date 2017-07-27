use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;

use error::Error;

pub struct Sdl {
    pub context: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub controller: sdl2::GameControllerSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub audio: sdl2::AudioSubsystem,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub last_event: Option<sdl2::event::Event>,
    pub audio_spec: sdl2::audio::AudioSpecDesired,
}

impl Sdl {
    pub fn new(width: u32, height: u32) -> Result<Sdl, Error> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("rs_hero", width, height)
                          .position_centered()
                          .resizable()
                          .opengl()
                          .build()?;
        let canvas = window.into_canvas().present_vsync().build()?;
        let controller = context.game_controller()?;
        let event_pump = context.event_pump()?;
        let audio = context.audio()?;
        let texture_creator = canvas.texture_creator();
        let audio_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),
            samples: Some(4),
        };
        let sdl = Sdl {
            context: context,
            video: video,
            controller: controller,
            canvas: canvas,
            event_pump: event_pump,
            audio: audio,
            texture_creator: texture_creator,
            last_event: None,
            audio_spec: audio_spec,
        };
        Ok(sdl)
    }

    pub fn draw_buffer (
        &mut self,
        buffer: &[u8],
        // texture: &mut sdl2::render::Texture<'a>,
        width: u32,
        height: u32,
        pitch: usize
    ) -> Result<(), Error> {
        self.canvas.clear();
        // TODO(CJS): Figure out how not to re-create this texture every time
        let mut texture = self.texture_creator.create_texture_target(
            // self.texture_creator.default_pixel_format(),
            // NOTE: This format is because I'm lazy and want to write out RGBA
            //       in that order on Little Endian machines...
            // FIX: if wanting to play on ARM... or other BE processors
            sdl2::pixels::PixelFormatEnum::ABGR8888,
            width, height)?;
        texture.update(None, buffer, pitch)?;
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();
        Ok(())
    }

    pub fn open_game_controllers(&mut self) {
    }

    pub fn handle_events(&mut self, game: &mut ::game::Game) -> bool {
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
                    &Event::KeyDown { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = true;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    },
                    &Event::KeyUp { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = false;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    },
                    &Event::Window { win_event: w_event, ..} => {
                        match w_event {
                            WindowEvent::Enter => (),
                            WindowEvent::Leave => (),
                            WindowEvent::SizeChanged(x, y) => {
                                game.resize_buffer(x as u32, y as u32);
                                println!("Window size change: ({},{})", x, y);
                            },
                            _ => (),
                        }
                    },
                    _ => (),
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

fn process_keycode(
    key: sdl2::keyboard::Keycode,
    is_down: bool,
    input: &mut ::game::input::ControllerInput
) {
    match key {
        Keycode::W => input.move_up.key_press(is_down),
        Keycode::A => input.move_left.key_press(is_down),
        Keycode::S => input.move_down.key_press(is_down),
        Keycode::D => input.move_right.key_press(is_down),
        Keycode::Q => input.left_shoulder.key_press(is_down),
        Keycode::E => input.right_shoulder.key_press(is_down),
        Keycode::Up => input.action_up.key_press(is_down),
        Keycode::Down => input.action_down.key_press(is_down),
        Keycode::Right => input.action_right.key_press(is_down),
        Keycode::Left => input.action_left.key_press(is_down),
        Keycode::F => (),
        Keycode::Space => (),
        _ => (),
    }
}

//  EXTRA STRUCTS -------------------------------------------------------------

pub struct AudioRingBuffer {
    pub data: Box<[u8]>,
    pub write_cursor: usize,
    pub play_cursor: usize,
}

pub struct SoundOutput {
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

fn gen_wave(bytes_to_write: usize) -> Vec<i16> {
    let tone_volume = 100i16;
    let period = 48000 / 256;
    let sample_count = bytes_to_write;
    let mut samples = Vec::new();
    for t in 0..sample_count {
        samples.push(
            if ( t / period ) % 2 == 0 {
                tone_volume
            } else {
                -tone_volume
            }
        );
    }
    samples
}

