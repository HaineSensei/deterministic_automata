//! Dynamic dispatch for automaton blueprints with heterogeneous state types.
//!
//! This module provides dyn-compatible traits that enable runtime polymorphism over
//! automata with different internal state structures while working within the same
//! formal language context (same alphabet, state classification, and error handling).
//!
//! # The State Type Erasure Problem
//!
//! The `State` associated type is inherently part of each automaton's structural
//! definition - different automaton implementations use fundamentally different
//! state representations (integers, enums, complex structs, etc.). This makes
//! the main automaton traits incompatible with `dyn` trait objects.
//!
//! However, when working with formal languages, we typically operate within
//! consistent contexts where the `Alphabet`, `StateSort`, and `ErrorType` remain
//! the same across different automaton implementations.
//!
//! # Solution: State-Focused Erasure
//!
//! This module provides companion traits that erase only the `State` type through
//! dynamic dispatch, while keeping other types as concrete generic parameters:
//!
//! - **[`ErasedAutomatonBlueprint`]**: Dyn-compatible blueprint trait
//! - **[`ErasedAutomaton`]**: Dyn-compatible runtime automaton trait
//! - **Universal Coverage**: All mutation automata (including deterministic ones) work seamlessly
//!
//! # Example: Heterogeneous State Types in Same Language Context
//!
//! ```
//! use deterministic_automata::{BasicStateSort, DeterministicAutomatonBlueprint, MutationAutomatonBlueprint, DynamicAutomatonBlueprint};
//!
//! # struct CounterAutomaton;
//! # impl DeterministicAutomatonBlueprint for CounterAutomaton {
//! #     type State = i32; type Alphabet = char; type StateSort = BasicStateSort; type ErrorType = String;
//! #     fn initial_state(&self) -> Self::State { 0 }
//! #     fn state_sort_map(&self, _: &Self::State) -> Result<Self::StateSort, Self::ErrorType> { Ok(BasicStateSort::Accept) }
//! #     fn transition_map(&self, state: &Self::State, _: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> { Ok(*state) }
//! # }
//! # 
//! # #[derive(Clone)] enum PatternState { Start, Found }
//! # struct PatternAutomaton;
//! # impl DeterministicAutomatonBlueprint for PatternAutomaton {
//! #     type State = PatternState; type Alphabet = char; type StateSort = BasicStateSort; type ErrorType = String;
//! #     fn initial_state(&self) -> Self::State { PatternState::Start }
//! #     fn state_sort_map(&self, _: &Self::State) -> Result<Self::StateSort, Self::ErrorType> { Ok(BasicStateSort::Accept) }
//! #     fn transition_map(&self, state: &Self::State, _: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> { Ok(state.clone()) }
//! # }
//!
//! let counter = CounterAutomaton;    // Uses i32 state for counting
//! let pattern = PatternAutomaton;    // Uses enum state for pattern matching
//!
//! // Same language context: char alphabet, BasicStateSort classification, String errors
//! let automata: Vec<&DynamicAutomatonBlueprint<char, BasicStateSort, String>> = vec![
//!     &counter,  // Different state structure, same language interface
//!     &pattern,  // Different state structure, same language interface
//! ];
//!
//! // Dynamic dispatch over different state implementations
//! for automaton in automata {
//!     let result = automaton.characterise(&['a', 'b']);
//!     // Each uses its own state structure internally
//! }
//! ```

use crate::{MutationAutomaton, MutationAutomatonBlueprint};

/// A dyn-compatible blueprint for defining automata with erased state types.
///
/// This trait enables dynamic dispatch over automata with different internal state
/// structures while maintaining compile-time knowledge of alphabet, state classification,
/// and error types. It serves as a companion to the main automaton blueprint traits,
/// solving the dyn compatibility problem by using generic type parameters instead
/// of associated types.
///
/// # Associated Types
///
/// * `Alphabet` - The type of input symbols. Must support equality comparison.
/// * `StateSort` - The classification type for states (e.g., Accept/Reject).
/// * `ErrorType` - The type used for error handling when states are invalid.
///
/// # Required Methods
///
/// * [`automaton`](Self::automaton) - Creates a runtime automaton instance with erased state
/// * [`characterise`](Self::characterise) - Processes an entire input sequence
///
/// # Universal Implementation
///
/// All types implementing [`MutationAutomatonBlueprint`] (including deterministic
/// automata via blanket implementation) automatically implement this trait,
/// providing seamless interoperability between different automaton paradigms.
pub trait ErasedAutomatonBlueprint {
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

    /// Creates a runtime automaton instance with erased state type.
    ///
    /// Returns a boxed trait object that can process input symbols one at a time
    /// while hiding the specific state type implementation. The automaton starts
    /// in the initial state defined by the underlying blueprint.
    fn automaton<'a>(&'a self) -> Box<dyn ErasedAutomaton<'a, Alphabet = Self::Alphabet, StateSort = Self::StateSort, ErrorType = Self::ErrorType> + 'a>;

    /// Processes an entire input sequence and returns the final state classification.
    ///
    /// Creates a runtime automaton, processes the input sequence, and returns
    /// the classification of the final state. Propagates any validation errors
    /// encountered during state transitions.
    fn characterise(&self, word: &[Self::Alphabet]) -> Result<Self::StateSort, Self::ErrorType>;
}

/// A dyn-compatible runtime instance of an automaton with erased state type.
///
/// This trait represents an automaton in execution, maintaining the current state
/// internally while providing methods to process input symbols one at a time.
/// The specific state type is erased, enabling dynamic dispatch over automata
/// with different state representations.
///
/// # Associated Types
///
/// * `Alphabet` - The type of input symbols that the automaton processes.
/// * `StateSort` - The classification type for states (e.g., Accept/Reject).
/// * `ErrorType` - The type used for error handling when states are invalid.
///
/// # Required Methods
///
/// * [`update_state`](Self::update_state) - Processes a single input symbol
/// * [`current_state_sort`](Self::current_state_sort) - Returns the current state classification
///
/// # Provided Methods
///
/// * [`update_sort_state`](Self::update_sort_state) - Updates state and returns classification in one call
pub trait ErasedAutomaton<'a> {
    /// The type of input symbols that the automaton processes.
    type Alphabet: PartialEq;
    
    /// The error type returned when state validation fails.
    ///
    /// Used to signal when a state is invalid or violates automaton invariants.
    type ErrorType;
    
    /// The classification type for states.
    ///
    /// Typically used to distinguish between accepting and rejecting states,
    /// but can represent any state categorization scheme.
    type StateSort;

    /// Processes a single input symbol and updates the automaton's state.
    ///
    /// This method transitions the automaton to the next state based on the
    /// current state and the input symbol. The specific state representation
    /// is hidden behind the trait abstraction.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the transition succeeds, or an error if the transition
    /// or state validation fails.
    fn update_state(&mut self, character: &Self::Alphabet) -> Result<(), Self::ErrorType>;

    /// Returns the classification of the current state.
    ///
    /// This method queries the underlying implementation to determine what kind
    /// of state the automaton is currently in (e.g., accepting or rejecting).
    /// The specific state data remains hidden behind the trait abstraction.
    fn current_state_sort(&self) -> Result<Self::StateSort,Self::ErrorType>;

    /// Processes a single input symbol and returns the new state classification.
    ///
    /// This convenience method combines [`update_state`](Self::update_state) and
    /// [`current_state_sort`](Self::current_state_sort) in a single call, updating
    /// the automaton's state and immediately returning its classification.
    ///
    /// # Returns
    ///
    /// The state classification after the transition, or an error if the
    /// transition or state validation fails.
    fn update_sort_state(&mut self, character: &Self::Alphabet) -> Result<Self::StateSort, Self::ErrorType> {
        self.update_state(character)?;
        self.current_state_sort()
    }
}

impl<'a, Blueprint: MutationAutomatonBlueprint> ErasedAutomaton<'a> for MutationAutomaton<'a, Blueprint> {
    type Alphabet = Blueprint::Alphabet;

    type ErrorType = Blueprint::ErrorType;

    type StateSort = Blueprint::StateSort;

    fn update_state(&mut self, character: &Self::Alphabet) -> Result<(), Self::ErrorType> {
        self.update_state(character)
    }

    fn current_state_sort(&self) -> Result<Self::StateSort,Self::ErrorType> {
        self.current_state_sort()
    }
}

impl<Blueprint: MutationAutomatonBlueprint> ErasedAutomatonBlueprint for Blueprint {
    type Alphabet = Blueprint::Alphabet;

    type StateSort = Blueprint::StateSort;

    type ErrorType = Blueprint::ErrorType;

    fn automaton<'a>(&'a self) -> Box<dyn ErasedAutomaton<'a, Alphabet = Self::Alphabet, StateSort = Self::StateSort, ErrorType = Self::ErrorType> + 'a> {
        Box::new(self.mutation_automaton())
    }

    fn characterise(&self, word: &[Self::Alphabet]) -> Result<Self::StateSort, Self::ErrorType> {
        self.mutation_characterise(word)
    }
}

pub type DynamicAutomatonBlueprint<Alphabet,StateSort,ErrorType> = dyn ErasedAutomatonBlueprint<Alphabet = Alphabet, StateSort = StateSort, ErrorType = ErrorType>;

pub type DynamicAutomaton<'a,Alphabet,StateSort,ErrorType> = dyn ErasedAutomaton<'a,Alphabet = Alphabet, ErrorType = ErrorType, StateSort = StateSort>;
