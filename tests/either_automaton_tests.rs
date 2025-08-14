use deterministic_automata::*;
use deterministic_automata::counter_automaton_example::CounterAutomatonBlueprint;
use deterministic_automata::product_automaton::{BasicUnionAutomatonBlueprint, BasicIntersectionAutomatonBlueprint};
use deterministic_automata::either_automaton::deterministic::Either;

fn str_to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn either_left_basic_functionality() -> Result<(), String> {
    let counter = CounterAutomatonBlueprint::new('a', 'b');
    let either_blueprint: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter);
    
    let mut automaton = DeterministicAutomaton::new(&either_blueprint);
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    automaton.update_state(&'a')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Reject);
    
    automaton.update_state(&'b')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn either_right_basic_functionality() -> Result<(), String> {
    let counter1 = CounterAutomatonBlueprint::new('a', 'b');
    let counter2 = CounterAutomatonBlueprint::new('x', 'y');
    let union = BasicUnionAutomatonBlueprint::new(&counter1, &counter2);
    let either_blueprint: Either<CounterAutomatonBlueprint<char>, BasicUnionAutomatonBlueprint<'_, '_, CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>, char, String>> = Either::Right(union);
    
    let mut automaton = DeterministicAutomaton::new(&either_blueprint);
    
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    automaton.update_state(&'a')?;
    automaton.update_state(&'b')?;
    assert_eq!(automaton.current_state_sort()?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn either_characterise_left() -> Result<(), String> {
    let counter = CounterAutomatonBlueprint::new('p', 'q');
    let either_blueprint: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter);
    
    assert_eq!(either_blueprint.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.characterise(&str_to_vec_char("pq"))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.characterise(&str_to_vec_char("p"))?, BasicStateSort::Reject);
    assert_eq!(either_blueprint.characterise(&str_to_vec_char("ppqq"))?, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn either_characterise_right() -> Result<(), String> {
    let counter1 = CounterAutomatonBlueprint::new('a', 'b');
    let counter2 = CounterAutomatonBlueprint::new('x', 'y');
    let intersection = BasicIntersectionAutomatonBlueprint::new(&counter1, &counter2);
    let either_blueprint: Either<CounterAutomatonBlueprint<char>, BasicIntersectionAutomatonBlueprint<'_, '_, CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>, char, String>> = Either::Right(intersection);
    
    assert_eq!(either_blueprint.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(either_blueprint.characterise(&str_to_vec_char("ab"))?, BasicStateSort::Reject);
    assert_eq!(either_blueprint.characterise(&str_to_vec_char("xy"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn either_runtime_selection() -> Result<(), String> {
    // Test Left variant
    let counter1 = CounterAutomatonBlueprint::new('t', 'u');
    let either_left: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter1);
    
    let result_empty = either_left.characterise(&str_to_vec_char(""))?;
    assert_eq!(result_empty, BasicStateSort::Accept);
    
    let result_tu = either_left.characterise(&str_to_vec_char("tu"))?;
    assert_eq!(result_tu, BasicStateSort::Accept);
    
    let result_t = either_left.characterise(&str_to_vec_char("t"))?;
    assert_eq!(result_t, BasicStateSort::Reject);
    
    // Test Right variant 
    let counter2 = CounterAutomatonBlueprint::new('t', 'u');
    let either_right: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Right(counter2);
    
    let result_empty2 = either_right.characterise(&str_to_vec_char(""))?;
    assert_eq!(result_empty2, BasicStateSort::Accept);
    
    let result_tu2 = either_right.characterise(&str_to_vec_char("tu"))?;
    assert_eq!(result_tu2, BasicStateSort::Accept);
    
    let result_t2 = either_right.characterise(&str_to_vec_char("t"))?;
    assert_eq!(result_t2, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn either_state_management() -> Result<(), String> {
    let counter = CounterAutomatonBlueprint::new('m', 'n');
    let either_blueprint: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter);
    
    let mut automaton = DeterministicAutomaton::new(&either_blueprint);
    
    let initial_state = automaton.view_state();
    if let Either::Left(_) = initial_state {
        // Expected
    } else {
        panic!("Expected Left variant");
    }
    
    automaton.update_state(&'m')?;
    
    let after_m_state = automaton.view_state();
    if let Either::Left(_) = after_m_state {
        // Expected
    } else {
        panic!("Expected Left variant after transition");
    }
    
    Ok(())
}

#[test]
fn either_type_equality() {
    let counter1 = CounterAutomatonBlueprint::new('a', 'b');
    let counter2 = CounterAutomatonBlueprint::new('a', 'b');
    let counter3 = CounterAutomatonBlueprint::new('a', 'b');
    let counter4 = CounterAutomatonBlueprint::new('a', 'b');
    
    let left1: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter1);
    let left2: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter2);
    
    assert_eq!(left1, left2);
    
    let union1 = BasicUnionAutomatonBlueprint::new(&counter3, &counter4);
    let union2 = BasicUnionAutomatonBlueprint::new(&counter3, &counter4);
    
    let right1: Either<CounterAutomatonBlueprint<char>, BasicUnionAutomatonBlueprint<'_, '_, CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>, char, String>> = Either::Right(union1);
    let right2: Either<CounterAutomatonBlueprint<char>, BasicUnionAutomatonBlueprint<'_, '_, CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>, char, String>> = Either::Right(union2);
    
    assert_eq!(right1, right2);
}

#[test]
fn either_clone_and_copy() {
    let counter = CounterAutomatonBlueprint::new('x', 'y');
    let either_orig: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter);
    
    let either_cloned = either_orig.clone();
    let either_copied = either_orig;
    
    assert_eq!(either_cloned, either_copied);
}

#[test]
fn either_debug_format() {
    let counter = CounterAutomatonBlueprint::new('d', 'e');
    let other_counter = CounterAutomatonBlueprint::new('f', 'g');
    
    let left_either: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Left(counter);
    let right_either: Either<CounterAutomatonBlueprint<char>, CounterAutomatonBlueprint<char>> = Either::Right(other_counter);
    
    let left_debug = format!("{:?}", left_either);
    let right_debug = format!("{:?}", right_either);
    
    assert!(left_debug.contains("Left"));
    assert!(right_debug.contains("Right"));
}