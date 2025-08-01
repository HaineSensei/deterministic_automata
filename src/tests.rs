use crate::*;
use counter_example::CounterAutomatonBlueprint;

fn str_to_vec_char(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[test]
fn counter_automaton_blueprint() -> Result<(),String> {
    let blueprint = CounterAutomatonBlueprint::new('a','b');

    assert_eq!(blueprint.characterise(&str_to_vec_char(""))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("ab"))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aabb"))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aaaaaaaabbbbbbbb"))?, BasicStateSort::Accept);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aaaabbb"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("bb"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("cab"))?, BasicStateSort::Reject);
    assert_eq!(blueprint.characterise(&str_to_vec_char("aacbb"))?, BasicStateSort::Reject);

    Ok(())
}