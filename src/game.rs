use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, FontId, RichText},
    EguiContexts, EguiPlugin,
};

use crate::{
    game::card_ui::CardUiPlugin,
    game::matchmaking::MatchMakingPlugin,
    AppState,
    HandleId,
    HP,
};

mod game_control;
mod card_ui;
mod matchmaking;
mod components;
mod events;

pub struct GamePlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CardUiPlugin,
                EguiPlugin,
                MatchMakingPlugin,
            ))
            .add_systems(
                Update,
                (
                    update_score_ui.run_if(in_state(AppState::Playing)),
                )
            );
    }
}

fn update_score_ui(mut contexts: EguiContexts, hp: Res<HP>) {
    let p1_hp = hp.values.get(&HandleId(0)).unwrap_or(&0);
    let p2_hp = hp.values.get(&HandleId(1)).unwrap_or(&0);

    egui::Area::new("hp")
        .anchor(Align2::CENTER_CENTER, (0., 25.))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(
                RichText::new(format!("{p1_hp} - {p2_hp}"))
                    .color(Color32::BLACK)
                    .font(FontId::proportional(72.0)),
            );
        });
}
