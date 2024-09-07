
pub struct StateOwner {
    state: String,
}

impl StateOwner {
    pub fn new(state: String) -> StateOwner {
        StateOwner { state }
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }
}