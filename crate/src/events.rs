use std::ops::Deref;

use bevy_ecs::{entity::Entity, event::EntityEvent};

use crate::{IosGCAuthResult, IosGCPlayer, IosGCSaveGamesResponse, IosGCSavedGameResponse};

#[derive(EntityEvent, Clone, Debug)]
pub struct GCAuthResult {
    pub entity: Entity,
    pub response: IosGCAuthResult,
}
impl Deref for GCAuthResult {
    type Target = IosGCAuthResult;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct PlayerResult {
    pub entity: Entity,
    pub response: IosGCPlayer,
}
impl Deref for PlayerResult {
    type Target = IosGCPlayer;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct SaveGamesResult {
    pub entity: Entity,
    pub response: IosGCSaveGamesResponse,
}
impl Deref for SaveGamesResult {
    type Target = IosGCSaveGamesResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct SavedGameResult {
    pub entity: Entity,
    pub response: IosGCSavedGameResponse,
}
impl Deref for SavedGameResult {
    type Target = IosGCSavedGameResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}
