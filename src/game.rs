use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::game::card_ui::CardUiPlugin;
use crate::GameState;

mod game_control;
mod card_ui;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_plugins(CardUiPlugin)
            .add_systems(
                Update,
                set_movement_actions.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(
    mut commands: Commands,
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
}
