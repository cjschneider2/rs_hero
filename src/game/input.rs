pub struct Input {
    pub mouse_buttons: [ButtonState; 5],
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_z: i32,
    pub controllers: [ControllerInput; 5],
}

impl Input {
    pub fn new() -> Input {
        Input {
            mouse_buttons: [ButtonState::new(); 5],
            mouse_x: 0,
            mouse_y: 0,
            mouse_z: 0,
            controllers: [ControllerInput::new(); 5],
        }
    }
}

#[derive(Clone, Copy)]
pub struct ButtonState {
    pub half_transition_count: usize,
    pub pressed: bool,
}

impl ButtonState {
    pub fn new() -> ButtonState {
        ButtonState {
            half_transition_count: 0,
            pressed: false,
        }
    }

    pub fn key_press(&mut self, is_down: bool) {
        assert!(self.pressed != is_down);
        self.pressed = is_down;
        self.half_transition_count += 1;
    }
}

#[derive(Clone, Copy)]
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

impl ControllerInput {
    pub fn new() -> ControllerInput {
        ControllerInput {
            is_connected: false,
            is_analog: false,
            stick_avg_x: 0.0,
            stick_avg_y: 0.0,
            move_up: ButtonState::new(),
            move_down: ButtonState::new(),
            move_left: ButtonState::new(),
            move_right: ButtonState::new(),
            action_up: ButtonState::new(),
            action_down: ButtonState::new(),
            action_left: ButtonState::new(),
            action_right: ButtonState::new(),
            left_shoulder: ButtonState::new(),
            right_shoulder: ButtonState::new(),
            btn_back: ButtonState::new(),
            btn_start: ButtonState::new(),
        }
    }
}



