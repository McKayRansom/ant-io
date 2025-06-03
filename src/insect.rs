use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::rand;

use crate::{
    map::{Faction, Map, OccupyError, Sight},
    pos::{
        Pos,
        dirs::{self, invert, rotate_left, rotate_right},
    },
};

pub mod ant;
pub mod pillbug;
pub mod spider;

// #[derive(Debug, Clone, Copy)]
// pub struct Perception {
// raw: [(Pos, u8); 5],
// }

pub type Perception = [(Pos, Sight); 5];

pub type Hunger = u16;

// pub const DE

/// Base class-ish for different insect types
#[derive(Debug, Clone)]
pub struct Insect {
    pub pos: Pos,
    pub dir: Pos,
    occupy: Rc<RefCell<Faction>>,
    pub hunger: Hunger,
}

impl Insect {
    pub fn new(pos: Pos, faction: Faction) -> Self {
        Self {
            pos,
            dir: dirs::rand(),
            occupy: Rc::new(RefCell::new(faction)),
            hunger: rand::gen_range(u8::MAX as u16 / 2 , u8::MAX as u16),
        }
    }

    pub fn sight(pos: Pos, map: &Map) -> (Pos, Sight) {
        (pos, map.sight(pos))
    }

    pub fn perception(&self, map: &Map) -> Perception {
        [
            Self::sight(self.pos + rotate_left(rotate_left(self.dir)), map),
            Self::sight(self.pos + rotate_left(self.dir), map),
            Self::sight(self.pos + self.dir, map),
            Self::sight(self.pos + rotate_right(self.dir), map),
            Self::sight(self.pos + rotate_right(rotate_right(self.dir)), map),
        ]
    }

    pub fn move_random(&self) -> Pos {
        // 50/50 chance to turn
        let val = rand::rand();
        if val < u32::MAX / 2 {
            // 50/50 chance of dir
            if val < u32::MAX / 4 {
                self.pos + dirs::rotate_left(self.dir)
            } else {
                self.pos + dirs::rotate_right(self.dir)
            }
        } else {
            self.pos + self.dir
        }
    }

    pub fn update(&mut self, next_pos: Option<Pos>, map: &mut Map, faction: Faction) -> bool {
        // check if we were killed!
        let mut ref_cell_faction = self.occupy.borrow_mut();
        if *ref_cell_faction != faction {
            *ref_cell_faction = faction;
            return false;
        }
        drop(ref_cell_faction);

        if let Some(next_pos) = next_pos {
            // FOR NOW: Only expend hunger when mooving
            // check if we die of hunger
            self.hunger = self.hunger.saturating_sub(1);
            if self.hunger == 0 {
                // we die
                return false;
            }
            match map.occupy(next_pos, faction) {
                Ok(occupy) => {
                    self.dir = next_pos - self.pos;
                    self.pos = next_pos;
                    self.occupy = occupy;
                }
                Err(OccupyError::Solid) => self.dir = invert(self.dir),
                Err(OccupyError::Fight) => {
                    // TODO: REAL FIGHTS
                    // map.occupy() should have marked opponent as occupied, so they should die too
                    // we die now
                    return false;
                }
            }
        }

        true
    }
}
