//! The official Rust implementation of the [RANDEVU](https://github.com/TypicalHog/randevu) algorithm
//!
//! # Example
//! ```rust
//! use randevu::{rdv, utc_date_with_offset};
//!
//! fn main() {
//!     let object = "THE_SIMPSONS";
//!     let date = utc_date_with_offset(0);
//!     let rdv = rdv(object, &date);
//!
//!     println!("Object {} has RDV{} today", object, rdv);
//! }
//! ```

use blake3;
use chrono::{TimeDelta, Utc};

/// Returns current UTC DATE `String` in ISO 8601 format (YYYY-MM-DD), with an OFFSET `i64` in days
pub fn utc_date_with_offset(offset: i64) -> String {
    (Utc::now() + TimeDelta::days(offset))
        .format("%Y-%m-%d")
        .to_string()
}

/// Returns RDV level `u32` for an OBJECT `&str` on a specific DATE `&str`
///
/// **RDV = number of leading zero bits in blake3(blake3(OBJECT) || blake3(DATE))**
pub fn rdv(object: &str, date: &str) -> u32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(blake3::hash(object.as_bytes()).as_bytes());
    hasher.update(blake3::hash(date.as_bytes()).as_bytes());
    let final_hash = hasher.finalize();

    let mut rdv = 0;
    for &byte in final_hash.as_bytes() {
        rdv += byte.leading_zeros();

        if byte != 0 {
            break;
        }
    }

    rdv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdv_1() {
        assert_eq!(rdv("NO_BOILERPLATE", "2024-01-26"), 11);
    }

    #[test]
    fn test_rdv_2() {
        assert_eq!(rdv("SHREK_2001", "2024-01-26"), 8);
    }

    #[test]
    fn test_rdv_3() {
        assert_eq!(rdv("RANDEVU", "2024-01-26"), 1);
    }

    #[test]
    fn test_rdv_4() {
        assert_eq!(rdv("RUST", "2024-01-26"), 0);
    }
}
