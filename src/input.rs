use piston_window::*;

pub struct InputState {
    pub going_up: bool,
    pub going_down: bool,
    pub going_left: bool,
    pub going_right: bool,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub scroll_dir: f64,
    pub win_size: (u32, u32)
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            going_up: false,
            going_down: false,
            going_left: false,
            going_right: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            scroll_dir: 0.0,
            win_size: (0, 0)
        }
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        (self.mouse_x, self.mouse_y)
    }

}

pub fn handle_input(ipt: Input, is: &mut InputState) {
    match ipt {
        Input::Move(motion) => {
            match motion {
                Motion::MouseCursor(x, y)   => {
                    is.mouse_x = x;
                    is.mouse_y = y;
                },
                Motion::MouseRelative(x, y) => {},
                Motion::MouseScroll(x, y)   => {
                    is.scroll_dir = y;
                },
                Motion::ControllerAxis(c)   => {}
            }
            // println!("{:?}", m);
        },
        Input::Press(press) => {
            match press {
                Button::Mouse(mouse) => {
                    //println!("{:?}", mouse);
                },
                Button::Keyboard(key) => {
                    match key {
                        Key::Up    | Key::W => { is.going_up    = true; },
                        Key::Down  | Key::S => { is.going_down  = true; },
                        Key::Left  | Key::A => { is.going_left  = true; },
                        Key::Right | Key::D => { is.going_right = true; },
                        _ => {}
                    }
                },
                Button::Controller(controller) => {}
            }
        },
        Input::Release(rel) => {
            match rel {
                Button::Mouse(mouse) => {
                    // println!("{:?}", player.world.loaded_chunks);
                },
                Button::Keyboard(key) => {
                    match key {
                        Key::Up    | Key::W => { is.going_up    = false; },
                        Key::Down  | Key::S => { is.going_down  = false; },
                        Key::Left  | Key::A => { is.going_left  = false; },
                        Key::Right | Key::D => { is.going_right = false; },
                        _ => {}
                    }
                },
                Button::Controller(controller) => {}
            }
        },
        Input::Text(t) => {},
        Input::Resize(x, y) => { is.win_size = (x, y) },
        Input::Focus(f) => {},
        Input::Cursor(c) => {},
    }
}
