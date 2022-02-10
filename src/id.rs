/// `Id` is a Graphviz `ID`.
pub struct Id<'a> {
    pub(crate) name: std::borrow::Cow<'a, str>,
}

impl<'a> Id<'a> {
    /// Creates an `Id` named `name`.
    ///
    /// The caller must ensure that the input conforms to an
    /// identifier format: it must be a non-empty string made up of
    /// alphanumeric or underscore characters, not beginning with a
    /// digit (i.e., the regular expression `[a-zA-Z_][a-zA-Z_0-9]*`).
    ///
    /// (Note: this format is a strict subset of the `ID` format
    /// defined by the DOT language. This function may change in the
    /// future to accept a broader subset, or the entirety, of DOT's
    /// `ID` format.)
    ///
    /// Passing an invalid string (containing spaces, brackets,
    /// quotes, ...) will return an empty `Err` value.
    pub fn new<Name: Into<std::borrow::Cow<'a, str>>>(name: Name) -> crate::Result<Self> {
        let name = name.into();

        match name.chars().next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
            _ => return Err(crate::Error::InvalidId),
        }

        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(crate::Error::InvalidId);
        }

        Ok(Self { name })
    }
}

impl<'a> std::fmt::Display for Id<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
