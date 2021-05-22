#[derive(Debug)]
pub enum StringContainer<'a> {
    Ref(&'a str),
    Owned(String),
}

impl<'a> AsRef<str> for StringContainer<'a> {
    fn as_ref(&self) -> &str {
        match self {
            Self::Ref(r) => r,
            Self::Owned(o) => &o,
        }
    }
}

impl<'a> PartialEq for StringContainer<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
