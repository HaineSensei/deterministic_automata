//! Product construction and boolean operations for deterministic automata.
//!
//! This module provides blueprints for combining multiple automata using the Cartesian
//! product of their state spaces. The product construction allows for parallel execution
//! of multiple automata on the same input, enabling complex language recognition patterns.
//!
//! # Core Concept: Product Construction
//!
//! The product construction takes two automata A and B and creates a new automaton whose
//! states are pairs (state_A, state_B). On each input symbol, both component automata
//! transition simultaneously, and the product automaton's new state is the pair of
//! resulting states.
//!
//! # Blueprints Provided
//!
//! ## [`ProductAutomatonBlueprint`]
//! 
//! The general product construction that preserves both component state sorts as a tuple.
//! This is useful when you need access to the individual classifications from both
//! component automata.
//!
//! ## [`BasicUnionAutomatonBlueprint`]
//!
//! A specialized product construction for automata using [`BasicStateSort`]. Implements
//! the logical OR operation: accepts if **either** component automaton accepts.
//! This recognizes the union of the languages accepted by the component automata.
//!
//! ## [`BasicIntersectionAutomatonBlueprint`]
//!
//! A specialized product construction for automata using [`BasicStateSort`]. Implements
//! the logical AND operation: accepts only if **both** component automata accept.
//! This recognizes the intersection of the languages accepted by the component automata.
//!
//! # Boolean Operations on Languages
//!
//! The union and intersection blueprints provide a way to perform boolean operations
//! on the languages recognized by deterministic automata:
//!
//! - **Union (OR)**: `L(A) ∪ L(B)` - strings accepted by A or B (or both)
//! - **Intersection (AND)**: `L(A) ∩ L(B)` - strings accepted by both A and B
//!
//! These operations are closed for the class of languages recognizable by deterministic
//! automata in this framework, meaning the result is always another recognizable language.

use crate::{BasicStateSort, DeterministicAutomatonBlueprint};

/// A blueprint for the general product construction of two deterministic automata.
///
/// This blueprint implements the Cartesian product of two automata, creating a new
/// automaton that runs both component automata in parallel. The resulting automaton's
/// state space is the product of the component state spaces, and its state sort
/// preserves both component classifications as a tuple.
///
/// # Type Parameters
///
/// * `A`, `B` - The component automaton blueprint types
/// * `Alphabet` - The input symbol type (must be the same for both automata)
/// * `ErrorType` - The error type (must be the same for both automata)
///
/// # State and Behavior
///
/// * **State**: `(A::State, B::State)` - Pairs of component states
/// * **StateSort**: `(A::StateSort, B::StateSort)` - Pairs of component classifications
/// * **Transitions**: Both component automata transition simultaneously
///
/// # Use Cases
///
/// This general product construction is useful when you need:
/// - Access to individual state classifications from both component automata
/// - Custom logic based on the combination of component states
/// - Building blocks for more specialized product constructions
///
/// For simple boolean operations on languages recognized by automata with
/// [`BasicStateSort`], consider using [`BasicUnionAutomatonBlueprint`] or
/// [`BasicIntersectionAutomatonBlueprint`] instead.
///
/// # Construction
///
/// Use [`new`](Self::new) to create an instance from two component blueprint references.
pub struct ProductAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    first: &'a A,
    second: &'b B
}

impl<'a, 'b, A, B, Alphabet, ErrorType> ProductAutomatonBlueprint<'a, 'b, A, B, Alphabet, ErrorType>
where
    A: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    B: DeterministicAutomatonBlueprint<Alphabet = Alphabet, ErrorType = ErrorType>,
    Alphabet: PartialEq
{
    /// Creates a new product automaton blueprint from two component blueprints.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first component automaton blueprint
    /// * `second` - Reference to the second component automaton blueprint
    ///
    /// # Returns
    ///
    /// A new product blueprint that preserves both component state classifications
    /// as a tuple, allowing access to individual automaton results.
    pub fn new(first: &'a A, second: &'b B) -> Self {
        Self {
            first,
            second
        }
    }
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


/// A blueprint for the union (logical OR) of two automata with [`BasicStateSort`].
///
/// This blueprint creates an automaton that accepts a string if **either** of the
/// component automata accepts it, implementing the union of their recognized languages:
/// `L(A) ∪ L(B)`.
///
/// # Boolean Logic
///
/// The state classification follows logical OR semantics:
/// - `Accept OR Accept → Accept`
/// - `Accept OR Reject → Accept`  
/// - `Reject OR Accept → Accept`
/// - `Reject OR Reject → Reject`
///
/// # Type Parameters
///
/// * `A`, `B` - Component automaton blueprints (must use [`BasicStateSort`])
/// * `Alphabet` - Input symbol type (shared by both automata)
/// * `ErrorType` - Error type (shared by both automata)
///
/// # Example Use Cases
///
/// - Recognizing strings that match any of several patterns
/// - Combining multiple validation rules with OR logic
/// - Building composite language recognizers from simpler components
///
/// # Construction
///
/// Use [`new`](Self::new) to create an instance from two component blueprint references.
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
    /// Creates a new union automaton blueprint from two component blueprints.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first component automaton blueprint
    /// * `second` - Reference to the second component automaton blueprint
    ///
    /// # Returns
    ///
    /// A new union blueprint that accepts strings accepted by either component.
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

/// A blueprint for the intersection (logical AND) of two automata with [`BasicStateSort`].
///
/// This blueprint creates an automaton that accepts a string only if **both** of the
/// component automata accept it, implementing the intersection of their recognized
/// languages: `L(A) ∩ L(B)`.
///
/// # Boolean Logic
///
/// The state classification follows logical AND semantics:
/// - `Accept AND Accept → Accept`
/// - `Accept AND Reject → Reject`
/// - `Reject AND Accept → Reject`
/// - `Reject AND Reject → Reject`
///
/// # Type Parameters
///
/// * `A`, `B` - Component automaton blueprints (must use [`BasicStateSort`])
/// * `Alphabet` - Input symbol type (shared by both automata)
/// * `ErrorType` - Error type (shared by both automata)
///
/// # Example Use Cases
///
/// - Recognizing strings that must satisfy multiple constraints simultaneously
/// - Combining validation rules with AND logic
/// - Finding the common subset of languages recognized by different automata
/// - Building strict composite validators
///
/// # Construction
///
/// Use [`new`](Self::new) to create an instance from two component blueprint references.
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
    /// Creates a new intersection automaton blueprint from two component blueprints.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first component automaton blueprint
    /// * `second` - Reference to the second component automaton blueprint
    ///
    /// # Returns
    ///
    /// A new intersection blueprint that accepts strings accepted by both components.
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