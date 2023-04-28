use core::fmt::{self, Display};

use num_derive::{FromPrimitive, ToPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Platform {
    #[default]
    Sqex = 0,
    Steam = 1,
}

impl Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Sqex => write!(f, "Square Enix Store"),
            Self::Steam => write!(f, "Steam"),
        }
    }
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum AccountType {
    #[default]
    Subscription = 0,
    FreeTrial = 1,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Subscription => write!(f, "Full game (subscription)"),
            Self::FreeTrial => write!(f, "Free trial"),
        }
    }
}