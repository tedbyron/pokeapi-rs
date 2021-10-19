//! Pokémon types.

use crate::berries::BerryFlavor;
use crate::evolution::EvolutionChain;
use crate::games::{Generation, Pokedex, Version, VersionGroup};
use crate::items::Item;
use crate::locations::{LocationArea, PalParkArea};
use crate::moves::{Move, MoveBattleStyle, MoveDamageClass, MoveLearnMethod};
use crate::utility::{
    APIResource, Description, Effect, FlavorText, GenerationGameIndex, Language, Name,
    NamedAPIResource, VerboseEffect, VersionEncounterDetail, VersionGameIndex,
};
use pokeapi_macro::pokeapi_struct;

#[pokeapi_struct]
struct Ability {
    id: i32,
    name: String,
    is_main_series: bool,
    generation: NamedAPIResource<Generation>,
    names: Vec<Name>,
    effect_entries: Vec<VerboseEffect>,
    effect_changes: Vec<AbilityEffectChange>,
    flavor_text_entries: Vec<AbilityFlavorText>,
    pokemon: Vec<AbilityPokemon>,
}

#[pokeapi_struct]
struct AbilityEffectChange {
    effect_entries: Vec<Effect>,
    version_group: NamedAPIResource<VersionGroup>,
}

#[pokeapi_struct]
struct AbilityFlavorText {
    flavor_text: String,
    language: NamedAPIResource<Language>,
    version_group: NamedAPIResource<VersionGroup>,
}

#[pokeapi_struct]
struct AbilityPokemon {
    is_hidden: bool,
    slot: i32,
    pokemon: NamedAPIResource<Pokemon>,
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
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

#[pokeapi_struct]
struct Gender {
    id: i32,
    name: String,
    pokemon_species_details: Vec<PokemonSpeciesGender>,
    required_for_evolution: Vec<NamedAPIResource<PokemonSpecies>>,
}

#[pokeapi_struct]
struct PokemonSpeciesGender {
    rate: i32,
    pokemon_species: NamedAPIResource<PokemonSpecies>,
}

#[pokeapi_struct]
struct GrowthRate {
    id: i32,
    name: String,
    formula: String,
    descriptions: Vec<Description>,
    levels: Vec<GrowthRateExperienceLevel>,
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
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
    decreased_stat: NamedAPIResource<Stat>,
    increased_stat: NamedAPIResource<Stat>,
    hates_flavor: NamedAPIResource<BerryFlavor>,
    likes_flavor: NamedAPIResource<BerryFlavor>,
    pokeathlon_stat_changes: Vec<NatureStatChange>,
    move_battle_style_preferences: Vec<MoveBattleStylePreference>,
    names: Vec<Name>,
}

#[pokeapi_struct]
struct NatureStatChange {
    max_change: i32,
    pokeathlon_stat: NamedAPIResource<PokeathlonStat>,
}

#[pokeapi_struct]
struct MoveBattleStylePreference {
    low_hp_preference: i32,
    high_hp_preference: i32,
    move_battle_style: NamedAPIResource<MoveBattleStyle>,
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
    nature: NamedAPIResource<Nature>,
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
    forms: Vec<NamedAPIResource<PokemonForm>>,
    game_indices: Vec<VersionGameIndex>,
    held_items: Vec<PokemonHeldItem>,
    location_area_encounters: String,
    moves: Vec<PokemonMove>,
    sprites: PokemonSprites,
    species: NamedAPIResource<PokemonSpecies>,
    stats: Vec<PokemonStat>,
    types: Vec<PokemonType>,
}

#[pokeapi_struct]
struct PokemonAbility {
    is_hidden: bool,
    slot: i32,
    ability: NamedAPIResource<Ability>,
}

#[pokeapi_struct]
struct PokemonType {
    slot: i32,
    #[serde(rename = "type")]
    type_: NamedAPIResource<Type>,
}

#[pokeapi_struct]
struct PokemonHeldItem {
    item: NamedAPIResource<Item>,
    version_details: Vec<PokemonHeldItemVersion>,
}

#[pokeapi_struct]
struct PokemonHeldItemVersion {
    version: NamedAPIResource<Version>,
    rarity: i32,
}

#[pokeapi_struct]
struct PokemonMove {
    #[serde(rename = "move")]
    move_: NamedAPIResource<Move>,
    version_group_details: Vec<PokemonMoveVersion>,
}

#[pokeapi_struct]
struct PokemonMoveVersion {
    move_learn_method: NamedAPIResource<MoveLearnMethod>,
    version_group: NamedAPIResource<VersionGroup>,
    level_learned_at: i32,
}

#[pokeapi_struct]
struct PokemonStat {
    stat: NamedAPIResource<Stat>,
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
    location_area: NamedAPIResource<LocationArea>,
    version_details: Vec<VersionEncounterDetail>,
}

#[pokeapi_struct]
struct PokemonColor {
    id: i32,
    name: String,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
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
    pokemon: NamedAPIResource<Pokemon>,
    sprites: PokemonFormSprites,
    version_group: NamedAPIResource<VersionGroup>,
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
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

#[pokeapi_struct]
struct PokemonShape {
    id: i32,
    name: String,
    awesome_names: Vec<AwesomeName>,
    names: Vec<Name>,
    pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

#[pokeapi_struct]
struct AwesomeName {
    awesome_name: String,
    language: NamedAPIResource<Language>,
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
    growth_rate: NamedAPIResource<GrowthRate>,
    pokedex_numbers: Vec<PokemonSpeciesDexEntry>,
    egg_groups: NamedAPIResource<EggGroup>,
    color: NamedAPIResource<PokemonColor>,
    shape: NamedAPIResource<PokemonShape>,
    evolves_from_species: NamedAPIResource<EvolutionChain>,
    evolution_chain: APIResource<EvolutionChain>,
    habitat: NamedAPIResource<PokemonHabitat>,
    generation: NamedAPIResource<Generation>,
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
    language: NamedAPIResource<Language>,
}

#[pokeapi_struct]
struct PokemonSpeciesDexEntry {
    entry_number: i32,
    pokedex: NamedAPIResource<Pokedex>,
}

#[pokeapi_struct]
struct PalParkEncounterArea {
    base_score: i32,
    rate: i32,
    area: NamedAPIResource<PalParkArea>,
}

#[pokeapi_struct]
struct PokemonSpeciesVariety {
    is_default: bool,
    pokemon: NamedAPIResource<Pokemon>,
}

#[pokeapi_struct]
struct Stat {
    id: i32,
    name: String,
    game_index: i32,
    is_battle_only: bool,
    affecting_moves: MoveStatAffectSets,
    affecting_natures: NatureStatAffectSets,
    characteristics: Vec<APIResource<Characteristic>>,
    move_damage_class: NamedAPIResource<MoveDamageClass>,
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
    move_: NamedAPIResource<Move>,
}

#[pokeapi_struct]
struct NatureStatAffectSets {
    increase: Vec<NamedAPIResource<Nature>>,
    decrease: Vec<NamedAPIResource<Nature>>,
}

#[pokeapi_struct]
struct Type {
    id: i32,
    name: String,
    damage_relations: TypeRelations,
    game_indices: Vec<GenerationGameIndex>,
    generation: NamedAPIResource<Generation>,
    move_damage_class: NamedAPIResource<MoveDamageClass>,
    names: Vec<Name>,
    pokemon: Vec<TypePokemon>,
    moves: Vec<NamedAPIResource<Move>>,
}

#[pokeapi_struct]
struct TypePokemon {
    slot: i32,
    pokemon: NamedAPIResource<Pokemon>,
}

#[pokeapi_struct]
struct TypeRelations {
    no_damage_to: Vec<NamedAPIResource<Type>>,
    half_damage_to: Vec<NamedAPIResource<Type>>,
    double_damage_to: Vec<NamedAPIResource<Type>>,
    no_damage_from: Vec<NamedAPIResource<Type>>,
    half_damage_from: Vec<NamedAPIResource<Type>>,
    double_damage_from: Vec<NamedAPIResource<Type>>,
}
