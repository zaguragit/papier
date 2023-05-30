use std::str::FromStr;
use base64::{engine::general_purpose, Engine, DecodeError};

pub mod table;
pub mod text;

pub struct FileDisplay {
    pub title: String,
    pub category: Category,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Text,
    Table,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileID(pub u64);

impl ToString for FileID {
    fn to_string(&self) -> String {
        general_purpose::URL_SAFE_NO_PAD.encode(self.0.to_be_bytes())
    }
}

impl FromStr for FileID {
    type Err = DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        general_purpose::URL_SAFE_NO_PAD.decode(s).map(|x| Self(u64::from_be_bytes(x.try_into().unwrap())))
    }
}