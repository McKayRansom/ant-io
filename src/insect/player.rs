use crate::insect::{Insect, InsectBehaviour};

pub struct InsectPlayer {
    sub_insect: Box<dyn InsectBehaviour>,
}

impl InsectPlayer {
    pub fn new(insect: Insect) -> Insect {
        Insect {
            base: insect.base,
            spec: Box::new(Self {
                sub_insect: insect.spec,
            }),
        }
    }
}

impl InsectBehaviour for InsectPlayer {
    fn update(
        &mut self,
        _base: &mut super::BaseInsect,
        _map: &mut crate::map::Map,
    ) -> Option<super::Event> {
        // Do nothing, because we want the player to do stuff...
        None
    }

    fn player_action(
        &mut self,
        base: &mut super::BaseInsect,
        map: &mut crate::map::Map,
        action: super::Action,
    ) -> Option<super::Event> {
        match action {
            super::Action::Move(pos) => {
                // WHAAAAA
                // if BaseInsect::will_move(&mut self.speed, max_speed)
                base.try_move(base.pos + pos, map)
            }
            _ => {
                self.sub_insect.player_action(base, map, action)
            }
        }
    }
}
