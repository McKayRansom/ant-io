use macroquad::color::colors;
use macroquad::{prelude::*, rand};

use crate::ant::{Ant, AntColony};
use crate::draw::{color, draw_colony, draw_map, draw_player};
use crate::map::Map;
// use crate::grid::{DOWN, Grid, LEFT, RIGHT, SQUARES, UP};
use crate::pos::{self, Pos, dirs};

pub struct Game {
    map: Map,

    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
    pub game_won: bool,
    ant_colonies: Vec<AntColony>,
    player: Ant,
}

const NEST_POS: Pos = Pos::new(40, 60);
const NEST_POS_2: Pos = Pos::new(20, 10);

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
        cell.set_type(crate::map::CellType::Food);

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
            ant_colonies: vec![
                AntColony::new(NEST_POS, &mut map, 1),
                AntColony::new(NEST_POS_2, &mut map, 2),
            ],
            map,
            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.1,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            game_won: true,
            player: Ant::new(NEST_POS, 1),
        }
    }

    pub fn update_player(&mut self) {
        self.player.dir = if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dirs::RIGHT
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dirs::LEFT
        } else if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dirs::UP
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dirs::DOWN
        } else {
            self.player.dir
        };

        let colony = &mut self.ant_colonies[0];
        let _ = self.player.update_food_scents(
            &mut self.map,
            &mut colony.scents,
            &mut colony.food,
            colony.faction,
        );
        if self.player.try_move(
            self.player.pos + self.player.dir,
            &mut self.map,
            self.ant_colonies[0].faction,
        ) == false
        {
            // we dead, make new player
            self.player = Ant::new(NEST_POS, 1);
        }
    }

    pub fn update(&mut self, _won: u32, _lost: u32) -> bool {
        if !self.game_over {
            if get_time() - self.last_update > self.speed {
                self.last_update = get_time();

                for colony in self.ant_colonies.iter_mut() {
                    colony.update(&mut self.map);
                }

                self.map.update();
                // if all_snakes_dead {
                //     self.game_won = true;
                //     self.game_over = true;
                // }
                self.navigation_lock = false;

                if rand::gen_range(0, 50) == 0 {
                    drop_food_bunch(&mut self.map);
                }

                self.update_player();
            }
        }

        // let mut player_color = self.ants[0].body_color;
        // player_color.r *= 0.5;
        // player_color.g *= 0.5;
        // player_color.b *= 0.5;

        clear_background(colors::BLACK);

        self.map.update_size();
        draw_map(&self.map);

        for colony in &self.ant_colonies {
            draw_colony(colony, &self.map);
        }

        draw_player(&self.player, &self.map);

        draw_text(
            format!("Pop: {}", self.ant_colonies[0].ants.len()).as_str(),
            10.,
            20.,
            24.,
            color(self.ant_colonies[0].faction),
        );

        draw_text(
            format!("Food: {}", self.ant_colonies[0].food).as_str(),
            10.,
            40.,
            24.,
            color(self.ant_colonies[0].faction),
        );

        draw_text(
            format!("Pop: {}", self.ant_colonies[1].ants.len()).as_str(),
            10.,
            60.,
            24.,
            color(self.ant_colonies[1].faction),
        );

        draw_text(
            format!("Food: {}", self.ant_colonies[1].food).as_str(),
            10.,
            80.,
            24.,
            color(self.ant_colonies[1].faction),
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
