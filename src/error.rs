#[derive(Debug, PartialEq, Eq)]
pub struct ErrorGatherer<E> {
    errors: Vec<E>,
}

impl<E> Default for ErrorGatherer<E> {
    fn default() -> Self {
        Self {
            errors: Default::default(),
        }
    }
}

impl<E> ErrorGatherer<E> {
    pub fn new() -> Self {
        Default::default()
    }

    fn store<Error: Into<E>>(&mut self, res: Vec<Error>) {
        self.errors.extend(res.into_iter().map(Into::into))
    }

    pub fn add<Error: Into<E>>(&mut self, error: Error) {
        self.errors.push(error.into())
    }

    #[must_use]
    pub fn gather<T, Error: Into<E>>(&mut self, res: CoralResult<T, Error>) -> Option<T> {
        match res {
            CoralResult::Ok(x) => Some(x),
            CoralResult::Err(e) => {
                self.store(e);
                None
            }
            CoralResult::Warning(x, e) => {
                self.store(e);
                Some(x)
            }
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn unrecoverable<T>(self) -> CoralResult<T, E> {
        CoralResult::Err(self.errors)
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn result<T>(self, data: T) -> CoralResult<T, E> {
        if self.errors.is_empty() {
            CoralResult::Ok(data)
        } else {
            CoralResult::Warning(data, self.errors)
        }
    }
}

#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum CoralResult<T, E> {
    Ok(T),
    Err(Vec<E>),
    Warning(T, Vec<E>),
}
