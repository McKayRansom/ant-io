use macroquad::{
    color::{Color, colors},
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

use crate::pos::Pos;

#[derive(Debug, Clone, Copy)]
pub enum Scents {
    Food,
    Nest,
    Len,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    flags: u16,
    scents: [u8; Scents::Len as usize],
}

impl Cell {
    pub fn has_flag(&self, flag: Scents) -> bool {
        self.flags & 1 << flag as usize != 0
    }
    pub fn set_flag(&mut self, flag: Scents) {
        self.flags |= 1 << flag as usize;
    }
    pub fn clear_flag(&mut self, flag: Scents) {
        self.flags &= !(1 << flag as usize);
    }

    pub fn take_flag(&mut self, flag: Scents) -> Option<()> {
        if self.has_flag(flag) {
            self.clear_flag(flag);
            Some(())
        } else {
            None
        }
    }
    pub fn get_scent(&self, scent: Scents) -> u8 {
        self.scents[scent as usize]
    }
    pub fn drop_scent(&mut self, scent: Scents, val: u8) {
        self.scents[scent as usize] = self.scents[scent as usize].max(val)
    }

    fn update(&mut self) {
        // self.nest_scent = self.nest_scent.saturating_sub(1);
        self.scents[Scents::Food as usize] = self.scents[Scents::Food as usize].saturating_sub(1);
    }
}

pub const SQUARES: i16 = 64;

pub struct Map {
    occupied: Vec<Vec<Cell>>,
    pub size: Pos,

    game_size: f32,
    offset_x: f32,
    offset_y: f32,
    sq_size: f32,
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

    pub fn update(&mut self) {
        for row in self.occupied.iter_mut() {
            for cell in row.iter_mut() {
                cell.update();
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.offset_x,
            self.offset_y,
            self.game_size - 20.,
            self.game_size - 20.,
            colors::BLACK,
        );

        for y in 0..self.occupied.len() {
            let row = &self.occupied[y];
            for x in 0..row.len() {
                let point: Pos = Pos::new(x as i16, y as i16);
                let cell = &row[x];
                if cell.has_flag(Scents::Food) {
                    self.draw_cell(point, colors::GREEN);
                } else if cell.has_flag(Scents::Nest) {
                    self.draw_cell(point, colors::WHITE);
                } 
                // draw scents
                // else if cell.get_scent(Scents::Food) > 0 {
                //     let scent_alpha = cell.get_scent(Scents::Food) as f32 / u8::MAX as f32;
                //     let mut color = colors::RED;
                //     color.a = scent_alpha;
                //     self.draw_cell(point, color);
                // } else if cell.get_scent(Scents::Nest) > 0 {
                //     let scent_alpha = cell.get_scent(Scents::Nest) as f32 / u8::MAX as f32;
                //     let mut color = colors::LIGHTGRAY;
                //     color.a = scent_alpha;
                //     self.draw_cell(point, color);
                // }
            }
        }
    }

    pub fn draw_cell(&self, pos: Pos, color: Color) {
        draw_rectangle(
            self.offset_x + pos.x as f32 * self.sq_size,
            self.offset_y + pos.y as f32 * self.sq_size,
            self.sq_size,
            self.sq_size,
            color,
        );
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
