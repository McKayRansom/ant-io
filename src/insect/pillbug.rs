use macroquad::prelude::rand;

use crate::{
    insect::{Action, BaseInsect, Event, Insect, InsectBehaviour},
    map::{CellType, FACTION_NONE, FACTION_PILLBUG, Map},
    pos::Pos,
};

use super::Hunger;

pub struct Pillbug {
    pub curled: bool,
    pub speed: u8,
    pub reproduce: u8,
}

const PILLBUG_REPRODUCE_TIME: u8 = 128;
const PILLBUG_REPRODUCE_COST: Hunger = u8::MAX as Hunger;

const PILLBUG_EAT_THRESHOLD: Hunger = u8::MAX as Hunger * 2;
const PILLBUG_FOOD_VALUE: Hunger = u8::MAX as Hunger;

const PILLBUG_SPEED: u8 = 1;

impl Pillbug {
    pub fn new(pos: Pos) -> Insect {
        Insect {
            base: BaseInsect::new(pos, FACTION_PILLBUG),
            spec: Box::new(Self {
                // insect: BaseInsect::new(pos, FACTION_PILLBUG),
                curled: false,
                speed: 0,
                reproduce: rand::gen_range(0, PILLBUG_REPRODUCE_TIME / 4),
            }),
        }
    }
}

impl InsectBehaviour for Pillbug {
    fn update(&mut self, base: &mut BaseInsect, map: &mut Map) -> Option<Event> {
        let mut will_move = BaseInsect::will_move(&mut self.speed, PILLBUG_SPEED);

        if base.update_reproduce(
            &mut self.reproduce,
            PILLBUG_REPRODUCE_TIME,
            PILLBUG_REPRODUCE_COST,
        ) {
            return Some(Event::Birth(Box::new(Self::new(base.pos))));
        }

        // base.hunger = base.hunger.saturating_sub(1);

        if will_move {
            // eat the food?
            if base.hunger < PILLBUG_EAT_THRESHOLD {
                if let Some(_food) = map
                    .get_cell_mut(base.pos)
                    .unwrap()
                    .take_type(CellType::Food)
                {
                    base.hunger += PILLBUG_FOOD_VALUE;
                }
            } else if map
                .get_cell(base.pos)
                .unwrap()
                .is_type(CellType::Food)
            {
                // no point in moving lol, stay on the food!
                will_move = false;
            }
        }

        let perception = base.perception(map);
        let mut best_pos = Some(base.move_random());
        if self.curled {
            self.curled = false;
            map.get_cell_mut(base.pos).unwrap().m_type = CellType::Empty;
        }
        for percep in &perception {
            if percep.1.faction != FACTION_PILLBUG && percep.1.faction != FACTION_NONE {
                // scary!
                self.curled = true;
                best_pos = None;
                // mark as rock or something so we can't be eaten
                // let cell = map.get_cell_mut(base.pos).unwrap();
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

        if will_move && best_pos.is_some() {
            base.try_move(best_pos.unwrap(), map) // || self.curled
        } else {
            None
        }
    }
    
    fn player_action(&mut self, _base: &mut BaseInsect, _map: &mut Map, _action: Action) -> Option<Event> {
        todo!()
    }

}
