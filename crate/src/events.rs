use std::ops::Deref;

use bevy_ecs::{entity::Entity, event::EntityEvent};

use crate::{
    IosGCAchievementProgressResponse, IosGCAchievementsResetResponse, IosGCAuthResult,
    IosGCDeleteSaveGameResponse, IosGCFetchItemsForSignatureVerificationResponse,
    IosGCLoadGamesResponse, IosGCPlayer, IosGCResolvedConflictsResponse, IosGCSaveGamesResponse,
    IosGCSavedGameResponse, IosGCScoreSubmitResponse,
};

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

#[derive(EntityEvent, Clone, Debug)]
pub struct LoadGamesResult {
    pub entity: Entity,
    pub response: IosGCLoadGamesResponse,
}
impl Deref for LoadGamesResult {
    type Target = IosGCLoadGamesResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct ResolvedConflictsResult {
    pub entity: Entity,
    pub response: IosGCResolvedConflictsResponse,
}
impl Deref for ResolvedConflictsResult {
    type Target = IosGCResolvedConflictsResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct DeleteSaveGameResult {
    pub entity: Entity,
    pub response: IosGCDeleteSaveGameResponse,
}
impl Deref for DeleteSaveGameResult {
    type Target = IosGCDeleteSaveGameResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct FetchItemsForSignatureVerificationResult {
    pub entity: Entity,
    pub response: IosGCFetchItemsForSignatureVerificationResponse,
}
impl Deref for FetchItemsForSignatureVerificationResult {
    type Target = IosGCFetchItemsForSignatureVerificationResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct AchievementProgressResult {
    pub entity: Entity,
    pub response: IosGCAchievementProgressResponse,
}
impl Deref for AchievementProgressResult {
    type Target = IosGCAchievementProgressResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct AchievementsResetResult {
    pub entity: Entity,
    pub response: IosGCAchievementsResetResponse,
}
impl Deref for AchievementsResetResult {
    type Target = IosGCAchievementsResetResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

#[derive(EntityEvent, Clone, Debug)]
pub struct ScoreSubmitResult {
    pub entity: Entity,
    pub response: IosGCScoreSubmitResponse,
}
impl Deref for ScoreSubmitResult {
    type Target = IosGCScoreSubmitResponse;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}
