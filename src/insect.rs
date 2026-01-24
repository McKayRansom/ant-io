use macroquad::prelude::rand;

use crate::{
    map::{Faction, Map, OccupyError, Sight},
    pos::{
        Pos,
        dirs::{self, invert, rotate_left, rotate_right},
    },
};

pub mod ant;
pub mod egg;
pub mod pillbug;
pub mod player;
pub mod spider;

// #[derive(Debug, Clone, Copy)]
// pub struct Perception {
// raw: [(Pos, u8); 5],
// }

pub type Perception = [(Pos, Sight); 5];

pub type Hunger = u16;
pub type Health = u16;

pub struct Interact {
    pub src: Id,
    pub dst: Id,
    pub amt: i8, // for now: negative is attack, positive is feed
}

impl Interact {
    pub fn fight(src: Id, dst: Id, amt: u8) -> Self {
        Self {
            src,
            dst,
            amt: -(amt as i8),
        }
    }
    pub fn feed(src: Id, dst: Id, amt: u8) -> Self {
        Self {
            src,
            dst,
            amt: amt as i8,
        }
    }
}

pub enum Event {
    Birth(Box<Insect>),
    Rebirth(Box<Insect>),
    Death(),
    Interact(Interact),
}

pub enum Action {
    Move(Pos),
    ActionA,
    ActionB,
}

pub trait InsectBehaviour {
    fn update(&mut self, base: &mut BaseInsect, map: &mut Map) -> Option<Event>;
    fn player_action(
        &mut self,
        base: &mut BaseInsect,
        map: &mut Map,
        action: Action,
    ) -> Option<Event>;
    // fn sprite(&mut self) -> Sprite // can change easily...
    // fn attacked(&mut self, strength: u8);
}

// pub const DE

pub type Id = u32;

/// Base class-ish for different insect types
#[derive(Debug, Clone)]
pub struct BaseInsect {
    pub pos: Pos,
    pub dir: Pos,
    pub hunger: Hunger,
    pub health: Health,
    pub faction: Faction,
    pub id: Id,
}

pub struct Insect {
    pub base: BaseInsect,
    pub spec: Box<dyn InsectBehaviour>,
}

impl Insect {
    pub fn update(&mut self, map: &mut Map) -> Option<Event> {
        self.spec.update(&mut self.base, map)
    }
    pub fn player_action(&mut self, map: &mut Map, action: Action) -> Option<Event> {
        self.spec.player_action(&mut self.base, map, action)
    }
    pub fn interact(&mut self, interact: Interact) {
        if interact.amt < 0 {
            // TODO: This is way too simple, have spec respond?
            self.base.health = self.base.health.saturating_sub((-interact.amt) as Health);
        } else {
            self.base.hunger = self.base.hunger.saturating_add(interact.amt as Hunger);
        }
    }
}

impl BaseInsect {
    pub fn new(pos: Pos, faction: Faction) -> Self {
        Self {
            pos,
            dir: dirs::rand(),
            hunger: rand::gen_range(u8::MAX as u16 / 2, u8::MAX as u16),
            health: 3,
            faction,
            id: 0,
        }
    }

    pub fn sight(pos: Pos, map: &Map) -> (Pos, Sight) {
        (pos, map.sight(pos))
    }

    pub fn perception(&self, map: &Map) -> Perception {
        [
            Self::sight(self.pos + rotate_left(rotate_left(self.dir)), map),
            Self::sight(self.pos + rotate_left(self.dir), map),
            Self::sight(self.pos + self.dir, map),
            Self::sight(self.pos + rotate_right(self.dir), map),
            Self::sight(self.pos + rotate_right(rotate_right(self.dir)), map),
        ]
    }

    pub fn move_random(&self) -> Pos {
        // 50/50 chance to turn
        let val = rand::rand();
        if val < u32::MAX / 2 {
            // 50/50 chance of dir
            if val < u32::MAX / 4 {
                self.pos + dirs::rotate_left(self.dir)
            } else {
                self.pos + dirs::rotate_right(self.dir)
            }
        } else {
            self.pos + self.dir
        }
    }

    pub fn will_move(speed: &mut u8, max_speed: u8) -> bool {
        if *speed == 0 {
            *speed = max_speed;
            true
        } else {
            *speed -= 1;
            false
        }
    }

    pub fn update_reproduce(
        &mut self,
        reproduce_cooldown: &mut u8,
        reproduce_time: u8,
        reproduce_cost: u16,
    ) -> bool {
        *reproduce_cooldown = reproduce_cooldown.saturating_add(1);
        // TODO: save some hunger so we don't starve
        if *reproduce_cooldown >= reproduce_time && self.hunger > reproduce_cost {
            *reproduce_cooldown = 0;
            self.hunger -= reproduce_cost;
            true
        } else {
            false
        }
    }

    pub fn try_move(&mut self, next_pos: Pos, map: &mut Map) -> Option<Event> {
        // FOR NOW: Only expend hunger when mooving
        // check if we die of hunger
        self.hunger = self.hunger.saturating_sub(1);
        if self.hunger == 0 {
            // we die
            return Some(Event::Death());
        }
        match map.occupy(next_pos, (self.faction, self.id)) {
            Ok(_) => {
                let _ = map.free(self.pos, (self.faction, self.id));
                self.dir = next_pos - self.pos;
                self.pos = next_pos;
                // self.occupy = occupy;
            }
            Err(OccupyError::Solid) => self.dir = invert(self.dir),
            Err(OccupyError::Fight(id)) => {
                // TODO: REAL FIGHTS
                // map.occupy() should have marked opponent as occupied, so they should die too
                // we die now
                return Some(Event::Interact(Interact::fight(self.id, id, 1)));
            }
        }

        None
    }
}
