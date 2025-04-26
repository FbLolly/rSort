use raylib::prelude::*;
use crate::globals::Globals;
use crate::algorithm::Algorithm;

pub struct InsertionSort {
    alg : Algorithm
}    

impl InsertionSort{
    pub fn new(lenght : usize) -> InsertionSort{
        InsertionSort {
            alg : Algorithm::new(lenght)
        }
    }

    pub fn start(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        self.alg.shuffle(globals, thread, rl);      
        
        for i in 0..self.alg.len-1{
            if rl.window_should_close(){return;}

            let key = self.alg.nums[i];
            let mut ii : i32 = i as i32 -1;
            
            while ii >= 0 && key < self.alg.nums[ii as usize]{
                if ii as usize + 1 < self.alg.len {
                    self.alg.nums[ii as usize + 1] = self.alg.nums[ii as usize];
                }
                ii -= 1;
            }
            self.alg.nums[(ii + 1) as usize] = key;

            globals.update(rl);
            self.alg.algorithm_graphics(globals, thread, rl);
        }
    }
}
