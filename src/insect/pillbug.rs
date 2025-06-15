use macroquad::prelude::rand;

use crate::{
    insect::{Insect, Species},
    map::{CellType, FACTION_NONE, FACTION_PILLBUG, Map},
    pos::Pos,
};

use super::Hunger;

pub struct Pillbug {
    pub insect: Insect,
    pub curled: bool,
    pub speed: u8,
    pub reproduce: u16,
}

const PILLBUG_REPRODUCE_TIME: u16 = 128;
const PILLBUG_REPRODUCE_COST: Hunger = u8::MAX as Hunger;
const PILLBUG_SPEED: u8 = 2;

const PILLBUG_EAT_THRESHOLD: Hunger = u8::MAX as Hunger * 2;
const PILLBUG_FOOD_VALUE: Hunger = u8::MAX as Hunger;

pub fn init() -> Species {
    Species {
        faction_id: FACTION_PILLBUG,
        speed: PILLBUG_SPEED,
        food_storage: PILLBUG_REPRODUCE_COST * 2,
    }
}

impl Pillbug {
    pub fn new(pos: Pos) -> Self {
        Self {
            insect: Insect::new(pos, FACTION_PILLBUG),
            curled: false,
            speed: 0,
            reproduce: rand::gen_range(0, PILLBUG_REPRODUCE_TIME / 4),
        }
    }

    pub fn update(&mut self, map: &mut Map, new_bugs: &mut Vec<Pos>) -> bool {
        let mut will_move = if self.speed == 1 {
            self.speed = 0;
            true
        } else {
            self.speed = 1;
            false
        };
        self.reproduce = self.reproduce.saturating_add(1);
        if self.reproduce >= PILLBUG_REPRODUCE_TIME && self.insect.hunger > PILLBUG_REPRODUCE_COST {
            self.reproduce = 0;
            self.insect.hunger -= PILLBUG_REPRODUCE_COST;
            new_bugs.push(self.insect.pos);
        }

        // self.insect.hunger = self.insect.hunger.saturating_sub(1);

        if will_move {
            // eat the food?
            if self.insect.hunger < PILLBUG_EAT_THRESHOLD {
                if let Some(_food) = map
                    .get_cell_mut(self.insect.pos)
                    .unwrap()
                    .take_type(CellType::Food)
                {
                    self.insect.hunger += PILLBUG_FOOD_VALUE;
                }
            } else if map
                .get_cell(self.insect.pos)
                .unwrap()
                .is_type(CellType::Food)
            {
                // no point in moving lol, stay on the food!
                will_move = false;
            }
        }

        let perception = self.insect.perception(map);
        let mut best_pos = Some(self.insect.move_random());
        if self.curled {
            self.curled = false;
            map.get_cell_mut(self.insect.pos).unwrap().m_type = CellType::Empty;
        }
        for percep in &perception {
            if percep.1.faction != FACTION_PILLBUG && percep.1.faction != FACTION_NONE {
                // scary!
                self.curled = true;
                best_pos = None;
                // mark as rock or something so we can't be eaten
                // let cell = map.get_cell_mut(self.insect.pos).unwrap();
                // if cell.m_type == CellType::Empty {
                //     cell.m_type = CellType::Rock;
                // }

                break;
            }
            if percep.1.cell_type == CellType::Food {
                // food!
                best_pos = Some(percep.0);
            }
        }

        self.insect.update(
            if will_move { best_pos } else { None },
            map,
            FACTION_PILLBUG,
        ) // || self.curled
    }
}
