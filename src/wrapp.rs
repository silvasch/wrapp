//! A simple wrapper for errors that allows you to easily chain errors.

/// A simple wrapper for errors that allows you to easily chain errors.
pub struct Wrapp<E>
where
    E: std::error::Error,
{
    /// The error that is being wrapped.
    error: E,
    /// The (optional) source of the error.
    source: Option<Box<dyn std::error::Error>>,
}

impl<E> Wrapp<E>
where
    E: std::error::Error + 'static,
{
    /// Create a new `Wrapp` from an error.
    pub fn new(error: E) -> Self {
        Self {
            error,
            source: None,
        }
    }

    /// Add a source to the error.
    pub fn with_source(mut self, source: Box<dyn std::error::Error>) -> Self {
        self.source = Some(source);
        self
    }

    /// Print the error with the full chain of sources.
    pub fn full_display(&self) -> String {
        full_display(self)
    }
}

/// Recursively display the error and its sources.
fn full_display(error: &dyn std::error::Error) -> String {
    match error.source() {
        Some(source) => format!("{}\nfrom: {}", error, full_display(source)),
        None => error.to_string(),
    }
}

impl<E> std::fmt::Display for Wrapp<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<E> std::fmt::Debug for Wrapp<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.error)
        } else {
            write!(f, "{:?}", self.error)
        }
    }
}

impl<E> std::error::Error for Wrapp<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}
