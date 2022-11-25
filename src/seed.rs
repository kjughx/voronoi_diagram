use std::slice::{Iter, IterMut};

use rand::Rng;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::{
    COLORS,
    WIDTH,
    HEIGHT,
    SEED_COUNT,
    SEED_SIZE,
    SEED_STEP,
};

pub struct Seeds(Vec<Seed>);

#[derive(Clone, Copy, Debug)]
pub struct Seed {
    pub pos: (i32, i32),
    pub color: Color,
}

impl Seed {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self {
            pos: (x, y),
            color,
        }
    }
    pub fn update(&mut self) {
        let (x, y) = &mut self.pos;
        *x = ((*x + rand::thread_rng().gen_range(-SEED_STEP..=SEED_STEP)) % WIDTH as i32).abs();
        *y = ((*y + rand::thread_rng().gen_range(-SEED_STEP..=SEED_STEP)) % HEIGHT as i32).abs();

        /* Find closest other seed and move away from it */
    }

    pub fn distance(&self, pos: (i32, i32)) -> i32 {
        return  (self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs()
    }
}

impl Into<Rect> for &Seed {
    fn into(self) -> Rect {
        let (x, y) = self.pos;
        let x = (x - (SEED_SIZE/2) as i32) % WIDTH as i32;
        let y = (y - (SEED_SIZE/2) as i32) % HEIGHT as i32;
        Rect::new(x.abs(), y.abs(), SEED_SIZE, SEED_SIZE)
    }
}

impl Seeds {
    pub fn new() -> Self {
    Self((0..SEED_COUNT).map(|idx|{
        let (x, y) = (rand::thread_rng().gen_range(0..WIDTH), rand::thread_rng().gen_range(0..HEIGHT));
            Seed::new(x as i32, y as i32, COLORS[idx])
        }).collect())
    }

    pub fn iter(&self) -> Iter<'_, Seed> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Seed> {
        self.0.iter_mut()
    }
}

impl<'a> Into<&'a Vec<Seed>> for &'a Seeds {
    fn into(self) -> &'a Vec<Seed> {
        &self.0
    }
}

impl<'a> Into<&'a Vec<Seed>> for &mut &'a Seeds {
    fn into(self) -> &'a Vec<Seed> {
        &self.0
    }
}

