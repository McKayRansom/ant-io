use macroquad::color::colors;
use macroquad::prelude::*;

use crate::draw::{PILLBUG_COLOR, SPIDER_COLOR, color, draw_game};
use crate::insect::ant::{Ant, AntColony};
use crate::insect::pillbug::Pillbug;
use crate::insect::spider::Spider;
use crate::map::{Faction, Map};
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
    pub player: Ant,
    pub show_scents: Faction,
    pub paused: bool,
    pub pillbugs: Vec<Pillbug>,
    pub spiders: Vec<Spider>,
}

const NEST_POS: Pos = Pos::new(40 * 2, 60 * 2);
const NEST_POS_2: Pos = Pos::new(20, 10);

impl Game {
    pub fn new() -> Self {
        let mut map = Map::new();

        map.drop_rand_bunch(crate::map::CellType::Food);

        Self {
            ant_colonies: vec![
                AntColony::new(NEST_POS, &mut map, 1),
                AntColony::new(NEST_POS_2, &mut map, 2),
            ],
            pillbugs: (0..100)
                .into_iter()
                .map(|_| Pillbug::new(map.rand_pos()))
                .collect(),
            spiders: (0..25)
                .into_iter()
                .map(|_| Spider::new(map.rand_pos()))
                .collect(),
            map,
            speed: Speed::SLOW,
            last_update: 0.,
            game_over: false,
            game_won: true,
            player: Ant::new(NEST_POS, 1),
            show_scents: 0,
            paused: false,
        }
    }

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
        self.player.insect.dir = input_dir;

        if is_key_down(KeyCode::LeftShift) {
            self.player.attack_scent = u8::MAX;
        } else {
            self.player.attack_scent = 0;
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

    pub fn update_player(&mut self) {
        let colony = &mut self.ant_colonies[0];

        let _seeking = self
            .player
            .update_behaviour(&mut self.map, &mut colony.food, 1);
        self.player.update_scents(&mut colony.scents);
        self.player.insect.hunger = u16::MAX;

        if self.player.insect.update(
            Some(self.player.insect.pos + self.player.insect.dir),
            &mut self.map,
            self.ant_colonies[0].faction,
        ) == false
        {
            // we dead, make new player
            self.player = Ant::new(NEST_POS, 1);
        }
    }

    pub fn update_sim(&mut self) {
        let mut new_bugs: Vec<Pos> = Vec::new();
        self.spiders
            .retain_mut(|spider| spider.update(&mut self.map, &mut new_bugs));
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

                self.update_player();
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

        // draw_text(
        //     format!("P Hunger: {}", self.player.insect.hunger).as_str(),
        //     10.,
        //     100.,
        //     24.,
        //     colors::YELLOW,
        // );

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

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn test_pops() {
        let mut game: Game = Game::new();

        for i in 0..4096 {
            game.update_sim();
            assert!(!game.pillbugs.is_empty(), "Pillbugs died out at gen {}", i);
            assert!(game.pillbugs.len() < 500, "Pillbugs overpoped at gen {}", i);

            assert!(!game.spiders.is_empty(), "Spiders died out at gen {}", i);
            assert!(game.spiders.len() < 500, "Spiders overpoped at gen {}", i);

            assert!(!game.ant_colonies[0].workers.is_empty(), "Ants[0] died out at gen {}", i);
            assert!(game.ant_colonies[0].workers.len() < 500, "Ants[0] overpoped at gen {}", i);

            assert!(!game.ant_colonies[1].workers.is_empty(), "Ants[1] died out at gen {}", i);
            assert!(game.ant_colonies[1].workers.len() < 500, "Ants[1] overpoped at gen {}", i);
        }

        println!("Final pillbugs: {}", game.pillbugs.len());
        println!("Final spiders: {}", game.spiders.len());
        println!("Final ant[0]: {}", game.ant_colonies[0].workers.len());
        println!("Final ant[1]: {}", game.ant_colonies[1].workers.len());

        assert!(false)
    }

}