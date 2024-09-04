use bevy::prelude::*;
use std::{collections::HashMap, marker::PhantomData};
use strum::{Display, EnumIter};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ComponentRegistry {
    pub player: PlayerBundle,
    pub weapon: WeaponBundle,
    pub spell: SpellBundle,
}

#[derive(Bundle, Resource, Default, Reflect)]
pub struct PlayerBundle {
    pub player_tag: Player,
    pub unit_tag: Unit,
    pub name: UnitName,
    pub player_name: PlayerName,
    pub ac: ArmorClass,
    pub speed: Speed,
    pub abilities: AbilitiesBundle,
    pub skills: SkillsBundle,
    pub race: Race,
    pub class: Class,
    pub wep_profs: WeaponProficiencies,
    pub prof_bonus: ProficiencyBonus,
    pub health: Health,
    pub max_health: MaxHealth,
    pub background: Background,
    pub alignment: Alignment,
    pub xp: Xp,
    pub level: Level,
    pub hit_dice: HitDice,
    pub settings: SettingsBundle,
}

#[derive(Bundle, Resource, Default, Reflect)]
pub struct EnemyBundle {
    pub unit_tag: Unit,
    pub enemy_tag: Enemy,
    pub name: UnitName,
    pub ac: ArmorClass,
    pub speed: Speed,
    pub abilities: AbilitiesBundle,
    pub skills: SkillsBundle,
    pub wep_profs: WeaponProficiencies,
    pub prof_bonus: ProficiencyBonus,
    pub health: Health,
    pub max_health: MaxHealth,
    pub alignment: Alignment,
    pub hit_dice: HitDice,
}

#[derive(Component)]
pub enum ButtonType {
    NewCharacter,
    LoadCharacter,
}

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct RootUI;

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Stat {
    pub base: i64,
    pub total: i64,
}

impl Stat {
    pub fn new(val: i64) -> Self {
        Self {
            base: val,
            total: val,
        }
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Ability {
    pub stat: Stat,
    pub modifier: i64,
    pub proficiency: Proficiency,
}

impl Ability {
    pub fn calculate_modifier(&mut self) {
        let new_mod = ((self.stat.total as f64 - 10.) / 2.).floor() as i64;
        if new_mod != self.modifier {
            self.modifier = new_mod;
        }
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Defense(pub Stat);

impl Moddable for Defense {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Strength(pub Ability);

impl Moddable for Strength {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0.stat
    }
}

impl AbilityTrait for Strength {
    fn set_base(&mut self, val: i64) {
        self.0.stat.base = val;
    }

    fn set_value(&mut self, val: i64) {
        self.0.stat.total = val;
    }

    fn base(&self) -> i64 {
        self.0.stat.base
    }

    fn value(&self) -> i64 {
        self.0.stat.total
    }

    fn modifier(&self) -> i64 {
        self.0.modifier
    }

    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }

    fn to_string(&self) -> String {
        "Strength".into()
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Constitution(pub Ability);

impl Moddable for Constitution {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0.stat
    }
}

impl AbilityTrait for Constitution {
    fn set_base(&mut self, val: i64) {
        self.0.stat.base = val;
    }

    fn set_value(&mut self, val: i64) {
        self.0.stat.total = val;
    }

    fn base(&self) -> i64 {
        self.0.stat.base
    }

    fn value(&self) -> i64 {
        self.0.stat.total
    }

    fn modifier(&self) -> i64 {
        self.0.modifier
    }

    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }

    fn to_string(&self) -> String {
        "Constitution".into()
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Dexterity(pub Ability);

impl Moddable for Dexterity {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0.stat
    }
}

impl AbilityTrait for Dexterity {
    fn set_base(&mut self, val: i64) {
        self.0.stat.base = val;
    }

    fn set_value(&mut self, val: i64) {
        self.0.stat.total = val;
    }

    fn base(&self) -> i64 {
        self.0.stat.base
    }

    fn value(&self) -> i64 {
        self.0.stat.total
    }

    fn modifier(&self) -> i64 {
        self.0.modifier
    }

    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }

    fn to_string(&self) -> String {
        "Dexterity".into()
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Intelligence(pub Ability);

impl Moddable for Intelligence {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0.stat
    }
}

impl AbilityTrait for Intelligence {
    fn set_base(&mut self, val: i64) {
        self.0.stat.base = val;
    }

    fn set_value(&mut self, val: i64) {
        self.0.stat.total = val;
    }

    fn base(&self) -> i64 {
        self.0.stat.base
    }

    fn value(&self) -> i64 {
        self.0.stat.total
    }

    fn modifier(&self) -> i64 {
        self.0.modifier
    }

    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }

    fn to_string(&self) -> String {
        "Intelligence".into()
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Wisdom(pub Ability);

impl Moddable for Wisdom {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0.stat
    }
}

impl AbilityTrait for Wisdom {
    fn set_base(&mut self, val: i64) {
        self.0.stat.base = val;
    }

    fn set_value(&mut self, val: i64) {
        self.0.stat.total = val;
    }

    fn base(&self) -> i64 {
        self.0.stat.base
    }

    fn value(&self) -> i64 {
        self.0.stat.total
    }

    fn modifier(&self) -> i64 {
        self.0.modifier
    }

    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }

    fn to_string(&self) -> String {
        "Wisdom".into()
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Charisma(pub Ability);

impl Moddable for Charisma {
    fn stat<'a>(&'a mut self) -> &'a mut Stat {
        &mut self.0.stat
    }
}

impl AbilityTrait for Charisma {
    fn set_base(&mut self, val: i64) {
        self.0.stat.base = val;
    }

    fn set_value(&mut self, val: i64) {
        self.0.stat.total = val;
    }

    fn base(&self) -> i64 {
        self.0.stat.base
    }

    fn value(&self) -> i64 {
        self.0.stat.total
    }

    fn modifier(&self) -> i64 {
        self.0.modifier
    }

    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }

    fn to_string(&self) -> String {
        "Charisma".into()
    }
}

pub trait AbilityTrait: Component {
    fn set_base(&mut self, val: i64);

    fn set_value(&mut self, val: i64);

    fn base(&self) -> i64;

    fn value(&self) -> i64;

    fn modifier(&self) -> i64;

    fn proficiency(&self) -> &Proficiency;

    fn to_string(&self) -> String;
}

#[derive(Bundle, Default, Clone, Reflect)]
pub struct AbilitiesBundle {
    pub str: Strength,
    pub con: Constitution,
    pub dex: Dexterity,
    pub int: Intelligence,
    pub wis: Wisdom,
    pub cha: Charisma,
}

#[derive(Default, Reflect, Component, Debug, PartialEq, Eq, Clone)]
#[reflect(Component)]
pub enum Proficiency {
    #[default]
    None,
    Proficient,
    Expert,
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Skill {
    pub stat: Stat,
    pub proficiency: Proficiency,
}

pub trait SkillTrait: Component {
    fn value(&self) -> i64;

    fn proficiency(&self) -> &Proficiency;

    fn to_string(&self) -> String;
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Athletics(pub Skill);

impl SkillTrait for Athletics {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Athletics".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Acrobatics(pub Skill);

impl SkillTrait for Acrobatics {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Acrobatics".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct SleightOfHand(pub Skill);

impl SkillTrait for SleightOfHand {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "SleightOfHand".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Stealth(pub Skill);

impl SkillTrait for Stealth {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Stealth".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Arcana(pub Skill);

impl SkillTrait for Arcana {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Arcana".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct History(pub Skill);

impl SkillTrait for History {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "History".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Investigation(pub Skill);

impl SkillTrait for Investigation {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Investigation".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Nature(pub Skill);

impl SkillTrait for Nature {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Nature".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Religion(pub Skill);

impl SkillTrait for Religion {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Religion".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct AnimalHandling(pub Skill);

impl SkillTrait for AnimalHandling {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "AnimalHandling".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Insight(pub Skill);

impl SkillTrait for Insight {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Insight".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Medicine(pub Skill);

impl SkillTrait for Medicine {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Medicine".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Perception(pub Skill);

impl SkillTrait for Perception {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Perception".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Survival(pub Skill);

impl SkillTrait for Survival {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Survival".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Deception(pub Skill);

impl SkillTrait for Deception {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Deception".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Intimidation(pub Skill);

impl SkillTrait for Intimidation {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Intimidation".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Performance(pub Skill);

impl SkillTrait for Performance {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Performance".into()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Persuasion(pub Skill);

impl SkillTrait for Persuasion {
    fn value(&self) -> i64 {
        self.0.stat.total
    }
    fn proficiency<'a>(&'a self) -> &'a Proficiency {
        &self.0.proficiency
    }
    fn to_string(&self) -> String {
        "Persuasion".into()
    }
}

#[derive(Bundle, Default, Clone, Reflect)]
pub struct SkillsBundle {
    pub athletics: Athletics,
    pub acrobatics: Acrobatics,
    pub sleight_of_hand: SleightOfHand,
    pub stealth: Stealth,
    pub arcana: Arcana,
    pub history: History,
    pub investigation: Investigation,
    pub nature: Nature,
    pub religion: Religion,
    pub animal_handling: AnimalHandling,
    pub insight: Insight,
    pub medicine: Medicine,
    pub perception: Perception,
    pub survival: Survival,
    pub deception: Deception,
    pub intimidation: Intimidation,
    pub performance: Performance,
    pub persuasion: Persuasion,
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct SimpleWeaponProficiency(pub Proficiency);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct MartialWeaponProficiency(pub Proficiency);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct IndividualWeaponProficiency(pub Vec<ItemName>);

#[derive(Bundle, Default, Clone, Reflect)]
pub struct WeaponProficiencies {
    pub simple: SimpleWeaponProficiency,
    pub martial: MartialWeaponProficiency,
    pub ind: IndividualWeaponProficiency,
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Unit;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct UnitName(pub String);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct PlayerName(pub String);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct ArmorClass(pub i64);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Speed(pub i64);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DarkVision(pub Stat);

#[derive(Component, Default, EnumIter, Display, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component)]
pub enum Race {
    #[default]
    DragonBorn,
    HillDwarf,
    MountainDwarf,
    HighElf,
    WoodElf,
    DarkElf,
    ForestGnome,
    RockGnome,
    HalfElf,
    LightfootHalfling,
    StoutHalfling,
    HalfOrc,
    Human,
    Tiefling,
}
#[derive(Component, Default, EnumIter, Display, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component)]
pub enum Class {
    #[default]
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(Component, Default, PartialEq, Eq, EnumIter, Debug, Display, Clone, Reflect)]
#[reflect(Component)]
pub enum Background {
    #[default]
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    Gladiator,
    GuildArtisan,
    Hermit,
    Knight,
    Noble,
    Outlander,
    Pirate,
    Sage,
    Sailor,
    Soldier,
    Urchin,
}

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Level(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ProficiencyBonus(pub i64);

impl ProficiencyBonus {
    pub fn from_level(level: i64) -> Self {
        Self(2 + ((level - 1) / 4))
    }
}

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Xp(pub u64);

#[derive(Component, Default, PartialEq, Eq, EnumIter, Debug, Display, Clone, Reflect)]
#[reflect(Component)]
pub enum Alignment {
    #[default]
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    Neutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Spell;

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SpellName(pub String);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Weapon;

#[derive(Debug, Clone, Default, PartialEq, Eq, EnumIter, Display, Reflect, Component)]
#[reflect(Component)]
pub enum DiceType {
    D2,
    D4,
    #[default]
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl DiceType {
    pub fn upper_limit(&self) -> i64 {
        match self {
            DiceType::D2 => 3,
            DiceType::D4 => 5,
            DiceType::D6 => 7,
            DiceType::D8 => 9,
            DiceType::D10 => 11,
            DiceType::D12 => 13,
            DiceType::D20 => 21,
            DiceType::D100 => 101,
        }
    }
}

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Dice {
    pub dice_type: DiceType,
    pub number: i64,
}

#[derive(Bundle, Default, Reflect)]
pub struct DamageBundle {
    pub damage_type: DamageType,
    pub base_damage: BaseDamage,
    pub dice: Dice,
}

#[derive(Bundle, Default, Reflect)]
pub struct ItemBundle {
    pub item_marker: Item,
    pub name: ItemName,
    pub weight: Weight,
    pub cost: Cost,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub enum WeaponType {
    #[default]
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
}

#[derive(Bundle, Default, Reflect)]
pub struct WeaponBundle {
    pub weapon_marker: Weapon,
    pub item_bundle: ItemBundle,
    pub damage: DamageBundle,
    pub weapon_type: WeaponType,
}

// #[derive(Component, Default, Reflect)]
// pub struct AttackBundle {
//     primary_ability: Box<dyn Ability>,
// }

#[derive(Bundle, Default, Reflect)]
pub struct SpellBundle {
    pub spell_marker: Spell,
    pub damage: DamageBundle,
    pub spell_name: SpellName,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum DamageType {
    Acid,
    #[default]
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum Target {
    #[default]
    Enemy,
    Team,
    Ally,
    SelfTarget,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum CritType {
    #[default]
    DoubleDamage,
    DoubleDice,
}

#[derive(Bundle, Default, Reflect)]
pub struct SettingsBundle {
    pub crit: CritType,
}

#[derive(Default, Reflect)]
pub enum ActionType {
    Movement,
    #[default]
    Standard,
    Bonus,
    Reaction,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Action {
    name: String,
    action_type: ActionType,
}

// #[derive(Component, Default, Reflect)]
// #[reflect(Component)]
// pub struct PlayerActions(Vec<Action>);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Weight(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NumberOfDice(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct BaseDamage(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Cost(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ItemName(pub String);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Range(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Health(pub i64);

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MaxHealth(pub i64);

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct HitDice(pub Dice);

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct AttackModifier(pub i64);

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct DamageModifier(pub i64);

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Advantage;

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Disadvantage;

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub enum Cover {
    #[default]
    Half,
    ThreeQuarters,
    Total,
}

#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MaxRange(pub i64);

#[derive(Component, Default, Reflect, Ord, PartialEq, PartialOrd, Eq)]
#[reflect(Component)]
pub struct UiAction(pub u64);

// WEAPON PROPERTIES
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ammunition(pub ItemName);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Finesse;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Heavy;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Light;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Loading;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Reach;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Thrown;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TwoHanded;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Versatile;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Mod<T: Moddable> {
    pub mod_type: ModType,
    pub value: f64,
    #[reflect(ignore)]
    _marker: PhantomData<T>,
}

impl<T: Moddable> Mod<T> {
    pub fn new(mod_type: ModType, value: f64) -> Self {
        Self {
            mod_type,
            value,
            _marker: PhantomData,
        }
    }
}

pub trait Moddable: Send + Sync {
    fn stat(&mut self) -> &mut Stat;
}

#[derive(Component, Reflect, Default, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub enum ModType {
    #[default]
    Add,
    Mult,
    Replace,
    BestOf,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum ModdableEnum {
    #[default]
    None,
    Health(Health),
    MaxHealth(MaxHealth),
    ArmorClass(ArmorClass),
    DarkVision(DarkVision),
}
