use crate::{button::Button, globals::Globals, sort_bubble::BubbleSort, sort_insertion::InsertionSort, sort_quick::QuickSort, sort_selection::SelectionSort};
use raylib::{prelude::{Color, RaylibDraw, RaylibHandle}, RaylibThread};

pub struct Menu{
    pub btns : Vec<Button>,
}

impl Menu{
    pub fn new(globals : &Globals, rl : &mut RaylibHandle) -> Menu{
        Menu{
            btns : vec![
                Button::new(Color::PURPLE, Color::RAYWHITE, 100.0, 100.0, "bubble_sort", globals, rl),
                Button::new(Color::PURPLE, Color::RAYWHITE, 100.0, 150.0, "selection_sort", globals, rl),
                Button::new(Color::PURPLE, Color::RAYWHITE, 100.0, 200.0, "insertion_sort", globals, rl),
                Button::new(Color::PURPLE, Color::RAYWHITE, 100.0, 250.0, "quick_sort", globals, rl),
                Button::new(Color::PURPLE, Color::RAYWHITE, 100.0, 300.0, "_QUIT_", globals, rl),
            ],
        }
    }

    fn handle_buttons_update(&mut self, actives : Vec<bool>,
                             globals : &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){
        if actives[0] {BubbleSort::new(globals.arr_lenght).start(globals, thread, rl);}
        if actives[1] {SelectionSort::new(globals.arr_lenght).start(globals, thread, rl);}
        if actives[2] {InsertionSort::new(globals.arr_lenght).start(globals, thread, rl);}
        if actives[3] {QuickSort::new(globals.arr_lenght).start(globals, thread, rl);}
        if actives[4] {globals.acted_to_close = true;}
    }

    pub fn start(&mut self, globals: &mut Globals, thread : &RaylibThread, rl : &mut RaylibHandle){

        while !(rl.window_should_close() || globals.acted_to_close){
            let mut stop = false;
            let mut actives : Vec<bool> = vec![];
            while (!rl.window_should_close()) && !stop{
                globals.update(rl);

                actives = vec![];
                for i in self.btns.iter_mut(){
                    let ret = i.update(globals);
                    actives.push(ret);

                    if ret {stop = true;}
                }

                if globals.fps != globals.fps_og {
                    rl.set_target_fps(globals.fps_og);
                    globals.fps = globals.fps_og;
                }

                let mut d = rl.begin_drawing(thread);

                d.clear_background(Color::BLACK);
                for i in self.btns.iter_mut(){
                    i.draw(globals, &mut d);
                }
            }

            self.handle_buttons_update(actives, globals, thread, rl);
        }
    }
}
