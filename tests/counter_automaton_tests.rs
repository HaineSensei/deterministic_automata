use deterministic_automata::*;
use deterministic_automata::counter_automaton_example::{CounterAutomatonBlueprint, CounterState};

fn str_to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn counter_automaton_basic_acceptance() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');

    assert_eq!(blueprint.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("ab"))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aabb"))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aaaaaaaabbbbbbbb"))?, BasicStateSort::Accept);

    Ok(())
}

#[test]
fn counter_automaton_basic_rejection() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');

    assert_eq!(blueprint.characterise(&str_to_vec_char("aaaabbb"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("bb"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("cab"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aacbb"))?, BasicStateSort::Reject);

    Ok(())
}

#[test]
fn counter_automaton_edge_cases() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');

    assert_eq!(blueprint.characterise(&str_to_vec_char("a"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("b"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("ba"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aba"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("abab"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("abba"))?, BasicStateSort::Reject);

    Ok(())
}

#[test]
fn counter_automaton_large_inputs() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');

    let large_n = 1000;
    let mut large_valid: Vec<char> = Vec::new();
    let mut large_invalid: Vec<char> = Vec::new();

    for _ in 0..large_n {
        large_valid.push('a');
        large_invalid.push('a');
    }
    for _ in 0..large_n {
        large_valid.push('b');
    }
    for _ in 0..(large_n - 1) {
        large_invalid.push('b');
    }

    assert_eq!(blueprint.characterise(&large_valid)?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&large_invalid)?, BasicStateSort::Reject);

    Ok(())
}

#[test]
fn counter_automaton_different_symbols() -> Result<(), String> {
    let blueprint1 = CounterAutomatonBlueprint::new('x', 'y');
    let blueprint2 = CounterAutomatonBlueprint::new('1', '2');
    let blueprint3 = CounterAutomatonBlueprint::new('(', ')');

    assert_eq!(blueprint1.characterise(&str_to_vec_char("xxyy"))?, BasicStateSort::Accept);
    assert_eq!(blueprint1.characterise(&str_to_vec_char("xyxy"))?, BasicStateSort::Reject);

    assert_eq!(blueprint2.characterise(&str_to_vec_char("1122"))?, BasicStateSort::Accept);
    assert_eq!(blueprint2.characterise(&str_to_vec_char("1212"))?, BasicStateSort::Reject);

    assert_eq!(blueprint3.characterise(&str_to_vec_char("(())"))?, BasicStateSort::Accept);
    assert_eq!(blueprint3.characterise(&str_to_vec_char("()()"))?, BasicStateSort::Reject);

    Ok(())
}

#[test]
fn counter_automaton_initial_state() {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');
    
    match blueprint.initial_state() {
        CounterState::Start(0) => {},
        _ => panic!("Initial state should be Start(0)")
    }
}

#[test]
fn counter_automaton_state_transitions() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');
    
    let start_0 = CounterState::Start(0);
    let start_1 = CounterState::Start(1);
    let end_1 = CounterState::End(1);
    let reject = CounterState::Reject;

    assert!(matches!(blueprint.transition_map(&start_0, &'a')?, CounterState::Start(1)));
    assert!(matches!(blueprint.transition_map(&start_1, &'a')?, CounterState::Start(2)));
    assert!(matches!(blueprint.transition_map(&start_1, &'b')?, CounterState::End(0)));
    assert!(matches!(blueprint.transition_map(&end_1, &'b')?, CounterState::End(0)));
    assert!(matches!(blueprint.transition_map(&start_0, &'b')?, CounterState::Reject));
    assert!(matches!(blueprint.transition_map(&reject, &'a')?, CounterState::Reject));
    assert!(matches!(blueprint.transition_map(&reject, &'b')?, CounterState::Reject));

    Ok(())
}

#[test]
fn counter_automaton_state_classification() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');
    
    assert_eq!(blueprint.state_sort_map(&CounterState::Start(0))?, BasicStateSort::Accept);
    assert_eq!(blueprint.state_sort_map(&CounterState::Start(1))?, BasicStateSort::Reject);
    assert_eq!(blueprint.state_sort_map(&CounterState::Start(10))?, BasicStateSort::Reject);
    assert_eq!(blueprint.state_sort_map(&CounterState::End(0))?, BasicStateSort::Accept);
    assert_eq!(blueprint.state_sort_map(&CounterState::End(1))?, BasicStateSort::Reject);
    assert_eq!(blueprint.state_sort_map(&CounterState::End(5))?, BasicStateSort::Reject);
    assert_eq!(blueprint.state_sort_map(&CounterState::Reject)?, BasicStateSort::Reject);

    Ok(())
}