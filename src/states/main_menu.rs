use crate::{despawn_ui, AppState, ButtonType, RootUI};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup);
        app.add_systems(OnExit(AppState::MainMenu), despawn_ui);
    }
}

fn setup(
    mut commands: Commands,
    ui_query: Query<Entity, With<RootUI>>,
    mut menu_state: ResMut<NextState<AppState>>,
) {
    let root = ui_query.single();
    let main_menu = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Grid,
                grid_column: GridPlacement::start(3),
                grid_row: GridPlacement::start(2),
                ..default()
            },
            ..default()
        })
        .id();
    commands.entity(root).add_child(main_menu);
    for (text, btype) in [
        ("New Character", ButtonType::NewCharacter),
        ("Load Character", ButtonType::LoadCharacter),
    ] {
        let button = commands
            .spawn((
                btype,
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.)),
                        border: UiRect::all(Val::Px(1.)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::all(Val::Px(10.)),
                    ..default()
                },
            ))
            .id();
        commands.entity(main_menu).add_child(button);

        let text = commands
            .spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 20.0,
                    color: Color::Srgba(Srgba::WHITE),
                    ..default()
                },
            ))
            .id();
        commands.entity(button).add_child(text);
    }
}
