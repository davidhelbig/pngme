use std::convert::TryFrom;
use std::str;
use std::fmt;
use std::error::Error;
use std::str::Utf8Error;


#[derive(PartialEq, Debug)]
pub struct ChunkType {
    type_code: String
}

#[derive(Debug, PartialEq)]
pub enum ChunkTypeError {
    InvalidBit,
    InvalidLength {
        num_bit_received: usize
    },
    Ascii,
    Utf8(Utf8Error)
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidBit => write!(f, "The reserved bit of the chunk type is not valid."),
            ChunkTypeError::Ascii => write!(f, "Received non-alphabetic ascii characters in chunk type."),
            ChunkTypeError::InvalidLength { num_bit_received} => write!(f, "Invalid length in chunk type, expecting exactly 4 bytes, got {num_bit_received}"),
            ChunkTypeError::Utf8(err) => write!(f, "Utf8 error while parsing chunk type: {}", err)
        }
    }
}

impl Error for ChunkTypeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChunkTypeError::Utf8(err) => Some(err),
            _ => None
        }
    }
}


impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.type_code.as_bytes().try_into().unwrap()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes()[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes()[1].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3].is_ascii_lowercase()
    }
}


impl TryFrom<&[u8]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let code = str::from_utf8(value);

        match code {
            Ok(code) => {
                if code.len() != 4 {
                    return Err(ChunkTypeError::InvalidLength{ num_bit_received: code.len() });
                }

                if !code.as_bytes().iter().all(|x| x.is_ascii_alphabetic()) {
                    return Err(ChunkTypeError::Ascii);
                }

                if !code.as_bytes()[2].is_ascii_uppercase() {
                    return Err(ChunkTypeError::InvalidBit);
                }
                Ok(ChunkType { type_code: code.to_string() })
            },
            Err(err) => Err(ChunkTypeError::Utf8(err))
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        ChunkType::try_from(&value[..])
    }
}

impl str::FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_code)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk_err = ChunkType::from_str("Rust").unwrap_err();
        assert_eq!(chunk_err, ChunkTypeError::InvalidBit);
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk_err = ChunkType::from_str("Ru1t").unwrap_err();
        assert_eq!(chunk_err, ChunkTypeError::Ascii)
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
