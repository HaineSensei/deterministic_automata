use deterministic_automata::*;
use deterministic_automata::mutation_automaton::MutationAutomatonBlueprint;
use deterministic_automata::either_automaton::mutation::Either;

#[derive(Debug, Clone, PartialEq)]
struct SimpleMutationBlueprint {
    increment_char: char,
    decrement_char: char,
}

impl SimpleMutationBlueprint {
    fn new(increment_char: char, decrement_char: char) -> Self {
        Self { increment_char, decrement_char }
    }
}

impl MutationAutomatonBlueprint for SimpleMutationBlueprint {
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

fn str_to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn mutation_either_left_basic_functionality() -> Result<(), String> {
    let blueprint = SimpleMutationBlueprint::new('+', '-');
    let either_blueprint: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint);
    
    let mut automaton = either_blueprint.mutation_automaton();
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    automaton.update_state(&'+')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    automaton.update_state(&'-')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn mutation_either_right_basic_functionality() -> Result<(), String> {
    let blueprint = SimpleMutationBlueprint::new('a', 'b');
    let either_blueprint: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Right(blueprint);
    
    let mut automaton = either_blueprint.mutation_automaton();
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    automaton.update_state(&'a')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    automaton.update_state(&'b')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn mutation_either_characterise_left() -> Result<(), String> {
    let blueprint = SimpleMutationBlueprint::new('x', 'y');
    let either_blueprint: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint);
    
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char("xy"))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char("x"))?, BasicStateSort::Reject);
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char("xyxy"))?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn mutation_either_characterise_right() -> Result<(), String> {
    let blueprint = SimpleMutationBlueprint::new('p', 'q');
    let either_blueprint: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Right(blueprint);
    
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char("pq"))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char("p"))?, BasicStateSort::Reject);
    assert_eq!(either_blueprint.mutation_characterise(&str_to_vec_char("ppqq"))?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn mutation_either_runtime_selection() -> Result<(), String> {
    // Test Left variant
    let blueprint1 = SimpleMutationBlueprint::new('t', 'u');
    let either_left: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint1);
    
    let result_empty = either_left.mutation_characterise(&str_to_vec_char(""))?;
    assert_eq!(result_empty, BasicStateSort::Accept);
    
    let result_tu = either_left.mutation_characterise(&str_to_vec_char("tu"))?;
    assert_eq!(result_tu, BasicStateSort::Accept);
    
    let result_t = either_left.mutation_characterise(&str_to_vec_char("t"))?;
    assert_eq!(result_t, BasicStateSort::Reject);
    
    // Test Right variant 
    let blueprint2 = SimpleMutationBlueprint::new('t', 'u');
    let either_right: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Right(blueprint2);
    
    let result_empty2 = either_right.mutation_characterise(&str_to_vec_char(""))?;
    assert_eq!(result_empty2, BasicStateSort::Accept);
    
    let result_tu2 = either_right.mutation_characterise(&str_to_vec_char("tu"))?;
    assert_eq!(result_tu2, BasicStateSort::Accept);
    
    let result_t2 = either_right.mutation_characterise(&str_to_vec_char("t"))?;
    assert_eq!(result_t2, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_either_state_management() -> Result<(), String> {
    let blueprint = SimpleMutationBlueprint::new('m', 'n');
    let either_blueprint: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint);
    
    let mut automaton = either_blueprint.mutation_automaton();
    
    let initial_state = automaton.view_state();
    if let Either::Left(state) = initial_state {
        assert_eq!(*state, 0);
    } else {
        panic!("Expected Left variant");
    }
    
    automaton.update_state(&'m')?;
    
    let after_m_state = automaton.view_state();
    if let Either::Left(state) = after_m_state {
        assert_eq!(*state, 1);
    } else {
        panic!("Expected Left variant after transition");
    }
    
    Ok(())
}

#[test]
fn mutation_either_type_equality() {
    let blueprint1 = SimpleMutationBlueprint::new('a', 'b');
    let blueprint2 = SimpleMutationBlueprint::new('a', 'b');
    
    let left1: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint1);
    let left2: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint2);
    
    assert_eq!(left1, left2);
}

#[test]
fn mutation_either_clone_and_copy() {
    let blueprint = SimpleMutationBlueprint::new('x', 'y');
    let either_orig: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint);
    
    let either_cloned = either_orig.clone();
    let either_copied = either_orig;
    
    assert_eq!(either_cloned, either_copied);
}

#[test]
fn mutation_either_debug_format() {
    let blueprint1 = SimpleMutationBlueprint::new('d', 'e');
    let blueprint2 = SimpleMutationBlueprint::new('f', 'g');
    
    let left_either: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Left(blueprint1);
    let right_either: Either<SimpleMutationBlueprint, SimpleMutationBlueprint> = Either::Right(blueprint2);
    
    let left_debug = format!("{:?}", left_either);
    let right_debug = format!("{:?}", right_either);
    
    assert!(left_debug.contains("Left"));
    assert!(right_debug.contains("Right"));
}