use std::io::Cursor;
use std::f32::consts::PI;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian as LE;

pub struct Game {
    state: State,
}

impl Game {
    pub fn new() -> Game {
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
        }
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
    pub memory: Box<[u8]>,
    pub height: usize,
    pub width: usize,
    pub pitch: usize,
    pub bytes_per_pixel: usize,
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
    x_offset: usize,
    y_offset: usize
)
{
    let mut cursor = Cursor::new(&mut *buffer.memory);
    for p_y in 0..buffer.height {
        for p_x in 0..buffer.width {
            let blue:u32  = 255;
            let red:u32   = (p_x + x_offset) as u32;
            let green:u32 = (p_y + y_offset) as u32;
            let pixel:u32 = (red << 16) | (green << 8) | blue;
            cursor.write_u32::<LE>(pixel).unwrap();
        }
    }
}

fn render_player(
    buffer: &mut OffscreenBuffer,
    player_x: usize,
    player_y: usize
) {
    let top = player_y;
    let bottom = player_y + 10;
    // TODO(CJS): We could probably assert! and use get_unchecked eventaully...
    // let end = buffer.memory.len();
    for x in player_x..player_x+10 {
        let x_offset = x * buffer.bytes_per_pixel;
        let mut y_offset = top * buffer.pitch;
        let mut pixel;
        for _ in top..bottom {
            pixel = x_offset + y_offset;
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
