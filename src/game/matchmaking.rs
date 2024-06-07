use bevy::prelude::*;
use bevy_ggrs::{ggrs::DesyncDetection, prelude::*, *};
use bevy_matchbox::prelude::*;
use bevy_roll_safe::prelude::*;

use crate::{
    game::card_ui::CardUiPlugin,
    AppState,
    HandleId,
    Card,
    HP,
};

pub struct MatchMakingPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for MatchMakingPlugin {
    fn build(&self, app: &mut App) {
        app
            .rollback_resource_with_clone::<HP>()
            .rollback_component_with_copy::<Card>()
            .add_systems(OnEnter(AppState::Playing), start_matchbox_socket)
            .add_systems(
                Update,
                (
                    wait_for_players.run_if(in_state(AppState::Playing)),
                )
            );

    }
}


fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/bevy_cardgame?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn wait_for_players(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game!");
    // TODO
}
