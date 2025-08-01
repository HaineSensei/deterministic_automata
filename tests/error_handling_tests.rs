use deterministic_automata::*;
use deterministic_automata::counter_automaton_example::{CounterAutomatonBlueprint, CounterState};
use deterministic_automata::product_automaton::{ProductAutomatonBlueprint, BasicUnionAutomatonBlueprint, BasicIntersectionAutomatonBlueprint};

struct FailingBlueprint {
    should_fail_state_sort: bool,
    should_fail_transition: bool,
}

impl FailingBlueprint {
    fn new(fail_state_sort: bool, fail_transition: bool) -> Self {
        Self {
            should_fail_state_sort: fail_state_sort,
            should_fail_transition: fail_transition,
        }
    }
}

impl DeterministicAutomatonBlueprint for FailingBlueprint {
    type State = i32;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn state_sort_map(&self, _state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        if self.should_fail_state_sort {
            Err("State sort validation failed".to_string())
        } else {
            Ok(BasicStateSort::Accept)
        }
    }

    fn transition_map(&self, _state: &Self::State, _character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        if self.should_fail_transition {
            Err("Transition validation failed".to_string())
        } else {
            Ok(1)
        }
    }
}

#[test]
fn error_propagation_from_state_sort() {
    let blueprint = FailingBlueprint::new(true, false);
    let automaton = DeterministicAutomaton::new(&blueprint);
    
    let result = automaton.current_state_sort();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "State sort validation failed");
}

#[test]
fn error_propagation_from_transition() {
    let blueprint = FailingBlueprint::new(false, true);
    let mut automaton = DeterministicAutomaton::new(&blueprint);
    
    let result = automaton.update_state(&'a');
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Transition validation failed");
}

#[test]
fn error_propagation_in_characterise() {
    let blueprint = FailingBlueprint::new(false, true);
    let input: Vec<char> = "test".chars().collect();
    
    let result = blueprint.characterise(&input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Transition validation failed");
}

#[test]
fn error_propagation_in_product_automaton() {
    let good_blueprint = CounterAutomatonBlueprint::new('a', 'b');
    let bad_blueprint = FailingBlueprint::new(true, false);
    let product = ProductAutomatonBlueprint::new(&good_blueprint, &bad_blueprint);
    
    let result = product.characterise(&vec![]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "State sort validation failed");
}

#[test]
fn error_propagation_in_union_automaton() {
    let good_blueprint = CounterAutomatonBlueprint::new('a', 'b');
    let bad_blueprint = FailingBlueprint::new(true, false);
    let union = BasicUnionAutomatonBlueprint::new(&good_blueprint, &bad_blueprint);
    
    let result = union.characterise(&vec![]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "State sort validation failed");
}

#[test]
fn error_propagation_in_intersection_automaton() {
    let good_blueprint = CounterAutomatonBlueprint::new('a', 'b');
    let bad_blueprint = FailingBlueprint::new(false, true);
    let intersection = BasicIntersectionAutomatonBlueprint::new(&good_blueprint, &bad_blueprint);
    
    let result = intersection.characterise(&vec!['a']);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Transition validation failed");
}

#[test]
fn successful_validation_counter_automaton() -> Result<(), String> {
    let blueprint = CounterAutomatonBlueprint::new('a', 'b');
    
    let valid_states = [
        CounterState::Start(0),
        CounterState::Start(100),
        CounterState::End(0),
        CounterState::End(50),
        CounterState::Reject,
    ];
    
    for state in &valid_states {
        blueprint.state_sort_map(state)?;
    }
    
    let transition_tests = [
        (CounterState::Start(0), 'a'),
        (CounterState::Start(5), 'b'),
        (CounterState::End(3), 'b'),
        (CounterState::Reject, 'a'),
        (CounterState::Reject, 'b'),
    ];
    
    for (state, char) in &transition_tests {
        blueprint.transition_map(state, char)?;
    }
    
    Ok(())
}

struct PanicBlueprint;

impl DeterministicAutomatonBlueprint for PanicBlueprint {
    type State = i32;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        match state {
            0 => Ok(BasicStateSort::Accept),
            1 => Ok(BasicStateSort::Reject),
            _ => Err(format!("Invalid state: {}", state))
        }
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        match (state, character) {
            (0, 'a') => Ok(1),
            (1, 'b') => Ok(0),
            _ => Err(format!("Invalid transition from state {} with character '{}'", state, character))
        }
    }
}

#[test]
fn graceful_error_handling_invalid_transitions() {
    let blueprint = PanicBlueprint;
    let mut automaton = DeterministicAutomaton::new(&blueprint);
    
    let result = automaton.update_state(&'x');
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid transition"));
}

#[test]
fn graceful_error_handling_invalid_states() {
    let blueprint = PanicBlueprint;
    
    let result = blueprint.state_sort_map(&999);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid state"));
}