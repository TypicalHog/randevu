//! The official Rust implementation of the [RANDEVU](https://github.com/TypicalHog/randevu) algorithm
//!
//! # Example
//! ```rust
//! use chrono::Utc;
//! use randevu::{rdv, rdvt};
//!
//! fn main() {
//!     let object = "THE_SIMPSONS";
//!     let date = Utc::now();
//!     let rdv = rdv(object, &date);
//!     let rdvt = rdvt(0, object, &date);
//!
//!     println!("Object {} has RDV{} today with RDVT0 at {:?}", object, rdv, rdvt);
//! }
//! ```

use blake3;
use chrono::{DateTime, Datelike, NaiveTime, TimeDelta, Utc};
use itoa;

/// Returns the 32-byte KEY `[u8; 32]` created from a given DATE `&DateTime<Utc>` and an optional RANK `Option<u32>`
fn create_key(date: &DateTime<Utc>, rank: Option<u32>) -> [u8; 32] {
    let mut key = [0u8; 32];

    let mut year = Datelike::year(date);
    let mut month = Datelike::month(date);
    let mut day = Datelike::day(date);

    let mut year_len = 4;
    let mut prefix_len = 0;

    // Add a prefix (-/+) if the year is not between 0 and 9999 (-YYYY-MM-DD / +YYYY-MM-DD)
    if year < 0 {
        key[0] = b'-';
        prefix_len = 1;

        year = year.abs(); // Make year positive
    } else if year > 9999 {
        key[0] = b'+';
        prefix_len = 1;
    }

    // Adjust year_len for very large years (both positive and negative)
    if year > 9999 {
        year_len += 1;
        if year > 99999 {
            year_len += 1;
        }
    }

    let full_year_len = prefix_len + year_len;

    // If a rank is provided, write it into the key after the date, separated by an '_'
    if rank != None {
        let mut buffer = itoa::Buffer::new();
        let rank_str = buffer.format(rank.unwrap());
        key[7 + full_year_len..7 + full_year_len + rank_str.len()]
            .copy_from_slice(&rank_str.as_bytes()[..rank_str.len()]);

        key[6 + full_year_len] = b'_';
    }

    // Write the day into the key
    key[5 + full_year_len] = b'0' + (day % 10) as u8;
    day /= 10;
    key[4 + full_year_len] = b'0' + day as u8;

    key[3 + full_year_len] = b'-';

    // Write the month into the key
    key[2 + full_year_len] = b'0' + (month % 10) as u8;
    month /= 10;
    key[1 + full_year_len] = b'0' + month as u8;

    key[full_year_len] = b'-';

    // Write the year into the key
    for i in (prefix_len..full_year_len).rev() {
        key[i] = b'0' + (year % 10) as u8;
        year /= 10;
    }

    key
}

/// Returns the RDV value `u32` for an OBJECT `&str` on a specific DATE `&DateTime<Utc>`
///
/// **RDV = number of leading zero bits in blake3::keyed_hash(key: DATE, data: OBJECT)**
pub fn rdv(object: &str, date: &DateTime<Utc>) -> u32 {
    let hash = blake3::keyed_hash(&create_key(date, None), object.as_bytes());

    // Count the number of leading zero bits in the hash
    let mut rdv = 0;
    for &byte in hash.as_bytes() {
        rdv += byte.leading_zeros();

        if byte != 0 {
            break;
        }
    }

    rdv
}

/// Returns the RDVT time `DateTime<Utc>` of a given RANK `u32` for an OBJECT `&str` on a specific DATE `&DateTime<Utc>`
pub fn rdvt(rank: u32, object: &str, date: &DateTime<Utc>) -> DateTime<Utc> {
    let hash = blake3::keyed_hash(&create_key(date, Some(rank)), object.as_bytes());

    // Calculate the time using bits from the hash
    let mut total: f64 = 0.0;
    let mut increment = 12.0 * 60.0 * 60.0 * 1_000_000_000.0; // 12h in nanoseconds
    for (i, byte) in hash.as_bytes().iter().enumerate() {
        for j in (0..8).rev() {
            let bit = (byte >> j) & 1;
            if bit == 1 {
                total += increment;
            }
            increment /= 2.0;
        }
        // Stop once increments become too small to affect the total
        if i > 4 && (2.0 * increment) < (1.0 - total.fract()) {
            break;
        }
    }

    // Construct the RDVT time from total
    let rdvt = date.with_time(NaiveTime::MIN).unwrap() + TimeDelta::nanoseconds(total as i64);

    rdvt
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_rdv0() {
        assert_eq!(
            rdv(
                "COREJOURNEY",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            0
        );
    }

    #[test]
    fn test_rdv7() {
        assert_eq!(
            rdv(
                "GTA_V_FLYING_MUSIC_Z7RfRLsqECI",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            7
        );
    }

    #[test]
    fn test_rdv8() {
        assert_eq!(
            rdv(
                "THE_COVENANT_2023",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            8
        );
    }

    #[test]
    fn test_rdv9() {
        assert_eq!(
            rdv(
                "NO_BOILERPLATE",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            9
        );
    }

    #[test]
    fn test_rdvt0() {
        assert_eq!(
            rdvt(
                0,
                "COREJOURNEY",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            Utc.with_ymd_and_hms(2024, 5, 10, 8, 34, 51).unwrap()
                + TimeDelta::nanoseconds(226747801)
        );
    }

    #[test]
    fn test_rdvt1() {
        assert_eq!(
            rdvt(
                1,
                "GTA_V_FLYING_MUSIC_Z7RfRLsqECI",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            Utc.with_ymd_and_hms(2024, 5, 10, 19, 33, 44).unwrap()
                + TimeDelta::nanoseconds(824030471)
        );
    }

    #[test]
    fn test_rdvt10() {
        assert_eq!(
            rdvt(
                10,
                "THE_COVENANT_2023",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            Utc.with_ymd_and_hms(2024, 5, 10, 16, 58, 30).unwrap()
                + TimeDelta::nanoseconds(927007898)
        );
    }

    #[test]
    fn test_rdvt100() {
        assert_eq!(
            rdvt(
                100,
                "NO_BOILERPLATE",
                &Utc.with_ymd_and_hms(2024, 5, 10, 0, 0, 0).unwrap()
            ),
            Utc.with_ymd_and_hms(2024, 5, 10, 0, 27, 37).unwrap()
                + TimeDelta::nanoseconds(142724096)
        );
    }
}
