/*
 * This module is intended to control game state, instantiate and update the gamedata object
 */

use crate::gamedata::*;
use std::io::{Write, stdout};
use std::{thread, time};
use termion::raw::IntoRawMode;

pub struct Renderer {
    game_data: GameData,
    framerate: f64,
    buffer: termion::raw::RawTerminal<std::io::Stdout>
}

impl Renderer {
    // Todo: impl proper error passing Result<(), std::io::Error>
    pub fn new(framerate: f64) -> Self {
        let mut buf = stdout()
            .into_raw_mode()
            .expect("unable to instantiate alternate screen");
        write!(buf, "{}", termion::screen::ToAlternateScreen).expect("unable to switch to alternate screen");
        write!(buf, "{}", termion::cursor::Hide).expect("unable to hide cursor");
        Renderer {
            game_data: GameData::new(),
            framerate: framerate,
            buffer: buf
        }
    }

    // Todo: fix origin to 1,1 not 0,1
    /// Resets cursor to its origin (1,1)
    fn reset_cursor_origin(&mut self) -> Result<(), std::io::Error>{
        write!(self.buffer, "{}", termion::cursor::Goto(0,1))?;
        Ok(())
    }

    /// Clears current alternate buffer
    fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        write!(self.buffer, "{}", termion::clear::BeforeCursor)?;
        Ok(())
    }

    /// Displays entire frame character by character
    pub fn display_frame_raw(&mut self) -> Result<(), std::io::Error> {
        self.reset_cursor_origin()?; // Required to stop flickering

        // Row.
        for i in &self.game_data.game_cells {
            // Line (col). 
            for j in i {
                write!(self.buffer, "{}", char::from(j))?;
            }
        }
        for j in &self.game_data.game_toolbar.bar_chars {
            write!(self.buffer, "{}", j)?;
        }
        Ok(())
    }

    /// Starts rendering the game
    pub fn render(&mut self) -> Result<(), std::io::Error>{
        let mut deaths = 0;
        let mut generation = 0;
        let max_generations = 200;
        let delay = time::Duration::from_secs_f64(self.framerate);
        self.game_data.seed_test(700)?;
        self.clear_screen()?;

        while generation <= max_generations {
            self.display_frame_raw()?;
            self.game_data.update_game_logic(
                &mut generation,
                &max_generations,
                &mut deaths
            );
            thread::sleep(delay);
        }

        Ok(())
    }
}
