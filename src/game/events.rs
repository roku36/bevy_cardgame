use bevy::prelude::Event;

use crate::{
    HandleId,
    game::{
        components::{
            Card,
        }
    }
};

#[derive(Event)]
pub struct DrawCardEvent(pub HandleId);

#[derive(Event)]
pub struct PlayCardEvent(pub Card);

