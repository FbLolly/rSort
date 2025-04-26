use raylib::prelude::*;
use crate::globals::Globals;
use crate::algorithm::Algorithm;

pub struct SelectionSort {
    alg : Algorithm
}    

impl SelectionSort{
    pub fn new(lenght : usize) -> SelectionSort{
        SelectionSort {
            alg : Algorithm::new(lenght)
        }
    }

    pub fn start(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        let mut done = false;

        self.alg.shuffle(globals, thread, rl);      
        
        for i in 0..self.alg.len-1{
            let mut min_idx = Clone::clone(&i); 

            for ii in i..self.alg.len{
                if self.alg.nums[ii] < self.alg.nums[min_idx]{min_idx = ii;}
            }

            self.alg.nums.swap(min_idx, i);

            globals.update(rl);
            self.alg.algorithm_graphics(globals, thread, rl);
        }
    }
}
