use macroquad::prelude::rand;

use crate::{
    insect::Insect,
    map::{CellType, FACTION_NONE, FACTION_SPIDER, Map},
    pos::Pos,
};

pub struct Spider {
    pub insect: Insect,
    // pub curled: bool,
    pub speed: u8,
        pub reproduce: u8,
}

impl Spider {
    pub fn new(pos: Pos) -> Self {
        Self {
            insect: Insect::new(pos, FACTION_SPIDER),
            // curled: false,
            speed: 0,
            reproduce: rand::gen_range(0, u8::MAX / 4),
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
        self.reproduce += 1;
        if self.reproduce == u8::MAX {
            self.reproduce = 0;
            new_bugs.push(self.insect.pos);
        }

        let perception = self.insect.perception(map);
        let mut best_pos = Some(self.insect.move_random());
        for percep in &perception {
            if percep.1.cell_type == CellType::Empty && (percep.1.faction != FACTION_SPIDER && percep.1.faction != FACTION_NONE) {
                // Eat or something IDK, we are still vulerable from behind, TBD if this is OP
                best_pos = Some(percep.0);
                let occupy = map.get_cell_mut(percep.0).unwrap().try_occupy(FACTION_SPIDER);
                if occupy.is_none() {
                    // we ate something! Horray!
                    self.insect.hunger = u8::MAX;
                }
                // mark as rock or something IDK
                break;
            }
        }

        self.insect.update(
            if will_move { best_pos } else { None },
            map,
            FACTION_SPIDER,
        )
    }
}
