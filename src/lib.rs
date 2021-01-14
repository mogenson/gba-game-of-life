#![no_std]
#![feature(exclusive_range_pattern)]

use gba::{
    io::display::DISPCNT,
    vram::bitmap::{Mode5, Page},
    Color,
};
use rand::{
    rngs::SmallRng,
    {Rng, SeedableRng},
};

const ALIVE: Color = Color::from_rgb(0, 31, 0);
const DEAD: Color = Color::from_rgb(0, 0, 0);
const WIDTH: i32 = 120;
const HEIGHT: i32 = 80;

pub struct Universe {
    page: Page,
}

impl Universe {
    pub fn new() -> Self {
        Universe { page: Page::Zero }
    }

    pub fn populate(&self, seed: u64) {
        let mut rng = SmallRng::seed_from_u64(seed);
        for _ in 0..(WIDTH * HEIGHT / 8) {
            let x = rng.gen_range(0..WIDTH) as usize;
            let y = rng.gen_range(0..HEIGHT) as usize;
            Mode5::write(self.page, x, y, ALIVE);
        }
    }

    fn next(&self, x: i32, y: i32) -> Color {
        const NEIGHBORS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (1, 0),
        ];

        let neighbors = NEIGHBORS
            .iter()
            .map(|(i, j)| {
                let (col, row) = if let (x @ 0..WIDTH, y @ 0..HEIGHT) = (x + i, y + j) {
                    (x, y)
                } else {
                    ((x + WIDTH) % WIDTH, (y + HEIGHT) % HEIGHT)
                };
                if Mode5::read(self.page, col as usize, row as usize).unwrap() == ALIVE {
                    1
                } else {
                    0
                }
            })
            .sum();

        match (
            Mode5::read(self.page, x as usize, y as usize).unwrap(),
            neighbors,
        ) {
            // rule 1: live cell with less than two live neighbors dies
            (ALIVE, x) if x < 2 => DEAD,
            // rule 2: live cell with 2 or 3 live neighbors lives
            (ALIVE, 2) | (ALIVE, 3) => ALIVE,
            // rule 3: live cell with more than 3 live neighbors dies
            (ALIVE, x) if x > 3 => DEAD,
            // rule 4: dead cell with 3 live neighbors lives
            (DEAD, 3) => ALIVE,
            // no change
            (cell, _) => cell,
        }
    }

    pub fn step(&mut self) {
        let (page, frame) = if self.page == Page::Zero {
            (Page::One, true)
        } else {
            (Page::Zero, false)
        };

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                Mode5::write(page, x as usize, y as usize, self.next(x, y));
            }
        }

        DISPCNT.write(DISPCNT.read().with_frame1(frame));
        self.page = page;
    }
}
