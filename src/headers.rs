use crate::{
    header::{HeaderKey, HeaderValue},
    Header,
};

/// A collection of Headers
#[derive(Debug, PartialEq, Clone)]
pub struct Headers<'a> {
    headers: Vec<Header<'a>>,
    max_value_length: usize,
}

impl<'a> Headers<'a> {
    /// Creates a new Headers-Instance, for performance reasons
    /// it is recommended to use the `with_capacity` method
    /// as that would avoid frequent reallocations
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            max_value_length: 0,
        }
    }

    /// Creates the Headers-Object with the given Capacity
    /// prereserved for future Headers.
    /// This should be used when you already kind of know
    /// how many Headers this will hold, as it will avoid
    /// extra allocations in the future
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            headers: Vec::with_capacity(cap),
            max_value_length: 0,
        }
    }

    /// Sets the Value of the of the Header for the given Key to
    /// the given Value
    ///
    /// ## Behaviour
    /// Checks if the Key is already present in the Collection and
    /// removes it if that is the case.
    /// Then adds the new Header to the End of the Collection
    pub fn set<'b, K, V>(&mut self, key: K, value: V)
    where
        'b: 'a,
        K: Into<HeaderKey<'a>>,
        V: Into<HeaderValue<'a>>,
    {
        let final_key = key.into();
        if let Some(index) = self.find(&final_key) {
            self.headers.remove(index);
        }

        let n_value: HeaderValue = value.into();
        let n_value_length = n_value.length();
        if n_value_length > self.max_value_length {
            self.max_value_length = n_value_length;
        }

        self.headers.push(Header {
            key: final_key,
            value: n_value,
        });
    }

    /// Appends the given Key-Value Pair to the end of the
    /// Collection, without checking if the Key is already
    /// present in the Collection
    pub fn append<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderKey<'a>>,
        V: Into<HeaderValue<'a>>,
    {
        let n_value: HeaderValue = value.into();
        let n_value_length = n_value.length();
        if n_value_length > self.max_value_length {
            self.max_value_length = n_value_length;
        }

        self.headers.push(Header {
            key: key.into(),
            value: n_value,
        })
    }

    fn find(&self, key: &HeaderKey<'a>) -> Option<usize> {
        for (index, pair) in self.headers.iter().enumerate() {
            if &pair.key == key {
                return Some(index);
            }
        }
        None
    }

    /// Removes the first Header, that matches the given
    /// Key, from the Collection
    pub fn remove<K>(&mut self, key: K)
    where
        K: Into<HeaderKey<'a>>,
    {
        if let Some(index) = self.find(&key.into()) {
            self.headers.remove(index);
        }
    }

    /// Searches the Collection for a Header that matches
    /// the given Key
    ///
    /// Returns:
    /// * None: if no Header matches the Key
    /// * A Reference to the underlying Header-Value that
    /// belongs to the Key
    pub fn get<K>(&self, key: K) -> Option<&HeaderValue<'a>>
    where
        K: Into<HeaderKey<'a>>,
    {
        self.find(&key.into())
            .map(|index| &self.headers.get(index).unwrap().value)
    }

    /// Serializes the Collection of Headers into the
    /// given Buffer by append to it
    pub fn serialize(&self, buf: &mut Vec<u8>) {
        for pair in self.headers.iter() {
            pair.serialize(buf);
        }
    }

    /// Returns the Size in bytes of the biggest Value as text.
    ///
    /// This means that all the Header-Values in this collection
    /// can fit in a buffer of this size.
    pub fn get_max_value_size(&self) -> usize {
        self.max_value_length
    }

    /// Returns the Number of Headers in this collection
    pub fn get_header_count(&self) -> usize {
        self.headers.len()
    }

    /// Clones all the assosicated Data to produce a new and
    /// independant Header-Collection
    pub fn to_owned<'refed, 'owned>(&'refed self) -> Headers<'owned> {
        let mut n_headers = Vec::with_capacity(self.headers.len());

        for tmp in self.headers.iter() {
            n_headers.push(tmp.to_owned());
        }

        Headers {
            headers: n_headers,
            max_value_length: self.max_value_length,
        }
    }
}

impl<'a> Default for Headers<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_add_new() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );
    }
    #[test]
    fn headers_add_already_exists() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );

        headers.set("test-key", "other value");
        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("other value")
            }],
            headers.headers
        );
    }

    #[test]
    fn headers_remove_existing() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );

        headers.remove("test-key");
        assert_eq!(Vec::<Header>::new(), headers.headers);
    }
    #[test]
    fn headers_remove_non_existing() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );

        headers.remove("other-key");
        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );
    }

    #[test]
    fn headers_get_existing() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );

        assert_eq!(
            Some(&HeaderValue::StrRef("test-value")),
            headers.get("test-key")
        );
    }
    #[test]
    fn headers_get_not_existing() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );

        assert_eq!(None, headers.get("other-key"));
    }

    #[test]
    fn headers_serialize() {
        let mut headers = Headers::new();
        headers.set("test-key", "test-value");

        assert_eq!(
            vec![Header {
                key: HeaderKey::StrRef("test-key"),
                value: HeaderValue::StrRef("test-value")
            }],
            headers.headers
        );

        let result = "test-key: test-value\r\n".as_bytes();
        let mut tmp: Vec<u8> = Vec::new();
        headers.serialize(&mut tmp);
        assert_eq!(result, &tmp);
    }
}
