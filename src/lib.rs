use std::rc::Rc;
use std::sync::RwLock;

pub struct StateOwner {
    state: RwLock<Rc<String>>,
}

impl StateOwner {
    pub fn new(state: String) -> StateOwner {
        StateOwner { state: RwLock::new(Rc::new(state)) }
    }

    pub fn get_state(&self) -> Rc<String> {
        self.state.read().unwrap().clone()
    }


}
