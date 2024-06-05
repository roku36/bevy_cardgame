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
        .insert(Deck(HandleId(0)));

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
        .insert(Deck(HandleId(1)));


    for _ in 0..5 {
        ev_draw_card.send(DrawCardEvent(HandleId(0)));
        ev_draw_card.send(DrawCardEvent(HandleId(1)));
    }
}

fn draw_cards(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    deck_query: Query<(Entity, &Deck), With<Deck>>,
    mut ev_draw_card: EventReader<DrawCardEvent>,
) {
    for event in ev_draw_card.read() {
        if let Some((deck_entity, deck)) = deck_query.iter().find(|(_, deck)| deck.0 == event.0) {
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
                    .insert(Card{ cardtype: card_type, handleid: deck.0 });
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
                ev_play_card.send(PlayCardEvent(card.clone()));
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
        let my_id = ev.0.handleid.0;
        let opponent_id = !my_id;

        match ev.0.cardtype {
            CardType::Heal => {
                hp.increase(HandleId(opponent_id), 3);
            }
            CardType::Attack => {
                hp.decrease(HandleId(opponent_id), 2);
            }
            CardType::Accelerate => {}
            CardType::Charge => {}
        }
    }
}
