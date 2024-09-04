use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

// pub mod load_character;
pub mod in_game;
pub mod load_character;
pub mod main_menu;
pub mod new_character;
use in_game::InGamePlugin;
use load_character::LoadCharacterPlugin;
use main_menu::MainMenuPlugin;
use new_character::NewCharacterPlugin;

pub struct StatePlugins;

impl PluginGroup for StatePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MainMenuPlugin)
            .add(NewCharacterPlugin)
            .add(InGamePlugin)
            .add(LoadCharacterPlugin)
    }
}
