use macroquad::prelude::rand;

use crate::{
    insect::Insect,
    map::{CellType, FACTION_NONE, FACTION_PILLBUG, FACTION_SPIDER, Map},
    pos::Pos,
};

pub struct Spider {
    pub insect: Insect,
    // pub curled: bool,
    pub speed: u8,
    pub reproduce: u8,
    pub digest: u8,
}

const DIGEST_TIME: u8 = 32;

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
        let will_move = if self.speed == 1 {
            self.speed = 0;
            true
        } else {
            self.speed = 1;
            false
        };

        self.reproduce = self.reproduce.saturating_add(1);
        if self.reproduce == u8::MAX && self.insect.hunger > u8::MAX as u16 {
            self.reproduce = 0;
            self.insect.hunger -= u8::MAX as u16;
            new_bugs.push(self.insect.pos);
        }

        // use more energy as balancing
        // self.insect.hunger = self.insect.hunger.saturating_sub(1);

        // oof
        self.digest = self.digest.saturating_sub(1);

        let perception = self.insect.perception(map);
        let mut best_pos = Some(self.insect.move_random());
        for percep in &perception {
            if percep.1.cell_type == CellType::Empty // temp fix to not eat hiding pillbugs (rocks)
                && (self.digest == 0 || percep.1.faction == FACTION_PILLBUG) // always eat pillbugs, they be pestin
                && (percep.1.faction != FACTION_SPIDER && percep.1.faction != FACTION_NONE)
            {
                // Eat or something IDK, we are still vulerable from behind, TBD if this is OP
                best_pos = None; // don't move into pos and die
                let occupy = map
                    .get_cell_mut(percep.0)
                    .unwrap()
                    .try_occupy(FACTION_SPIDER);
                if occupy.is_none() {
                    // we ate something! Horray!
                    self.insect.hunger = self.insect.hunger.saturating_add(u8::MAX as u16);
                    self.digest = DIGEST_TIME;
                }
                break;
            }
        }

        self.insect
            .update(if will_move { best_pos } else { None }, map, FACTION_SPIDER)
    }
}
