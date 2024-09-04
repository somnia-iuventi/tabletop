use bevy::ecs::system::SystemState;
use bevy::ecs::world::Command;
use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use items::ItemsPlugin;
use std::fs::File;
use std::io::Write;

mod components;
mod items;
mod states;
mod ui;

use components::*;
use states::StatePlugins;
use ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StatePlugins)
        .add_plugins(ItemsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<ComponentRegistry>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .insert_state(AppState::Startup)
        .add_systems(OnEnter(AppState::SaveCharacter), save_game)
        .run();
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    commands.spawn((Camera2dBundle { ..default() }, CameraMarker));
    commands.spawn((
        RootUI,
        NodeBundle {
            style: Style {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                // Make node fill the entirety of its parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                //   - The first column will size to the size of its contents
                //   - The second column will take up the remaining available space
                // grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                //  - The first row will size to the size of its contents
                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                //  - The third row will be exactly 20px high
                // grid_template_rows: vec![
                //     GridTrack::auto(),
                //     GridTrack::flex(1.0),
                //     GridTrack::px(20.),
                // ],
                grid_template_columns: vec![
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                ],
                grid_template_rows: vec![
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                    GridTrack::percent(20.),
                ],
                ..default()
            },
            ..default()
        },
    ));
    next_state.set(AppState::MainMenu);
}

fn save_game(world: &mut World, params: &mut SystemState<ResMut<NextState<AppState>>>) {
    let mut units: QueryState<
        (Entity, Option<&Children>),
        Or<(With<Unit>, With<Item>, With<Spell>)>,
    > = QueryState::new(world);
    let parents = units.iter(world).map(|x| x.0);
    let mut builder = DynamicSceneBuilder::from_world(world).extract_entities(parents);
    let children = units.iter(world).filter_map(|x| x.1);
    builder = builder.extract_entities(children.flatten().map(|x| *x));
    let scene = builder.build();
    let registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize(&registry.read()).unwrap();
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/saves/test-save.scn.ron"))
                .and_then(|mut file| file.write_all(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
    {
        let mut state = params.get_mut(world);
        state.set(AppState::InGame);
    }
    params.apply(world);
}

fn despawn_ui(mut commands: Commands, ui_root: Query<Entity, With<RootUI>>) {
    let root = ui_root.single();
    commands.entity(root).despawn_descendants();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::None => color.0 = Color::Srgba(Srgba::hex("282828").unwrap()),
            Interaction::Hovered => color.0 = Color::Srgba(Srgba::hex("505050").unwrap()),
            Interaction::Pressed => {
                color.0 = Color::Srgba(Srgba::hex("787878").unwrap());
                match button_type {
                    ButtonType::NewCharacter => next_state.set(AppState::NewCharacter),
                    ButtonType::LoadCharacter => next_state.set(AppState::LoadCharacter),
                }
            }
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Startup,
    MainMenu,
    LoadCharacter,
    NewCharacter,
    SaveCharacter,
    InGame,
}

#[derive(Component)]
struct Immortal;

struct RecursiveDespawn(Entity);

impl Command for RecursiveDespawn {
    fn apply(self, world: &mut World) {
        let ent = self.0;
        let name = world.get::<UnitName>(ent).unwrap();
        info!("{}", name.0);
        let mut immortals = Vec::new();
        if let Some(ch) = world.get::<Children>(ent) {
            for child in ch {
                let cname = world.get::<UnitName>(*child).unwrap();
                info!("{}", cname.0);
                if let Some(_) = world.get::<Immortal>(*child) {
                    immortals.push(*child);
                }
            }
        }
        for immortal in immortals {
            if let Some(mut mutable) = world.get_entity_mut(immortal) {
                mutable.remove_parent();
            }
        }
        world.entity_mut(ent).despawn_recursive();
    }
}
