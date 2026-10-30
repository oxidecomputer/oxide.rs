// The contents of this file are generated; do not modify them.

/// Error from a `TryFrom` or `FromStr` implementation.
pub struct ConversionError(::std::borrow::Cow<'static, str>);
impl ::std::error::Error for ConversionError {}
impl ::std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        ::std::fmt::Display::fmt(&self.0, f)
    }
}

impl ::std::fmt::Debug for ConversionError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.0, f)
    }
}

impl From<&'static str> for ConversionError {
    fn from(value: &'static str) -> Self {
        Self(value.into())
    }
}

impl From<String> for ConversionError {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
