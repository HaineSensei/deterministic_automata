use crate::{BasicStateSort, DeterministicAutomatonBlueprint};

pub struct ProductAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    first: &'a A,
    second: &'b B
}

impl<'a, 'b, A, B, Alphabet, ErrorType> DeterministicAutomatonBlueprint for ProductAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    type State = (A::State, B::State);

    type Alphabet = Alphabet;

    type StateSort = (A::StateSort, B::StateSort);

    type ErrorType = ErrorType;

    fn initial_state(&self) -> Self::State {
        (self.first.initial_state(),self.second.initial_state())
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        let (a, b) = (self.first, self.second);
        let (a_sort, b_sort) = (a.state_sort_map(&state.0)?,b.state_sort_map(&state.1)?);
        Ok((a_sort, b_sort))
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        let (a, b) = (self.first, self.second);
        let (a_next, b_next) = (a.transition_map(&state.0, character)?,b.transition_map(&state.1, character)?);
        Ok((a_next, b_next))
    }
}


pub struct BasicUnionAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    first: &'a A,
    second: &'b B
}

impl<'a, 'b, A, B, Alphabet, ErrorType> BasicUnionAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    pub fn new(first: &'a A, second: &'b B) -> Self {
        Self {
            first,
            second
        }
    }
}

impl<'a, 'b, A, B, Alphabet, ErrorType> DeterministicAutomatonBlueprint for BasicUnionAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    type State = (A::State, B::State);

    type Alphabet = Alphabet;

    type StateSort = BasicStateSort;

    type ErrorType = ErrorType;

    fn initial_state(&self) -> Self::State {
        (self.first.initial_state(), self.second.initial_state())
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        Ok(match (self.first.state_sort_map(&state.0)?, self.second.state_sort_map(&state.1)?) {
            (BasicStateSort::Accept, BasicStateSort::Accept) => BasicStateSort::Accept,
            (BasicStateSort::Accept, BasicStateSort::Reject) => BasicStateSort::Accept,
            (BasicStateSort::Reject, BasicStateSort::Accept) => BasicStateSort::Accept,
            (BasicStateSort::Reject, BasicStateSort::Reject) => BasicStateSort::Reject,
        })
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        let (a, b) = (self.first, self.second);
        let (a_next, b_next) = (a.transition_map(&state.0, character)?,b.transition_map(&state.1, character)?);
        Ok((a_next, b_next))
    }
}

pub struct BasicIntersectionAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    first: &'a A,
    second: &'b B
}

impl<'a, 'b, A, B, Alphabet, ErrorType> BasicIntersectionAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    pub fn new(first: &'a A, second: &'b B) -> Self {
        Self {
            first,
            second
        }
    }
}

impl<'a, 'b, A, B, Alphabet, ErrorType> DeterministicAutomatonBlueprint for BasicIntersectionAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, StateSort = BasicStateSort, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    type State = (A::State, B::State);

    type Alphabet = Alphabet;

    type StateSort = BasicStateSort;

    type ErrorType = ErrorType;

    fn initial_state(&self) -> Self::State {
        (self.first.initial_state(), self.second.initial_state())
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        Ok(match (self.first.state_sort_map(&state.0)?, self.second.state_sort_map(&state.1)?) {
            (BasicStateSort::Accept, BasicStateSort::Accept) => BasicStateSort::Accept,
            (BasicStateSort::Accept, BasicStateSort::Reject) => BasicStateSort::Reject,
            (BasicStateSort::Reject, BasicStateSort::Accept) => BasicStateSort::Reject,
            (BasicStateSort::Reject, BasicStateSort::Reject) => BasicStateSort::Reject,
        })
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        let (a, b) = (self.first, self.second);
        let (a_next, b_next) = (a.transition_map(&state.0, character)?,b.transition_map(&state.1, character)?);
        Ok((a_next, b_next))
    }
}