use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct CardUiPlugin;

#[derive(Component)]
pub struct Card;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for CardUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_card)
            .add_systems(Update, card_system.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_card(mut commands: Commands, textures: Res<TextureAssets>) {
    // commands
    //     .spawn(SpriteBundle {
    //         texture: textures.card1.clone(),
    //         // transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
    //         transform: Transform {
    //             scale: Vec3::splat(0.3),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(Card);


    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            // background_color: Color::DARK_GREEN.into(),
            ..default()
        })
        .with_children(|parent| {
            for _ in 0..5 {
                parent
                    .spawn(ButtonBundle {
                        image: textures.card1.clone().into(),
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(300.0),
                            ..Default::default()
                        },
                        border_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .insert(Card);
            }
        });
}

fn card_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &mut Style), (Changed<Interaction>, With<Card>)>,
) {
    for (entity, interaction, mut style) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // use card
                commands.entity(entity).despawn_recursive();

            }
            Interaction::Hovered => {
                style.border = UiRect::all(Val::Px(2.));
            }
            Interaction::None => {
                style.border = UiRect::all(Val::Px(0.));
            }
        }
    }
}
