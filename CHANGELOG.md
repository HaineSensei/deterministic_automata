# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8] - 2025-09-04

### Fixed
- Corrected changelog dates to accurately reflect actual release dates

## [0.1.7] - 2025-09-04

### Added
- CHANGELOG.md file to track changes between versions following Keep a Changelog format
- Changelog reference section in README.md

## [0.1.6] - 2025-09-04

### Added
- Dynamic dispatch support for automata with heterogeneous state types
- New `dynamic_automaton` module with `ErasedAutomatonBlueprint` and `ErasedAutomaton` traits
- Dyn-compatible traits enabling runtime polymorphism while preserving type safety for alphabet, state sort, and error types
- Re-exported `DynamicAutomatonBlueprint` and `DynamicAutomaton` types at crate root
- Comprehensive documentation and examples for dynamic dispatch functionality
- Tests demonstrating interoperability between deterministic and mutation automata via dynamic dispatch

### Changed
- Updated crate-level documentation to include dynamic dispatch examples
- Enhanced README with dynamic dispatch capabilities and examples

## [0.1.5] - 2025-09-03

### Added
- Re-exported `MutationAutomatonBlueprint` and `MutationAutomaton` at crate root for improved API ergonomics

### Changed
- Applied Clippy suggestions for code quality improvements
- Used lifetime elision where appropriate to reduce verbosity
- Fixed documentation warnings

## [0.1.4] - 2025-08-14

### Added
- Mutation automata paradigm with `MutationAutomatonBlueprint` trait for in-place state modifications
- `MutationAutomaton` runtime struct for mutation-based automata
- Automatic interoperability between deterministic and mutation automata via blanket implementations
- Product constructions for mutation automata (union, intersection, general product)
- Either types for mutation automata in `either_automaton::mutation` module

### Changed
- Refactored codebase architecture to support both deterministic and mutation paradigms
- Updated all existing functionality to work seamlessly with new mutation automata

## [0.1.3] - 2025-08-04

### Added
- `view_state()` method for read-only access to automaton's internal state
- `take_state()` method to consume automaton and extract current state
- Comprehensive documentation for new state access methods

## [0.1.2] - 2025-08-03

### Removed
- Unused `either` crate dependency

### Fixed
- Dependency issue that caused 0.1.1 to be yanked

## [0.1.1] - 2025-08-03

### Added
- `either_automaton` module with custom `Either` type
- Runtime choice between different automaton blueprint types
- Comprehensive module documentation with examples

### Changed
- Updated crate-level documentation and README to mention new module

### Note
- This version was yanked due to dependency issues, fixed in 0.1.2

## [0.1.0] - 2025-08-01

### Added
- Initial release of deterministic automata framework
- `DeterministicAutomatonBlueprint` trait for defining automaton behavior
- `DeterministicAutomaton` runtime struct for step-by-step processing
- `BasicStateSort` enum for simple Accept/Reject state classification
- `counter_automaton_example` module demonstrating context-free language recognition (a^n b^n)
- `product_automaton` module with union, intersection, and general product constructions
- Support for arbitrary state complexity beyond traditional finite state automata
- Generic alphabets and custom error types
- Comprehensive documentation and examples
- Full test coverage