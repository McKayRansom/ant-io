use macroquad::prelude::rand;

use crate::{
    insect::Insect,
    map::{CellType, FACTION_NONE, FACTION_PILLBUG, Map},
    pos::Pos,
};

pub struct Pillbug {
    pub insect: Insect,
    pub curled: bool,
    pub speed: u8,
    pub reproduce: u8,
}

impl Pillbug {
    pub fn new(pos: Pos) -> Self {
        Self {
            insect: Insect::new(pos, FACTION_PILLBUG),
            curled: false,
            speed: 0,
            reproduce: rand::gen_range(0, u8::MAX / 4),
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
        self.reproduce += 1;
        if self.reproduce == u8::MAX {
            self.reproduce = 0;
            new_bugs.push(self.insect.pos);
        }

        self.insect.hunger = self.insect.hunger.saturating_sub(1);

        if will_move {
            // eat the food?
            if self.insect.hunger < u8::MAX / 2 {
                if let Some(_food) = map
                    .get_cell_mut(self.insect.pos)
                    .unwrap()
                    .take_type(CellType::Food)
                {
                    self.insect.hunger = u8::MAX;
                }
            } else if map.get_cell(self.insect.pos).unwrap().is_type(CellType::Food) {
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
