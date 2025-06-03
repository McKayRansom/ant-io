use std::collections::HashMap;

use macroquad::prelude::rand;

use crate::insect::{Insect, Perception};
use crate::map::{CellType, Faction, Map};

use crate::pos::Pos;
use crate::pos::dirs::invert;

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
    pub workers: Vec<Ant>,
    // pub soldiers: Vec<Ant>,
    pub nest_pos: Pos,
    // the plan is to have the map much bigger, so most of it will be scent-less...
    pub scents: ScentGrid,
}

const STARTING_FOOD: Food = 32;
const STARTING_ANTS: usize = 32;

impl AntColony {
    pub fn new(pos: Pos, map: &mut Map, faction: Faction) -> Self {
        map.get_cell_mut(pos)
            .unwrap()
            .set_type(crate::map::CellType::Nest(faction));

        Self {
            faction,
            food: STARTING_FOOD,
            workers: vec![Ant::new(pos, faction); STARTING_ANTS],
            scents: HashMap::new(),
            nest_pos: pos,
        }
    }

    pub fn update(&mut self, grid: &mut Map) {
        for cell in self.scents.values_mut() {
            cell.update();
        }

        self.workers
            .retain_mut(|ant| ant.update(grid, &mut self.scents, &mut self.food, self.faction));

        if self.food > self.workers.len() {
            // create new ants!
            self.workers.push(Ant::new(self.nest_pos, self.faction));
            self.food -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub insect: Insect,
    pub food: Option<()>,
    nest_scent: u8,
    pub food_scent: u8,
}

impl Ant {
    pub fn new(pos: Pos, faction: Faction) -> Self {
        Self {
            insect: Insect::new(pos, faction),
            food: None,
            food_scent: 0,
            nest_scent: u8::MAX,
        }
    }

    // smell pheremones only
    fn smell(scents: &ScentGrid, pos: Pos, scent: Scents) -> u8 {
        scents
            .get(&pos)
            .map(|cell| cell.get_scent(scent))
            .unwrap_or(0)
    }

    // fn perceive(&self, map: &Map, scents: &ScentGrid, seeking: (Scents, CellType)) -> Perception {
    // self.insect
    // .perception(map, seeking.1, |pos| Self::smell(scents, pos, seeking.0))
    // }

    pub fn update_scents(&mut self, scents: &mut ScentGrid) {
        let scent_cell = scents.entry(self.insect.pos).or_default();

        if self.food.is_some() {
            let dist_approx = u8::MAX - self.nest_scent;
            self.food_scent = dist_approx
                .saturating_add(dist_approx)
                .saturating_add(dist_approx / 10)
                .saturating_add(10);
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
        let cell = map
            .get_cell_mut(self.insect.pos)
            .expect("Ant in invalid pos");

        if cell.is_type(CellType::Nest(faction)) {
            if self.food.is_some() {
                *food += 1;
                self.food = None;
                self.nest_scent = u8::MAX - 1;
                self.food_scent = 0;
                self.insect.dir = invert(self.insect.dir);
            }
        }
        // find food!
        else if self.food.is_none() {
            if let Some(food) = cell.take_type(CellType::Food) {
                if self.insect.hunger < u8::MAX / 4 {
                    // eat it for ourselves
                    self.insect.hunger = u8::MAX;
                } else {
                    // take it back to the nest I guess
                    self.food = Some(food);
                    self.insect.dir = invert(self.insect.dir);
                }
            }
        }

        if self.food.is_some() {
            (Scents::Nest, CellType::Nest(faction))
        } else {
            (Scents::Food, CellType::Food)
        }
    }

    pub fn update_movement(
        &self,
        perception: &mut Perception,
        scents: &ScentGrid,
        seeking: (Scents, CellType),
    ) -> Option<Pos> {
        for percep in perception.iter() {
            // TODO: Run from enemies, how determine this?
            // if percep.1.faction !=
            if percep.1.cell_type == seeking.1 {
                // found it, no reason not to go there
                return Some(percep.0);
            }
        }

        // chance to just go randomly
        let val = rand::rand();
        if val < u32::MAX / 4 {
            return Some(self.insect.move_random());
        }

        // didn't see anything interesting, time to go by scents
        // I wish we could do a weighted thing, but it just doesn't seem to work right...
        let mut total: u16 = 0;
        for percep in perception.iter_mut() {
            // re-using cus lazy? or should we only smell directly ahead maybe?
            let scent = Self::smell(scents, percep.0, seeking.0);
            percep.1.faction = scent;
            total += scent as u16;
        }

        // no scents, just go randomly
        if total == 0 {
            return Some(self.insect.move_random());
        }

        // pick a dir based on random chance, but weighted towards the highest scent dir
        // let mut pick = rand::gen_range(0, total);
        let mut pos: Pos = self.insect.pos;
        let mut max: u8 = 0;
        for val in perception.iter() {
            if max <= val.1.faction {
                pos = val.0;
                max = val.1.faction;
            }
        }
        return Some(pos);
    }

    pub fn update(
        &mut self,
        map: &mut Map,
        scents: &mut ScentGrid,
        food: &mut Food,
        faction: Faction,
    ) -> bool {
        // TEMP
        if self.insect.hunger < u8::MAX / 8 && *food > 0 {
            *food = *food - 1;
            self.insect.hunger = u8::MAX;
        }

        let seeking = self.update_behaviour(map, food, faction);
        self.update_scents(scents);

        let mut perception = self.insect.perception(map);
        let next_pos = self.update_movement(&mut perception, scents, seeking);
        self.insect.update(next_pos, map, faction)
    }

    pub fn has_food(&self) -> bool {
        self.food.is_some()
    }
}
