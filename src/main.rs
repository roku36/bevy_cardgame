use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(GamePlugin)
        .run();
}
