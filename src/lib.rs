//! A framework for implementing deterministic automata with arbitrary state complexity.
//!
//! This crate provides a generic trait-based framework for creating deterministic automata
//! that can handle state machines more complex than traditional finite state automata.
//! States can carry arbitrary data, allowing recognition of some patterns beyond regular
//! languages.
//!
//! # Core Concepts
//!
//! - **Blueprint**: Defines the structure and behavior of an automaton through the
//!   [`DeterministicAutomatonBlueprint`] trait
//! - **State**: Can be any `Clone` type, not limited to simple enums
//! - **Alphabet**: Input symbols that can be compared for equality
//! - **StateSort**: Classification of states (e.g., Accept/Reject)
//!
//! # Example
//!
//! The included [`CounterAutomatonBlueprint`] recognizes the context-free language
//! a^n b^n (equal numbers of 'a's followed by equal numbers of 'b's):
//!
//! ```
//! use deterministic_automata::{DeterministicAutomatonBlueprint, BasicStateSort, counter_example::CounterAutomatonBlueprint};
//!
//! let blueprint = CounterAutomatonBlueprint::new('a', 'b');
//! let input: Vec<char> = "aabb".chars().collect();
//!
//! assert_eq!(blueprint.characterise(&input).unwrap(), BasicStateSort::Accept);
//! ```
//!
//! This demonstrates how the framework can handle automata with states indexed by
//! counters, going beyond traditional finite state machines while maintaining
//! deterministic behavior.

/// A blueprint for defining deterministic automata with custom state and alphabet types.
///
/// This trait allows you to define the structure and behavior of a deterministic automaton
/// by specifying how states transition, how states are classified, and what the initial
/// state should be.
///
/// # Associated Types
///
/// * `State` - The type representing internal automaton states. Must be `Clone`.
/// * `Alphabet` - The type of input symbols. Must support equality comparison.
/// * `StateSort` - The classification type for states (e.g., Accept/Reject).
/// * `ErrorType` - The type used for error handling when states are invalid.
///
/// # Error Handling
///
/// The `Result` return types in [`state_sort_map`](Self::state_sort_map) and 
/// [`transition_map`](Self::transition_map) are intended for runtime validation of state
/// invariants. If your `State` type represents a refinement of a broader type space,
/// these methods can return errors when encountering invalid states that have somehow
/// escaped the intended state machine constraints.
///
/// # Required Methods
///
/// * [`initial_state`](Self::initial_state) - Returns the starting state
/// * [`state_sort_map`](Self::state_sort_map) - Classifies a state, with validation
/// * [`transition_map`](Self::transition_map) - Defines state transitions, with validation
///
/// # Provided Methods
///
/// * [`characterise`](Self::characterise) - Processes an entire input sequence
pub trait DeterministicAutomatonBlueprint {
    /// The type representing internal automaton states.
    ///
    /// States can carry arbitrary data and are not limited to simple enumerations.
    /// This allows for automata with unbounded state spaces.
    type State: Clone;
    
    /// The type of input symbols that the automaton processes.
    type Alphabet: PartialEq;
    
    /// The classification type for states.
    ///
    /// Typically used to distinguish between accepting and rejecting states,
    /// but can represent any state categorization scheme.
    type StateSort;
    
    /// The error type returned when state validation fails.
    ///
    /// Used to signal when a state is invalid or violates automaton invariants.
    type ErrorType;

    /// Returns the initial state of the automaton.
    fn initial_state(&self) -> Self::State;

    /// Maps a state to its classification, with validation.
    ///
    /// This function determines what kind of state the given state represents.
    /// Returns an error if the provided state is invalid.
    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort,Self::ErrorType>;

    /// Defines the transition function with state validation.
    ///
    /// Given a current state and an input symbol, returns the next state.
    /// Returns an error if the current state is invalid or if the transition
    /// would produce an invalid state.
    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType>;

    /// Processes an entire input sequence and returns the final state classification.
    ///
    /// Creates a runtime automaton, processes the input sequence, and returns
    /// the classification of the final state. Propagates any validation errors
    /// encountered during state transitions.
    fn characterise(&self, word: &[Self::Alphabet]) -> Result<Self::StateSort, Self::ErrorType>
    where
        Self: Sized
    {
        let mut automaton: DeterministicAutomaton<'_, Self> = DeterministicAutomaton::new(self);
        for character in word {
            automaton.update_state(character)?;
        }
        automaton.current_state_sort()
    }
}

/// A runtime instance of a deterministic automaton.
///
/// This struct represents an automaton in execution, maintaining the current state
/// and providing methods to process input symbols one at a time. It borrows a
/// blueprint that defines the automaton's behavior.
///
/// # Lifetime
///
/// The automaton holds a reference to its blueprint for the lifetime `'a`, ensuring
/// the blueprint remains valid while the automaton is in use.
pub struct DeterministicAutomaton<'a, Blueprint: DeterministicAutomatonBlueprint> {
    blueprint: &'a Blueprint,
    current_state: Blueprint::State,
}

impl<'a, Blueprint> DeterministicAutomaton<'a, Blueprint>
where
    Blueprint: DeterministicAutomatonBlueprint
{
    /// Creates a new automaton instance from a blueprint.
    ///
    /// The automaton starts in the initial state defined by the blueprint.
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            current_state: blueprint.initial_state()
        }
    }

    /// Returns the classification of the current state.
    ///
    /// This method queries the blueprint to determine what kind of state
    /// the automaton is currently in (e.g., accepting or rejecting).
    pub fn current_state_sort(&self) -> Result<Blueprint::StateSort,Blueprint::ErrorType> {
        self.blueprint.state_sort_map(&self.current_state)
    }

    /// Processes a single input symbol and updates the automaton's state.
    ///
    /// This method transitions the automaton to the next state based on the
    /// current state and the input symbol, then returns the classification
    /// of the new state.
    ///
    /// # Returns
    ///
    /// The state classification after the transition, or an error if the
    /// transition or state validation fails.
    pub fn update_state(&mut self, character: &Blueprint::Alphabet) -> Result<Blueprint::StateSort, Blueprint::ErrorType> {
        let next_state: <Blueprint as DeterministicAutomatonBlueprint>::State = self.blueprint.transition_map(&self.current_state, character)?;
        self.current_state = next_state;
        self.blueprint.state_sort_map(&self.current_state)
    }
}

/// Basic binary classification for automaton states.
///
/// This simple enum distinguishes between accepting and rejecting states,
/// suitable for recognizing formal languages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BasicStateSort {
    /// The state accepts the input string.
    Accept, 
    
    /// The state rejects the input string.
    Reject
}

pub mod counter_example;

#[cfg(test)]
mod tests;
