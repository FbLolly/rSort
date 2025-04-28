use crate::{algorithm::Algorithm, globals::Globals};
use raylib::prelude::*;

pub struct BubbleSort{
    pub alg : Algorithm
}

impl BubbleSort{
    pub fn new(lenght : usize) -> BubbleSort{
        BubbleSort {
            alg : Algorithm::new(lenght)
        }
    }

    pub fn start(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        let mut done = false;

        self.alg.shuffle(globals, thread, rl);

        while !done{
            //update
            done = true;

            for ii in 0..self.alg.len-1{
                if self.alg.window_should_close(rl, globals){return;}

                if self.alg.nums[ii] > self.alg.nums[ii+1]{
                    done = false;
                    self.alg.nums.swap(ii, ii+1);
                }
            }

            self.alg.algorithm_graphics(globals, thread, rl);
        }
    }
}
