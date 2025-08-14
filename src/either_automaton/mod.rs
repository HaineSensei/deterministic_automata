//! Either automaton implementation for runtime choice between two automata blueprints.
//!
//! This module provides `Either` types for both deterministic and mutation automaton paradigms,
//! allowing you to create blueprints that represent a choice between two different automaton types.
//! This enables runtime selection between automata while maintaining compile-time type safety.
//!
//! # Submodules
//!
//! * [`deterministic`] - Either type for deterministic automaton blueprints
//! * [`mutation`] - Either type for mutation automaton blueprints
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

pub mod deterministic;
pub mod mutation;