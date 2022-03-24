//! Move types.

use pokeapi_macro::pokeapi_struct;

use crate::{
    APIResource, AbilityEffectChange, ContestEffect, ContestType, Description, Generation,
    Language, MachineVersionDetail, Name, NamedAPIResource, Stat, SuperContestEffect, Type,
    VerboseEffect, VersionGroup,
};

#[pokeapi_struct]
struct Move {
    id: i32,
    name: String,
    accuracy: i32,
    effect_chance: i32,
    pp: i32,
    priority: i32,
    power: i32,
    contest_combos: ContestComboSets,
    contest_type: NamedAPIResource<ContestType>,
    contest_effect: APIResource<ContestEffect>,
    damage_class: NamedAPIResource<MoveDamageClass>,
    effect_entries: Vec<VerboseEffect>,
    effect_changes: Vec<AbilityEffectChange>,
    flavor_text_entries: Vec<MoveFlavorText>,
    generation: NamedAPIResource<Generation>,
    machines: Vec<MachineVersionDetail>,
    meta: MoveMetaData,
    names: Vec<Name>,
    past_values: Vec<PastMoveStatValues>,
    stat_changes: Vec<MoveStatChange>,
    super_contest_effect: APIResource<SuperContestEffect>,
    target: NamedAPIResource<MoveTarget>,
    #[serde(rename = "type")]
    type_: NamedAPIResource<Type>,
}

#[pokeapi_struct]
struct ContestComboSets {
    normal: ContestComboDetail,
    #[serde(rename = "super")]
    super_: ContestComboDetail,
}

#[pokeapi_struct]
struct ContestComboDetail {
    use_before: Vec<NamedAPIResource<Move>>,
    use_after: Vec<NamedAPIResource<Move>>,
}

#[pokeapi_struct]
struct MoveFlavorText {
    flavor_text: String,
    language: NamedAPIResource<Language>,
    version_group: NamedAPIResource<VersionGroup>,
}

#[pokeapi_struct]
struct MoveMetaData {
    ailment: NamedAPIResource<MoveAilment>,
    category: NamedAPIResource<MoveCategory>,
    min_hits: i32,
    max_hits: i32,
    min_turns: i32,
    max_turns: i32,
    drain: i32,
    healing: i32,
    crit_rate: i32,
    ailment_chance: i32,
    flinch_chance: i32,
    stat_chance: i32,
}

#[pokeapi_struct]
struct MoveStatChange {
    change: i32,
    stat: NamedAPIResource<Stat>,
}

#[pokeapi_struct]
struct PastMoveStatValues {
    accuracy: i32,
    effect_chance: i32,
    power: i32,
    pp: i32,
    effect_entries: Vec<VerboseEffect>,
    #[serde(rename = "type")]
    type_: NamedAPIResource<Type>,
    version_group: NamedAPIResource<VersionGroup>,
}

#[pokeapi_struct]
struct MoveAilment {
    id: i32,
    name: String,
    moves: Vec<NamedAPIResource<Move>>,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct MoveBattleStyle {
    id: i32,
    name: String,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct MoveCategory {
    id: i32,
    name: String,
    moves: Vec<NamedAPIResource<Move>>,
    descriptions: Vec<Description>,
}

#[pokeapi_struct]
struct MoveDamageClass {
    id: i32,
    name: String,
    descriptions: Vec<Description>,
    moves: Vec<NamedAPIResource<Move>>,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct MoveLearnMethod {
    id: i32,
    name: String,
    descriptions: Vec<Description>,
    names: Vec<Name>,
    version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

#[pokeapi_struct]
struct MoveTarget {
    id: i32,
    name: String,
    descriptions: Vec<Description>,
    moves: Vec<NamedAPIResource<Move>>,
    names: Vec<Name>,
}
