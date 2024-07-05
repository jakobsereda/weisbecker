use core::*;

use std::env;
use std::fs::File;
use std::io::Read;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

    let mut cpu = CPU::new();

    let mut rom = File::open(&args[1]).expect("Cannot access file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    cpu.load(&buffer);

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

        cpu.tick();
        draw_screen(&cpu, &mut canvas);
    }
}

fn draw_screen(cpu: &CPU, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = cpu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i % DISPLAY_HEIGHT) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
