use std::fmt::Display;

use macroquad::color::colors;
use macroquad::prelude::*;

use crate::draw::{PILLBUG_COLOR, SPIDER_COLOR, color, draw_game};
use crate::insect::ant::{Ant, AntColony};
use crate::insect::pillbug::Pillbug;
use crate::insect::spider::Spider;
use crate::map::{Faction, Map, FACTION_SPIDER, MAP_SIZE};
// use crate::grid::{DOWN, Grid, LEFT, RIGHT, SQUARES, UP};
use crate::pos::{Pos, dirs};

pub enum Speed {
    SLOW,
    FAST,
}

const SPEED_FAST: f64 = 0.1;
const SPEED_SLOW: f64 = 0.25;

impl Speed {
    pub fn val(&self) -> f64 {
        match self {
            Speed::SLOW => SPEED_FAST,
            Speed::FAST => SPEED_SLOW,
        }
    }

    fn invert(&self) -> Speed {
        match self {
            Speed::SLOW => Speed::FAST,
            Speed::FAST => Speed::SLOW,
        }
    }
}

pub struct Game {
    pub map: Map,

    speed: Speed,
    last_update: f64,
    game_over: bool,
    pub game_won: bool,
    pub ant_colonies: Vec<AntColony>,
    pub player: Spider,
    pub player_dir: Pos,
    pub show_scents: Faction,
    pub paused: bool,
    pub pillbugs: Vec<Pillbug>,
    pub spiders: Vec<Spider>,
}

const NEST_POS: Pos = Pos::new(MAP_SIZE - 20, MAP_SIZE - 10);
const NEST_POS_2: Pos = Pos::new(20, 10);

const STARTING_PILLBUGS: usize = 250;
const STARTING_SPIDERS: usize = 50;

impl Game {
    pub fn new() -> Self {
        let mut map = Map::new();

        Self {
            ant_colonies: vec![
                AntColony::new(NEST_POS, &mut map, 1),
                AntColony::new(NEST_POS_2, &mut map, 2),
            ],
            pillbugs: (0..STARTING_PILLBUGS)
                .into_iter()
                .map(|_| Pillbug::new(map.rand_pos()))
                .collect(),
            spiders: (0..STARTING_SPIDERS)
                .into_iter()
                .map(|_| Spider::new(map.rand_pos()))
                .collect(),
            player: Spider::new(map.rand_pos()),
            player_dir: dirs::NONE,
            map,
            speed: Speed::SLOW,
            last_update: 0.,
            game_over: false,
            game_won: true,
            show_scents: 0,
            paused: false,
        }
    }

    /// NOTE: will be run more than once per sim tick!! must handle this correctly
    pub fn update_player_input(&mut self) {
        let mut input_dir = dirs::NONE;
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            input_dir.x = 1;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            input_dir.x = -1;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            input_dir.y = -1;
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            input_dir.y = 1;
        }
        self.player_dir = input_dir;

        // if is_key_down(KeyCode::LeftShift) {
        //     self.player.food_scent = u8::MAX;
        // } else {
        //     self.player.food_scent = 0;
        // }
        if is_key_down(KeyCode::X) {
            self.player.reproduce = u8::MAX;
            // self.player.insect.hunger = 550;
        } else {
            self.player.reproduce = 0;
        }
        if is_key_pressed(KeyCode::Key1) {
            if self.show_scents == 1 {
                self.show_scents = 0;
            } else {
                self.show_scents = 1;
            }
        }
        if is_key_pressed(KeyCode::Key2) {
            if self.show_scents == 2 {
                self.show_scents = 0;
            } else {
                self.show_scents = 2;
            }
        }

        if is_key_pressed(KeyCode::Minus) {
            self.map.camera.change_zoom(-0.1);
        }
        if is_key_pressed(KeyCode::Equal) {
            self.map.camera.change_zoom(0.1);
        }

        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }
        if is_key_pressed(KeyCode::F) {
            self.speed = self.speed.invert();
        }
    }

    pub fn update_player(&mut self, new_bugs: &mut Vec<Pos>) -> bool {

        let will_move = self.player.speed == 1;
        let old_speed = self.player.speed;
        self.player.speed = 1;
        let res = self.player.update(&mut self.map, new_bugs);
        if old_speed == 0 {
            self.player.speed = 1;
        } else {
            self.player.speed = 0
        }
        if will_move && self.player_dir != dirs::NONE {
            res && self.player.insect.update(Some(self.player.insect.pos + self.player_dir), &mut self.map, FACTION_SPIDER)
        } else {
            res
        }

        // let colony = &mut self.ant_colonies[0];

        // let scent = self.player.food_scent;

        // let _seeking = self
        //     .player
        //     .update_behaviour(&mut self.map, &mut colony.food, 1);

        // self.player.update_scents(&mut colony.scents);

        // self.player.food_scent = scent;
        // self.player.insect.hunger = u16::MAX;

        // if self.player.insect.update(
        //     Some(self.player.insect.pos + self.player.insect.dir),
        //     &mut self.map,
        //     self.ant_colonies[0].faction,
        // ) == false
        // {
        //     // we dead, make new player
        //     self.player = Ant::new(NEST_POS, 1);
        // }
    }

    pub fn update_sim(&mut self) {
        let mut new_bugs: Vec<Pos> = Vec::new();
        self.spiders
            .retain_mut(|spider| spider.update(&mut self.map, &mut new_bugs));

        if !self.update_player(&mut new_bugs) {
            // find a new spider for the player
            self.player = Spider::new(self.map.rand_pos());
        }

        // this is stupid but IDK
        for pos in new_bugs.iter() {
            self.spiders.push(Spider::new(*pos));
        }

        for colony in self.ant_colonies.iter_mut() {
            colony.update(&mut self.map);
        }

        let mut new_bugs: Vec<Pos> = Vec::new();
        self.pillbugs
            .retain_mut(|bug| bug.update(&mut self.map, &mut new_bugs));
        // this is stupid but IDK
        for pos in new_bugs.iter() {
            self.pillbugs.push(Pillbug::new(*pos));
        }

        self.map.update();
    }

    pub fn update(&mut self, _won: u32, _lost: u32) -> bool {
        self.update_player_input();
        if !self.game_over {
            if !self.paused && get_time() - self.last_update > self.speed.val() {
                self.last_update = get_time();

                self.update_sim();

                // if all_snakes_dead {
                //     self.game_won = true;
                //     self.game_over = true;
                // }

                // self.update_player();
            }
        }

        // let mut player_color = self.ants[0].body_color;
        // player_color.r *= 0.5;
        // player_color.g *= 0.5;
        // player_color.b *= 0.5;

        clear_background(colors::BLACK);

        self.map.update_size(self.player.insect.pos);
        draw_game(&self);

        draw_text(
            format!(
                "Colony 0: Pop: {} Food: {}",
                self.ant_colonies[0].workers.len(),
                self.ant_colonies[0].food
            )
            .as_str(),
            10.,
            20.,
            24.,
            color(self.ant_colonies[0].faction),
        );

        draw_text(
            format!(
                "Colony 1: Pop: {} Food: {}",
                self.ant_colonies[1].workers.len(),
                self.ant_colonies[1].food
            )
            .as_str(),
            10.,
            40.,
            24.,
            color(self.ant_colonies[1].faction),
        );

        draw_text(
            format!("Pillbugs: {}", self.pillbugs.len()).as_str(),
            10.,
            60.,
            24.,
            PILLBUG_COLOR,
        );

        draw_text(
            format!("Spiders: {}", self.spiders.len()).as_str(),
            10.,
            80.,
            24.,
            SPIDER_COLOR,
        );

        draw_text(
            format!("P Hunger: {}", self.player.insect.hunger / 500).as_str(),
            10.,
            100.,
            24.,
            colors::YELLOW,
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

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ant 0: Pop: {} \nAnt 1: Pop: {} \nPillbugs: {}\nSpiders: {}",
            self.ant_colonies[0].workers.len(),
            self.ant_colonies[1].workers.len(),
            self.pillbugs.len(),
            self.spiders.len()
        )
        // writ("Ants[0]: {}", self.ant_colonies[0].workers.len())
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn test_pops() {
        let mut game: Game = Game::new();

        for _ in 0..4 {
            for i in 0..4096 {
                game.update_sim();
                assert!(
                    !game.pillbugs.is_empty(),
                    "Pillbugs died out at gen {}\n{}",
                    i,
                    game
                );
                assert!(
                    game.pillbugs.len() < 1024,
                    "Pillbugs overpoped at gen {}\n{}",
                    i,
                    game
                );

                assert!(
                    !game.spiders.is_empty(),
                    "Spiders died out at gen {}\n{}",
                    i,
                    game
                );
                assert!(
                    game.spiders.len() < 500,
                    "Spiders overpoped at gen {}\n{}",
                    i,
                    game
                );

                assert!(
                    !game.ant_colonies[0].workers.is_empty(),
                    "Ants[0] died out at gen {}\n{}",
                    i,
                    game
                );
                assert!(
                    game.ant_colonies[0].workers.len() < 500,
                    "Ants[0] overpoped at gen {}\n{}",
                    i,
                    game
                );

                assert!(
                    !game.ant_colonies[1].workers.is_empty(),
                    "Ants[1] died out at gen {}\n{}",
                    i,
                    game
                );
                assert!(
                    game.ant_colonies[1].workers.len() < 500,
                    "Ants[1] overpoped at gen {}\n{}",
                    i,
                    game
                );
            }

            println!("{}", game);
        }
        assert!(false, "PASSED");
    }
}
