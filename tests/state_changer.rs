use state_queue::StateOwner;

#[test]
fn create_state_owner() {
    //this test is just to show how to initialize state
    //and verify that cargo test is actually working
    let example = StateOwner{
        state: String::from("State Owner Created!"),
    };

    assert_eq!(example.state, "State Owner Created!");
}