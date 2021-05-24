/// A single HeaderValue that can hold Data
/// in a variety of forms allowing for easier
/// and more flexible use
#[derive(Debug, PartialEq, Clone)]
pub enum HeaderValue<'a> {
    /// Stores the Value as a reference to a String
    StrRef(&'a str),
    /// Stores the Value as an owned String
    Str(String),
    /// Stores the Value in its raw Number format
    NumberUsize(usize),
}

impl<'a> From<&'a str> for HeaderValue<'a> {
    fn from(val: &'a str) -> Self {
        HeaderValue::StrRef(val)
    }
}
impl<'a> From<String> for HeaderValue<'a> {
    fn from(val: String) -> Self {
        HeaderValue::Str(val)
    }
}
impl<'a> From<usize> for HeaderValue<'a> {
    fn from(val: usize) -> Self {
        HeaderValue::NumberUsize(val)
    }
}

impl<'a> HeaderValue<'a> {
    /// Serializes the Value into the given Buffer by
    /// appending the Data to it
    pub fn serialize(&self, buf: &mut Vec<u8>) {
        match *self {
            Self::StrRef(ref value) => {
                buf.extend_from_slice(value.as_bytes());
            }
            Self::Str(ref value) => {
                buf.extend_from_slice(value.as_bytes());
            }
            Self::NumberUsize(ref value) => {
                buf.extend_from_slice(value.to_string().as_bytes());
            }
        }
    }

    /// Turns the given Value, regardless of how it is stored,
    /// into an owned String
    pub fn to_string(&self) -> String {
        match *self {
            Self::StrRef(ref value) => value.to_string(),
            Self::Str(ref value) => value.clone(),
            Self::NumberUsize(ref value) => value.to_string(),
        }
    }

    /// Compares the Two values without case
    ///
    /// Any number type in either of them immediately
    /// returns false
    pub fn eq_ignore_case(&self, other: &Self) -> bool {
        let own_ref = match self.try_as_str_ref() {
            Some(r) => r,
            None => return false,
        };

        let other_ref = match other.try_as_str_ref() {
            Some(r) => r,
            None => return false,
        };

        caseless::default_caseless_match_str(own_ref, other_ref)
    }

    /// Tries to return a reference to the underlying String,
    /// if it is a String, otherwise returns None
    pub fn try_as_str_ref(&self) -> Option<&str> {
        match self {
            Self::StrRef(value) => Some(value),
            Self::Str(value) => Some(&value),
            Self::NumberUsize(_) => None,
        }
    }

    /// Returns the amount of space in bytes that
    /// this Value needs
    pub fn length(&self) -> usize {
        match self {
            Self::Str(tmp) => tmp.len(),
            Self::StrRef(tmp) => tmp.len(),
            Self::NumberUsize(val) => {
                let mut tmp = *val;
                let mut result = 1;

                loop {
                    if tmp < 10 {
                        return result;
                    }
                    if tmp < 100 {
                        return result + 1;
                    }
                    if tmp < 1000 {
                        return result + 2;
                    }
                    if tmp < 10000 {
                        return result + 3;
                    }

                    tmp /= 10000;
                    result += 4;
                }
            }
        }
    }

    /// Clones all the needed Data in order to create a new
    /// HeaderValue that is completly independant of the given
    /// self reference
    pub fn to_owned<'refed, 'owned>(&'refed self) -> HeaderValue<'owned> {
        match self {
            Self::Str(tmp) => HeaderValue::Str(tmp.clone()),
            Self::StrRef(tmp) => HeaderValue::Str(tmp.to_string()),
            Self::NumberUsize(tmp) => HeaderValue::NumberUsize(*tmp),
        }
    }
}

impl PartialEq<std::string::String> for HeaderValue<'_> {
    fn eq(&self, other: &std::string::String) -> bool {
        match *self {
            Self::StrRef(ref value) => value == other,
            Self::Str(ref value) => value == other,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_str() {
        let mut result: Vec<u8> = Vec::new();
        HeaderValue::Str("test-value".to_owned()).serialize(&mut result);

        assert_eq!("test-value".as_bytes(), &result);
    }
    #[test]
    fn serialize_str_ref() {
        let mut result: Vec<u8> = Vec::new();
        HeaderValue::StrRef("test-value").serialize(&mut result);

        assert_eq!("test-value".as_bytes(), &result);
    }
    #[test]
    fn serialize_number_usize() {
        let mut result: Vec<u8> = Vec::new();
        HeaderValue::NumberUsize(80).serialize(&mut result);

        assert_eq!("80".as_bytes(), &result);
    }

    #[test]
    fn equals_ignore_case() {
        assert_eq!(
            true,
            HeaderValue::StrRef("test").eq_ignore_case(&HeaderValue::StrRef("TEST"))
        );
        assert_eq!(
            true,
            HeaderValue::StrRef("test").eq_ignore_case(&HeaderValue::StrRef("test"))
        );
        assert_eq!(
            true,
            HeaderValue::StrRef("TeSt").eq_ignore_case(&HeaderValue::StrRef("test"))
        );
    }
}
