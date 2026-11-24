/*
 * This module is intended to provide basic state and data management for the underlying game
 */

#![allow(dead_code)]
use rand::{distr::weighted::Weight, prelude::*};

#[derive(Clone)]
pub enum CellState {
    Alive,
    Dead,
}

pub struct Toolbar {
    pub bar_chars: Vec<char>,
    current_generation: usize,
    maxima_generation: usize,
    current_deaths: usize,
    frame_width: u16
}

pub struct GameData {
    pub game_cells: Vec<Vec<CellState>>,
    pub game_toolbar: Toolbar,
    pub frame_height: u16,
    pub frame_width: u16,
}

impl From<char> for CellState {
    fn from(value: char) -> Self {
        match value {
            ' ' => CellState::Dead,
            '●' => CellState::Alive,
            _ => CellState::Dead
        }
    }
}
impl From<&CellState> for char {
    fn from(value: &CellState) -> char {
        match value {
            CellState::Dead => ' ',
            CellState::Alive => '●',
            _ => ' '
        }
    }
}
impl From<&char> for CellState {
    fn from(value: &char) -> Self {
        match value {
            ' ' => CellState::Dead,
            '●' => CellState::Alive,
            _ => CellState::Dead
        }
    }
}
impl From<i32> for CellState {
    fn from(value: i32) -> Self {
        match value {
            1 => CellState::Dead,
            2 => CellState::Alive,
            _ => CellState::Dead
        }
    }
}

impl Toolbar {
    pub fn new(width: &u16) -> Self {
        let mut i = Vec::new();
        for _ in 0..*width {
            i.push('=');
        }
        Toolbar { bar_chars: i, current_generation: 0, maxima_generation: 0, current_deaths: 0, frame_width: *width }
    }

    pub fn update(&mut self, cgen: &i32, mgen: &i32, cdeaths: &i32) {
        let mut charlist: Vec<char> = format!("|CURR_GEN:{}|MAX_GEN:{}|CURR_DEATHS:{}|", *cgen, *mgen, *cdeaths).chars().collect();
        for _ in charlist.len()..(self.frame_width as usize)-1 {
            charlist.push('='); // Right pad
        }
        self.bar_chars = charlist; 
    }
}

impl GameData {
    /// Initiates a new instance of game data, including toolbar info.
    pub fn new() -> Self {
        let mut gf = GameData {
            game_cells: Vec::new(),
            game_toolbar: Toolbar{
                bar_chars: Vec::new(), 
                current_generation: 0, 
                maxima_generation: 0,
                current_deaths: 0,
                frame_width: 0
            },
            frame_height: 0,
            frame_width: 0,
        };

        // Initialise frame data as blank screen...
        if let Ok((w, h)) = termion::terminal_size() {
            gf.frame_height = h;
            gf.frame_width = w;

            // Print frame data, reserve height for toolbar
            for _ in 0..(h - 1) {
                let rowdata = {
                    let mut i = Vec::new();
                    for _ in 0..w {
                        i.push(CellState::Dead);
                    }
                    i
                };

                // Push row data to column index
                gf.game_cells.push(rowdata);
            }

            // Initialise toolbar
            gf.game_toolbar = Toolbar::new(&w);
            gf.game_toolbar.update(&0, &0, &0);
        }
        gf
    }

    pub fn update_game_logic(&mut self, current_generation: &mut i32, max_generations: &i32, current_deaths: &mut i32) {
        let mut frame = self.game_cells.clone();

        for cell_counter_y in 0..self.game_cells.len() {
            for cell_counter_x in 0..self.game_cells[cell_counter_y].len() {

                // Get total neighbors (3x3) and count
                let mut neighbor_count = 0;
                for o_y in -1..=1 {
                    let offset_y: usize = match (cell_counter_y as usize).checked_add_signed(o_y) {
                        Some(y) => y,
                        None => {continue}
                    };
                    for o_x in -1..=1 {
                        if o_x == 0 && o_y == 0 { continue } // Looking at our current cell :)

                        let offset_x: usize = match (cell_counter_x as usize).checked_add_signed(o_x) {
                            Some(x) => x,
                            None => {continue}
                        };

                        match self.game_cells.get(offset_y).and_then(|row| row.get(offset_x)) {
                            Some(CellState::Alive) => neighbor_count += 1,
                            _ => {}
                        }
                    }
                }

                // Decide the poor fellas fate, he might just become a statistic.
                match self.game_cells[cell_counter_y][cell_counter_x] {
                    CellState::Alive => {
                        if neighbor_count < 2 {
                            frame[cell_counter_y][cell_counter_x] = CellState::Dead;
                            *current_deaths += 1;
                        }
                        if neighbor_count > 3 {
                            frame[cell_counter_y][cell_counter_x] = CellState::Dead;
                            *current_deaths += 1;
                        }
                    }
                    CellState::Dead => {
                        if neighbor_count == 3 {
                            frame[cell_counter_y][cell_counter_x] = CellState::Alive;
                        }
                    }
                }

            }
            self.game_toolbar.update(&current_generation,&max_generations, &current_deaths);
        };
        *current_generation += 1;
        self.game_cells = frame;
    }

    pub fn update_cell(&mut self, cellstate: CellState, posx: usize, posy: usize) {
        self.game_cells[posy][posx] = cellstate;
    }

    pub fn seed_test(&mut self, maxalive: i32) -> Result<(), std::io::Error> {
        let mut r = rand::rng(); // Seed only
        let generations = r.random_range(1..10);

        for _ in 1..generations {
            for _ in 1..maxalive {
                let state = r.random_range(1..=2).into();
                self.update_cell(
                    state,
                    r.random_range(0..self.frame_width) as usize,
                    r.random_range(0..self.frame_height - 1) as usize,
                );
            }
        }

        Ok(())
    }
}
