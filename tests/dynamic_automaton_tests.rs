use deterministic_automata::{BasicStateSort, DeterministicAutomatonBlueprint, MutationAutomatonBlueprint, DynamicAutomatonBlueprint};

// Simple counting automaton that accepts if count >= 0
struct CountingBlueprint;

impl DeterministicAutomatonBlueprint for CountingBlueprint {
    type State = i32;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_state(&self) -> Self::State {
        0
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        Ok(if *state >= 0 { BasicStateSort::Accept } else { BasicStateSort::Reject })
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        Ok(match character {
            '+' => state + 1,
            '-' => state - 1,
            _ => return Err("Invalid character".to_string()),
        })
    }
}

// Simple finite state automaton that detects "ab" pattern
#[derive(Clone, PartialEq, Debug)]
enum SimpleState {
    Start,
    SawA,
    AcceptAB,
}

struct EndsWithAB;

impl DeterministicAutomatonBlueprint for EndsWithAB {
    type State = SimpleState;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_state(&self) -> Self::State {
        SimpleState::Start
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        Ok(match state {
            SimpleState::AcceptAB => BasicStateSort::Accept,
            _ => BasicStateSort::Reject,
        })
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        Ok(match (state, character) {
            (SimpleState::Start, 'a') => SimpleState::SawA,
            (SimpleState::Start, _) => SimpleState::Start,
            (SimpleState::SawA, 'a') => SimpleState::SawA,
            (SimpleState::SawA, 'b') => SimpleState::AcceptAB,
            (SimpleState::SawA, _) => SimpleState::Start,
            (SimpleState::AcceptAB, 'a') => SimpleState::SawA,
            (SimpleState::AcceptAB, _) => SimpleState::Start,
        })
    }
}

#[test]
fn test_dynamic_automaton_heterogeneous_collection() {
    // Create a collection of different automaton blueprints with the same interface types
    let blueprints: Vec<&DynamicAutomatonBlueprint<char, BasicStateSort, String>> = vec![
        &CountingBlueprint,
        &EndsWithAB,
    ];

    // Test the counting automaton through dynamic dispatch
    let counting_result = blueprints[0].characterise(&['+', '+', '-']);
    assert_eq!(counting_result.unwrap(), BasicStateSort::Accept);

    // Test the pattern matching automaton through dynamic dispatch
    let pattern_result = blueprints[1].characterise(&['a', 'b']);
    assert_eq!(pattern_result.unwrap(), BasicStateSort::Accept);
}

#[test]
fn test_dynamic_automaton_runtime_creation() {
    let counting = CountingBlueprint;
    let pattern = EndsWithAB;

    // Create dynamic automaton instances
    let mut counting_automaton = counting.automaton();
    let mut pattern_automaton = pattern.automaton();

    // Test step-by-step processing
    counting_automaton.update_state(&'+').unwrap();
    counting_automaton.update_state(&'+').unwrap();
    assert_eq!(counting_automaton.current_state_sort().unwrap(), BasicStateSort::Accept);

    pattern_automaton.update_state(&'a').unwrap();
    assert_eq!(pattern_automaton.current_state_sort().unwrap(), BasicStateSort::Reject);
    pattern_automaton.update_state(&'b').unwrap();
    assert_eq!(pattern_automaton.current_state_sort().unwrap(), BasicStateSort::Accept);
}

// Mutation automaton implementation for testing interoperability
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
fn test_deterministic_and_mutation_automata_interoperability() {
    // Create a deterministic automaton (functional style)
    let deterministic = CountingBlueprint;
    
    // Create a mutation automaton (in-place style) 
    let mutation = MutableCounterBlueprint::new('+', '-');
    
    // Store both in the same dynamic collection - this demonstrates the key benefit!
    let blueprints: Vec<&DynamicAutomatonBlueprint<char, BasicStateSort, String>> = vec![
        &deterministic,  // DeterministicAutomatonBlueprint -> works via blanket impl
        &mutation,       // MutationAutomatonBlueprint -> works directly
    ];
    
    // Both can be used through the same dynamic interface
    // CountingBlueprint accepts when count >= 0, so ['+', '+', '-'] -> count=1 -> Accept 
    let det_result = blueprints[0].characterise(&['+', '+', '-']);
    assert_eq!(det_result.unwrap(), BasicStateSort::Accept);
    
    // MutableCounterBlueprint accepts when count == 0, so ['+', '-'] -> count=0 -> Accept
    let mut_result = blueprints[1].characterise(&['+', '-']);
    assert_eq!(mut_result.unwrap(), BasicStateSort::Accept);
    
    // They can also create dynamic automaton instances 
    let mut det_automaton = blueprints[0].automaton();
    let mut mut_automaton = blueprints[1].automaton();
    
    // Both support the same runtime interface despite different implementation paradigms
    // CountingBlueprint: starts at 0, after '+' -> 1 -> Accept (since 1 >= 0)
    det_automaton.update_state(&'+').unwrap();
    assert_eq!(det_automaton.current_state_sort().unwrap(), BasicStateSort::Accept);
    
    // MutableCounterBlueprint: starts at 0, after '+' -> 1 -> Reject (since 1 != 0)  
    mut_automaton.update_state(&'+').unwrap();
    assert_eq!(mut_automaton.current_state_sort().unwrap(), BasicStateSort::Reject);
}