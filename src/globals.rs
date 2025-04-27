use raylib::{RaylibHandle, RaylibThread};
use raylib::prelude::Font;

use crate::mouse::Mouse;

pub struct Globals{
    pub fps : u32,
    pub fps_og : u32,
    pub fps_update : bool,
    pub fps_change : u32,
    pub width : i32,
    pub height : i32,
    pub single_size : i32,
    pub arr_lenght : usize,
    pub font : Option<Font>,
    pub font_size : i32,
    pub mouse : Mouse,
    pub button_animation_speed : f32,
}

impl Globals {
    pub fn new() -> Globals{
        Self {
            fps : 60,
            fps_og : 60,
            fps_update : true,
            fps_change : 10,
            width : 600,
            height : 400,
            single_size : 1,
            arr_lenght : 600,
            font : None,
            font_size : 0,
            mouse : Mouse::new(),
            button_animation_speed : 2.5,
        }
    }

    pub fn load_font(&mut self, rl : &mut RaylibHandle, thread : &mut RaylibThread, name : &str, size : i32){
        if self.font.is_some(){return;} 

        let f = rl.load_font_ex(thread, name, size, None);
        let _ = self.font.insert(f.unwrap());
        self.font_size = size;
    }

    pub fn update(&mut self, rl : &mut RaylibHandle){
        self.mouse.update(rl);
    }
}
