use core::*;
use std::env;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 20;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Try running: cargo run path/to/game");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("WEISBECKER", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{ .. } | Event::KeyDown { 
                    keycode: Some(Keycode::Escape), 
                    .. 
                } => break 'running,
                _ => ()
            }
        }
    }
}
