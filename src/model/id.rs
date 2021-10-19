//! Various types wrapping different IDs used in the API.
#![allow(clippy::module_name_repetitions)]

use std::{fmt::Display, ops::Deref, str::FromStr};

use regex::Regex;
use serde::{self, Deserialize, Serialize};

use crate::errors::Error;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a video.
pub struct VideoId(pub(crate) String);

impl Display for VideoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for VideoId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for VideoId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for VideoId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl FromStr for VideoId {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::expect_used)]
        let regex =
            Regex::new(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]").expect("Video ID regex broke.");

        Ok(regex
            .find(s)
            .ok_or_else(|| Error::InvalidVideoId(s.to_owned()))?
            .as_str()
            .into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
/// The ID of a channel.
pub struct ChannelId(pub(crate) String);

impl Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ChannelId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for ChannelId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for ChannelId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl FromStr for ChannelId {
    type Err = Error;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::expect_used)]
        let regex = Regex::new(r"[0-9A-Za-z_-]{21}[AQgw]").expect("Channel ID regex broke.");

        Ok(regex
            .find(s)
            .ok_or_else(|| Error::InvalidChannelId(s.to_owned()))?
            .as_str()
            .into())
    }
}
