use std::cmp::Ordering;

/// Allows the HeaderKey to take the form of a variety of different
/// valid Types, mostly related to their lifetimes.
/// This however also gives more control over how they are compared
/// to each other, ignoring case in this case
///
/// ```rust
/// use stream_httparse::header::HeaderKey;
///
/// assert_eq!(HeaderKey::StrRef("TeSt"), HeaderKey::StrRef("test"));
/// ```
#[derive(Debug, Clone)]
pub enum HeaderKey<'a> {
    /// Stores the Key as a refernce to a String
    StrRef(&'a str),
    /// Stores the Key as an owned String
    Str(String),
}

impl<'a> From<&'a str> for HeaderKey<'a> {
    fn from(val: &'a str) -> Self {
        HeaderKey::StrRef(val)
    }
}
impl<'a> From<String> for HeaderKey<'a> {
    fn from(val: String) -> Self {
        HeaderKey::Str(val)
    }
}

impl<'a> HeaderKey<'a> {
    /// Serializes the Key into the Buffer by appending
    /// the Data to it
    pub fn serialize(&self, buf: &mut Vec<u8>) {
        match *self {
            Self::StrRef(ref value) => {
                buf.extend_from_slice(value.as_bytes());
            }
            Self::Str(ref value) => {
                buf.extend_from_slice(value.as_bytes());
            }
        }
    }

    /// Clones all the needed Data in order to create a new
    /// HeaderKey that is completly independant of the given
    /// self reference
    pub fn to_owned<'refed, 'owned>(&'refed self) -> HeaderKey<'owned> {
        let value = match self {
            Self::StrRef(tmp) => tmp.to_string(),
            Self::Str(tmp) => tmp.to_owned(),
        };

        HeaderKey::Str(value)
    }
}

impl AsRef<str> for HeaderKey<'_> {
    fn as_ref(&self) -> &str {
        match *self {
            Self::Str(ref value) => &value,
            Self::StrRef(ref value) => value,
        }
    }
}

impl PartialEq for HeaderKey<'_> {
    fn eq(&self, other: &Self) -> bool {
        caseless::default_caseless_match_str(self.as_ref(), other.as_ref())
    }
}

impl Eq for HeaderKey<'_> {}

impl PartialOrd for HeaderKey<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Ord for HeaderKey<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_ignore_case() {
        assert_eq!(HeaderKey::StrRef("test"), HeaderKey::StrRef("test"));
        assert_eq!(HeaderKey::StrRef("TEST"), HeaderKey::StrRef("test"));
        assert_eq!(HeaderKey::StrRef("TeSt"), HeaderKey::StrRef("test"));
    }

    #[test]
    fn serialize_str() {
        let mut result: Vec<u8> = Vec::new();
        HeaderKey::Str("test-key".to_owned()).serialize(&mut result);

        assert_eq!("test-key".as_bytes(), &result);
    }
    #[test]
    fn serialize_str_ref() {
        let mut result: Vec<u8> = Vec::new();
        HeaderKey::StrRef("test-key").serialize(&mut result);

        assert_eq!("test-key".as_bytes(), &result);
    }

    #[test]
    fn partial_ord() {
        assert_eq!(
            "first" < "second",
            HeaderKey::StrRef("first") < HeaderKey::StrRef("second")
        );
    }
}
