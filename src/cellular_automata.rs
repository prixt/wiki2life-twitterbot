#![allow(dead_code)]
use rayon::prelude::*;

static RULES: [u16;2] = [
    0b_0_0_0_1_0_0_0_0_0,
    0b_0_0_1_1_0_0_0_0_0,
];

pub struct CellularAutomata {
    width: usize, height: usize,
    current_cells: Vec<bool>,
    previous_cells: Vec<Box<[bool]>>,
}

impl CellularAutomata {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            current_cells: vec![false; width*height],
            previous_cells: vec![],
        }
    }

    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn load(&mut self, new_cells: &[bool])
    {
        assert!(self.current_cells.len() == new_cells.len(),
            "Length of new cells must be equal to CA-cells length.");
        self.current_cells
            .as_mut_slice()
            .copy_from_slice(new_cells);
    }

    pub fn step(&mut self) {
        let mut new_cells = vec![true; self.width * self.height];
        new_cells
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, cell)| {
                let neighbors = self.neighbors(i);
                *cell = (RULES[self.current_cells[i] as usize] & (0b_1_0_0_0_0_0_0_0_0u16 >> neighbors)) != 0;
            });
        let old_cells = std::mem::replace(&mut self.current_cells, new_cells);
        self.previous_cells.push(old_cells.into_boxed_slice());
    }

    #[inline]
    pub fn get_current_cells(&self) -> &[bool] {
        self.current_cells.as_slice()
    }

    #[inline]
    pub fn get_previous_cells(&self) -> &[Box<[bool]>] {
        self.previous_cells.as_slice()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.previous_cells.len() + 1
    }

    pub fn is_oscillating(&self) -> bool {
        let current_slice = self.current_cells.as_slice();
        self.previous_cells
            .par_iter()
            .rev()
            .any(|x| x.as_ref() == current_slice)
    }

    fn neighbors(&self, i: usize) -> usize {
        let mut count = 0;
        let (x, y) = (i % self.width, i / self.width);
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {continue}
                let (mut cx, mut cy) = (x as i32 + dx, y as i32 + dy);
                if cx < 0 {
                    cx += self.width as i32;
                }
                else if cx >= self.width as i32 {
                    cx -= self.width as i32;
                }
                if cy < 0 {
                    cy += self.height as i32;
                }
                else if cy >= self.height as i32 {
                    cy -= self.height as i32;
                }

                if self.current_cells[cy as usize * self.width + cx as usize] {
                    count += 1;
                }
            }
        }
        count
    }
}