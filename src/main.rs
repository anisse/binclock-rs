extern crate sdl2;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::Sdl;

use chrono::{Local, Timelike};

const LED_SIZE: u32 = 30;

struct Resources<'a> {
    sdl_context: Sdl,
    canvas: &'a mut WindowCanvas,
    ledon: Texture<'a>,
    ledoff: Texture<'a>,
    ts: i64,
}

impl<'a> Resources<'a> {
    pub fn new(
        sdl_context: Sdl,
        canvas: &'a mut WindowCanvas,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        let path1 = Path::new(&"ledon.bmp");
        let surface1 = Surface::load_bmp(path1)?;
        let ledon = texture_creator
            .create_texture_from_surface(surface1)
            .unwrap();
        let path2 = Path::new(&"ledoff.bmp");
        let surface2 = Surface::load_bmp(path2)?;
        let ledoff = texture_creator
            .create_texture_from_surface(surface2)
            .unwrap();

        Ok(Resources {
            sdl_context: sdl_context,
            canvas: canvas,
            ledon: ledon,
            ledoff: ledoff,
            ts: 0,
        })
    }
}

pub fn main() -> Result<(), String> {
    run()?;

    Ok(())
}
fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 6 * LED_SIZE, 4 * LED_SIZE)
        .position_centered()
        .borderless()
        //.opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let mut state = Resources::new(sdl_context, &mut canvas, &texture_creator)?;

    let mut event_pump = state.sdl_context.event_pump()?;

    render(&mut state, true)?;

    let mut should_render = false;

    'running: loop {
        for event in event_pump.wait_timeout_iter(1000) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Exposed,
                    ..
                } => {
                    //force refresh when window is exposed
                    should_render = true;
                }
                _ => {} //ignore most events
            }
            render(&mut state, should_render)?;
            should_render = false;
        }
        render(&mut state, should_render)?;
        print!("done...\n");
    }

    Ok(())
}

fn render(state: &mut Resources, force: bool) -> Result<(), String> {
    let now = Local::now();

    if !force && state.ts == now.timestamp() {
        //no need to refresh
        return Ok(());
    }

    state.ts = now.timestamp();

    puttimecomponent(state, now.hour(), 0, 2, 4)?;
    puttimecomponent(state, now.minute(), 2, 3, 4)?;
    puttimecomponent(state, now.second(), 4, 3, 4)?;

    state.canvas.present();
    Ok(())
}

fn puttimecomponent(
    state: &mut Resources,
    value: u32,
    position: u8,
    bits0: u8,
    bits1: u8,
) -> Result<(), String> {
    let component_low = value % 10;
    putdigit(state, (value - component_low) as u8 / 10, bits0, position)?;
    putdigit(state, component_low as u8, bits1, position + 1)?;
    Ok(())
}
fn putdigit(state: &mut Resources, digit: u8, bits: u8, position: u8) -> Result<(), String> {
    for bit in 0..bits {
        putled(
            state,
            digit & (1 << bit) != 0,
            position as i32,
            3 - bit as i32,
        )?;
    }
    Ok(())
}
fn putled(state: &mut Resources, on: bool, x: i32, y: i32) -> Result<(), String> {
    let led = match on {
        true => &state.ledon,
        false => &state.ledoff,
    };
    state.canvas.copy(
        led,
        None,
        Rect::new(LED_SIZE as i32 * x, LED_SIZE as i32 * y, LED_SIZE, LED_SIZE),
    )?;
    Ok(())
}
