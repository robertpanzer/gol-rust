extern crate rand;
extern crate rustty;

use std::time::Duration;
use rand::Rng;
use rustty::HasSize;
use rustty::Event::*;
use rustty::ui::Painter;

use rustty::{Terminal, CellAccessor};

struct World {
    cols: usize,
    rows: usize,
    current: Vec<bool>,
    living_neighbours: Vec<u8>,
}

impl World {

    fn new(cols: usize, rows: usize) -> World {
        let mut rng = rand::thread_rng();
        let mut current: Vec<bool> = Vec::with_capacity(cols * rows);
        let mut living_neighbours: Vec<u8> = Vec::with_capacity(cols * rows);

        for _i in 0..(cols * rows) {
            current.push(rng.gen::<bool>());
            living_neighbours.push(0);
        }

        World { cols, rows, current, living_neighbours }
    }

    fn print_into_term(&self, term: &mut Terminal) {
        let cells = term.cellvec_mut();
        for i in 0..self.current.len() {
            let cell = &mut cells[i];
            cell.set_ch(match self.current[i] {
                true => 'X',
                false => ' '
            });

        }
    }

    fn next_iteration(&mut self) {
        for idx in 0..(self.cols*self.rows) {
            self.living_neighbours[idx] = self.living_neighbours(idx);
        }
        for idx in 0..(self.cols*self.rows) {
            if self.current[idx] {
                // A cell "starves" with less than 2 living neighbours
                // or dies to overpopulation with more than 4 living neighbours
                self.current[idx] = match self.living_neighbours[idx] {
                    2|3 =>  true,
                    _ => false,
                }
            } else {
                // A cell is "born" if exactly 3 neighbours are alive
                self.current[idx] = self.living_neighbours[idx] == 3
            }
        }
    }

    fn living_neighbours(&self, index: usize) -> u8 {
        let row = index / self.cols;
        let col = index % self.cols;

        let mut result = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 || j != 1 {
                    let x = (col + self.cols + i - 1) % self.cols;
                    let y = (row + self.rows + j - 1) % self.rows;
                    if self.current[y * self.cols + x] {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}


fn main() {

    let mut term = Terminal::new().unwrap();
    let term_size = term.size();

    let mut w = World::new(term_size.0, term_size.1 - 1);

    loop {
        w.print_into_term(&mut term);
        term.printline(0, term_size.1 - 1, " Press 'q' to exit, any other key to continue ");
        term.swap_buffers().unwrap();
        match term.get_event(Duration::from_secs(100000)).unwrap() {
            Some(Key('q')) => {
                break;
            },
            Some(Key(_)) => {
                w.next_iteration();
            },
            None => {}
        }
    }
}
