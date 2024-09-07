use std::borrow::Cow;
use std::rc::Rc;
use std::sync::{RwLock, RwLockWriteGuard};

pub struct StateOwner {
    state: Rc<RwLock<String>>,
}

impl StateOwner {
    pub fn new(state: String) -> StateOwner {
        StateOwner { state: Rc::new(RwLock::new(state)) }
    }

    pub fn get_state(&self) -> Cow<str> {
        let ret = self.state.read().unwrap().clone();

        Cow::from(ret)
    }


}
