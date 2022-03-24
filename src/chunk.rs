use std::{io::Read, error::Error, fmt::{Display, Formatter}};

use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};
pub const CRC_CHECKER: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);


pub struct Chunk {
    pub chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Self {
        Chunk { chunk_type, chunk_data }
    }

    pub fn length(&self) -> u32 {
        self.chunk_data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        let assembled_data: Vec<u8> = self.chunk_type.bytes().iter().chain(self.chunk_data.iter()).cloned().collect();
        CRC_CHECKER.checksum(&assembled_data)
    }

    pub fn data_as_string(&self) -> Result<String, ()> {
        let conversion = String::from_utf8(self.chunk_data.clone());
        match conversion {
            Ok(s) => Ok(s),
            Err(_) => Err(())
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let assembled_bytes: Vec<u8> = self.length().to_be_bytes().iter().chain(self.chunk_type.bytes().iter()).chain(self.chunk_data.iter()).chain(self.crc().to_be_bytes().iter()).cloned().collect();
        assembled_bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ();
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value_len = value.len();
        let data_length = value_len - 12;
        if value_len < 12 {
            return Err(());
        }
        let v_length = u32::from_be_bytes(value[0..4].try_into().unwrap());
        if v_length as usize != data_length {
            return Err(());
        }
        let v_chunk_type: [u8; 4] = value[4..8].try_into().unwrap();
        let v_chunk_type = match ChunkType::try_from(v_chunk_type) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };
        let crc_index = 8 + data_length;
        let v_chunk_data = &value[8..crc_index];
        let v_crc = u32::from_be_bytes(value[crc_index..].try_into().unwrap());
        
        
        let v_chunk = Chunk::new(v_chunk_type, v_chunk_data.to_vec());
        if v_chunk.crc() != v_crc {
            Err(())
        } else {
            Ok(v_chunk)
        }
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}, {}", self.length(), self.chunk_type(), self.data(), self.crc())
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
        
        let _chunk_string = format!("{}", chunk);
    }
}