pub mod cli;
pub use cli::{Cli, Commands};

use std::fmt;

#[derive(Clone)]
pub struct ApiTier {
    pub name: &'static str,
    pub rpm: usize,
    pub tpm: usize,
    pub rpd: usize,
}

impl fmt::Display for ApiTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name == "Custom" {
            write!(f, "Custom Limits")
        } else {
            write!(
                f,
                "{} (RPM: {}, TPM: {}, RPD: {})",
                self.name, self.rpm, self.tpm, self.rpd
            )
        }
    }
}

pub fn get_api_tiers() -> Vec<ApiTier> {
    vec![
        ApiTier {
            name: "Gemini 3.1 Flash Lite",
            rpm: 15,
            tpm: 100_000,
            rpd: 500,
        },
        ApiTier {
            name: "Free Tier",
            rpm: 15,
            tpm: 100_000,
            rpd: 500,
        },
        ApiTier {
            name: "Tier 1 (Paid)",
            rpm: 2_000,
            tpm: 4_000_000,
            rpd: 0,
        },
        ApiTier {
            name: "Tier 2 (Paid)",
            rpm: 5_000,
            tpm: 10_000_000,
            rpd: 0,
        },
        ApiTier {
            name: "Custom",
            rpm: 0,
            tpm: 0,
            rpd: 0,
        },
    ]
}
