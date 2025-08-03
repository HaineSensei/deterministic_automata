# Deterministic Automata

A Rust framework for implementing deterministic automata with arbitrary state complexity.

## Overview

This crate provides a generic trait-based framework for creating deterministic automata that can handle state machines more complex than traditional finite state automata. States can carry arbitrary data, allowing recognition of some patterns beyond regular languages, and multiple automata can be composed using product constructions.

## Key Features

- **Flexible State Types**: States can be any `Clone` type, not limited to simple enums
- **Generic Alphabets**: Input symbols can be any type supporting equality comparison  
- **Beyond Regular Languages**: Support for context-free and other complex language patterns
- **Product Constructions**: Combine multiple automata with union, intersection, and custom operations
- **Type-Safe Error Handling**: Comprehensive validation with custom error types

## Quick Start

### Basic Finite State Automaton

```rust
use deterministic_automata::{DeterministicAutomatonBlueprint, BasicStateSort};

#[derive(Clone, PartialEq, Debug)]
enum State {
    Start,
    SawA,
    AcceptAB,
}

struct EndsWithAB;

impl DeterministicAutomatonBlueprint for EndsWithAB {
    type State = State;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_state(&self) -> Self::State { State::Start }

    fn state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        Ok(match state {
            State::AcceptAB => BasicStateSort::Accept,
            _ => BasicStateSort::Reject,
        })
    }

    fn transition_map(&self, state: &Self::State, character: &Self::Alphabet) -> Result<Self::State, Self::ErrorType> {
        Ok(match (state, character) {
            (State::Start, 'a') => State::SawA,
            (State::Start, _) => State::Start,
            (State::SawA, 'b') => State::AcceptAB,
            (State::SawA, 'a') => State::SawA,
            (State::SawA, _) => State::Start,
            (State::AcceptAB, 'a') => State::SawA,
            (State::AcceptAB, _) => State::Start,
        })
    }
}

// Usage
let dfa = EndsWithAB;
assert_eq!(dfa.characterise(&"cab".chars().collect::<Vec<_>>()).unwrap(), BasicStateSort::Accept);
```

### Context-Free Language Recognition

```rust
use deterministic_automata::counter_automaton_example::CounterAutomatonBlueprint;

let blueprint = CounterAutomatonBlueprint::new('a', 'b');
assert_eq!(blueprint.characterise(&"aabb".chars().collect()).unwrap(), BasicStateSort::Accept);
```

### Combining Automata

```rust
use deterministic_automata::product_automaton::BasicUnionAutomatonBlueprint;

let a_blueprint = CounterAutomatonBlueprint::new('a', 'b');
let b_blueprint = CounterAutomatonBlueprint::new('x', 'y');
let union = BasicUnionAutomatonBlueprint::new(&a_blueprint, &b_blueprint);

// Accepts strings from either language
assert_eq!(union.characterise(&"aabb".chars().collect()).unwrap(), BasicStateSort::Accept);
assert_eq!(union.characterise(&"xxyy".chars().collect()).unwrap(), BasicStateSort::Accept);
```

## Core Components

### `DeterministicAutomatonBlueprint` Trait

The main trait for defining automaton behavior with associated types for states, alphabet, state classification, and errors.

### Provided Examples

- **`counter_automaton_example`**: Recognizes the context-free language a^n b^n using counter-based states
- **`product_automaton`**: Product constructions including union and intersection operations
- **`either_automaton`**: Runtime choice between two different automaton blueprint types using a sum type

### Runtime Execution

- **`DeterministicAutomaton`**: Runtime instance for step-by-step input processing
- **`BasicStateSort`**: Simple Accept/Reject state classification

## Testing

The crate includes comprehensive integration tests covering:

- Core framework functionality
- All provided automaton implementations  
- Product construction operations
- Error handling and edge cases

Run tests with:

```bash
cargo test
```

## Documentation

Generate and view the full documentation:

```bash
cargo doc --open
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Documentation and comprehensive test suite contributions by Claude (Anthropic). All core automata logic and framework design by the original author.