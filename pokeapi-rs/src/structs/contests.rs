use super::utility::NamedAPIResource;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ContestType {
    pub id: i32,
    pub name: String,
    pub berry_flavor: NamedAPIResource,
    pub names: Vec<ContestName>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContestName {
    pub id: i32,
    pub name: String,
}
