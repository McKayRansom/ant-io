use macroquad::prelude::rand;

use crate::{
    insect::{Action, BaseInsect, Event, Insect, InsectBehaviour},
    map::{FACTION_NONE, FACTION_SPIDER, Map},
    pos::Pos,
};

use super::Hunger;

pub struct Spider {
    // pub curled: bool,
    pub speed: u8,
    pub reproduce: u8,
    pub digest: u8,
}

const SPIDER_REPRODUCE_TIME: u8 = 250;
const SPIDER_REPRODUCE_COST: Hunger = 512;
const SPIDER_REPRODUCE_THRESHOLD: Hunger = 512;

const SPIDER_EAT_VAL: Hunger = 255;

// ANTS ONLY FOR NOW
const DIGEST_TIME: u8 = 32;

const SPIDER_SPEED: u8 = 1;

impl Spider {
    pub fn new(pos: Pos) -> Insect {
        Insect {
            base: BaseInsect::new(pos, FACTION_SPIDER),
            spec: Box::new(Self {
                // curled: false,
                speed: 0,
                reproduce: rand::gen_range(0, u8::MAX / 4),
                digest: 0,
            }),
        }
    }
}

impl InsectBehaviour for Spider {
    fn update(&mut self, base: &mut BaseInsect, map: &mut Map) -> Option<Event> {
        if base.update_reproduce(
            &mut self.reproduce,
            SPIDER_REPRODUCE_TIME,
            SPIDER_REPRODUCE_COST,
        ) {
            return Some(Event::Birth(Box::new(Self::new(base.pos))));
        }

        // oof
        self.digest = self.digest.saturating_sub(1);

        let perception = base.perception(map);
        let mut best_pos = Some(base.move_random());
        for percep in &perception {
            // if percep.1.cell_type == CellType::Empty // temp fix to not eat hiding pillbugs (rocks)
            // && (self.digest == 0 || percep.1.faction == FACTION_PILLBUG) // always eat pillbugs, they be pestin
            // only eat pillbugs (temp)
            if self.digest == 0
                && (percep.1.faction != FACTION_SPIDER && percep.1.faction != FACTION_NONE)
            // || percep.1.faction == FACTION_PILLBUG
            // if self.digest == 0 && (percep.1.faction == FACTION_PILLBUG)
            // if percep.1.faction == FACTION_PILLBUG
            {
                // Eat or something IDK, we are still vulerable from behind, TBD if this is OP
                // Attack the thing!
                best_pos = Some(percep.0); // don't move into pos and die
                // we ate something! Horray!
                // base.hunger = base.hunger.saturating_add(SPIDER_EAT_VAL);
                self.digest = DIGEST_TIME;
                break;
            }
        }

        if BaseInsect::will_move(&mut self.speed, SPIDER_SPEED) && best_pos.is_some() {
            base.try_move(best_pos.unwrap(), map)
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
                if base.update_reproduce(
                    &mut self.reproduce,
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
