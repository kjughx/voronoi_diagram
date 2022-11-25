pub mod seed;
use seed::Seeds;
use seed::Seed;
use sdl2::pixels::Color;

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
pub const SEED_COUNT: usize = 10;
pub const SEED_SIZE: u32 = 10;
pub const SEED_STEP: i32 = 20;
pub const COLORS: [Color; 10] = [
    Color::RGB(164, 196, 0), Color::RGB(96, 169, 23),
    Color::RGB(0, 138, 0), Color::RGB(0, 171, 169),
    Color::RGB(27, 161, 226), Color::RGB(0, 80, 239),
    Color::RGB(106, 0, 255), Color::RGB(170, 0, 255),
    Color::RGB(244, 114, 208), Color::RGB(216, 0, 115)
];


pub fn get_color(x: i32, y: i32, seeds: &Seeds) -> Color {
    let mut distance = (WIDTH ^ 2 + HEIGHT ^ 2) as i32;
    let seeds: &Vec<Seed> = seeds.into();
    let mut color = seeds.get(0).unwrap().color;
    for seed in seeds {
        let tmp = seed.distance((x, y));
        if tmp < distance {
            color = seed.color;
            distance = tmp;
        }
    }
    color
}

