pub struct ThatError<E>
where
    E: std::error::Error,
{
    error: E,
    source: Option<Box<dyn std::error::Error>>,
}

impl<E> ThatError<E>
where
    E: std::error::Error,
{
    pub fn new(error: E) -> Self {
        Self {
            error,
            source: None,
        }
    }

    pub fn with_source(mut self, source: Box<dyn std::error::Error>) -> Self {
        self.source = Some(source);
        self
    }
}

impl<E> std::fmt::Display for ThatError<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)?;
        if let Some(source) = &self.source {
            write!(f, "\nfrom: {}", source)?;
        }
        Ok(())
    }
}

impl<E> std::fmt::Debug for ThatError<E>
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

impl<E> std::error::Error for ThatError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}
