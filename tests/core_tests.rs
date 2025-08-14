use deterministic_automata::*;
use deterministic_automata::counter_automaton_example::CounterAutomatonBlueprint;

#[test]
fn deterministic_automaton_creation_and_update_sort_state() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');
    let mut automaton = DeterministicAutomaton::new(&blueprint);
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    assert_eq!(automaton.update_sort_state(&'a')?, BasicStateSort::Reject);
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    assert_eq!(automaton.update_sort_state(&'b')?, BasicStateSort::Accept);
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn deterministic_automaton_step_by_step_processing() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('x', 'y');
    let mut automaton = DeterministicAutomaton::new(&blueprint);
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    automaton.update_state(&'x')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    automaton.update_state(&'x')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    automaton.update_state(&'y')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    automaton.update_state(&'y')?; 
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn basic_state_sort_equality() {
    assert_eq!(BasicStateSort::Accept, BasicStateSort::Accept);
    assert_eq!(BasicStateSort::Reject, BasicStateSort::Reject);
    assert_ne!(BasicStateSort::Accept, BasicStateSort::Reject);
    assert_ne!(BasicStateSort::Reject, BasicStateSort::Accept);
}

#[test]
fn basic_state_sort_debug_format() {
    assert_eq!(format!("{:?}", BasicStateSort::Accept), "Accept");
    assert_eq!(format!("{:?}", BasicStateSort::Reject), "Reject");
}

#[test]
fn basic_state_sort_clone_copy() {
    let accept = BasicStateSort::Accept;
    let accept_clone = accept.clone();
    let accept_copy = accept;
    
    assert_eq!(accept_clone, BasicStateSort::Accept);
    assert_eq!(accept_copy, BasicStateSort::Accept);
}

#[test]
fn update_state_returns_unit() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('x', 'y');
    let mut automaton = DeterministicAutomaton::new(&blueprint);
    
    let result = automaton.update_state(&'x')?;
    assert_eq!(result, ());
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    Ok(())
}