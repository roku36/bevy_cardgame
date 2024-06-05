use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, FontId, RichText},
    EguiContexts, EguiPlugin,
};

use crate::{
    game::card_ui::CardUiPlugin,
    GameState,
    HP,
};

mod game_control;
mod card_ui;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_plugins((
                CardUiPlugin,
                EguiPlugin,
            ))
            .add_systems(
                Update,
                (
                    // set_movement_actions.run_if(in_state(GameState::Playing)),
                    update_score_ui.run_if(in_state(GameState::Playing)),
                )
            );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

fn update_score_ui(mut contexts: EguiContexts, hp: Res<HP>) {
    let HP(p1_hp, p2_hp) = *hp;

    egui::Area::new("hp")
        .anchor(Align2::CENTER_TOP, (0., 25.))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(
                RichText::new(format!("{p1_hp} - {p2_hp}"))
                    .color(Color32::BLACK)
                    .font(FontId::proportional(72.0)),
            );
        });
}
