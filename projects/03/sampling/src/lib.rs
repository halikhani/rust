#[derive(Debug, Clone, PartialEq)]
pub enum SamplingMode {
    Greedy,
    TopK { k: u32 },
    TopP { p: f32 },
    MinP { p: f32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SamplingParams {
    pub mode: SamplingMode,
    pub temperature: f32,
    pub repetition_penalty: f32,
    pub max_tokens: usize,
    pub stop: Vec<String>,
}


impl Default for SamplingParams {
    fn default() -> Self {
        Self {
            mode: SamplingMode::Greedy,
            temperature: 1.0,
            repetition_penalty: 1.0,
            max_tokens: 100,
            stop: Vec::new(), // or vec![]
        }
    }
}

//NOTE: Rust's standard success or error enum:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl SamplingParams {
    pub fn validate(&self) -> Result<(), String> { // &self: borrow the struct, so we can't modify it, return either success with nothing () or error with a string
        if self.temperature <= 0.0 {
            return Err(format!("Temperature must be positive, got {}", self.temperature));
        }

        if self.max_tokens == 0 {
            return Err("Max tokens must be greater than 0".into());
        }

        if let SamplingMode::TopP { p } = self.mode { // if self.mode matches the pattern SamplingMode::TopP { p }, then bind the value of p to the pattern
            if p <= 0.0 || p > 1.0 {
                return Err(format!("TopP must be in (0, 1], got {}", p));
            }
        }
        Ok(())

        // equivalent match statement:
        // match self.mode {
        //     SamplingMode::TopP { p } => {
        //         if p <= 0.0 || p > 1.0 {
        //             return Err(format!("TopP must be in (0, 1], got {}", p));
        //         }
        //     }
        //     _ => {}
        // }
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_greedy() {
        let params = SamplingParams::default();
        assert_eq!(params.mode, SamplingMode::Greedy);
        assert!(params.max_tokens > 0);
    }

    #[test]
    fn validate_accepts_default() {
        assert!(SamplingParams::default().validate().is_ok());
    }

    #[test]
    fn validate_rejects_negative_temperature() {
        let mut p = SamplingParams::default();
        p.temperature = -1.0;
        assert!(p.validate().is_err());
    }

    #[test]
    fn validate_rejects_zero_max_tokens() {
        let mut p = SamplingParams::default();
        p.max_tokens = 0;
        assert!(p.validate().is_err());
    }

    #[test]
    fn validate_accepts_valid_top_p() {
        let p = SamplingParams {
            mode: SamplingMode::TopP { p: 0.9},
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn validate_rejects_invalid_top_p() {
        for bad_p in [0.0, -0.1, 1.5] {
            let p = SamplingParams {
                mode: SamplingMode::TopP { p: bad_p},
                ..Default::default()
            };
            assert!(p.validate().is_err(), "Expected error for TopP {}", bad_p);
        }
    }
}
