use macroquad::prelude::rand;

use crate::{
    insect::Insect,
    map::{FACTION_NONE, FACTION_PILLBUG, FACTION_SPIDER, Map},
    pos::Pos,
};

use super::Hunger;

pub struct Spider {
    pub insect: Insect,
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
    pub fn new(pos: Pos) -> Self {
        Self {
            insect: Insect::new(pos, FACTION_SPIDER),
            // curled: false,
            speed: 0,
            reproduce: rand::gen_range(0, u8::MAX / 4),
            digest: 0,
        }
    }

    pub fn update(&mut self, map: &mut Map, new_bugs: &mut Vec<Pos>) -> bool {

        if self.insect.update_reproduce(&mut self.reproduce, SPIDER_REPRODUCE_TIME, SPIDER_REPRODUCE_COST) {
            new_bugs.push(self.insect.pos);
        }

        // oof
        self.digest = self.digest.saturating_sub(1);

        let perception = self.insect.perception(map);
        let mut best_pos = Some(self.insect.move_random());
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
                best_pos = None; // don't move into pos and die
                let occupy = map
                    .get_cell_mut(percep.0)
                    .unwrap()
                    .try_occupy(FACTION_SPIDER);
                if occupy.is_none() {
                    // we ate something! Horray!
                    self.insect.hunger = self.insect.hunger.saturating_add(SPIDER_EAT_VAL);
                    self.digest = DIGEST_TIME;
                }
                break;
            }
        }

        let will_move = Insect::will_move(&mut self.speed, SPIDER_SPEED);

        self.insect
            .update(if will_move { best_pos } else { None }, map, FACTION_SPIDER)
    }
}
