use deterministic_automata::*;
use deterministic_automata::counter_automaton_example::CounterAutomatonBlueprint;
use deterministic_automata::product_automaton::{ProductAutomatonBlueprint, BasicUnionAutomatonBlueprint, BasicIntersectionAutomatonBlueprint};

fn str_to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn product_automaton_basic_functionality() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('x', 'y');
    let product = ProductAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let result_empty = product.characterise(&str_to_vec_char(""))?;
    assert_eq!(result_empty.0, BasicStateSort::Accept);
    assert_eq!(result_empty.1, BasicStateSort::Accept);
    
    let result_mixed = product.characterise(&str_to_vec_char("ax"))?;
    assert_eq!(result_mixed.0, BasicStateSort::Reject);
    assert_eq!(result_mixed.1, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn product_automaton_independent_processing() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('a', 'b');
    let product = ProductAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let result_valid = product.characterise(&str_to_vec_char("aabb"))?;
    assert_eq!(result_valid.0, BasicStateSort::Accept);
    assert_eq!(result_valid.1, BasicStateSort::Accept);
    
    let result_invalid = product.characterise(&str_to_vec_char("aaab"))?;
    assert_eq!(result_invalid.0, BasicStateSort::Reject);
    assert_eq!(result_invalid.1, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn product_automaton_state_management() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('x', 'y');
    let product = ProductAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let mut automaton = DeterministicAutomaton::new(&product);
    
    let initial_sort = automaton.current_state_sort()?;
    assert_eq!(initial_sort.0, BasicStateSort::Accept);
    assert_eq!(initial_sort.1, BasicStateSort::Accept);
    
    let after_a = automaton.update_state(&'a')?;
    assert_eq!(after_a.0, BasicStateSort::Reject);
    assert_eq!(after_a.1, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn basic_union_automaton_or_logic() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('x', 'y');
    let union = BasicUnionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    assert_eq!(union.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(union.characterise(&str_to_vec_char("ab"))?, BasicStateSort::Accept);
    assert_eq!(union.characterise(&str_to_vec_char("xy"))?, BasicStateSort::Accept);
    assert_eq!(union.characterise(&str_to_vec_char("aabb"))?, BasicStateSort::Accept);
    assert_eq!(union.characterise(&str_to_vec_char("xxyy"))?, BasicStateSort::Accept);
    
    assert_eq!(union.characterise(&str_to_vec_char("a"))?, BasicStateSort::Reject);
    assert_eq!(union.characterise(&str_to_vec_char("x"))?, BasicStateSort::Reject);
    assert_eq!(union.characterise(&str_to_vec_char("ax"))?, BasicStateSort::Reject);
    assert_eq!(union.characterise(&str_to_vec_char("abx"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn basic_union_automaton_mixed_acceptance() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('x', 'y');
    let union = BasicUnionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    assert_eq!(union.characterise(&str_to_vec_char("abx"))?, BasicStateSort::Reject);
    assert_eq!(union.characterise(&str_to_vec_char("xab"))?, BasicStateSort::Reject);
    assert_eq!(union.characterise(&str_to_vec_char("axy"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn basic_intersection_automaton_and_logic() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('a', 'b');
    let intersection = BasicIntersectionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    assert_eq!(intersection.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(intersection.characterise(&str_to_vec_char("ab"))?, BasicStateSort::Accept);
    assert_eq!(intersection.characterise(&str_to_vec_char("aabb"))?, BasicStateSort::Accept);
    
    assert_eq!(intersection.characterise(&str_to_vec_char("a"))?, BasicStateSort::Reject);
    assert_eq!(intersection.characterise(&str_to_vec_char("b"))?, BasicStateSort::Reject);
    assert_eq!(intersection.characterise(&str_to_vec_char("abb"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn basic_intersection_automaton_different_languages() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('x', 'y');
    let intersection = BasicIntersectionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    assert_eq!(intersection.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    
    assert_eq!(intersection.characterise(&str_to_vec_char("ab"))?, BasicStateSort::Reject);
    assert_eq!(intersection.characterise(&str_to_vec_char("xy"))?, BasicStateSort::Reject);
    assert_eq!(intersection.characterise(&str_to_vec_char("aabb"))?, BasicStateSort::Reject);
    assert_eq!(intersection.characterise(&str_to_vec_char("xxyy"))?, BasicStateSort::Reject);
    
    Ok(())
}

#[test]
fn union_vs_intersection_comparison() -> Result<(), String> {
    let blueprint_a = CounterAutomatonBlueprint::new('a', 'b');
    let blueprint_b = CounterAutomatonBlueprint::new('x', 'y');
    let union = BasicUnionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    let intersection = BasicIntersectionAutomatonBlueprint::new(&blueprint_a, &blueprint_b);
    
    let test_cases = vec!["", "ab", "xy", "aabb", "xxyy", "a", "x", "ax"];
    
    for case in test_cases {
        let union_result = union.characterise(&str_to_vec_char(case))?;
        let intersection_result = intersection.characterise(&str_to_vec_char(case))?;
        
        match case {
            "" => {
                assert_eq!(union_result, BasicStateSort::Accept);
                assert_eq!(intersection_result, BasicStateSort::Accept);
            }
            "ab" | "xy" | "aabb" | "xxyy" => {
                assert_eq!(union_result, BasicStateSort::Accept);
                assert_eq!(intersection_result, BasicStateSort::Reject);
            }
            _ => {
                assert_eq!(union_result, BasicStateSort::Reject);
                assert_eq!(intersection_result, BasicStateSort::Reject);
            }
        }
    }
    
    Ok(())
}