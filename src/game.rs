use std::io::Cursor;
use std::f32::consts::PI;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian as LE;

use error::Error;

pub struct Game {
    pub state: State,
    pub render_buffer: OffscreenBuffer,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let bbp = 4; // 4 bytes per pixel
        let size = width * height * bbp;
        let mut memory = Vec::with_capacity(size as usize);
        // TODO(CJS): This is just to set the size of the vec because it starts
        //            out with a size of 0... maybe find a better way?
        for _ in 0..size { memory.push(0); }
        Game {
            state: State {
                tone_hz: 440,
                x_offset: 0,
                y_offset: 0,
                t_sine: 0.0,
                player_x: 50,
                player_y: 50,
                t_jump: 0.0,
            },
            render_buffer: OffscreenBuffer {
                memory: memory,
                width: width,
                height: height,
                pitch: width * bbp,
                bytes_per_pixel: bbp,
            },
        }
    }

    pub fn update_and_render(&mut self) {
        // Update
        self.state.x_offset += 1;
        self.state.y_offset += 1;
        // Render
        let _ = render_weird_gradient(&mut self.render_buffer,
                                      self.state.x_offset as u32,
                                      self.state.y_offset as u32);
        let _ = render_player(&mut self.render_buffer, 50, 50);
    }

    pub fn output_sound(
        &mut self,
        sound_buffer: &mut SoundOutputBuffer,
        tone_hz: usize)
    {
        let tone_volume = 500.0;
        let wave_period = (sound_buffer.samples_per_second / tone_hz) as f32;
        for sample in sound_buffer.samples.iter_mut() {
            let sine_value = self.state.t_sine.sin();
            let sample_value = (sine_value * tone_volume) as i16;
            *sample = sample_value;
            self.state.t_sine += 2.0 * PI / wave_period;
            if self.state.t_sine > 2.0 * PI {
                self.state.t_sine -= 2.0 * PI;
            }
        }
    }
}

pub struct State {
    pub tone_hz: usize,
    pub x_offset: usize,
    pub y_offset: usize,

    pub t_sine: f32,

    pub player_x: usize,
    pub player_y: usize,
    pub t_jump: f32,
}

pub struct OffscreenBuffer {
    pub memory: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bytes_per_pixel: u32,
}

pub struct SoundOutputBuffer {
    pub samples: Box<[i16]>,
    pub samples_per_second: usize,
    pub sample_count: usize,
}

pub struct ButtonState {
    pub half_transition_count: usize,
    pub ended_down: bool,
}

pub struct ControllerInput {
    pub is_connected: bool,
    pub is_analog: bool,
    pub stick_avg_x: f32,
    pub stick_avg_y: f32,
    pub move_up: ButtonState,
    pub move_down: ButtonState,
    pub move_left: ButtonState,
    pub move_right: ButtonState,
    pub action_up: ButtonState,
    pub action_down: ButtonState,
    pub action_left: ButtonState,
    pub action_right: ButtonState,
    pub left_shoulder: ButtonState,
    pub right_shoulder: ButtonState,
    pub btn_back: ButtonState,
    pub btn_start: ButtonState,
}

pub struct Input {
    mouse_buttons: [ButtonState; 5],
    mouse_x: i32,
    mouse_y: i32,
    mouse_z: i32,
    controllers: [ControllerInput; 5],
}

fn render_weird_gradient(
    buffer: &mut OffscreenBuffer,
    x_offset: u32,
    y_offset: u32
) -> Result<(), Error> {
    let mut cursor = Cursor::new(&mut *buffer.memory);
    for p_y in 0..buffer.height {
        for p_x in 0..buffer.width {
            let blue  = 0xFF;
            let red   = ((p_x + x_offset) % 0xFF) as u8;
            let green = ((p_y + y_offset) % 0xFF) as u8;
            let alpha = 0xFF;
            cursor.write_u8(red)?;
            cursor.write_u8(green)?;
            cursor.write_u8(blue)?;
            cursor.write_u8(alpha)?;
        }
    }
    Ok(())
}

fn render_player(
    buffer: &mut OffscreenBuffer,
    player_x: u32,
    player_y: u32 )
{
    let top = player_y;
    let bottom = player_y + 10;
    // TODO(CJS): We could probably assert! and use get_unchecked eventaully...
    // let end = buffer.memory.len();
    for x in player_x..player_x+10 {
        let x_offset = x * buffer.bytes_per_pixel;
        let mut y_offset = top * buffer.pitch;
        let mut pixel;
        for _ in top..bottom {
            pixel = (x_offset + y_offset) as usize;
            // HACK(CJS): until I get byteorder to write here efficently i'll just
            //            hack in the for bytes manually in LE order...
            // let color = 0xFFFFFFFF; // white
            buffer.memory[pixel] = 0xFF;
            buffer.memory[pixel + 1] = 0xFF;
            buffer.memory[pixel + 2] = 0xFF;
            buffer.memory[pixel + 3] = 0xFF;
            y_offset += buffer.pitch;
        }
    }
}
