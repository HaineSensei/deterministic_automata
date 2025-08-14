use deterministic_automata::*;
use deterministic_automata::mutation_automaton::{MutationAutomatonBlueprint, MutationAutomaton};

struct MutableCounterBlueprint {
    increment_char: char,
    decrement_char: char,
}

impl MutableCounterBlueprint {
    fn new(increment_char: char, decrement_char: char) -> Self {
        Self { increment_char, decrement_char }
    }
}

impl MutationAutomatonBlueprint for MutableCounterBlueprint {
    type State = i32;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_mutation_state(&self) -> Self::State {
        0
    }

    fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        if *state == 0 {
            Ok(BasicStateSort::Accept)
        } else {
            Ok(BasicStateSort::Reject)
        }
    }

    fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(), Self::ErrorType> {
        if *character == self.increment_char {
            *state += 1;
        } else if *character == self.decrement_char {
            *state -= 1;
        } else {
            return Err(format!("Invalid character: {}", character));
        }
        Ok(())
    }
}

#[test]
fn mutation_automaton_basic_functionality() -> Result<(), String> {
    let blueprint = MutableCounterBlueprint::new('+', '-');
    let mut automaton = MutationAutomaton::new(&blueprint);
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    assert_eq!(*automaton.view_state(), 0);
    
    automaton.update_state(&'+')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    assert_eq!(*automaton.view_state(), 1);
    
    automaton.update_state(&'-')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    assert_eq!(*automaton.view_state(), 0);
    
    Ok(())
}

#[test]
fn mutation_automaton_update_sort_state() -> Result<(), String> {
    let blueprint = MutableCounterBlueprint::new('a', 'b');
    let mut automaton = MutationAutomaton::new(&blueprint);
    
    let sort_after_a = automaton.update_sort_state(&'a')?;
    assert_eq!(sort_after_a, BasicStateSort::Reject);
    assert_eq!(*automaton.view_state(), 1);
    
    let sort_after_b = automaton.update_sort_state(&'b')?;
    assert_eq!(sort_after_b, BasicStateSort::Accept);
    assert_eq!(*automaton.view_state(), 0);
    
    Ok(())
}

#[test]
fn mutation_automaton_error_handling() -> Result<(), String> {
    let blueprint = MutableCounterBlueprint::new('x', 'y');
    let mut automaton = MutationAutomaton::new(&blueprint);
    
    let result = automaton.update_state(&'z');
    assert!(result.is_err());
    assert_eq!(*automaton.view_state(), 0); // State unchanged on error
    
    Ok(())
}

#[test]
fn mutation_automaton_take_state() -> Result<(), String> {
    let blueprint = MutableCounterBlueprint::new('i', 'd');
    let mut automaton = MutationAutomaton::new(&blueprint);
    
    automaton.update_state(&'i')?;
    automaton.update_state(&'i')?;
    assert_eq!(*automaton.view_state(), 2);
    
    let final_state = automaton.take_state();
    assert_eq!(final_state, 2);
    
    Ok(())
}

#[test]
fn mutation_automaton_multiple_transitions() -> Result<(), String> {
    let blueprint = MutableCounterBlueprint::new('u', 'd');
    let mut automaton = MutationAutomaton::new(&blueprint);
    
    let operations = ['u', 'u', 'u', 'd', 'd', 'u'];
    let expected_states = [1, 2, 3, 2, 1, 2];
    let expected_sorts = [
        BasicStateSort::Reject, BasicStateSort::Reject, BasicStateSort::Reject,
        BasicStateSort::Reject, BasicStateSort::Reject, BasicStateSort::Reject
    ];
    
    for (i, &op) in operations.iter().enumerate() {
        automaton.update_state(&op)?;
        assert_eq!(*automaton.view_state(), expected_states[i]);
        assert_eq!(automaton.current_state_sort()?, expected_sorts[i]);
    }
    
    Ok(())
}