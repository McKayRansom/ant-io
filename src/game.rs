
use macroquad::color::colors;
use macroquad::prelude::*;

use crate::ant::{Ant, Nest};
use crate::grid::{Grid, Scents};
// use crate::grid::{DOWN, Grid, LEFT, RIGHT, SQUARES, UP};
use crate::pos::{self, Pos};

pub struct Game {
    grid: Grid,

    ants: Vec<Ant>,

    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
    pub game_won: bool,
    nest: Nest,
}

const ANTS_NUMBER: usize = 64;
const NEST_POS: Pos = Pos::new(40, 70);

impl Game {
    pub fn new() -> Self {
        let mut grid = Grid::new();
        grid.get_cell_mut(NEST_POS).unwrap().set_flag(Scents::Nest);

        let mut food_pos: Pos = Pos::new(10, 20);

        for _ in 0..50 {
            grid.get_cell_mut(food_pos).unwrap().set_flag(Scents::Food);
            let new_pos = food_pos + pos::dirs::rand();
            if grid.is_valid(new_pos) {
                food_pos = new_pos;
            }
        }

        Self {
            grid,
            ants: vec![Ant::new(NEST_POS); ANTS_NUMBER],

            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.15,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            game_won: true,
            nest: Nest::default(),
        }
    }

    pub fn update(&mut self, _won: u32, _lost: u32) -> bool {
        if !self.game_over {
            // if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
            //     && self.ants[0].dir != LEFT
            //     && !self.navigation_lock
            // {
            //     self.ants[0].dir = RIGHT;
            //     self.navigation_lock = true;
            // } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
            //     && self.ants[0].dir != RIGHT
            //     && !self.navigation_lock
            // {
            //     self.ants[0].dir = LEFT;
            //     self.navigation_lock = true;
            // } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W))
            //     && self.ants[0].dir != DOWN
            //     && !self.navigation_lock
            // {
            //     self.ants[0].dir = UP;
            //     self.navigation_lock = true;
            // } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
            //     && self.ants[0].dir != UP
            //     && !self.navigation_lock
            // {
            //     self.ants[0].dir = DOWN;
            //     self.navigation_lock = true;
            // }

            if get_time() - self.last_update > self.speed {
                self.last_update = get_time();

                // let mut all_snakes_dead = true;
                for (_i, ant) in self.ants.iter_mut().enumerate() {
                    ant.update(&mut self.grid, &mut self.nest);
                    //     if i == 0 {
                    //         // player died
                    //         self.game_over = true;
                    //         self.game_won = false;
                    //     }
                    // } else if i != 0 {
                    //     all_snakes_dead = false;
                    // }
                }
                self.grid.update();
                // if all_snakes_dead {
                //     self.game_won = true;
                //     self.game_over = true;
                // }
                self.navigation_lock = false;
            }
        }

        // let mut player_color = self.ants[0].body_color;
        // player_color.r *= 0.5;
        // player_color.g *= 0.5;
        // player_color.b *= 0.5;

        clear_background(colors::BLACK);

        self.grid.update_size();
        self.grid.draw();

        for ants in &self.ants {
            ants.draw(&self.grid);
        }

        draw_text(
            format!("Food: {}", self.nest.food).as_str(),
            10.,
            20.,
            20.,
            WHITE,
        );

        // if self.game_over {
        //     // clear_background(BLACK);
        //     let text = if self.game_won {
        //         "Game Won! Press [enter] to play agin."
        //     } else {
        //         "Game Over. Press [enter] to play again."
        //     };
        //     let font_size = 30.;
        //     let text_size = measure_text(text, None, font_size as _, 1.0);

        //     draw_text(
        //         text,
        //         screen_width() / 2. - text_size.width / 2.,
        //         screen_height() / 2. + text_size.height / 2.,
        //         font_size,
        //         WHITE,
        //     );

        //     if is_key_down(KeyCode::Enter) {
        //         // start new game
        //         return true;
        //     }
        // }
        false
    }
}
