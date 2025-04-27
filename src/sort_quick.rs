use raylib::prelude::*;
use crate::globals::{self, Globals};
use crate::algorithm::Algorithm;

pub struct QuickSort {
    alg : Algorithm
}    

impl QuickSort{
    pub fn new(lenght : usize) -> QuickSort{
        QuickSort {
            alg : Algorithm::new(lenght)
        }
    }

    pub fn start(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        self.alg.shuffle(globals, thread, rl);      
        
        self.quick_sort(globals, thread, rl, 0, self.alg.len-1);
    }

    fn quick_sort(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle,
                  low : usize, high : usize){
        if low >= high {return;}

        let pivot_idx = self.partition(low, high, globals, thread, rl) as i32;

        self.quick_sort(globals, thread, rl, low, (pivot_idx-1) as usize);
        self.quick_sort(globals, thread, rl, (pivot_idx+1) as usize, high);

        globals.update(rl);
        self.alg.algorithm_graphics(globals, thread, rl);
    }

    fn partition(&mut self, low : usize, high : usize,
                 globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle) -> usize{
        let pivot = self.alg.nums[high];
        let mut i : i32 = low as i32 - 1;

        for ii in low..=high{
            if self.alg.nums[ii] >= pivot {continue;}
            
            i += 1;
            self.alg.nums.swap(i as usize, ii);

            globals.update(rl);
            self.alg.algorithm_graphics(globals, thread, rl);
        }

        self.alg.nums.swap((i + 1) as usize, high);
        i as usize+1
    }
}
