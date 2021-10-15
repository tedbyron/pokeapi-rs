use serde::{de::DeserializeOwned, Deserialize};
use super::utility::{NamedAPIResource};

#[derive(Deserialize)]
pub struct Berry<'a> {
    id: i32,
    name: &'a str,
    growth_time: i32,
    max_harvest: i32,
    natural_gift_power: i32,
    size: i32,
    smoothness: i32,
    soil_dryness: i32,
    firmness: NamedAPIResource<'a>,
    flavors: Vec<BerryFlavorMap>,
    item: NamedAPIResource<'a>,
    natural_gift_type: NamedAPIResource<'a>,
}

struct BerryFlavorMap {
    potency: ,
    flavor: ,
}
