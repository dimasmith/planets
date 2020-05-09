use hecs::World;
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedWorld = Rc<RefCell<World>>;

pub fn create() -> SharedWorld {
    Rc::new(RefCell::new(World::new()))
}
