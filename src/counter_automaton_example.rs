//! Example automaton that recognizes the context-free language a^n b^n.
//!
//! This module provides a concrete example of how the deterministic automata framework
//! can handle languages beyond regular languages by using states that carry additional
//! data (counters). The [`CounterAutomatonBlueprint`] demonstrates recognition of the
//! context-free language a^n b^n, which cannot be recognized by traditional finite
//! state automata.
//!
//! # The Language a^n b^n
//!
//! The language a^n b^n consists of strings with:
//! - Zero or more occurrences of symbol 'a'
//! - Followed by exactly the same number of occurrences of symbol 'b'
//! - For any n ≥ 0
//!
//! Examples of strings in this language:
//! - `""` (empty string, n=0)
//! - `"ab"` (n=1)
//! - `"aabb"` (n=2)
//! - `"aaabbb"` (n=3)
//!
//! Examples of strings NOT in this language:
//! - `"a"` (unbalanced)
//! - `"ba"` (wrong order)
//! - `"aab"` (unequal counts)
//! - `"abab"` (interleaved)
//!
//! # Key Insight: Beyond Regular Languages
//!
//! This example is significant because it demonstrates how the framework can recognize
//! languages that are provably impossible for traditional finite state automata to
//! handle. The key insight is that states can carry arbitrary data - in this case,
//! a counter that tracks the balance between 'a' and 'b' symbols.
//!
//! # State Machine Design
//!
//! The automaton uses three types of states:
//! - [`CounterState::Start(n)`] - Reading 'a' symbols, counter tracks how many seen
//! - [`CounterState::End(n)`] - Reading 'b' symbols, counter tracks how many more needed
//! - [`CounterState::Reject`] - Invalid input detected
//!
//! The state space is theoretically infinite (counters can grow arbitrarily large),
//! but the automaton remains deterministic and efficiently processable.
//!
//! # Framework Benefits
//!
//! This example showcases several advantages of the framework:
//! - **Expressiveness**: Can handle non-regular languages
//! - **Determinism**: No backtracking or ambiguity in state transitions
//! - **Composability**: Can be combined with other automata using product operations
//! - **Type Safety**: Counter overflow could be caught at runtime depending on build configuration

use crate::{DeterministicAutomatonBlueprint, BasicStateSort};

/// A blueprint for an automaton that recognizes the language a^n b^n.
///
/// This automaton accepts strings consisting of n occurrences of a first symbol
/// followed by exactly n occurrences of a second symbol, for any n ≥ 0.
/// It demonstrates how the framework can handle context-free languages using
/// states that carry counter information.
#[derive(Debug, Clone, PartialEq)]
pub struct CounterAutomatonBlueprint<Alphabet> {
    first: Alphabet,
    second: Alphabet
}

impl<Alphabet> CounterAutomatonBlueprint<Alphabet> {
    /// Creates a new counter automaton blueprint.
    ///
    /// # Parameters
    ///
    /// * `first` - The symbol that must appear first (the 'a' in a^n b^n)
    /// * `second` - The symbol that must appear second (the 'b' in a^n b^n)
    pub fn new(first: Alphabet, second: Alphabet) -> Self {
        Self { first, second }
    }
}

/// The state type for the counter automaton.
///
/// This enum represents the different phases of processing input in the a^n b^n
/// language recognizer, with states carrying counter information.
#[derive(Clone)]
pub enum CounterState {
    /// Reading the first symbol ('a'), counting occurrences.
    ///
    /// The `usize` value tracks how many first symbols have been seen.
    Start(usize),
    
    /// Reading the second symbol ('b'), counting down.
    ///
    /// The `usize` value tracks how many more second symbols are needed
    /// to match the count of first symbols.
    End(usize),
    
    /// Invalid input detected - the automaton has rejected the string.
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
            CounterState::Reject => return Ok(BasicStateSort::Reject)
        } {
            0 => Ok(BasicStateSort::Accept),
            _ => Ok(BasicStateSort::Reject)
        }
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        Ok(match state {
            CounterState::Start(counter) => {
                if *character == self.first {
                    CounterState::Start(counter+1)
                } else if *character == self.second && *counter > 0 {
                    CounterState::End(*counter - 1)
                } else {
                    CounterState::Reject
                }
            },
            CounterState::End(counter) => {
                if *character == self.second && *counter > 0 {
                    CounterState::End(counter-1)
                } else {
                    CounterState::Reject
                }
            },
            CounterState::Reject => CounterState::Reject,
        })
    }
}