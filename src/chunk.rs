pub struct Chunk {
    pub length: u32,
    pub chunk_type: [u8; 4],
    pub data: Vec<u8>,
    pub crc: u32,
}

impl Chunk {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 12 {
            // Not enough bytes for a valid chunk
            return None;
        }

        let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let end_of_chunk = length as usize + 12;
        if bytes.len() < end_of_chunk {
            return None;
        }

        let chunk_type = bytes[4..8].try_into().unwrap();
        let data = bytes[8..8 + length as usize].to_vec();
        let crc_start = 8 + length as usize;
        let crc = u32::from_be_bytes(bytes[crc_start..crc_start + 4].try_into().unwrap());

        Some(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }

    pub fn is_critical(&self) -> bool {
        matches!(
            self.chunk_type,
            [b'I', b'H', b'D', b'R'] | 
            [b'I', b'D', b'A', b'T'] | 
            [b'P', b'L', b'T', b'E'] | 
            [b't', b'R', b'N', b'S'] | 
            [b'I', b'E', b'N', b'D']
        )
    }
}
