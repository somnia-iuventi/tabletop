use std::fs;
use std::path::Path;

use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct LoadCharacterPlugin;

impl Plugin for LoadCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveFileNames>();
        app.add_systems(OnEnter(AppState::LoadCharacter), populate_savefile_names);
        app.add_event::<LoadGame>();
        app.add_event::<GoToInGame>();
        app.add_systems(Update, setup.run_if(in_state(AppState::LoadCharacter)));
        app.add_systems(
            Update,
            transition_to_in_game.run_if(on_event::<GoToInGame>()),
        );
        app.observe(load_character);
    }
}

#[derive(Resource, Default)]
struct SaveFileNames(Vec<String>);

#[derive(Event)]
pub struct LoadGame(pub String);

#[derive(Event)]
struct GoToInGame;

fn populate_savefile_names(mut names: ResMut<SaveFileNames>) {
    let path = Path::new("assets/saves/");
    let mut str_names = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let this_path = entry.path();
        let name = this_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .to_string();
        str_names.push(name);
    }
    names.0 = str_names;
}

fn setup(mut contexts: EguiContexts, characters: Res<SaveFileNames>, mut commands: Commands) {
    let ctx = contexts.ctx_mut();
    // load character save file names
    // iterate over those names and then load that character
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            for name in &characters.0 {
                if ui.button(name).clicked() {
                    commands.trigger(LoadGame(name.clone()));
                }
            }
        })
    });
}

fn transition_to_in_game(
    mut ev_reader: EventReader<GoToInGame>,
    mut appstate: ResMut<NextState<AppState>>,
) {
    for _ in ev_reader.read() {
        appstate.set(AppState::InGame);
    }
}

fn load_character(
    trigger: Trigger<LoadGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_writer: EventWriter<GoToInGame>,
) {
    let file_path = format!("saves/{}.scn.ron", trigger.event().0);
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(file_path),
        ..default()
    });
    ev_writer.send(GoToInGame);
}
