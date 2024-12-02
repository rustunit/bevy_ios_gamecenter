use std::marker::PhantomData;

use bevy_app::{App, PreUpdate};
use bevy_ecs::{
    prelude::*,
    system::{EntityCommands, IntoObserverSystem, SystemParam},
};

use crate::{
    IosGCAchievementProgressResponse, IosGCAchievementsResetResponse, IosGCAuthResult,
    IosGCDeleteSaveGameResponse, IosGCFetchItemsForSignatureVerificationResponse,
    IosGCLoadGamesResponse, IosGCPlayer, IosGCResolvedConflictsResponse, IosGCSaveGame,
    IosGCSaveGames, IosGCSaveGamesResponse, IosGCSavedGameResponse, IosGCScoreSubmitResponse,
    IosGamecenterEvents,
};

#[derive(Resource, Default)]
struct BevyIosGamecenterState {
    request_id: i64,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestAuthentication;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestPlayer;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestSaveGames;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestSaveGame;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestLoadGame;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestResolveConflicts;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestDeleteSavegame;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestFetchSignature;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestAchievementProgress;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestAchievementsReset;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestLeaderboardScore;

#[derive(Component)]
struct RequestId(i64);

#[derive(Component)]
struct RequestEntity;

/// Observer based API conveniently call and async wait for a gamecenter API response
#[derive(SystemParam)]
pub struct BevyIosGamecenter<'w, 's> {
    commands: Commands<'w, 's>,
    res: ResMut<'w, BevyIosGamecenterState>,
}

impl<'w, 's> BevyIosGamecenter<'w, 's> {
    pub fn authenticate(&mut self) -> BevyIosGCRequestBuilder<'_, IosGCAuthResult> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::authenticate(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestAuthentication,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn request_player(&mut self) -> BevyIosGCRequestBuilder<'_, IosGCPlayer> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::request_player(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestPlayer,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn fetch_save_games(&mut self) -> BevyIosGCRequestBuilder<'_, IosGCSaveGamesResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::fetch_save_games(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestSaveGames,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn save_game(
        &mut self,
        name: String,
        data: &[u8],
    ) -> BevyIosGCRequestBuilder<'_, IosGCSavedGameResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::save_game(id, name, data);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestSaveGame,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn load_game(
        &mut self,
        game: IosGCSaveGame,
    ) -> BevyIosGCRequestBuilder<'_, IosGCLoadGamesResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::load_game(id, game);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestLoadGame,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn resolve_conflicts(
        &mut self,
        save_games: IosGCSaveGames,
        data: &[u8],
    ) -> BevyIosGCRequestBuilder<'_, IosGCResolvedConflictsResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::resolve_conflicting_games(id, save_games, data);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestResolveConflicts,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn delete_savegame(
        &mut self,
        name: String,
    ) -> BevyIosGCRequestBuilder<'_, IosGCDeleteSaveGameResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::delete_savegame(id, name);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestDeleteSavegame,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn fetch_signature(
        &mut self,
    ) -> BevyIosGCRequestBuilder<'_, IosGCFetchItemsForSignatureVerificationResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::fetch_signature(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestFetchSignature,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn achievement_progress(
        &mut self,
        achievement_id: String,
        progress: f64,
    ) -> BevyIosGCRequestBuilder<'_, IosGCAchievementProgressResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::achievement_progress(id, achievement_id, progress);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestAchievementProgress,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn achievements_reset(
        &mut self,
    ) -> BevyIosGCRequestBuilder<'_, IosGCAchievementsResetResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::achievements_reset(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestAchievementsReset,
            RequestId(id),
            RequestEntity,
        )))
    }

    pub fn leaderboards_score(
        &mut self,
        leaderboard_id: String,
        score: i64,
        context: i64,
    ) -> BevyIosGCRequestBuilder<'_, IosGCScoreSubmitResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::leaderboards_score(id, leaderboard_id, score, context);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestLeaderboardScore,
            RequestId(id),
            RequestEntity,
        )))
    }
}

pub struct BevyIosGCRequestBuilder<'a, T>(EntityCommands<'a>, PhantomData<T>);

impl<'a, T> BevyIosGCRequestBuilder<'a, T>
where
    T: 'static + Event,
{
    fn new(ec: EntityCommands<'a>) -> Self {
        Self(ec, PhantomData)
    }

    pub fn on_response<RB: Bundle, RM, OR: IntoObserverSystem<T, RB, RM>>(
        &mut self,
        on_response: OR,
    ) -> &mut Self {
        self.0.observe(on_response);
        self
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct BevyIosGamecenterSet;

pub fn plugin(app: &mut App) {
    app.init_resource::<BevyIosGamecenterState>();
    app.add_systems(
        PreUpdate,
        (
            cleanup_finished_requests,
            process_events.run_if(on_event::<IosGamecenterEvents>),
        )
            .chain()
            .in_set(BevyIosGamecenterSet),
    );
}

fn cleanup_finished_requests(
    mut commands: Commands,
    query: Query<Entity, (With<RequestEntity>, Without<RequestId>)>,
) {
    for e in query.iter() {
        if let Some(mut ec) = commands.get_entity(e) {
            ec.despawn();
        }
    }
}

#[allow(unused_variables, unused_mut)]
fn process_events(
    mut events: EventReader<IosGamecenterEvents>,
    mut commands: Commands,
    request_authentication: Query<(Entity, &RequestId), With<RequestAuthentication>>,
    request_player: Query<(Entity, &RequestId), With<RequestPlayer>>,
    request_save_games: Query<(Entity, &RequestId), With<RequestSaveGames>>,
    request_save_game: Query<(Entity, &RequestId), With<RequestSaveGame>>,
    request_load_game: Query<(Entity, &RequestId), With<RequestLoadGame>>,
    request_resolve_conflicts: Query<(Entity, &RequestId), With<RequestResolveConflicts>>,
    request_delete_savegame: Query<(Entity, &RequestId), With<RequestDeleteSavegame>>,
    request_fetch_signature: Query<(Entity, &RequestId), With<RequestFetchSignature>>,
    request_achievement_progress: Query<(Entity, &RequestId), With<RequestAchievementProgress>>,
    request_achievements_reset: Query<(Entity, &RequestId), With<RequestAchievementsReset>>,
    request_leaderboard_score: Query<(Entity, &RequestId), With<RequestLeaderboardScore>>,
) {
    for e in events.read() {
        match e {
            IosGamecenterEvents::Authentication((r, response)) => {
                for (e, id) in &request_authentication {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::Player((r, response)) => {
                for (e, id) in &request_player {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::SaveGames((r, response)) => {
                for (e, id) in &request_save_games {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::SavedGame((r, response)) => {
                for (e, id) in &request_save_game {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::LoadGame((r, response)) => {
                for (e, id) in &request_load_game {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::ResolvedConflicts((r, response)) => {
                for (e, id) in &request_resolve_conflicts {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::DeletedSaveGame((r, response)) => {
                for (e, id) in &request_delete_savegame {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::ItemsForSignatureVerification((r, response)) => {
                for (e, id) in &request_fetch_signature {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::AchievementProgress((r, response)) => {
                for (e, id) in &request_achievement_progress {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::AchievementsReset((r, response)) => {
                for (e, id) in &request_achievements_reset {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::LeaderboardScoreSubmitted((r, response)) => {
                for (e, id) in &request_leaderboard_score {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::ConflictingSaveGames(_) => {}
        }
    }
}
