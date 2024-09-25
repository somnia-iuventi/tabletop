use crate::components::*;
use crate::items::{ItemsEnum, SpawnItem};
use crate::AppState;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::utils::tracing::info;
use rand::Rng;
use std::marker::PhantomData;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AbilityRollPlugin);
        app.add_plugins(SkillRollPlugin);
        app.add_plugins(Wireframe2dPlugin);
        app.init_resource::<KeyCombo>();
        app.add_systems(OnEnter(AppState::InGame), in_game_setup);
        app.add_systems(Update, keyboard_input.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, paused_menu.run_if(in_state(InGameState::Paused)));
        // app.add_systems(
        //     Update,
        //     narrative_ui.run_if(in_state(InGameState::Narrative)),
        // );
        app.add_systems(Update, combat_ui.run_if(in_state(InGameState::Combat)));
        app.add_event::<Attack>();
        // app.observe(handle_attack);
        app.observe(handle_taking_damage);
        // app.insert_resource(InCombat(false));
        app.add_sub_state::<InGameState>();
    }
}

struct AbilityRollPlugin;

impl Plugin for AbilityRollPlugin {
    fn build(&self, app: &mut App) {}
}

struct SkillRollPlugin;

impl Plugin for SkillRollPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HandleSkillAction>();
        app.observe(handle_skill_action);
        // app.observe(handle_s)
    }
}

fn in_game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<Entity, With<Player>>,
) {
    let ent = player_query.single();
    commands.entity(ent).insert(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        material: materials.add(Color::Srgba(Srgba::BLUE)),
        transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
            0.0, 0.0, 0.0,
        ),
        ..default()
    });
    commands.trigger(SpawnItem(ItemsEnum::RingOfHealth));
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut combo: ResMut<KeyCombo>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        info!("{:?}: Length: {}", combo.0, combo.0.len());
        match combo.0.len() {
            3 => commands.trigger(HandleAbilityAction),
            5 => commands.trigger(HandleSkillAction),
            _ => combo.0.clear(),
        }
    }
    let dont_track = [KeyCode::Enter, KeyCode::Escape];
    for k in keys
        .get_just_released()
        .filter(|x| !dont_track.contains(*x))
    {
        combo.0.push(*k);
    }
    if keys.just_pressed(KeyCode::Escape) {
        combo.0.clear();
    }
    if keys.just_pressed(KeyCode::KeyU) {}
}

// fn handle_ability_action(
//     _trigger: Trigger<HandleAbilityAction>,
//     mut commands: Commands,
//     mut combo: ResMut<KeyCombo>,
// ) {
//     match combo.0[0] {
//         // Ability
//         KeyCode::KeyA => {
//             match combo.0[1] {
//                 // Ability Check
//                 KeyCode::KeyC => match combo.0[2] {
//                     KeyCode::KeyS => commands
//                         .trigger(AbilityRoll::<Strength>::new(AbilityRollType::AbilityCheck)),
//                     KeyCode::KeyC => commands.trigger(AbilityRoll::<Constitution>::new(
//                         AbilityRollType::AbilityCheck,
//                     )),
//                     KeyCode::KeyD => commands
//                         .trigger(AbilityRoll::<Dexterity>::new(AbilityRollType::AbilityCheck)),
//                     KeyCode::KeyI => commands.trigger(AbilityRoll::<Intelligence>::new(
//                         AbilityRollType::AbilityCheck,
//                     )),
//                     KeyCode::KeyW => {
//                         commands.trigger(AbilityRoll::<Wisdom>::new(AbilityRollType::AbilityCheck))
//                     }
//                     KeyCode::KeyH => commands
//                         .trigger(AbilityRoll::<Charisma>::new(AbilityRollType::AbilityCheck)),
//                     _ => {}
//                 },
//                 // Ability Saving Throw
//                 KeyCode::KeyS => {
//                     match combo.0[2] {
//                         KeyCode::KeyS => commands
//                             .trigger(AbilityRoll::<Strength>::new(AbilityRollType::SavingThrow)),
//                         KeyCode::KeyC => commands.trigger(AbilityRoll::<Constitution>::new(
//                             AbilityRollType::SavingThrow,
//                         )),
//                         KeyCode::KeyD => commands
//                             .trigger(AbilityRoll::<Dexterity>::new(AbilityRollType::SavingThrow)),
//                         KeyCode::KeyI => commands.trigger(AbilityRoll::<Intelligence>::new(
//                             AbilityRollType::SavingThrow,
//                         )),
//                         KeyCode::KeyW => commands
//                             .trigger(AbilityRoll::<Wisdom>::new(AbilityRollType::SavingThrow)),
//                         KeyCode::KeyH => commands
//                             .trigger(AbilityRoll::<Charisma>::new(AbilityRollType::SavingThrow)),
//                         _ => {}
//                     }
//                 }
//                 _ => {}
//             }
//         }
//         // Skill
//         // KeyCode::KeyS => {
//         //     match combo.0[1] {
//         //
//         //     }
//         // }
//         _ => {}
//     }
//     combo.0.clear();
// }

fn handle_skill_action(
    _trigger: Trigger<HandleSkillAction>,
    mut commands: Commands,
    mut combo: ResMut<KeyCombo>,
) {
}

fn combat_ui(
    mut commands: Commands,
    mut set_state: ResMut<NextState<AppState>>,
    actions_query: Query<(Entity, &ItemName, &UiAction)>,
    abilities_query: Query<
        (
            &Strength,
            &Constitution,
            &Dexterity,
            &Intelligence,
            &Wisdom,
            &Charisma,
        ),
        With<Player>,
    >,
    target_query: Query<Entity, With<Enemy>>,
) {
}

fn roll(dice: &Dice) -> i64 {
    let mut total = 0;
    let mut rng = rand::thread_rng();
    for each in 0..dice.number {
        let roll: i64 = rng.gen_range(1..dice.dice_type.upper_limit());
        total += roll;
    }
    total
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
enum InGameState {
    #[default]
    // Narrative,
    Combat,
    Paused,
}

#[derive(Resource, Default)]
struct KeyCombo(Vec<KeyCode>);

// #[derive(Resource)]
// struct InCombat(bool);

#[derive(Event)]
struct HandleAbilityAction;

#[derive(Event)]
struct HandleSkillAction;

#[derive(Event)]
struct StatRoll {
    rolltype: RollType,
    stat: StatEnum,
}

enum RollType {
    Check,
    SavingThrow,
}

#[derive(Event)]
struct Attack {
    from: Entity,
    with: Entity,
    to: Entity,
}

#[derive(Event)]
struct TakeDamage {
    unit: Entity,
    amount: f64,
}

fn paused_menu(mut commands: Commands) {}

// fn narrative_ui(mut contexts: EguiContexts, mut commands: Commands, mut combat: ResMut<InCombat>) {
//     combat.0 = false;
//     let ctx = contexts.ctx_mut();
//     egui::Window::new("narrative")
//         .movable(false)
//         .show(ctx, |ui| {});
// }

// fn handle_roll<T: Component>(
//     trigger: Trigger<StatRoll>,
//     ab_query: Query<&T, With<Player>>,
//     prof_query: Query<&ProficiencyBonus, With<Player>>,
// ) {
//     let dice = Dice {
//         number: 1,
//         dice_type: DiceType::D20,
//     };
//     let rolled = roll(&dice);
//     let ability = ab_query.single();
//     let modifier = ability.modifier();
//     let prof = ability.proficiency();
//     let ab_string = ability.to_string();
//     let (total, resp) = match trigger.event().rolltype {
//         AbilityRollType::AbilityCheck => (rolled + modifier, format!("{ab_string} check")),
//         AbilityRollType::SavingThrow => match prof {
//             Proficiency::None => (rolled + modifier, format!("{ab_string} saving throw")),
//             Proficiency::Proficient => {
//                 let bonus = prof_query.single().0;
//                 (
//                     rolled + modifier + bonus,
//                     format!("{ab_string} saving throw"),
//                 )
//             }
//             Proficiency::Expert => {
//                 let bonus = prof_query.single().0;
//                 (
//                     rolled + modifier + (bonus * 2),
//                     format!("{ab_string} saving throw"),
//                 )
//             }
//         },
//     };
//
//     info! {"Rolling {resp}!"}
//     info! {"Rolled {rolled}, plus modifiers equals {total}"};
// }

// fn handle_attack(
//     trigger: Trigger<Attack>,
//     mut commands: Commands,
//     from_query: Query<(
//         &StrengthModifier,
//         &DexterityModifier,
//         &SimpleWeaponProficiency,
//         &MartialWeaponProficiency,
//         &IndividualWeaponProficiency,
//         &ProficiencyBonus,
//         &CritType,
//     )>,
//     with_query: Query<(
//         &WeaponType,
//         Option<&AttackModifier>,
//         Option<&Advantage>,
//         Option<&Disadvantage>,
//     )>,
//     to_query: Query<(&ArmorClass, Option<&Cover>)>,
//     damage_query: Query<(&BaseDamage, &Dice, &DamageType, Option<&DamageModifier>)>,
// ) {
//     info!("Inside handle attack fn");
//     let event = trigger.event();
//     let (strmod, dexmod, simp, mart, ind, profbonus, crit_type) = from_query
//         .get(event.from)
//         .expect("The Attack.from entity to exist");
//
//     let (wep_type, att_mod, adv, disadv) = with_query
//         .get(event.with)
//         .expect("the Attack.with entity to exist");
//
//     let (ac, cover) = to_query
//         .get(event.to)
//         .expect("the Attack.to entity to exist");
//
//     let mut dice_addition = match *wep_type {
//         WeaponType::SimpleMelee => match simp.0 {
//             Proficiency::None => strmod.0,
//             Proficiency::Proficient => strmod.0 + profbonus.0,
//             Proficiency::Expert => strmod.0 + (profbonus.0 * 2),
//         },
//         WeaponType::SimpleRanged => match simp.0 {
//             Proficiency::None => dexmod.0,
//             Proficiency::Proficient => dexmod.0 + profbonus.0,
//             Proficiency::Expert => dexmod.0 + (profbonus.0 * 2),
//         },
//         WeaponType::MartialMelee => match mart.0 {
//             Proficiency::None => strmod.0,
//             Proficiency::Proficient => strmod.0 + profbonus.0,
//             Proficiency::Expert => strmod.0 + (profbonus.0 * 2),
//         },
//         WeaponType::MartialRanged => match mart.0 {
//             Proficiency::None => dexmod.0,
//             Proficiency::Proficient => dexmod.0 + profbonus.0,
//             Proficiency::Expert => dexmod.0 + (profbonus.0 * 2),
//         },
//     };
//     if let Some(modd) = att_mod {
//         dice_addition += modd.0;
//     }
//     let mut rng = rand::thread_rng();
//     let first_roll: i64 = rng.gen_range(1..21);
//     let second_roll: i64 = rng.gen_range(1..21);
//     let mut final_roll = match (adv, disadv) {
//         (Some(_), Some(_)) => first_roll,
//         (None, None) => first_roll,
//         (Some(_), _) => {
//             if first_roll >= second_roll {
//                 first_roll
//             } else {
//                 second_roll
//             }
//         }
//         (_, Some(_)) => {
//             if first_roll <= second_roll {
//                 first_roll
//             } else {
//                 second_roll
//             }
//         }
//     };
//     info!("Roll: {final_roll}");
//     let critical_success = final_roll == 20;
//     // Not sure what to do with crit_failure right now
//     // let critical_failure = final_roll == 1;
//     final_roll += dice_addition;
//     if critical_success {
//         info!("CRIT!");
//     }
//     info!("Final attack number: {final_roll}");
//     let total_ac = match cover {
//         None => ac.0,
//         Some(c) => match *c {
//             Cover::Half => ac.0 + 2,
//             Cover::ThreeQuarters => ac.0 + 5,
//             Cover::Total => ac.0 + 999,
//         },
//     };
//     info!("AC: {total_ac}");
//
//     if final_roll > total_ac {
//         info!("HIT!");
//         let (base, dice, dmg_type, dmg_mod) = damage_query
//             .get(event.with)
//             .expect("There to be a damage bundle from Event.with");
//         let mut dice_total = 0;
//         let dice_max = match dice.dice_type {
//             DiceType::D2 => 3,
//             DiceType::D4 => 5,
//             DiceType::D6 => 7,
//             DiceType::D8 => 9,
//             DiceType::D10 => 11,
//             DiceType::D12 => 13,
//             DiceType::D20 => 21,
//             DiceType::D100 => 101,
//         };
//         for _ in 0..dice.number {
//             let roll = rng.gen_range(1..dice_max);
//             info!(roll);
//             dice_total += roll
//         }
//         info!("Rolled Damage: {dice_total}");
//         let mut dmg_total = match dmg_mod {
//             None => dice_total + base.0,
//             Some(x) => dice_total + base.0 + x.0,
//         };
//
//         // Currently defaults to double damage crit type, even though we query for it
//         if critical_success {
//             dmg_total *= 2;
//         }
//
//         info!("Total damage: {dmg_total}");
//
//         commands.trigger(TakeDamage {
//             unit: event.to,
//             amount: dmg_total,
//         })
//     }
// }

fn handle_taking_damage(
    trigger: Trigger<TakeDamage>,
    mut commands: Commands,
    mut health_query: Query<(&mut Health, Option<&Player>, &MaxHealth)>,
) {
    info!("Inside taking damage function");
    let event = trigger.event();
    let (mut health, player, max_health) = health_query
        .get_mut(event.unit)
        .expect("The event.unit to exist and point to an existing entity");
    info!("Previous health: {}", health.0);
    health.0 -= event.amount;
    if health.0 > 0. {
        info!("Current health: {}", health.0);
        return;
    } else {
        info!("Uh oh, somebody's in trouble!");
        match player {
            None => commands.entity(event.unit).despawn(),
            Some(_) => {
                let dead = health.0.abs() >= (max_health.0.total * 2.);
                if dead {}
            }
        }
    }
}
