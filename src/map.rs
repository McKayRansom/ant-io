use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use macroquad::{
    prelude::rand,
    window::{screen_height, screen_width},
};

use crate::pos::{Pos, dirs};

pub type Faction = u8;

pub const FACTION_NONE: u8 = 0;

pub const FACTION_PILLBUG: u8 = u8::MAX - 1;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CellType {
    #[default]
    Empty,
    Food,
    Nest(Faction),
    Rock,
    Wall,
}

#[derive(Debug, Default, Clone)]
pub struct Cell {
    pub m_type: CellType,

    // not so sure about this one fam
    occupied: Weak<RefCell<Faction>>,
}

impl Cell {
    pub fn is_type(&self, flag: CellType) -> bool {
        self.m_type == flag
    }
    pub fn set_type(&mut self, flag: CellType) {
        if self.m_type == CellType::Empty {
            self.m_type = flag
        }
    }
    // pub fn clear_type(&mut self, _flag: CellType) {
    //     self.m_type = CellType::Empty
    // }

    pub fn take_type(&mut self, flag: CellType) -> Option<()> {
        if self.is_type(flag) {
            self.m_type = CellType::Empty;
            Some(())
        } else {
            None
        }
    }

    // pub fn _is_occupied(&self) -> bool {
    //     self.occupied.strong_count() > 0
    // }

    pub fn occupied_faction(&self) -> Faction {
        Weak::<RefCell<Faction>>::upgrade(&self.occupied)
            .map(|rc| *rc.borrow())
            .unwrap_or(FACTION_NONE)
    }

    pub fn try_occupy(&mut self, faction: Faction) -> Option<Rc<RefCell<Faction>>> {
        if let Some(rc) = self.occupied.upgrade() {
            // occupied
            if *rc.borrow() == faction {
                Some(rc)
            } else {
                // Enemy is here! mark that
                *rc.borrow_mut() = faction;
                None
            }
        } else {
            // empty
            let rc = Rc::new(RefCell::new(faction));
            self.occupied = Rc::<RefCell<Faction>>::downgrade(&rc);
            Some(rc)
        }
    }
}

pub const SQUARES: i16 = 128;

pub struct Map {
    pub occupied: Vec<Vec<Cell>>,
    pub size: Pos,

    pub game_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub sq_size: f32,
}

pub struct Sight {
    pub cell_type: CellType,
    pub faction: Faction,
}

impl Sight {
    pub fn new(cell_type: CellType, faction: Faction) -> Self {
        Self { cell_type, faction }
    }
}

pub enum OccupyError {
    Solid,
    Fight,
}

impl Map {
    pub fn new() -> Self {
        let mut map: Map = Self {
            occupied: vec![vec![Cell::default(); SQUARES as usize]; SQUARES as usize],
            size: Pos::new(SQUARES, SQUARES),
            game_size: 0.,
            offset_x: 0.,
            offset_y: 0.,
            sq_size: 0.,
        };
        map.update_size();
        for _ in 0..10 {
            map.drop_rand_bunch(CellType::Rock);
        }
        for _ in 0..10 {
            map.drop_rand_bunch(CellType::Food);
        }
        map
    }

    pub fn rand_pos(&self) -> Pos {
        Pos::rand(self.size)
    }

    pub fn drop_rand_bunch(&mut self, t: CellType) {
        let mut pos = self.rand_pos();
        for _ in 0..rand::gen_range(10, 70) {
            let Some(cell) = self.get_cell_mut(pos) else {
                continue;
            };
            cell.set_type(t);

            let new_pos = pos + dirs::rand();
            if self.is_valid(new_pos) {
                pos = new_pos;
            }
        }
    }

    pub fn update_size(&mut self) {
        self.game_size = screen_width().min(screen_height());
        self.offset_x = (screen_width() - self.game_size) / 2. + 10.;
        self.offset_y = (screen_height() - self.game_size) / 2. + 10.;
        self.sq_size = (screen_height() - self.offset_y * 2.) / SQUARES as f32;
    }

    pub fn update(&mut self) {
        if rand::gen_range(0, 50) == 0 {
            self.drop_rand_bunch(crate::map::CellType::Food);
        }
    }

    pub fn is_valid(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y
    }

    pub fn get_cell_mut(&mut self, pos: Pos) -> Option<&mut Cell> {
        if self.is_valid(pos) {
            Some(&mut self.occupied[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    pub fn get_cell(&self, pos: Pos) -> Option<&Cell> {
        if self.is_valid(pos) {
            Some(&self.occupied[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    pub(crate) fn occupy(
        &mut self,
        next_pos: Pos,
        faction: Faction,
    ) -> Result<Rc<RefCell<Faction>>, OccupyError> {
        let cell = self.get_cell_mut(next_pos).ok_or(OccupyError::Solid)?;
        if cell.m_type == CellType::Rock {
            return Err(OccupyError::Solid);
        }
        cell.try_occupy(faction).ok_or(OccupyError::Fight)
    }

    pub fn sight(&self, pos: Pos) -> Sight {
        self.get_cell(pos)
            .map(|cell| Sight::new(cell.m_type, cell.occupied_faction()))
            .unwrap_or(Sight::new(CellType::Wall, FACTION_NONE))
    }

    // pub fn occupy(&mut self, pos: Point) -> bool {
    //     if pos.0 < 0 || pos.1 < 0 || pos.0 >= SQUARES || pos.1 >= SQUARES {
    //         return true;
    //     }

    //     if self.occupied[pos.1 as usize][pos.0 as usize] {
    //         return true;
    //     }
    //     self.occupied[pos.1 as usize][pos.0 as usize] = true;
    //     return false;
    // }
}
