//! Mutation automaton implementation for in-place state modification.
//!
//! This module provides the [`MutationAutomatonBlueprint`] trait and related types for
//! creating automata that modify their state in-place rather than returning new states.
//! This paradigm can be more efficient for complex state types and provides a natural
//! interface for stateful computations.
//!
//! # Key Differences from Deterministic Automata
//!
//! - **State Transitions**: `mutation_transition_map` takes `&mut State` and returns `()`
//! - **In-Place Updates**: State changes happen directly rather than through functional updates
//! - **Interoperability**: All deterministic automata automatically work as mutation automata
//!
//! # Example
//!
//! ```
//! use deterministic_automata::{BasicStateSort, mutation_automaton::MutationAutomatonBlueprint};
//!
//! struct Counter;
//!
//! impl MutationAutomatonBlueprint for Counter {
//!     type State = i32;
//!     type Alphabet = char;
//!     type StateSort = BasicStateSort;
//!     type ErrorType = String;
//!
//!     fn initial_mutation_state(&self) -> Self::State { 0 }
//!
//!     fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
//!         Ok(if *state >= 0 { BasicStateSort::Accept } else { BasicStateSort::Reject })
//!     }
//!
//!     fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(), Self::ErrorType> {
//!         match character {
//!             '+' => *state += 1,
//!             '-' => *state -= 1,
//!             _ => return Err("Invalid character".to_string()),
//!         }
//!         Ok(())
//!     }
//! }
//! ```

use crate::DeterministicAutomatonBlueprint;

/// A blueprint for defining mutation automata with in-place state modification.
///
/// This trait allows you to define automata that modify their state directly rather than
/// returning new states. This can be more efficient for complex state types and provides
/// a natural interface for stateful computations.
///
/// # Associated Types
///
/// * `State` - The type representing internal automaton states. Must be `Clone`.
/// * `Alphabet` - The type of input symbols. Must support equality comparison.
/// * `StateSort` - The classification type for states (e.g., Accept/Reject).
/// * `ErrorType` - The type used for error handling when states are invalid.
///
/// # Required Methods
///
/// * [`initial_mutation_state`](Self::initial_mutation_state) - Returns the starting state
/// * [`mutation_state_sort_map`](Self::mutation_state_sort_map) - Classifies a state
/// * [`mutation_transition_map`](Self::mutation_transition_map) - Modifies state in-place
///
/// # Provided Methods
///
/// * [`mutation_characterise`](Self::mutation_characterise) - Processes an entire input sequence
/// * [`mutation_automaton`](Self::mutation_automaton) - Creates a runtime automaton instance
pub trait MutationAutomatonBlueprint {
    type State: Clone;

    type Alphabet: PartialEq;

    type StateSort;

    type ErrorType;

    /// Returns the initial state of the automaton.
    fn initial_mutation_state(&self) -> Self::State;

    /// Maps a state to its classification, with validation.
    ///
    /// This function determines what kind of state the given state represents.
    /// Returns an error if the provided state is invalid.
    fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType>;

    /// Defines the transition function with in-place state mutation and validation.
    ///
    /// Given a current state and an input symbol, modifies the state in-place.
    /// Returns an error if the current state is invalid or if the transition
    /// would produce an invalid state.
    fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(),Self::ErrorType>;

    /// Processes an entire input sequence and returns the final state classification.
    ///
    /// Creates a runtime automaton, processes the input sequence, and returns
    /// the classification of the final state. Propagates any validation errors
    /// encountered during state transitions.
    fn mutation_characterise(&self, word: &[Self::Alphabet]) -> Result<Self::StateSort, Self::ErrorType>
    where
        Self: Sized
    {
        let mut automaton = self.mutation_automaton();
        for character in word {
            automaton.update_state(character)?;
        }
        automaton.current_state_sort()
    }

    /// Creates a runtime automaton instance from this blueprint.
    fn mutation_automaton<'a>(&'a self) -> MutationAutomaton<'a, Self> 
    where 
        Self: Sized
    {
        MutationAutomaton::new(self)
    }
}

/// A runtime instance of a mutation automaton.
///
/// This struct represents an automaton in execution, maintaining the current state
/// and providing methods to process input symbols one at a time with in-place state
/// mutations. It borrows a blueprint that defines the automaton's behavior.
pub struct MutationAutomaton<'a, Blueprint:MutationAutomatonBlueprint> {
    blueprint: &'a Blueprint,
    current_state: Blueprint::State
}

impl<'a, Blueprint:MutationAutomatonBlueprint> MutationAutomaton<'a, Blueprint> {
    /// Creates a new mutation automaton instance from a blueprint.
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            current_state: blueprint.initial_mutation_state()
        }
    }

    /// Returns the classification of the current state.
    pub fn current_state_sort(&self) -> Result<Blueprint::StateSort,Blueprint::ErrorType> {
        self.blueprint.mutation_state_sort_map(&self.current_state)
    }

    /// Processes a single input symbol, updating the current state in-place.
    pub fn update_state(&mut self, character: &Blueprint::Alphabet) -> Result<(), Blueprint::ErrorType> {
        self.blueprint.mutation_transition_map(&mut self.current_state, character)
    }

    /// Processes a single input symbol and returns the new state classification.
    pub fn update_sort_state(&mut self, character: &Blueprint::Alphabet) -> Result<Blueprint::StateSort, Blueprint::ErrorType> {
        self.update_state(character)?;
        self.current_state_sort()
    }

    /// Returns a reference to the current state.
    pub fn view_state(&'a self) -> &'a Blueprint::State {
        &self.current_state
    }

    /// Consumes the automaton and returns the current state.
    pub fn take_state(self) -> Blueprint::State {
        self.current_state
    }
}

impl<Blueprint: DeterministicAutomatonBlueprint> MutationAutomatonBlueprint for Blueprint {
    type State = Blueprint::State;

    type Alphabet = Blueprint::Alphabet;

    type StateSort = Blueprint::StateSort;

    type ErrorType = Blueprint::ErrorType;

    fn initial_mutation_state(&self) -> Self::State {
        self.initial_state()
    }

    fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType> {
        self.state_sort_map(state)
    }

    fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(),Self::ErrorType> {
        *state = self.transition_map(state, character)?;
        Ok(())
    }
}
