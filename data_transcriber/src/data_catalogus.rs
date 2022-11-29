use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// First pass at catalogus
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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
}

impl CatalogusItem {
    pub fn to_js(&self) -> String {
        let js = format!("{{\"loc\": [{}], \"title\":\"{}\", \"Heldfrom\":\"{}\", \"Numknights\": \"{}\", \"Aknights\": \"{}\", \"Aserg\": \"{}\",}}",
                         self.location,
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
