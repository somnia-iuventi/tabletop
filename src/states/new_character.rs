use crate::components::*;
use crate::despawn_ui;
use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use strum::{Display, EnumIter, IntoEnumIterator};

pub struct NewCharacterPlugin;

impl Plugin for NewCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerBundle>();
        app.add_event::<CreateCharacter>();
        app.add_systems(Update, ui.run_if(in_state(AppState::NewCharacter)));
        app.add_systems(OnEnter(AppState::NewCharacter), setup);
        app.add_systems(OnExit(AppState::NewCharacter), despawn_ui);
        app.observe(on_character_creation);
    }
}

#[derive(Event)]
struct CreateCharacter;

fn on_character_creation(
    trigger: Trigger<CreateCharacter>,
    newchar: Res<PlayerBundle>,
    mut menu_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    let mut abilities = newchar.abilities.clone();
    abilities.str.0.stat.total = abilities.str.0.stat.base;
    abilities.con.0.stat.total = abilities.con.0.stat.base;
    abilities.dex.0.stat.total = abilities.dex.0.stat.base;
    abilities.int.0.stat.total = abilities.int.0.stat.base;
    abilities.wis.0.stat.total = abilities.wis.0.stat.base;
    abilities.cha.0.stat.total = abilities.cha.0.stat.base;
    abilities.str.0.calculate_modifier();
    abilities.con.0.calculate_modifier();
    abilities.dex.0.calculate_modifier();
    abilities.int.0.calculate_modifier();
    abilities.wis.0.calculate_modifier();
    abilities.cha.0.calculate_modifier();
    let mut skills = newchar.skills.clone();
    skills.athletics.0.stat.total = skills.athletics.0.stat.base;
    skills.acrobatics.0.stat.total = skills.acrobatics.0.stat.base;
    skills.sleight_of_hand.0.stat.total = skills.sleight_of_hand.0.stat.base;
    skills.stealth.0.stat.total = skills.stealth.0.stat.base;
    skills.arcana.0.stat.total = skills.arcana.0.stat.base;
    skills.history.0.stat.total = skills.history.0.stat.base;
    skills.investigation.0.stat.total = skills.investigation.0.stat.base;
    skills.nature.0.stat.total = skills.nature.0.stat.base;
    skills.religion.0.stat.total = skills.religion.0.stat.base;
    skills.animal_handling.0.stat.total = skills.animal_handling.0.stat.base;
    skills.insight.0.stat.total = skills.insight.0.stat.base;
    skills.medicine.0.stat.total = skills.medicine.0.stat.base;
    skills.perception.0.stat.total = skills.perception.0.stat.base;
    skills.survival.0.stat.total = skills.survival.0.stat.base;
    skills.deception.0.stat.total = skills.deception.0.stat.base;
    skills.intimidation.0.stat.total = skills.intimidation.0.stat.base;
    skills.performance.0.stat.total = skills.performance.0.stat.base;
    skills.persuasion.0.stat.total = skills.persuasion.0.stat.base;

    let darkvision: Option<DarkVision> = match newchar.race {
        Race::HillDwarf => Some(DarkVision(Stat::new(60))),
        Race::MountainDwarf => Some(DarkVision(Stat::new(60))),
        Race::HighElf => Some(DarkVision(Stat::new(60))),
        Race::WoodElf => Some(DarkVision(Stat::new(60))),
        Race::DarkElf => Some(DarkVision(Stat::new(120))),
        Race::HalfElf => Some(DarkVision(Stat::new(60))),
        Race::RockGnome => Some(DarkVision(Stat::new(60))),
        Race::ForestGnome => Some(DarkVision(Stat::new(60))),
        Race::HalfOrc => Some(DarkVision(Stat::new(60))),
        Race::Tiefling => Some(DarkVision(Stat::new(60))),
        _ => None,
    };
    let char_id = commands
        .spawn(PlayerBundle {
            unit_tag: Unit,
            player_tag: Player,
            name: newchar.name.clone(),
            player_name: newchar.player_name.clone(),
            ac: newchar.ac.clone(),
            speed: newchar.speed.clone(),
            abilities,
            skills: newchar.skills.clone(),
            race: newchar.race.clone(),
            class: newchar.class.clone(),
            health: Health(newchar.max_health.0).clone(),
            max_health: newchar.max_health.clone(),
            background: newchar.background.clone(),
            alignment: newchar.alignment.clone(),
            xp: newchar.xp.clone(),
            level: newchar.level.clone(),
            hit_dice: newchar.hit_dice.clone(),
            wep_profs: WeaponProficiencies::default(),
            prof_bonus: ProficiencyBonus::from_level(newchar.level.clone().0),
            settings: SettingsBundle::default(),
        })
        .id();
    if let Some(dv) = darkvision {
        commands.entity(char_id).insert(dv);
    }
    menu_state.set(AppState::SaveCharacter);
}

fn setup(mut newchar: ResMut<PlayerBundle>, mut commands: Commands) {}

fn ui(mut contexts: EguiContexts, mut newchar: ResMut<PlayerBundle>, mut commands: Commands) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("toppanel").show(ctx, |ui| {
        egui::Grid::new("toppanelgrid")
            .min_col_width(100.)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Character Name");
                ui.text_edit_singleline(&mut newchar.name.0);
                ui.label("Class");
                egui::ComboBox::from_id_source("class")
                    .selected_text(format!("{}", newchar.class.to_string()))
                    .show_ui(ui, |ui| {
                        for class in Class::iter() {
                            ui.selectable_value(
                                &mut newchar.class,
                                class.clone(),
                                class.to_string(),
                            );
                        }
                    });
                ui.label("Background");
                egui::ComboBox::from_id_source("background")
                    .selected_text(format!("{}", newchar.background.to_string()))
                    .show_ui(ui, |ui| {
                        for background in Background::iter() {
                            ui.selectable_value(
                                &mut newchar.background,
                                background.clone(),
                                background.to_string(),
                            );
                        }
                    });
                ui.label("Level");
                ui.add(egui::DragValue::new(&mut newchar.level.0));
                ui.end_row();
                ui.label("Player Name");
                ui.text_edit_singleline(&mut newchar.player_name.0);
                ui.label("Race");
                egui::ComboBox::from_id_source("race")
                    .selected_text(format!("{}", newchar.race.to_string()))
                    .show_ui(ui, |ui| {
                        for race in Race::iter() {
                            ui.selectable_value(&mut newchar.race, race.clone(), race.to_string());
                        }
                    });
                ui.label("Alignment");
                egui::ComboBox::from_id_source("alignment")
                    .selected_text(format!("{}", newchar.alignment.to_string()))
                    .show_ui(ui, |ui| {
                        for alignment in Alignment::iter() {
                            ui.selectable_value(
                                &mut newchar.alignment,
                                alignment.clone(),
                                alignment.to_string(),
                            );
                        }
                    });
                ui.label("XP");
                ui.add(egui::DragValue::new(&mut newchar.xp.0));
            });
    });
    egui::SidePanel::left("left-panel").show(ctx, |ui| {
        egui::Grid::new("abilitygrid")
            .num_columns(4)
            .show(ui, |ui| {
                ui.label("ABILITIES");
                ui.end_row();
                ui.label("Strength");
                ui.add(egui::DragValue::new(&mut newchar.abilities.str.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("str")
                    .selected_text(format!("{:?}", newchar.abilities.str.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.abilities.str.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.str.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.str.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Constitution");
                ui.add(egui::DragValue::new(&mut newchar.abilities.con.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("con")
                    .selected_text(format!("{:?}", newchar.abilities.con.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.abilities.con.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.con.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.con.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Dexterity");
                ui.add(egui::DragValue::new(&mut newchar.abilities.dex.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("dex")
                    .selected_text(format!("{:?}", newchar.abilities.dex.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.abilities.dex.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.dex.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.dex.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Intelligence");
                ui.add(egui::DragValue::new(&mut newchar.abilities.int.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("int")
                    .selected_text(format!("{:?}", newchar.abilities.int.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.abilities.int.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.int.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.int.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Wisdom");
                ui.add(egui::DragValue::new(&mut newchar.abilities.wis.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("wis")
                    .selected_text(format!("{:?}", newchar.abilities.wis.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.abilities.wis.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.wis.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.wis.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Charisma");
                ui.add(egui::DragValue::new(&mut newchar.abilities.cha.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("cha")
                    .selected_text(format!("{:?}", newchar.abilities.cha.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.abilities.cha.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.cha.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.abilities.cha.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.end_row();
                ui.label("SKILLS");
                ui.end_row();
                ui.label("Acrobatics");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.acrobatics.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("acro")
                    .selected_text(format!("{:?}", newchar.skills.acrobatics.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.acrobatics.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.acrobatics.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.acrobatics.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Animal Handling");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.animal_handling.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("animal")
                    .selected_text(format!(
                        "{:?}",
                        newchar.skills.animal_handling.0.proficiency
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.animal_handling.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.animal_handling.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.animal_handling.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Arcana");
                ui.add(egui::DragValue::new(&mut newchar.skills.arcana.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("arcana")
                    .selected_text(format!("{:?}", newchar.skills.arcana.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.arcana.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.arcana.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.arcana.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Athletics");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.athletics.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("athletics")
                    .selected_text(format!("{:?}", newchar.skills.athletics.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.athletics.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.athletics.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.athletics.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Deception");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.deception.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("deception")
                    .selected_text(format!("{:?}", newchar.skills.deception.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.deception.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.deception.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.deception.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("History");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.history.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("history")
                    .selected_text(format!("{:?}", newchar.skills.history.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.history.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.history.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.history.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Insight");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.insight.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("insight")
                    .selected_text(format!("{:?}", newchar.skills.insight.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.insight.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.insight.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.insight.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Intimidation");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.intimidation.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("intimidation")
                    .selected_text(format!("{:?}", newchar.skills.intimidation.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.intimidation.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.intimidation.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.intimidation.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Investigation");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.investigation.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("investigation")
                    .selected_text(format!("{:?}", newchar.skills.investigation.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.investigation.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.investigation.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.investigation.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Medicine");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.medicine.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("medicine")
                    .selected_text(format!("{:?}", newchar.skills.medicine.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.medicine.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.medicine.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.medicine.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Nature");
                ui.add(egui::DragValue::new(&mut newchar.skills.nature.0.stat.base));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("nature")
                    .selected_text(format!("{:?}", newchar.skills.nature.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.nature.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.nature.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.nature.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Perception");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.perception.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("perception")
                    .selected_text(format!("{:?}", newchar.skills.perception.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.perception.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.perception.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.perception.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Performance");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.performance.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("performance")
                    .selected_text(format!("{:?}", newchar.skills.performance.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.performance.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.performance.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.performance.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Persuasion");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.persuasion.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("persuasion")
                    .selected_text(format!("{:?}", newchar.skills.persuasion.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.persuasion.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.persuasion.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.persuasion.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Religion");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.religion.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("religion")
                    .selected_text(format!("{:?}", newchar.skills.religion.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.religion.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.religion.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.religion.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Sleight of Hand");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.sleight_of_hand.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("sleight_of_hand")
                    .selected_text(format!(
                        "{:?}",
                        newchar.skills.sleight_of_hand.0.proficiency
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.sleight_of_hand.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.sleight_of_hand.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.sleight_of_hand.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Stealth");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.stealth.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("stealth")
                    .selected_text(format!("{:?}", newchar.skills.stealth.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.stealth.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.stealth.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.stealth.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
                ui.label("Survival");
                ui.add(egui::DragValue::new(
                    &mut newchar.skills.survival.0.stat.base,
                ));
                ui.label("Proficiency");
                egui::ComboBox::from_id_source("survival")
                    .selected_text(format!("{:?}", newchar.skills.survival.0.proficiency))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut newchar.skills.survival.0.proficiency,
                            Proficiency::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.survival.0.proficiency,
                            Proficiency::Proficient,
                            "Proficient",
                        );
                        ui.selectable_value(
                            &mut newchar.skills.survival.0.proficiency,
                            Proficiency::Expert,
                            "Expert",
                        );
                    });
                ui.end_row();
            });
    });
    egui::SidePanel::right("rightpanel").show(ctx, |ui| {
        egui::Grid::new("sidepanelgrid")
            .min_col_width(50.)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Max Health");
                ui.add(egui::DragValue::new(&mut newchar.max_health.0));
                ui.label("Hit Dice");
                ui.add(egui::DragValue::new(&mut newchar.hit_dice.0.number));
                egui::ComboBox::from_id_source("hitdice")
                    .selected_text(format!("{}", newchar.hit_dice.0.dice_type.to_string()))
                    .show_ui(ui, |ui| {
                        for dice in DiceType::iter() {
                            ui.selectable_value(
                                &mut newchar.hit_dice.0.dice_type,
                                dice.clone(),
                                dice.to_string(),
                            );
                        }
                    });
                ui.end_row();
                ui.label("Speed");
                ui.add(egui::DragValue::new(&mut newchar.speed.0));
                ui.label("Armor Class");
                ui.add(egui::DragValue::new(&mut newchar.ac.0));
            });
    });
    egui::TopBottomPanel::bottom("bottompannel").show(ctx, |ui| {
        ui.vertical_centered_justified(|ui| {
            if ui.button("Create").clicked() {
                commands.trigger(CreateCharacter);
            }
        });
    });
}
