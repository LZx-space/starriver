use crate::error::ApiError;

pub type DomainState<S> = State<S, ApiError>;

pub struct State<S, E> {
    pub state: Option<S>,
    pub error: Option<E>,
}

impl<S, E> State<S, E> {
    pub fn new(state: Option<S>, error: Option<E>) -> Self {
        Self { state, error }
    }

    pub fn with_both(state: S, error: E) -> Self {
        Self {
            state: Some(state),
            error: Some(error),
        }
    }

    pub fn with_all_none() -> Self {
        Self {
            state: None,
            error: None,
        }
    }

    pub fn with_state(state: S) -> Self {
        Self {
            state: Some(state),
            error: None,
        }
    }

    pub fn with_error(error: E) -> Self {
        Self {
            state: None,
            error: Some(error),
        }
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn map<F, U>(self, f: F) -> State<U, E>
    where
        F: FnOnce(S) -> U,
    {
        State {
            state: self.state.map(f),
            error: self.error,
        }
    }

    pub fn map_err<F, U>(self, f: F) -> State<S, U>
    where
        F: FnOnce(E) -> U,
    {
        State {
            state: self.state,
            error: self.error.map(f),
        }
    }
}

impl<S, E> From<Result<S, E>> for State<S, E> {
    fn from(result: Result<S, E>) -> Self {
        match result {
            Ok(state) => Self::new(Some(state), None),
            Err(error) => Self::new(None, Some(error)),
        }
    }
}
