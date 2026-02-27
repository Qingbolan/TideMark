//! TideMark
//! ========
//!
//! File: src/core/time.rs
//! Description: Timezone policy parsing and day-delta date math utilities.
//!
//! Responsibility:
//! - Provide explicit UTC or fixed-offset date conversion with deterministic day calculations.
//!
//! Architectural Position:
//! - Core temporal utility layer used by version-coordinate algorithms.
//!
//! Author: Silan.Hu
//! Email: silan.hu@u.nus.edu
//! Copyright (c) 2026-2027 easynet. All rights reserved.

use chrono::{DateTime, FixedOffset, NaiveDate, Utc};

use crate::error::{TideError, TideResult};

#[derive(Debug, Clone)]
pub enum TimezonePolicy {
    Utc,
    Fixed(FixedOffset),
}

impl TimezonePolicy {
    pub fn parse(raw: &str) -> TideResult<Self> {
        let trimmed = raw.trim();
        if trimmed.eq_ignore_ascii_case("utc") || trimmed == "Z" {
            return Ok(Self::Utc);
        }

        parse_fixed_offset(trimmed)
            .map(Self::Fixed)
            .ok_or_else(|| TideError::InvalidTimezone {
                value: raw.to_string(),
            })
    }

    pub fn canonical_name(&self) -> String {
        match self {
            Self::Utc => "UTC".to_string(),
            Self::Fixed(offset) => offset.to_string(),
        }
    }

    pub fn date_for_timestamp(&self, ts: i64) -> TideResult<NaiveDate> {
        let dt_utc = DateTime::<Utc>::from_timestamp(ts, 0).ok_or_else(|| TideError::Internal {
            message: format!("invalid unix timestamp: {ts}"),
        })?;

        let date = match self {
            Self::Utc => dt_utc.date_naive(),
            Self::Fixed(offset) => dt_utc.with_timezone(offset).date_naive(),
        };
        Ok(date)
    }

    pub fn day_delta(&self, anchor_ts: i64, target_ts: i64) -> TideResult<i64> {
        let anchor_date = self.date_for_timestamp(anchor_ts)?;
        let target_date = self.date_for_timestamp(target_ts)?;
        Ok(target_date.signed_duration_since(anchor_date).num_days())
    }
}

fn parse_fixed_offset(raw: &str) -> Option<FixedOffset> {
    if raw.len() != 6 {
        return None;
    }
    let sign = raw.chars().next()?;
    if sign != '+' && sign != '-' {
        return None;
    }
    if raw.as_bytes().get(3).copied()? != b':' {
        return None;
    }

    let hours: i32 = raw[1..3].parse().ok()?;
    let minutes: i32 = raw[4..6].parse().ok()?;
    if hours > 23 || minutes > 59 {
        return None;
    }

    let total = hours * 3600 + minutes * 60;
    match sign {
        '+' => FixedOffset::east_opt(total),
        '-' => FixedOffset::west_opt(total),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_timezone_variants() {
        assert!(matches!(
            TimezonePolicy::parse("UTC").unwrap(),
            TimezonePolicy::Utc
        ));
        assert!(matches!(
            TimezonePolicy::parse("+08:00").unwrap(),
            TimezonePolicy::Fixed(_)
        ));
        assert!(TimezonePolicy::parse("+8").is_err());
    }

    #[test]
    fn computes_day_delta() {
        let tz = TimezonePolicy::parse("UTC").unwrap();
        let anchor = 1_704_067_200; // 2024-01-01T00:00:00Z
        let target = 1_704_153_600; // 2024-01-02T00:00:00Z
        assert_eq!(tz.day_delta(anchor, target).unwrap(), 1);
    }
}
