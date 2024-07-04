use core::*;
use std::env;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Try running: cargo run path/to/game");
        return;
    }

    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("WEISBECKER", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
}
