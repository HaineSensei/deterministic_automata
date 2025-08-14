//! Either type implementation for mutation automaton blueprints.
//!
//! This module provides an [`Either`] type that implements [`MutationAutomatonBlueprint`],
//! allowing you to create a blueprint that represents a choice between two different 
//! mutation automaton types. This enables runtime selection between automata while 
//! maintaining compile-time type safety with in-place state mutation.
//!
//! # Example: Runtime Selection Between Different Mutation Automaton Types
//!
//! ```
//! use deterministic_automata::mutation_automaton::MutationAutomatonBlueprint;
//! use deterministic_automata::either_automaton::mutation::Either;
//! # use deterministic_automata::BasicStateSort;
//! 
//! # // Mock blueprints for example - you'd use real ones
//! # struct MockMutationBlueprint;
//! # impl MutationAutomatonBlueprint for MockMutationBlueprint {
//! #     type State = i32;
//! #     type Alphabet = char;
//! #     type StateSort = BasicStateSort;
//! #     type ErrorType = String;
//! #     fn initial_mutation_state(&self) -> Self::State { 0 }
//! #     fn mutation_state_sort_map(&self, _: &Self::State) -> Result<Self::StateSort, Self::ErrorType> { Ok(BasicStateSort::Accept) }
//! #     fn mutation_transition_map(&self, state: &mut Self::State, _: &Self::Alphabet) -> Result<(), Self::ErrorType> { *state += 1; Ok(()) }
//! # }
//!
//! // Create different types of mutation automata
//! let simple_automaton = MockMutationBlueprint;
//! let complex_automaton = MockMutationBlueprint;
//! 
//! // Choose which type to use at runtime
//! let use_simple = true;
//! let chosen_automaton = if use_simple {
//!     Either::Left(simple_automaton)
//! } else {
//!     Either::Right(complex_automaton)
//! };
//! ```

use crate::mutation_automaton::MutationAutomatonBlueprint;

/// A sum type representing a choice between two values for mutation automata.
///
/// This type allows runtime selection between two different mutation automaton 
/// blueprint types, with in-place state mutation for both variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Either<A,B> {
    /// The left variant containing a value of type `A`.
    Left(A),
    /// The right variant containing a value of type `B`.
    Right(B)
}

impl<A,B, StateSort, Alphabet, ErrorType> MutationAutomatonBlueprint for Either<A,B> 
where
    A: MutationAutomatonBlueprint<StateSort = StateSort, Alphabet = Alphabet, ErrorType = ErrorType>,
    B: MutationAutomatonBlueprint<StateSort = StateSort, Alphabet = Alphabet, ErrorType = ErrorType>,
    StateSort: Clone,
    Alphabet: PartialEq,
    ErrorType: Default
{
    type State = Either<A::State,B::State>;

    type Alphabet = Alphabet;

    type StateSort = StateSort;

    type ErrorType = ErrorType;

    fn initial_mutation_state(&self) -> Self::State {
        match self {
            Either::Left(x) => Either::Left(x.initial_mutation_state()),
            Either::Right(y) => Either::Right(y.initial_mutation_state()),
        }
    }

    fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        match (self,state) {
            (Either::Left(blueprint), Either::Left(state)) => blueprint.mutation_state_sort_map(state),
            (Either::Left(_), Either::Right(_)) => Err(Default::default()),
            (Either::Right(_), Either::Left(_)) => Err(Default::default()),
            (Either::Right(blueprint), Either::Right(state)) => blueprint.mutation_state_sort_map(state),
        }
    }

    fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(), Self::ErrorType> {
        match (self, state) {
            (Either::Left(blueprint), Either::Left(state)) => blueprint.mutation_transition_map(state, character),
            (Either::Left(_), Either::Right(_)) => Err(Default::default()),
            (Either::Right(_), Either::Left(_)) => Err(Default::default()),
            (Either::Right(blueprint), Either::Right(state)) => blueprint.mutation_transition_map(state, character),
        }
    }
}