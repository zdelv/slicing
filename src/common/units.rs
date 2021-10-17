use crate::error::Error;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    Meter,
    Centimeter,
    Millimeter,
    Foot,
    Inch,
    Unknown,
}

impl Unit {
    pub fn new(name: &str) -> Self {
        match name {
            "meters" => Self::Meter,
            "centimeters" => Self::Centimeter,
            "millimeters" => Self::Millimeter,
            "feet" => Self::Foot,
            "inches" => Self::Inch,
            _ => Self::Unknown,
        }
    }
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Unit::new(s))
    }
}
