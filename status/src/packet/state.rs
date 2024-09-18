/// State represents the desired next state of the
/// exchange.
///
/// It's a bit silly now as there's only
/// one entry, but technically there is more than
/// one type that can be sent here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Status,
}

impl From<State> for usize {
    fn from(state: State) -> Self {
        match state {
            State::Status => 1,
        }
    }
}
