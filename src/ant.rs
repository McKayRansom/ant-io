use std::collections::HashMap;
use std::rc::Rc;

use macroquad::prelude::rand;

use crate::map::{CellType, Faction, Map, OccupyError};

use crate::pos::dirs::{invert, rotate_left, rotate_right};
use crate::pos::{Pos, dirs};

// const

type Food = usize;

#[derive(Debug, Clone, Copy)]
pub enum Scents {
    Food,
    Nest,
    Len,
}

#[derive(Debug, Default)]
pub struct ScentCell {
    scents: [u8; Scents::Len as usize],
}

impl ScentCell {
    pub fn get_scent(&self, scent: Scents) -> u8 {
        self.scents[scent as usize]
    }
    pub fn drop_scent(&mut self, scent: Scents, val: u8) {
        self.scents[scent as usize] = self.scents[scent as usize].max(val)
    }

    fn update(&mut self) {
        // self.nest_scent = self.nest_scent.saturating_sub(1);
        self.scents[Scents::Food as usize] = self.scents[Scents::Food as usize].saturating_sub(1);
    }
}

pub type ScentGrid = HashMap<Pos, ScentCell>;

#[derive(Debug)]
pub struct AntColony {
    pub faction: Faction,
    pub food: Food,
    pub ants: Vec<Ant>,
    pub nest_pos: Pos,
    // the plan is to have the map much bigger, so most of it will be scent-less...
    pub scents: ScentGrid,
}

const STARTING_FOOD: Food = 32;
const ANTS_NUMBER: usize = 32;

impl AntColony {
    pub fn new(pos: Pos, map: &mut Map, faction: Faction) -> Self {
        map.get_cell_mut(pos)
            .unwrap()
            .set_type(crate::map::CellType::Nest(faction));

        Self {
            faction,
            food: STARTING_FOOD,
            ants: vec![Ant::new(pos, faction); ANTS_NUMBER],
            scents: HashMap::new(),
            nest_pos: pos,
        }
    }

    pub fn update(&mut self, grid: &mut Map) {
        // for row in self.occupied.iter_mut() {
        for cell in self.scents.values_mut() {
            cell.update();
        }
        // }

        // let mut all_snakes_dead = true;
        self.ants
            .retain_mut(|ant| ant.update(grid, &mut self.scents, &mut self.food, self.faction));
        //     if i == 0 {
        //         // player died
        //         self.game_over = true;
        //         self.game_won = false;
        //     }
        // } else if i != 0 {
        //     all_snakes_dead = false;
        // }
        // }

        if self.food > self.ants.len() {
            // create new ants!
            self.ants.push(Ant::new(self.nest_pos, self.faction));
            self.food -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub pos: Pos,
    pub food: Option<()>,
    nest_scent: u8,
    pub food_scent: u8,
    pub dir: Pos,
    occupy: Rc<Faction>,
    // more optimal ways to do this (i.e. for the whole colony)
    // also would be better if we had to go back to the colony to eat...
    pub hunger: u8,
}

impl Ant {
    pub fn new(pos: Pos, faction: Faction) -> Self {
        Self {
            pos,
            dir: dirs::rand(),
            food: None,
            food_scent: 0,
            nest_scent: u8::MAX,
            occupy: Rc::new(faction),
            hunger: rand::gen_range(u8::MAX / 2, u8::MAX),
        }
    }

    fn follow_scent(
        &self,
        grid: &Map,
        scents: &mut ScentGrid,
        scent: Scents,
        flag: CellType,
    ) -> Option<Pos> {
        let (_max_scent, pos) = [
            self.pos + rotate_left(rotate_left(self.dir)),
            self.pos + rotate_left(self.dir),
            self.pos + self.dir,
            self.pos + rotate_right(self.dir),
            self.pos + rotate_right(rotate_right(self.dir)),
        ]
        .iter()
        .fold((0, self.pos), |max, pos| {
            if let Some(scent) = grid.get_cell(*pos).map(|cell| {
                if cell.is_type(flag) {
                    u8::MAX
                } else {
                    scents
                        .get(pos)
                        .map(|cell| cell.get_scent(scent))
                        .unwrap_or(0)
                }
            }) {
                if scent > max.0 { (scent, *pos) } else { max }
            } else {
                max
            }
        });
        if pos != self.pos { Some(pos) } else { None }
    }

    pub fn update_food_scents(
        &mut self,
        map: &mut Map,
        scents: &mut ScentGrid,
        food: &mut Food,
        faction: Faction,
    ) -> Option<Pos> {
        // take food
        let cell = map.get_cell_mut(self.pos).expect("Ant in invalid pos");
        let scent_cell = scents.entry(self.pos).or_default();

        // mark food scent
        scent_cell.drop_scent(Scents::Food, self.food_scent);
        self.food_scent = self.food_scent.saturating_sub(2);

        // mark nest scent
        scent_cell.drop_scent(Scents::Nest, self.nest_scent);
        self.nest_scent = self.nest_scent.saturating_sub(1);

        if self.food.is_some() {
            // find nest!
            if cell.is_type(CellType::Nest(faction)) {
                *food += 1;
                self.food = None;
                self.nest_scent = u8::MAX - 1;
                self.food_scent = 0;
                self.dir = invert(self.dir);
                return None;
            } else {
                self.follow_scent(map, scents, Scents::Nest, CellType::Nest(faction))
            }
        } else {
            // find food!
            if let Some(food) = cell.take_type(CellType::Food) {
                self.food = Some(food);
                self.food_scent = u8::MAX - 1;
                self.nest_scent = 0;
                self.dir = invert(self.dir);
                return None;
            } else {
                self.follow_scent(map, scents, Scents::Food, CellType::Food)
            }
        }
    }

    pub fn try_move(&mut self, next_pos: Pos, map: &mut Map, faction: Faction) -> bool {
        match map.occupy(next_pos, faction) {
            Ok(occupy) => {
                self.dir = next_pos - self.pos;
                self.pos = next_pos;
                self.occupy = occupy;
            }
            Err(OccupyError::Solid) => self.dir = invert(self.dir),
            Err(OccupyError::Fight) => {
                // TODO: REAL FIGHTS
                // we die now
                return false;
            }
        }
        true
    }

    pub fn update(
        &mut self,
        map: &mut Map,
        scents: &mut ScentGrid,
        food: &mut Food,
        faction: Faction,
    ) -> bool {
        let mut next_pos = self.update_food_scents(map, scents, food, faction);

        // TEMP
        self.hunger = self.hunger.saturating_sub(1);
        if self.hunger == 0 && *food > 0 {
            *food = *food - 1;
            self.hunger = u8::MAX;
        }

        // 25% chance of ignoring best pheremone dir...
        if next_pos.is_none() || rand::rand() < u32::MAX / 4 {
            // move random, but prefer same dir
            next_pos = Some(
                self.pos
                    + if rand::rand() < u32::MAX / 2 {
                        if rand::rand() < u32::MAX / 2 {
                            rotate_right(self.dir)
                        } else {
                            rotate_left(self.dir)
                        }
                    } else {
                        self.dir
                    },
            );
        }

        self.try_move(next_pos.unwrap(), map, faction)

        // self.body.push_front(self.pos);

        // let new_head = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);

        // if grid.occupy(new_head) {
        //     if is_ai {
        //         let dirs = if rand::RandomRange::gen_range(0, 2) == 0 {
        //             DIRS
        //         } else {
        //             DIRS_REV
        //         };
        //         for dir in dirs {
        //             let new_head = (self.pos.0 + dir.0, self.pos.1 + dir.1);
        //             if !grid.occupy(new_head) {
        //                 self.pos = new_head;
        //                 self.dir = *dir;
        //                 return false;
        //             }
        //         }
        //     }
        //     return true;
        // }

        // self.pos = new_head;
        // false
    }

    pub fn has_food(&self) -> bool {
        self.food.is_some()
    }
}
