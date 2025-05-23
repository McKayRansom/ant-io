use std::u8;

use macroquad::{
    color::{Color, colors},
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

use crate::pos::Pos;

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    food: bool,
    nest: bool,
    food_scent: u8,
    nest_scent: u8,
}

impl Cell {
    pub fn is_food(&self) -> bool {
        self.food
    }
    pub fn is_nest(&self) -> bool {
        self.nest
    }

    pub fn take_food(&mut self) -> Option<()> {
        if self.food {
            self.food = false;
            Some(())
        } else {
            None
        }
    }
    pub fn get_food_scent(&self) -> u8 {
        self.food_scent
    }
    pub fn get_nest_scent(&self) -> u8 {
        self.nest_scent
    }
    pub fn set_food_scent(&mut self, scent: u8) {
        self.food_scent = self.food_scent.max(scent);
    }
    pub fn set_nest_scent(&mut self, scent: u8) {
        self.nest_scent = self.nest_scent.max(scent);
    }

    fn update(&mut self) {
        // self.nest_scent = self.nest_scent.saturating_sub(1);
        self.food_scent = self.food_scent.saturating_sub(1);
    }

    pub(crate) fn set_nest(&mut self, arg: bool) {
        self.nest = arg;
    }

    pub(crate) fn set_food(&mut self, arg: bool) {
        self.food = arg;
    }
}

pub const SQUARES: i16 = 80;

pub struct Grid {
    occupied: Vec<Vec<Cell>>,
    size: Pos,

    game_size: f32,
    offset_x: f32,
    offset_y: f32,
    sq_size: f32,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid: Grid = Self {
            occupied: vec![vec![Cell::default(); SQUARES as usize]; SQUARES as usize],
            size: Pos::new(SQUARES, SQUARES),
            game_size: 0.,
            offset_x: 0.,
            offset_y: 0.,
            sq_size: 0.,
        };
        grid.update_size();
        grid
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
                if cell.is_food() {
                    self.draw_cell(point, colors::GREEN);
                } else if cell.is_nest() {
                    self.draw_cell(point, colors::WHITE);
                } else if cell.food_scent > 0 {
                    let scent_alpha = cell.food_scent as f32 / u8::MAX as f32;
                    let mut color = colors::RED;
                    color.a = scent_alpha;
                    self.draw_cell(point, color);
                } else if cell.nest_scent > 0 {
                    let scent_alpha = cell.nest_scent as f32 / u8::MAX as f32;
                    let mut color = colors::LIGHTGRAY;
                    color.a = scent_alpha;
                    self.draw_cell(point, color);
                }
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
