

pub trait DeterministicAutomatonBlueprint {
    type State: Clone;
    type Alphabet: PartialEq;
    type StateSort;
    type ErrorType;

    fn initial_state(&self) -> Self::State;

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType>;

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType>;

    fn characterise(&self, word: &[Self::Alphabet]) -> Result<Self::StateSort, Self::ErrorType>
    where
        Self : Sized
    {
        let mut automaton: DeterministicAutomaton<'_, Self> = DeterministicAutomaton::new(self);
        for character in word {
            automaton.update_state(character)?;
        }
        automaton.current_state_sort()
    }
}

pub struct DeterministicAutomaton<'a, Blueprint: DeterministicAutomatonBlueprint> {
    blueprint: &'a Blueprint,
    current_state: Blueprint::State,
}

impl<'a, Blueprint> DeterministicAutomaton<'a, Blueprint>
where
    Blueprint: DeterministicAutomatonBlueprint
{
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            current_state: blueprint.initial_state()
        }
    }

    pub fn current_state_sort(&self) -> Result<Blueprint::StateSort,Blueprint::ErrorType> {
        self.blueprint.state_sort_map(&self.current_state)
    }

    pub fn update_state(&mut self, character: &Blueprint::Alphabet) -> Result<Blueprint::StateSort, Blueprint::ErrorType> {
        let next_state: <Blueprint as DeterministicAutomatonBlueprint>::State = self.blueprint.transition_map(&self.current_state, character)?;
        self.current_state = next_state;
        self.blueprint.state_sort_map(&self.current_state)
    }
}

pub struct CounterAutomatonBlueprint<Alphabet> {
    first: Alphabet,
    second: Alphabet
}

impl<Alphabet> CounterAutomatonBlueprint<Alphabet> {
    pub fn new(first: Alphabet, second: Alphabet) -> Self {
        Self { first, second }
    }
}

#[derive(Clone)]
pub enum CounterState {
    Start(usize),
    End(usize),
    Reject
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BasicStateSort {
    Accept,
    Reject
}

impl<Alphabet> DeterministicAutomatonBlueprint for CounterAutomatonBlueprint<Alphabet>
where 
    Alphabet: PartialEq
{
    type State = CounterState;

    type Alphabet = Alphabet;

    type StateSort = BasicStateSort;

    type ErrorType = String;

    fn initial_state(&self) -> Self::State {
        CounterState::Start(0)
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        match match state {
            CounterState::Start(x) => x,
            CounterState::End(x) => x,
            CounterState::Reject => &1
        } {
            0 => Ok(BasicStateSort::Accept),
            _ => Ok(BasicStateSort::Reject)
        }
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        Ok(match state {
            CounterState::Start(counter) => {
                if character == &self.first {
                    CounterState::Start(counter+1)
                } else if character == &self.second && counter > &0 {
                    CounterState::End(*counter - 1)
                } else {
                    CounterState::Reject
                }
            },
            CounterState::End(counter) => {
                if character == &self.second && counter > &0 {
                    CounterState::End(counter-1)
                } else {
                    CounterState::Reject
                }
            },
            CounterState::Reject => CounterState::Reject,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_automaton_blueprint() -> Result<(),String> {
        let blueprint = CounterAutomatonBlueprint::new('a','b');
        assert_eq!(blueprint.characterise(&"".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Accept);
        assert_eq!(blueprint.characterise(&"ab".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Accept);
        assert_eq!(blueprint.characterise(&"aabb".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Accept);
        assert_eq!(blueprint.characterise(&"aaaaaaaabbbbbbbb".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Accept);
        assert_eq!(blueprint.characterise(&"aaaabbb".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Reject);
        assert_eq!(blueprint.characterise(&"bb".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Reject);
        assert_eq!(blueprint.characterise(&"cab".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Reject);
        assert_eq!(blueprint.characterise(&"aacbb".to_string().chars().collect::<Vec<_>>())?, BasicStateSort::Reject);
        Ok(())
    }
}
