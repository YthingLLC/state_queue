use state_queue::{StateModifier, StateOwner};

#[test]
fn create_state_owner() {
    //this test is just to show how to initialize state
    //and verify that cargo test is actually working
    let example = StateOwner::new(String::from("State Owner Created!"));

    assert_eq!(example.get_state(), "State Owner Created!");
}

#[test]
fn create_and_modify_state_owner() {
    let example = StateOwner::new(String::from("State Owner Created!"));

    let mut modifiers: Vec<StateModifier> = Vec::new();

    modifiers.push(StateModifier::ReplaceAt(0, b's'));
    modifiers.push(StateModifier::ReplaceAt(6, b'o'));
    modifiers.push(StateModifier::ReplaceAt(12, b'c'));
    modifiers.push(StateModifier::ReplaceFull(String::from("State Fully Changed!")));

    let keeper = example.begin_transaction();

    //check individually
    for modifier in &modifiers {
        let is_valid = keeper.check_transactions(std::slice::from_ref(modifier));
        println!("{modifier:?} - Valid? {is_valid}");

        assert!(is_valid);
    }

    //check all at once

    let is_valid = keeper.check_transactions(modifiers.as_slice());

    println!("All valid in one shot? {is_valid}");

    assert!(is_valid);

    keeper.close_transaction(modifiers);

    println!("Current State? {}", example.get_state());

    assert_eq!(example.get_state(), "State Fully Changed!");


}