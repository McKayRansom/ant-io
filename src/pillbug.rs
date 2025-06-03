use crate::{
    insect::Insect,
    map::{CellType, FACTION_NONE, FACTION_PILLBUG, Map},
    pos::Pos,
};

pub struct Pillbug {
    pub insect: Insect,
    pub curled: bool,
    pub speed: u8,
}

impl Pillbug {
    pub fn new(pos: Pos) -> Self {
        Self {
            insect: Insect::new(pos, FACTION_PILLBUG),
            curled: false,
            speed: 0,
        }
    }

    pub fn update(&mut self, map: &mut Map) -> bool {
        let will_move = if self.speed == 1 {
            self.speed = 0;
            true
        } else {
            self.speed = 1;
            false
        };

        if will_move {
            // eat the food?
            if let Some(_food) = map
                .get_cell_mut(self.insect.pos)
                .unwrap()
                .take_type(CellType::Food)
            {
                self.insect.hunger = u8::MAX;
            }
        }

        let perception = self.insect.perception(map);
        let mut best_pos = Some(self.insect.move_random());
        for percep in &perception {
            if percep.1.faction != FACTION_PILLBUG && percep.1.faction != FACTION_NONE {
                // scary!
                self.curled = true;
                best_pos = None;
                break;
            }
            if percep.1.cell_type == CellType::Food {
                // food!
                best_pos = Some(percep.0);
            }
        }

        // don't die if we're curled, TBD if this is OP
        self.insect.update(
            if will_move { best_pos } else { None },
            map,
            FACTION_PILLBUG,
        ) || self.curled
    }
}
