use super::utility::{Name, NamedAPIResource};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Berry {
    pub id: i32,
    pub name: String,
    pub growth_time: i32,
    pub max_harvest: i32,
    pub natural_gift_power: i32,
    pub size: i32,
    pub smoothness: i32,
    pub soil_dryness: i32,
    pub firmness: NamedAPIResource,
    pub flavors: Vec<BerryFlavorMap>,
    pub item: NamedAPIResource,
    pub natural_gift_type: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BerryFlavorMap {
    pub potency: i32,
    pub flavor: NamedAPIResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BerryFirmness {
    pub id: i32,
    pub name: String,
    pub berries: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BerryFlavor {
    pub id: i32,
    pub name: String,
    pub berries: Vec<FlavorBerryMap>,
    pub contest_type: NamedAPIResource,
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlavorBerryMap {
    pub potency: i32,
    pub berry: NamedAPIResource,
}
