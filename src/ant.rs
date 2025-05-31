use std::cell::RefCell;
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
    pub fn drop_scent(&mut self, scent: Scents, val: u8) -> u8 {
        let val = self.scents[scent as usize].max(val);
        self.scents[scent as usize] = val;
        val.saturating_sub(1)
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
    occupy: Rc<RefCell<Faction>>,
    // more optimal ways to do this (i.e. for the whole colony)
    // also would be better if we had to go back to the colony to eat...
    pub hunger: u8,
    // pub target_scent: ()
}

impl Ant {
    pub fn new(pos: Pos, faction: Faction) -> Self {
        Self {
            pos,
            dir: dirs::rand(),
            food: None,
            food_scent: 0,
            nest_scent: u8::MAX,
            occupy: Rc::new(RefCell::new(faction)),
            hunger: rand::gen_range(u8::MAX / 2, u8::MAX),
        }
    }

    fn sense(
        &self,
        pos: Pos,
        grid: &Map,
        scents: &ScentGrid,
        scent: Scents,
        flag: CellType,
    ) -> (Pos, u8) {
        (
            pos,
            if grid.get_cell(pos).is_some_and(|cell| cell.is_type(flag)) {
                u8::MAX
            } else {
                scents
                    .get(&pos)
                    .map(|cell| cell.get_scent(scent))
                    .unwrap_or(0)
                // .max(u8::MAX / 40)
                // we always want some chance to pick a direction...
                // but it it's too high we won't ofter go the way we want!
            },
        )
    }

    fn perceive(
        &self,
        grid: &Map,
        scents: &ScentGrid,
        scent: Scents,
        flag: CellType,
    ) -> [(Pos, u8); 5] {
        [
            self.sense(
                self.pos + rotate_left(rotate_left(self.dir)),
                grid,
                scents,
                scent,
                flag,
            ),
            self.sense(self.pos + rotate_left(self.dir), grid, scents, scent, flag),
            self.sense(self.pos + self.dir, grid, scents, scent, flag),
            self.sense(self.pos + rotate_right(self.dir), grid, scents, scent, flag),
            self.sense(
                self.pos + rotate_right(rotate_right(self.dir)),
                grid,
                scents,
                scent,
                flag,
            ),
        ]
    }

    pub fn update_scents(&mut self, scents: &mut ScentGrid) {
        let scent_cell = scents.entry(self.pos).or_default();

        if self.food.is_some() {
            let dist_approx = u8::MAX - self.nest_scent;
            self.food_scent = dist_approx
                .saturating_add(dist_approx)
                .saturating_add(dist_approx / 4);
        }

        let _ = scent_cell
            .drop_scent(Scents::Food, self.food_scent)
            .saturating_sub(1);
        // self.food_scent = self.food_scent.saturating_sub(2);

        // mark nest scent
        self.nest_scent = scent_cell.drop_scent(Scents::Nest, self.nest_scent);
        // self.nest_scent = self.nest_scent.saturating_sub(1);
    }

    pub fn update_behaviour(
        &mut self,
        map: &mut Map,
        food: &mut Food,
        faction: Faction,
    ) -> (Scents, CellType) {
        // take food
        let cell = map.get_cell_mut(self.pos).expect("Ant in invalid pos");

        if cell.is_type(CellType::Nest(faction)) {
            if self.food.is_some() {
                *food += 1;
                self.food = None;
                self.nest_scent = u8::MAX - 1;
                self.food_scent = 0;
                self.dir = invert(self.dir);
            }
        }
        // find food!
        else if let Some(food) = cell.take_type(CellType::Food) {
            self.food = Some(food);

            // distance to nest is approximately u8::MAX - self.nest_scent
            // but we need some margin because we won't take the optimal route due to randomness

            // self.nest_scent = 0;
            self.dir = invert(self.dir);
        }

        if self.food.is_some() {
            (Scents::Nest, CellType::Nest(faction))
            // find nest!
            //  else {
            // self.follow_scent(map, scents, Scents::Nest, CellType::Nest(faction))
            // }
        } else {
            (Scents::Food, CellType::Food)
            // else {
            //     self.follow_scent(map, scents, Scents::Food, CellType::Food)
            // }
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

    pub fn perception_weighted_random(&self, perception: [(Pos, u8); 5]) -> Option<Pos> {
        let mut max: u8 = 0;
        let mut max_pos: Pos = self.pos;
        let _total: u16 = perception.iter().fold(0, |sum, val| {
            if max < val.1 || (max == val.1 && rand::rand() % 2 == 0) {
                max = val.1;
                max_pos = val.0;
            }
            sum + val.1 as u16
        });

        // chance to just go randomly
        let val = rand::rand();
        if val < u32::MAX / 4 || max == 0 {
            // 50/50 chance to turn
            let val = rand::rand();
            if val < u32::MAX / 2 {
                // 50/50 chance of dir
                if val < u32::MAX / 4 {
                    Some(self.pos + rotate_left(self.dir))
                } else {
                    Some(self.pos + rotate_right(self.dir))
                }
            } else {
                Some(self.pos + self.dir)
            }
        } else {
            Some(max_pos)
        }

        // let mut pick = rand::gen_range(0, total);
        // for val in perception {
        //     if pick <= val.1 as u16 {
        //         return Some(val.0);
        //     }
        //     pick -= val.1 as u16;
        // }
        // None
    }

    pub fn update(
        &mut self,
        map: &mut Map,
        scents: &mut ScentGrid,
        food: &mut Food,
        faction: Faction,
    ) -> bool {
        // check if we were killed!
        let mut ref_cell_faction = self.occupy.borrow_mut();
        if *ref_cell_faction != faction {
            *ref_cell_faction = faction;
            return false;
        }
        drop(ref_cell_faction);

        // TEMP
        self.hunger = self.hunger.saturating_sub(1);
        if self.hunger == 0 && *food > 0 {
            *food = *food - 1;
            self.hunger = u8::MAX;
        }

        self.update_scents(scents);
        let scent_seeking = self.update_behaviour(map, food, faction);

        let perception = self.perceive(map, scents, scent_seeking.0, scent_seeking.1);

        // pick a dir based on random chance
        let next_pos = self.perception_weighted_random(perception);

        self.try_move(next_pos.unwrap(), map, faction)
    }

    pub fn has_food(&self) -> bool {
        self.food.is_some()
    }
}
