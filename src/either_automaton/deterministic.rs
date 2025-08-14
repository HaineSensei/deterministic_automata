//! Either type implementation for deterministic automaton blueprints.
//!
//! This module provides an [`Either`] type that implements [`DeterministicAutomatonBlueprint`],
//! allowing you to create a blueprint that represents a choice between two different 
//! deterministic automaton types. This enables runtime selection between automata while 
//! maintaining compile-time type safety.
//!
//! # Example: Runtime Selection Between Different Automaton Types
//!
//! ```
//! use deterministic_automata::{DeterministicAutomatonBlueprint, BasicStateSort};
//! use deterministic_automata::counter_automaton_example::CounterAutomatonBlueprint;
//! use deterministic_automata::product_automaton::BasicUnionAutomatonBlueprint;
//! use deterministic_automata::either_automaton::deterministic::Either;
//!
//! // Create different types of automata
//! let counter_automaton = CounterAutomatonBlueprint::new('a', 'b');
//! let other_counter = CounterAutomatonBlueprint::new('x', 'y');
//! let union_automaton = BasicUnionAutomatonBlueprint::new(&counter_automaton, &other_counter);
//! 
//! // Choose which type to use at runtime
//! let use_simple = true;
//! let chosen_automaton = if use_simple {
//!     Either::Left(counter_automaton)
//! } else {
//!     Either::Right(union_automaton)
//! };
//! ```

use crate::DeterministicAutomatonBlueprint;

/// A sum type representing a choice between two values for deterministic automata.
///
/// This type mimics the required functionality of `either::Either` for use in 
/// deterministic automaton composition, allowing runtime selection between two different 
/// automaton blueprint types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Either<A,B> {
    /// The left variant containing a value of type `A`.
    Left(A),
    /// The right variant containing a value of type `B`.
    Right(B)
}

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