use sdl2::{event::Event, keyboard::Keycode};
use sdl2::pixels::PixelFormatEnum;

use voronoi_diagram::{
    WIDTH,
    HEIGHT,
    SEED_SIZE,
    get_color,
    seed::Seeds,
};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Voronoi", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .map_err(|e| e.to_string())?;

    let mut seeds = Seeds::new();
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..(HEIGHT as usize) {
            for x in 0..(WIDTH as usize) {
                let color = get_color(x as i32, y as i32, &seeds);
                let offset = y * pitch + x * 3;
                buffer[offset] = color.r;
                buffer[offset + 1] = color.g;
                buffer[offset + 2] = color.b;
            }
        }
    })?;

    canvas.copy(&texture, None, None)?;
    for seed in seeds.iter() {
        let mut seed_texture = texture_creator
            .create_texture_static(PixelFormatEnum::RGB24, SEED_SIZE, SEED_SIZE)
            .map_err(|e| e.to_string())?;

        seed_texture.set_color_mod(0, 0, 0);

        canvas.copy(&seed_texture, None, Some(seed.into()))?;
    }
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        /* Update seed positions */
        for seed in seeds.iter_mut() { seed.update(); }

        /* Redraw tiles */
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..(HEIGHT as usize) {
                for x in 0..(WIDTH as usize) {
                    let color = get_color(x as i32, y as i32, &seeds);
                    let offset = y * pitch + x * 3;
                    buffer[offset] = color.r;
                    buffer[offset + 1] = color.g;
                    buffer[offset + 2] = color.b;
                }
            }
        })?;
        canvas.copy(&texture, None, None)?; 

        /* Redraw seeds */
        for seed in seeds.iter() {
            let mut seed_texture = texture_creator
                .create_texture_static(PixelFormatEnum::RGB24, SEED_SIZE, SEED_SIZE)
                .map_err(|e| e.to_string())?;

            seed_texture.set_color_mod(0, 0, 0);

            let seed = &seed.clone();
            canvas.copy(&seed_texture, None, Some(seed.into()))?;
        }
        canvas.present();
    }

    Ok(())
}
