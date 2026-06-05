// safetensors header format
// {
//     "__metadata__": { "format": "pt" },
//     "layer.weight": {
//       "dtype": "F32",
//       "shape": [2, 2],
//       "data_offsets": [0, 16]
//     }
//   }


// A safetensors file is one contiguous blob:
// byte 0
// │
// ├─ [Part 1] 8 bytes          → header length N (u64, little-endian)
// ├─ [Part 2] N bytes          → JSON header (UTF-8 text)
// └─ [Part 3] rest of file     → raw tensor bytes (binary, no JSON)




use serde::Deserialize; // trait that allows us to deserialize JSON into a struct
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::Read;

const HEADER_LEN_SIZE: usize = 8;
const MAX_HEADER_BYTES: u64 = 100_000_000; // 100 MB, same as huggingface's safetensors library

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)] // this tells the compiler that the struct can be deserialized from a JSON object
pub struct TensorInfo {
    pub dtype: String,
    pub shape: Vec<usize>,
    pub data_offsets: [usize; 2],
}


// Debug: trait that allows us to print the struct as a string (e.g., println!("{:?}", header))
// Clone: trait that allows us to clone the struct
// Default: trait that allows us to create a default instance of the struct
// PartialEq: trait that allows us to compare the struct with another struct
// Eq: trait that allows us to compare the struct with another struct
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Header {
    pub metadata: HashMap<String, String>,
    pub tensors: HashMap<String, TensorInfo>,
}

impl Header {
    fn from_raw_map(raw: HashMap<String, serde_json::Value>) -> Result<Self, LoadError> {
        let mut header = Header::default();
        for (key, value) in raw {
            if key == "__metadata__" {
                header.metadata = serde_json::from_value(value)?;
            }
            else {
                let info: TensorInfo = serde_json::from_value(value)?;
                header.tensors.insert(key, info);
            }
        }
        Ok(header)

    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("bad header length: {0} bytes (sanity limit is 100 MB)")]
    BadMagic(u64),
    #[error("invalid header JSON: {0}")]
    BadJson(#[from] serde_json::Error),
}


pub fn read_header(path: &Path) -> Result<Header, LoadError> {
    let mut file = File::open(path)?;

    let mut len_buf = [0u8; HEADER_LEN_SIZE]; 
    file.read_exact(&mut len_buf)?;
    let header_len = u64::from_le_bytes(len_buf);

    if header_len > MAX_HEADER_BYTES {
        return Err(LoadError::BadMagic(header_len));
    }
    let header_len = header_len as usize;
    let mut json_buf = vec![0u8; header_len];
    file.read_exact(&mut json_buf)?;

    let raw: HashMap<String, serde_json::Value> = serde_json::from_slice(&json_buf)?;
    Header::from_raw_map(raw)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn test_tiny_header() {
        let header = read_header(&fixture_path("tiny.safetensors")).unwrap();

        assert_eq!(header.tensors.len(), 1);

        let info = header.tensors.get("x").unwrap();
        assert_eq!(info.dtype, "F32");
        assert_eq!(info.shape, vec![2]);
        assert_eq!(info.data_offsets, [0, 8]);
    }

    #[test]
    fn rejects_oversized_header_length() {
        let path = std::env::temp_dir().join("oversized_header.safetensors");
        let bad_len = (MAX_HEADER_BYTES + 1).to_le_bytes();
        std::fs::write(&path, bad_len).unwrap();

        match read_header(&path) {
            Err(LoadError::BadMagic(len)) => assert_eq!(len, MAX_HEADER_BYTES + 1),
            _ => panic!("expected BadMagic error"),
        }
    }

    #[test]
    fn rejects_invalid_json() {
        let path = std::env::temp_dir().join("invalid_json.safetensors");

        let garbage = b"{not valid json";
        let mut bytes = (garbage.len() as u64).to_le_bytes().to_vec();
        bytes.extend_from_slice(garbage);
        std::fs::write(&path, bytes).unwrap();
        assert!(matches!(read_header(&path), Err(LoadError::BadJson(_))));

    }

}