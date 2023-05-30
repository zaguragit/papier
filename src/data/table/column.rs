use std::str::FromStr;

use base64::{engine::general_purpose, DecodeError, Engine};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnID(u64);

impl ToString for ColumnID {
    fn to_string(&self) -> String {
        general_purpose::URL_SAFE_NO_PAD.encode(self.0.to_be_bytes())
    }
}

impl FromStr for ColumnID {
    type Err = DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        general_purpose::URL_SAFE_NO_PAD.decode(s).map(|x| Self(u64::from_be_bytes(x.try_into().unwrap())))
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub id: ColumnID,
    pub name: String,
    pub unique: bool,
}