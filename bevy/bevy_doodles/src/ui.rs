use bevy::prelude::*;
use crate::scene::{AutoRotation, RotatingCube};
use crate::text_input::{TextInput, InputField};

// UI constants
const UI_PADDING: f32 = 20.0;
const BUTTON_SPACING: f32 = 10.0;
const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_FONT_SIZE: f32 = 18.0;
const BUTTON_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const BUTTON_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);

// Button rotation amount
const BUTTON_ROTATION_AMOUNT: f32 = 0.1;

// Input panel constants
const PANEL_BG_COLOR: (f32, f32, f32) = (0.1, 0.1, 0.1);
const PANEL_PADDING: f32 = 15.0;
const PANEL_ROW_GAP: f32 = 8.0;
const MAIN_PANEL_BOTTOM_OFFSET: f32 = 280.0;

// Input field constants
const INPUT_WIDTH: f32 = 80.0;
const INPUT_HEIGHT: f32 = 30.0;
const INPUT_PADDING: f32 = 5.0;
const INPUT_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const INPUT_COLUMN_GAP: f32 = 8.0;

// Text constants
const PANEL_TITLE_FONT_SIZE: f32 = 16.0;
const SECTION_HEADER_FONT_SIZE: f32 = 14.0;
const INPUT_FONT_SIZE: f32 = 14.0;
const TITLE_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);
const SECTION_TEXT_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7);

// Initial configuration values
struct CubeConfig {
    main_rotation_x: f32,
    main_rotation_y: f32,
    main_rotation_z: f32,
    leaf_rotation_x: f32,
    leaf_rotation_y: f32,
    leaf_rotation_z: f32,
    leaf_position_x: f32,
    leaf_position_y: f32,
    leaf_position_z: f32,
}

const CUBE_CONFIG: CubeConfig = CubeConfig {
    main_rotation_x: 115.0,
    main_rotation_y: 0.0,
    main_rotation_z: -45.0,
    leaf_rotation_x: 0.0,
    leaf_rotation_y: 45.0,
    leaf_rotation_z: 45.0,
    leaf_position_x: 0.9,
    leaf_position_y: 0.0,
    leaf_position_z: -0.9,
};

#[derive(Component)]
pub enum RotationButton {
    ToggleAuto,
    Reset,
    PlusX,
    MinusX,
    PlusY,
    MinusY,
    PlusZ,
    MinusZ,
}

pub fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(UI_PADDING)),
            ..default()
        })
        .with_children(|parent| {
            // Left side button panel
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(BUTTON_SPACING),
                    ..default()
                })
                .with_children(|panel| {
                    spawn_button(panel, "⏯ Auto (Space)", RotationButton::ToggleAuto);
                    spawn_button(panel, "↺ Reset (R)", RotationButton::Reset);
                });

            // Right side button panel for individual axis rotation
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(UI_PADDING),
                    top: Val::Px(UI_PADDING),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(BUTTON_SPACING),
                    ..default()
                })
                .with_children(|panel| {
                    spawn_button(panel, "+X (J)", RotationButton::PlusX);
                    spawn_button(panel, "-X (U)", RotationButton::MinusX);
                    spawn_button(panel, "+Y (K)", RotationButton::PlusY);
                    spawn_button(panel, "-Y (I)", RotationButton::MinusY);
                    spawn_button(panel, "+Z (L)", RotationButton::PlusZ);
                    spawn_button(panel, "-Z (O)", RotationButton::MinusZ);
                });
        });

    // Configuration panels (left side)
    spawn_main_rotation_panel(&mut commands);
    spawn_leaf_config_panel(&mut commands);
}

fn spawn_button(parent: &mut ChildSpawnerCommands, text: &str, button_type: RotationButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(BUTTON_BG_COLOR.0, BUTTON_BG_COLOR.1, BUTTON_BG_COLOR.2)),
            button_type,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font_size: BUTTON_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(BUTTON_TEXT_COLOR.0, BUTTON_TEXT_COLOR.1, BUTTON_TEXT_COLOR.2)),
            ));
        });
}

pub fn handle_button_interaction(
    interaction_query: Query<(&Interaction, &RotationButton), Changed<Interaction>>,
    mut cube_query: Query<&mut Transform, With<RotatingCube>>,
    mut auto_rotation: ResMut<AutoRotation>,
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                RotationButton::ToggleAuto => {
                    auto_rotation.enabled = !auto_rotation.enabled;
                }
                RotationButton::Reset => {
                    for mut transform in &mut cube_query {
                        transform.rotation = Quat::IDENTITY;
                    }
                }
                _ => {
                    for mut transform in &mut cube_query {
                        match button_type {
                            RotationButton::PlusX => transform.rotate_local_x(BUTTON_ROTATION_AMOUNT),
                            RotationButton::MinusX => transform.rotate_local_x(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::PlusY => transform.rotate_local_y(BUTTON_ROTATION_AMOUNT),
                            RotationButton::MinusY => transform.rotate_local_y(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::PlusZ => transform.rotate_local_z(BUTTON_ROTATION_AMOUNT),
                            RotationButton::MinusZ => transform.rotate_local_z(-BUTTON_ROTATION_AMOUNT),
                            RotationButton::ToggleAuto | RotationButton::Reset => {}
                        }
                    }
                }
            }
        }
    }
}

fn spawn_main_rotation_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(UI_PADDING),
                bottom: Val::Px(MAIN_PANEL_BOTTOM_OFFSET),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(PANEL_ROW_GAP),
                padding: UiRect::all(Val::Px(PANEL_PADDING)),
                ..default()
            },
            BackgroundColor(Color::srgb(PANEL_BG_COLOR.0, PANEL_BG_COLOR.1, PANEL_BG_COLOR.2)),
        ))
        .with_children(|panel: &mut ChildSpawnerCommands| {
            // Title
            panel.spawn((
                Text::new("Main Cube Rotation"),
                TextFont {
                    font_size: PANEL_TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(TITLE_TEXT_COLOR.0, TITLE_TEXT_COLOR.1, TITLE_TEXT_COLOR.2)),
            ));

            // Rotation inputs
            spawn_input_row(panel, "X:", &CUBE_CONFIG.main_rotation_x.to_string(), InputField::MainRotationX);
            spawn_input_row(panel, "Y:", &CUBE_CONFIG.main_rotation_y.to_string(), InputField::MainRotationY);
            spawn_input_row(panel, "Z:", &CUBE_CONFIG.main_rotation_z.to_string(), InputField::MainRotationZ);
        });
}

fn spawn_leaf_config_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(UI_PADDING),
                bottom: Val::Px(UI_PADDING),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(PANEL_ROW_GAP),
                padding: UiRect::all(Val::Px(PANEL_PADDING)),
                ..default()
            },
            BackgroundColor(Color::srgb(PANEL_BG_COLOR.0, PANEL_BG_COLOR.1, PANEL_BG_COLOR.2)),
        ))
        .with_children(|panel: &mut ChildSpawnerCommands| {
            // Title
            panel.spawn((
                Text::new("Leaf Cube Configuration"),
                TextFont {
                    font_size: PANEL_TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(TITLE_TEXT_COLOR.0, TITLE_TEXT_COLOR.1, TITLE_TEXT_COLOR.2)),
            ));

            // Rotation section
            panel.spawn((
                Text::new("Rotation"),
                TextFont {
                    font_size: SECTION_HEADER_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(SECTION_TEXT_COLOR.0, SECTION_TEXT_COLOR.1, SECTION_TEXT_COLOR.2)),
            ));
            spawn_input_row(panel, "X:", &CUBE_CONFIG.leaf_rotation_x.to_string(), InputField::LeafRotationX);
            spawn_input_row(panel, "Y:", &CUBE_CONFIG.leaf_rotation_y.to_string(), InputField::LeafRotationY);
            spawn_input_row(panel, "Z:", &CUBE_CONFIG.leaf_rotation_z.to_string(), InputField::LeafRotationZ);

            // Translation section
            panel.spawn((
                Text::new("Position"),
                TextFont {
                    font_size: SECTION_HEADER_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(SECTION_TEXT_COLOR.0, SECTION_TEXT_COLOR.1, SECTION_TEXT_COLOR.2)),
            ));
            spawn_input_row(panel, "X:", &CUBE_CONFIG.leaf_position_x.to_string(), InputField::LeafTranslationX);
            spawn_input_row(panel, "Y:", &CUBE_CONFIG.leaf_position_y.to_string(), InputField::LeafTranslationY);
            spawn_input_row(panel, "Z:", &CUBE_CONFIG.leaf_position_z.to_string(), InputField::LeafTranslationZ);
        });
}

fn spawn_input_row(parent: &mut ChildSpawnerCommands, label: &str, initial: &str, field_type: InputField) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(INPUT_COLUMN_GAP),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: INPUT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(TITLE_TEXT_COLOR.0, TITLE_TEXT_COLOR.1, TITLE_TEXT_COLOR.2)),
            ));

            // Input field
            row.spawn((
                Button,
                Node {
                    width: Val::Px(INPUT_WIDTH),
                    height: Val::Px(INPUT_HEIGHT),
                    padding: UiRect::all(Val::Px(INPUT_PADDING)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(INPUT_BG_COLOR.0, INPUT_BG_COLOR.1, INPUT_BG_COLOR.2)),
                TextInput {
                    value: initial.to_string(),
                    is_focused: false,
                    cursor_visible: false,
                    cursor_timer: 0.0,
                },
                field_type,
            ))
            .with_children(|input| {
                input.spawn((
                    Text::new(initial),
                    TextFont {
                        font_size: INPUT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(Color::srgb(TITLE_TEXT_COLOR.0, TITLE_TEXT_COLOR.1, TITLE_TEXT_COLOR.2)),
                ));
            });

            // "degrees" suffix
            row.spawn((
                Text::new("degrees"),
                TextFont {
                    font_size: INPUT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(SECTION_TEXT_COLOR.0, SECTION_TEXT_COLOR.1, SECTION_TEXT_COLOR.2)),
            ));
        });
}
