use glam;
use glm::*;

mod movment {

    trait Kenetic {
        fn new(pos: vec3) -> self;
    }

    pub impl Kenetic {
        fn pos() -> Vec3 {
            return self.pos;
        }
    }
}

pub struct world {
    pub components: Vec<Box<dyn Kenetic>>,
}
