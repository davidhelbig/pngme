use crate::chunk_type::ChunkType;
use crate::chunk_type::ChunkTypeError;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::{convert::TryFrom, io::Read, string::FromUtf8Error};
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    length: u32,
    crc: u32
}

#[derive(Debug)]
pub enum ChunkError {
    IO(std::io::Error),
    Crc,
    ChunkType(ChunkTypeError)
}

impl From<std::io::Error> for ChunkError {
    fn from(err: std::io::Error) -> Self {
        ChunkError::IO(err)
    }
}

impl From<ChunkTypeError> for ChunkError {
    fn from(err: ChunkTypeError) -> Self {
        ChunkError::ChunkType(err)
    }
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::IO(err) => write!(f, "IO Error: {err}"),
            ChunkError::Crc => write!(f, "CRC error while parsing chunk!"),
            ChunkError::ChunkType(err) => write!(f, "Chunk Type error: {}", err)
        }    
    }
}

impl Error for ChunkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChunkError::IO(err) => Some(err),
            ChunkError::ChunkType(err) => Some(err),
            _ => None
        }
    }
}


impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len().try_into().unwrap();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let checksum = crc.checksum(&[&chunk_type.bytes()[..], &data[..]].concat());

        Chunk { chunk_type, data, length, crc: checksum }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes().iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let mut length_bytes = [0; 4];
        value.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes);

        let mut chunk_type_bytes = [0; 4];
        value.read_exact(&mut chunk_type_bytes)?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let mut data = vec![0_u8; length as usize];
        value.read_exact(&mut data)?;

        let mut crc_bytes = [0; 4];
        value.read_exact(&mut crc_bytes)?;
        let crc_checksum = u32::from_be_bytes(crc_bytes);

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let validation_checksum = crc.checksum(&[&chunk_type.bytes()[..], &data[..]].concat());

        if crc_checksum != validation_checksum {
            return Err(ChunkError::Crc);
        }

        Ok(Chunk {
            chunk_type,
            length,
            data,
            crc: crc_checksum
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        // let _chunk_string = format!("{}", chunk);
    }
}
