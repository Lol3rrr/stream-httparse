/// A single HTTP-Chunk used for sending
/// Data with `Transfer-Encoding: Chunked`
#[derive(Debug, PartialEq)]
pub struct Chunk {
    size: usize,
    body: Vec<u8>,
}

impl Chunk {
    /// Creates a new Chunk with the given Data as its
    /// state
    pub fn new(size: usize, data: Vec<u8>) -> Self {
        Self { size, body: data }
    }

    /// Serializes the Chunk into the given Buffer
    /// by appending the final Data to the End of it
    pub fn serialize(&self, buf: &mut Vec<u8>) {
        let length = format!("{:x}", self.size);
        buf.extend_from_slice(length.as_bytes());
        buf.extend_from_slice("\r\n".as_bytes());
        buf.extend_from_slice(&self.body);
        buf.extend_from_slice("\r\n".as_bytes());
    }

    /// The given Size of the Chunk
    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_valid() {
        let tmp = Chunk::new(9, "Developer".as_bytes().to_vec());

        let mut buf: Vec<u8> = Vec::new();
        tmp.serialize(&mut buf);

        assert_eq!("9\r\nDeveloper\r\n".as_bytes().to_vec(), buf);
    }
    #[test]
    fn serialize_valid_2() {
        let tmp = Chunk::new(
            55,
            "This is just some random Data to fill the Response with"
                .as_bytes()
                .to_vec(),
        );

        let mut buf: Vec<u8> = Vec::new();
        tmp.serialize(&mut buf);

        assert_eq!(
            "37\r\nThis is just some random Data to fill the Response with\r\n"
                .as_bytes()
                .to_vec(),
            buf
        );
    }
}
