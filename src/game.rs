use macroquad::color::colors;
use macroquad::{prelude::*, rand};

use crate::ant::AntColony;
use crate::map::{Map, Scents};
// use crate::grid::{DOWN, Grid, LEFT, RIGHT, SQUARES, UP};
use crate::pos::{self, Pos};

pub struct Game {
    map: Map,

    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
    pub game_won: bool,
    nest: AntColony,
}

const NEST_POS: Pos = Pos::new(40, 60);

fn drop_food_bunch(map: &mut Map) {
    let mut food_pos = (
        rand::gen_range(0, map.size.x),
        rand::gen_range(0, map.size.y),
    )
        .into();
    for _ in 0..rand::gen_range(10, 70) {
        let Some(cell) = map.get_cell_mut(food_pos) else {
            continue;
        };
        cell.set_flag(Scents::Food);

        let new_pos = food_pos + pos::dirs::rand();
        if map.is_valid(new_pos) {
            food_pos = new_pos;
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let mut map = Map::new();

        drop_food_bunch(&mut map);

        Self {
            nest: AntColony::new(NEST_POS, &mut map),
            map,
            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.5,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            game_won: true,
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

                self.nest.update(&mut self.map);

                self.map.update();
                // if all_snakes_dead {
                //     self.game_won = true;
                //     self.game_over = true;
                // }
                self.navigation_lock = false;

                if rand::gen_range(0, 50) == 0 {
                    drop_food_bunch(&mut self.map);
                }
            }
        }

        // let mut player_color = self.ants[0].body_color;
        // player_color.r *= 0.5;
        // player_color.g *= 0.5;
        // player_color.b *= 0.5;

        clear_background(colors::BLACK);

        self.map.update_size();
        self.map.draw();

        for ants in &self.nest.ants {
            ants.draw(&self.map);
        }

        draw_text(
            format!("Pop: {}", self.nest.ants.len()).as_str(),
            10.,
            20.,
            24.,
            WHITE,
        );

        draw_text(
            format!("Food: {}", self.nest.food).as_str(),
            10.,
            40.,
            24.,
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
