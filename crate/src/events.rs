use bevy_ecs::{entity::Entity, event::EntityEvent};

use crate::{IosGCAuthResult, IosGCPlayer, IosGCSaveGamesResponse, IosGCSavedGameResponse};

#[derive(EntityEvent, Clone, Debug)]
pub struct GCAuthResult {
    pub entity: Entity,
    pub response: IosGCAuthResult,
}

#[derive(EntityEvent, Clone, Debug)]
pub struct PlayerResult {
    pub entity: Entity,
    pub response: IosGCPlayer,
}

#[derive(EntityEvent, Clone, Debug)]
pub struct SaveGamesResult {
    pub entity: Entity,
    pub response: IosGCSaveGamesResponse,
}

#[derive(EntityEvent, Clone, Debug)]
pub struct SavedGameResult {
    pub entity: Entity,
    pub response: IosGCSavedGameResponse,
}
