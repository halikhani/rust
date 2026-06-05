use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum DType {
    F32,
    F16,
    BF16,
    F8E4M3,
    F8E5M2,
    I8,
    U8,
    I4,
    F4,
}

impl DType {
    pub fn size_in_bits(&self) -> usize {
        match self {
            Self::F32 => 32,
            Self::F16 | Self::BF16 => 16,
            Self::F8E4M3 | Self::F8E5M2 | Self::I8 | Self::U8 => 8,
            Self::I4 => 4,
            Self::F4 => 4,
        }
    }

    pub fn parse_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "f32" | "float32" => Some(Self::F32),
            "fp16" | "f16" | "float16" => Some(Self::F16),
            "bf16" | "bfloat16" => Some(Self::BF16),
            "f8e4m3" | "fp8_e4m3" | "e4m3" => Some(Self::F8E4M3),
            "f8e5m2" | "fp8_e5m2" | "e5m2" => Some(Self::F8E5M2),
            "i8" => Some(Self::I8),
            "u8" => Some(Self::U8),
            "i4" | "int4" => Some(Self::I4),
            "f4" | "float4" => Some(Self::F4),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::F32 => "f32",
            Self::F16 => "fp16",
            Self::BF16 => "bf16",
            Self::F8E4M3 => "f8e4m3",
            Self::F8E5M2 => "f8e5m2",
            Self::I8 => "i8",
            Self::U8 => "u8",
            Self::I4 => "i4",
            Self::F4 => "f4",
        }
    }
}

pub fn bytes_for(numel: usize, dt: DType) -> usize {
    match dt {
        DType::I4 => (numel + 1) / 2, // two 4 bit values per byte
        _ => numel * dt.size_in_bits() / 8,
    }
}


impl fmt::Display for DType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn size_in_bits_all_variants() {
        assert_eq!(DType::F32.size_in_bits(), 32);
        assert_eq!(DType::F16.size_in_bits(), 16);
        assert_eq!(DType::BF16.size_in_bits(), 16);
        assert_eq!(DType::F8E4M3.size_in_bits(), 8);
        assert_eq!(DType::F8E5M2.size_in_bits(), 8);
        assert_eq!(DType::I8.size_in_bits(), 8);
        assert_eq!(DType::U8.size_in_bits(), 8);
        assert_eq!(DType::I4.size_in_bits(), 4);
        assert_eq!(DType::F4.size_in_bits(), 4);
    }

    #[test]
    fn parse_and_display_roundtrip() {
        for dt in [DType::F32, DType::F16, DType::BF16, DType::F8E4M3, DType::F8E5M2, DType::I8, DType::U8, DType::I4, DType::F4] {
            let s = dt.to_string();
            assert_eq!(DType::parse_str(&s), Some(dt));
        }
    }

    #[test]
    fn parse_aliases() {
        assert_eq!(DType::parse_str("fp16"), Some(DType::F16));
        assert_eq!(DType::parse_str("bfloat16"), Some(DType::BF16));
    }

    #[test]
    fn bytes_for_i4_packed() {
        assert_eq!(bytes_for(3, DType::I4), 2);
        assert_eq!(bytes_for(7, DType::I4), 4);
    }


}