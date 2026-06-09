use crate::{Config, TS};
use serde::Serialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeMode {
    HumanReadable,
    Compact,
}

#[derive(Debug)]
struct TimeModeProbeError;

impl std::fmt::Display for TimeModeProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unsupported serialization shape")
    }
}

impl std::error::Error for TimeModeProbeError {}

impl serde::ser::Error for TimeModeProbeError {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        TimeModeProbeError
    }
}

struct CompactTupleProbe;

impl serde::ser::SerializeTuple for CompactTupleProbe {
    type Ok = TimeMode;
    type Error = TimeModeProbeError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(TimeMode::Compact)
    }
}

struct TimeModeProbeSerializer;

impl serde::ser::Serializer for TimeModeProbeSerializer {
    type Ok = TimeMode;
    type Error = TimeModeProbeError;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = CompactTupleProbe;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(TimeMode::Compact)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(TimeMode::HumanReadable)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(TimeModeProbeError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(TimeModeProbeError)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(TimeModeProbeError)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(CompactTupleProbe)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(TimeModeProbeError)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(TimeModeProbeError)
    }
}

fn detect_time_serde_mode() -> TimeMode {
    serde::Serialize::serialize(&time::OffsetDateTime::UNIX_EPOCH, TimeModeProbeSerializer)
        .unwrap_or_else(|_| TimeMode::HumanReadable)
}

fn time_mode_binding() -> TimeMode {
    static TIME_BINDING: OnceLock<TimeMode> = OnceLock::new();
    *TIME_BINDING.get_or_init(|| detect_time_serde_mode())
}

impl TS for time::Month {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12".to_owned(),
            TimeMode::HumanReadable => r#""January" | "February" | "March" | "April" | "May" | "June" | "July" | "August" | "September" | "October" | "November" | "December""#.to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::Weekday {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "1 | 2 | 3 | 4 | 5 | 6 | 7".to_owned(),
            TimeMode::HumanReadable => {
                r#""Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday""#.to_owned()
            }
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::Duration {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "[seconds: number, nanoseconds: number]".to_owned(),
            TimeMode::HumanReadable => "string".to_string(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::Date {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "[year: number, ordinal: number]".to_owned(),
            TimeMode::HumanReadable => "string".to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::Time {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => {
                "[hour: number, minute: number, second: number, nanosecond: number]".to_owned()
            }
            TimeMode::HumanReadable => "string".to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::PrimitiveDateTime {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "[year: number, ordinal: number, hour: number, minute: number, second: number, nanosecond: number]".to_owned(),
            TimeMode::HumanReadable => "string".to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::UtcDateTime {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "[year: number, ordinal: number, hour: number, minute: number, second: number, nanosecond: number]".to_owned(),
            TimeMode::HumanReadable => "string".to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::OffsetDateTime {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "[year: number, ordinal: number, hour: number, minute: number, second: number, nanosecond: number, offset_hours: number, offset_minutes: number, offset_seconds: number]".to_owned(),
            TimeMode::HumanReadable => "string".to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

impl TS for time::UtcOffset {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_cfg: &Config) -> String {
        match time_mode_binding() {
            TimeMode::Compact => "[hours: number, minutes: number, seconds: number]".to_owned(),
            TimeMode::HumanReadable => "string".to_owned(),
        }
    }

    fn inline(cfg: &Config) -> String {
        <Self as TS>::name(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    fn assert_name_matches_serialization<T: Serialize + TS>(value: T) {
        let json = serde_json::to_value(&value).unwrap();
        let name = T::name(&Config::default());

        if name == "string" || name.contains('"') {
            assert!(
                json.is_string(),
                "expected a string for `{name}`, got {json}"
            );
        } else if name.starts_with('[') {
            assert!(
                json.is_array(),
                "expected an array for `{name}`, got {json}"
            );
        } else {
            assert!(
                json.is_number(),
                "expected a number for `{name}`, got {json}"
            );
        }
    }

    #[test]
    fn time_ts_names_match_serialization() {
        let date = time::OffsetDateTime::UNIX_EPOCH.date();
        let clock = time::OffsetDateTime::UNIX_EPOCH.time();

        assert_name_matches_serialization(time::Month::June);
        assert_name_matches_serialization(time::Weekday::Wednesday);
        assert_name_matches_serialization(time::Duration::new(5, 250));
        assert_name_matches_serialization(date);
        assert_name_matches_serialization(clock);
        assert_name_matches_serialization(time::PrimitiveDateTime::new(date, clock));
        assert_name_matches_serialization(time::UtcDateTime::new(date, clock));
        assert_name_matches_serialization(time::OffsetDateTime::UNIX_EPOCH);
        assert_name_matches_serialization(time::UtcOffset::UTC);
    }
}
