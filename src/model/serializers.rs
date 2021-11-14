use ::core::fmt;
use std::str::FromStr;
use std::{convert::TryFrom, fmt::Display};

use serde::de::value::Error;
use serde::{de::IntoDeserializer as _, Deserialize, Deserializer, Serialize, Serializer};

use super::{Language, Organisation};

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[allow(dead_code)]
        enum LangSer {
            #[serde(rename(serialize = "all"))]
            All,
            #[serde(rename(serialize = "en"))]
            English,
            #[serde(rename(serialize = "es"))]
            Spanish,
            #[serde(rename(serialize = "id"))]
            Indonesian,
            #[serde(rename(serialize = "ja"))]
            Japanese,
            #[serde(rename(serialize = "ko"))]
            Korean,
            #[serde(rename(serialize = "ru"))]
            Russian,
            #[serde(rename(serialize = "zh"))]
            Chinese,
        }

        let value = match *self {
            Self::All => LangSer::All,
            Self::English => LangSer::English,
            Self::Spanish => LangSer::Spanish,
            Self::Indonesian => LangSer::Indonesian,
            Self::Japanese => LangSer::Japanese,
            Self::Korean => LangSer::Korean,
            Self::Russian => LangSer::Russian,
            Self::Chinese => LangSer::Chinese,
            Self::Other(ref s) => return Serialize::serialize(s, serializer),
        };

        Serialize::serialize(&value, serializer)
    }
}

impl Display for Language {
    #[allow(clippy::wildcard_enum_match_arm)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use serde::Serialize as _;

        match *self {
            Self::Other(ref s) => write!(f, "{}", s),
            _ => self.serialize(f),
        }
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[allow(dead_code)]
        enum LangDe {
            #[serde(rename(deserialize = "all"))]
            All,
            #[serde(rename(deserialize = "en"))]
            English,
            #[serde(rename(deserialize = "es"))]
            Spanish,
            #[serde(rename(deserialize = "id"))]
            Indonesian,
            #[serde(rename(deserialize = "ja"))]
            Japanese,
            #[serde(rename(deserialize = "ko"))]
            Korean,
            #[serde(rename(deserialize = "ru"))]
            Russian,
            #[serde(rename(deserialize = "zh"))]
            Chinese,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum LangDeUntagged {
            Enum(LangDe),
            Other(String),
        }

        let value = match <LangDeUntagged as Deserialize>::deserialize(deserializer)? {
            LangDeUntagged::Enum(e) => match e {
                LangDe::All => Language::All,
                LangDe::English => Language::English,
                LangDe::Spanish => Language::Spanish,
                LangDe::Indonesian => Language::Indonesian,
                LangDe::Japanese => Language::Japanese,
                LangDe::Korean => Language::Korean,
                LangDe::Russian => Language::Russian,
                LangDe::Chinese => Language::Chinese,
            },
            LangDeUntagged::Other(v) => Language::Other(v),
        };
        Ok(value)
    }
}

impl FromStr for Language {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl TryFrom<String> for Language {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Language {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'de> Deserialize<'de> for Organisation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "PascalCase"))]
        #[allow(dead_code)]
        enum OrgDe {
            Hololive,
            Nijisanji,
            Independents,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OrgDeUntagged {
            Enum(OrgDe),
            Other(String),
        }

        let value = match <OrgDeUntagged as serde::Deserialize>::deserialize(deserializer)? {
            OrgDeUntagged::Enum(e) => match e {
                OrgDe::Hololive => Organisation::Hololive,
                OrgDe::Nijisanji => Organisation::Nijisanji,
                OrgDe::Independents => Organisation::Independents,
            },
            OrgDeUntagged::Other(v) => Organisation::Other(v),
        };

        Ok(value)
    }
}
impl ::core::str::FromStr for Organisation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl TryFrom<String> for Organisation {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl TryFrom<&str> for Organisation {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Serialize for Organisation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all(serialize = "PascalCase"))]
        #[allow(dead_code)]
        enum OrgSer {
            Hololive,
            Nijisanji,
            Independents,
        }

        let value = match *self {
            Self::Hololive => OrgSer::Hololive,
            Self::Nijisanji => OrgSer::Nijisanji,
            Self::Independents => OrgSer::Independents,
            Self::Other(ref s) => return Serialize::serialize(s, serializer),
        };

        Serialize::serialize(&value, serializer)
    }
}

impl Display for Organisation {
    #[allow(clippy::wildcard_enum_match_arm)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Other(ref s) => write!(f, "{}", s),
            _ => self.serialize(f),
        }
    }
}
