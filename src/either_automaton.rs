use crate::DeterministicAutomatonBlueprint;
use either::Either;


impl<A,B, StateSort, Alphabet, ErrorType> DeterministicAutomatonBlueprint for Either<A,B> 
where
    A: DeterministicAutomatonBlueprint<StateSort = StateSort, Alphabet = Alphabet, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<StateSort = StateSort, Alphabet = Alphabet, ErrorType = ErrorType>,
    StateSort: Clone,
    Alphabet: PartialEq,
    ErrorType: Default
{
    type State = Either<A::State,B::State>;

    type Alphabet = Alphabet;

    type StateSort = StateSort;

    type ErrorType = ErrorType;

    fn initial_state(&self) -> Self::State {
        match self {
            Either::Left(x) => Either::Left(x.initial_state()),
            Either::Right(y) => Either::Right(y.initial_state()),
        }
    }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        match (self,state) {
            (Either::Left(blueprint), Either::Left(state)) => blueprint.state_sort_map(state),
            (Either::Left(_), Either::Right(_)) => Err(Default::default()),
            (Either::Right(_), Either::Left(_)) => Err(Default::default()),
            (Either::Right(blueprint), Either::Right(state)) => blueprint.state_sort_map(state),
        }
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        match (self,state) {
            (Either::Left(blueprint), Either::Left(state)) => Ok(Either::Left(blueprint.transition_map(state, character)?)),
            (Either::Left(_), Either::Right(_)) => Err(Default::default()),
            (Either::Right(_), Either::Left(_)) => Err(Default::default()),
            (Either::Right(blueprint), Either::Right(state)) => Ok(Either::Right(blueprint.transition_map(state, character)?)),
        }
    }
}