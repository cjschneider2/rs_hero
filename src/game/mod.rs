use std::f32::consts::PI;

pub mod buffer;
pub mod input;
pub mod render;

pub struct State {
    pub tone_hz: usize,
    pub x_offset: usize,
    pub y_offset: usize,

    pub t_sine: f32,

    pub player_x: usize,
    pub player_y: usize,
    pub t_jump: f32,
}

pub struct Game {
    pub state: State,
    pub render_buffer: buffer::Image,
    pub input: input::Input,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let bpp = 4; // 4 bytes per pixel
        let size = width * height * bpp;
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
            render_buffer: buffer::Image {
                memory: memory,
                width: width,
                height: height,
                pitch: width * bpp,
                bytes_per_pixel: bpp,
            },
            input: input::Input::new(),
        }
    }

    pub fn resize_buffer(&mut self, width: u32, height: u32) {
        let bpp = self.render_buffer.bytes_per_pixel;
        let size = width * height * bpp;
        self.render_buffer.pitch = width * bpp;
        self.render_buffer.width = width;
        self.render_buffer.height = height;
        self.render_buffer.memory.clear();
        for _ in 0..size { self.render_buffer.memory.push(0); }
    }

    pub fn update_and_render(&mut self) {
        // Update
        if self.input.controllers[0].action_up.pressed {
            self.state.y_offset += 1;
        }
        if self.input.controllers[0].action_down.pressed {
            self.state.y_offset -= 1;
        }
        if self.input.controllers[0].action_right.pressed {
            self.state.x_offset -= 1;
        }
        if self.input.controllers[0].action_left.pressed {
            self.state.x_offset += 1;
        }

        // Render
        let _ = render::weird_gradient(&mut self.render_buffer,
                                       self.state.x_offset as u32,
                                       self.state.y_offset as u32);
        let _ = render::player(&mut self.render_buffer, 50, 50);
    }

    pub fn output_sound(
        &mut self,
        sound_buffer: &mut buffer::SoundOutput,
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

    pub fn get_mut_keyboard(&mut self) -> &mut input::ControllerInput {
        &mut self.input.controllers[0]
    }
}
