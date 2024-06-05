use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::{
    loading::TextureAssets,
    GameState,
    HP,
    HandleId,
    CardType,
    Card,
    Deck,
    PlayCardEvent,
    DrawCardEvent,
};
use bevy::prelude::*;

pub struct CardUiPlugin;


// impl Distribution<CardType> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardType {
//         match rng.gen_range(0..=3) {
//             0 => CardType::Heal,
//             1 => CardType::Attack,
//             2 => CardType::Accelerate,
//             _ => CardType::Charge,
//         }
//     }
// }

impl Plugin for CardUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DrawCardEvent>()
            .add_event::<PlayCardEvent>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, (
                card_system.run_if(in_state(GameState::Playing)),
                draw_cards.run_if(in_state(GameState::Playing)),
                play_card.run_if(in_state(GameState::Playing)),
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
        .insert(Deck(HandleId(false)));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(Deck(HandleId(true)));


    for _ in 0..5 {
        ev_draw_card.send(DrawCardEvent(HandleId(false)));
        ev_draw_card.send(DrawCardEvent(HandleId(true)));
    }
}

fn draw_cards(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    deck_query: Query<(Entity, &Deck), With<Deck>>,
    mut ev_draw_card: EventReader<DrawCardEvent>,
) {
    for event in ev_draw_card.read() {
        if let Some((deck_entity, _)) = deck_query.iter().find(|(_, deck)| deck.0 == event.0) {
            // generate random card type
            let card_type = CardType::Heal;
            commands.entity(deck_entity).with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        image: textures.heal.clone().into(),
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(300.0),
                            ..Default::default()
                        },
                        border_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .insert(Card(card_type));
            });
        }
    }
}

fn card_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &mut Style, &Card), (Changed<Interaction>, With<Card>)>,
    mut ev_play_card: EventWriter<PlayCardEvent>,
) {
    for (entity, interaction, mut style, card) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // use card
                commands.entity(entity).despawn_recursive();
                ev_play_card.send(PlayCardEvent(card.0));
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

fn play_card(
    mut ev_play_card: EventReader<PlayCardEvent>,
    mut hp: ResMut<HP>,
) {
    for ev in ev_play_card.read() {
        match ev.0 {
            CardType::Heal => {
                hp.0 += 1;
            }
            CardType::Attack => {
                hp.1 -= 1;
            }
            CardType::Accelerate => {}
            CardType::Charge => {}
        }
    }
}
