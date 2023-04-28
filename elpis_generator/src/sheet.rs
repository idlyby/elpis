use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SheetDefinition {
    #[serde(alias = "sheet")]
    pub name: String,
    pub default_column: Option<String>,
    pub definitions: Vec<Option<RootDefinition>>,
}

impl Display for SheetDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:#?}\n{:#?}", self.name, self.default_column, self.definitions)
    }
}

/// wtf is this schema layout
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RootDefinition {
    name: Option<String>,
    index: Option<u32>,
    converter: Option<Converter>,
}

impl Display for RootDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}: {:#?}\n{:#?}", self.name, self.index, self.converter)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Converter {
    Basic {
        #[serde(alias = "type")]
        kind: String,
    },
    Target {
        #[serde(alias = "type")]
        kind: String,
        target: String,
    },
    Complex {
        #[serde(alias = "type")]
        kind: String,
        links: Vec<Link>,
    }
}

impl Display for Converter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic { kind } => write!(f, "{}", kind),
            Self::Target { kind, target } => write!(f, "{}: {}", kind, target ),
            Self::Complex { kind, links } => write!(f, "{}:\n{:#?}", kind, links),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    sheet: String,
    when: When,
}

impl Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.sheet, self.when)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct When {
    key: String,
    value: u32,
}

impl Display for When {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}