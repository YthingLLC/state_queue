use std::borrow::Cow;
use std::rc::Rc;
use std::sync::{RwLock, RwLockWriteGuard};

pub struct StateOwner {
    state: Rc<RwLock<String>>,
}

pub struct StateKeeper<'a> {
    state: RwLockWriteGuard<'a, String>,
}

impl StateOwner {
    pub fn new(state: String) -> StateOwner {
        StateOwner { state: Rc::new(RwLock::new(state)) }
    }

    pub fn get_state(&self) -> Cow<str> {
        let ret = self.state.read().unwrap().clone();

        Cow::from(ret)
    }

    //WARNING: As this is just a "proof of concept"
    //you can unintentionally cause deadlocks by trying to call
    //.get_state() before closing out the transaction
    //WARNING: StateKeeper can not be instantiated outside of this
    //file as there is no public access to StateKeeper.state; however,
    //.close_transaction() does not verify that the StateKeeper provided
    //is the same one that was generated for &self, again, this is a "proof of concept"
    //and handling this is outside the scope of the concept
    pub fn begin_transaction(&self) -> StateKeeper {
        StateKeeper { state: self.state.write().unwrap() }
    }

    pub fn check_transactions(&self, state_keeper: &StateKeeper, change_queue: &[StateModifier]) -> bool {
        for modifier in change_queue {
            match modifier {
                //ReplaceFull will always be valid
                StateModifier::ReplaceFull(_) => {},
                StateModifier::ReplaceAt(pos, c) => {
                    if pos > &state_keeper.state.len() {
                        return false;
                    }
                    if !c.is_ascii_alphanumeric() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn close_transaction(&self, state_keeper: StateKeeper, change_queue: Vec<StateModifier>) {

        let mut state = state_keeper.state;

        for modifier in change_queue {
            print!("Applying modifier: ");
            match modifier {
                StateModifier::ReplaceFull(new_state) => {
                    print!("Replace Full: Before: {state}, ");
                    *state = new_state;
                }
                StateModifier::ReplaceAt(pos, c) => {
                    //WARNING: This will panic if pos is out of bounds
                    //out of scope of concept to handle this
                    print!("Replace At: Before: {state}, ");
                    //This is being *really* naughty and unsafe
                    //but again, proof of concept
                    unsafe {
                        state.as_bytes_mut()[pos] = c;
                    }
                }
            }
            println!("After: {state}");
        }
    }

}


#[derive(Debug)]
pub enum StateModifier {
    ReplaceFull(String),
    ReplaceAt(usize, u8),
}