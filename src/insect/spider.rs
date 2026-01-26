use macroquad::prelude::rand;

use crate::{
    insect::{Action, BaseInsect, Event, Insect, InsectBehaviour},
    map::{CellType, FACTION_NONE, FACTION_SPIDER, Map},
    pos::Pos,
};

use super::Hunger;

pub struct Spider {
    // pub curled: bool,
    pub speed: u8,
    pub reproduce: u8,
    // pub digest: u8,
}

const SPIDER_REPRODUCE_TIME: u8 = 250;
const SPIDER_REPRODUCE_COST: Hunger = 512;
const SPIDER_REPRODUCE_THRESHOLD: Hunger = 512;

const SPIDER_EAT_VAL: Hunger = 255;

// ANTS ONLY FOR NOW
// const DIGEST_TIME: u8 = 32;

const SPIDER_SPEED: u8 = 1;

impl Spider {
    pub fn new(pos: Pos) -> Insect {
        Insect {
            base: BaseInsect::new(pos, FACTION_SPIDER),
            spec: Box::new(Self {
                // curled: false,
                speed: 0,
                reproduce: rand::gen_range(0, u8::MAX / 4),
                // digest: 0,
            }),
        }
    }
}

impl InsectBehaviour for Spider {
    fn update(&mut self, base: &mut BaseInsect, map: &mut Map) -> Option<Event> {

        let mut fake_repro = SPIDER_REPRODUCE_TIME;
        if base.update_reproduce(
            &mut fake_repro,
            SPIDER_REPRODUCE_TIME,
            SPIDER_REPRODUCE_COST,
        ) {
            return Some(Event::Birth(Box::new(Self::new(base.pos))));
        }

        if base.hunger < SPIDER_REPRODUCE_COST {
            if let Some(_food) = map
                .get_cell_mut(base.pos)
                .unwrap()
                .take_type(CellType::Food)
            {
                base.hunger += SPIDER_EAT_VAL;
            }
        }

        // oof
        // self.digest = self.digest.saturating_sub(1);

        let perception = base.perception(map);
        let mut enemy_pos = None;
        let mut food_pos = None;
        for percep in &perception {
            // if percep.1.cell_type == CellType::Empty // temp fix to not eat hiding pillbugs (rocks)
            // && (self.digest == 0 || percep.1.faction == FACTION_PILLBUG) // always eat pillbugs, they be pestin
            // only eat pillbugs (temp)
            if percep.1.faction != FACTION_SPIDER && percep.1.faction != FACTION_NONE
            // || percep.1.faction == FACTION_PILLBUG
            // if self.digest == 0 && (percep.1.faction == FACTION_PILLBUG)
            // if percep.1.faction == FACTION_PILLBUG
            {
                // Eat or something IDK, we are still vulerable from behind, TBD if this is OP
                // Attack the thing!
                enemy_pos = Some(percep.0); // don't move into pos and die
                // we ate something! Horray!
                // base.hunger = base.hunger.saturating_add(SPIDER_EAT_VAL);
                // self.digest = DIGEST_TIME;
                break;
            }
            if percep.1.cell_type == CellType::Food {
                food_pos = Some(percep.0);
            }
        }

        if BaseInsect::will_move(&mut self.speed, SPIDER_SPEED)
            && (enemy_pos.is_some() || food_pos.is_some())
        {
            base.try_move(enemy_pos.unwrap_or_else(|| food_pos.unwrap()), map)
        } else {
            None
        }
    }

    fn player_action(
        &mut self,
        base: &mut BaseInsect,
        _map: &mut Map,
        action: Action,
    ) -> Option<Event> {
        match action {
            Action::ActionA => {

                let mut fake_repro = SPIDER_REPRODUCE_TIME;
                if base.update_reproduce(
                    &mut fake_repro,
                    SPIDER_REPRODUCE_TIME,
                    SPIDER_REPRODUCE_COST,
                ) {
                    return Some(Event::Birth(Box::new(Self::new(base.pos))));
                }
            }

            Action::ActionB => todo!(),
            _ => {}
        }
        None
    }
}
