use crate::{
    loading::TextureAssets,
    GameState,
    HP,
};
use bevy::prelude::*;

pub struct CardUiPlugin;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
pub struct Deck;

#[derive(Event)]
pub struct DrawCardEvent;


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for CardUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DrawCardEvent>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, (
                card_system.run_if(in_state(GameState::Playing)),
                draw_cards.run_if(in_state(GameState::Playing)),
            ));
    }
}

fn setup(
    mut commands: Commands,
    mut ev_draw_card: EventWriter<DrawCardEvent>,
) {
    // spawning deck
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(Deck);

    for _ in 0..9 {
        ev_draw_card.send(DrawCardEvent);
    }
}

fn draw_cards(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    deck_query: Query<Entity, With<Deck>>,
    mut ev_draw_card: EventReader<DrawCardEvent>,

) {
    for _ in ev_draw_card.read() {
        let deck_entity = deck_query.single();
        commands.entity(deck_entity).with_children(|parent| {
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
        });
    }
}

fn card_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &mut Style), (Changed<Interaction>, With<Card>)>,
    mut hp: ResMut<HP>,
) {
    for (entity, interaction, mut style) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // use card
                hp.0 -= 1;
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

