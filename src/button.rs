use raylib::prelude::*;
use crate::globals::Globals;

pub struct Button{
    pub col_rect : Rectangle,
    pub gra_rect : Rectangle,
    pub fore_col : Color,
    pub text_col : Color,
    pub txt : String
}

impl Button{
    pub fn new(fore_color : Color, text_color : Color, midx : f32, midy : f32, text : &str,
               globals : &Globals, rl : &mut RaylibHandle) -> Button{

        let rect = Rectangle::new(
            midx - (rl.measure_text(text, globals.font_size) as f32)/2.0 - globals.font_size as f32,
            midy - globals.font_size as f32 / 2.0,
            rl.measure_text(text, globals.font_size) as f32 + globals.font_size as f32,
            globals.font_size as f32 
        );

        let g_rect = Rectangle::new(
            midx - globals.font_size as f32 * 0.5, //math betrayed me
            midy,
            0.0,
            0.0
        );

        Self {
            col_rect : rect,
            gra_rect : g_rect,
            fore_col : fore_color,
            text_col : text_color,
            txt : text.to_string()
        }
    }

    fn check_click(&mut self, globals : &Globals) -> i32{
        let collinding : bool = self.col_rect.check_collision_point_rec(globals.mouse.pos);
        if collinding && globals.mouse.click {return 2;}
        if collinding {return 1;}
        0
    }

    fn expand(&mut self, globals : &Globals){
        if self.gra_rect.width < self.col_rect.width{
            self.gra_rect.width += globals.button_animation_speed * 2.0;            
        }
        if self.gra_rect.x > self.col_rect.x{
            self.gra_rect.x -= globals.button_animation_speed;
        }

        if self.gra_rect.height < self.col_rect.height{
            self.gra_rect.y -= globals.button_animation_speed;
            self.gra_rect.height += globals.button_animation_speed * 2.0;            
        }
    }

    fn compress(&mut self, globals : &Globals){
        if self.gra_rect.width > 0.0{
            self.gra_rect.x += globals.button_animation_speed;
            self.gra_rect.width -= globals.button_animation_speed * 2.0;            
        }
        if self.gra_rect.height > 0.0{
            self.gra_rect.y += globals.button_animation_speed;
            self.gra_rect.height -= globals.button_animation_speed * 2.0;            
        }
    }

    pub fn update(&mut self, globals : &Globals) -> bool{
        match self.check_click(globals){
            1 => self.expand(globals),
            2 => return true,
            _ => self.compress(globals)
        }
        false
    }

    pub fn draw(&mut self, globals : &Globals, d : &mut RaylibDrawHandle){
        d.draw_rectangle_rounded(self.gra_rect, 0.6, 60, self.fore_col);

        let f = (globals.font.as_ref()).unwrap();
        d.draw_text_ex(f,
                       &self.txt, Vector2::new(self.col_rect.x + (globals.font_size as f32), self.col_rect.y),
                       globals.font_size as f32, 0.0, self.text_col);
    }
}
