use raylib::prelude::*;
use rand::prelude::*;

use crate::globals::Globals;

pub struct Algorithm {
    pub nums : Vec<i32>,
    pub len : usize
}

fn generate_array(len : usize) -> Vec<i32>{
    let mut ret = vec![];
    let mut add = 0;

    for i in 0..len{
        ret.push(add);

        if i % 2 == 0{add += 1;}
    }
    ret
}

impl Algorithm {
    pub fn new(lenght : usize) -> Algorithm{
        Self {
            nums : generate_array(lenght),
            len : lenght
        }
    }

    fn shuffle_cycle(&mut self, i : &usize, j : &usize){
        self.nums.swap(*i, *j);
    }

    pub fn window_should_close(&mut self, rl : &mut RaylibHandle, globals : &mut Globals) -> bool{
        if globals.acted_to_close || rl.window_should_close() {return true;}
        false
    }

    fn manage_speeds(&mut self, globals: &mut Globals, rl : &mut RaylibHandle){
        let change = (globals.fps_change/2) + (globals.fps/globals.fps_change);

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT){
            if change >= globals.fps {return;}
            globals.fps -= change;
            globals.fps_update = true;
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT){
            globals.fps += change;
            globals.fps_update = true;
        }
        
        if globals.fps <= 1 {
            globals.fps = 1;
            globals.fps_update = true;
        }
    }

    pub fn algorithm_graphics(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        self.manage_speeds(globals, rl);

        if globals.fps_update {
            globals.fps_update = false;
            rl.set_target_fps(globals.fps);
        }

        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK);
        self.paint_self(&mut d, globals);
    }

    pub fn shuffle(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        let finished = false;
        let mut rng = rand::rng();

        for invi in 0..self.len{
            let i = self.len-invi-1;

            if finished || self.window_should_close(rl, globals){
                break;
            }

            //update
            
            match i {
                0 => self.shuffle_cycle(&i, &0),
                _ => self.shuffle_cycle(&i, &(rng.random_range(0..i))) 
            }

            //draw
            self.algorithm_graphics(globals, thread, rl);
        }
    }

    pub fn paint_self(&mut self, d : &mut RaylibDrawHandle, globals: &Globals){
        let mut x = globals.width/2 - self.len as i32*globals.single_size/2;

        for i in 0..self.len{
            let rect = Rectangle::new(  (x) as f32,
                                        (globals.height - self.nums[i]*globals.single_size) as f32,
                                        (globals.single_size) as f32,
                                        (self.nums[i]*globals.single_size) as f32);
            d.draw_rectangle_rec(rect, Color::WHITE);

            x += globals.single_size;
        }
    }
}
