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
        // app.add_event::<EquipItem>();
        app.add_event::<UnequipItem>();
        app.add_event::<SpawnItem>();
        // app.add_systems(Update, equip_item.run_if(on_event::<EquipItem>()));
        // app.add_systems(Update, unequip_item.run_if(on_event::<UnequipItem>()));
        app.observe(spawn_item);
    }
}

pub struct EquipItem<T: Moddable> {
    unit: Entity,
    item: Entity,
    _marker: PhantomData<T>,
}

// impl<T: Moddable> Command for EquipItem<T> {
// fn apply(self, world: &mut World) {
//     let mut mods = Vec::new();
//     let ent = self.unit;
//     let item = self.item;
//     let maybe = world.get::<Mod>(item);
//     let children = world.get::<Children>(ent);
//     if let Some(ch) = children {
//         for each in ch {
//             let chmods = world.get::<Mod<T>>(*each);
//             if let Some(m) = chmods {
//                 mods.push(m);
//             }
//         }
//     }
//     let replace = mods
//         .iter()
//         .filter(|x| x.mod_type == ModType::Replace)
//         .map(|x| x.value)
//         .last();
//     let add = mods
//         .iter()
//         .filter(|x| x.mod_type == ModType::Add)
//         .map(|x| x.value)
//         .sum::<f64>();
//
//     let mult = mods
//         .iter()
//         .filter(|x| x.mod_type == ModType::Mult)
//         .map(|x| x.value)
//         .sum::<f64>();
//
//     let mut best_list = mods
//         .iter()
//         .filter(|x| x.mod_type == ModType::BestOf)
//         .map(|x| x.value)
//         .collect::<Vec<f64>>();
//
//     best_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     let current = world.get_mut::<T>(ent);
//     let Some(mut curr) = current else { return };
//     let currstat = curr.stat();
//     if let Some(rep) = replace {
//         currstat.total = rep as i64;
//         return;
//     }
//     let mut new_total = currstat.base as f64;
//     new_total += add;
//     new_total *= (1. + mult);
//     if let Some(best) = best_list.pop() {
//         if best > new_total {
//             currstat.total = best as i64;
//             return;
//         }
//     }
//     currstat.total = new_total as i64;
// }
// }

// fn equip_item(
//     ent_q: Query<Entity, With<Player>>,
//     mut ev_r: EventReader<EquipItem>,
//     mut commands: Commands,
// ) {
//     info!("Inside equip item");
//     let ent = ent_q.single();
//     for each in ev_r.read() {
//         let item = each.0;
//         commands.entity(ent).add_child(item);
//         commands.entity(ent).modify_stats(modd.stat);
//     }
// }

// fn unequip_item(
//     ent_q: Query<Entity, With<Player>>,
//     mod_q: Query<&mut PlayerMods>,
//     mut ev_r: EventReader<UnequipItem>,
//     mut commands: Commands,
// ) {
//     info!("Inside equip item");
//     let ent = ent_q.single();
//     for each in ev_r.read() {
//         let item = each.0;
//         for mods in mod_q.get(item).unwrap().0.iter() {
//             let mut rev = mods.clone();
//             rev.value = -(rev.value);
//             commands.entity(ent).modify_stats(rev);
//         }
//         commands.entity(ent).remove_children(&[item]);
//     }
// }

fn spawn_item(
    trigger: Trigger<SpawnItem>,
    mut commands: Commands,
    // mut ev_w: EventWriter<EquipItem>,
) {
    info!("Inside spawn_item");
    let item = trigger.event().0.spawn_id(&mut commands);
    // other stuff later, for now just equip it to the player
    // ev_w.send(EquipItem(item));
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
            ItemsEnum::RingOfHealth => {
                let item = ExampleItem::default();
                let thing = item.reflect_type_ident().unwrap();
                info!("{thing}");
                for each in item.iter_fields() {
                    let path = each.reflect_type_path();
                }
                commands.spawn(item).id()
            }
        }
    }
}

// #[derive(Event)]
// pub struct EquipItem(Entity);

#[derive(Event)]
pub struct UnequipItem(Entity);

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
pub struct ExampleItem {
    item: ItemBundle,
    modd: Mod<Intelligence>,
    modd2: Mod<Strength>,
}

impl Default for ExampleItem {
    fn default() -> Self {
        Self {
            item: ItemBundle {
                item_marker: Item,
                name: ItemName("Example".into()),
                weight: Weight(0),
                cost: Cost(1000),
            },
            modd: Mod::new(ModType::Add, 10.),
            modd2: Mod::new(ModType::Add, 5.),
        }
    }
}
