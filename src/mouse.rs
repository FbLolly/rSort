use raylib::prelude::*;

pub struct Mouse{
    pub pos : Vector2,
    pub click : bool
}

impl Mouse{
    pub fn new() -> Mouse{
        Mouse {
            pos : Vector2::new(0.0, 0.0),
            click : false
        }
    }

    pub fn update(&mut self, rl : &mut RaylibHandle){
        self.pos = rl.get_mouse_position();
        self.click = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT); // 0 => mouse_button_left
    }
}
