
use macroquad::color::colors;
use macroquad::prelude::rand;

use crate::map::{Map, Scents};

use crate::pos::dirs::{invert, rotate_left, rotate_right};
use crate::pos::{Pos, dirs};

// const

type Food = usize;

#[derive(Debug)]
pub struct AntColony {
    pub food: Food,
    pub ants: Vec<Ant>,
}

const STARTING_FOOD: Food = 32;
const ANTS_NUMBER: usize = 32;

impl AntColony {
    pub fn new(pos: Pos, map: &mut Map) -> Self {
        map.get_cell_mut(pos).unwrap().set_flag(Scents::Nest);

        Self {
            food: STARTING_FOOD,
            ants: vec![Ant::new(pos); ANTS_NUMBER],
        }
    }

    pub fn update(&mut self, grid: &mut Map) {
        // let mut all_snakes_dead = true;
        for (_i, ant) in self.ants.iter_mut().enumerate() {
            ant.update(grid, &mut self.food);
            //     if i == 0 {
            //         // player died
            //         self.game_over = true;
            //         self.game_won = false;
            //     }
            // } else if i != 0 {
            //     all_snakes_dead = false;
            // }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub pos: Pos,
    pub food: Option<()>,
    nest_scent: u8,
    food_scent: u8,
    dir: Pos,
}

impl Ant {
    pub fn new(pos: Pos) -> Self {
        Self {
            pos,
            dir: dirs::rand(),
            food: None,
            food_scent: 0,
            nest_scent: u8::MAX,
        }
    }

    fn follow_scent(&self, grid: &Map, scent: Scents) -> Option<Pos> {
        let (_max_scent, pos) = [
            rotate_left(rotate_left(self.dir)),
            rotate_left(self.dir),
            self.dir,
            rotate_right(self.dir),
            rotate_right(rotate_right(self.dir)),
        ]
        .iter()
        .fold((0, self.pos), |max, dir| {
            if let Some(scent) = grid.get_cell(self.pos + *dir).map(|cell| {
                if cell.has_flag(scent) {
                    u8::MAX
                } else {
                    cell.get_scent(scent)
                }
            }) {
                if scent > max.0 {
                    (scent, self.pos + *dir)
                } else {
                    max
                }
            } else {
                max
            }
        });
        if pos != self.pos { Some(pos) } else { None }
    }

    pub fn update(&mut self, map: &mut Map, food: &mut Food) {
        // take food
        let cell = map.get_cell_mut(self.pos).expect("Ant in invalid pos");

        let mut next_pos = if self.food.is_some() {
            // find nest!
            if cell.has_flag(Scents::Nest) {
                *food += 1;
                self.food = None;
                self.nest_scent = u8::MAX;
                self.dir = invert(self.dir);
                return;
            } else {
                // mark food scent
                cell.drop_scent(Scents::Food, self.food_scent);
                self.food_scent = self.food_scent.saturating_sub(2);

                self.follow_scent(map, Scents::Nest)
            }
        } else {
            // find food!
            if let Some(food) = cell.take_flag(Scents::Food) {
                self.food = Some(food);
                self.food_scent = u8::MAX;
                self.dir = invert(self.dir);
                return;
            } else {
                // mark nest scent
                cell.drop_scent(Scents::Nest, self.nest_scent);
                self.nest_scent = self.nest_scent.saturating_sub(1);

                self.follow_scent(map, Scents::Food)
            }
        };

        // 25% chance of ignoring best pheremone dir...
        if next_pos.is_none() || rand::rand() < u32::MAX / 4 {
            // move random, but prefer same dir
            next_pos = Some(
                self.pos
                    + if rand::rand() < u32::MAX / 2 {
                        if rand::rand() < u32::MAX / 2 {
                            rotate_right(self.dir)
                        } else {
                            rotate_left(self.dir)
                        }
                    } else {
                        self.dir
                    },
            );
        }

        let next_pos = next_pos.unwrap();
        if map.is_valid(next_pos) {
            self.dir = next_pos - self.pos;
            self.pos = next_pos;
        } else {
            self.dir = invert(self.dir);
        }

        // self.body.push_front(self.pos);

        // let new_head = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);

        // if grid.occupy(new_head) {
        //     if is_ai {
        //         let dirs = if rand::RandomRange::gen_range(0, 2) == 0 {
        //             DIRS
        //         } else {
        //             DIRS_REV
        //         };
        //         for dir in dirs {
        //             let new_head = (self.pos.0 + dir.0, self.pos.1 + dir.1);
        //             if !grid.occupy(new_head) {
        //                 self.pos = new_head;
        //                 self.dir = *dir;
        //                 return false;
        //             }
        //         }
        //     }
        //     return true;
        // }

        // self.pos = new_head;
        // false
    }

    pub fn draw(&self, grid: &Map) {
        grid.draw_cell(
            self.pos,
            if self.food.is_none() {
                colors::BLUE
            } else {
                colors::SKYBLUE
            },
        );

        // for pos in &self.body {
        //     grid.draw_cell(*pos, self.body_color);
        // }
    }
}
