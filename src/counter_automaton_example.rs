use crate::{DeterministicAutomatonBlueprint, BasicStateSort};

/// A blueprint for an automaton that recognizes the language a^n b^n.
///
/// This automaton accepts strings consisting of n occurrences of a first symbol
/// followed by exactly n occurrences of a second symbol, for any n ≥ 0.
/// It demonstrates how the framework can handle context-free languages using
/// states that carry counter information.
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