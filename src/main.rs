use menu::Menu;

mod globals;
mod algorithm;
mod menu;
mod button;
mod mouse;
mod sort_bubble;
mod sort_selection;
mod sort_insertion;

fn main() {
    let mut globals = globals::Globals::new();
    let (mut rl, mut thread) = raylib::init()
        .size(globals.width, globals.height)
        .title("stuff")
        .build();

    globals.load_font(&mut rl, &mut thread, "fonts/CaskaydiaCove-Bold.ttf", 16);

    rl.set_target_fps(globals.fps);
    let mut menu = Menu::new(&globals, &mut rl);
    menu.start(&mut globals, &thread, &mut rl);
}
