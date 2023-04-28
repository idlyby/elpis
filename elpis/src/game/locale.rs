
use core::fmt::{self, Display};

use num_derive::{FromPrimitive, ToPrimitive};

use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_json::Value;

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Language {
    Japanese = 0,
    #[default]
    English = 1,
    German = 2,
    French = 3,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Japanese => "ja",
            Self::German => "de",
            Self::French => "fr",
            Self::English => {
                if let Some(locale) = sys_locale::get_locale() {
                    if locale.contains("US") {
                        "en-us"
                    } else {
                        "en-gb"
                    }
                } else {
                    "en-gb"
                }
            },
        }
    }
}

impl From<&Value> for Language {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Region {
    Japan = 1,
    #[default]
    America = 2,
    Europe = 3,
    Korea = 101,
}

impl Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&Value> for Region {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

