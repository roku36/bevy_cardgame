use bevy::prelude::*;
use crate::{
    CardType,
    HandleId,
};

#[derive(Component, Clone, Copy)]
pub struct Card {
    pub cardtype: CardType,
    pub handleid: HandleId,
}

#[derive(Component)]
pub struct Deck(pub HandleId);

