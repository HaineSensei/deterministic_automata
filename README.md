# Deterministic Automata

A Rust framework for implementing deterministic and mutation automata with arbitrary state complexity.

## Overview

This crate provides a generic trait-based framework for creating deterministic and mutation automata that can handle state machines more complex than traditional finite state automata. States can carry arbitrary data, allowing recognition of some patterns beyond regular languages, and multiple automata can be composed using product constructions.

## Key Features

- **Flexible State Types**: States can be any `Clone` type, not limited to simple enums
- **Generic Alphabets**: Input symbols can be any type supporting equality comparison  
- **Dual Paradigms**: Both functional (deterministic) and in-place mutation approaches
- **Beyond Regular Languages**: Support for context-free and other complex language patterns
- **Product Constructions**: Combine multiple automata with union, intersection, and custom operations
- **Interoperability**: Seamless integration between deterministic and mutation automata
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

### Mutation Automaton

```rust
use deterministic_automata::{BasicStateSort, MutationAutomatonBlueprint};

struct Counter;

impl MutationAutomatonBlueprint for Counter {
    type State = i32;
    type Alphabet = char;
    type StateSort = BasicStateSort;
    type ErrorType = String;

    fn initial_mutation_state(&self) -> Self::State { 0 }

    fn mutation_state_sort_map(&self, state: &Self::State) -> Result<Self::StateSort, Self::ErrorType> {
        Ok(if *state >= 0 { BasicStateSort::Accept } else { BasicStateSort::Reject })
    }

    fn mutation_transition_map(&self, state: &mut Self::State, character: &Self::Alphabet) -> Result<(), Self::ErrorType> {
        match character {
            '+' => *state += 1,
            '-' => *state -= 1,
            _ => return Err("Invalid character".to_string()),
        }
        Ok(())
    }
}

let counter = Counter;
assert_eq!(counter.mutation_characterise(&['+', '+', '-']).unwrap(), BasicStateSort::Accept);
```

## Core Components

### Traits

- **`DeterministicAutomatonBlueprint`**: Functional automaton behavior with immutable state transitions  
- **`MutationAutomatonBlueprint`**: In-place automaton behavior with mutable state updates

### Modules

- **`counter_automaton_example`**: Recognizes the context-free language a^n b^n using counter-based states
- **`product_automaton`**: Product constructions including union and intersection operations for both paradigms
- **`either_automaton`**: Runtime choice between different automaton types with deterministic/mutation submodules
- **`mutation_automaton`**: Core mutation automaton types and blanket interoperability implementation

### Runtime Execution

- **`DeterministicAutomaton`**: Runtime instance for functional step-by-step input processing
- **`MutationAutomaton`**: Runtime instance for mutation-based step-by-step input processing  
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