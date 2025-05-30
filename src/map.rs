use std::rc::{Rc, Weak};

use macroquad::window::{screen_height, screen_width};

use crate::pos::Pos;

pub type Faction = u8;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CellType {
    #[default]
    Empty,
    Food,
    Nest(Faction),
    // Rock,
}

#[derive(Debug, Default, Clone)]
pub struct Cell {
    pub m_type: CellType,

    // not so sure about this one fam
    occupied: Weak<Faction>,
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

    pub fn _is_occupied(&self) -> bool {
        self.occupied.strong_count() > 0
    }

    pub fn _occupied_faction(&self) -> Option<Faction> {
        Weak::<Faction>::upgrade(&self.occupied).map(|rc| *rc)
    }

    pub fn try_occupy(&mut self, faction: Faction) -> Option<Rc<Faction>> {
        if let Some(rc) = self.occupied.upgrade() {
            // occupied
            if *rc == faction { Some(rc) } else { None }
        } else {
            // empty
            let rc = Rc::new(faction);
            self.occupied = Rc::<Faction>::downgrade(&rc);
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
        map
    }

    pub fn update_size(&mut self) {
        self.game_size = screen_width().min(screen_height());
        self.offset_x = (screen_width() - self.game_size) / 2. + 10.;
        self.offset_y = (screen_height() - self.game_size) / 2. + 10.;
        self.sq_size = (screen_height() - self.offset_y * 2.) / SQUARES as f32;
    }

    pub fn update(&mut self) {}

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
    ) -> Result<Rc<Faction>, OccupyError> {
        let cell = self.get_cell_mut(next_pos).ok_or(OccupyError::Solid)?;
        cell.try_occupy(faction).ok_or(OccupyError::Fight)
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
