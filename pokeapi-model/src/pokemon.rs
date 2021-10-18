//! Pokémon types.

use crate::utility::common_models::{
    APIResource, Description, Effect, FlavorText, GenerationGameIndex, Name, NamedAPIResource,
    VerboseEffect, VersionEncounterDetail, VersionGameIndex,
};
use pokeapi_macro::pokeapi_struct;
use serde::Deserialize;

#[pokeapi_struct]
struct Ability {
    id: i32,
    name: String,
    is_main_series: bool,
    generation: NamedAPIResource,
    names: Vec<Name>,
    effect_entries: Vec<VerboseEffect>,
    effect_changes: Vec<AbilityEffectChange>,
    flavor_text_entries: Vec<AbilityFlavorText>,
    pokemon: Vec<AbilityPokemon>,
}

#[pokeapi_struct]
struct AbilityEffectChange {
    effect_entries: Vec<Effect>,
    version_group: NamedAPIResource,
}

#[pokeapi_struct]
struct AbilityFlavorText {
    flavor_text: String,
    language: NamedAPIResource,
    version_group: NamedAPIResource,
}

#[pokeapi_struct]
struct AbilityPokemon {
    is_hidden: bool,
    slot: i32,
    pokemon: NamedAPIResource,
}

#[pokeapi_struct]
struct Characteristic {
    id: i32,
    gene_modulo: i32,
    possible_values: i32,
}

#[pokeapi_struct]
struct EggGroup {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct Gender {
    id: i32,
    name: String,
    pokemon_species_details: Vec<PokemonSpeciesGender>,
    required_for_evolution: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct PokemonSpeciesGender {
    rate: i32,
    pokemon_species: NamedAPIResource,
}

#[pokeapi_struct]
struct GrowthRate {
    id: i32,
    name: String,
    formula: String,
    descriptions: Vec<Description>,
    levels: Vec<GrowthRateExperienceLevel>,
    pokemon_species: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct GrowthRateExperienceLevel {
    level: i32,
    experience: i32,
}

#[pokeapi_struct]
struct Nature {
    id: i32,
    name: String,
    decreased_stat: NamedAPIResource,
    increased_stat: NamedAPIResource,
    hates_flavor: NamedAPIResource,
    likes_flavor: NamedAPIResource,
    pokeathlon_stat_changes: Vec<NatureStatChange>,
    move_battle_style_preferences: Vec<MoveBattleStylePreference>,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct NatureStatChange {
    max_change: i32,
    pokeathlon_stat: NamedAPIResource,
}

#[pokeapi_struct]
struct MoveBattleStylePreference {
    low_hp_preference: i32,
    high_hp_preference: i32,
    move_battle_style: NamedAPIResource,
}

#[pokeapi_struct]
struct PokeathlonStat {
    id: i32,
    name: String,
    names: Vec<Name>,
    affecting_natures: NaturePokeathlonStatAffectSets,
}

#[pokeapi_struct]
struct NaturePokeathlonStatAffectSets {
    increase: Vec<NaturePokeathlonStatAffect>,
    decrease: Vec<NaturePokeathlonStatAffect>,
}

#[pokeapi_struct]
struct NaturePokeathlonStatAffect {
    max_change: i32,
    nature: NamedAPIResource,
}

#[pokeapi_struct]
struct Pokemon {
    id: i32,
    name: String,
    base_experience: i32,
    height: i32,
    is_default: bool,
    order: i32,
    weight: i32,
    abilities: Vec<PokemonAbility>,
    forms: Vec<NamedAPIResource>,
    game_indices: Vec<VersionGameIndex>,
    held_items: Vec<PokemonHeldItem>,
    location_area_encounters: String,
    moves: Vec<PokemonMove>,
    sprites: PokemonSprites,
    species: NamedAPIResource,
    stats: Vec<PokemonStat>,
    types: Vec<PokemonType>,
}

#[pokeapi_struct]
struct PokemonAbility {
    is_hidden: bool,
    slot: i32,
    ability: NamedAPIResource,
}

#[pokeapi_struct]
struct PokemonType {
    slot: i32,
    #[serde(rename = "type")]
    type_: NamedAPIResource,
}

#[pokeapi_struct]
struct PokemonHeldItem {
    item: NamedAPIResource,
    version_details: Vec<PokemonHeldItemVersion>,
}

#[pokeapi_struct]
struct PokemonHeldItemVersion {
    version: NamedAPIResource,
    rarity: i32,
}

#[pokeapi_struct]
struct PokemonMove {
    #[serde(rename = "move")]
    move_: NamedAPIResource,
    version_group_details: Vec<PokemonMoveVersion>,
}

#[pokeapi_struct]
struct PokemonMoveVersion {
    move_learn_method: NamedAPIResource,
    version_group: NamedAPIResource,
    level_learned_at: i32,
}

#[pokeapi_struct]
struct PokemonStat {
    stat: NamedAPIResource,
    effort: i32,
    base_stat: i32,
}

#[pokeapi_struct]
struct PokemonSprites {
    front_default: String,
    front_shiny: String,
    front_female: String,
    front_shiny_female: String,
    back_default: String,
    back_shiny: String,
    back_female: String,
    back_shiny_female: String,
}

#[pokeapi_struct]
struct LocationAreaEncounter {
    location_area: NamedAPIResource,
    version_details: Vec<VersionEncounterDetail>,
}

#[pokeapi_struct]
struct PokemonColor {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct PokemonForm {
    id: i32,
    name: String,
    order: i32,
    form_order: i32,
    is_default: bool,
    is_battle_only: bool,
    is_mega: bool,
    form_name: String,
    pokemon: NamedAPIResource,
    sprites: PokemonFormSprites,
    version_group: NamedAPIResource,
    names: Vec<Name>,
    form_names: Vec<Name>,
}

#[pokeapi_struct]
struct PokemonFormSprites {
    front_default: String,
    front_shiny: String,
    back_default: String,
    back_shiny: String,
}

#[pokeapi_struct]
struct PokemonHabitat {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct PokemonShape {
    id: i32,
    name: String,
    awesome_names: Vec<AwesomeName>,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct AwesomeName {
    awesome_name: String,
    language: NamedAPIResource,
}

#[pokeapi_struct]
struct PokemonSpecies {
    id: i32,
    name: String,
    order: i32,
    gender_rate: i32,
    capture_rate: i32,
    base_happiness: i32,
    is_baby: bool,
    is_legendary: bool,
    is_mythical: bool,
    hatch_counter: i32,
    has_gender_differences: bool,
    forms_switchable: bool,
    growth_rate: NamedAPIResource,
    pokedex_numbers: Vec<PokemonSpeciesDexEntry>,
    egg_groups: NamedAPIResource,
    color: NamedAPIResource,
    shape: NamedAPIResource,
    evolves_from_species: NamedAPIResource,
    evolution_chain: APIResource,
    habitat: NamedAPIResource,
    generation: NamedAPIResource,
    names: Vec<Name>,
    pal_park_encounters: Vec<PalParkEncounterArea>,
    flavor_text_entries: Vec<FlavorText>,
    form_descriptions: Vec<Description>,
    genera: Vec<Genus>,
    varieties: Vec<PokemonSpeciesVariety>,
}

#[pokeapi_struct]
struct Genus {
    genus: String,
    language: NamedAPIResource,
}

#[pokeapi_struct]
struct PokemonSpeciesDexEntry {
    entry_number: i32,
    pokedex: NamedAPIResource,
}

#[pokeapi_struct]
struct PalParkEncounterArea {
    base_score: i32,
    rate: i32,
    area: NamedAPIResource,
}

#[pokeapi_struct]
struct PokemonSpeciesVariety {
    is_default: bool,
    pokemon: NamedAPIResource,
}

#[pokeapi_struct]
struct Stat {
    id: i32,
    name: String,
    game_index: i32,
    is_battle_only: bool,
    affecting_moves: MoveStatAffectSets,
    affecting_natures: NatureStatAffectSets,
    characteristics: Vec<APIResource>,
    move_damage_class: NamedAPIResource,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct MoveStatAffectSets {
    increase: Vec<MoveStatAffect>,
    decrease: Vec<MoveStatAffect>,
}

#[pokeapi_struct]
struct MoveStatAffect {
    change: i32,
    #[serde(rename = "move")]
    move_: NamedAPIResource,
}

#[pokeapi_struct]
struct NatureStatAffectSets {
    increase: Vec<NamedAPIResource>,
    decrease: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct Type {
    id: i32,
    name: String,
    damage_relations: TypeRelations,
    game_indices: Vec<GenerationGameIndex>,
    generation: NamedAPIResource,
    move_damage_class: NamedAPIResource,
    names: Vec<Name>,
    pokemon: Vec<TypePokemon>,
    moves: Vec<NamedAPIResource>,
}

#[pokeapi_struct]
struct TypePokemon {
    slot: i32,
    pokemon: NamedAPIResource,
}

#[pokeapi_struct]
struct TypeRelations {
    no_damage_to: Vec<NamedAPIResource>,
    half_damage_to: Vec<NamedAPIResource>,
    double_damage_to: Vec<NamedAPIResource>,
    no_damage_from: Vec<NamedAPIResource>,
    half_damage_from: Vec<NamedAPIResource>,
    double_damage_from: Vec<NamedAPIResource>,
}
