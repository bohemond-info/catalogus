use serde::{Deserialize, Serialize};
use tracing::Level;
use std::path::PathBuf;

pub mod opt_from_string {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
        where
            D: Deserializer<'de>,
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        String::deserialize(deserializer)?
            .parse::<T>()
            .map(|r| Some(r))
            .map_err(|e| D::Error::custom(format!("{}", e)))
    }

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: std::fmt::Display,
    {
        match value {
            Some(x) => format!("{}", x),
            None => format!("")
        }.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BohemondConfig {
    #[serde(with = "opt_from_string")]
    pub log_level: Option<Level>,
    pub static_asset_directory: Option<PathBuf>,
}
