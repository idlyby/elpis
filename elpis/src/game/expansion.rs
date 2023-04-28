use core::fmt::{self, Display};

use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};

use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_json::Value;

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Expansion {
    #[default]
    ARealmReborn = 0,
    Heavensward,
    Stormblood,
    Shadowbringers,
    Endwalker
}

impl Display for Expansion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ARealmReborn => write!(f, "Final Fantasy XIV: A Realm Reborn"),
            Self::Heavensward => write!(f, "Final Fantasy XIV: Heavensward"),
            Self::Stormblood => write!(f, "Final Fantasy XIV: Stormblood"),
            Self::Shadowbringers => write!(f, "Final Fantasy XIV: Shadowbringers"),
            Self::Endwalker => write!(f, "Final Fantasy XIV: Endwalker"),
        }
    }
}

impl From<&Value> for Expansion {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

impl From<&str> for Expansion {
    fn from(value: &str) -> Self {
        if value.contains("ex") {
            FromPrimitive::from_u8(value.as_bytes()[2]).unwrap_or_default()
        } else {
            Self::ARealmReborn
        }
    }
}