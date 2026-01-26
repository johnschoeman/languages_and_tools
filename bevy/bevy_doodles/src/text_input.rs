use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;

#[derive(Component)]
pub struct TextInput {
    pub value: String,
    pub is_focused: bool,
    pub cursor_visible: bool,
    pub cursor_timer: f32,
}

#[derive(Component)]
pub enum InputField {
    MainRotationX,
    MainRotationY,
    MainRotationZ,
    LeafRotationX,
    LeafRotationY,
    LeafRotationZ,
    LeafTranslationX,
    LeafTranslationY,
    LeafTranslationZ,
}

#[derive(Resource, Default)]
pub struct InputFocusState {
    pub focused_entity: Option<Entity>,
}

fn is_valid_char(c: &str, current_value: &str) -> bool {
    // Allow digits
    if c.chars().all(|ch| ch.is_ascii_digit()) {
        return true;
    }

    // Allow minus only at start
    if c == "-" && current_value.is_empty() {
        return true;
    }

    // Allow decimal point only once
    if c == "." && !current_value.contains('.') {
        return true;
    }

    false
}

pub fn handle_text_input_focus(
    interaction_query: Query<(&Interaction, Entity), Changed<Interaction>>,
    mut focus_state: ResMut<InputFocusState>,
    mut all_inputs: Query<&mut TextInput>,
) {
    for (interaction, entity) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Check if this entity has a TextInput component
            if all_inputs.get(entity).is_ok() {
                // Clear previous focus
                if let Some(prev_entity) = focus_state.focused_entity {
                    if prev_entity != entity {
                        if let Ok(mut prev_input) = all_inputs.get_mut(prev_entity) {
                            prev_input.is_focused = false;
                        }
                    }
                }

                // Set new focus
                if let Ok(mut input) = all_inputs.get_mut(entity) {
                    focus_state.focused_entity = Some(entity);
                    input.is_focused = true;
                    input.cursor_visible = true;
                    input.cursor_timer = 0.0;
                }
            }
        }
    }
}

pub fn handle_keyboard_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    focus_state: Res<InputFocusState>,
    mut input_query: Query<&mut TextInput>,
) {
    let Some(focused_entity) = focus_state.focused_entity else {
        return;
    };

    let Ok(mut input) = input_query.get_mut(focused_entity) else {
        return;
    };

    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Character(c) => {
                let c_str = c.as_str();
                if is_valid_char(c_str, &input.value) {
                    input.value.push_str(c_str);
                }
            }
            Key::Backspace => {
                input.value.pop();
            }
            Key::Escape => {
                // Clear focus
                input.is_focused = false;
            }
            _ => {}
        }
    }
}

pub fn update_cursor_blink(
    time: Res<Time>,
    mut input_query: Query<&mut TextInput>,
) {
    for mut input in &mut input_query {
        if input.is_focused {
            input.cursor_timer += time.delta_secs();
            if input.cursor_timer >= 0.5 {
                input.cursor_visible = !input.cursor_visible;
                input.cursor_timer = 0.0;
            }
        }
    }
}

pub fn update_text_input_display(
    input_query: Query<(&TextInput, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (input, children) in &input_query {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                let cursor = if input.is_focused && input.cursor_visible {
                    "|"
                } else {
                    ""
                };
                **text = format!("{}{}", input.value, cursor);
            }
        }
    }
}
