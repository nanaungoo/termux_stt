use std::fmt;

#[derive(Clone)]
pub struct ApiTier {
    pub name: &'static str,
    pub rpm: usize,
    pub tpm: usize,
}

impl fmt::Display for ApiTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name == "Custom" {
            write!(f, "Custom Limits")
        } else {
            write!(f, "{} (RPM: {}, TPM: {})", self.name, self.rpm, self.tpm)
        }
    }
}

pub fn get_api_tiers() -> Vec<ApiTier> {
    vec![
        ApiTier { name: "Free Tier", rpm: 15, tpm: 1_000_000 },
        ApiTier { name: "Custom", rpm: 0, tpm: 0 },
    ]
}

fn main() {
    let tiers = get_api_tiers();
    for tier in tiers {
        println!("{}", tier.to_string());
    }
}
