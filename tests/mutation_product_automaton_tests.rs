use deterministic_automata::*;
use deterministic_automata::mutation_automaton::MutationAutomatonBlueprint;
use deterministic_automata::product_automaton::{MutationProductAutomatonBlueprint, MutationBasicUnionAutomatonBlueprint, MutationBasicIntersectionAutomatonBlueprint};

#[derive(Debug, Clone, PartialEq)]
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
            // Ignore unknown characters instead of erroring
            // This allows automata with different alphabets to work in products
        }
        Ok(())
    }
}

// Simple automaton that accepts only specific characters
#[derive(Debug, Clone, PartialEq)]
struct SimpleAcceptBlueprint {
    accept_chars: Vec<char>,
}

impl SimpleAcceptBlueprint {
    fn new(accept_chars: Vec<char>) -> Self {
        Self { accept_chars }
    }
}

impl MutationAutomatonBlueprint for SimpleAcceptBlueprint {
    type State = bool; // true = accept, false = reject
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_mutation_state(&self) -> Self::State {
        true // Start in accept state
    }

    fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        if *state {
            Ok(BasicStateSort::Accept)
        } else {
            Ok(BasicStateSort::Reject)
        }
    }

    fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(), Self::ErrorType> {
        if !self.accept_chars.contains(character) {
            *state = false; // Move to reject state on invalid character
        }
        Ok(())
    }
}

fn str_to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn mutation_product_automaton_basic_functionality() -> Result<(), String> {
    let blueprint_a = MutableCounterBlueprint::new('a', 'b');
    let blueprint_b = MutableCounterBlueprint::new('a', 'b');
    let product = MutationProductAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let result_empty = product.mutation_characterise(&str_to_vec_char(""))?;
    assert_eq!(result_empty.0, BasicStateSort::Accept);
    assert_eq!(result_empty.1, BasicStateSort::Accept);
    
    let result_mixed = product.mutation_characterise(&str_to_vec_char("ab"))?;
    assert_eq!(result_mixed.0, BasicStateSort::Accept);
    assert_eq!(result_mixed.1, BasicStateSort::Accept);
    
    Ok(())
}

#[test]
fn mutation_product_automaton_independent_processing() -> Result<(), String> {
    let blueprint_a = MutableCounterBlueprint::new('a', 'b');
    let blueprint_b = MutableCounterBlueprint::new('a', 'b');
    let product = MutationProductAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let result_valid = product.mutation_characterise(&str_to_vec_char("aabb"))?;
    assert_eq!(result_valid.0, BasicStateSort::Accept);
    assert_eq!(result_valid.1, BasicStateSort::Accept);
    
    let result_invalid = product.mutation_characterise(&str_to_vec_char("aaab"))?;
    assert_eq!(result_invalid.0, BasicStateSort::Reject);
    assert_eq!(result_invalid.1, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_product_automaton_state_management() -> Result<(), String> {
    let blueprint_a = MutableCounterBlueprint::new('a', 'b');
    let blueprint_b = MutableCounterBlueprint::new('a', 'b');
    let product = MutationProductAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let mut automaton = product.mutation_automaton();
    
    let initial_sort = automaton.current_state_sort()?;
    assert_eq!(initial_sort.0, BasicStateSort::Accept);
    assert_eq!(initial_sort.1, BasicStateSort::Accept);
    
    let after_a = automaton.update_sort_state(&'a')?;
    assert_eq!(after_a.0, BasicStateSort::Reject);
    assert_eq!(after_a.1, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_union_automaton_or_logic() -> Result<(), String> {
    // Automaton A accepts only 'a' and 'b' 
    let blueprint_a = SimpleAcceptBlueprint::new(vec!['a', 'b']);
    // Automaton B accepts only 'x' and 'y'
    let blueprint_b = SimpleAcceptBlueprint::new(vec!['x', 'y']);
    let union = MutationBasicUnionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    // Empty string should be accepted (both start in accept state)
    assert_eq!(union.mutation_characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    
    // Strings with only 'a','b' should be accepted (A accepts, B rejects, OR = Accept)
    assert_eq!(union.mutation_characterise(&str_to_vec_char("ab"))?, BasicStateSort::Accept);
    assert_eq!(union.mutation_characterise(&str_to_vec_char("a"))?, BasicStateSort::Accept);
    
    // Strings with only 'x','y' should be accepted (A rejects, B accepts, OR = Accept)  
    assert_eq!(union.mutation_characterise(&str_to_vec_char("xy"))?, BasicStateSort::Accept);
    assert_eq!(union.mutation_characterise(&str_to_vec_char("x"))?, BasicStateSort::Accept);
    
    // Mixed strings should be rejected (both A and B reject, OR = Reject)
    assert_eq!(union.mutation_characterise(&str_to_vec_char("ax"))?, BasicStateSort::Reject);
    assert_eq!(union.mutation_characterise(&str_to_vec_char("abx"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_union_automaton_mixed_acceptance() -> Result<(), String> {
    // Same setup as above - test additional mixed cases
    let blueprint_a = SimpleAcceptBlueprint::new(vec!['a', 'b']);
    let blueprint_b = SimpleAcceptBlueprint::new(vec!['x', 'y']);
    let union = MutationBasicUnionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    // Additional mixed cases that should be rejected
    assert_eq!(union.mutation_characterise(&str_to_vec_char("abx"))?, BasicStateSort::Reject);
    assert_eq!(union.mutation_characterise(&str_to_vec_char("xab"))?, BasicStateSort::Reject);
    assert_eq!(union.mutation_characterise(&str_to_vec_char("axy"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_intersection_automaton_and_logic() -> Result<(), String> {
    let blueprint_a = MutableCounterBlueprint::new('a', 'b');
    let blueprint_b = MutableCounterBlueprint::new('a', 'b');
    let intersection = MutationBasicIntersectionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("ab"))?, BasicStateSort::Accept);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("aabb"))?, BasicStateSort::Accept);
    
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("a"))?, BasicStateSort::Reject);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("b"))?, BasicStateSort::Reject);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("abb"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_intersection_automaton_different_languages() -> Result<(), String> {
    // Automaton A accepts only 'a','b', Automaton B accepts only 'x','y'
    // Intersection should only accept empty string (both start accepting)
    let blueprint_a = SimpleAcceptBlueprint::new(vec!['a', 'b']);
    let blueprint_b = SimpleAcceptBlueprint::new(vec!['x', 'y']);
    let intersection = MutationBasicIntersectionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    // Empty string accepted (both automata start in accept state)
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    
    // Any non-empty string should be rejected (can't satisfy both automata)
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("ab"))?, BasicStateSort::Reject);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("xy"))?, BasicStateSort::Reject);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("a"))?, BasicStateSort::Reject);
    assert_eq!(intersection.mutation_characterise(&str_to_vec_char("x"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn mutation_union_vs_intersection_comparison() -> Result<(), String> {
    let blueprint_a = SimpleAcceptBlueprint::new(vec!['a', 'b']);
    let blueprint_b = SimpleAcceptBlueprint::new(vec!['x', 'y']);
    let union = MutationBasicUnionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    let intersection = MutationBasicIntersectionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let test_cases = vec!["", "ab", "xy", "a", "x", "ax", "abx"];
    
    for case in test_cases {
        let union_result = union.mutation_characterise(&str_to_vec_char(case))?;
        let intersection_result = intersection.mutation_characterise(&str_to_vec_char(case))?;
        
        match case {
            "" => {
                // Both start in accept state
                assert_eq!(union_result, BasicStateSort::Accept);
                assert_eq!(intersection_result, BasicStateSort::Accept);
            }
            "ab" | "a" => {
                // A accepts, B rejects -> Union: Accept, Intersection: Reject
                assert_eq!(union_result, BasicStateSort::Accept);
                assert_eq!(intersection_result, BasicStateSort::Reject);
            }
            "xy" | "x" => {
                // A rejects, B accepts -> Union: Accept, Intersection: Reject  
                assert_eq!(union_result, BasicStateSort::Accept);
                assert_eq!(intersection_result, BasicStateSort::Reject);
            }
            _ => {
                // Both reject -> Union: Reject, Intersection: Reject
                assert_eq!(union_result, BasicStateSort::Reject);
                assert_eq!(intersection_result, BasicStateSort::Reject);
            }
        }
    }
    
    Ok(())
}