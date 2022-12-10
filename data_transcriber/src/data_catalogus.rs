use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//
// pub mod loc_from_str {
//     use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
//     use tracing::Level;
//
//     pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
//         where
//             D: Deserializer<'de>,
//             T: std::str::FromStr,
//             <T as std::str::FromStr>::Err: std::fmt::Display,
//     {
//         String::deserialize(deserializer)?
//             .parse::<(f64,f64)>()
//             .map(|r| Some(r))
//             .map_err(|e| D::Error::custom(format!("{}", e)))
//     }
//
//     pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//             T: std::fmt::Display,
//     {
//         match value {
//             Some(x) => format!("{}", x),
//             None => format!("")
//         }.serialize(serializer)
//     }
// }

// First pass at catalogus
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, JsonSchema, Clone)]
pub struct CatalogusItem {
    pub name_anglicized: String,
    pub number: String,
    pub name_latin: String,
    pub title: String,
    pub region_anglicized: String,
    pub region_latin: String,
    pub fief_area: String,
    pub fief_latin: String,
    pub knights: String,
    pub knights_shared: String,
    pub villanos: String,
    pub commendatarios: String,
    pub held_from: String,
    pub held_from_latin: String,
    pub augmentum_knights: String,
    pub augmentum_knights_shared: String,
    pub augmentum_sergeant: String,
    pub augmentum_sergeant_shared: String,
    pub augmentum_crossbow: String,
    pub augmentum_crossbow_shared: String,
    pub augmentum_infantry: String,
    pub augmentum_infantry_shared: String,
    pub location: String,
    // #[serde(with = "loc_from_str")]
    // pub location: (f64,f64),
}

impl CatalogusItem {
    pub fn to_js(&self) -> String {
        let js = format!("{{\"loc\": [{}], \"title\":\"{}\", \"Heldfrom\":\"{}\", \"Numknights\": \"{}\", \"Aknights\": \"{}\", \"Aserg\": \"{}\",}}",
                         self.location,
                         // "TODO: Serialize LOC".to_string(),
                         self.title,
                         self.held_from,
                         self.knights,
                         self.augmentum_knights,
                         self.augmentum_sergeant,
        );
        js
    }
}

pub type Catalogus = Vec<CatalogusItem>;
