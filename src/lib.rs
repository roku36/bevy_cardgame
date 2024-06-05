#![allow(clippy::type_complexity)]

mod game;
mod audio;
mod loading;
mod menu;

use crate::game::GamePlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Debug)]
pub struct HP(u32, u32);

#[derive(Clone, Copy)]
pub enum CardType {
    Heal,
    Attack,
    Accelerate,
    Charge,
}

#[derive(PartialEq, Clone, Copy, Component)]
pub struct HandleId(bool);

#[derive(Component, Clone, Copy)]
pub struct Card {
    cardtype: CardType,
    handleid: HandleId,
}

#[derive(Component)]
pub struct Deck(HandleId);

#[derive(Event)]
pub struct DrawCardEvent(HandleId);

#[derive(Event)]
pub struct PlayCardEvent(Card);


// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .insert_resource(HP(100, 100))
            .add_plugins((
            LoadingPlugin,
            MenuPlugin,
            GamePlugin,
            InternalAudioPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            // app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
