#![allow(clippy::type_complexity)]

use std::collections::HashMap;

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

#[derive(PartialEq, Clone, Copy, Component, Eq, Hash)]
pub struct HandleId(bool);

#[derive(Resource, Clone)]
pub struct HP {
    values: HashMap<HandleId, i32>,
}

impl HP {
    fn new() -> Self {
        let mut values = HashMap::new();
        values.insert(HandleId(true), 100); // プレイヤーの初期HP
        values.insert(HandleId(false), 100); // 対戦相手の初期HP
        HP { values }
    }

    fn increase(&mut self, id: HandleId, amount: i32) {
        if let Some(hp) = self.values.get_mut(&id) {
            *hp += amount;
        }
    }

    fn decrease(&mut self, id: HandleId, amount: i32) {
        if let Some(hp) = self.values.get_mut(&id) {
            *hp -= amount;
        }
    }
}

#[derive(Clone, Copy)]
pub enum CardType {
    Heal,
    Attack,
    Accelerate,
    Charge,
}

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
            .insert_resource(HP::new())
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
