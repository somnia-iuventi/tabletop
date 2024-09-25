use crate::components::*;
use bevy::{
    ecs::{system::EntityCommands, world::Command},
    prelude::*,
    reflect::DynamicTypePath,
};
use std::{marker::PhantomData, mem::discriminant};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EquipItem>();
        app.add_event::<UnequipItem>();
        app.add_event::<SpawnItem>();
        app.add_systems(Update, equip_item.run_if(on_event::<EquipItem>()));
        app.add_systems(Update, unequip_item.run_if(on_event::<UnequipItem>()));
        app.observe(spawn_item);
    }
}

pub struct UpdateStat(Entity, StatEnum);

impl Command for UpdateStat {
    fn apply(self, world: &mut World) {
        info!("Applying update stat command");
        let unit_id = self.0;
        let Some(ch) = world.entity(unit_id).get::<Children>() else {
            return;
        };
        let mods = ch
            .into_iter()
            .filter_map(|x| world.entity(*x).get::<StatModList>())
            .map(|x| &x.0)
            .flatten()
            .filter(|x| x.stat == self.1)
            .collect::<Vec<&StatMod>>();

        // add timestamp to replace arm of modtype enum
        let replace = mods
            .iter()
            .filter(|x| x.mod_type == ModType::Replace)
            .map(|x| x.value)
            .last();
        let add = mods
            .iter()
            .filter(|x| x.mod_type == ModType::Add)
            .map(|x| x.value)
            .sum::<f64>();

        let mult = mods
            .iter()
            .filter(|x| x.mod_type == ModType::Mult)
            .map(|x| x.value)
            .sum::<f64>();

        let mut best_list = mods
            .iter()
            .filter(|x| x.mod_type == ModType::BestOf)
            .map(|x| x.value)
            .collect::<Vec<f64>>();

        best_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let best = best_list.pop();
        let (stat, parent_modifier) = match self.1 {
            StatEnum::Strength => (&mut world.get_mut::<Strength>(unit_id).unwrap().0.stat, 0.),
            StatEnum::Constitution => (
                &mut world.get_mut::<Constitution>(unit_id).unwrap().0.stat,
                0.,
            ),
            StatEnum::Dexterity => (&mut world.get_mut::<Dexterity>(unit_id).unwrap().0.stat, 0.),
            StatEnum::Intelligence => (
                &mut world.get_mut::<Intelligence>(unit_id).unwrap().0.stat,
                0.,
            ),
            StatEnum::Wisdom => (&mut world.get_mut::<Wisdom>(unit_id).unwrap().0.stat, 0.),
            StatEnum::Charisma => (&mut world.get_mut::<Charisma>(unit_id).unwrap().0.stat, 0.),
            StatEnum::MaxHealth => {
                info!("Found MaxHealth Statenum");
                (&mut world.get_mut::<MaxHealth>(unit_id).unwrap().0, 0.)
            }
            StatEnum::DarkVision => (&mut world.get_mut::<DarkVision>(unit_id).unwrap().0, 0.),
            StatEnum::Acrobatics => {
                let parent = world
                    .get::<Dexterity>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Acrobatics>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::ArmorClass => {
                let parent = world
                    .get::<Dexterity>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Acrobatics>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Athletics => {
                let parent = world
                    .get::<Strength>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Athletics>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Arcana => {
                let parent = world
                    .get::<Intelligence>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Arcana>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::SleightOfHand => {
                let parent = world
                    .get::<Dexterity>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<SleightOfHand>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Stealth => {
                let parent = world
                    .get::<Dexterity>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Stealth>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::History => {
                let parent = world
                    .get::<Intelligence>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<History>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Investigation => {
                let parent = world
                    .get::<Intelligence>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Investigation>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Nature => {
                let parent = world
                    .get::<Intelligence>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Nature>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Religion => {
                let parent = world
                    .get::<Intelligence>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Religion>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::AnimalHandling => {
                let parent = world.get::<Wisdom>(unit_id).unwrap().0.calculate_modifier();
                (
                    &mut world.get_mut::<AnimalHandling>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Insight => {
                let parent = world.get::<Wisdom>(unit_id).unwrap().0.calculate_modifier();
                (
                    &mut world.get_mut::<Insight>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Medicine => {
                let parent = world.get::<Wisdom>(unit_id).unwrap().0.calculate_modifier();
                (
                    &mut world.get_mut::<Medicine>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Perception => {
                let parent = world.get::<Wisdom>(unit_id).unwrap().0.calculate_modifier();
                (
                    &mut world.get_mut::<Perception>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Survival => {
                let parent = world.get::<Wisdom>(unit_id).unwrap().0.calculate_modifier();
                (
                    &mut world.get_mut::<Survival>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Deception => {
                let parent = world
                    .get::<Charisma>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Deception>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Intimidation => {
                let parent = world
                    .get::<Charisma>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Intimidation>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Performance => {
                let parent = world
                    .get::<Charisma>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Performance>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
            StatEnum::Persuasion => {
                let parent = world
                    .get::<Charisma>(unit_id)
                    .unwrap()
                    .0
                    .calculate_modifier();
                (
                    &mut world.get_mut::<Persuasion>(unit_id).unwrap().0.stat,
                    parent,
                )
            }
        };
        let total_changed = stat.calculate_total(replace, best, add, mult, parent_modifier);
        if total_changed {
            for dep in stat.deps.clone() {
                let update_dep = UpdateStat(unit_id, dep);
                update_dep.apply(world);
            }
        }
    }
}

trait UpdateStatExt {
    fn update_stat(&mut self, stat: StatEnum);
}

impl UpdateStatExt for EntityCommands<'_> {
    fn update_stat(&mut self, stat: StatEnum) {
        let ent = self.id();
        self.commands().add(UpdateStat(ent, stat));
    }
}

fn equip_item(mut evr: EventReader<EquipItem>, modq: Query<&StatModList>, mut commands: Commands) {
    for ev in evr.read() {
        info!("Inside equip_item");
        commands.entity(ev.unit).add_child(ev.item);
        let Ok(s) = modq.get(ev.item) else { return };
        for each in s.0.clone() {
            commands.entity(ev.unit).update_stat(each.stat);
        }
    }
}

fn unequip_item(
    mod_q: Query<&StatModList>,
    mut ev_r: EventReader<UnequipItem>,
    mut commands: Commands,
) {
    info!("Inside equip item");
    for each in ev_r.read() {
        commands.entity(each.item).remove_parent();
        for mods in mod_q.get(each.item).unwrap().0.iter() {
            let stat = mods.stat.clone();
            commands.entity(each.unit).update_stat(stat);
        }
    }
}

fn spawn_item(
    trigger: Trigger<SpawnItem>,
    mut commands: Commands,
    mut ev_w: EventWriter<EquipItem>,
    player_q: Query<Entity, With<Player>>,
) {
    let unit = player_q.single();
    info!("Inside spawn_item");
    let item = trigger.event().0.spawn_id(&mut commands);
    // other stuff later, for now just equip it to the player
    ev_w.send(EquipItem { unit, item });
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum ItemsEnum {
    #[default]
    Club,
    RingOfHealth,
}

impl ItemsEnum {
    fn spawn_id(&self, commands: &mut Commands) -> Entity {
        match self {
            ItemsEnum::Club => commands.spawn(Club::default()).id(),
            ItemsEnum::RingOfHealth => commands.spawn(RingOfHealth::default()).id(),
        }
    }
}

#[derive(Event)]
struct EquipItem {
    unit: Entity,
    item: Entity,
}

#[derive(Event)]
struct UnequipItem {
    unit: Entity,
    item: Entity,
}

#[derive(Event)]
pub struct SpawnItem(pub ItemsEnum);

#[derive(Bundle, Reflect)]
pub struct Club {
    wep: WeaponBundle,
    light: Light,
}

impl Default for Club {
    fn default() -> Self {
        Self {
            wep: WeaponBundle {
                weapon_marker: Weapon,
                item_bundle: ItemBundle {
                    item_marker: Item,
                    name: ItemName("Club".into()),
                    weight: Weight(2),
                    cost: Cost(10),
                },
                damage: DamageBundle {
                    damage_type: DamageType::Bludgeoning,
                    base_damage: BaseDamage(0),
                    dice: Dice {
                        dice_type: DiceType::D4,
                        number: 1,
                    },
                },
                weapon_type: WeaponType::SimpleMelee,
            },
            light: Light,
        }
    }
}

#[derive(Bundle, Reflect)]
pub struct RingOfHealth {
    item: ItemBundle,
    mods: StatModList,
}

impl Default for RingOfHealth {
    fn default() -> Self {
        Self {
            item: ItemBundle {
                item_marker: Item,
                name: ItemName("Example".into()),
                weight: Weight(0),
                cost: Cost(1000),
            },
            mods: StatModList(vec![StatMod {
                stat: StatEnum::MaxHealth,
                mod_type: ModType::Add,
                value: 10.,
            }]),
        }
    }
}
