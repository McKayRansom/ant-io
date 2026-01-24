use crate::{insect::{BaseInsect, Insect, InsectBehaviour}, pos::Pos};

pub struct Egg {
    germination: u8,
    hatch: Option<Box<Insect>>,
}

impl Egg {
    pub fn new(pos: Pos, hatch: Insect, germination: u8) -> Insect {
        Insect {
            base: BaseInsect::new(pos, hatch.base.faction),
            spec: Box::new(Self {
                germination,
                hatch: Some(Box::new(hatch)),
            }),
        }
    }
}

impl InsectBehaviour for Egg {
    fn update(&mut self, _base: &mut BaseInsect, _map: &mut crate::map::Map) -> Option<super::Event> {
        self.germination = self.germination.saturating_sub(1);
        if self.germination == 0 && self.hatch.is_some() {
            Some(super::Event::Rebirth(self.hatch.take().unwrap()))
        } else {
            None
        }
    }

    fn player_action(&mut self, _base: &mut BaseInsect, _map: &mut crate::map::Map, _action: super::Action) -> Option<super::Event> {
        todo!()
    }
}
