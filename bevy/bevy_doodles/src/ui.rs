use bevy::prelude::*;
use crate::scene::{AutoRotation, RotatingCube};

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
