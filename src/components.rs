use bevy::{ecs::query::QueryData, prelude::*};
use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
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

#[derive(Component, Default, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct Stat {
    pub base: f64,
    pub total: f64,
    pub deps: Vec<StatEnum>,
}

impl Stat {
    pub fn new(num: f64, deps: Vec<StatEnum>) -> Self {
        Self {
            base: num,
            total: num,
            deps,
        }
    }
    pub fn calculate_total(
        &mut self,
        replace: Option<f64>,
        best: Option<f64>,
        add: f64,
        mult: f64,
        parent_modifier: f64,
    ) -> bool {
        info!("inside maxhealth calculation");
        info!("current base: {}", self.base);
        info!("current total: {}", self.total);
        if let Some(rep) = replace {
            if self.total != rep {
                self.total = rep;
                return true;
            } else {
                return false;
            }
        }

        info!("no replace mods");
        let mut new_total = self.base;
        new_total += add;
        new_total += parent_modifier;
        new_total *= 1. + mult;
        new_total = if let Some(b) = best {
            b.max(new_total)
        } else {
            new_total
        };
        if new_total != self.total {
            self.total = new_total;
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Component, Default, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct StatModList(pub Vec<StatMod>);

#[derive(Component, Default, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct StatMod {
    pub stat: StatEnum,
    pub value: f64,
    pub mod_type: ModType,
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Ability {
    pub stat: Stat,
    pub proficiency: Proficiency,
}

impl Ability {
    pub fn calculate_modifier(&self) -> f64 {
        ((self.stat.total - 10.) / 2.).floor()
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Strength(pub Ability);

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Constitution(pub Ability);

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Dexterity(pub Ability);

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Intelligence(pub Ability);

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Wisdom(pub Ability);

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct Charisma(pub Ability);

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

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Athletics(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Acrobatics(pub Skill);

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AcrobaticsUpdateQ {
    stat: &'static mut Acrobatics,
    par: AcroParQ,
}
#[derive(QueryData)]
pub struct AcroParQ {
    s1: &'static Dexterity,
}

impl<'a> AcroParQItem<'a> {
    fn mods(&self) -> f64 {
        self.s1.calculate_modifier()
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct SleightOfHand(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Stealth(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Arcana(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct History(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Investigation(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Nature(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Religion(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct AnimalHandling(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Insight(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Medicine(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Perception(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Survival(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Deception(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Intimidation(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Performance(pub Skill);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Persuasion(pub Skill);

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

#[derive(Component, Default, Clone, Reflect, PartialEq)]
#[reflect(Component)]
pub struct ArmorClass(pub Stat);

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Speed(pub Stat);

#[derive(Component, Reflect, Default, Clone, PartialEq)]
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
pub struct Xp(pub f64);

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

#[derive(Component, Default, PartialEq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Health(pub f64);

#[derive(Component, Default, PartialEq, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MaxHealth(pub Stat);

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

#[derive(Component, Reflect, Default, Clone, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub enum ModType {
    #[default]
    Add,
    Mult,
    Replace,
    BestOf,
}

#[derive(Component, Reflect, Default, Clone, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub enum StatEnum {
    #[default]
    MaxHealth,
    ArmorClass,
    DarkVision,
    Strength,
    Constitution,
    Dexterity,
    Intelligence,
    Wisdom,
    Charisma,
    Athletics,
    Acrobatics,
    SleightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}
